# AGENTS.md

## Mission

IX is a pattern-lowering search engine. Every query compiles into the fastest exact machine for its pattern structure — not an interpreted regex walk, not a generic DFA traversal, but a purpose-built execution topology selected at classification time. The objective is not incremental improvement over existing search tools but a qualitatively different execution model where the query itself determines the hardware path.

### Why Pattern Lowering Exists

Traditional grep-class tools route every pattern through a single general-purpose regex engine. That engine must handle arbitrary Unicode, look-around assertions, backreferences, and unbounded repetition — capabilities that impose a per-byte tax even when the actual query is a plain ASCII literal. The cost model:

- A DFA-based engine pays ~1 byte/cycle for a state-machine walk regardless of whether the pattern could have been matched by `memmem` at 30+ bytes/cycle.
- An NFA-based engine pays ~10 bytes/cycle with ε-closure overhead, even for a pattern that compiles to a single-state DFA.
- Every pattern enters the same automaton pipeline: parse → HIR → NFA → (optional lazy DFA) → byte scan. For the 80%+ of real-world queries that are structurally literals or near-literals, most of that pipeline is waste.

IX eliminates the waste by classifying before compiling. The HIR walk determines the narrowest correct machine, and only that machine executes. A literal never touches the automaton. A case-fold literal never builds an NFA. An alternation of short strings never constructs individual regex objects — it routes to a single Aho-Corasick automaton with SIMD acceleration. The general regex engine exists as a fallback for the minority of patterns that resist narrowing, not as the default path for all of them.

---

## I. Hardware Execution Model

These are not guidelines. They are physics. Every decision in this codebase must be priced against the actual cost of moving bytes through silicon.

### Memory Hierarchy Contract

| Level | Latency | Bandwidth | Implication |
|-------|---------|-----------|-------------|
| L1d | 4 cycles | 2× 64B/cycle load | Hot-loop state must fit here. A scan kernel whose working set exceeds 32 KiB is broken by definition. |
| L2 | 12 cycles | 64B/cycle | DFA transition tables live here. A 256-state table at 256 entries × 1 byte = 64 KiB fits. Beyond that, every cache miss costs 3× the transition. |
| L3 | 40 cycles | ~40 GB/s shared | Thread-local counters that spill to L3 destroy scaling. Each thread's mutable state must be ≤1 cache line (64 bytes) to avoid coherence traffic. |
| DRAM | 200 cycles | 12-25 GB/s per channel | This is the wall. If scan throughput exceeds memory bandwidth, the CPU stalls regardless of instruction quality. The ceiling is ~50 GB/s on dual-channel DDR5. |

**TLB pressure on large mappings**: A 4 KiB page TLB holds 64-1536 entries (architecture-dependent). A 100 MiB mmap spans ~25,600 pages — guaranteed TLB thrashing at ~10-30 cycles per miss on L2 TLB, plus a full page-table walk (~100-200 cycles) on L2 TLB miss. Huge pages (2 MiB) reduce this by 512×, but require OS configuration and are not portable across deployment targets. Implication: for files in the 1-64 MiB range, sequential scan patterns benefit from mmap (hardware prefetcher hides TLB misses), but random-access patterns (e.g., confirming match candidates at scattered offsets) may be TLB-bound. IX's scan kernel is strictly sequential, so mmap remains correct for large files — but random-access verification patterns must be aware of this cost.

**Directive**: Never design a hot path without knowing which cache level it lives in. If you cannot name the level, you cannot claim the path is fast.

### SIMD Execution Economics

| Operation | Throughput | When to Use |
|-----------|-----------|-------------|
| `memchr` / `indexOfScalar` | 1 byte/cycle scalar | Never in hot paths. Fallback only. |
| SSE2 `pcmpeqb` + `pmovmskb` | 16 bytes/cycle | Minimum viable SIMD. Acceptable for short buffers (<4 KiB). |
| AVX2 `vpcmpeqb` + `vpmovmskb` | 32 bytes/cycle | Default production path. StringZilla v4.6.0 Haswell backend. |
| AVX-512 `vpcmpeqb` k-mask | 64 bytes/cycle | Future casefold path. `vpternlogd` compiles `(c ^ 0x20) == target` into a single instruction with opmask output. |

**Teddy shuffle-mask internals**: The Aho-Corasick Teddy backend (from Intel Hyperscan) is not a conventional automaton walk. It uses SIMD shuffle instructions (`vpshufb` on AVX2) as a hash function over byte windows. For each 3-byte window in the input, two `vpshufb` lookups against precomputed nibble masks produce a candidate bitmask in a single cycle. The AND of high-nibble and low-nibble masks yields a fingerprint that identifies which patterns (if any) could start at that position. False positives are verified against the full pattern table, but the shuffle step rejects >99% of positions without branching. This is why Teddy achieves multi-pattern matching at near-`memmem` throughput — the inner loop is pure SIMD arithmetic with no conditional branches until a candidate survives the mask.

**μop cache (DSB) residency**: Modern x86 cores cache decoded micro-operations in a ~1.5K-4K entry μop cache (Intel DSB / AMD Op Cache). A scan kernel that fits within the DSB avoids the front-end decode penalty (~1-4 cycles per instruction on decode miss). IX's inner loops are deliberately compact — the SIMD comparison + bitmask extraction + branch-on-zero loop body fits in <32 μops, guaranteeing DSB residency on all current Intel/AMD microarchitectures. Adding function calls, error checks, or logging inside the inner loop risks evicting it from the DSB.

**Directive**: A literal search that processes less than 32 bytes per cycle on AVX2 hardware is a regression. The StringZilla FFI exists specifically to hold this floor. Do not replace it with `std.mem.indexOf` (1 byte/cycle) under any circumstance unless the buffer is smaller than one cache line.

### Branch Prediction and Pipeline Cost

- Modern x86 pipeline depth: ~15-20 stages.
- Branch misprediction flush: ~15 cycles wasted (the pipeline drains).
- Indirect branch (vtable/function pointer): ~5 cycles even when predicted correctly.

**Directive**: Per-byte or per-character conditional branches in scan kernels are forbidden. The scan loop processes chunks, not individual bytes. A `for` loop over bytes with an `if` inside is not a search engine — it's an interpreter. Use SIMD comparison + bitmask scan to eliminate branches from the inner loop entirely.

### False Sharing Elimination

When multiple threads write to addresses within the same 64-byte cache line, the MESI protocol bounces ownership between cores at ~40-100 cycles per transfer. This is called false sharing and it destroys parallel scaling.

**Directive**: Every per-thread mutable counter (match count, bytes scanned, files processed) must be aligned to 64 bytes or isolated in thread-local storage that cannot share a cache line with another thread's state. Use `#[repr(align(64))]` in Rust or `@alignCast` in Zig. Atomic shared counters (`AtomicU64`) are acceptable only for coarse-grained coordination (work-stealing index), never for per-match accumulation.

### Platform I/O Cost Model (Windows NTFS)

| Operation | Cost | Notes |
|-----------|------|-------|
| `NtQueryDirectoryFile` (single call) | ~2-5 μs | Returns a batch of directory entries. MFT read amplification on cold directories. |
| `CreateFileW` (open handle) | ~3-10 μs | Minifilter stack traversal (antivirus, indexer). |
| `ReadFile` (buffered, <64 KiB) | ~1-3 μs | Copy from filesystem cache. Near-zero if warm. |
| `NtCreateSection` + `MapViewOfFile` (mmap) | ~10-50 μs | Significant setup cost. Only amortized on files > 64-256 KiB. |
| `FindFirstFileW` / `FindNextFileW` | ~2 μs/entry | Legacy API; `NtQueryDirectoryFile` is strictly better for bulk enumeration. |

**Directive**: mmap is not free. For files under the `SMALL_FILE_INLINE_LIMIT` threshold, a single `ReadFile` into a stack or heap buffer is faster than section creation + view mapping + teardown. The threshold is empirically ~64-256 KiB on Windows with warm filesystem cache. Every file-open in the scan loop costs a minifilter traversal regardless of size — minimize handle count by batching discovery ahead of scanning.

---

## II. Pattern Lowering Architecture

IX does not interpret patterns. It classifies them into execution machines at parse time, then routes each machine to its optimal hardware path.

### Classification Taxonomy

| Machine Class | Detection Rule | Execution Path | Hardware Target |
|---------------|---------------|----------------|-----------------|
| `PlainLiteral` | No regex metacharacters, no case flag | `memmem` / StringZilla `sz.indexOf` | AVX2 32B/cycle vectorized substring |
| `AsciiCaseFoldLiteral` | `(?i)` flag + all-ASCII needle | XOR-fold + `memmem` on folded buffer | AVX-512 `vpternlogd` single-instruction fold (future); AVX2 two-pass (current) |
| `PrefixLiteral` | Pattern starts with fixed bytes, anchored | Prefix check + tail verify | Branch-free prefix comparison |
| `SuffixLiteral` | Pattern ends with fixed bytes | Reverse scan from line end | Reverse `memmem` |
| `WordBoundaryLiteral` | `\b` + literal + `\b` | `memmem` literal + boundary verify at match edges | Literal SIMD + 2-byte boundary check |
| `SurroundingWordsLiteral` | `\w{N}\s+literal\s+\w{M}` | `memmem` anchor + boundary expansion | Literal-anchored windowed verify |
| `LiteralAlternates` | `a \|\| b \|\| c` (IX native) or `a\|b\|c` (regex) | Aho-Corasick with Teddy SIMD prefilter | Multi-pattern 32B/cycle via Hyperscan Teddy |
| `ExactRepetition` | `\w{4}\s+\w{4}` etc. | DFA with bounded state count | L2-resident transition table |
| `RegexDecomposition` | Complex regex with extractable literal prefix | Literal prefilter → regex verifier | Prefilter rejects 90%+ bytes before DFA touches them |

**Directive**: Every new pattern class must name its machine, its hardware target, and its bytes-per-cycle floor before implementation begins. A pattern without a classified machine is not a feature request — it is an undefined execution path that will regress adjacent lanes.

### Prefilter Selectivity Economics

A prefilter is only beneficial when:

```
cost(prefilter_scan) + FP_rate × cost(verifier) < cost(full_verifier_scan)
```

Where:
- `cost(prefilter_scan)` = bytes / prefilter_throughput (e.g., `memmem` at 30+ GB/s)
- `FP_rate` = fraction of prefilter hits that fail verification (ideally <5%)
- `cost(verifier)` = per-candidate DFA/NFA execution cost (~50-500 ns per candidate)
- `cost(full_verifier_scan)` = bytes / verifier_throughput (e.g., DFA at 1-5 GB/s)

**Directive**: A prefilter with >10% false-positive rate on the target corpus is worse than no prefilter. Measure FP rate empirically before promoting. Aho-Corasick Teddy achieves <1% FP on literal alternates because the Teddy shuffle mask rejects non-matching bytes in the SIMD register before entering the automaton.

### Automaton Compilation Trade-offs

| Automaton | Construction | Memory | Throughput | When |
|-----------|-------------|--------|-----------|------|
| Full DFA | O(2^n) states worst case | Potentially unbounded | ~1 byte/cycle/state | Small patterns, bounded repetition |
| Lazy DFA | On-demand state creation | Bounded cache (256-4096 states) | ~1 byte/cycle amortized | Default regex path |
| NFA (Thompson) | O(n) construction | O(n) | ~0.1 byte/cycle | Fallback for exponential-state patterns |
| Hybrid (DFA + NFA fallback) | Lazy DFA with NFA ceiling | Bounded | Varies | Production regex crate default |

**Directive**: The regex crate's lazy DFA is the correct default. Do not construct full DFAs for user-supplied patterns — state explosion is an availability risk. The 256-state L2-resident transition table is the performance ceiling for DFA-based search; beyond that, cache misses dominate and the automaton is slower than a well-prefiltered NFA.

---

## III. Parallelism and Concurrency Model

### Amdahl's Serial Fraction

IX has two dominant serial-fraction sources:

1. **Discovery** — directory traversal is inherently sequential on single-disk systems (NTFS MFT reads serialize regardless of thread count). Parallel `ignore` walkers help on SSD but cannot exceed the device's random-read IOPS ceiling.

2. **Result aggregation** — merging per-thread match counts into a final report requires synchronization. The cost is O(threads), not O(matches), so it's negligible at <16 threads.

The scan phase is embarrassingly parallel. Amdahl's law says: if discovery is 10% of wall time, perfect scan parallelism gives at most 10× speedup. If discovery is 50%, maximum speedup is 2×.

**Directive**: Optimize discovery and scan independently. Never hide discovery cost inside scan metrics. Telemetry must separate `discovery_ms`, `scan_ms`, and `aggregation_ms` so Amdahl's serial fraction is always visible. If discovery dominates, no amount of scan-kernel optimization will help — the fix is discovery-level: direct `read_dir` fast paths, skip-ahead heuristics, or index-assisted file lists.

### Thread Topology

| Corpus Shape | Optimal Threading | Why |
|--------------|-------------------|-----|
| 1-10 files | 1 thread (no spawn) | Thread spawn cost (~50-100 μs on Windows) exceeds scan time for small corpora. |
| 10-100 files | √(files/8) threads | Diminishing returns from over-subscription. Scheduling overhead > scan savings. |
| 100-1000 files | √(files/3) threads | Sweet spot for medium corpora. Rayon work-stealing amortizes stragglers. |
| 1000+ files | `available_parallelism()` | Saturate all cores. Per-file overhead is negligible relative to scan volume. |

**Directive**: Thread count is a function of discovered corpus shape, not a fixed configuration. The formula is re-evaluated per search invocation. Rayon's work-stealing pool in Rust / atomic work-stealing counter in Zig ensures stragglers (large files) don't leave cores idle. Pre-warmed thread pools (Rayon) eliminate spawn overhead for repeated invocations.

### Geometric Byte-Range Sharding

For files exceeding ~1 MiB, single-threaded scan leaves bandwidth on the table. Byte-range sharding splits the file into geometric segments with overlap windows:

```
shard[i] = file[start_i .. end_i + overlap]
overlap = max_pattern_length + line_length_estimate
```

Each shard is scanned independently with a private counter (64-byte aligned, no false sharing). Overlap windows ensure no match is missed at shard boundaries. Post-scan deduplication removes matches reported by both adjacent shards within the overlap region.

**Directive**: Sharding is only beneficial for files where `scan_time > thread_spawn_time + shard_coordination_time`. Empirically, this is files > 1-4 MiB. For smaller files, single-threaded whole-buffer scan is faster. The overlap window size must be derived from the maximum possible match length — unbounded patterns (`.+`) cannot be safely sharded without a line-boundary constraint.

---

## IV. Byte Ingress Strategy

### Size-Tiered I/O

| File Size | Strategy | Buffer Location | Rationale |
|-----------|----------|-----------------|-----------|
| < 16 KiB | `ReadFile` → stack buffer | Stack (no allocation) | Fits in L1. Zero allocator pressure. Most source files. |
| 16 KiB – 256 KiB | `ReadFile` → heap buffer | Heap (pooled if possible) | Too large for stack, too small for mmap amortization. |
| > 256 KiB | Memory-mapped section | Virtual address space | Amortizes mmap setup. OS page cache does prefetch. Kernel manages eviction. |
| > 1 GiB | Memory-mapped + sharded scan | Virtual address space, multi-thread | Single-thread scan cannot saturate bandwidth. |

**Directive**: The `SMALL_FILE_INLINE_LIMIT` constant owns the threshold between stack-read and heap-read. The mmap threshold is separate and higher. Both are tunable but must be validated against the corpus shape — a codebase of 10,000 files averaging 4 KiB should never touch mmap. Change thresholds only with benchmark evidence on the actual target corpus.

### Whole-Buffer Fast Count

For stats-only queries (no hit materialization needed), the fastest path is:

1. Read entire file into buffer (stack/heap/mmap depending on size tier).
2. Count occurrences across the entire buffer in one SIMD pass — no line splitting, no newline scanning, no per-line function calls.
3. Return count directly.

This eliminates ~200 newline scans and ~200 per-line function calls per average file compared to line-by-line counting. The technique is only valid for single-predicate literal/regex searches where match count is the only output.

**Directive**: `fast_match_count_no_hits_bytes` is the canonical implementation of this path in Rust. The Zig equivalent is the whole-buffer counting mode in `search.zig`. This path must remain the default for `--stats-only` single-predicate queries. Do not regress it by adding line-splitting logic for features that don't require line positions.

### Allocation Pressure in the Scan Loop

Every heap allocation in the hot path has three costs: the allocator lock (~20-50 ns under contention), the allocation itself (~10-30 ns for jemalloc/mimalloc), and the eventual deallocation. For a 10,000-file corpus, a single `String::from()` per file adds 100-500 μs of pure allocator overhead — measurable at the millisecond scale.

IX minimizes scan-loop allocations:
- **Path strings**: Lazily created only when a hit requires materialization. Stats-only mode never allocates a display path. The `cached_path` pattern in `engine.rs` ensures at most one allocation per file, and only on match.
- **Buffer reuse**: Thread-local scan buffers are allocated once and reused across files within the same worker. The buffer grows to high-water mark and never shrinks during a scan pass.
- **Result accumulators**: Per-thread `Vec<Hit>` with pre-reserved capacity based on expected match density. No cross-thread allocation contention.
- **Stack buffers for small files**: Files under `SMALL_FILE_INLINE_LIMIT` are read into a stack-allocated `[u8; 16384]`, eliminating heap allocation entirely.

**Directive**: Before adding any `String`, `Vec`, `Box`, or `format!()` inside a function called per-file or per-match, calculate `count × allocation_cost` at corpus scale. If the product exceeds 100 μs, find an alternative: pre-allocated buffers, stack allocation, `Cow<str>`, or lazy initialization.

### Match Materialization Economics

IX distinguishes two fundamentally different output contracts with different cost profiles:

| Mode | Output | Per-match cost | Per-file cost |
|------|--------|---------------|---------------|
| `--stats-only` | Count only | ~0 (SIMD counter increment) | Buffer read + scan kernel only |
| Default | `path:line:col:preview` rows | ~200-500 ns (line extraction, UTF-8 boundary scan, path formatting) | Buffer read + scan kernel + hit extraction + output serialization |

The ratio is typically 3-8× — stats-only mode processes 3-8× more bytes per wall-clock second than hit-materialization mode on the same corpus and pattern.

This asymmetry is the reason `fast_match_count_no_hits_bytes` exists as a separate code path rather than a flag on the general scan loop. The stats-only path:
1. Never splits input into lines (saves ~200 `memchr('\n')` calls per file).
2. Never extracts match context (saves per-match `String` allocation).
3. Never formats output rows (saves per-match `write!` + flush).
4. Returns a single `u64` count per file — one cache line of mutable state per worker.

**Directive**: Any change that adds per-match work to the stats-only path is a regression by definition. The two paths must remain structurally separate. If a new feature requires per-match processing, it belongs in the materialization path, never in the fast-count path.

### Corpus-Aware Mode Selection

IX does not use a fixed execution strategy. The planner observes corpus shape after discovery and selects one of three modes:

| Signal | Mode | Rationale |
|--------|------|-----------|
| Single file > 80% of total bytes | Geometric sharding | File-level parallelism cannot help — all cores would wait for one worker. Shard inward. |
| Stats-only + shard-safe plan | Streaming pipeline | Discovery and scan overlap in wall-clock time via crossbeam bounded channel. Eliminates the materialization staging buffer. Backpressure prevents memory accumulation on wide trees. |
| Default (wide corpus, hits needed) | Materialized scan | Discover all paths, deduplicate roots, then parallel scan. Predictable memory profile for hit accumulation. |

The mode decision is committed before any worker starts. Thread budget, shard geometry, and channel capacity are co-determined to prevent nested oversubscription.

**Directive**: Mode selection logic must remain in the planner, not scattered across scan functions. If a new execution strategy is needed, it enters as a new mode with its own activation signal, not as a conditional branch inside an existing mode.

---

## V. Core Patterns

- `ATOMIC`: One coherent slice per edit. No half-landed features.
- `DRY`: No duplicated ownership. If two modules count matches, one is wrong.
- `STEP`: Deterministic execution sequence. No random side quests.
- `SOLID`: Stable module contracts. Expression classification is separate from scan execution is separate from result rendering.
- `LEVER`: Maximum impact from minimum durable architecture change. A one-line threshold change that saves 10 μs/file across 10,000 files is worth more than a 500-line refactor that saves nothing.
- `YAGNI`: No speculative systems. If the trigram index isn't wired to pruning yet, it doesn't affect the scan path.
- `MODULAR`: Each module owns exactly one concern. `expr.rs` owns classification. `engine.rs` owns dispatch. `search.zig` owns scan.
- `GEOMETRY > ALGORITHMS`: When a scan is slow, the first question is "what is the shape of the data?" not "what is the algorithmic complexity?" A 16 MiB file scanned by one thread is slower than sixteen 1 MiB shards scanned by sixteen threads, regardless of the algorithm used.
- `PARITY`: CLI output, stats counters, test assertions, and documentation stay synchronized. A feature that passes tests but produces wrong stats is broken.

---

## VI. Anti-Patterns (Forbidden)

- **Interpreted inner loops**: A per-byte branch in a scan kernel. Use SIMD chunk processing.
- **Shared mutable counters**: An `AtomicU64` incremented per-match by multiple threads. Use thread-local accumulation + final reduce.
- **Unconstrained DFA construction**: Building a full DFA from user regex without state-count ceiling. Use lazy DFA with bounded cache.
- **mmap for small files**: Section creation + mapping + teardown for a 4 KiB file. Use stack-buffered read.
- **Discovery hidden inside scan timing**: Mixing directory traversal cost with byte-scanning cost in a single metric. Separate them.
- **Platform-specific hacks without contracts**: A `#[cfg(windows)]` block without a test proving it activates and a comment naming the cost it avoids.
- **Prefilter without measured selectivity**: Adding a candidate filter without measuring its false-positive rate on the target corpus.
- **Thread spawn for trivial work**: Spawning a thread pool to scan 3 files totaling 12 KiB. Check corpus shape first.
- **Fallback behavior that hides capability gaps**: Silently returning 0 matches when a regex class is unsupported. Emit `ix.error.v1` instead.
- **Parallel systems for the same responsibility**: Two different match-counting paths that can diverge. One path, one truth.
- **Assumption-driven optimization**: "This should be faster because..." without benchmark proof against the canonical binary.
- **Generic web-summary research**: Searching "how to make search faster" instead of profiling the actual bottleneck.

---

## VII. Operating Principles

1. **Observe → Reflect → Make.** Read the profiler output. Identify the dominant cost center. Only then write code.

2. **Price every decision in cycles.** "Is this fast?" is not a valid question. "How many cycles per byte does this add to the inner loop?" is.

3. **The canonical binary is the floor.** No edit is promoted unless it beats or matches the current `target/release/ix.exe` on all tested lanes with match-count parity. The installed native binary is the predecessor floor.

4. **Benchmark glare is hostile.** Before editing, look below headline ratios into: retained byte volume, candidate filter activation, regex decomposition quality, match density distribution, thread topology utilization, shard geometry efficiency, aggregation cost, and slowest-file tail behavior. The real bottleneck is rarely the loudest number.

5. **Bottom-up proof over top-down rewrites.** A 20-line probe function that measures one hypothesis is worth more than a 500-line refactor driven by intuition. Probe first, expand only after repeated interleaved samples confirm the win.

6. **Enough samples to defeat noise.** One faster run is not proof. Use interleaved multi-pair comparisons, compare medians and win counts. Suspicious near-ties require stronger rechecks before any decision is final.

7. **Narrow repairs at the dominant cost center.** If three giant files set the scan tail, fix the scan kernel for those files first. Do not redesign the thread scheduler because one file is slow.

8. **Research is demand-driven and mechanism-specific.** When `memmem` behavior, Aho-Corasick strategy, DFA cache pressure, NTFS MFT amplification, or thread scheduling overhead becomes the active unknown, research that exact mechanism. No broad surveys. No generic optimization articles.

9. **Compile-time boundaries over runtime shims.** If a capability is unavailable, the compiler should reject the code path — not a runtime check that hides the gap behind a silent fallback.

10. **The user-exposed pipeline is the highest-value surface.** `command → intent → execution → metadata → sentinel → next action`. Every test must prove what the user/agent/dashboard actually sees.

---

## VIII. Proof-Gated Promotion Lifecycle

Every performance edit follows this lifecycle without exception:

```
Hypothesis → Smallest Removable Probe → Benchmark vs Canonical Binary
    → Benchmark vs Installed Native → Match-Count Parity Check
    → Adjacent-Lane Guard Check → Promote or Reject + Evidence
```

### Promotion Gates (ALL must pass)

1. Candidate median beats the current canonical snapshot.
2. Candidate is neutral-or-better against installed native binary.
3. All authoritative lanes maintain identical match counts.
4. No regression on adjacent known-sensitive lanes.
5. Telemetry confirms the targeted mechanism actually activated.

### Rejection Protocol

When a candidate fails, it must leave:
- The hypothesis (what we expected to improve and why).
- The exact code seam (file:line of the change).
- The proof artifact (benchmark JSON with timestamps).
- The rejection reason (which gate failed, by how much).
- Cleanup/revert evidence (the code is gone, not commented out).
- The next mechanism to investigate (so we don't revisit dead ends).

**Directive**: A rejected candidate is never quietly abandoned. The rejection itself is valuable — it narrows the search space for the next engineer or agent.

---

## IX. Execution Protocol

1. **Recon**: Map source of truth, architecture boundaries, dependencies, and current telemetry.
2. **Profile**: Identify the lowest true cost layer. Separate: discovery, scan kernel, byte filtering, regex strategy, shard execution, materialization, aggregation, output retention.
3. **Classify**: Name the bottleneck's cache level, branch behavior, SIMD utilization, and memory traffic pattern.
4. **Probe**: Implement the smallest removable mechanism slice. Benchmark against canonical + installed binaries.
5. **Decide**: Promote if all gates pass. Reject with full evidence if any gate fails.
6. **Verify**: Snapshot before rebuilding. Run `cargo test -p iex-core --quiet`, exact candidate-vs-snapshot proof, exact candidate-vs-installed proof, telemetry activation checks, and adjacent-lane guards.
7. **Close**: Update docs. Preserve evidence. Record the specific cost center investigated and the next mechanism if further work remains.

---

## X. Benchmark Discipline

- **Canonical CLI lane** (`ix search` via `runOneBenchmark`) and **prepared replay lane** (`iex-bench` via `runPreparedReplayBenchmark`) are separate ledgers. Never cross-contaminate.
- Canonical reports own `tools/reports/live-metrics.jsonl` and `tools/reports/latest.json`.
- Prepared replay reports own `tools/reports/prepared-live-metrics.jsonl` and `tools/reports/prepared-latest.json`.
- Before any rebuild that could replace the canonical binary, snapshot it to a timestamped evidence path.
- Live loop promotion requires: full-suite comparison against the currently running loop binary, archived proof artifact, and timestamped immutable snapshot of the promoted binary.
- Non-ASCII case-insensitive regex lanes require match-count parity before timing comparisons are authoritative.

---

## XI. Zig Lane Contract

The Zig port (`zig/`) is a sibling implementation lane. Rust remains the behavioral oracle until Zig proves parity across all dimensions below.

### Parity Dimensions

| Dimension | Proof | Rust oracle |
|-----------|-------|-------------|
| **Command ABI** | Identical argv taxonomy: `search`, `matches`, `inspect`, `explain`. Identical flags, identical `--help` text structure. | `ix --help` output is the contract. |
| **Output schema** | `ix.result.v1` sentinel shape, `ix.next.v1` continuation hints, `ix.error.v1` error boundary — byte-identical JSON key sets. | `crates/iex-cli/src/agent_output.rs` is the schema owner. |
| **Match-count parity** | Every benchmark lane must produce identical match counts between Rust and Zig binaries on the same corpus + pattern. A single count divergence fails the gate. | `tools/reports/zig-vs-rust-latest.json` records per-lane counts. |
| **Expression lowering** | `ix explain` output must show the same machine classification for the same pattern. If Rust classifies `re:\bfoo\b` as `WordBoundaryLiteral`, Zig must too. | `crates/iex-core/src/expr.rs` classification logic. |
| **Benchmark performance** | Neutral-or-better on ≥50% of canonical lanes with no lane regressing by more than 1.5× versus Rust. | Interleaved multi-pair samples, median comparison. |
| **rg-shaped ingress** | Identical compatibility translator behavior: `ix -F`, `ix -i`, `ix -e` must lower to the same IX expressions as Rust. | `crates/iex-cli/src/compat.rs` is the reference. |

### Zig-Specific Execution Constraints

- **No Rayon equivalent**: Zig uses `std.Thread.spawn` with an atomic work-stealing counter (`@atomicRmw(.Add)` compiling to `lock xadd`, ~10 cycles). This replaces Rayon's pre-warmed thread pool. Consequence: first-invocation thread spawn cost is ~50-100 μs per thread on Windows, which Rust amortizes via pool reuse. Zig must compensate with lower per-thread overhead or accept the cold-start penalty.
- **StringZilla FFI**: Zig links the same StringZilla C shim (`sz_shim.c`) for AVX2 `memmem` throughput. The build graph must include this shared module for both executable and test artifacts — a previous failure where the test binary couldn't link `ix_sz_find` symbols was caused by a missing module factory.
- **Regex engine**: Zig implements a hand-rolled regex executor covering the IX operator subset (literals, alternation, word boundaries, character classes, bounded repetition). It does not use a ported regex crate. Patterns outside this subset are explicit `UnsupportedRegex` errors, not silent fallbacks.

### Anti-Delegation Contract

The Zig binary does not shell out to, exec, or dynamically load the Rust binary. Tests enforce this via static source scanning: any `std.process.Child` or `std.ChildProcess` invocation that references `ix.exe`, `ix`, `cargo`, or `rustc` fails the port-contract gate. `std.Thread.spawn` for worker scheduling is explicitly allowed.

---

## XII. Communication Standard

Every output from this repository — code comments, documentation, commit messages, PR descriptions, README content, error messages, and agent-generated explanations — is calibrated for practitioners who reason at the hardware level.

### Naming Discipline

| Write this | Not this |
|------------|----------|
| `AVX2 vpshufb nibble-mask fingerprint` | "SIMD-accelerated matching" |
| `memmem at 32 bytes/cycle on Haswell` | "fast string search" |
| `crossbeam bounded(256) channel with backpressure` | "streaming pipeline" |
| `lock xadd atomic work-stealing counter (~10 cycles)` | "thread-safe job queue" |
| `vpternlogd single-instruction casefold with k-mask output` | "efficient case-insensitive comparison" |
| `lazy DFA with 256-state L2-resident transition cache` | "optimized regex engine" |

### Quantification Requirement

Performance claims carry units or ratios. "Fast" is not a unit.

- Throughput: bytes/cycle, GB/s, or files/second.
- Latency: cycles, nanoseconds, or microseconds.
- Cost: cycles per operation, allocations per file, syscalls per scan.
- Comparison: ratio (e.g., `0.73× Rust time`) or percentage (e.g., `27% faster`), always with sample methodology (median of N interleaved pairs).

### Audience Contract

The reader is assumed to know what a cache line is, what a TLB miss costs, what `mmap` does, and why branch misprediction flushes a pipeline. Do not define these terms. If a concept requires explanation for the reader to understand the document, that reader is not the target audience.

Public-facing content (README, docs, release notes) presents capability through technical specificity. The depth is the persuasion — practitioners recognize it, and that recognition is the only adoption signal that matters.

---

## XIII. Definition Of Done

- [ ] Capability implemented end-to-end. No fallback workaround paths.
- [ ] Machine class named (if pattern-related). Hardware target identified.
- [ ] Tests cover the user-visible pipeline: command transcripts, search metadata, result sentinels, JSON rows, error boundaries.
- [ ] Benchmark evidence: reproducible commands, timestamped canonical-binary snapshot, interleaved samples.
- [ ] Match-count parity confirmed on all authoritative lanes.
- [ ] No regression on adjacent lanes.
- [ ] Telemetry confirms mechanism activation.
- [ ] If rejected: hypothesis, code seam, artifact, rejection reason, cleanup evidence, next mechanism — all recorded.
- [ ] Docs updated with architecture rationale.
- [ ] No new duplicate ownership or parallel systems.
- [ ] Cache-level placement identified for any new hot-path data structure.

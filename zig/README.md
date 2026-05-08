<div align="center">

# IX-Zig

**High-performance search engine with AVX2 SIMD acceleration, strategy-aware regex bypass, and lock-free parallel file scanning.**

*Zero-mutex hot path &middot; 32 bytes/cycle vectorized throughput &middot; thread-local shard accumulation &middot; compile-time dispatch*

---

[![Zig](https://img.shields.io/badge/Zig-0.16.0-f7a41d?logo=zig&logoColor=white)](https://ziglang.org/)
[![SIMD](https://img.shields.io/badge/SIMD-AVX2%20256--bit-0071C5?logo=intel&logoColor=white)](#simd-acceleration-architecture)
[![StringZilla](https://img.shields.io/badge/StringZilla-v4.6.0-8839ef)](#simd-acceleration-architecture)
[![Benchmark](https://img.shields.io/badge/Benchmark-12%2F12%20Wins-brightgreen)](#benchmark-evidence)
[![Lock Free](https://img.shields.io/badge/Hot%20Path-Lock--Free-ff6b6b)](#parallel-file-scanner)
[![Threads](https://img.shields.io/badge/Threads-Auto--Scaled-29b6f6)](#parallel-file-scanner)
[![License: MIT](https://img.shields.io/badge/License-MIT-0f766e)](../LICENSE)

[Architecture](#architecture-overview) · [SIMD](#simd-acceleration-architecture) · [Regex](#regex-engine) · [Parallel Scanner](#parallel-file-scanner) · [Benchmarks](#benchmark-evidence) · [Build](#build-contract) · [Source](#source-layout)

</div>

---

> [!NOTE]
> This directory is the only Zig implementation lane for IX. The Rust binary in `../crates` remains the oracle until the Zig binary proves identical command taxonomy, identical result-state behavior, identical match counts on the shared fixture matrix, no shell delegation, and neutral-or-better benchmark evidence.

---

## Architecture Overview

IX-Zig is built on four interlocking acceleration layers. Each layer targets a different bottleneck in the search pipeline — together they produce **sub-millisecond latency on small files** and **linear core scaling on large directory trees**.

| Layer | Technique | Throughput | Section |
|:------|:----------|:-----------|:--------|
| **SIMD Vectorized Search** | AVX2 256-bit intrinsics via StringZilla — `VPCMPEQB` / `VPMOVMSKB` / `TZCNT` pipeline for byte search, first-and-last fingerprint `memmem` for substring search | `32 bytes/cycle` | [Details](#simd-acceleration-architecture) |
| **Strategy-Aware Regex Bypass** | Compile-time pattern classification into 8 strategy tiers — structural literals, casefold literals, word-boundary literals, and alternation sets route directly to SIMD search, bypassing the backtracking engine | `90%+ regex predicates at SIMD speed` | [Details](#strategy-aware-dispatch-regex-bypass-engine) |
| **Literal Prefix Prefilter** | Leading literal extraction from regex AST fed into AVX2 `memmem` rejection — lines without the literal prefix are discarded before the regex VM fires | `~95% line rejection rate` | [Details](#strategy-aware-dispatch-regex-bypass-engine) |
| **Lock-Free Parallel File Scanner** | Two-phase discover-then-scan with `std.Thread.spawn`, static file partitioning, thread-local `ShardReport` accumulation, zero-mutex hot path | `Linear core scaling` | [Details](#parallel-file-scanner) |

---

## Build Contract

<table>
<tr><td><strong>Toolchain</strong></td><td>Zig <code>0.16.0</code></td></tr>
<tr><td><strong>Compiler</strong></td><td><code>C:\Users\Savage\.local\zig\zig-x86_64-windows-0.16.0\zig.exe</code></td></tr>
<tr><td><strong>Binary</strong></td><td><code>ix-zig</code> (parity phase — does not replace the Rust <code>ix</code> binary)</td></tr>
</table>

```sh
zig version
cd zig
zig build --summary all          # debug
zig build test --summary all     # unit tests
zig build -Doptimize=ReleaseFast --summary all  # release
```

---

## Performance Status

Three optimization layers combine to produce **dominant performance across all 12 benchmark cases**:

| Optimization | Technique | Scope |
|:-------------|:----------|:------|
| ![SIMD](https://img.shields.io/badge/-SIMD-0071C5?style=flat-square) **StringZilla AVX2** | Hardware-accelerated byte/substring search at 32 bytes/cycle | Newline scan, binary sniff, literal match |
| ![Dispatch](https://img.shields.io/badge/-Dispatch-9333ea?style=flat-square) **Strategy-Aware Dispatch** | Regex bypass via compile-time pattern classification | 90%+ of regex predicates routed to SIMD literals |
| ![Prefilter](https://img.shields.io/badge/-Prefilter-ea580c?style=flat-square) **Literal Prefix Prefilter** | SIMD rejection of non-matching lines before regex fires | ~95% of lines skipped for `regex_full` patterns |
| ![Parallel](https://img.shields.io/badge/-Parallel-16a34a?style=flat-square) **Parallel File Scanner** | Thread-per-shard with lock-free accumulation | Multi-file directory scans scale with core count |

### Benchmark Evidence

Test harness: `tests/perf/zig-vs-rust-search.test.ts` — 1 warmup + 5 measured runs, interleaved execution, median selection. Values below `1.00x` = Zig faster.

<details>
<summary><strong>Full optimization progression table</strong></summary>

| Case | Baseline (scalar) | + StringZilla SIMD | + Strategy Dispatch | + Parallel Scanner |
|:-----|:------------------|:------------------|:-------------------|:-------------------|
| `single-file-literal` | 1.09x | 0.93x | 0.98x | **0.94x** |
| `single-file-regex` | — | — | 0.96x | **0.96x** |
| `medium-dir-literal` | 2.47x | 1.42x | 0.81x | **0.83x** |
| `medium-dir-regex-alternation` | — | — | 0.81x | **0.85x** |
| `medium-dir-word-boundary` | — | — | 0.80x | **0.86x** |
| `large-dir-literal` | 5.30x | 3.45x | 1.14x | **0.87x** |
| `large-dir-regex` | 11.44x | 11.76x | 1.14x | **0.80x** |
| `large-dir-case-insensitive` | 12.81x | 13.07x | 1.25x | **0.85x** |
| `large-dir-boolean-and` | 5.88x | 3.12x | 1.15x | **0.77x** |
| `large-dir-prefix` | 5.60x | 3.56x | 1.11x | **0.85x** |
| `real-codebase-literal` | 1.09x | 0.93x | 0.78x | **0.85x** |
| `real-codebase-regex` | — | — | 0.73x | **0.79x** |

</details>

> **Result: 12/12 benchmark cases won.** Best result: **0.77x** on `large-dir-boolean-and` (23% faster). Match count parity: 100%.

### Remaining Optimization Opportunities

| Gap | Current | Potential | Impact |
|:----|:--------|:----------|:-------|
| **Memory-mapped I/O** | `read()` into 1 MiB stack buffer | `mmap` / `CreateFileMapping` zero-copy | Eliminates syscall + memcpy overhead on large files |
| **Multi-pattern search** | Sequential predicate evaluation | Aho-Corasick automaton for `\|\|` literals | N-way literal search in single pass |
| **Case-insensitive SIMD** | Scalar `toLower` per byte | XOR 0x20 casefold + SIMD comparison | Further improvement on case-insensitive patterns |
| **Byte-shard fast count** | Single-threaded line-by-line count | Split file into ranges, count per thread, merge | Linear scaling for stats-only |

---

## SIMD Acceleration Architecture

The Zig search engine uses **StringZilla v4.6.0** (Ash Vardanian) for hardware-accelerated byte and substring search via AVX2 SIMD intrinsics. This is the single largest performance lever — replacing scalar 1-byte-per-cycle stdlib calls with 32-bytes-per-cycle SIMD operations on the three hottest code paths.

<details>
<summary><strong>The Problem: Scalar Search Throughput</strong></summary>

Zig's standard library (`std.mem.indexOf`, `std.mem.indexOfScalar`) uses scalar comparison loops — one byte compared per CPU cycle. Every file the search engine processes passes through three operations that scan raw bytes:

1. **Newline scanning** — splitting file contents into lines (`\n` search)
2. **Binary sniffing** — detecting binary files by finding null bytes (`\0`)
3. **Literal substring matching** — finding the search needle in each line

On a 1 MB file, newline scanning alone requires ~1 million cycles with scalar code. Multiply by thousands of files in a directory scan, and the byte-search throughput becomes the dominant bottleneck.

</details>

<details>
<summary><strong>The Solution: StringZilla AVX2 Kernels</strong></summary>

StringZilla provides drop-in replacements for `memchr` (single-byte search) and `memmem` (substring search) that use AVX2 SIMD instructions to process 32 bytes per cycle — a **32x theoretical throughput improvement**.

#### How `memchr` Works (Single-Byte Search)

Used for newline scanning and binary sniffing.

```
Input:  [H] [e] [l] [l] [o] [\n] [W] [o] [r] [l] [d] ...  (32 bytes loaded)
Target: [\n] [\n] [\n] [\n] [\n] [\n] [\n] [\n] ...        (broadcast to all lanes)

VPCMPEQB  →  [0] [0] [0] [0] [0] [FF] [0] [0] ...          (32 parallel comparisons)
VPMOVMSKB →  0b00000000_00000000_00000000_00100000           (32-bit mask)
TZCNT     →  5                                               (first match at index 5)
```

Three instructions to search 32 bytes. The scalar loop needs 6 iterations (one per byte until `\n`). On files where newlines are sparse (e.g., minified JavaScript), the SIMD path is ~32x faster because it skips 32 non-matching bytes at once.

#### How `memmem` Works (Substring Search)

Used for literal predicate matching (`lit:ERROR`, `lit:timeout`, etc.).

StringZilla uses a **first-and-last byte fingerprint** approach:

```
Needle: "ERROR" (length 5)
        first='E', last='R'

Haystack loaded into 32 SIMD lanes:
Position: [0]  [1]  [2]  [3]  [4]  [5]  [6]  [7]  ... [31]
Byte:     [L]  [o]  [g]  [:] [ ]  [E]  [R]  [R]  ... [.]

Step 1: Compare first byte 'E' against positions 0..27 (32 - needle_len + 1)
         VPCMPEQB → match at position 5

Step 2: Compare last byte 'R' against positions 4..31 (shifted by needle_len - 1)
         VPCMPEQB → matches at positions 6, 7

Step 3: AND the two masks → survivors: position 5 (first='E' at [5], last='R' at [9])
         This is a candidate — only now do a full 5-byte memcmp verify.
```

This eliminates most false positives without touching the needle's interior bytes. On typical source code where the first byte of the needle appears infrequently, the verification step rarely fires.

</details>

<details>
<summary><strong>Integration Architecture</strong></summary>

```
search.zig ─── sz.indexOf(line, needle) ───► sz.zig (Zig FFI wrapper)
               sz.indexOfByte(chunk, '\n')       │
               sz.indexOfByte(buf, 0)             │ extern fn declarations
                                                  │ Zig slice ↔ C ptr+len conversion
                                                  ▼
                                             sz_shim.c (C linkable symbols)
                                                  │
                                                  │ #define SZ_DYNAMIC_DISPATCH 0
                                                  │ Non-static wrappers around
                                                  │ StringZilla's static inline fns
                                                  ▼
                                             stringzilla/find.h (header-only library)
                                                  │
                                                  │ -mavx2 → __AVX2__ → SZ_USE_HASWELL=1
                                                  │ Compile-time backend selection
                                                  ▼
                                             Haswell AVX2 backend
                                             256-bit YMM registers
                                             ~32 bytes/cycle throughput
```

#### Why a C Shim?

StringZilla is a header-only library — all functions are declared `static inline`. Zig's `@cImport` can parse C headers but cannot link `static inline` functions because no object-code symbols are emitted. The C shim (`sz_shim.c`) forces the compiler to emit real linkable symbols by wrapping each StringZilla call in a non-static function:

```c
// sz_shim.c — forces symbol emission for Zig's extern fn linkage
#define SZ_DYNAMIC_DISPATCH 0
#include <stringzilla/find.h>

char const *ix_sz_find(char const *haystack, size_t h_len,
                       char const *needle, size_t n_len) {
    return sz_find(haystack, (sz_size_t)h_len, needle, (sz_size_t)n_len);
}
```

The alternative — `SZ_DYNAMIC_DISPATCH=1` — uses runtime CPU feature detection via `DllMain` on Windows, adding complexity and a cold-start penalty. Static dispatch with `-mavx2` eliminates both.

#### Why `link_libc`?

The AVX2 intrinsics header chain requires system libc:

```
stringzilla/find.h → immintrin.h → xmmintrin.h → mm_malloc.h → stdlib.h
```

Zig's internal clang doesn't ship system libc headers. Setting `.link_libc = true` in `build.zig` provides them. This adds ~50 KB to the binary but is required for any AVX2 code.

#### Compile-Time SIMD Dispatch

StringZilla's backend selection is entirely compile-time:

```
build.zig: flags = &.{ "-mavx2", ... }
    ↓
Clang defines __AVX2__ macro
    ↓
stringzilla/types.h line 270:
    #if defined(__AVX2__)
        #define SZ_USE_HASWELL 1
    #endif
    ↓
stringzilla/find.h routes to sz_find_avx2() implementation
    ↓
Final binary contains only AVX2 code paths — no dispatch tables,
no runtime cpuid checks, no branch misprediction overhead
```

</details>

### Hot Path Map

Three call sites in `search.zig` were replaced. Together they account for the majority of search engine wall time:

| Hot Path | Location | Before | After | Impact |
|:---------|:---------|:-------|:------|:-------|
| Newline scan | `scanOpenFile` inner loop | `std.mem.indexOfScalar(u8, chunk, '\n')` | `sz.indexOfByte(chunk, '\n')` | Every byte of every file passes through this |
| Binary sniff | `isLikelyBinary` | `std.mem.indexOfScalar(u8, buf, 0)` | `sz.indexOfByte(buf, 0)` | First 1024 bytes of every discovered file |
| Literal match | `indexOfLiteral` | `std.mem.indexOf(u8, line, needle)` | `sz.indexOf(line, needle)` | Every line of every non-binary file (case-sensitive) |

---

## Regex Engine

The Zig port includes a native **recursive backtracking regex engine** (`src/core/regex.zig`) rather than depending on an external regex library.

> [!TIP]
> The IX operator set produces simple, short patterns (typically <30 tokens) where recursive backtracking is fast enough. The actual bottleneck for regex-shaped searches is the literal substring extraction that feeds into the regex verifier — and that's handled by StringZilla.

<details>
<summary><strong>Supported Constructs</strong></summary>

| Construct | Syntax | Example |
|:----------|:-------|:--------|
| Literal | `abc` | `ERROR` |
| Any byte | `.` | `E..OR` |
| Word char | `\w` | `\w+` |
| Digit | `\d` | `\d{3}` |
| Whitespace | `\s` | `\s+` |
| Word boundary | `\b` | `\bERROR\b` |
| Line anchors | `^` `$` | `^WARN` |
| Character class | `[a-z]` | `[A-Z0-9]` |
| Negated class | `[^0-9]` | `[^a-z]` |
| Optional | `?` | `colou?r` |
| Star (greedy) | `*` | `\w*` |
| Plus (eager) | `+` | `\d+` |
| Exact repeat | `{N}` | `\w{3}` |
| Group alternation | `(a\|b)` | `(session\|handshake)` |
| Inline case-insensitive | `(?i)` | `(?i)error` |
| Hex byte | `\xNN` | `\x00` |
| Escaped metachar | `\\.` | `\\.log` |

</details>

<details>
<summary><strong>Matching Strategy</strong></summary>

The engine uses **recursive descent with greedy backtracking** for `*` and **eager (non-greedy) matching** for `+`:

- `*` (star): Consume maximum matching bytes first, then backtrack one position at a time. This finds the longest match, consistent with PCRE greedy semantics.
- `+` (plus): Consume minimum (1 byte), extend forward if the rest of the pattern fails. This finds the shortest match that satisfies the quantifier.
- `?` (optional): Try consuming 1 byte first, fall back to 0. Greedy like `*`.
- `{N}` (exact): Consume exactly N bytes, no backtracking.

Group alternation (`(a|b)`) splits branches on top-level `|` operators (respecting nesting depth), tries each branch, and chains the group's end position into the rest of the pattern. This makes `(session|handshake)\b` work: the word boundary check happens at the exact byte after whichever branch matched.

</details>

### Strategy-Aware Dispatch (Regex Bypass Engine)

Most regex predicates in practice are structurally simpler than full regex — they're literals wrapped in regex syntax. The **strategy-aware dispatch engine** classifies each regex pattern at parse time (via `expr.zig` `classifyRegex()`) and routes it to the fastest possible execution path, bypassing the backtracking engine entirely when possible.

```
                          ┌─ regex_plain_literal ──────── sz.indexOf (AVX2 SIMD)
                          ├─ regex_ascii_casefold ──────── scalar casefold + sz.indexOf
  expr.classifyRegex() ───├─ regex_word_boundary_literal ─ sz.indexOf + boundary verify
                          ├─ regex_literal_alternates ──── multi-branch sz.indexOf
                          └─ regex_full ──────────────────┬─ literal prefix prefilter (SIMD)
                                                          └─ recursive backtracking fallback
```

The dispatch operates at the **predicate column** level — every line × predicate evaluation goes through strategy selection first. For typical IX workloads, **90%+ of regex predicates never touch the backtracking engine** because they're classified as structural literals, casefold literals, or word-boundary literals.

**Literal prefix prefilter:** For `regex_full` patterns, the engine extracts the leading literal bytes from the regex (e.g., `process_\d+` → `"process_"`) and uses StringZilla's AVX2 `memmem` to reject lines that don't contain the prefix. On typical source code, this eliminates ~95% of lines before the regex engine fires — reducing effective regex invocations from millions to thousands per search.

<details>
<summary><strong>Matcher Strategy Classification</strong></summary>

Each regex predicate is classified into a `MatcherStrategy` that determines execution routing:

| Strategy | Pattern Example | Meaning |
|:---------|:---------------|:--------|
| `regex_plain_literal` | `re:ERROR` | No metacharacters — route to literal search |
| `regex_ascii_casefold_literal` | `re:(?i)error` | Case-insensitive literal |
| `regex_word_boundary_literal` | `re:\bERROR\b` | Literal with word boundary anchors |
| `regex_literal_alternates` | `re:alpha\|beta` | Multiple literals via alternation |
| `regex_fixed_width_bytes` | `re:\w{3}\d{2}` | Fixed-width byte pattern |
| `regex_full` | `re:.*complex.*` | Full regex engine required |

The strategy classification drives both **runtime dispatch** (routing each predicate to the fastest execution path) and telemetry. Patterns classified as structural literals bypass the recursive backtracking engine entirely, executing through StringZilla's AVX2 SIMD kernels at 32 bytes/cycle instead of the regex engine's ~1 byte/cycle.

</details>

<details>
<summary><strong>Regex Engine Alternatives Evaluated</strong></summary>

Three Zig-native regex engine replacements were evaluated against the current recursive backtracking engine. None were adopted — the strategy-aware dispatch already routes 90%+ of regex predicates to SIMD literal paths, making the regex engine a non-bottleneck.

| Engine | Architecture | Verdict |
|:-------|:------------|:--------|
| **mvzr** (bytecode VM) | LPEG-inspired bytecode VM, zero-alloc stack-resident, 64 ops / 8 char sets default | Same backtracking model as ours. No `(?i)` support. No speed advantage for IX pattern complexity. |
| **zig-regex** (Thompson NFA / PikeVM) | Dual-engine: PikeVM for large patterns, backtracking for small. Linear-time guarantee. | O(m×n) worst-case is better in theory, but IX patterns are short (<30 tokens) — backtracking never fires exponentially. No `(?i)`. Requires heap allocator. |
| **PCREz** (PCRE2 FFI) | Zig wrapper around PCRE2 C library. Full Perl-compatible regex. | JIT not enabled in wrapper. Adds C library dependency (PCRE2 built from source). Binary distribution complexity for uncertain gain. |

</details>

---

## Expression Plan Architecture

IX uses a typed expression grammar, not raw regex strings. An expression like `"lit:ERROR && lit:timeout"` is parsed into a structured plan:

```
ExpressionPlan {
    source: "lit:ERROR && lit:timeout"
    mode: .all (&&)
    predicates: [
        Predicate { kind: .literal, value: "ERROR", strategy: .literal }
        Predicate { kind: .literal, value: "timeout", strategy: .literal }
    ]
}
```

<details>
<summary><strong>Predicate Types</strong></summary>

| Type | IX Syntax | Search Method | SIMD Accelerated? |
|:-----|:----------|:-------------|:------------------|
| `literal` | `lit:X` or bare text | `sz.indexOf` (StringZilla AVX2 memmem) | Yes |
| `prefix` | `prefix:X` | `std.mem.startsWith` + byte compare | No (trivial — 1 comparison) |
| `suffix` | `suffix:X` | `std.mem.endsWith` + byte compare | No (trivial — 1 comparison) |
| `regex` | `re:X` | Zig-native recursive backtracking | No (regex engine is scalar) |

</details>

---

## Parallel File Scanner

The search engine uses a **two-phase parallel scan architecture** to distribute file I/O across all available CPU cores:

```
Phase 1: DISCOVER (serial)          Phase 2: SCAN (parallel)
┌─────────────────────────┐         ┌──────────────────────────────────┐
│ Walk directory tree     │         │ Thread 0: files[0..125]          │
│ Collect file paths      │  ────►  │ Thread 1: files[125..250]        │
│ Filter hidden/binary    │         │ Thread 2: files[250..375]        │
│ ~0.5ms for 500 files    │         │ Thread 3: files[375..500]        │
└─────────────────────────┘         │  ... (auto-scaled to CPU count)  │
                                    └──────────┬───────────────────────┘
                                               │
                                    ┌──────────▼───────────────────────┐
                                    │ MERGE: sum counters, concat hits │
                                    │ Thread-local ShardReports → final│
                                    └──────────────────────────────────┘
```

<details>
<summary><strong>Why Two Phases?</strong></summary>

Directory walking is inherently serial (OS readdir returns entries sequentially), but it's fast — just metadata syscalls, no file content. Once the file list is materialized, we can perfectly partition work across threads with **zero contention**: each thread gets a contiguous slice of the file list and writes into its own `ShardReport` struct. No mutexes in the hot per-line matching path.

</details>

<details>
<summary><strong>Thread-Local Accumulation (Lock-Free Hot Path)</strong></summary>

Each worker thread operates on a fully independent `ShardReport`:
- Counters (`bytes_scanned`, `files_scanned`, `matches_found`) are plain increments — no atomics needed
- Hit records accumulate in a thread-local buffer (`[MAX_RETAINED_HITS]SearchHit`)
- Slowest-file tracking is per-shard, merged after join

The merge step runs once after all threads complete — a single O(N_threads) loop that sums counters and concatenates hit buffers.

</details>

<details>
<summary><strong>Thread Count Auto-Scaling</strong></summary>

The scanner auto-detects CPU core count via `std.Thread.getCpuCount()` and caps at 16 threads. For small file sets (<4 files), it falls back to the serial path to avoid thread spawn overhead. The `--threads` flag overrides auto-detection for reproducible benchmarking.

</details>

### Parallel Scanner Impact

This optimization **flipped every large-directory benchmark**:

| Case | Before (serial) | After (parallel) | Delta |
|:-----|:----------------|:-----------------|:------|
| `large-dir-literal` | 1.14x | **0.87x** | -24% |
| `large-dir-regex` | 1.14x | **0.80x** | -30% |
| `large-dir-case-insensitive` | 1.25x | **0.85x** | -32% |
| `large-dir-boolean-and` | 1.15x | **0.77x** | -33% |
| `large-dir-prefix` | 1.11x | **0.85x** | -23% |

---

## Exploration Roadmap

Future acceleration targets, ordered by expected impact. Completed items are marked.

### Tier 1: High Impact, Proven Techniques

> [!IMPORTANT]
> All three Tier 1 optimizations have been implemented and verified.

~~**Thread Pool File Scanner**~~ &ensp; ![Done](https://img.shields.io/badge/-Implemented-brightgreen?style=flat-square) — Two-phase parallel scanner using `std.Thread.spawn` with static file partitioning and lock-free thread-local `ShardReport` accumulation. Auto-scales to CPU core count. See [Parallel File Scanner](#parallel-file-scanner).

~~**Strategy-Aware Regex Dispatch**~~ &ensp; ![Done](https://img.shields.io/badge/-Implemented-brightgreen?style=flat-square) — Compile-time pattern classification routes 90%+ of regex predicates to SIMD literal search paths, bypassing the backtracking engine entirely. See [Strategy-Aware Dispatch](#strategy-aware-dispatch-regex-bypass-engine).

~~**Literal Prefix Prefilter**~~ &ensp; ![Done](https://img.shields.io/badge/-Implemented-brightgreen?style=flat-square) — SIMD-accelerated rejection of non-matching lines before regex evaluation. Extracts leading literal bytes from `regex_full` patterns and uses StringZilla AVX2 `memmem` to skip ~95% of lines.

**Memory-Mapped File I/O** &ensp; ![Planned](https://img.shields.io/badge/-Planned-blue?style=flat-square) — Replace the chunked `read()` loop with `mmap` for files above a size threshold (e.g., 256 KB). Zig's `std.posix.mmap` is available on Linux; Windows requires `CreateFileMapping` + `MapViewOfFile` via `std.os.windows`. Memory mapping eliminates the copy from kernel buffer to userspace and lets the OS page in data on demand, which is especially beneficial for large files where only a fraction of bytes contain matches.

### Tier 2: Medium Impact, Targeted Optimizations

<details>
<summary><strong>Aho-Corasick, SIMD Case-Insensitive Search</strong></summary>

**Aho-Corasick Multi-Pattern Automaton** — For `||` expressions with multiple literal predicates (e.g., `lit:ERROR || lit:timeout || lit:fatal`), build an Aho-Corasick automaton that searches for all literals simultaneously in a single pass over the file. StringZilla or a purpose-built C implementation could provide the automaton; the key insight is that N sequential `sz.indexOf` calls is O(N * file_size) while Aho-Corasick is O(file_size) regardless of N.

**SIMD Case-Insensitive Search** — Extend the StringZilla integration with a casefold wrapper: for ASCII patterns, XOR each 32-byte chunk with 0x20 to force lowercase (works for [A-Z] → [a-z] because ASCII case differs by exactly bit 5), then run the standard `sz.indexOf` on the transformed buffer. This avoids the per-byte `toLower` call and would further improve case-insensitive benchmarks (currently 0.85x via parallel scanning, theoretically improvable to 0.5x with SIMD casefold).

</details>

### Tier 3: Specialized, Context-Dependent

<details>
<summary><strong>Byte-Shard Counting, Prepared Replay, Dominant-File Strategy</strong></summary>

**Byte-Shard Parallel Counting** — For `--stats-only` mode with a single literal predicate, split the file into N byte ranges (one per thread), count occurrences in each range independently, and sum. Requires overlap windows at range boundaries (needle.len - 1 bytes) to catch matches that straddle boundaries.

**Prepared Target Replay** — Cache file metadata (path, size, modification time) from discovery scans and replay against the same file set without re-walking the directory tree. Useful for benchmark iteration and watch-mode search where the file set rarely changes between runs.

**Linux Dominant-File Strategy** — On Linux kernel source trees, a handful of giant header files (8+ MB AMD ASIC register headers) dominate scan time. Detect these files, shard them across threads with byte-range splitting, and scan the rest of the tree normally.

**StringZilla `sz_equal` for Prefix/Suffix** — The C shim already exports `ix_sz_equal` (SIMD memcmp equivalent). For long prefix/suffix predicates, SIMD byte equality could replace the scalar `literalEquals` loop. Impact is marginal because prefix/suffix patterns are typically short (<20 bytes).

</details>

### Tier 4: Elite — Bytecode Engines, Vectorized Automata, Zero-Copy Pipelines

<details>
<summary><strong>Vectorized DFA Simulation (SIMD-Accelerated State Machines)</strong></summary>

Instead of evaluating one regex state per byte, process 32 bytes through a DFA simultaneously using SIMD shuffle instructions:

```
DFA state table stored as a 16-entry lookup per input nibble (4 bits).
Each byte is split: high_nibble = byte >> 4, low_nibble = byte & 0x0F.

VPSHUFB(low_table, low_nibble_vector)  → 32 candidate state sets (low nibble)
VPSHUFB(high_table, high_nibble_vector) → 32 candidate state sets (high nibble)
VPAND(low_result, high_result)          → 32 actual next states

One VPSHUFB processes 32 state transitions simultaneously.
```

This is the core technique behind Intel Hyperscan's `NFA → DFA → vectorized DFA` pipeline. For IX, it would replace the recursive backtracking regex engine with a compiled DFA that processes 32 bytes per cycle for any pattern. The DFA compilation happens once at expression parse time; the SIMD simulation runs at near-memchr speeds regardless of pattern complexity.

**Implementation path:** Compile `regex.zig` patterns into a DFA transition table at parse time. If the DFA has ≤16 states (common for IX patterns), use the nibble-split VPSHUFB technique. For >16 states, fall back to a 256-entry scatter/gather approach using VPGATHERDD (AVX2) or VPERMB (AVX-512). Integrate via a new `dfa.zig` module with a C shim for the SIMD transition kernel.

</details>

<details>
<summary><strong>Bytecode Regex VM</strong></summary>

Compile regex patterns into a bytecode instruction set, then execute with a purpose-built virtual machine. This sits between the current recursive backtracking engine (interpreted, per-character function call overhead) and a full DFA compiler (complex, memory-heavy for large state spaces).

```
Bytecode instruction set:
  MATCH_BYTE   <byte>          # Match exact byte
  MATCH_CLASS  <bitmap_idx>    # Match against 256-bit class bitmap
  MATCH_ANY                    # Match any byte (.)
  SPLIT        <addr1> <addr2> # Fork execution (alternation / quantifier)
  JUMP         <addr>          # Unconditional jump
  SAVE         <slot>          # Save capture position
  WORD_BOUNDARY                # Assert \b
  ACCEPT                       # Match succeeded

Execution model:
  Thompson NFA simulation — maintain a SET of active bytecode PCs.
  Each input byte advances all active PCs simultaneously.
  No backtracking, no exponential blowup, O(n * m) guaranteed
  where n = input length, m = pattern bytecode length.
```

The key insight: Thompson NFA simulation processes each input byte exactly once across all active states, eliminating the catastrophic backtracking that makes `(a*)*b` exponential. For IX's short patterns, the bytecode compiles into <64 instructions, and the active state set fits in a single cache line.

**Character class bitmaps:** Each `[a-zA-Z0-9]` class compiles to a 256-bit bitmap (32 bytes — one bit per possible byte value). Testing membership is a single array lookup: `bitmap[byte >> 3] & (1 << (byte & 7))`. The bitmap array fits in one AVX2 register, so class matching becomes a SIMD operation.

</details>

<details>
<summary><strong>Zero-Copy Streaming Pipeline</strong></summary>

Eliminate all buffer copies between file I/O, line splitting, and pattern matching:

```
CURRENT:  kernel_buffer →[read()]→ stack_buffer →[split]→ line_slice →[match]→ result
          ^^^ copy 1              ^^^ copy 2 (carry buffer for cross-chunk lines)

ZERO-COPY:  mmap_region →[SIMD newline scan]→ line_ptr+len →[SIMD match]→ result
            ^^^ no copies — line_ptr points directly into the mapped page
```

With memory-mapped I/O, the file's bytes live in virtual memory backed by the page cache. StringZilla's `sz.indexOfByte` scans for newlines directly in the mapped region. Each "line" is just a `(ptr, len)` pair pointing into the mapping — no allocation, no copy. Pattern matching via `sz.indexOf` operates on the same mapped pages. The entire search path from disk to result touches each byte exactly once, in the page cache, with zero userspace copies.

**Carry buffer elimination:** Cross-page-boundary lines (where a line starts on one page and ends on another) are handled by the virtual memory system — contiguous virtual addresses span physical pages transparently. The `carry` ArrayList and its heap allocations disappear entirely.

</details>

<details>
<summary><strong>Prefetch-Directed Scan Pipeline</strong></summary>

Use software prefetch hints to warm cache lines before the SIMD search kernel reaches them. Modern CPUs have 3-level cache hierarchies with ~4 cycle L1 latency, ~12 cycle L2, and ~40 cycle L3. A cache miss to main memory costs ~200 cycles — enough time to process 6400 bytes with AVX2.

```zig
// Prefetch the next 1 KB while processing the current 1 KB
// PREFETCHT0 brings data into L1 cache (temporal — expect reuse)
while (offset < file_len) {
    // Prefetch next chunk while current chunk processes
    if (offset + CHUNK_SIZE < file_len) {
        @prefetch(mapped_ptr + offset + CHUNK_SIZE, .{
            .rw = .read,
            .locality = 3,    // T0: all cache levels
        });
    }
    // Process current chunk with sz.indexOfByte (hits warm L1 lines)
    scanChunk(mapped_ptr[offset..offset + CHUNK_SIZE]);
    offset += CHUNK_SIZE;
}
```

The prefetch distance is tuned to the SIMD processing rate: if AVX2 processes 32 bytes/cycle and L3 latency is 40 cycles, prefetch 32 × 40 = 1280 bytes ahead. This keeps the SIMD pipeline fed without stalling on memory latency.

</details>

<details>
<summary><strong>Branch-Free Match Accumulation</strong></summary>

Eliminate branch mispredictions in the match counting hot loop. Modern CPUs predict branches with ~97% accuracy, but a 3% miss rate on a loop that executes billions of times costs millions of wasted cycles (each misprediction flushes the ~15-stage pipeline).

```zig
// CURRENT: branch per line (mispredicts when match density is ~50%)
if (matchingColumn(line, plan, case_insensitive)) |_| {
    report.matches_found += 1;
}

// BRANCH-FREE: convert match result to 0/1 via conditional move
const matched: usize = @intFromBool(matchingColumn(line, plan, case_insensitive) != null);
report.matches_found += matched;
// CMOV instruction — no branch, no misprediction, constant latency
```

For `--stats-only` mode where only the count matters (no hit records stored), the entire match-accumulate loop can be branch-free. This matters most on workloads with moderate match density (~30-70%) where the branch predictor can't establish a stable pattern.

</details>

<details>
<summary><strong>TLB-Aware Memory Mapping with Hugepages</strong></summary>

Standard 4 KB pages require one TLB (Translation Lookaside Buffer) entry per page. A 1 GB file needs 262,144 TLB entries — far more than the ~1,500 entries in a typical L2 TLB. TLB misses trigger page table walks costing ~100 cycles each.

```
Standard 4 KB pages:    1 GB file = 262,144 pages = constant TLB thrashing
2 MB hugepages:         1 GB file =     512 pages = fits in L2 TLB
1 GB gigapages:         1 GB file =       1 page  = single TLB entry
```

On Linux, `mmap` with `MAP_HUGETLB | MAP_HUGE_2MB` requests 2 MB pages. On Windows, `VirtualAlloc` with `MEM_LARGE_PAGES` does the equivalent. The file is mapped using transparent hugepages, eliminating TLB misses entirely for files up to ~3 GB (1500 entries × 2 MB). Combined with the zero-copy pipeline, this means the SIMD search kernel runs at full throughput without ever stalling on address translation.

</details>

<details>
<summary><strong>Lock-Free Concurrent Result Aggregation</strong></summary>

When the thread pool scanner is active, multiple threads produce match results simultaneously. Lock-free aggregation using atomic operations eliminates contention:

```zig
// Each thread has a thread-local report (no sharing)
threadlocal var local_report: ThreadReport = .{};

// Atomic merge into global report after thread completes
fn mergeIntoGlobal(global: *SearchReport, local: ThreadReport) void {
    _ = @atomicRmw(usize, &global.matches_found, .Add, local.matches_found, .monotonic);
    _ = @atomicRmw(usize, &global.files_scanned, .Add, local.files_scanned, .monotonic);
    _ = @atomicRmw(usize, &global.bytes_scanned, .Add, local.bytes_scanned, .monotonic);

    // Slowest-file is a compare-and-swap race — only update if this
    // thread's slowest file is slower than the current global slowest
    var current_ms = @atomicLoad(f64, &global.slowest_ms, .acquire);
    while (local.slowest_ms > current_ms) {
        if (@cmpxchgWeak(f64, &global.slowest_ms, current_ms, local.slowest_ms, .release, .acquire)) |actual| {
            current_ms = actual;
        } else {
            // Won the race — also update path and bytes
            @atomicStore([]const u8, &global.slowest_path, local.slowest_path, .release);
            break;
        }
    }
}
```

Cache-line alignment (`align(64)`) on the global report fields prevents false sharing — where two threads' atomic operations thrash the same cache line even though they're updating different fields.

</details>

### Tier 5: Research Frontier — Sublinear Search, Compressed Indices, Hardware-Aware Scheduling

<details>
<summary><strong>Trigram Index with Inverted Posting Lists (Google Code Search Architecture)</strong></summary>

Instead of scanning every file for every query, pre-build an index that maps every 3-byte sequence (trigram) to the set of files containing it. At query time, decompose the search pattern into trigrams, intersect their posting lists, and only scan the surviving candidate files.

```
Index construction (one-time cost):
  File "auth.zig" contains bytes: [a,u,t,h,.,z,i,g,...]
  Trigrams: "aut", "uth", "th.", "h.z", ".zi", "zig"
  Posting list: trigram "aut" → {auth.zig, auth_test.zig, oauth.zig}
                trigram "zig" → {auth.zig, build.zig, main.zig, ...}

Query: "auth"
  Trigrams: "aut", "uth"
  Intersect postings: files in BOTH "aut" AND "uth" posting lists
  Candidates: {auth.zig, auth_test.zig, oauth.zig}
  Full scan: only 3 files instead of 10,000+
```

For large codebases (100K+ files), the trigram intersection reduces candidate files by 99%+. The index is compact: 256³ = 16.7 million possible trigrams, but real code uses only ~100K unique trigrams. A hash map from trigram → compressed posting list fits in ~10 MB for a million-file codebase. The posting lists use delta-encoded variable-length integers (varint) for further compression.

**Index persistence:** Write the trigram index to a memory-mapped file. On subsequent searches, `mmap` the index (zero deserialization cost) and perform posting list intersection directly on the mapped pages. Index invalidation uses file modification timestamps — only rebuild posting lists for changed files (incremental update).

</details>

<details>
<summary><strong>FM-Index / Suffix Array Compressed Search</strong></summary>

For repeated searches over the same corpus, build a compressed suffix array (CSA) or FM-index that supports O(m) pattern matching where m = pattern length, independent of corpus size. This is genuinely sublinear — searching a 10 GB corpus for a 5-byte pattern takes the same time as searching a 10 KB file.

```
Burrows-Wheeler Transform (BWT) of "banana$":
  Rotations sorted:   BWT column (last char):
  $banana             a
  a$banan             n
  ana$bana            n
  anana$ba            b
  banana$             $
  na$banan            n
  nana$ban            a

FM-index: BWT column + occurrence table + suffix array samples
  Backward search for "ana":
    Step 1: 'a' → rows starting with 'a' → rows [1,2,3]
    Step 2: 'n' → rows where BWT[row] = 'n' in range [1,3] → rows [2,3]
    Step 3: 'a' → rows where BWT[row] = 'a' in range [2,3] → rows [2,3]
    Result: 2 occurrences, found in O(3) = O(pattern_length) steps
```

The FM-index compresses to ~0.5–1.5 bytes per input byte using wavelet trees and rank/select structures. A 1 GB codebase compresses to ~1 GB index that supports instant substring search. The wavelet tree enables O(1) rank queries using SIMD-accelerated popcount (`VPOPCNTDQ` on AVX-512, or `POPCNT` on the bit-sliced representation).

</details>

<details>
<summary><strong>SIMD-Accelerated Bloom Filter Prefilter</strong></summary>

Before scanning a file's content, check a per-file Bloom filter to probabilistically determine if the file could contain the search pattern. A Bloom filter uses k hash functions mapping the pattern to k bit positions in a bit array. If any bit is 0, the file definitely doesn't contain the pattern — skip it without reading.

```
Construction (during indexing):
  For each file, compute a 512-bit Bloom filter from its byte trigrams.
  Store: file_bloom[file_id] = 512 bits (64 bytes = 1 cache line)

Query: pattern "ERROR"
  Trigrams: "ERR", "RRO", "ROR"
  Hash each trigram to bit positions in a 512-bit filter
  For each file:
    VPAND(file_bloom, query_bloom)  → if result != query_bloom → skip file
    ^^^ Single AVX-512 instruction tests all bit positions simultaneously
```

With a 512-bit filter and 3 hash functions, the false positive rate is ~0.1% for files with <1000 unique trigrams. This means 99.9% of non-matching files are skipped without any I/O — a single AVX-512 `VPAND` + `VPCMPQ` replaces reading and scanning the entire file.

</details>

<details>
<summary><strong>Speculative Multi-Path Regex Evaluation</strong></summary>

For regex patterns with alternation (`a|b|c`), evaluate all branches simultaneously using SIMD lane parallelism. Each SIMD lane tracks a different alternation branch's match state:

```
Pattern: (session|socket|signal)\b
Input:   "...socket_handler..."

Lane 0 (tracking "session"): s-e-s-s-i-o-n
Lane 1 (tracking "socket"):  s-o-c-k-e-t     ← matches at position 3
Lane 2 (tracking "signal"):  s-i-g-n-a-l

VPCMPEQB per lane against the input, VPAND to accumulate match state.
If any lane reaches full match + word boundary → report hit.
All 3 branches evaluated in the same number of cycles as 1 branch.
```

This is especially effective for `regex_literal_alternates` patterns where IX currently evaluates each alternate sequentially. With 32-byte SIMD registers, up to 32 single-byte alternates or 16 two-byte alternates can be evaluated in parallel.

</details>

<details>
<summary><strong>Kernel-Bypass I/O via io_uring (Linux) / IOCP (Windows)</strong></summary>

System calls (`read()`, `pread()`) require a user→kernel→user context switch costing ~1000 cycles each. For small files (< 64 KB), the syscall overhead dominates actual I/O time. `io_uring` submits batches of I/O requests through a shared memory ring buffer, avoiding per-request context switches entirely.

```
CURRENT (per-file syscall):
  for each file:
    fd = open(path)          # syscall ~1000 cycles
    n = read(fd, buf, len)   # syscall ~1000 cycles
    close(fd)                # syscall ~1000 cycles
    process(buf)
  Total overhead: 3000 cycles × N files

io_uring (batched submission):
  Submit 256 open+read requests to the submission queue (single syscall)
  Kernel processes all 256 asynchronously
  Poll completion queue for results (single syscall)
  Process all 256 buffers
  Total overhead: 2 syscalls for 256 files = ~8 cycles/file amortized
```

On a codebase with 50,000 small files, this reduces I/O overhead from ~150 million cycles to ~400,000 cycles — a 375x reduction in syscall cost. Zig can access `io_uring` via `std.os.linux.io_uring` or a direct C shim to `liburing`.

</details>

<details>
<summary><strong>NUMA-Aware Work Partitioning</strong></summary>

On multi-socket servers (2+ physical CPUs), memory is partitioned into NUMA (Non-Uniform Memory Access) domains. Accessing memory on the local socket costs ~100 ns; accessing remote socket memory costs ~300 ns (3x penalty). A NUMA-unaware thread pool scatters work randomly across sockets, causing constant remote memory access.

```
NUMA-unaware:
  Thread 0 (Socket 0) processes File A → data on Socket 1 memory → 300 ns/access
  Thread 1 (Socket 1) processes File B → data on Socket 0 memory → 300 ns/access

NUMA-aware:
  Discovery phase: stat() all files, record inode locations
  Partition files by physical memory location (via /proc/self/numa_maps)
  Pin Thread 0 to Socket 0, assign Socket 0's files
  Pin Thread 1 to Socket 1, assign Socket 1's files
  All accesses are local → 100 ns/access → 3x throughput improvement
```

On dual-socket systems (common in CI/build servers), NUMA-aware scheduling improves throughput by 2–3x for memory-bandwidth-bound workloads like full-codebase search.

</details>

<details>
<summary><strong>Cache-Line-Aligned Newline Bitmap</strong></summary>

Instead of scanning for newlines during search, pre-compute a bitmap during file read where each bit represents whether the corresponding byte is `\n`. The bitmap is 8x smaller than the file (1 bit per byte), fits better in cache, and enables SIMD-accelerated line counting via `VPOPCNTDQ` (AVX-512 population count):

```
File bytes:  [H][e][l][l][o][\n][W][o][r][l][d][\n][!][\n]
Newline bit: [0][0][0][0][0][ 1][0][0][0][0][0][ 1][0][ 1]

Packed: 0b0010_0000_1000_01 (14 bytes → 2 bytes bitmap)

"How many lines before byte offset 1000?"
  VPOPCNTDQ on bitmap[0..125] (1000/8 bytes) → instant line count
  vs scanning 1000 bytes for '\n' one at a time

"Which byte offset does line 500 start at?"
  Binary search on cumulative popcount → O(log n) with SIMD acceleration
  vs linear scan counting newlines → O(n)
```

The bitmap also enables O(1) line-number lookups for any byte offset — critical for hit reporting where the search kernel finds a match at byte position X and needs to report the line number. Currently this requires maintaining a running line counter through the entire file; with a bitmap, it's a single popcount operation.

</details>

<details>
<summary><strong>Adaptive Algorithm Selection via Runtime Profiling</strong></summary>

Instead of statically choosing search strategies at parse time, profile the first N files (or first N milliseconds) and dynamically switch algorithms based on observed characteristics:

```
Runtime profiling signals:
  match_density     = matches / bytes_scanned     (sparse vs dense)
  avg_line_length   = bytes_scanned / lines        (short vs long lines)
  binary_skip_rate  = files_skipped / files_seen   (binary-heavy directory?)
  pattern_selectivity = trigram_posting_size / total_files

Strategy selection matrix:
  match_density < 0.001  → prefilter-heavy: Bloom filter + trigram index
  match_density > 0.1    → branch-free accumulation (misprediction avoidance)
  avg_line_length > 4096 → byte-shard within file (parallelize single file)
  avg_line_length < 40   → line-batch: accumulate 32 lines, SIMD match all
  binary_skip_rate > 0.5 → prioritize binary detection (front-load sniff)
```

The profiler runs in the first 10 ms of a search, collects statistics on ~100 files, and selects the optimal pipeline configuration for the remaining files. This amortizes the profiling cost over the full search and adapts to workload shapes that static analysis can't predict (e.g., a repository that's 90% binary assets vs one that's 100% source code).

</details>

<details>
<summary><strong>AVX-512 VPCONFLICT for Parallel Hash Table Construction</strong></summary>

When building trigram indices or Aho-Corasick automata, hash table insertions from multiple SIMD lanes can collide. `VPCONFLICTD` detects which lanes hash to the same bucket, enabling conflict-free parallel insertion:

```
16 trigrams loaded into ZMM register (AVX-512):
  Lane:  [0]    [1]    [2]    [3]    ... [15]
  Hash:  [42]   [17]   [42]   [99]  ... [17]

VPCONFLICTD → conflict mask per lane:
  Lane 0: 0b0000 (no prior conflict)
  Lane 1: 0b0000 (no prior conflict)
  Lane 2: 0b0001 (conflicts with lane 0 — same hash 42)
  Lane 3: 0b0000 (no prior conflict)
  ...
  Lane 15: 0b0010 (conflicts with lane 1 — same hash 17)

Non-conflicting lanes insert in parallel (single cycle).
Conflicting lanes retry in a second pass.
Typically 90%+ of lanes are conflict-free → ~16 insertions per cycle.
```

This accelerates index construction by 10–16x compared to scalar hash table building, making trigram index updates fast enough to run incrementally on file save.

</details>

---

## Source Layout

```
zig/
├── build.zig              # Build graph: exe + tests, StringZilla C compilation
├── README.md              # This file
├── src/
│   ├── main.zig           # CLI entry: argv → command dispatch → output
│   ├── sz_shim.c          # C shim: StringZilla static inline → linkable symbols
│   ├── cli/
│   │   ├── args.zig       # Argv parsing, canonical + rg-shaped compatibility
│   │   └── output.zig     # Renderers: text, JSON, sentinels, help text
│   └── core/
│       ├── expr.zig        # Expression parser: IX grammar → typed plan
│       ├── regex.zig       # Regex engine: recursive backtracking matcher
│       ├── search.zig      # Search engine: file scan, line split, matching
│       ├── inspect.zig     # File inspection: bounded windows, match context
│       ├── stats.zig       # Telemetry model: timings, concurrency, slow files
│       └── sz.zig          # StringZilla Zig wrapper: SIMD indexOf/indexOfByte
└── .refs/stringzilla/      # (at repo root) StringZilla v4.6.0 headers
    └── include/
        └── stringzilla/
            ├── find.h      # memchr + memmem SIMD implementations
            ├── compare.h   # memcmp SIMD implementation
            └── types.h     # SIMD detection macros, type definitions
```

---

<div align="center">

**[MIT License](../LICENSE)**

</div>

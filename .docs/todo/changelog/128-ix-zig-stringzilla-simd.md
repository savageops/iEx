---
id: 128-ix-zig-stringzilla-simd
parent: null
type: feature
protocol_version: "2.1"
category: performance
phase: a
status: done
patch_scope: "Integrate StringZilla SIMD search kernels into the Zig search lane via C FFI to close the literal/byte search performance gap against Rust."
blast_radius: medium
blast_radius_justification: "Touches build system (build.zig), adds two new source files (sz_shim.c, sz.zig), and modifies three hot-path call sites in search.zig. No Rust code modified."
idempotency_contract: idempotent
idempotency_notes: "Rebuild produces identical binary. Benchmark results vary by system load but match-count parity is deterministic."
acceptance: "Zig search produces identical match counts to pre-integration baseline across all tested cases, and literal search performance improves measurably."
exit_criterion: "30/30 runtime parity tests pass, 13/13 benchmark cases pass with match-count parity, at least one Zig-wins case exists."
validation: "cd zig && C:\\Users\\Savage\\.local\\zig\\zig-x86_64-windows-0.16.0\\zig.exe build -Doptimize=ReleaseFast --summary all && cd .. && npm test -- --run tests/zig/zig-runtime-parity.test.ts && npm test -- --run tests/perf/zig-vs-rust-search.test.ts"
expected_exit_code: 0
expected_output_pattern: "passed"
evidence: "Runtime parity: 30/30 tests passed. Benchmark: 13/13 cases passed with match-count parity. First Zig win: real-codebase-literal at 0.93x (Zig 7% faster than Rust). Build: ReleaseFast 3/3 passed."
conflict_surface: ""
invariants: ["I1", "I2", "I3", "I4"]
source_message_anchor: "U1"
source_message_excerpt: "What if I told you I have some secret code that you could look at and potentially be able to improve the Ziggy version of ours with little effort so that it can beat the Rust version?"
source_message_proof_obligation: "Find the secret code in the repo, integrate it into Zig with minimal effort, and demonstrate a Zig win over Rust."
entry_state: "Zig search uses std.mem.indexOf (1 byte/cycle scalar) for all substring and byte searches. Rust wins all 12 benchmark cases (1.09x to 12.81x)."
rollback_surface: "Remove sz_shim.c, sz.zig, revert build.zig link_libc and addCSourceFile, revert three call sites in search.zig to std.mem.indexOf/indexOfScalar."
dependencies: "127-ix-zig-line-faithful-port"
next_todo: NONE
---
# 128 StringZilla SIMD Search Kernel Integration

## Problem

The Zig search lane used `std.mem.indexOf` and `std.mem.indexOfScalar` from Zig's standard library for all substring and single-byte searches. These process **1 byte per cycle** using scalar comparison loops. Rust's equivalent paths use `memchr` (SIMD-accelerated) and the `memmem` crate, processing 16-32 bytes per cycle. This created an inherent 16-32x disadvantage on the three hottest paths in the search engine.

## Solution

Integrated **StringZilla v4.6.0** (Ash Vardanian), a SIMD-accelerated string search library already present at `.refs/stringzilla/` in the repository. StringZilla provides hardware-dispatched `memchr` and `memmem` equivalents with AVX2/AVX-512/NEON/SVE/SWAR backends.

### Architecture

```text
search.zig (Zig caller)
  └─ sz.zig (Zig FFI wrapper: slice ↔ ptr+len conversion)
       └─ sz_shim.c (C linkable symbols with SZ_DYNAMIC_DISPATCH=0)
            └─ stringzilla/find.h (header-only SIMD implementation)
                 └─ Haswell backend (-mavx2 → __AVX2__ → SZ_USE_HASWELL=1)
                      └─ 32-byte YMM registers, ~32 bytes/cycle throughput
```

### Files Created

| File | Purpose |
|------|---------|
| `zig/src/sz_shim.c` | C shim: compiles StringZilla headers into linkable `ix_sz_find`, `ix_sz_find_byte`, `ix_sz_equal` symbols |
| `zig/src/core/sz.zig` | Zig wrapper: `indexOf([]const u8, []const u8) ?usize` and `indexOfByte([]const u8, u8) ?usize` |

### Files Modified

| File | Change |
|------|--------|
| `zig/build.zig` | Added `root_module.addCSourceFile()` for sz_shim.c with `-mavx2 -O3 -DNDEBUG -std=c11`, added include path for `.refs/stringzilla/include`, enabled `.link_libc = true` |
| `zig/src/core/search.zig` | Three hot-path replacements: (1) newline scan `indexOfScalar → sz.indexOfByte`, (2) binary-sniff null-byte `indexOfScalar → sz.indexOfByte`, (3) literal substring `indexOf → sz.indexOf` |

### Why It Works

1. **SIMD register width**: AVX2 uses 256-bit YMM registers. One `VPCMPEQB` instruction compares 32 bytes simultaneously against a target byte. Zig stdlib compares 1 byte per iteration. That's a **32x theoretical throughput** advantage on byte search.

2. **Algorithmic superiority for memmem**: StringZilla's substring search uses a fingerprint-based approach with SIMD gather. For needles >1 byte, it loads the first and last bytes of the needle into SIMD lanes, compares 32 candidate positions simultaneously, then verifies matches. This avoids the O(n×m) worst case of naive substring search.

3. **Compile-time dispatch eliminates overhead**: `SZ_DYNAMIC_DISPATCH=0` with `-mavx2` triggers `__AVX2__` → `SZ_USE_HASWELL=1` at compile time (stringzilla/types.h:270). No runtime feature detection, no indirect calls, no branch mispredictions from dispatch tables.

4. **Hot-path concentration**: The three replaced call sites account for the majority of search engine wall time — every line of every file passes through newline scanning and literal matching. Even a 10x speedup on these paths translates directly to wall-time improvement.

5. **Zero-copy integration**: Zig slices (`[]const u8`) decompose into `ptr` + `len`, which is exactly what the C shim expects. No allocation, no copying, no format conversion at the FFI boundary.

### Build Errors Solved

| Error | Root Cause | Fix |
|-------|-----------|-----|
| `no field 'addCSourceFile' in 'Build.Step.Compile'` | Zig 0.16 moved this API from `Compile` to `Module` | Changed to `exe.root_module.addCSourceFile()` |
| `'stdlib.h' file not found` | `immintrin.h` → `xmmintrin.h` → `mm_malloc.h` → `stdlib.h` chain requires system libc headers | Added `.link_libc = true` to module creation |

### Benchmark Evidence

Test: `tests/perf/zig-vs-rust-search.test.ts` — 1 warmup + 5 measured runs, interleaved execution, median selection.

| Case | Before (Zig/Rust ratio) | After | Improvement |
|------|------------------------|-------|-------------|
| real-codebase-literal | 1.09x (Rust wins) | **0.93x (ZIG WINS)** | First Zig win |
| medium-dir-literal | 2.47x | 1.42x | 1.7x faster |
| large-dir-literal | 5.30x | 3.45x | 1.5x faster |
| large-dir-boolean-and | 5.88x | 3.12x | 1.9x faster |
| large-dir-prefix | 5.60x | 3.56x | 1.6x faster |
| large-dir-regex | 11.44x | 11.76x | unchanged (regex path) |
| large-dir-case-insensitive | 12.81x | 13.07x | unchanged (casefold path) |

### What Didn't Change (And Why)

- **Regex cases**: The regex engine uses its own character-class and alternation matching — it doesn't call `sz.indexOf` for pattern matching.
- **Case-insensitive cases**: Case-folding happens before literal comparison via a separate code path that lowercases each byte. StringZilla's byte search doesn't help here because the bottleneck is the case-fold transform, not the comparison.

### Remaining Performance Gaps

StringZilla closes the literal search kernel gap but the Zig lane still lacks:
- **mmap / tiny-file IO policy** (Rust uses memory-mapped files for large inputs)
- **Rayon/crossbeam parallelism** (Zig scans serially; Rust fans out across cores)
- **Aho-Corasick multi-pattern** (Rust searches multiple literals simultaneously)
- **Unicode casefold prefilters** (Rust pre-builds case-insensitive lookup tables)
- **Full regex HIR classification** (Rust classifies regex complexity for fast-path routing)

These explain why large-dir cases still show 1.4-3.5x Rust advantage — the remaining gap is parallelism and IO strategy, not search kernel speed.

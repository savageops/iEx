# IX Developer Inspection Command Surface

IX now owns read-only code inspection primitives beside search. The goal is to let agents replace shell-specific file-reading fragments with one native command family while preserving the existing search hot path.

## Command Taxonomy

```text
ix search <expr> [PATH]...      // hit records plus one terminal result sentinel
ix matches <expr> [PATH]...     // hit records only, same search engine
ix inspect <PATH>... [bounds]   // read-only grouped file windows, default bounded first window
ix inspect --expr <expr> ...    // grouped search-backed match context
ix explain <expr>               // expression plan JSON
ix PATTERN [PATH]...            // rg-shaped translator into canonical search
```

`ix inspect` is read-only. Replacement, write, in-place editing, shell execution, PowerShell delegation, and sed delegation are boundary violations for this surface. A future transform command can own mutation with a separate preview/apply contract.

The top-level translator is intentionally narrower than ripgrep. It accepts `PATTERN`, `-e PATTERN`, repeated `-e`, `-F`, `-i`, `-j`, `-n`, `--json`, and `--hidden`; accepted input lowers into `SearchArgs` and then uses the same `ix search` execution path. Unsupported flags fail with guided syntax. Raw rg-shaped regex patterns containing `&&` or `||` fail as ambiguous because those tokens are native IX boolean operators; explicit IX boolean composition belongs on `ix search <expr> [PATH]...`.

## Shell Workflow Mapping

```text
Get-Content file
ix inspect file

Get-Content file -TotalCount 40
ix inspect file --total-count 40

Get-Content file | Select-Object -Skip 120 -First 30
ix inspect file --skip 120 --limit 30

sed -n '40,80p' file
ix inspect file --range 40:80

Select-String -Path src -Pattern SearchConfig -Context 2
ix inspect --expr 'lit:SearchConfig' src --context 2

Select-String -Path src -Pattern 'TODO|FIXME' -Context 1 | ConvertTo-Json
ix inspect --expr 're:TODO|FIXME' src --context 1 --json
```

## Output Shape

Default `ix search` output keeps hit records first, then emits one versioned JSON sentinel for the terminal command state:

```text
path:line:column:preview
-- ix.result.v1 {"cmd":"search","status":"ok","expr":"expr","matches":N,"files":{"discovered":N,"scanned":N,"skipped":N},"bytes":N,"ms":{"discover":N.NNN,"scan":N.NNN,"aggregate":N.NNN,"total":N.NNN},"slowest":{"path":"path","ms":N.NNN,"bytes":N},"dedupe":{"overlap_pruned_roots":N,"discovered_duplicate_paths":N}} --
```

`ix matches` intentionally keeps the hit-record-only stream. It is the pipe contract and must not receive terminal result sentinels.

Zero-match search is a successful terminal state represented as `status:"ok"` with `matches:0`. Agents should read the `expr` field before interpreting a zero count, especially when distinguishing native `ix search "a|b"` literal semantics from top-level `ix "a|b"` compatibility regex lowering. `slowest` is an object when a file was scanned and `null` when no scanned file exists.

`ix.error.v1` is reserved for structured unsupported/error boundaries where the CLI can emit a narrow machine-readable record without rewiring the global error path. It is an agent boundary, not a fallback execution path.

Raw process stdout is the sentinel-count authority. If a transcript surface repeats an identical terminal `ix.result.v1` row while raw stdout contains one row, the transcript adapter should collapse the exact duplicate before model consumption.

Default file-window output is grouped by file to avoid repeating the same path on every emitted line:

```text
== ix.inspect.file path="path" request=start:end range=start:end emitted=N eof=false ==
line | text
-- ix.next.v1 {"cmd":"inspect","argv":["ix","inspect","path","--start-line","next","--limit","N"]} --
-- ix.next.v1 {"cmd":"inspect","argv":["ix","inspect","path","--range","next:next+span-1"]} --
== ix.inspect.file path="path" request=start:end emitted=0 eof=true total_lines=N ==
```

If a file-window command omits bounds, CLI lowering injects a 240-line limit before calling the core reader. Files at or below that horizon render completely and carry `eof=true total_lines=N`; larger files render a bounded first window and a parseable continuation vector. `--all` remains the explicit opt-in for full or tail-shaped unbounded reads.

The `ix.next.v1` footer is emitted only when the displayed file window exactly fills its requested bound and EOF was not reached. Limit-shaped reads continue with `--start-line` and `--limit`; range-shaped reads continue with a concrete `--range` window of the same span. `eof:true` is a terminal file-window assertion; when the inspect scan reaches EOF, `total_lines` gives agents the final line cardinality without a follow-up empty-range probe.

Default match-context output uses a count-only file header because context lines can span disjoint ranges:

```text
== ix.inspect.context path="path" emitted=N ==
line role    | text
```

Record-compatible output remains available when a pipe consumer requires one record per line:

```text
ix inspect file --range 40:80 --format records
path:line:text

ix inspect --expr 'lit:SearchConfig' src --context 2 --format records
path:line:match:text
path:line:context:text
```

JSON file windows emit:

```json
{
  "reports": [
    {
      "path": "src/main.rs",
      "requested": { "start_line": 1, "end_line": null, "skip": 0, "limit": 40, "allow_full": false },
      "total_emitted_lines": 40,
      "eof": false,
      "total_lines": null,
      "lines": [{ "line": 1, "text": "..." }]
    }
  ]
}
```

JSON match context emits:

```json
{
  "expression": "lit:SearchConfig",
  "reports": [
    {
      "path": "src/main.rs",
      "lines": [{ "line": 10, "role": "match", "text": "..." }]
    }
  ]
}
```

## Help Contract

The CLI help surface is part of the operator contract, not fallback text:

```text
ix --help             // command taxonomy, expression grammar, translator subset, agent output sentinels
ix search --help      // native expression rules plus terminal result sentinel contract
ix matches --help     // same expression rules, hit-record-only output
ix inspect --help     // read-only inspection modes, formats, and ix.next.v1 continuation rules
ix explain --help     // ExpressionPlan JSON and parser parity with search/matches
```

## Ownership

- `crates/iex-core/src/inspect.rs` owns bounded UTF-8 file-window extraction.
- `crates/iex-cli/src/agent_output.rs` owns versioned agent sentinels: `ix.result.v1`, `ix.next.v1`, and narrow `ix.error.v1` records.
- `crates/iex-cli/src/inspect.rs` owns inspection grammar and grouped/records/JSON rendering.
- `crates/iex-cli/src/search.rs` owns both `search` and `matches`, with `matches` reusing the same `run_search` path.
- `crates/iex-cli/src/compat.rs` owns rg-shaped compatibility translator lowering.
- `crates/iex-cli/src/main.rs` owns only command enumeration and dispatch.

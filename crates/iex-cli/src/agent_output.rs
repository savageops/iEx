use iex_core::SearchReport;
use serde_json::{json, Value};

pub(crate) const RESULT_V1: &str = "ix.result.v1";
pub(crate) const NEXT_V1: &str = "ix.next.v1";
pub(crate) const ERROR_V1: &str = "ix.error.v1";

pub(crate) fn json_string(value: &str) -> String {
    serde_json::to_string(value).expect("string serialization should not fail")
}

pub(crate) fn normalize_display_path(path: &str) -> String {
    path.replace('\\', "/")
}

pub(crate) fn render_search_result_v1(report: &SearchReport) -> String {
    render_sentinel(RESULT_V1, search_result_payload(report))
}

pub(crate) fn render_next_v1(argv: &[String]) -> String {
    render_sentinel(
        NEXT_V1,
        json!({
            "cmd": "inspect",
            "argv": normalize_argv_paths(argv),
        }),
    )
}

pub(crate) fn render_error_v1(code: &str, message: &str, hint: Option<&str>) -> String {
    render_sentinel(
        ERROR_V1,
        json!({
            "cmd": "ix",
            "status": "error",
            "code": code,
            "severity": "error",
            "message": message,
            "hint": hint,
        }),
    )
}

fn search_result_payload(report: &SearchReport) -> Value {
    json!({
        "cmd": "search",
        "status": "ok",
        "expr": report.expression,
        "matches": report.stats.matches_found,
        "files": {
            "discovered": report.stats.files_discovered,
            "scanned": report.stats.files_scanned,
            "skipped": report.stats.files_skipped,
        },
        "bytes": report.stats.bytes_scanned,
        "ms": {
            "discover": report.stats.timings.discover_ms,
            "scan": report.stats.timings.scan_ms,
            "aggregate": report.stats.timings.aggregate_ms,
            "total": report.stats.timings.total_ms,
        },
        "slowest": report.stats.slowest_files.first().map(|slowest| json!({
            "path": normalize_display_path(&slowest.path),
            "ms": slowest.duration_ms,
            "bytes": slowest.bytes,
        })),
        "dedupe": {
            "overlap_pruned_roots": report.stats.overlap_pruned_roots,
            "discovered_duplicate_paths": report.stats.discovered_duplicate_paths,
        },
    })
}

fn render_sentinel(kind: &str, payload: Value) -> String {
    format!("-- {kind} {} --", compact_json(payload))
}

fn compact_json(payload: Value) -> String {
    serde_json::to_string(&payload).expect("agent output payload should serialize")
}

fn normalize_argv_paths(argv: &[String]) -> Vec<String> {
    argv.iter()
        .map(|arg| {
            if arg.contains('\\') {
                normalize_display_path(arg)
            } else {
                arg.clone()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use iex_core::{
        stats::{SearchStats, SlowFileStat},
        SearchReport,
    };

    use super::*;

    #[test]
    fn search_result_v1_is_parseable_single_line_json() {
        let line = render_search_result_v1(&sample_report(0));
        let payload = payload_from_sentinel(&line, RESULT_V1);

        assert_eq!(payload["cmd"], "search");
        assert_eq!(payload["status"], "ok");
        assert_eq!(payload["matches"], 0);
        assert_eq!(payload["slowest"]["path"], "crates/iex-core/src/engine.rs");
        assert!(!line.contains('\n'));
    }

    #[test]
    fn search_result_v1_uses_null_slowest_when_no_files_scanned() {
        let mut report = sample_report(0);
        report.stats.slowest_files.clear();

        let line = render_search_result_v1(&report);
        let payload = payload_from_sentinel(&line, RESULT_V1);

        assert!(payload["slowest"].is_null());
    }

    #[test]
    fn search_result_v1_is_single_terminal_packet_vs_legacy_rows() {
        let report = sample_report(0);
        let line = render_search_result_v1(&report);
        let legacy = legacy_search_trailer(&report);
        let payload = payload_from_sentinel(&line, RESULT_V1);

        assert_eq!(line.lines().count(), 1);
        assert_eq!(line.matches(RESULT_V1).count(), 1);
        assert_eq!(payload["status"], "ok");
        assert_eq!(payload["matches"], 0);
        assert!(approx_agent_packets(&line) < approx_agent_packets(&legacy));
    }

    #[test]
    fn next_v1_normalizes_argv_paths() {
        let line = render_next_v1(&[
            "ix".to_owned(),
            "inspect".to_owned(),
            "crates\\iex-cli\\src\\main.rs".to_owned(),
            "--start-line".to_owned(),
            "3".to_owned(),
        ]);
        let payload = payload_from_sentinel(&line, NEXT_V1);

        assert_eq!(payload["cmd"], "inspect");
        assert_eq!(payload["argv"][2], "crates/iex-cli/src/main.rs");
    }

    #[test]
    fn error_v1_has_explicit_boundary_fields() {
        let line = render_error_v1(
            "unsupported_flag",
            "flag is not supported",
            Some("use ix search"),
        );
        let payload = payload_from_sentinel(&line, ERROR_V1);

        assert_eq!(payload["status"], "error");
        assert_eq!(payload["code"], "unsupported_flag");
        assert_eq!(payload["hint"], "use ix search");
    }

    fn sample_report(matches: usize) -> SearchReport {
        let mut stats = SearchStats::default();
        stats.matches_found = matches;
        stats.files_discovered = 2;
        stats.files_scanned = 2;
        stats.bytes_scanned = 128;
        stats.slowest_files.push(SlowFileStat {
            path: "crates\\iex-core\\src\\engine.rs".to_owned(),
            duration_ms: 1.25,
            bytes: 128,
            linux_dominant_target: false,
        });

        SearchReport {
            expression: "lit:nope".to_owned(),
            hits: Vec::new(),
            stats,
        }
    }

    fn payload_from_sentinel(line: &str, kind: &str) -> serde_json::Value {
        let prefix = format!("-- {kind} ");
        let payload = line
            .strip_prefix(&prefix)
            .and_then(|rest| rest.strip_suffix(" --"))
            .expect("sentinel should wrap JSON payload");
        serde_json::from_str(payload).expect("sentinel payload should parse as JSON")
    }

    fn legacy_search_trailer(report: &SearchReport) -> String {
        let slowest = report.stats.slowest_files.first().unwrap();
        let summary_kind = ["ix.search", "summary"].join(".");
        let timings_kind = ["ix.search", "timings_ms"].join(".");
        let slowest_kind = ["ix.search", "slowest"].join(".");
        let matches_found_key = ["matches", "found"].join("_");
        format!(
            "-- {summary_kind} expression={} files_discovered={} files_scanned={} files_skipped={} {matches_found_key}={} bytes_scanned={} --\n\
             -- {timings_kind} discover={} scan={} aggregate={} total={} --\n\
             -- {slowest_kind} path={} ms={} bytes={} --",
            json_string(&report.expression),
            report.stats.files_discovered,
            report.stats.files_scanned,
            report.stats.files_skipped,
            report.stats.matches_found,
            report.stats.bytes_scanned,
            report.stats.timings.discover_ms,
            report.stats.timings.scan_ms,
            report.stats.timings.aggregate_ms,
            report.stats.timings.total_ms,
            json_string(&normalize_display_path(&slowest.path)),
            slowest.duration_ms,
            slowest.bytes,
        )
    }

    fn approx_agent_packets(output: &str) -> usize {
        output.split_ascii_whitespace().count()
    }
}

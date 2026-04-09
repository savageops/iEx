use std::{
    collections::BTreeSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::mpsc,
    time::Instant,
};

use anyhow::{Context, Result};
use ignore::{WalkBuilder, WalkState};
use memchr::memchr;
use memmap2::MmapOptions;
use rayon::prelude::*;
use serde::Serialize;

use crate::{expr::ExpressionPlan, stats::SearchStats};

const TINY_FILE_INLINE_LIMIT: usize = 16 * 1024;
const SMALL_FILE_INLINE_LIMIT: u64 = 256 * 1024;
const BINARY_SNIFF_LIMIT: usize = 1024;
const PARALLEL_FAST_COUNT_FILE_THRESHOLD: usize = 64 * 1024 * 1024;
const PARALLEL_FAST_COUNT_CHUNK_BYTES: usize = 64 * 1024 * 1024;
const MAX_PARALLEL_FAST_COUNT_THREADS: usize = 12;

#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub roots: Vec<PathBuf>,
    pub plan: ExpressionPlan,
    pub include_hidden: bool,
    pub follow_symlinks: bool,
    pub max_hits: Option<usize>,
    pub threads: Option<usize>,
    pub collect_hits: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchHit {
    pub path: String,
    pub line: usize,
    pub column: usize,
    pub preview: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchReport {
    pub expression: String,
    pub hits: Vec<SearchHit>,
    pub stats: SearchStats,
}

#[derive(Debug)]
struct FileScanOutcome {
    path: PathBuf,
    duration_ms: f64,
    bytes: u64,
    hits: Vec<SearchHit>,
    match_count: usize,
    scanned: bool,
}

impl SearchConfig {
    pub fn new(root: PathBuf, plan: ExpressionPlan) -> Self {
        Self::from_roots(vec![root], plan)
    }

    pub fn from_roots(mut roots: Vec<PathBuf>, plan: ExpressionPlan) -> Self {
        if roots.is_empty() {
            roots.push(PathBuf::from("."));
        }
        Self {
            roots,
            plan,
            include_hidden: false,
            follow_symlinks: false,
            max_hits: None,
            threads: None,
            collect_hits: true,
        }
    }
}

pub fn run_search(config: &SearchConfig) -> Result<SearchReport> {
    run_search_inner(config)
}

fn run_search_inner(config: &SearchConfig) -> Result<SearchReport> {
    let total_started = Instant::now();
    let mut stats = SearchStats::default();
    let mut hits = Vec::new();
    let discover_started = Instant::now();
    let files = discover_files(config)?;
    stats.timings.discover_ms = discover_started.elapsed().as_secs_f64() * 1_000.0;
    stats.files_discovered = files.len();

    let resolved_threads = config
        .threads
        .unwrap_or_else(|| auto_threads_for_shape(files.len()));
    let scan_started = Instant::now();
    let mut outcomes = scan_paths(&files, &config.plan, config.collect_hits, resolved_threads)?;
    stats.timings.scan_ms = scan_started.elapsed().as_secs_f64() * 1_000.0;

    for outcome in outcomes.drain(..) {
        merge_outcome(&mut stats, &mut hits, config.collect_hits, outcome);
    }

    let aggregate_started = Instant::now();
    if config.collect_hits {
        hits.sort_by(|a, b| {
            a.path
                .cmp(&b.path)
                .then(a.line.cmp(&b.line))
                .then(a.column.cmp(&b.column))
        });
        if let Some(max_hits) = config.max_hits {
            hits.truncate(max_hits);
        }
    }
    stats.timings.aggregate_ms = aggregate_started.elapsed().as_secs_f64() * 1_000.0;
    stats.timings.total_ms = total_started.elapsed().as_secs_f64() * 1_000.0;

    Ok(SearchReport {
        expression: config.plan.source.clone(),
        hits,
        stats,
    })
}

fn discover_files(config: &SearchConfig) -> Result<Vec<PathBuf>> {
    let roots = partition_roots(config);
    if roots.directory_roots.is_empty() {
        return Ok(finalize_discovered_files(roots.direct_files, false));
    }

    let needs_dedupe = config.roots.len() > 1 || !roots.direct_files.is_empty();
    let mut files = roots.direct_files;
    files.extend(discover_files_parallel(config, &roots.directory_roots)?);
    Ok(finalize_discovered_files(files, needs_dedupe))
}

fn build_walk_builder(config: &SearchConfig, roots: &[PathBuf]) -> WalkBuilder {
    let mut builder = WalkBuilder::new(&roots[0]);
    for root in roots.iter().skip(1) {
        builder.add(root);
    }
    builder
        .hidden(!config.include_hidden)
        .follow_links(config.follow_symlinks)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .ignore(true)
        .parents(true);
    builder
}

fn discover_files_parallel(config: &SearchConfig, roots: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let builder = build_walk_builder(config, roots);
    let (sender, receiver) = mpsc::channel::<PathBuf>();
    let walker = builder.build_parallel();
    walker.run(|| {
        let sender = sender.clone();
        Box::new(move |result| {
            if let Ok(entry) = result {
                if entry.file_type().is_some_and(|kind| kind.is_file()) {
                    let _ = sender.send(entry.into_path());
                }
            }
            WalkState::Continue
        })
    });
    drop(sender);
    let files: Vec<PathBuf> = receiver.into_iter().collect();

    Ok(files)
}

#[derive(Debug)]
struct RootTargets {
    direct_files: Vec<PathBuf>,
    directory_roots: Vec<PathBuf>,
}

fn partition_roots(config: &SearchConfig) -> RootTargets {
    let mut direct_files = Vec::new();
    let mut directory_roots = Vec::new();

    for root in &config.roots {
        if std::fs::metadata(root)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
        {
            direct_files.push(root.clone());
        } else {
            directory_roots.push(root.clone());
        }
    }

    RootTargets {
        direct_files,
        directory_roots,
    }
}

fn finalize_discovered_files(files: Vec<PathBuf>, dedupe: bool) -> Vec<PathBuf> {
    if !dedupe {
        return files;
    }
    let mut unique = BTreeSet::new();
    for path in files {
        unique.insert(path);
    }
    unique.into_iter().collect()
}

fn auto_threads_for_shape(file_count: usize) -> usize {
    let available = std::thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1);

    let desired = if file_count <= 24 {
        1
    } else if file_count <= 128 {
        4
    } else if file_count <= 1_024 {
        8
    } else if file_count <= 8_192 {
        12
    } else {
        16
    };
    available.min(desired).max(1)
}

fn scan_paths(
    files: &[PathBuf],
    plan: &ExpressionPlan,
    collect_hits: bool,
    resolved_threads: usize,
) -> Result<Vec<FileScanOutcome>> {
    if resolved_threads <= 1 {
        return Ok(files
            .iter()
            .map(|path| scan_file(path, plan, collect_hits, resolved_threads))
            .collect());
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(resolved_threads)
        .build()
        .context("failed to configure rayon thread pool")?;
    Ok(pool.install(|| {
        files
            .par_iter()
            .map(|path| scan_file(path, plan, collect_hits, resolved_threads))
            .collect()
    }))
}

fn merge_outcome(
    stats: &mut SearchStats,
    hits: &mut Vec<SearchHit>,
    collect_hits: bool,
    outcome: FileScanOutcome,
) {
    if outcome.scanned {
        stats.files_scanned += 1;
        stats.bytes_scanned += outcome.bytes;
        stats.consider_slow_file(&outcome.path, outcome.duration_ms, outcome.bytes);
        stats.matches_found += outcome.match_count;
        if collect_hits {
            hits.extend(outcome.hits);
        }
    } else {
        stats.files_skipped += 1;
    }
}

fn scan_file(
    path: &Path,
    plan: &ExpressionPlan,
    collect_hits: bool,
    thread_budget: usize,
) -> FileScanOutcome {
    let started = Instant::now();
    let mut outcome = {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return failed_outcome(path, 0),
        };

        let mut tiny = [0u8; TINY_FILE_INLINE_LIMIT];
        let tiny_read = match file.read(&mut tiny) {
            Ok(read) => read,
            Err(_) => return failed_outcome(path, 0),
        };

        if tiny_read < TINY_FILE_INLINE_LIMIT {
            scan_loaded_bytes(
                path,
                plan,
                collect_hits,
                thread_budget,
                &tiny[..tiny_read],
                tiny_read as u64,
            )
        } else {
            let mut sentinel = [0u8; 1];
            let sentinel_read = match file.read(&mut sentinel) {
                Ok(read) => read,
                Err(_) => return failed_outcome(path, tiny_read as u64),
            };

            if sentinel_read == 0 {
                scan_loaded_bytes(
                    path,
                    plan,
                    collect_hits,
                    thread_budget,
                    &tiny,
                    tiny.len() as u64,
                )
            } else {
                let metadata = match file.metadata() {
                    Ok(metadata) => metadata,
                    Err(_) => return failed_outcome(path, (tiny_read + sentinel_read) as u64),
                };

                if metadata.len() <= SMALL_FILE_INLINE_LIMIT {
                    let mut bytes = Vec::with_capacity(metadata.len() as usize);
                    bytes.extend_from_slice(&tiny);
                    bytes.push(sentinel[0]);
                    if file.read_to_end(&mut bytes).is_err() {
                        return failed_outcome(path, metadata.len());
                    }
                    scan_loaded_bytes(
                        path,
                        plan,
                        collect_hits,
                        thread_budget,
                        &bytes,
                        metadata.len(),
                    )
                } else {
                    let mmap = match unsafe { MmapOptions::new().map(&file) } {
                        Ok(mmap) => mmap,
                        Err(_) => return failed_outcome(path, metadata.len()),
                    };
                    scan_loaded_bytes(
                        path,
                        plan,
                        collect_hits,
                        thread_budget,
                        &mmap,
                        metadata.len(),
                    )
                }
            }
        }
    };
    outcome.duration_ms = started.elapsed().as_secs_f64() * 1_000.0;
    outcome
}

fn scan_loaded_bytes(
    path: &Path,
    plan: &ExpressionPlan,
    collect_hits: bool,
    thread_budget: usize,
    bytes: &[u8],
    file_bytes: u64,
) -> FileScanOutcome {
    if is_likely_binary(bytes) {
        return failed_outcome(path, file_bytes);
    }

    let mut hits = Vec::new();
    let mut match_count = 0usize;
    let mut cached_path = None::<String>;

    if !collect_hits {
        if let Some(count) = maybe_parallel_fast_count_bytes(plan, bytes, thread_budget) {
            match_count = count;
            return completed_outcome(path, file_bytes, hits, match_count);
        }
        if let Some(count) = plan.fast_match_count_no_hits_bytes(bytes) {
            match_count = count;
            return completed_outcome(path, file_bytes, hits, match_count);
        }
    }

    if plan.supports_byte_mode() {
        let mut line_start = 0usize;
        let mut line_no = 0usize;

        loop {
            if line_start == bytes.len() {
                break;
            }

            let line_end = match memchr(b'\n', &bytes[line_start..]) {
                Some(offset) => line_start + offset,
                None => bytes.len(),
            };

            let raw_line = &bytes[line_start..line_end];
            let line = if raw_line.last() == Some(&b'\r') {
                &raw_line[..raw_line.len().saturating_sub(1)]
            } else {
                raw_line
            };

            line_no += 1;
            if plan.matches_bytes(line) {
                match_count += 1;
                if collect_hits {
                    hits.push(SearchHit {
                        path: cached_path
                            .get_or_insert_with(|| path.display().to_string())
                            .clone(),
                        line: line_no,
                        column: plan.first_match_column_bytes(line).unwrap_or(1),
                        preview: String::from_utf8_lossy(line).chars().take(220).collect(),
                    });
                }
            }

            if line_end == bytes.len() {
                break;
            }
            line_start = line_end + 1;
        }
    } else {
        let text = String::from_utf8_lossy(bytes);
        if !collect_hits {
            if let Some(count) = plan.fast_match_count_no_hits(&text) {
                match_count = count;
                return completed_outcome(path, file_bytes, hits, match_count);
            }
        }

        for (line_idx, line) in text.lines().enumerate() {
            if !plan.matches(line) {
                continue;
            }
            match_count += 1;
            if collect_hits {
                hits.push(SearchHit {
                    path: cached_path
                        .get_or_insert_with(|| path.display().to_string())
                        .clone(),
                    line: line_idx + 1,
                    column: plan.first_match_column(line).unwrap_or(1),
                    preview: line.chars().take(220).collect(),
                });
            }
        }
    }

    completed_outcome(path, file_bytes, hits, match_count)
}

fn is_likely_binary(bytes: &[u8]) -> bool {
    let sniff_len = bytes.len().min(BINARY_SNIFF_LIMIT);
    bytes[..sniff_len].contains(&0)
}

fn maybe_parallel_fast_count_bytes(
    plan: &ExpressionPlan,
    bytes: &[u8],
    thread_budget: usize,
) -> Option<usize> {
    if thread_budget > 1 || bytes.len() < PARALLEL_FAST_COUNT_FILE_THRESHOLD {
        return None;
    }

    let available_threads = std::thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
        .min(MAX_PARALLEL_FAST_COUNT_THREADS);
    if available_threads <= 1 {
        return None;
    }

    let chunk_size = bytes
        .len()
        .div_ceil(available_threads.saturating_mul(2))
        .max(PARALLEL_FAST_COUNT_CHUNK_BYTES);
    let ranges = build_chunk_ranges(bytes.len(), chunk_size);
    if ranges.len() <= 1 {
        return None;
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(available_threads)
        .build()
        .ok()?;
    pool.install(|| {
        ranges
            .par_iter()
            .map(|(start, end)| plan.fast_match_count_no_hits_bytes_in_range(bytes, *start, *end))
            .collect::<Option<Vec<_>>>()
            .map(|counts| counts.into_iter().sum())
    })
}

fn build_chunk_ranges(total_len: usize, chunk_size: usize) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut start = 0usize;

    while start < total_len {
        let end = total_len.min(start.saturating_add(chunk_size));
        ranges.push((start, end));
        start = end;
    }

    ranges
}

fn completed_outcome(
    path: &Path,
    bytes: u64,
    hits: Vec<SearchHit>,
    match_count: usize,
) -> FileScanOutcome {
    FileScanOutcome {
        path: path.to_path_buf(),
        duration_ms: 0.0,
        bytes,
        hits,
        match_count,
        scanned: true,
    }
}

fn failed_outcome(path: &Path, bytes: u64) -> FileScanOutcome {
    FileScanOutcome {
        path: path.to_path_buf(),
        duration_ms: 0.0,
        bytes,
        hits: Vec::new(),
        match_count: 0,
        scanned: false,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::ExpressionPlan;

    use super::{auto_threads_for_shape, discover_files, run_search, SearchConfig};

    #[test]
    fn auto_threads_scales_monotonically_with_file_count() {
        let tiny = auto_threads_for_shape(16);
        let medium = auto_threads_for_shape(1_000);
        let huge = auto_threads_for_shape(100_000);
        assert!(tiny >= 1);
        assert!(medium >= tiny);
        assert!(huge >= medium);
    }

    #[test]
    fn auto_threads_uses_parallelism_for_large_corpora_without_byte_samples() {
        assert_eq!(auto_threads_for_shape(1), 1);
        assert!(auto_threads_for_shape(2_048) >= 8);
    }

    #[test]
    fn auto_search_handles_small_corpus_end_to_end() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("iex-engine-small-corpus-{unique}"));
        fs::create_dir_all(&root).expect("temp corpus should be created");
        fs::write(root.join("one.txt"), "alpha\nneedle here\n")
            .expect("first fixture should write");
        fs::write(root.join("two.txt"), "needle again\n").expect("second fixture should write");

        let plan = ExpressionPlan::parse("lit:needle").expect("plan should parse");
        let report =
            run_search(&SearchConfig::new(root.clone(), plan)).expect("search should succeed");

        assert_eq!(report.stats.files_discovered, 2);
        assert_eq!(report.stats.files_scanned, 2);
        assert_eq!(report.stats.files_skipped, 0);
        assert_eq!(report.stats.matches_found, 2);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_files_treats_direct_file_root_as_single_scan_target() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("iex-engine-file-root-{unique}.txt"));
        fs::write(&root, "needle here\n").expect("fixture file should write");

        let plan = ExpressionPlan::parse("lit:needle").expect("plan should parse");
        let config = SearchConfig::new(root.clone(), plan);
        let files = discover_files(&config).expect("single file discovery should succeed");

        assert_eq!(files, vec![root.clone()]);

        let _ = fs::remove_file(root);
    }

    #[test]
    fn discover_files_supports_multiple_directory_roots() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("iex-engine-multi-root-{unique}"));
        let left = root.join("left");
        let right = root.join("right");
        fs::create_dir_all(&left).expect("left dir should be created");
        fs::create_dir_all(&right).expect("right dir should be created");
        fs::write(left.join("one.txt"), "needle left\n").expect("left file should write");
        fs::write(right.join("two.txt"), "needle right\n").expect("right file should write");

        let plan = ExpressionPlan::parse("lit:needle").expect("plan should parse");
        let config = SearchConfig::from_roots(vec![left.clone(), right.clone()], plan);
        let files = discover_files(&config).expect("multi-root discovery should succeed");

        assert_eq!(files.len(), 2);
        assert!(files.contains(&left.join("one.txt")));
        assert!(files.contains(&right.join("two.txt")));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn discover_files_dedupes_file_root_under_directory_root() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("iex-engine-overlap-root-{unique}"));
        fs::create_dir_all(&root).expect("root dir should be created");
        let file = root.join("needle.txt");
        fs::write(&file, "needle here\n").expect("fixture file should write");

        let plan = ExpressionPlan::parse("lit:needle").expect("plan should parse");
        let config = SearchConfig::from_roots(vec![root.clone(), file.clone()], plan);
        let files = discover_files(&config).expect("overlap discovery should succeed");

        assert_eq!(files, vec![file.clone()]);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn run_search_supports_multiple_roots_end_to_end() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("iex-engine-search-multi-root-{unique}"));
        let left = root.join("left");
        let right = root.join("right");
        fs::create_dir_all(&left).expect("left dir should be created");
        fs::create_dir_all(&right).expect("right dir should be created");
        fs::write(left.join("one.txt"), "alpha\nneedle left\n").expect("left file should write");
        fs::write(right.join("two.txt"), "needle right\n").expect("right file should write");

        let plan = ExpressionPlan::parse("lit:needle").expect("plan should parse");
        let report = run_search(&SearchConfig::from_roots(
            vec![left.clone(), right.clone()],
            plan,
        ))
        .expect("multi-root search should succeed");

        assert_eq!(report.stats.files_discovered, 2);
        assert_eq!(report.stats.files_scanned, 2);
        assert_eq!(report.stats.matches_found, 2);

        let _ = fs::remove_dir_all(root);
    }
}

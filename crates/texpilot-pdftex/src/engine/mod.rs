//! v2 native engine skeleton.
//!
//! This module is intentionally report-only for now. It establishes the public
//! data shape for the fused native engine and, most importantly, makes the
//! bottleneck ledger visible before the real executor exists.

use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::{CatCodeTable, Token, tokenize};

const TEX_EXT: &str = "tex";
const BIB_EXT: &str = "bib";
const COMMON_GRAPHIC_EXTENSIONS: &[&str] = &["pdf", "png", "jpg", "jpeg"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineV2Options {
    pub main: PathBuf,
    pub output_dir: PathBuf,
    pub job_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineV2Report {
    pub status: EngineV2Status,
    pub main: PathBuf,
    pub output_dir: PathBuf,
    pub job_name: String,
    pub input_paths: Vec<PathBuf>,
    pub coverage: DependencyCoverage,
    pub ledger: BottleneckLedger,
    pub stages: Vec<StageTiming>,
}

impl EngineV2Report {
    pub fn unsupported_feature_count(&self) -> usize {
        self.coverage.unsupported_features.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineV2Status {
    ReportOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DependencyCoverage {
    pub tex_inputs: Vec<PathBuf>,
    pub bibliography_inputs: Vec<PathBuf>,
    pub asset_inputs: Vec<PathBuf>,
    pub package_names: Vec<String>,
    pub class_names: Vec<String>,
    pub unsupported_features: Vec<UnsupportedFeature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedFeature {
    pub kind: &'static str,
    pub detail: String,
}

impl UnsupportedFeature {
    fn new(kind: &'static str, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageTiming {
    pub name: &'static str,
    pub elapsed_ms: u128,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BottleneckLedger {
    pub resolver: ResolverCounters,
    pub snapshot: SnapshotCounters,
    pub executor: ExecutorCounters,
    pub document: DocumentCounters,
    pub layout: LayoutCounters,
    pub fixed_point: FixedPointCounters,
    pub assets: AssetCounters,
    pub pdf: PdfCounters,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResolverCounters {
    pub lookup_count: u64,
    pub filesystem_stat_count: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SnapshotCounters {
    pub hot_array_copy_bytes: u64,
    pub cold_page_faults: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExecutorCounters {
    pub tokens_read: u64,
    pub tokens_from_source: u64,
    pub tokens_from_slices: u64,
    pub macro_calls: u64,
    pub fast_macro_calls: u64,
    pub materialized_token_lists: u64,
    pub generic_scanner_fallbacks: u64,
    pub conditional_skip_tokens: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DocumentCounters {
    pub events_emitted: u64,
    pub registry_intern_lookups: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LayoutCounters {
    pub nodes_created: u64,
    pub node_bytes: u64,
    pub paragraphs_broken: u64,
    pub page_builder_iterations: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FixedPointCounters {
    pub iterations: u64,
    pub regions_replayed: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AssetCounters {
    pub jobs_submitted: u64,
    pub decode_bytes: u64,
    pub copy_through_bytes: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PdfCounters {
    pub objects: u64,
    pub compressed_bytes: u64,
    pub format_string_bytes: u64,
}

pub fn run_report_only(options: &EngineV2Options) -> io::Result<EngineV2Report> {
    let mut ledger = BottleneckLedger::default();
    let mut stages = Vec::new();
    let started = Instant::now();
    let main = resolve_existing_file(&options.main, &mut ledger)?;
    stages.push(StageTiming {
        name: "resolve_root",
        elapsed_ms: started.elapsed().as_millis(),
    });

    let root_dir = main.parent().unwrap_or(Path::new(".")).to_path_buf();
    let mut input_paths = Vec::new();
    let mut visited = HashSet::new();
    let mut coverage = DependencyCoverage::default();
    let collect_started = Instant::now();
    collect_tex_tree(
        &main,
        &root_dir,
        &mut input_paths,
        &mut visited,
        &mut coverage,
        &mut ledger,
    )?;
    stages.push(StageTiming {
        name: "dependency_probe",
        elapsed_ms: collect_started.elapsed().as_millis(),
    });

    sort_dedup_paths(&mut input_paths);
    sort_dedup_paths(&mut coverage.tex_inputs);
    sort_dedup_paths(&mut coverage.bibliography_inputs);
    sort_dedup_paths(&mut coverage.asset_inputs);
    coverage.package_names.sort();
    coverage.package_names.dedup();
    coverage.class_names.sort();
    coverage.class_names.dedup();
    coverage
        .unsupported_features
        .sort_by(|left, right| (&left.kind, &left.detail).cmp(&(&right.kind, &right.detail)));
    coverage
        .unsupported_features
        .dedup_by(|left, right| left.kind == right.kind && left.detail == right.detail);

    Ok(EngineV2Report {
        status: EngineV2Status::ReportOnly,
        main,
        output_dir: options.output_dir.clone(),
        job_name: options.job_name.clone(),
        input_paths,
        coverage,
        ledger,
        stages,
    })
}

fn collect_tex_tree(
    path: &Path,
    root_dir: &Path,
    input_paths: &mut Vec<PathBuf>,
    visited: &mut HashSet<PathBuf>,
    coverage: &mut DependencyCoverage,
    ledger: &mut BottleneckLedger,
) -> io::Result<()> {
    let canonical = resolve_existing_file(path, ledger)?;
    if !visited.insert(canonical.clone()) {
        return Ok(());
    }
    input_paths.push(canonical.clone());
    coverage.tex_inputs.push(canonical.clone());

    let source = fs::read_to_string(&canonical)?;
    count_source_tokens(&source, ledger);
    collect_source_features(&source, coverage);
    let base_dir = canonical.parent().unwrap_or(root_dir);

    let mut nested_tex = Vec::new();
    for input in braced_command_values(&source, "input")
        .into_iter()
        .chain(braced_command_values(&source, "include"))
    {
        if let Some(path) = resolve_relative_candidate(base_dir, &input, &[TEX_EXT], ledger) {
            nested_tex.push(path);
        }
    }

    for bibliography in braced_command_values(&source, "bibliography") {
        for entry in comma_separated_values(&bibliography) {
            if let Some(path) = resolve_relative_candidate(base_dir, entry, &[BIB_EXT], ledger) {
                input_paths.push(path.clone());
                coverage.bibliography_inputs.push(path);
            }
        }
    }

    for resource in braced_command_values(&source, "addbibresource") {
        if let Some(path) = resolve_relative_candidate(base_dir, &resource, &[BIB_EXT], ledger) {
            input_paths.push(path.clone());
            coverage.bibliography_inputs.push(path);
        }
    }

    for graphic in braced_command_values(&source, "includegraphics") {
        if let Some(path) =
            resolve_relative_candidate(base_dir, &graphic, COMMON_GRAPHIC_EXTENSIONS, ledger)
        {
            input_paths.push(path.clone());
            coverage.asset_inputs.push(path);
            ledger.assets.jobs_submitted += 1;
        }
    }

    for nested in nested_tex {
        collect_tex_tree(&nested, root_dir, input_paths, visited, coverage, ledger)?;
    }

    Ok(())
}

fn resolve_existing_file(path: &Path, ledger: &mut BottleneckLedger) -> io::Result<PathBuf> {
    ledger.resolver.lookup_count += 1;
    ledger.resolver.filesystem_stat_count += 1;
    fs::canonicalize(path)
}

fn resolve_relative_candidate(
    base_dir: &Path,
    value: &str,
    default_extensions: &[&str],
    ledger: &mut BottleneckLedger,
) -> Option<PathBuf> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let raw = Path::new(trimmed);
    let base = if raw.is_absolute() {
        raw.to_path_buf()
    } else {
        base_dir.join(raw)
    };
    if let Some(path) = canonical_if_file(&base, ledger) {
        return Some(path);
    }
    if base.extension().is_some() {
        return None;
    }
    for extension in default_extensions {
        let candidate = base.with_extension(extension);
        if let Some(path) = canonical_if_file(&candidate, ledger) {
            return Some(path);
        }
    }
    None
}

fn canonical_if_file(path: &Path, ledger: &mut BottleneckLedger) -> Option<PathBuf> {
    ledger.resolver.lookup_count += 1;
    ledger.resolver.filesystem_stat_count += 1;
    if path.is_file() {
        fs::canonicalize(path).ok()
    } else {
        None
    }
}

fn count_source_tokens(source: &str, ledger: &mut BottleneckLedger) {
    let tokens = tokenize(source, &CatCodeTable::plain_tex());
    ledger.executor.tokens_read += tokens.len() as u64;
    ledger.executor.tokens_from_source += tokens.len() as u64;
    ledger.executor.macro_calls += tokens
        .iter()
        .filter(|token| matches!(token, Token::ControlSequence(_)))
        .count() as u64;
}

fn collect_source_features(source: &str, coverage: &mut DependencyCoverage) {
    for class in braced_command_values(source, "documentclass") {
        coverage
            .class_names
            .extend(comma_separated_values(&class).map(str::to_owned));
    }
    for package in braced_command_values(source, "usepackage")
        .into_iter()
        .chain(braced_command_values(source, "RequirePackage"))
    {
        coverage
            .package_names
            .extend(comma_separated_values(&package).map(str::to_owned));
    }
    if !braced_command_values(source, "special").is_empty()
        || source_contains_control(source, "special")
    {
        coverage.unsupported_features.push(UnsupportedFeature::new(
            "special",
            "\\special requires backend-specific effect handling",
        ));
    }
    if source_contains_control(source, "write18") {
        coverage.unsupported_features.push(UnsupportedFeature::new(
            "shell_escape",
            "\\write18 requires explicit shell-escape policy",
        ));
    }
    for primitive in ["pdfobj", "pdfximage", "pdfrefximage", "pdfannot"] {
        if source_contains_control(source, primitive) {
            coverage.unsupported_features.push(UnsupportedFeature::new(
                "pdf_primitive",
                format!("\\{primitive} requires native PDF primitive execution"),
            ));
        }
    }
}

fn braced_command_values(source: &str, command: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut cursor = source;
    while let Some(index) = find_control(cursor, command) {
        let after = &cursor[index + command.len() + 1..];
        let after = after.trim_start();
        let after = skip_optional_brackets(after).trim_start();
        if let Some((value, rest)) = take_braced(after) {
            values.push(value.to_string());
            cursor = rest;
        } else {
            cursor = after;
        }
    }
    values
}

fn source_contains_control(source: &str, command: &str) -> bool {
    find_control(source, command).is_some()
}

fn find_control(source: &str, command: &str) -> Option<usize> {
    let needle = format!("\\{command}");
    let mut offset = 0;
    while let Some(index) = source[offset..].find(&needle) {
        let absolute = offset + index;
        if control_boundary(source, absolute + needle.len()) && !is_commented(source, absolute) {
            return Some(absolute);
        }
        offset = absolute + needle.len();
    }
    None
}

fn control_boundary(source: &str, index: usize) -> bool {
    source[index..]
        .chars()
        .next()
        .is_none_or(|ch| !ch.is_ascii_alphabetic() && ch != '@')
}

fn is_commented(source: &str, index: usize) -> bool {
    let line_start = source[..index].rfind('\n').map_or(0, |start| start + 1);
    let mut escaped = false;
    for ch in source[line_start..index].chars() {
        match ch {
            '\\' => escaped = !escaped,
            '%' if !escaped => return true,
            _ => escaped = false,
        }
    }
    false
}

fn skip_optional_brackets(mut source: &str) -> &str {
    loop {
        let trimmed = source.trim_start();
        if !trimmed.starts_with('[') {
            return source;
        }
        let Some((_, rest)) = take_balanced(trimmed, '[', ']') else {
            return source;
        };
        source = rest;
    }
}

fn take_braced(source: &str) -> Option<(&str, &str)> {
    take_balanced(source, '{', '}')
}

fn take_balanced(source: &str, open: char, close: char) -> Option<(&str, &str)> {
    let mut chars = source.char_indices();
    let (_, first) = chars.next()?;
    if first != open {
        return None;
    }
    let mut depth = 1_usize;
    let mut escaped = false;
    for (index, ch) in chars {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth -= 1;
            if depth == 0 {
                let value = &source[open.len_utf8()..index];
                let rest = &source[index + close.len_utf8()..];
                return Some((value, rest));
            }
        }
    }
    None
}

fn comma_separated_values(value: &str) -> impl Iterator<Item = &str> {
    value
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
}

fn sort_dedup_paths(paths: &mut Vec<PathBuf>) {
    let mut seen = BTreeSet::new();
    paths.retain(|path| seen.insert(path.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::process;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn report_only_runner_discovers_inputs_and_emits_ledger() {
        let root = unique_temp_dir("texpilot-pdftex-engine-report");
        fs::create_dir_all(root.join("figures")).unwrap();
        fs::write(
            root.join("main.tex"),
            r#"\documentclass{article}
\usepackage{graphicx}
\input{section}
\begin{document}
\includegraphics[width=1in]{figures/plot}
\bibliography{refs}
\end{document}
"#,
        )
        .unwrap();
        fs::write(root.join("section.tex"), "Section text.\n").unwrap();
        fs::write(root.join("refs.bib"), "@book{knuth,title={TeX}}\n").unwrap();
        fs::write(root.join("figures/plot.png"), b"png placeholder").unwrap();

        let report = run_report_only(&EngineV2Options {
            main: root.join("main.tex"),
            output_dir: root.join("out"),
            job_name: "main".to_string(),
        })
        .unwrap();

        assert_eq!(report.status, EngineV2Status::ReportOnly);
        assert!(
            report
                .input_paths
                .iter()
                .any(|path| path.ends_with("main.tex"))
        );
        assert!(
            report
                .input_paths
                .iter()
                .any(|path| path.ends_with("section.tex"))
        );
        assert!(
            report
                .coverage
                .bibliography_inputs
                .iter()
                .any(|path| path.ends_with("refs.bib"))
        );
        assert!(
            report
                .coverage
                .asset_inputs
                .iter()
                .any(|path| path.ends_with("plot.png"))
        );
        assert_eq!(report.coverage.package_names, vec!["graphicx"]);
        assert_eq!(report.coverage.class_names, vec!["article"]);
        assert!(report.ledger.executor.tokens_read > 0);
        assert_eq!(report.ledger.executor.materialized_token_lists, 0);
        assert!(report.ledger.assets.jobs_submitted > 0);
        assert!(
            report
                .stages
                .iter()
                .any(|stage| stage.name == "dependency_probe")
        );
    }

    #[test]
    fn report_only_runner_names_unsupported_effects() {
        let root = unique_temp_dir("texpilot-pdftex-engine-unsupported");
        fs::write(
            root.join("main.tex"),
            r#"\documentclass{article}
\begin{document}
\special{pdf:literal direct 0 0 m}
\write18{echo unsafe}
\pdfobj stream {}
\end{document}
"#,
        )
        .unwrap();

        let report = run_report_only(&EngineV2Options {
            main: root.join("main.tex"),
            output_dir: root.join("out"),
            job_name: "main".to_string(),
        })
        .unwrap();
        let kinds = report
            .coverage
            .unsupported_features
            .iter()
            .map(|feature| feature.kind)
            .collect::<BTreeSet<_>>();

        assert!(kinds.contains("special"));
        assert!(kinds.contains("shell_escape"));
        assert!(kinds.contains("pdf_primitive"));
        assert_eq!(report.ledger.executor.materialized_token_lists, 0);
    }

    #[test]
    fn report_only_runner_handles_bundled_arxiv_examples() {
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("crate should live under workspace/crates")
            .to_path_buf();
        for relative in [
            "examples/arXiv-2605.26379v1/main.tex",
            "examples/arXiv-2511.08544v3/main.tex",
        ] {
            let main = workspace.join(relative);
            let report = run_report_only(&EngineV2Options {
                main: main.clone(),
                output_dir: workspace.join("target/texpilot-pdftex-report-only"),
                job_name: "main".to_string(),
            })
            .unwrap_or_else(|error| panic!("report-only run failed for {relative}: {error}"));

            assert_eq!(report.status, EngineV2Status::ReportOnly);
            assert!(report.input_paths.iter().any(|path| path == &report.main));
            assert!(
                report.ledger.executor.tokens_read > 0,
                "expected token coverage for {relative}"
            );
            assert_eq!(
                report.ledger.executor.materialized_token_lists, 0,
                "report-only runner must not introduce token-list churn for {relative}"
            );
        }
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{}-{stamp}", process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("failed to create temp dir");
        path
    }
}

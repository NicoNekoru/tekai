use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use notify::{Event, RecursiveMode, Watcher};

use crate::compiler::{
    BibMode, BuildOptions, DraftPrepass, Engine, Runner, build, build_dependency_paths,
};
use crate::lint::{LintConfig, format_diagnostic, has_errors, lint_paths};

#[derive(Debug, Clone)]
pub struct WatchOptions {
    pub main: PathBuf,
    pub root: PathBuf,
    pub build_options: BuildOptions,
    pub final_build_options: Option<BuildOptions>,
    pub final_after_idle: Option<Duration>,
    pub lint_config: LintConfig,
    pub lint: bool,
    pub fail_on_warnings: bool,
}

pub fn watch(options: WatchOptions) -> Result<()> {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(move |result| {
        let _ = tx.send(result);
    })
    .context("failed to create filesystem watcher")?;
    watcher
        .watch(&options.root, RecursiveMode::Recursive)
        .with_context(|| format!("failed to watch {}", options.root.display()))?;

    let mut filter = WatchFilter {
        root: canonical_for_watch(&options.root).unwrap_or_else(|| options.root.clone()),
        ignored_out_dir: absolute_output_dir(&options.build_options.out_dir)?,
        dependency_paths: HashSet::new(),
        watched_dependency_dirs: HashSet::new(),
    };
    let mut hot_preview = HotPreviewState::new(&options);
    eprintln!("watching {}", options.root.display());
    let initial_build_ok = run_once(&options, LintScope::Full, "rebuild", None);
    refresh_dependency_filter(&options, &mut filter, &mut watcher);
    if let Some(hot_preview) = &mut hot_preview {
        hot_preview.remember_paths(filter.dependency_paths.iter());
        if initial_build_ok {
            hot_preview.prewarm(&options);
        }
    }
    drain_startup_events(&rx);

    let mut pending_event = None;
    if initial_build_ok
        && let (Some(final_build_options), Some(final_after_idle)) =
            (&options.final_build_options, options.final_after_idle)
    {
        match wait_for_relevant_event(&rx, final_after_idle, &filter) {
            WaitForEvent::Idle => {
                eprintln!("--- tekai final rebuild {:?} ---", Instant::now());
                run_build_once(final_build_options, "final rebuild");
                refresh_dependency_filter(&options, &mut filter, &mut watcher);
            }
            WaitForEvent::Event(event) => pending_event = Some(event),
            WaitForEvent::Closed => return Ok(()),
        }
    }

    loop {
        let event = if let Some(event) = pending_event.take() {
            event
        } else {
            rx.recv().context("watch channel closed")??
        };
        let Some(paths) = relevant_event_paths(&event, &filter) else {
            continue;
        };
        let (quiet_duration, max_duration) = debounce_durations(hot_preview.is_some());
        let paths = debounced_relevant_paths(&rx, quiet_duration, max_duration, &filter, paths);
        let build_ok = run_once(
            &options,
            LintScope::Changed(paths),
            "rebuild",
            hot_preview.as_mut(),
        );
        refresh_dependency_filter(&options, &mut filter, &mut watcher);
        if let Some(hot_preview) = &mut hot_preview {
            hot_preview.remember_paths(filter.dependency_paths.iter());
        }
        if build_ok
            && let (Some(final_build_options), Some(final_after_idle)) =
                (&options.final_build_options, options.final_after_idle)
        {
            match wait_for_relevant_event(&rx, final_after_idle, &filter) {
                WaitForEvent::Idle => {
                    eprintln!("--- tekai final rebuild {:?} ---", Instant::now());
                    run_build_once(final_build_options, "final rebuild");
                    refresh_dependency_filter(&options, &mut filter, &mut watcher);
                }
                WaitForEvent::Event(event) => pending_event = Some(event),
                WaitForEvent::Closed => break,
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
enum LintScope {
    Full,
    Changed(Vec<PathBuf>),
}

fn run_once(
    options: &WatchOptions,
    lint_scope: LintScope,
    label: &str,
    hot_preview: Option<&mut HotPreviewState>,
) -> bool {
    eprintln!("--- tekai {label} {:?} ---", Instant::now());
    if options.lint {
        match lint_targets(options, &lint_scope).and_then(|targets| {
            if targets.is_empty() {
                Ok(Vec::new())
            } else {
                lint_paths(&targets, &options.lint_config)
            }
        }) {
            Ok(diagnostics) => {
                for diagnostic in &diagnostics {
                    eprintln!("{}", format_diagnostic(diagnostic));
                }
                if has_errors(&diagnostics) || (options.fail_on_warnings && !diagnostics.is_empty())
                {
                    eprintln!("lint failed; build skipped");
                    return false;
                }
            }
            Err(error) => {
                eprintln!("lint failed: {error:#}");
                return false;
            }
        }
    }
    if let (Some(hot_preview), LintScope::Changed(paths)) = (hot_preview, &lint_scope) {
        match hot_preview.run(options, paths, label) {
            HotPreviewOutcome::Built(ok) => return ok,
            HotPreviewOutcome::NotApplicable => {}
            HotPreviewOutcome::Failed(error) => {
                eprintln!(
                    "warning: hot preview unavailable: {error:#}; falling back to full preview"
                )
            }
        }
    }
    run_build_once(&options.build_options, label)
}

fn run_build_once(build_options: &BuildOptions, label: &str) -> bool {
    match build(build_options) {
        Ok(report) => {
            if let Some(pdf) = report.pdf_path {
                if report.skipped {
                    eprintln!(
                        "cached {} in {:.2?} (inputs unchanged)",
                        pdf.display(),
                        report.elapsed
                    );
                } else if report.tex_runs == 0 {
                    eprintln!("built {} in {:.2?}", pdf.display(), report.elapsed);
                } else {
                    eprintln!(
                        "built {} in {:.2?} ({} TeX run{}, {} bibliography run{}, {} index run{}, {} external run{})",
                        pdf.display(),
                        report.elapsed,
                        report.tex_runs,
                        if report.tex_runs == 1 { "" } else { "s" },
                        report.bibliography_runs,
                        if report.bibliography_runs == 1 {
                            ""
                        } else {
                            "s"
                        },
                        report.index_runs,
                        if report.index_runs == 1 { "" } else { "s" },
                        report.external_runs,
                        if report.external_runs == 1 { "" } else { "s" }
                    );
                }
            } else {
                eprintln!("build finished in {:.2?}", report.elapsed);
            }
            true
        }
        Err(error) => {
            eprintln!("{label} failed: {error:#}");
            false
        }
    }
}

fn debounce_durations(hot_preview: bool) -> (Duration, Duration) {
    if hot_preview {
        (Duration::from_millis(12), Duration::from_millis(100))
    } else {
        (Duration::from_millis(120), Duration::from_millis(1_000))
    }
}

fn lint_targets(options: &WatchOptions, lint_scope: &LintScope) -> Result<Vec<PathBuf>> {
    match lint_scope {
        LintScope::Full => Ok(vec![options.root.clone()]),
        LintScope::Changed(paths) => {
            let mut targets = paths
                .iter()
                .filter(|path| has_lint_extension(path))
                .filter(|path| path.exists())
                .filter_map(|path| canonical_for_watch(path))
                .collect::<Vec<_>>();
            targets.sort();
            targets.dedup();
            Ok(targets)
        }
    }
}

fn debounced_relevant_paths(
    rx: &mpsc::Receiver<notify::Result<Event>>,
    quiet_duration: Duration,
    max_duration: Duration,
    filter: &WatchFilter,
    initial_paths: Vec<PathBuf>,
) -> Vec<PathBuf> {
    let start = Instant::now();
    let mut quiet_started = start;
    let mut paths = initial_paths;
    loop {
        let elapsed = start.elapsed();
        if elapsed >= max_duration || quiet_started.elapsed() >= quiet_duration {
            break;
        }
        let quiet_remaining = quiet_duration.saturating_sub(quiet_started.elapsed());
        let max_remaining = max_duration.saturating_sub(elapsed);
        let timeout = quiet_remaining
            .min(max_remaining)
            .min(Duration::from_millis(40));
        match rx.recv_timeout(timeout) {
            Ok(Ok(event)) => {
                if let Some(mut event_paths) = relevant_event_paths(&event, filter) {
                    paths.append(&mut event_paths);
                    quiet_started = Instant::now();
                }
            }
            Ok(Err(error)) => eprintln!("warning: watch event failed: {error}"),
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
    paths.sort();
    paths.dedup();
    paths
}

#[derive(Debug)]
enum WaitForEvent {
    Idle,
    Event(Event),
    Closed,
}

fn wait_for_relevant_event(
    rx: &mpsc::Receiver<notify::Result<Event>>,
    idle_duration: Duration,
    filter: &WatchFilter,
) -> WaitForEvent {
    let start = Instant::now();
    loop {
        let remaining = idle_duration.saturating_sub(start.elapsed());
        if remaining.is_zero() {
            return WaitForEvent::Idle;
        }
        match rx.recv_timeout(remaining.min(Duration::from_millis(40))) {
            Ok(Ok(event)) => {
                if relevant_event_paths(&event, filter).is_some() {
                    return WaitForEvent::Event(event);
                }
            }
            Ok(Err(error)) => eprintln!("warning: watch event failed: {error}"),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if start.elapsed() >= idle_duration {
                    return WaitForEvent::Idle;
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => return WaitForEvent::Closed,
        }
    }
}

fn drain_startup_events(rx: &mpsc::Receiver<notify::Result<Event>>) {
    loop {
        match rx.try_recv() {
            Ok(Ok(_event)) => {}
            Ok(Err(error)) => eprintln!("warning: watch event failed: {error}"),
            Err(mpsc::TryRecvError::Empty) | Err(mpsc::TryRecvError::Disconnected) => break,
        }
    }
}

const HOT_PREVIEW_DIR: &str = ".tekai-hmr";
const HOT_PREVIEW_WARM_DIR: &str = ".tekai-hmr-warm";
const HOT_PREVIEW_CONTEXT_BYTES: usize = 900;
const HOT_PREVIEW_MAX_SNIPPET_BYTES: usize = 2_400;
const HOT_PREVIEW_MAX_INLINE_BYTES: u64 = 128 * 1024;

#[derive(Debug)]
enum HotPreviewOutcome {
    Built(bool),
    NotApplicable,
    Failed(anyhow::Error),
}

#[derive(Debug)]
struct HotPreviewState {
    snapshots: HashMap<PathBuf, String>,
    static_document: Option<HotPreviewStaticDocument>,
    warmed: bool,
}

#[derive(Debug)]
struct HotPreviewStaticDocument {
    main: PathBuf,
    out_dir: PathBuf,
    hmr_dir: PathBuf,
    preamble: String,
}

#[derive(Debug)]
struct HotPreviewDocument {
    build_options: BuildOptions,
    warm_out_dir: PathBuf,
}

#[derive(Debug)]
struct HotPreviewTarget {
    path: PathBuf,
    source: String,
    edit_offset: usize,
}

impl HotPreviewState {
    fn new(options: &WatchOptions) -> Option<Self> {
        let preview_like = options.build_options.fast
            && options.build_options.once
            && options.build_options.precompile_preamble
            && options.build_options.runner == Runner::Direct;
        preview_like.then(|| Self {
            snapshots: HashMap::new(),
            static_document: None,
            warmed: false,
        })
    }

    fn remember_paths<'a>(&mut self, paths: impl Iterator<Item = &'a PathBuf>) {
        for path in paths {
            if !has_tex_source_extension(path) {
                continue;
            }
            let Some(canonical) = canonical_for_watch(path) else {
                continue;
            };
            if let Ok(source) = fs::read_to_string(&canonical) {
                self.snapshots.insert(canonical, source);
            }
        }
    }

    fn prewarm(&mut self, options: &WatchOptions) {
        if self.warmed {
            return;
        }
        let document = match self.prepare_document(options, &[]) {
            Ok(Some(document)) => document,
            Ok(None) => return,
            Err(error) => {
                eprintln!("warning: hot preview prewarm skipped: {error:#}");
                return;
            }
        };
        let mut warm_options = document.build_options;
        warm_options.out_dir = document.warm_out_dir;
        warm_options.quiet = true;
        if run_build_once(&warm_options, "hot preview prewarm") {
            self.warmed = true;
        }
    }

    fn run(
        &mut self,
        options: &WatchOptions,
        changed_paths: &[PathBuf],
        _label: &str,
    ) -> HotPreviewOutcome {
        let document = match self.prepare_document(options, changed_paths) {
            Ok(Some(document)) => document,
            Ok(None) => {
                self.static_document = None;
                return HotPreviewOutcome::NotApplicable;
            }
            Err(error) => {
                self.static_document = None;
                return HotPreviewOutcome::Failed(error);
            }
        };
        let ok = run_build_once(&document.build_options, "hot preview");
        self.remember_paths(changed_paths.iter());
        if ok {
            self.warmed = true;
            HotPreviewOutcome::Built(true)
        } else {
            self.static_document = None;
            HotPreviewOutcome::NotApplicable
        }
    }

    fn prepare_document(
        &mut self,
        options: &WatchOptions,
        changed_paths: &[PathBuf],
    ) -> Result<Option<HotPreviewDocument>> {
        let main = canonical_for_watch(&options.build_options.main)
            .unwrap_or_else(|| options.build_options.main.clone());
        let doc_dir = main
            .parent()
            .context("root TeX file has no parent directory")?
            .to_path_buf();
        let out_dir = absolute_output_dir(&options.build_options.out_dir)?;
        let Some(target) = self.select_target(&main, &out_dir, changed_paths)? else {
            return Ok(None);
        };
        let Some(snippet) = hot_preview_snippet(&main, &target)? else {
            return Ok(None);
        };

        let static_document_matches = self
            .static_document
            .as_ref()
            .is_some_and(|document| document.main == main && document.out_dir == out_dir);
        if !static_document_matches {
            let root_source_storage = if target.path == main {
                None
            } else {
                Some(fs::read_to_string(&main).with_context(|| {
                    format!("failed to read root TeX source {}", main.display())
                })?)
            };
            let root_source = root_source_storage.as_deref().unwrap_or(&target.source);
            let Some(mut preamble) = hot_preview_preamble(&doc_dir, root_source)? else {
                return Ok(None);
            };
            preamble.push_str(&hot_preview_definition_inputs(&doc_dir, root_source)?);
            preamble.push_str(&hot_preview_graphicspath(&doc_dir)?);

            let hmr_dir = out_dir.join(HOT_PREVIEW_DIR);
            fs::create_dir_all(&hmr_dir).with_context(|| {
                format!(
                    "failed to create hot preview directory {}",
                    hmr_dir.display()
                )
            })?;
            copy_hot_preview_local_support(&doc_dir, &hmr_dir)?;
            self.static_document = Some(HotPreviewStaticDocument {
                main: main.clone(),
                out_dir: out_dir.clone(),
                hmr_dir,
                preamble,
            });
        }
        let static_document = self
            .static_document
            .as_ref()
            .context("hot preview static document was not prepared")?;

        let hmr_main = static_document.hmr_dir.join("main.tex");
        let source = assemble_hot_preview_source(&static_document.preamble, &snippet, &target.path);
        write_if_changed(&hmr_main, source.as_bytes()).with_context(|| {
            format!("failed to write hot preview source {}", hmr_main.display())
        })?;

        let mut build_options = options.build_options.clone();
        build_options.main = hmr_main;
        build_options.job_name = Some(hot_preview_job_name(&options.build_options));
        build_options.engine = Engine::TekaiPdftex;
        build_options.runner = Runner::Direct;
        build_options.bib_mode = BibMode::None;
        build_options.draft_prepass = DraftPrepass::Never;
        build_options.out_dir = out_dir.clone();
        build_options.fast = true;
        build_options.once = true;
        build_options.max_runs = 1;
        build_options.force = true;
        build_options.precompile_preamble = true;
        build_options.synctex = false;
        build_options.print_command = false;

        Ok(Some(HotPreviewDocument {
            build_options,
            warm_out_dir: out_dir.join(HOT_PREVIEW_WARM_DIR),
        }))
    }

    fn select_target(
        &mut self,
        main: &Path,
        out_dir: &Path,
        changed_paths: &[PathBuf],
    ) -> Result<Option<HotPreviewTarget>> {
        let canonical_paths = changed_paths
            .iter()
            .filter_map(|path| canonical_for_watch(path))
            .collect::<Vec<_>>();
        if canonical_paths
            .iter()
            .any(|path| !hot_preview_can_slice_path(path, main, out_dir))
        {
            return Ok(None);
        }
        if let Some(canonical) = canonical_paths.into_iter().next() {
            let source = fs::read_to_string(&canonical).with_context(|| {
                format!("failed to read changed TeX source {}", canonical.display())
            })?;
            let edit_offset = self
                .snapshots
                .get(&canonical)
                .map(|previous| first_diff_offset(previous, &source))
                .unwrap_or_else(|| default_hot_preview_offset(&canonical, main, &source));
            return Ok(Some(HotPreviewTarget {
                path: canonical,
                source,
                edit_offset,
            }));
        }

        if !changed_paths.is_empty() {
            return Ok(None);
        }

        let source = fs::read_to_string(main)
            .with_context(|| format!("failed to read root TeX source {}", main.display()))?;
        Ok(Some(HotPreviewTarget {
            path: main.to_path_buf(),
            edit_offset: default_hot_preview_offset(main, main, &source),
            source,
        }))
    }
}

fn hot_preview_can_slice_path(path: &Path, main: &Path, out_dir: &Path) -> bool {
    has_tex_source_extension(path)
        && !is_under_output_dir(path, out_dir)
        && path
            .file_name()
            .and_then(OsStr::to_str)
            .is_some_and(|name| !name.starts_with('.'))
        && (path == main || !looks_like_structural_tex_file(path))
}

fn has_tex_source_extension(path: &Path) -> bool {
    extension_is_any(path, &["tex", "ltx"])
}

fn looks_like_structural_tex_file(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(OsStr::to_str) else {
        return true;
    };
    let lower = name.to_ascii_lowercase();
    lower.contains("command")
        || lower.contains("macro")
        || lower.contains("preamble")
        || lower.contains("header")
        || lower == "main.tex"
}

fn hot_preview_job_name(options: &BuildOptions) -> String {
    options.job_name.clone().unwrap_or_else(|| {
        options
            .main
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap_or("main")
            .to_string()
    })
}

fn hot_preview_preamble(doc_dir: &Path, root_source: &str) -> Result<Option<String>> {
    let Some(begin) = root_source.find("\\begin{document}") else {
        return Ok(None);
    };
    let mut preamble = root_source[..begin].to_string();
    for command in [
        "title",
        "author",
        "date",
        "thanks",
        "icmltitle",
        "icmlauthors",
        "icmlaffiliations",
        "icmlabstract",
        "icmlrunningtitle",
        "icmlpdfinfo",
        "sectionheaderline",
    ] {
        preamble = strip_latex_command_blocks(&preamble, command);
    }
    inline_hot_preview_inputs(doc_dir, &preamble, 0)
        .map(Some)
        .with_context(|| "failed to inline hot preview preamble inputs")
}

fn hot_preview_definition_inputs(doc_dir: &Path, root_source: &str) -> Result<String> {
    let Some(begin) = root_source.find("\\begin{document}") else {
        return Ok(String::new());
    };
    let body = &root_source[begin + "\\begin{document}".len()..];
    let scan_limit = body.len().min(8_192);
    let mut definitions = String::new();
    let mut offset = 0;
    while let Some((input_start, input_end, name)) = next_input_command(&body[..scan_limit], offset)
    {
        offset = input_end;
        let Some(path) = resolve_hot_preview_input(doc_dir, &name) else {
            continue;
        };
        if !definition_like_tex_file(&path)? {
            continue;
        }
        let source = fs::read_to_string(&path)
            .with_context(|| format!("failed to read definition input {}", path.display()))?;
        definitions.push_str("\n% begin hot-preview definition input ");
        definitions.push_str(&name);
        definitions.push('\n');
        definitions.push_str(&source);
        definitions.push_str("\n% end hot-preview definition input\n");
        if input_start > scan_limit {
            break;
        }
    }
    Ok(definitions)
}

fn hot_preview_graphicspath(doc_dir: &Path) -> Result<String> {
    let mut paths = vec![doc_dir.to_path_buf()];
    for name in ["figures", "figure", "images", "img", "toy_figures"] {
        let path = doc_dir.join(name);
        if path.is_dir() {
            paths.push(path);
        }
    }
    let mut source = String::from("\n\\makeatletter\n\\@ifpackageloaded{graphicx}{\\graphicspath{");
    for path in paths {
        source.push('{');
        source.push_str(&escape_tex_path(&path)?);
        source.push_str("/}");
    }
    source.push_str("}}{}\n\\makeatother\n");
    Ok(source)
}

fn hot_preview_snippet(main: &Path, target: &HotPreviewTarget) -> Result<Option<String>> {
    let (body, body_offset) = if target.path == main {
        let Some(begin) = target.source.find("\\begin{document}") else {
            return Ok(None);
        };
        if target.edit_offset < begin {
            return Ok(None);
        }
        let start = begin + "\\begin{document}".len();
        let end = target
            .source
            .find("\\end{document}")
            .unwrap_or(target.source.len());
        (&target.source[start..end], start)
    } else {
        (target.source.as_str(), 0)
    };
    let relative_offset = target
        .edit_offset
        .saturating_sub(body_offset)
        .min(body.len());
    let (start, end) = hot_preview_window(body, relative_offset);
    let mut snippet = body[start..end].to_string();
    for environment in [
        "figure",
        "figure*",
        "table",
        "table*",
        "tikzpicture",
        "algorithm",
        "algorithmic",
        "lstlisting",
        "minted",
    ] {
        snippet = strip_latex_environment(&snippet, environment);
    }
    for command in ["bibliography", "bibliographystyle", "addbibresource"] {
        snippet = strip_latex_command_blocks(&snippet, command);
    }
    snippet = strip_latex_command_blocks(&snippet, "maketitle");
    snippet = strip_latex_command_blocks(&snippet, "icmlmaketitle");
    snippet = strip_input_commands(&snippet);
    snippet = trim_unbalanced_latex_environments(&snippet).to_string();
    if snippet.trim().is_empty() {
        snippet = "\\noindent Live preview source changed.\n".to_string();
    }
    Ok(Some(snippet))
}

fn hot_preview_window(source: &str, offset: usize) -> (usize, usize) {
    if source.is_empty() {
        return (0, 0);
    }
    let offset = floor_char_boundary(source, offset.min(source.len()));
    let half = HOT_PREVIEW_CONTEXT_BYTES / 2;
    let mut start = floor_char_boundary(source, offset.saturating_sub(half));
    let mut end = ceil_char_boundary(source, (offset + half).min(source.len()));
    if let Some(paragraph_start) = source[..start].rfind("\n\n") {
        start = paragraph_start + 2;
    }
    if let Some(paragraph_end) = source[end..].find("\n\n") {
        end += paragraph_end;
    }
    if end.saturating_sub(start) > HOT_PREVIEW_MAX_SNIPPET_BYTES {
        start = floor_char_boundary(source, offset.saturating_sub(half));
        end = ceil_char_boundary(source, (offset + half).min(source.len()));
        if let Some(line_start) = source[..start].rfind('\n') {
            start = line_start + 1;
        }
        if let Some(line_end) = source[end..].find('\n') {
            end += line_end;
        }
    }
    (start.min(source.len()), end.min(source.len()))
}

fn default_hot_preview_offset(path: &Path, main: &Path, source: &str) -> usize {
    if path == main
        && let Some(begin) = source.find("\\begin{document}")
    {
        return begin + "\\begin{document}".len();
    }
    source.len() / 2
}

fn first_diff_offset(previous: &str, current: &str) -> usize {
    let limit = previous.len().min(current.len());
    for (index, (left, right)) in previous.bytes().zip(current.bytes()).enumerate() {
        if left != right {
            return floor_char_boundary(current, index);
        }
    }
    floor_char_boundary(current, limit)
}

fn assemble_hot_preview_source(preamble: &str, snippet: &str, target_path: &Path) -> String {
    format!(
        "% Generated by tekai watch --preview for {}.\n{}\\begin{{document}}\n\\pagestyle{{empty}}\n{}\n\\end{{document}}\n",
        target_path.display(),
        preamble,
        snippet
    )
}

fn inline_hot_preview_inputs(doc_dir: &Path, source: &str, depth: usize) -> Result<String> {
    if depth >= 4 {
        return Ok(source.to_string());
    }
    let mut output = String::with_capacity(source.len());
    let mut offset = 0;
    while let Some((start, end, name)) = next_input_command(source, offset) {
        output.push_str(&source[offset..start]);
        let replacement = resolve_hot_preview_input(doc_dir, &name)
            .filter(|path| hot_preview_inlineable_input(path).unwrap_or(false))
            .map(|path| -> Result<String> {
                let nested = fs::read_to_string(&path)
                    .with_context(|| format!("failed to read TeX input {}", path.display()))?;
                inline_hot_preview_inputs(doc_dir, &nested, depth + 1)
            })
            .transpose()?;
        if let Some(replacement) = replacement {
            output.push_str("\n% begin hot-preview inlined input ");
            output.push_str(&name);
            output.push('\n');
            output.push_str(&replacement);
            output.push_str("\n% end hot-preview inlined input\n");
        } else {
            output.push_str(&source[start..end]);
        }
        offset = end;
    }
    output.push_str(&source[offset..]);
    Ok(output)
}

fn next_input_command(source: &str, offset: usize) -> Option<(usize, usize, String)> {
    let mut search = offset;
    while let Some(relative) = source[search..].find("\\input") {
        let start = search + relative;
        let command_end = start + "\\input".len();
        if source[command_end..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            search = command_end;
            continue;
        }
        let Some((arg_start, arg_end, _name)) = braced_argument_after(source, command_end) else {
            search = command_end;
            continue;
        };
        return Some((
            start,
            arg_end,
            source[arg_start..arg_end - 1].trim().to_string(),
        ));
    }
    None
}

fn braced_argument_after(source: &str, mut offset: usize) -> Option<(usize, usize, String)> {
    while source[offset..]
        .chars()
        .next()
        .is_some_and(char::is_whitespace)
    {
        offset += source[offset..].chars().next()?.len_utf8();
    }
    if source[offset..].chars().next()? != '{' {
        return None;
    }
    let end = find_matching_delimiter(source, offset, '{', '}')?;
    Some((offset + 1, end + 1, source[offset + 1..end].to_string()))
}

fn resolve_hot_preview_input(doc_dir: &Path, name: &str) -> Option<PathBuf> {
    let mut candidate = PathBuf::from(name.trim());
    if candidate.extension().is_none() {
        candidate.set_extension("tex");
    }
    let path = if candidate.is_absolute() {
        candidate
    } else {
        doc_dir.join(candidate)
    };
    path.exists().then_some(path)
}

fn hot_preview_inlineable_input(path: &Path) -> Result<bool> {
    let metadata = fs::metadata(path)
        .with_context(|| format!("failed to stat TeX input {}", path.display()))?;
    if metadata.len() > HOT_PREVIEW_MAX_INLINE_BYTES {
        return Ok(false);
    }
    let source =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(!source.contains("\\begin{document}") && !source.contains("\\documentclass"))
}

fn definition_like_tex_file(path: &Path) -> Result<bool> {
    if !hot_preview_inlineable_input(path)? {
        return Ok(false);
    }
    let source =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    Ok(![
        "\\section",
        "\\subsection",
        "\\includegraphics",
        "\\bibliography",
        "\\begin{figure",
        "\\begin{table",
        "\\begin{abstract",
    ]
    .iter()
    .any(|needle| source.contains(needle)))
}

fn strip_latex_command_blocks(source: &str, command: &str) -> String {
    let needle = format!("\\{command}");
    let mut output = String::with_capacity(source.len());
    let mut offset = 0;
    while let Some(relative) = source[offset..].find(&needle) {
        let start = offset + relative;
        let command_end = start + needle.len();
        if source[command_end..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            output.push_str(&source[offset..command_end]);
            offset = command_end;
            continue;
        }
        output.push_str(&source[offset..start]);
        let end = latex_command_block_end(source, command_end).unwrap_or(command_end);
        offset = end;
    }
    output.push_str(&source[offset..]);
    output
}

fn latex_command_block_end(source: &str, mut offset: usize) -> Option<usize> {
    while source[offset..]
        .chars()
        .next()
        .is_some_and(char::is_whitespace)
    {
        offset += source[offset..].chars().next()?.len_utf8();
    }
    if source[offset..].starts_with('[') {
        let optional_end = find_matching_delimiter(source, offset, '[', ']')?;
        offset = optional_end + 1;
        while source[offset..]
            .chars()
            .next()
            .is_some_and(char::is_whitespace)
        {
            offset += source[offset..].chars().next()?.len_utf8();
        }
    }
    if source[offset..].starts_with('{') {
        return find_matching_delimiter(source, offset, '{', '}').map(|end| end + 1);
    }
    Some(offset)
}

fn strip_latex_environment(source: &str, environment: &str) -> String {
    let begin = format!("\\begin{{{environment}}}");
    let end = format!("\\end{{{environment}}}");
    let mut output = String::with_capacity(source.len());
    let mut offset = 0;
    while let Some(relative) = source[offset..].find(&begin) {
        let start = offset + relative;
        output.push_str(&source[offset..start]);
        let body_start = start + begin.len();
        if let Some(end_relative) = source[body_start..].find(&end) {
            offset = body_start + end_relative + end.len();
        } else {
            offset = body_start;
        }
    }
    output.push_str(&source[offset..]);
    output
}

fn strip_input_commands(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut offset = 0;
    while let Some((start, end, _name)) = next_input_command(source, offset) {
        output.push_str(&source[offset..start]);
        offset = end;
    }
    output.push_str(&source[offset..]);
    output
}

fn trim_unbalanced_latex_environments(source: &str) -> &str {
    let mut stack = Vec::<(String, usize)>::new();
    let mut safe_start = 0;
    let mut offset = 0;
    while let Some((start, end, begin, name)) = next_environment_command(source, offset) {
        if begin {
            stack.push((name, start));
        } else if stack.last().is_some_and(|(open, _)| open == &name) {
            stack.pop();
        } else {
            safe_start = end;
            stack.clear();
        }
        offset = end;
    }
    let safe_end = stack
        .first()
        .map(|(_, start)| *start)
        .unwrap_or(source.len());
    if safe_start >= safe_end {
        ""
    } else {
        &source[safe_start..safe_end]
    }
}

fn next_environment_command(
    source: &str,
    mut offset: usize,
) -> Option<(usize, usize, bool, String)> {
    while offset < source.len() {
        let begin = source[offset..].find("\\begin{").map(|found| {
            let start = offset + found;
            (start, true, start + "\\begin{".len())
        });
        let end = source[offset..].find("\\end{").map(|found| {
            let start = offset + found;
            (start, false, start + "\\end{".len())
        });
        let (start, is_begin, name_start) = match (begin, end) {
            (Some(begin), Some(end)) if begin.0 <= end.0 => begin,
            (Some(_), Some(end)) => end,
            (Some(begin), None) => begin,
            (None, Some(end)) => end,
            (None, None) => return None,
        };
        let relative_end = source[name_start..].find('}')?;
        let command_end = name_start + relative_end + 1;
        if environment_command_is_commented(source, start) {
            offset = command_end;
            continue;
        }
        let name = source[name_start..command_end - 1].trim();
        if name.is_empty() {
            offset = command_end;
            continue;
        }
        return Some((start, command_end, is_begin, name.to_string()));
    }
    None
}

fn environment_command_is_commented(source: &str, command_start: usize) -> bool {
    let line_start = source[..command_start]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let bytes = source.as_bytes();
    let mut index = line_start;
    while index < command_start {
        if bytes[index] != b'%' {
            index += 1;
            continue;
        }
        let mut backslashes = 0;
        let mut previous = index;
        while previous > line_start && bytes[previous - 1] == b'\\' {
            backslashes += 1;
            previous -= 1;
        }
        if backslashes % 2 == 0 {
            return true;
        }
        index += 1;
    }
    false
}

fn find_matching_delimiter(
    source: &str,
    open_index: usize,
    open: char,
    close: char,
) -> Option<usize> {
    let mut depth = 0usize;
    let mut escaped = false;
    for (relative, ch) in source[open_index..].char_indices() {
        let index = open_index + relative;
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
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return Some(index);
            }
        }
    }
    None
}

fn copy_hot_preview_local_support(doc_dir: &Path, hmr_dir: &Path) -> Result<()> {
    for entry in fs::read_dir(doc_dir)
        .with_context(|| format!("failed to read document directory {}", doc_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() || !extension_is_any(&path, &["sty", "cls", "clo", "def", "cfg"]) {
            continue;
        }
        let target = hmr_dir.join(entry.file_name());
        fs::copy(&path, &target).with_context(|| {
            format!(
                "failed to copy local TeX support {} to {}",
                path.display(),
                target.display()
            )
        })?;
    }
    Ok(())
}

fn write_if_changed(path: &Path, bytes: &[u8]) -> Result<()> {
    if fs::read(path).is_ok_and(|existing| existing == bytes) {
        return Ok(());
    }
    fs::write(path, bytes)?;
    Ok(())
}

fn escape_tex_path(path: &Path) -> Result<String> {
    let path = path
        .canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .replace('\\', "/");
    Ok(path.replace(' ', "\\space "))
}

fn floor_char_boundary(source: &str, mut index: usize) -> usize {
    index = index.min(source.len());
    while !source.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn ceil_char_boundary(source: &str, mut index: usize) -> usize {
    index = index.min(source.len());
    while index < source.len() && !source.is_char_boundary(index) {
        index += 1;
    }
    index
}

#[derive(Debug, Clone)]
struct WatchFilter {
    root: PathBuf,
    ignored_out_dir: PathBuf,
    dependency_paths: HashSet<PathBuf>,
    watched_dependency_dirs: HashSet<PathBuf>,
}

fn refresh_dependency_filter<W: Watcher>(
    options: &WatchOptions,
    filter: &mut WatchFilter,
    watcher: &mut W,
) {
    match watch_dependency_paths(options) {
        Ok(paths) => {
            filter.dependency_paths = paths
                .into_iter()
                .filter_map(|path| canonical_for_watch(&path))
                .collect();
            sync_dependency_watches(watcher, filter);
        }
        Err(error) => {
            eprintln!("warning: failed to refresh dependency watch set: {error:#}");
            filter.dependency_paths.clear();
            sync_dependency_watches(watcher, filter);
        }
    }
}

fn watch_dependency_paths(options: &WatchOptions) -> Result<Vec<PathBuf>> {
    let mut paths = build_dependency_paths(&options.build_options)?;
    if let Some(final_build_options) = &options.final_build_options {
        paths.extend(build_dependency_paths(final_build_options)?);
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn sync_dependency_watches<W: Watcher>(watcher: &mut W, filter: &mut WatchFilter) {
    let desired_dirs = dependency_watch_dirs(
        &filter.root,
        &filter.ignored_out_dir,
        &filter.dependency_paths,
    );

    let stale_dirs = filter
        .watched_dependency_dirs
        .difference(&desired_dirs)
        .cloned()
        .collect::<Vec<_>>();
    for dir in stale_dirs {
        if let Err(error) = watcher.unwatch(&dir) {
            eprintln!(
                "warning: failed to stop watching {}: {error}",
                dir.display()
            );
        }
        filter.watched_dependency_dirs.remove(&dir);
    }

    let new_dirs = desired_dirs
        .difference(&filter.watched_dependency_dirs)
        .cloned()
        .collect::<Vec<_>>();
    for dir in new_dirs {
        match watcher.watch(&dir, RecursiveMode::NonRecursive) {
            Ok(()) => {
                filter.watched_dependency_dirs.insert(dir);
            }
            Err(error) => eprintln!(
                "warning: failed to watch dependency dir {}: {error}",
                dir.display()
            ),
        }
    }
}

fn dependency_watch_dirs(
    root: &Path,
    ignored_out_dir: &Path,
    dependency_paths: &HashSet<PathBuf>,
) -> HashSet<PathBuf> {
    dependency_paths
        .iter()
        .filter(|path| !is_under_output_dir(path, ignored_out_dir))
        .filter(|path| !path_is_under_or_equal(path, root))
        .filter(|path| !is_probably_system_dependency(path))
        .filter_map(|path| path.parent().and_then(canonical_for_watch))
        .collect()
}

fn relevant_event_paths(event: &Event, filter: &WatchFilter) -> Option<Vec<PathBuf>> {
    let mut paths = event
        .paths
        .iter()
        .filter(|path| !is_ignored(path, &filter.ignored_out_dir))
        .filter(|path| {
            is_known_dependency(path, &filter.dependency_paths)
                || has_relevant_file_name(path)
                || has_relevant_extension(path)
        })
        .cloned()
        .collect::<Vec<_>>();
    if paths.is_empty() {
        None
    } else {
        paths.sort();
        paths.dedup();
        Some(paths)
    }
}

fn is_known_dependency(path: &Path, dependency_paths: &HashSet<PathBuf>) -> bool {
    canonical_for_watch(path).is_some_and(|path| dependency_paths.contains(&path))
}

fn has_relevant_extension(path: &Path) -> bool {
    extension_is_any(
        path,
        &[
            "tex", "ltx", "sty", "cls", "bib", "bst", "ist", "xdy", "png", "jpg", "jpeg", "pdf",
            "eps", "svg", "svgz", "asy", "mp", "gnuplot", "gp",
        ],
    )
}

fn has_relevant_file_name(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            ["biber.conf", ".biber.conf"]
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
        })
}

fn has_lint_extension(path: &Path) -> bool {
    extension_is_any(path, &["tex", "ltx", "cls"])
}

fn extension_is_any(path: &Path, extensions: &[&str]) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            extensions
                .iter()
                .any(|candidate| ext.eq_ignore_ascii_case(candidate))
        })
}

fn is_ignored(path: &Path, ignored_out_dir: &Path) -> bool {
    if is_under_output_dir(path, ignored_out_dir) {
        return true;
    }
    path.components().any(|component| {
        let name = component.as_os_str().to_string_lossy();
        matches!(name.as_ref(), ".git" | "target" | ".tekai")
    })
}

fn absolute_output_dir(path: &Path) -> Result<PathBuf> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    Ok(canonicalize_with_missing_tail(&absolute).unwrap_or(absolute))
}

fn canonicalize_with_missing_tail(path: &Path) -> Option<PathBuf> {
    let mut ancestor = path;
    let mut tail = Vec::new();
    loop {
        if let Ok(mut canonical) = ancestor.canonicalize() {
            for component in tail.iter().rev() {
                canonical.push(component);
            }
            return Some(canonical);
        }
        tail.push(ancestor.file_name()?.to_os_string());
        ancestor = ancestor.parent()?;
    }
}

fn is_under_output_dir(path: &Path, out_dir: &Path) -> bool {
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(path),
            Err(_) => path.to_path_buf(),
        }
    };
    absolute_path.starts_with(out_dir)
}

fn path_is_under_or_equal(path: &Path, directory: &Path) -> bool {
    canonical_for_watch(path)
        .zip(canonical_for_watch(directory))
        .is_some_and(|(path, directory)| path.starts_with(directory))
}

fn is_probably_system_dependency(path: &Path) -> bool {
    [
        "/Library/TeX",
        "/System",
        "/usr/local/texlive",
        "/usr/share/texlive",
        "/usr/share",
        "/usr/lib",
        "/usr/bin",
    ]
    .iter()
    .any(|prefix| path.starts_with(prefix))
}

fn canonical_for_watch(path: &Path) -> Option<PathBuf> {
    path.canonicalize().ok().or_else(|| {
        if path.is_absolute() {
            Some(path.to_path_buf())
        } else {
            std::env::current_dir().ok().map(|cwd| cwd.join(path))
        }
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::thread;

    use crate::compiler::{BibMode, DraftPrepass, Engine, Runner, build};
    use notify::event::{DataChange, ModifyKind};
    use notify::{Event, EventKind};

    use super::*;

    #[test]
    fn dependency_paths_are_relevant_even_without_known_extension() {
        let root = unique_temp_dir("tekai-watch-filter-test");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let data_path = root.join("values.dat");
        fs::write(&data_path, "1 2 3\n").expect("failed to write dependency");
        let ignored_out_dir = root.join("out");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir,
            dependency_paths: HashSet::from([data_path.canonicalize().unwrap()]),
            watched_dependency_dirs: HashSet::new(),
        };
        let event = Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content)))
            .add_path(data_path.clone());

        assert!(relevant_event_paths(&event, &filter).is_some());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn unknown_extensions_are_ignored_when_not_recorded_dependencies() {
        let root = unique_temp_dir("tekai-watch-filter-test");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let data_path = root.join("values.dat");
        fs::write(&data_path, "1 2 3\n").expect("failed to write dependency");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: root.join("out"),
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };
        let event = Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content)))
            .add_path(data_path.clone());

        assert!(relevant_event_paths(&event, &filter).is_none());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn dependency_watch_dirs_include_external_dependency_parent() {
        let root = unique_temp_dir("tekai-watch-filter-test");
        let external = unique_temp_dir("tekai-watch-filter-external");
        fs::create_dir_all(&root).expect("failed to create root directory");
        fs::create_dir_all(&external).expect("failed to create external directory");
        let external_dep = external.join("values.dat");
        fs::write(&external_dep, "1 2 3\n").expect("failed to write dependency");
        let inside_root = root.join("inside.dat");
        fs::write(&inside_root, "4 5 6\n").expect("failed to write root dependency");
        let output_dir = root.join("out");
        fs::create_dir_all(&output_dir).expect("failed to create output directory");
        let output_dep = output_dir.join("main.aux");
        fs::write(&output_dep, "generated\n").expect("failed to write output dependency");

        let dependency_paths = HashSet::from([
            external_dep.canonicalize().unwrap(),
            inside_root.canonicalize().unwrap(),
            output_dep.canonicalize().unwrap(),
        ]);
        let dirs = dependency_watch_dirs(&root, &output_dir, &dependency_paths);

        assert!(
            dirs.contains(&external.canonicalize().unwrap()),
            "{dirs:#?}"
        );
        assert!(!dirs.contains(&root.canonicalize().unwrap()), "{dirs:#?}");
        assert!(
            !dirs.contains(&output_dir.canonicalize().unwrap()),
            "{dirs:#?}"
        );

        let _ = fs::remove_dir_all(root);
        let _ = fs::remove_dir_all(external);
    }

    #[test]
    fn source_directory_named_build_is_not_ignored_unless_it_is_the_output_dir() {
        let root = unique_temp_dir("tekai-watch-build-source");
        fs::create_dir_all(root.join("build")).expect("failed to create source build directory");
        let source = root.join("build").join("section.tex");
        fs::write(&source, "\\section{Build}\n").expect("failed to write source file");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: root.join("out"),
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };
        let event =
            Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content))).add_path(source);

        assert!(relevant_event_paths(&event, &filter).is_some());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn configured_output_directory_is_still_ignored() {
        let root = unique_temp_dir("tekai-watch-output-filter");
        let out_dir = root.join("build");
        fs::create_dir_all(&out_dir).expect("failed to create output directory");
        let generated = out_dir.join("main.tex");
        fs::write(&generated, "\\relax\n").expect("failed to write generated file");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: out_dir,
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };
        let event = Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content)))
            .add_path(generated);

        assert!(relevant_event_paths(&event, &filter).is_none());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn makeindex_style_files_are_relevant_watch_events() {
        let root = unique_temp_dir("tekai-watch-index-style");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let ist = root.join("main.ist");
        let xdy = root.join("main.xdy");
        fs::write(&ist, "headings_flag 1\n").expect("failed to write ist file");
        fs::write(&xdy, "(markup-index :open \"\")\n").expect("failed to write xdy file");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: root.join("out"),
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };

        for path in [ist, xdy] {
            let event =
                Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content))).add_path(path);
            assert!(relevant_event_paths(&event, &filter).is_some());
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn svg_files_are_relevant_watch_events() {
        let root = unique_temp_dir("tekai-watch-svg-event");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let svg = root.join("figure.svg");
        let svgz = root.join("compressed.svgz");
        fs::write(&svg, "<svg xmlns=\"http://www.w3.org/2000/svg\"/>\n")
            .expect("failed to write svg file");
        fs::write(&svgz, "<svg xmlns=\"http://www.w3.org/2000/svg\"/>\n")
            .expect("failed to write svgz file");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: root.join("out"),
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };

        for path in [svg, svgz] {
            let event =
                Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content))).add_path(path);
            assert!(relevant_event_paths(&event, &filter).is_some());
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn relevant_watch_extensions_are_case_insensitive() {
        let root = unique_temp_dir("tekai-watch-extension-case");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: root.join("out"),
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };

        for name in ["MAIN.TEX", "FIGURE.SVG", "PLOT.EPS", "REFS.BIB"] {
            let path = root.join(name);
            fs::write(&path, "\n").expect("failed to write temp file");
            let event =
                Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content))).add_path(path);
            assert!(relevant_event_paths(&event, &filter).is_some(), "{name}");
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn external_tool_source_files_are_relevant_watch_events() {
        let root = unique_temp_dir("tekai-watch-external-tool-source");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: root.join("out"),
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };

        for name in ["figure.asy", "figure.mp", "plot.gnuplot", "plot.gp"] {
            let path = root.join(name);
            fs::write(&path, "\n").expect("failed to write temp file");
            let event =
                Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content))).add_path(path);
            assert!(relevant_event_paths(&event, &filter).is_some(), "{name}");
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn biber_config_files_are_relevant_watch_events() {
        let root = unique_temp_dir("tekai-watch-biber-config");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: root.join("out"),
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };

        for name in ["biber.conf", ".biber.conf", "BIBER.CONF"] {
            let path = root.join(name);
            fs::write(&path, "\n").expect("failed to write temp file");
            let event =
                Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content))).add_path(path);
            assert!(relevant_event_paths(&event, &filter).is_some(), "{name}");
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn changed_lint_scope_only_lints_tex_like_paths() {
        let root = unique_temp_dir("tekai-watch-lint-targets");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let main = root.join("main.tex");
        let style = root.join("paper.sty");
        let bib = root.join("refs.bib");
        let figure = root.join("figure.png");
        for path in [&main, &style, &bib, &figure] {
            fs::write(path, "\n").expect("failed to write temp file");
        }

        let options = watch_options(&main, &root);
        let targets = lint_targets(
            &options,
            &LintScope::Changed(vec![
                main.clone(),
                bib.clone(),
                style.clone(),
                figure.clone(),
            ]),
        )
        .expect("failed to resolve lint targets");

        assert_eq!(targets, vec![main.canonicalize().unwrap()]);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn changed_lint_scope_matches_tex_like_paths_case_insensitively() {
        let root = unique_temp_dir("tekai-watch-lint-extension-case");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let main = root.join("MAIN.TEX");
        let style = root.join("PAPER.STY");
        let bib = root.join("REFS.BIB");
        for path in [&main, &style, &bib] {
            fs::write(path, "\n").expect("failed to write temp file");
        }

        let options = watch_options(&main, &root);
        let targets = lint_targets(
            &options,
            &LintScope::Changed(vec![bib.clone(), style.clone(), main.clone()]),
        )
        .expect("failed to resolve lint targets");

        assert_eq!(targets, vec![main.canonicalize().unwrap()]);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn changed_lint_scope_skips_non_tex_events() {
        let root = unique_temp_dir("tekai-watch-lint-empty");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let main = root.join("main.tex");
        let bib = root.join("refs.bib");
        fs::write(&main, "\\documentclass{article}\n").expect("failed to write main");
        fs::write(&bib, "\n").expect("failed to write bibliography");

        let options = watch_options(&main, &root);
        let targets = lint_targets(&options, &LintScope::Changed(vec![bib]))
            .expect("failed to resolve lint targets");

        assert!(targets.is_empty(), "{targets:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn debounce_extends_until_relevant_events_are_quiet() {
        let root = unique_temp_dir("tekai-watch-debounce");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let first = root.join("main.tex");
        let second = root.join("section.tex");
        let third = root.join("refs.bib");
        for path in [&first, &second, &third] {
            fs::write(path, "\n").expect("failed to write temp file");
        }
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: root.join("out"),
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };
        let (tx, rx) = mpsc::channel();
        let second_event =
            Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content))).add_path(second);
        let third_event =
            Event::new(EventKind::Modify(ModifyKind::Data(DataChange::Content))).add_path(third);
        tx.send(Ok(second_event))
            .expect("failed to queue second debounce event");
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(35));
            tx.send(Ok(third_event))
                .expect("failed to send third debounce event");
        });

        let paths = debounced_relevant_paths(
            &rx,
            Duration::from_millis(200),
            Duration::from_millis(800),
            &filter,
            vec![first.clone()],
        );

        assert_eq!(
            paths,
            vec![first, root.join("refs.bib"), root.join("section.tex")]
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn hot_preview_uses_low_latency_debounce() {
        assert_eq!(
            debounce_durations(true),
            (Duration::from_millis(12), Duration::from_millis(100))
        );
    }

    #[test]
    fn idle_wait_ignores_irrelevant_events() {
        let root = unique_temp_dir("tekai-watch-idle-ignore");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let ignored = root.join("notes.tmp");
        fs::write(&ignored, "\n").expect("failed to write ignored file");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: root.join("out"),
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };
        let (tx, rx) = mpsc::channel();
        tx.send(Ok(Event::new(EventKind::Modify(ModifyKind::Data(
            DataChange::Content,
        )))
        .add_path(ignored)))
            .expect("failed to send ignored event");

        assert!(matches!(
            wait_for_relevant_event(&rx, Duration::from_millis(30), &filter),
            WaitForEvent::Idle
        ));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn idle_wait_returns_relevant_events() {
        let root = unique_temp_dir("tekai-watch-idle-relevant");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let source = root.join("main.tex");
        fs::write(&source, "\n").expect("failed to write source file");
        let filter = WatchFilter {
            root: root.clone(),
            ignored_out_dir: root.join("out"),
            dependency_paths: HashSet::new(),
            watched_dependency_dirs: HashSet::new(),
        };
        let (tx, rx) = mpsc::channel();
        tx.send(Ok(Event::new(EventKind::Modify(ModifyKind::Data(
            DataChange::Content,
        )))
        .add_path(source.clone())))
            .expect("failed to send source event");

        match wait_for_relevant_event(&rx, Duration::from_millis(200), &filter) {
            WaitForEvent::Event(event) => assert_eq!(event.paths, vec![source]),
            other => panic!("expected relevant event, got {other:?}"),
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn preview_watch_dependency_paths_include_final_build_state() {
        let root = unique_temp_dir("tekai-watch-final-dependencies");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}Hi\\end{document}\n",
        )
        .expect("failed to write main source");
        let final_options = watch_options(&main, &root).build_options;
        build(&final_options).expect("failed to seed final build state");
        let mut options = watch_options(&main, &root);
        options.build_options.fast = true;
        options.build_options.once = true;
        options.final_build_options = Some(final_options);

        let paths = watch_dependency_paths(&options).expect("failed to read watch dependencies");

        assert!(
            paths.iter().any(|path| path.ends_with("main.tex")),
            "{paths:#?}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn watch_dependency_paths_seed_source_inputs_before_first_build() {
        let root = unique_temp_dir("tekai-watch-seed-dependencies");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let main = root.join("main.tex");
        let data = root.join("values.dat");
        let refs = root.join("refs.bib");
        let biber_refs = root.join("bib").join("more.bib");
        let style = root.join("custom.bst");
        let class = root.join("customclass.cls");
        let package = root.join("localpkg.sty");
        let figure = root.join("figures").join("plot.pdf");
        let animation_frame_0 = root.join("figures").join("frames").join("frame-0.png");
        let animation_frame_1 = root.join("figures").join("frames").join("frame-1.png");
        let animation_frame_2 = root.join("figures").join("frames").join("frame-2.png");
        let svg = root.join("figures").join("icon.svg");
        let included_pdf = root.join("figures").join("supplement.pdf");
        let snippet = root.join("snippets").join("example.py");
        let listing = root.join("snippets").join("data.json");
        let kpathsea_listing = root
            .join("shared")
            .join("data")
            .join("kpathsea-listing.json");
        let plot_points = root.join("tables").join("points.dat");
        let plot_curve = root.join("tables").join("curve.csv");
        let kpathsea_curve = root.join("shared").join("data").join("kpathsea-curve.csv");
        let datatool_measurements = root.join("tables").join("measurements.csv");
        let csvsimple_rows = root.join("tables").join("rows.csv");
        let kpathsea_csv_rows = root.join("shared").join("data").join("kpathsea-rows.csv");
        let external_aux = root.join("refs").join("supplement.aux");
        let external_nested_aux = root.join("refs").join("sections").join("chapter.aux");
        let zref_aux = root.join("refs").join("zref.aux");
        let zref_nested_aux = root.join("refs").join("sections").join("zchapter.aux");
        let standalone = root.join("figures").join("standalone.tex");
        let standalone_data = root.join("figures").join("standalone-data.txt");
        let media = root.join("media").join("demo.mp4");
        let attachment = root.join("artifacts").join("data.csv");
        fs::create_dir_all(root.join("bib")).expect("failed to create bibliography directory");
        fs::create_dir_all(root.join("figures")).expect("failed to create figure directory");
        fs::create_dir_all(root.join("figures").join("frames"))
            .expect("failed to create animation frame directory");
        fs::create_dir_all(root.join("shared").join("data"))
            .expect("failed to create shared data directory");
        fs::create_dir_all(root.join("snippets")).expect("failed to create snippets directory");
        fs::create_dir_all(root.join("tables")).expect("failed to create tables directory");
        fs::create_dir_all(root.join("refs")).expect("failed to create external refs directory");
        fs::create_dir_all(root.join("refs").join("sections"))
            .expect("failed to create nested external refs directory");
        fs::create_dir_all(root.join("media")).expect("failed to create media directory");
        fs::create_dir_all(root.join("artifacts")).expect("failed to create artifacts directory");
        fs::write(
            &main,
            "\\documentclass{customclass}\n\
             \\usepackage{localpkg}\n\
             \\graphicspath{{figures/}}\n\
             \\includegraphics{plot}\n\
             \\animategraphics[controls]{12}{frames/frame-}{0}{2}\n\
             \\includesvg{icon}\n\
             \\includepdf{supplement}\n\
             \\inputminted{python}{snippets/example.py}\n\
             \\lstinputlisting{snippets/data.json}\n\
             \\lstinputlisting{data/kpathsea-listing.json}\n\
             \\DTLloaddb[noheader]{measurements}{tables/measurements.csv}\n\
             \\csvreader[head to column names]{tables/rows.csv}{}{\\name}\n\
             \\csvreader{data/kpathsea-rows.csv}{}{\\name}\n\
             \\externaldocument[prefix-]{refs/supplement}\n\
             \\zexternaldocument[z-]{refs/zref}\n\
             \\pgfplotstableread[col sep=comma]{tables/curve.csv}\\curve\n\
             \\pgfplotstableread[col sep=comma]{data/kpathsea-curve.csv}\\sharedcurve\n\
             \\addplot+[blue] table[x=x,y=y] {tables/points.dat};\n\
             \\includestandalone{figures/standalone}\n\
             \\includemedia{Poster}{media/demo.mp4}\n\
             \\attachfile{artifacts/data.csv}\n\
             \\bibliography{refs}\n\
             \\bibliographystyle{custom}\n\
             \\addbibresource[datatype=bibtex]{bib/more.bib}\n\
             \\begin{document}\\input{values.dat}\\end{document}\n",
        )
        .expect("failed to write main source");
        fs::write(&data, [0xff]).expect("failed to write binary data dependency");
        fs::write(&refs, "@book{x,title={X}}\n").expect("failed to write bibliography");
        fs::write(&biber_refs, "@book{y,title={Y}}\n").expect("failed to write Biber bibliography");
        fs::write(&style, "ENTRY{}{}{}\n").expect("failed to write bibliography style");
        fs::write(&class, "\\NeedsTeXFormat{LaTeX2e}\n").expect("failed to write class");
        fs::write(&package, "\\newcommand{\\localpkg}{}\n").expect("failed to write package");
        fs::write(&figure, "%PDF placeholder\n").expect("failed to write figure");
        fs::write(&animation_frame_0, "not a real PNG\n")
            .expect("failed to write animation frame 0");
        fs::write(&animation_frame_1, "not a real PNG\n")
            .expect("failed to write animation frame 1");
        fs::write(&animation_frame_2, "not a real PNG\n")
            .expect("failed to write animation frame 2");
        fs::write(&svg, "<svg/>").expect("failed to write SVG");
        fs::write(&included_pdf, "%PDF supplement\n").expect("failed to write included PDF");
        fs::write(&snippet, "print('hello')\n").expect("failed to write snippet");
        fs::write(&listing, "{\"ok\": true}\n").expect("failed to write listing");
        fs::write(&kpathsea_listing, "{\"shared\": true}\n")
            .expect("failed to write Kpathsea listing");
        fs::write(&plot_points, "x y\n0 0\n1 1\n").expect("failed to write plot points");
        fs::write(&plot_curve, "x,y\n0,0\n1,1\n").expect("failed to write plot curve");
        fs::write(&kpathsea_curve, "x,y\n2,3\n").expect("failed to write Kpathsea plot curve");
        fs::write(&datatool_measurements, "name,value\nalpha,1\n")
            .expect("failed to write datatool measurements");
        fs::write(&csvsimple_rows, "name,value\nbeta,2\n").expect("failed to write csv rows");
        fs::write(&kpathsea_csv_rows, "name,value\ngamma,3\n")
            .expect("failed to write Kpathsea csv rows");
        fs::write(
            &external_aux,
            "\\relax\n\\@input{sections/chapter.aux}\n\\newlabel{x}{{1}{1}}\n",
        )
        .expect("failed to write external aux");
        fs::write(&external_nested_aux, "\\newlabel{nested}{{2}{2}}\n")
            .expect("failed to write nested external aux");
        fs::write(
            &zref_aux,
            "\\relax\n\\@input{sections/zchapter.aux}\n\\zref@newlabel{z}{}\n",
        )
        .expect("failed to write zref aux");
        fs::write(&zref_nested_aux, "\\zref@newlabel{znested}{}\n")
            .expect("failed to write nested zref aux");
        fs::write(&standalone, "\\input{figures/standalone-data.txt}\n")
            .expect("failed to write standalone source");
        fs::write(&standalone_data, "standalone data\n").expect("failed to write standalone data");
        fs::write(&media, "not a real mp4\n").expect("failed to write media");
        fs::write(&attachment, "value\n").expect("failed to write attachment");
        let options = watch_options(&main, &root);

        let paths = watch_dependency_paths(&options).expect("failed to read watch dependencies");

        assert!(
            paths.iter().any(|path| path.ends_with("values.dat")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("refs.bib")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("bib/more.bib")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("custom.bst")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("customclass.cls")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("localpkg.sty")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("figures/plot.pdf")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("figures/frames/frame-0.png")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("figures/frames/frame-1.png")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("figures/frames/frame-2.png")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("figures/icon.svg")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("figures/supplement.pdf")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("snippets/example.py")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("snippets/data.json")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("shared/data/kpathsea-listing.json")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("tables/points.dat")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("tables/curve.csv")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("shared/data/kpathsea-curve.csv")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("tables/measurements.csv")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("tables/rows.csv")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("shared/data/kpathsea-rows.csv")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("refs/supplement.aux")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("refs/sections/chapter.aux")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("refs/zref.aux")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("refs/sections/zchapter.aux")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("figures/standalone.tex")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("figures/standalone-data.txt")),
            "{paths:#?}"
        );
        assert!(
            paths.iter().any(|path| path.ends_with("media/demo.mp4")),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path.ends_with("artifacts/data.csv")),
            "{paths:#?}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn hot_preview_preamble_strips_metadata_and_inlines_macro_inputs() {
        let root = unique_temp_dir("tekai-hot-preview-preamble");
        fs::create_dir_all(root.join("content")).expect("failed to create content directory");
        fs::write(
            root.join("math_commands.tex"),
            "\\newcommand{\\vect}[1]{\\mathbf{#1}}\n",
        )
        .expect("failed to write macro input");
        fs::write(root.join("content").join("abstract.tex"), "Abstract body\n")
            .expect("failed to write abstract input");
        let source = "\\documentclass{article}\n\
            \\title{A title}\n\
            \\author{Alice \\And Bob}\n\
            \\icmlabstract{\\input{content/abstract}}\n\
            \\input{math_commands}\n\
            \\begin{document}Body\\end{document}\n";

        let preamble = hot_preview_preamble(&root, source)
            .expect("preamble generation failed")
            .expect("preamble should be generated");

        assert!(!preamble.contains("\\title"), "{preamble}");
        assert!(!preamble.contains("\\author"), "{preamble}");
        assert!(!preamble.contains("\\And"), "{preamble}");
        assert!(!preamble.contains("Abstract body"), "{preamble}");
        assert!(preamble.contains("\\newcommand{\\vect}"), "{preamble}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn hot_preview_moves_definition_only_document_inputs_to_preamble() {
        let root = unique_temp_dir("tekai-hot-preview-definitions");
        fs::create_dir_all(root.join("content")).expect("failed to create content directory");
        fs::write(
            root.join("document_commands.tex"),
            "\\newcommand{\\important}[1]{\\textbf{#1}}\n",
        )
        .expect("failed to write document commands");
        fs::write(root.join("content").join("body.tex"), "\\section{Body}\n")
            .expect("failed to write body");
        let source = "\\documentclass{article}\n\
            \\begin{document}\n\
            \\input{document_commands}\n\
            \\input{content/body}\n\
            \\end{document}\n";

        let definitions = hot_preview_definition_inputs(&root, source)
            .expect("definition input extraction failed");

        assert!(definitions.contains("\\important"), "{definitions}");
        assert!(!definitions.contains("\\section{Body}"), "{definitions}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn hot_preview_snippet_rejects_root_preamble_edits() {
        let root = unique_temp_dir("tekai-hot-preview-root-edit");
        let main = root.join("main.tex");
        let source = "\\documentclass{article}\n\
            \\newcommand{\\x}{1}\n\
            \\begin{document}\n\
            Before edit.\n\nAfter edit.\n\
            \\end{document}\n";
        let preamble_target = HotPreviewTarget {
            path: main.clone(),
            source: source.to_string(),
            edit_offset: source.find("\\newcommand").unwrap(),
        };
        let body_target = HotPreviewTarget {
            path: main.clone(),
            source: source.to_string(),
            edit_offset: source.find("After edit").unwrap(),
        };

        assert!(
            hot_preview_snippet(&main, &preamble_target)
                .expect("snippet extraction failed")
                .is_none()
        );
        let snippet = hot_preview_snippet(&main, &body_target)
            .expect("snippet extraction failed")
            .expect("body edit should produce a snippet");
        assert!(snippet.contains("After edit"), "{snippet}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn hot_preview_snippet_trims_an_unmatched_environment_tail() {
        let root = unique_temp_dir("tekai-hot-preview-environment-tail");
        let main = root.join("main.tex");
        let first_abstract_paragraph = "a".repeat(600);
        let second_abstract_paragraph = "b".repeat(600);
        let source = format!(
            "\\documentclass{{article}}\n\\begin{{document}}\n\\begin{{abstract}}\n{first_abstract_paragraph}\n\n{second_abstract_paragraph}\n\\end{{abstract}}\n\n\\section{{Introduction}}\nEdited body text.\n\\end{{document}}\n"
        );
        let target = HotPreviewTarget {
            path: main.clone(),
            edit_offset: source.find("Introduction").unwrap(),
            source,
        };

        let snippet = hot_preview_snippet(&main, &target)
            .expect("snippet extraction failed")
            .expect("body edit should produce a snippet");

        assert!(snippet.contains("\\section{Introduction}"), "{snippet}");
        assert!(!snippet.contains("\\end{abstract}"), "{snippet}");
    }

    #[test]
    fn hot_preview_rejects_mixed_or_structural_change_sets() {
        let root = unique_temp_dir("tekai-hot-preview-structural-change");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let main = root.join("main.tex");
        let commands = root.join("document_commands.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}Body\\end{document}\n",
        )
        .expect("failed to write root source");
        fs::write(&commands, "\\newcommand{\\x}{1}\n").expect("failed to write structural source");
        let main = main.canonicalize().unwrap();
        let mut state = HotPreviewState {
            snapshots: HashMap::new(),
            static_document: None,
            warmed: true,
        };

        let target = state
            .select_target(&main, &root.join("out"), &[commands])
            .expect("target selection failed");

        assert!(target.is_none());
        let _ = fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn output_dir_canonicalizes_an_existing_symlinked_parent() {
        let root = unique_temp_dir("tekai-watch-output-symlink");
        let actual = root.join("actual");
        let alias = root.join("alias");
        fs::create_dir_all(&actual).expect("failed to create actual output parent");
        std::os::unix::fs::symlink(&actual, &alias)
            .expect("failed to create output parent symlink");

        let output = absolute_output_dir(&alias.join("out")).expect("failed to resolve output");

        assert_eq!(output, actual.canonicalize().unwrap().join("out"));
        let _ = fs::remove_dir_all(root);
    }

    fn watch_options(main: &Path, root: &Path) -> WatchOptions {
        WatchOptions {
            main: main.to_path_buf(),
            root: root.to_path_buf(),
            build_options: BuildOptions {
                main: main.to_path_buf(),
                job_name: None,
                engine: Engine::PdfLatex,
                runner: Runner::Direct,
                bib_mode: BibMode::Auto,
                out_dir: root.join("out"),
                fast: false,
                draft_prepass: DraftPrepass::Never,
                once: false,
                max_runs: 8,
                force: false,
                precompile_preamble: false,
                synctex: false,
                shell_escape: false,
                quiet: true,
                print_command: false,
            },
            final_build_options: None,
            final_after_idle: None,
            lint_config: LintConfig::default(),
            lint: true,
            fail_on_warnings: true,
        }
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let unique = format!(
            "{}-{}-{}-{}",
            prefix,
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock before UNIX epoch")
                .as_nanos()
        );
        std::env::temp_dir().join(unique)
    }
}

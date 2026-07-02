use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use notify::{Event, RecursiveMode, Watcher};

use crate::compiler::{BuildOptions, build, build_dependency_paths};
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
    eprintln!("watching {}", options.root.display());
    let initial_build_ok = run_once(&options, LintScope::Full, "rebuild");
    refresh_dependency_filter(&options, &mut filter, &mut watcher);

    let mut pending_event = None;
    if initial_build_ok
        && let (Some(final_build_options), Some(final_after_idle)) =
            (&options.final_build_options, options.final_after_idle)
    {
        match wait_for_relevant_event(&rx, final_after_idle, &filter) {
            WaitForEvent::Idle => {
                eprintln!("--- texpilot final rebuild {:?} ---", Instant::now());
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
        let paths = debounced_relevant_paths(
            &rx,
            Duration::from_millis(120),
            Duration::from_millis(1_000),
            &filter,
            paths,
        );
        let build_ok = run_once(&options, LintScope::Changed(paths), "rebuild");
        refresh_dependency_filter(&options, &mut filter, &mut watcher);
        if build_ok
            && let (Some(final_build_options), Some(final_after_idle)) =
                (&options.final_build_options, options.final_after_idle)
        {
            match wait_for_relevant_event(&rx, final_after_idle, &filter) {
                WaitForEvent::Idle => {
                    eprintln!("--- texpilot final rebuild {:?} ---", Instant::now());
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

fn run_once(options: &WatchOptions, lint_scope: LintScope, label: &str) -> bool {
    eprintln!("--- texpilot {label} {:?} ---", Instant::now());
    if options.lint {
        match lint_targets(options, lint_scope).and_then(|targets| {
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

fn lint_targets(options: &WatchOptions, lint_scope: LintScope) -> Result<Vec<PathBuf>> {
    match lint_scope {
        LintScope::Full => Ok(vec![options.root.clone()]),
        LintScope::Changed(paths) => {
            let mut targets = paths
                .into_iter()
                .filter(|path| has_lint_extension(path))
                .filter(|path| path.exists())
                .filter_map(|path| canonical_for_watch(&path))
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
    extension_is_any(path, &["tex", "ltx", "sty", "cls"])
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
        matches!(name.as_ref(), ".git" | "target" | ".texpilot")
    })
}

fn absolute_output_dir(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
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
        let root = unique_temp_dir("texpilot-watch-filter-test");
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
        let root = unique_temp_dir("texpilot-watch-filter-test");
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
        let root = unique_temp_dir("texpilot-watch-filter-test");
        let external = unique_temp_dir("texpilot-watch-filter-external");
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
        let root = unique_temp_dir("texpilot-watch-build-source");
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
        let root = unique_temp_dir("texpilot-watch-output-filter");
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
        let root = unique_temp_dir("texpilot-watch-index-style");
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
        let root = unique_temp_dir("texpilot-watch-svg-event");
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
        let root = unique_temp_dir("texpilot-watch-extension-case");
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
        let root = unique_temp_dir("texpilot-watch-external-tool-source");
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
        let root = unique_temp_dir("texpilot-watch-biber-config");
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
        let root = unique_temp_dir("texpilot-watch-lint-targets");
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
            LintScope::Changed(vec![
                main.clone(),
                bib.clone(),
                style.clone(),
                figure.clone(),
            ]),
        )
        .expect("failed to resolve lint targets");

        assert_eq!(
            targets,
            vec![main.canonicalize().unwrap(), style.canonicalize().unwrap()]
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn changed_lint_scope_matches_tex_like_paths_case_insensitively() {
        let root = unique_temp_dir("texpilot-watch-lint-extension-case");
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
            LintScope::Changed(vec![bib.clone(), style.clone(), main.clone()]),
        )
        .expect("failed to resolve lint targets");

        assert_eq!(
            targets,
            vec![main.canonicalize().unwrap(), style.canonicalize().unwrap()]
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn changed_lint_scope_skips_non_tex_events() {
        let root = unique_temp_dir("texpilot-watch-lint-empty");
        fs::create_dir_all(&root).expect("failed to create temp directory");
        let main = root.join("main.tex");
        let bib = root.join("refs.bib");
        fs::write(&main, "\\documentclass{article}\n").expect("failed to write main");
        fs::write(&bib, "\n").expect("failed to write bibliography");

        let options = watch_options(&main, &root);
        let targets = lint_targets(&options, LintScope::Changed(vec![bib]))
            .expect("failed to resolve lint targets");

        assert!(targets.is_empty(), "{targets:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn debounce_extends_until_relevant_events_are_quiet() {
        let root = unique_temp_dir("texpilot-watch-debounce");
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
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(35));
            tx.send(Ok(second_event))
                .expect("failed to send second debounce event");
            thread::sleep(Duration::from_millis(60));
            tx.send(Ok(third_event))
                .expect("failed to send third debounce event");
        });

        let paths = debounced_relevant_paths(
            &rx,
            Duration::from_millis(70),
            Duration::from_millis(300),
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
    fn idle_wait_ignores_irrelevant_events() {
        let root = unique_temp_dir("texpilot-watch-idle-ignore");
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
        let root = unique_temp_dir("texpilot-watch-idle-relevant");
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
        let root = unique_temp_dir("texpilot-watch-final-dependencies");
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
        let root = unique_temp_dir("texpilot-watch-seed-dependencies");
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

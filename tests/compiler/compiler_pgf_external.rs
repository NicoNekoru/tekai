use std::fs;
use std::path::{Path, PathBuf};

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const PGF_EXTERNAL_DOC: &str = r#"\documentclass{article}
\usepackage{tikz}
\usetikzlibrary{external}
\tikzexternalize[mode=list and make]
\begin{document}
Before.
\begin{tikzpicture}
\draw[blue, thick] (0,0) circle (1cm);
\end{tikzpicture}
Middle.
\begin{tikzpicture}
\draw[red, thick] (0,0) rectangle (1cm,1cm);
\end{tikzpicture}
After.
\end{document}
"#;

const PGF_DEFAULT_EXTERNAL_DOC: &str = r#"\documentclass{article}
\usepackage{tikz}
\usetikzlibrary{external}
\tikzexternalize
\begin{document}
Before.
\begin{tikzpicture}
\draw[blue, thick] (0,0) circle (1cm);
\end{tikzpicture}
After.
\end{document}
"#;

#[cfg(unix)]
#[test]
fn bundled_runner_reuses_pgf_figures_without_publishing_whole_document_state() {
    let _test_guard = super::lock();
    let _environment = super::compiler_cache::bundled_environment();
    for (mode, source) in [
        ("explicit", PGF_EXTERNAL_DOC.to_string()),
        (
            "implicit",
            PGF_EXTERNAL_DOC.replace(r"\tikzexternalize[mode=list and make]", r"\tikzexternalize"),
        ),
    ] {
        let fixture = PgfExternalFixture::new(mode);
        let main = fixture.root.join("main.tex");
        let out_dir = fixture.root.join("out");
        let child_input = fixture.root.join("figure-color.tex");
        fs::write(&child_input, "\\colorlet{childfill}{yellow}\n").unwrap();
        // The parent collects this body without executing it. Only figure0's
        // child job reads the color and adds its absolute path to the .dep.
        let source = source.replace(
            r"\draw[blue, thick] (0,0) circle (1cm);",
            &format!(
                "\\tikzexternalfiledependsonfile{{main-figure0}}{{{path}}}\n\
                 \\input{{{path}}}\n\
                 \\draw[blue, thick, fill=childfill] (0,0) circle (1cm);",
                path = child_input.display(),
            ),
        );
        fs::write(&main, &source).unwrap();
        let options = options(&main, &out_dir);
        let state = out_dir.join(".tekai-main.state.json");
        let figure0 = out_dir.join("main-figure0.pdf");
        let figure1 = out_dir.join("main-figure1.pdf");

        let first = build(&options).unwrap_or_else(|error| {
            let makefile = fs::read_to_string(out_dir.join("main.makefile"));
            for figure in 0..2 {
                if let Ok(log) =
                    fs::read_to_string(out_dir.join(format!("main-figure{figure}.log")))
                {
                    let tail = log.lines().rev().take(20).collect::<Vec<_>>();
                    eprintln!(
                        "figure {figure} log:\n{}",
                        tail.into_iter().rev().collect::<Vec<_>>().join("\n")
                    );
                }
            }
            panic!("initial bundled PGF build failed: {error:#}\nmakefile: {makefile:?}");
        });
        assert!(!first.skipped, "{mode}: {first:#?}");
        assert_eq!(first.tex_runs, 2, "{mode}: {first:#?}");
        assert_eq!(first.external_runs, 1, "{mode}: {first:#?}");
        assert!(!state.exists(), "{mode}: initial build published a state");
        assert!(out_dir.join("main.pdf").is_file());
        assert!(figure0.is_file());
        assert!(figure1.is_file());
        assert_eq!(fixture.child_invocations(), 2, "{mode}");
        let unchanged_figure = fs::read(&figure1).unwrap();
        let recorder = fs::read_to_string(out_dir.join("main.fls")).unwrap();
        assert!(recorder.contains("main-figure0.pdf"), "{recorder}");
        assert!(recorder.contains("main-figure1.pdf"), "{recorder}");
        assert!(!recorder.contains("figure-color.tex"), "{recorder}");
        let child_recorder = fs::read_to_string(out_dir.join("main-figure0.fls")).unwrap();
        assert!(
            child_recorder.contains("figure-color.tex"),
            "{child_recorder}"
        );
        let dependencies = fs::read_to_string(out_dir.join("main-figure0.dep")).unwrap();
        assert!(
            dependencies.contains(&child_input.display().to_string()),
            "{dependencies}"
        );

        // A cold build can decline caching because it writes .md5 inputs.
        // Exercise warm passes too, when those files are only read and the
        // generated makefile is consumed by make rather than the TeX engine.
        for rebuild in 1..=3 {
            let report = build(&options).expect("bundled PGF rebuild failed");
            assert!(!report.skipped, "{mode}, rebuild {rebuild}: {report:#?}");
            assert_eq!(report.tex_runs, 1, "{mode}: {report:#?}");
            assert_eq!(report.external_runs, 0, "{mode}: {report:#?}");
            assert!(
                !state.exists(),
                "{mode}, rebuild {rebuild} published a state"
            );
            assert_eq!(fixture.child_invocations(), 2, "{mode}");
            assert_eq!(fs::read(&figure1).unwrap(), unchanged_figure);
        }

        fs::remove_file(&figure0).unwrap();
        let regenerated = build(&options).expect("missing bundled PGF figure rebuild failed");
        assert!(!regenerated.skipped, "{mode}: {regenerated:#?}");
        assert_eq!(regenerated.tex_runs, 2, "{mode}: {regenerated:#?}");
        assert_eq!(regenerated.external_runs, 1, "{mode}: {regenerated:#?}");
        assert!(
            !state.exists(),
            "{mode}: missing-figure build published a state"
        );
        assert!(figure0.is_file());
        assert_eq!(fixture.child_invocations(), 3, "{mode}");
        assert_eq!(fs::read(&figure1).unwrap(), unchanged_figure);

        let previous_hash = fs::read(out_dir.join("main-figure0.md5")).unwrap();
        let previous_figure = fs::read(&figure0).unwrap();
        let edited_source = source.replace("blue, thick", "green, thick");
        fs::write(&main, &edited_source).unwrap();
        let edited = build(&options).expect("edited bundled PGF picture build failed");
        assert!(!edited.skipped, "{mode}: {edited:#?}");
        assert_eq!(edited.tex_runs, 2, "{mode}: {edited:#?}");
        assert_eq!(edited.external_runs, 1, "{mode}: {edited:#?}");
        assert!(!state.exists(), "{mode}: edited build published a state");
        assert_eq!(fixture.child_invocations(), 4, "{mode}");
        assert_ne!(
            fs::read(out_dir.join("main-figure0.md5")).unwrap(),
            previous_hash
        );
        assert_ne!(fs::read(&figure0).unwrap(), previous_figure);
        assert_eq!(fs::read(&figure1).unwrap(), unchanged_figure);

        // A figure-child dependency can change without changing either the
        // parent's consumed inputs or PGF's hash of the picture body. The .dep
        // still makes the figure stale, so whole-document reuse must not bypass
        // make's freshness check.
        let stable_hash = fs::read(out_dir.join("main-figure0.md5")).unwrap();
        let stable_recorder = fs::read(out_dir.join("main.fls")).unwrap();
        let previous_figure = fs::read(&figure0).unwrap();
        // Older make versions compare mtimes at whole-second resolution.
        // Wait only until this edit is distinguishable from the last figure.
        let earliest_edit =
            fs::metadata(&figure0).unwrap().modified().unwrap() + std::time::Duration::from_secs(1);
        if let Ok(delay) = earliest_edit.duration_since(std::time::SystemTime::now()) {
            assert!(
                delay <= std::time::Duration::from_secs(2),
                "figure timestamp is unexpectedly ahead of the test clock: {delay:?}"
            );
            std::thread::sleep(delay);
        }
        fs::write(&child_input, "\\colorlet{childfill}{red}\n").unwrap();
        let child_edited = build(&options).expect("edited PGF child dependency build failed");
        assert!(!child_edited.skipped, "{mode}: {child_edited:#?}");
        assert_eq!(child_edited.tex_runs, 2, "{mode}: {child_edited:#?}");
        assert_eq!(child_edited.external_runs, 1, "{mode}: {child_edited:#?}");
        assert!(
            !state.exists(),
            "{mode}: child-dependency build published a state"
        );
        assert_eq!(fixture.child_invocations(), 5, "{mode}");
        assert_eq!(fs::read(&main).unwrap(), edited_source.as_bytes());
        assert_eq!(
            fs::read(out_dir.join("main-figure0.md5")).unwrap(),
            stable_hash
        );
        assert_eq!(fs::read(out_dir.join("main.fls")).unwrap(), stable_recorder);
        assert_ne!(fs::read(&figure0).unwrap(), previous_figure);
        assert_eq!(fs::read(&figure1).unwrap(), unchanged_figure);
    }
}

#[cfg(unix)]
struct PgfExternalFixture {
    root: PathBuf,
    _environment: [PgfEnvironmentGuard; 4],
}

#[cfg(unix)]
impl PgfExternalFixture {
    fn new(mode: &str) -> Self {
        use std::os::unix::fs::PermissionsExt;

        let root = unique_temp_dir(&format!("tekai-bundled-pgf-{mode}"));
        let bin = root.join("bin");
        fs::create_dir_all(&bin).unwrap();
        let shim = bin.join("pdflatex");
        // Real make runs PGF's generated recipes. Both parent and figure jobs
        // use the bundled engine, so no installed TeX or kpsewhich is needed.
        // pdfTeX's -8bit preserves the literal tabs required by make syntax
        // instead of writing recipe prefixes as the printable string ^^I.
        // Child jobs run in out, so use make's source-directory TEXINPUTS
        // rather than bundled_environment's empty program-specific override.
        fs::write(
            &shim,
            "#!/bin/sh\nset -eu\n\
             if [ \"${1-}\" = __tekai-engine ]; then\n\
               shift\n\
             else\n\
               unset TEXINPUTS_pdflatex\n\
               printf '%s\\n' \"$*\" >> pgf-child.invocations\n\
               set -- -recorder \"$@\"\n\
             fi\n\
             exec \"${TEKAI_PGF_TEST_ENGINE:?missing bundled engine}\" __tekai-engine -8bit \"$@\"\n",
        )
        .unwrap();
        fs::set_permissions(&shim, fs::Permissions::from_mode(0o755)).unwrap();
        let mut paths = vec![bin];
        if let Some(previous) = std::env::var_os("PATH") {
            paths.extend(std::env::split_paths(&previous));
        }
        let path = std::env::join_paths(paths).unwrap();
        let environment = [
            PgfEnvironmentGuard::set("PATH", path),
            PgfEnvironmentGuard::set("TEKAI_PGF_TEST_ENGINE", env!("CARGO_BIN_EXE_tekai")),
            PgfEnvironmentGuard::set("TEKAI_AUX_CACHE", root.join("aux-cache")),
            PgfEnvironmentGuard::set(tekai::compiler::EMBEDDED_ENGINE_RUNNER_ENV, &shim),
        ];
        Self {
            root,
            _environment: environment,
        }
    }

    fn child_invocations(&self) -> usize {
        fs::read_to_string(self.root.join("out/pgf-child.invocations"))
            .expect("bundled PGF child invocation log disappeared")
            .lines()
            .count()
    }
}

#[cfg(unix)]
impl Drop for PgfExternalFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[cfg(unix)]
struct PgfEnvironmentGuard {
    name: &'static str,
    previous: Option<std::ffi::OsString>,
}

#[cfg(unix)]
impl PgfEnvironmentGuard {
    fn set(name: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let previous = std::env::var_os(name);
        // All compiler integration tests hold the process-wide test lock.
        unsafe {
            std::env::set_var(name, value);
        }
        Self { name, previous }
    }
}

#[cfg(unix)]
impl Drop for PgfEnvironmentGuard {
    fn drop(&mut self) {
        unsafe {
            if let Some(previous) = &self.previous {
                std::env::set_var(self.name, previous);
            } else {
                std::env::remove_var(self.name);
            }
        }
    }
}

#[test]
fn direct_runner_builds_pgf_externalized_figures_from_makefile() {
    let _test_guard = super::lock();
    if !command_available("pdflatex")
        || !command_available("make")
        || !tex_file_available("tikz.sty")
        || !tex_file_available("tikzlibraryexternal.code.tex")
    {
        eprintln!("skipping PGF externalization test; pdflatex, make, or TikZ is unavailable");
        return;
    }

    let root = unique_temp_dir("tekai-pgf-external-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, PGF_EXTERNAL_DOC).expect("failed to write PGF externalization document");

    let options = options(&main, &out_dir);
    let first = build(&options).expect("initial PGF externalization build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main-figure0.pdf").exists());
    assert!(out_dir.join("main-figure1.pdf").exists());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(fls.contains("main-figure0.pdf"), "{fls}");
    assert!(fls.contains("main-figure1.pdf"), "{fls}");

    let rebuilt = build(&options).expect("PGF externalization rebuild failed");
    // The figure cache remains reusable; generated-and-consumed files still
    // prevent claiming a fresh whole-document cache without read receipts.
    assert!(!rebuilt.skipped, "{rebuilt:#?}");
    assert_eq!(rebuilt.tex_runs, 1, "{rebuilt:#?}");
    assert_eq!(rebuilt.external_runs, 0, "{rebuilt:#?}");
    assert!(!out_dir.join(".tekai-main.state.json").exists());
    assert!(out_dir.join("main-figure0.pdf").is_file());
    assert!(out_dir.join("main-figure1.pdf").is_file());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_forces_pgf_list_and_make_without_shell_escape() {
    let _test_guard = super::lock();
    if !command_available("pdflatex")
        || !command_available("make")
        || !tex_file_available("tikz.sty")
        || !tex_file_available("tikzlibraryexternal.code.tex")
    {
        eprintln!("skipping PGF externalization test; pdflatex, make, or TikZ is unavailable");
        return;
    }

    let root = unique_temp_dir("tekai-pgf-default-external-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, PGF_DEFAULT_EXTERNAL_DOC)
        .expect("failed to write PGF externalization document");

    let options = options(&main, &out_dir);
    let first = build(&options).expect("default PGF externalization build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main-figure0.pdf").exists());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(fls.contains("main-figure0.pdf"), "{fls}");

    let rebuilt = build(&options).expect("default PGF externalization rebuild failed");
    assert!(!rebuilt.skipped, "{rebuilt:#?}");
    assert_eq!(rebuilt.tex_runs, 1, "{rebuilt:#?}");
    assert_eq!(rebuilt.external_runs, 0, "{rebuilt:#?}");
    assert!(!out_dir.join(".tekai-main.state.json").exists());
    assert!(out_dir.join("main-figure0.pdf").is_file());

    let _ = fs::remove_dir_all(root);
}

fn options(main: &Path, out_dir: &Path) -> BuildOptions {
    BuildOptions {
        main: main.to_path_buf(),
        job_name: None,
        engine: Engine::PdfLatex,
        runner: Runner::Direct,
        bib_mode: BibMode::Auto,
        out_dir: out_dir.to_path_buf(),
        fast: false,
        draft_prepass: DraftPrepass::Never,
        once: false,
        max_runs: 8,
        force: false,
        precompile_preamble: false,
        synctex: false,
        shell_escape: false,
        external_tools: true,
        quiet: true,
        print_command: false,
    }
}

fn command_available(program: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|directory| directory.join(program).is_file())
        })
        .unwrap_or(false)
}

fn tex_file_available(name: &str) -> bool {
    std::process::Command::new("kpsewhich")
        .arg(name)
        .status()
        .is_ok_and(|status| status.success())
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock before UNIX epoch")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

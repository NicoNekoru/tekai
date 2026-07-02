use std::fs;
use std::path::{Path, PathBuf};

use texpilot::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

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

#[test]
fn direct_runner_builds_pgf_externalized_figures_from_makefile() {
    if !command_available("pdflatex")
        || !command_available("make")
        || !tex_file_available("tikz.sty")
        || !tex_file_available("tikzlibraryexternal.code.tex")
    {
        eprintln!("skipping PGF externalization test; pdflatex, make, or TikZ is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-pgf-external-test");
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

    let cached = build(&options).expect("cached PGF externalization build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_forces_pgf_list_and_make_without_shell_escape() {
    if !command_available("pdflatex")
        || !command_available("make")
        || !tex_file_available("tikz.sty")
        || !tex_file_available("tikzlibraryexternal.code.tex")
    {
        eprintln!("skipping PGF externalization test; pdflatex, make, or TikZ is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-pgf-default-external-test");
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

    let cached = build(&options).expect("cached default PGF externalization build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");

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

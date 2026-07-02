use std::fs;
use std::path::{Path, PathBuf};

use texpilot::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const ASYMPTOTE_DOC: &str = r#"\documentclass{article}
\usepackage{asymptote}
\begin{document}
Figure:
\begin{asy}
size(1cm);
draw((0,0)--(1,1));
\end{asy}
\end{document}
"#;

#[test]
fn direct_runner_builds_and_caches_asymptote_figures() {
    if !command_available("pdflatex")
        || !command_available("asy")
        || !tex_file_available("asymptote.sty")
    {
        eprintln!("skipping Asymptote test; pdflatex, asy, or asymptote.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-asymptote-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, ASYMPTOTE_DOC).expect("failed to write Asymptote document");

    let first = build(&options(&main, &out_dir)).expect("initial Asymptote build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main-1.pdf").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    assert!(log.contains("main-1.pdf Graphic file"), "{log}");
    assert!(!log.contains("file `main-1.pdf' not found"), "{log}");

    let cached = build(&options(&main, &out_dir)).expect("cached Asymptote build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");

    fs::write(&main, ASYMPTOTE_DOC.replace("(1,1)", "(1,0)"))
        .expect("failed to update Asymptote source");
    let edited = build(&options(&main, &out_dir)).expect("edited Asymptote build failed");
    assert_eq!(edited.external_runs, 1, "{edited:#?}");
    assert_eq!(edited.tex_runs, 2, "{edited:#?}");

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

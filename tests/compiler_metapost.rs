use std::fs;
use std::path::{Path, PathBuf};

use texpilot::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const FEYNMP_DOC: &str = r#"\documentclass{article}
\usepackage{feynmp}
\DeclareGraphicsRule{*}{mps}{*}{}
\unitlength=1mm
\begin{document}
Before.
\begin{fmffile}{diagram}
\begin{fmfgraph}(30,20)
  \fmfleft{i}
  \fmfright{o}
  \fmf{plain}{i,o}
\end{fmfgraph}
\end{fmffile}
After.
\end{document}
"#;

#[test]
fn direct_runner_builds_and_caches_metapost_figures() {
    if !command_available("pdflatex")
        || !command_available("mpost")
        || !tex_file_available("feynmp.sty")
    {
        eprintln!("skipping MetaPost test; pdflatex, mpost, or feynmp.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-metapost-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, FEYNMP_DOC).expect("failed to write MetaPost document");

    let first = build(&options(&main, &out_dir)).expect("initial MetaPost build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("diagram.mp").exists());
    assert!(out_dir.join("diagram.1").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    assert!(log.contains("diagram.1 Graphic file"), "{log}");
    assert!(!log.contains("File diagram.1 not found"), "{log}");
    assert!(!log.contains("Process diagram.mp with MetaPost"), "{log}");

    let cached = build(&options(&main, &out_dir)).expect("cached MetaPost build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");

    fs::write(
        &main,
        FEYNMP_DOC.replace(r"\fmf{plain}{i,o}", r"\fmf{dashes}{i,o}"),
    )
    .expect("failed to update MetaPost source");
    let edited = build(&options(&main, &out_dir)).expect("edited MetaPost build failed");
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

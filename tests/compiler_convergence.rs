use std::fs;
use std::path::{Path, PathBuf};

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const SILENT_GENERATED_CHANGE_DOC: &str = r#"\documentclass{article}
\newwrite\stageout
\newcommand{\stagevalue}{0}
\IfFileExists{\jobname.stage}{\input{\jobname.stage}}{}
\begin{document}
Stage \stagevalue.
\ifnum\stagevalue=0
  \typeout{Rerun to get generated stage}
  \immediate\openout\stageout=\jobname.stage
  \immediate\write\stageout{\gdef\string\stagevalue{1}}
  \immediate\closeout\stageout
\else
  \immediate\openout\stageout=\jobname.stage
  \immediate\write\stageout{\gdef\string\stagevalue{2}}
  \immediate\closeout\stageout
\fi
\end{document}
"#;

const STALE_DRAFT_FILE_CHANGE_DOC: &str = r#"\documentclass{article}
\usepackage{graphicx}
\AtEndDocument{%
  \ifnum\pdfdraftmode=1
    \PackageWarningNoLine{rerunfilecheck}{File "main.out" has changed}%
  \fi
}
\begin{document}
See Figure~\ref{fig:box}.
\begin{figure}
\includegraphics[width=1cm]{example-image}
\caption{Box}\label{fig:box}
\end{figure}
\end{document}
"#;

const CHANGED_STANDARD_SIDECAR_DOC: &str = r#"\documentclass{article}
\usepackage{graphicx}
\newcommand{\backref}[1]{#1}
\newwrite\sidecar
\AtEndDocument{%
  \PackageWarningNoLine{rerunfilecheck}{File `main.out' has changed}%
}
\begin{document}
See Figure~\ref{fig:box}.
\begin{figure}
\includegraphics[width=1cm]{example-image}
\caption{Box}\label{fig:box}
\end{figure}
\makeatletter
\ifGin@draft
  \immediate\openout\sidecar=\jobname.out
  \immediate\write\sidecar{draft-sidecar}
  \immediate\closeout\sidecar
\else
  \immediate\openout\sidecar=\jobname.out
  \immediate\write\sidecar{final-sidecar}
  \immediate\closeout\sidecar
\fi
\makeatother
\end{document}
"#;

#[test]
fn direct_runner_tracks_generated_output_changes_even_when_log_is_silent() {
    if !command_available("pdflatex") {
        eprintln!("skipping convergence test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-convergence-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, SILENT_GENERATED_CHANGE_DOC).expect("failed to write test document");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert_eq!(first.tex_runs, 3, "{first:#?}");
    assert!(!first.skipped, "{first:#?}");

    let cached = build(&options(&main, &out_dir)).expect("cached build failed");
    assert!(cached.skipped, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_promotes_after_stale_file_change_warning_without_standard_file_changes() {
    if !command_available("pdflatex") || !tex_file_available("example-image.pdf") {
        eprintln!(
            "skipping stale file-change convergence test; pdflatex or example-image is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("tekai-stale-file-change-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, STALE_DRAFT_FILE_CHANGE_DOC).expect("failed to write test document");

    let first = build(&BuildOptions {
        draft_prepass: DraftPrepass::Auto,
        max_runs: 4,
        ..options(&main, &out_dir)
    })
    .expect("stale file-change warning should settle within the promoted PDF pass");

    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert_eq!(first.draft_tex_runs, 1, "{first:#?}");
    assert_eq!(first.final_tex_runs, 1, "{first:#?}");
    assert_eq!(first.pdf_tex_runs, 1, "{first:#?}");
    assert_eq!(first.passes.len(), 2, "{first:#?}");
    assert!(first.passes[0].draft, "{first:#?}");
    assert!(!first.passes[1].draft, "{first:#?}");
    assert!(first.passes[1].pdf_output, "{first:#?}");
    assert!(
        first.passes[0]
            .rerun_reasons
            .iter()
            .any(|reason| reason == "file-changed"),
        "{first:#?}"
    );
    assert!(out_dir.join("main.pdf").exists());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_reruns_when_promoted_pdf_changes_standard_sidecars() {
    if !command_available("pdflatex") || !tex_file_available("example-image.pdf") {
        eprintln!(
            "skipping changed standard-sidecar convergence test; pdflatex or example-image is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("tekai-changed-standard-sidecar-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, CHANGED_STANDARD_SIDECAR_DOC).expect("failed to write test document");

    let first = build(&BuildOptions {
        draft_prepass: DraftPrepass::Auto,
        max_runs: 4,
        ..options(&main, &out_dir)
    })
    .expect("changed standard sidecar should force one settling final pass");

    assert_eq!(first.tex_runs, 3, "{first:#?}");
    assert_eq!(first.draft_tex_runs, 1, "{first:#?}");
    assert_eq!(first.final_tex_runs, 2, "{first:#?}");
    assert_eq!(first.pdf_tex_runs, 1, "{first:#?}");
    assert_eq!(first.passes.len(), 3, "{first:#?}");
    assert!(first.passes[0].draft, "{first:#?}");
    assert!(!first.passes[1].draft, "{first:#?}");
    assert!(!first.passes[1].pdf_output, "{first:#?}");
    assert!(!first.passes[2].draft, "{first:#?}");
    assert!(first.passes[2].pdf_output, "{first:#?}");
    if tex_file_available("mylatexformat.ltx") {
        assert!(first.passes[1].preamble_format_used, "{first:#?}");
        assert!(first.passes[1].preamble_format_built, "{first:#?}");
        assert!(first.passes[2].preamble_format_used, "{first:#?}");
        assert!(!first.passes[2].preamble_format_built, "{first:#?}");
    }
    assert!(
        first.passes[1]
            .rerun_reasons
            .iter()
            .any(|reason| reason == "file-changed"),
        "{first:#?}"
    );
    assert_eq!(
        fs::read_to_string(out_dir.join("main.out")).expect("failed to read standard sidecar"),
        "final-sidecar\n"
    );
    assert!(out_dir.join("main.pdf").exists());

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
        .output()
        .map(|output| output.status.success() && !output.stdout.is_empty())
        .unwrap_or(false)
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

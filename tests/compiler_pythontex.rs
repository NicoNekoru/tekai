use std::fs;
use std::path::{Path, PathBuf};

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const PYTHONTEX_DOC: &str = r#"\documentclass{article}
\usepackage{pythontex}
\setpythontexworkingdir{..}
\begin{document}
\begin{pycode}
pytex.add_dependencies('data.txt')
def read_value():
    with open('data.txt') as handle:
        return handle.read().strip()
\end{pycode}
Value: \py{read_value()}
\end{document}
"#;

#[test]
fn direct_runner_builds_and_caches_pythontex_output() {
    if !command_available("pdflatex")
        || !command_available("pythontex")
        || !tex_file_available("pythontex.sty")
    {
        eprintln!("skipping PythonTeX test; pdflatex, pythontex, or pythontex.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("tekai-pythontex-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, PYTHONTEX_DOC).expect("failed to write PythonTeX document");
    fs::write(root.join("data.txt"), "alpha\n").expect("failed to write PythonTeX data");

    let first = build(&options(&main, &out_dir)).expect("initial PythonTeX build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("pythontex-files-main/main.pytxmcr").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    let compact_log = log.split_whitespace().collect::<String>();
    assert!(
        compact_log.contains("pythontex-files-main/main.pytxmcr"),
        "{log}"
    );
    assert!(!log.contains("Run PythonTeX to create it"), "{log}");
    assert!(!log.contains("Missing autoprint content"), "{log}");

    let cached = build(&options(&main, &out_dir)).expect("cached PythonTeX build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");

    fs::write(root.join("data.txt"), "beta\n").expect("failed to update PythonTeX data");
    let data_edit = build(&options(&main, &out_dir)).expect("data-edited PythonTeX build failed");
    assert_eq!(data_edit.external_runs, 1, "{data_edit:#?}");
    assert!(data_edit.tex_runs >= 1, "{data_edit:#?}");

    fs::write(
        &main,
        PYTHONTEX_DOC.replace(r"Value: \py{read_value()}", r"Value: \py{'source-edit'}"),
    )
    .expect("failed to update PythonTeX source");
    let edited = build(&options(&main, &out_dir)).expect("edited PythonTeX build failed");
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

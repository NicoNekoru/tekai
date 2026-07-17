use std::fs;
use std::path::{Path, PathBuf};

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const MAIN_TEX: &str = r#"\documentclass{article}
\begin{document}
\include{sections/ch1}
\end{document}
"#;

const MULTILINE_MAIN_TEX: &str = r#"\documentclass{article}
\begin{document}
\include
  {sections/ch1}
\end{document}
"#;

const CHAPTER_TEX: &str = r#"\section{One}
Hello.
"#;

#[test]
fn direct_runner_creates_output_subdirectories_for_included_aux_files() {
    if !command_available("pdflatex") {
        eprintln!("skipping include-directory test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-include-dir-test");
    let section_dir = root.join("sections");
    let out_dir = root.join("out");
    fs::create_dir_all(&section_dir).expect("failed to create section directory");
    let main = root.join("main.tex");
    let chapter = section_dir.join("ch1.tex");
    fs::write(&main, MAIN_TEX).expect("failed to write main document");
    fs::write(&chapter, CHAPTER_TEX).expect("failed to write chapter document");

    let first = build(&options(&main, &out_dir)).expect("include-directory build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(
        out_dir.join("sections").join("ch1.aux").exists(),
        "included aux file should be written under the mirrored output subdirectory"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached build failed");
    assert!(cached.skipped, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_creates_output_subdirectories_for_multiline_include() {
    if !command_available("pdflatex") {
        eprintln!("skipping multiline include-directory test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-multiline-include-dir-test");
    let section_dir = root.join("sections");
    let out_dir = root.join("out");
    fs::create_dir_all(&section_dir).expect("failed to create section directory");
    let main = root.join("main.tex");
    let chapter = section_dir.join("ch1.tex");
    fs::write(&main, MULTILINE_MAIN_TEX).expect("failed to write main document");
    fs::write(&chapter, CHAPTER_TEX).expect("failed to write chapter document");

    let first = build(&options(&main, &out_dir)).expect("multiline include build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(
        out_dir.join("sections").join("ch1.aux").exists(),
        "multiline included aux file should be written under the mirrored output subdirectory"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached build failed");
    assert!(cached.skipped, "{cached:#?}");

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

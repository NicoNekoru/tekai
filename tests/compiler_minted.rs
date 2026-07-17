use std::fs;
use std::path::{Path, PathBuf};

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const MINTED_DOC: &str = r#"\documentclass{article}
\usepackage{minted}
\begin{document}
Before.
\begin{minted}{python}
print("highlighted")
\end{minted}
After.
\end{document}
"#;

#[test]
fn direct_runner_reruns_after_cold_minted_cache_generation() {
    if !command_available("pdflatex")
        || !command_available("latexminted")
        || !tex_file_available("minted.sty")
        || !minted_supports_v3_cache_index()
    {
        eprintln!(
            "skipping minted v3 cache test; pdflatex, latexminted, or minted v3 is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("tekai-minted-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, MINTED_DOC).expect("failed to write minted document");

    let options = options(&main, &out_dir);
    let first = build(&options).expect("initial minted build failed");
    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert_eq!(first.external_runs, 0, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("_minted").is_dir());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(fls.contains(".index.minted"), "{fls}");
    assert!(fls.contains(".highlight.minted"), "{fls}");

    let cached = build(&options).expect("cached minted build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");

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

fn minted_supports_v3_cache_index() -> bool {
    let Ok(output) = std::process::Command::new("kpsewhich")
        .arg("minted.sty")
        .output()
    else {
        return false;
    };
    if !output.status.success() {
        return false;
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    fs::read_to_string(path)
        .is_ok_and(|source| source.contains("placeholder/.is if=minted@placeholder"))
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

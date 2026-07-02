use std::fs;
use std::path::{Path, PathBuf};

use texpilot::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const BIB_DOC: &str = r#"\documentclass{article}
\begin{document}
A citation \cite{knuth1984}.
\bibliographystyle{plain}
\bibliography{refs}
\end{document}
"#;

const REFS_BIB: &str = r#"@book{knuth1984,
  author = {Donald E. Knuth},
  title = {The TeXbook},
  year = {1984},
  publisher = {Addison-Wesley}
}
"#;

#[test]
fn direct_runner_does_not_cache_unsettled_max_run_builds() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping max-runs test; pdflatex or bibtex is not available");
        return;
    }

    let root = unique_temp_dir("texpilot-max-runs-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, BIB_DOC).expect("failed to write test document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let too_few_runs = build(&options(&main, &out_dir, 1)).expect_err("build should not settle");
    assert!(
        too_few_runs.to_string().contains("did not settle"),
        "{too_few_runs:#}"
    );
    assert!(
        !out_dir.join(".texpilot-main.state.toml").exists(),
        "unsettled build should not write a successful cache state"
    );

    let settled = build(&options(&main, &out_dir, 8)).expect("settled build failed");
    assert!(!settled.skipped, "{settled:#?}");
    assert!(out_dir.join("main.pdf").exists());

    let _ = fs::remove_dir_all(root);
}

fn options(main: &Path, out_dir: &Path, max_runs: usize) -> BuildOptions {
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
        max_runs,
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

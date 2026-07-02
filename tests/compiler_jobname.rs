use std::fs;
use std::path::{Path, PathBuf};

use texpilot::compiler::{
    BibMode, BuildOptions, DraftPrepass, Engine, Runner, build, build_dependency_paths,
};

const JOBNAME_DOC: &str = r#"\documentclass{article}
\begin{document}
Job name: \jobname.
See~\cite{knuth}.
\bibliographystyle{plain}
\bibliography{refs}
\end{document}
"#;

const REFS: &str = r#"@book{knuth,
  author = {Donald E. Knuth},
  title = {The TeXbook},
  year = {1984}
}
"#;

#[test]
fn direct_runner_honors_custom_job_name_for_outputs_and_aux_tools() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping custom job-name test; pdflatex or bibtex is not available");
        return;
    }

    let root = unique_temp_dir("texpilot-jobname-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, JOBNAME_DOC).expect("failed to write test document");
    fs::write(&refs, REFS).expect("failed to write bibliography");

    let options = BuildOptions {
        job_name: Some("paper-final.v1".to_string()),
        ..options(&main, &out_dir)
    };
    let first = build(&options).expect("custom job-name build failed");
    assert!(!first.skipped, "{first:#?}");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(out_dir.join("paper-final.v1.pdf").exists());
    assert!(out_dir.join("paper-final.v1.aux").exists());
    assert!(out_dir.join("paper-final.v1.bbl").exists());
    assert!(out_dir.join(".texpilot-paper-final.v1.state.toml").exists());
    assert!(!out_dir.join("main.pdf").exists());

    let dependencies =
        build_dependency_paths(&options).expect("failed to read custom job-name dependencies");
    let refs = refs
        .canonicalize()
        .expect("failed to canonicalize bibliography");
    assert!(
        dependencies.iter().any(|path| path == &refs),
        "{dependencies:#?}"
    );

    let cached = build(&options).expect("cached custom job-name build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");
    assert_eq!(cached.bibliography_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn default_job_name_preserves_dotted_root_stem() {
    if !command_available("pdflatex") {
        eprintln!("skipping dotted root-stem job-name test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("texpilot-dotted-jobname-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("paper.v1.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        r#"\documentclass{article}
\begin{document}
Dotted default job name: \jobname.
\end{document}
"#,
    )
    .expect("failed to write dotted-stem test document");

    let options = options(&main, &out_dir);
    let first = build(&options).expect("dotted root-stem build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("paper.v1.pdf").exists());
    assert!(out_dir.join("paper.v1.aux").exists());
    assert!(out_dir.join(".texpilot-paper.v1.state.toml").exists());
    assert!(!out_dir.join("paper.pdf").exists());
    assert!(!out_dir.join(".texpilot-paper.state.toml").exists());

    let cached = build(&options).expect("cached dotted root-stem build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn custom_job_name_must_be_a_single_filename_component() {
    let root = unique_temp_dir("texpilot-jobname-invalid-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, "\\documentclass{article}\n").expect("failed to write test document");

    let error = build(&BuildOptions {
        job_name: Some("nested/paper".to_string()),
        ..options(&main, &out_dir)
    })
    .expect_err("path-like job names should be rejected");
    assert!(
        error
            .to_string()
            .contains("--job-name must be a single filename component"),
        "{error:#}"
    );

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

use std::fs;
use std::path::{Path, PathBuf};

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const AUX_TOOLS_DOC: &str = r#"\documentclass{article}
\usepackage{makeidx}
\makeindex
\begin{document}
A citation \cite{knuth1984}.
Alpha\index{alpha}
\bibliographystyle{plain}
\bibliography{refs}
\printindex
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
fn direct_runner_builds_bibliography_and_index_in_one_aux_round() {
    if !command_available("pdflatex")
        || !command_available("bibtex")
        || !command_available("makeindex")
    {
        eprintln!("skipping combined aux tool test; pdflatex, bibtex, or makeindex is unavailable");
        return;
    }

    let root = unique_temp_dir("tekai-combined-aux-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, AUX_TOOLS_DOC).expect("failed to write test document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial combined aux build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert_eq!(first.index_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let bbl = fs::read_to_string(out_dir.join("main.bbl")).expect("failed to read bibliography");
    let ind = fs::read_to_string(out_dir.join("main.ind")).expect("failed to read index");
    assert!(bbl.contains("The TeXbook"), "{bbl}");
    assert!(ind.contains("alpha"), "{ind}");

    let cached = build(&options(&main, &out_dir)).expect("cached combined aux build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.bibliography_runs, 0, "{cached:#?}");
    assert_eq!(cached.index_runs, 0, "{cached:#?}");

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

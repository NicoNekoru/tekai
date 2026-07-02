use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use texpilot::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const BIBER_DOC: &str = r#"\documentclass{article}
\usepackage[backend=biber]{biblatex}
\addbibresource{refs.bib}
\begin{document}
A citation \cite{knuth1984}.
\printbibliography
\end{document}
"#;

const BIBLATEX_BIBTEX_DOC: &str = r#"\documentclass{article}
\usepackage[backend=bibtex]{biblatex}
\addbibresource{refs.bib}
\begin{document}
A citation \cite{knuth1984}.
\printbibliography
\end{document}
"#;

const NO_BIBLIOGRAPHY_DOC: &str = r#"\documentclass{article}
\begin{document}
No bibliography now.
\end{document}
"#;

const MIXED_BIBER_AND_BIBTEX_DOC: &str = r#"\documentclass{article}
\usepackage[backend=biber]{biblatex}
\addbibresource{refs.bib}
\begin{document}
Biber citation \cite{knuth1984}.
\newwrite\hiddenaux
\immediate\openout\hiddenaux=hidden.aux
\immediate\write\hiddenaux{\string\relax}
\immediate\write\hiddenaux{\string\citation{knuth1984}}
\immediate\write\hiddenaux{\string\bibstyle{plain}}
\immediate\write\hiddenaux{\string\bibdata{refs}}
\immediate\closeout\hiddenaux
Hidden BibTeX output:
\IfFileExists{hidden.bbl}{\input{hidden.bbl}}{Missing hidden bibliography.}
\printbibliography
\end{document}
"#;

const REFS_BIB: &str = r#"@book{knuth1984,
  author = {Donald E. Knuth},
  title = {The TeXbook},
  year = {1984},
  publisher = {Addison-Wesley}
}
"#;

static BIBER_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn direct_runner_builds_and_caches_biber_output() {
    if !command_available("pdflatex")
        || !command_available("biber")
        || !tex_file_available("biblatex.sty")
    {
        eprintln!("skipping Biber build test; pdflatex, biber, or biblatex.sty is unavailable");
        return;
    }

    let _guard = BIBER_TEST_LOCK.lock().expect("Biber test lock poisoned");
    let root = unique_temp_dir("texpilot-biber-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, BIBER_DOC).expect("failed to write biblatex document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial Biber build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let bbl_path = out_dir.join("main.bbl");
    let bbl = fs::read_to_string(&bbl_path).expect("failed to read generated biber output");
    assert!(
        bbl.contains("TheTeXbook") || bbl.contains("The TeXbook"),
        "{bbl}"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached Biber build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.bibliography_runs, 0, "{cached:#?}");

    fs::write(
        &main,
        format!("{BIBER_DOC}\n% source-only edit that leaves Biber inputs unchanged\n"),
    )
    .expect("failed to update biblatex document");
    let text_edit = build(&options(&main, &out_dir)).expect("text edit build failed");
    assert_eq!(text_edit.bibliography_runs, 0, "{text_edit:#?}");

    fs::write(&refs, REFS_BIB.replace("The TeXbook", "The Biber Book"))
        .expect("failed to update bibliography");
    let bib_edit = build(&options(&main, &out_dir)).expect("bibliography edit build failed");
    assert_eq!(bib_edit.bibliography_runs, 1, "{bib_edit:#?}");
    assert_eq!(bib_edit.tex_runs, 1, "{bib_edit:#?}");
    let bbl = fs::read_to_string(bbl_path).expect("failed to read regenerated biber output");
    assert!(
        bbl.contains("TheBiberBook") || bbl.contains("The Biber Book"),
        "{bbl}"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_auto_runs_current_bibtex_aux_and_biber_control_files() {
    if !command_available("pdflatex")
        || !command_available("bibtex")
        || !command_available("biber")
        || !tex_file_available("biblatex.sty")
    {
        eprintln!(
            "skipping mixed bibliography test; pdflatex, bibtex, biber, or biblatex.sty is unavailable"
        );
        return;
    }

    let _guard = BIBER_TEST_LOCK.lock().expect("Biber test lock poisoned");
    let root = unique_temp_dir("texpilot-mixed-bib-backends-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, MIXED_BIBER_AND_BIBTEX_DOC).expect("failed to write mixed document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("mixed bibliography build failed");
    assert_eq!(first.bibliography_runs, 2, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    let main_bbl =
        fs::read_to_string(out_dir.join("main.bbl")).expect("failed to read Biber output");
    assert!(
        main_bbl.contains("TheTeXbook") || main_bbl.contains("The TeXbook"),
        "{main_bbl}"
    );
    let hidden_bbl =
        fs::read_to_string(out_dir.join("hidden.bbl")).expect("failed to read BibTeX output");
    assert!(hidden_bbl.contains(r"\bibitem{knuth1984}"), "{hidden_bbl}");
    assert!(hidden_bbl.contains("The TeXbook"), "{hidden_bbl}");

    let cached = build(&options(&main, &out_dir)).expect("cached mixed bibliography build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.bibliography_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_skips_tex_when_biber_preflight_output_is_unchanged() {
    if !command_available("pdflatex")
        || !command_available("biber")
        || !tex_file_available("biblatex.sty")
    {
        eprintln!(
            "skipping unchanged Biber output test; pdflatex, biber, or biblatex.sty is unavailable"
        );
        return;
    }

    let _guard = BIBER_TEST_LOCK.lock().expect("Biber test lock poisoned");
    let root = unique_temp_dir("texpilot-biber-unchanged-output-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, BIBER_DOC).expect("failed to write biblatex document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial Biber build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let bbl_path = out_dir.join("main.bbl");
    let original_bbl = fs::read_to_string(&bbl_path).expect("failed to read biber output");

    fs::write(
        &refs,
        format!(
            "{REFS_BIB}\n@book{{unused2026,\n  author = {{Ignored Author}},\n  title = {{Unused Entry}},\n  year = {{2026}}\n}}\n"
        ),
    )
    .expect("failed to update bibliography");
    let unused_bib_edit =
        build(&options(&main, &out_dir)).expect("unused bibliography edit build failed");
    assert!(unused_bib_edit.skipped, "{unused_bib_edit:#?}");
    assert_eq!(unused_bib_edit.bibliography_runs, 0, "{unused_bib_edit:#?}");
    assert_eq!(unused_bib_edit.tex_runs, 0, "{unused_bib_edit:#?}");
    assert!(!unused_bib_edit.aux_preflight_used, "{unused_bib_edit:#?}");
    let updated_bbl = fs::read_to_string(&bbl_path).expect("failed to read biber output");
    assert_eq!(updated_bbl, original_bbl);

    let cached = build(&options(&main, &out_dir)).expect("cached Biber build failed");
    assert!(cached.skipped, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_auto_prefers_current_bibtex_aux_over_stale_bcf() {
    if !command_available("pdflatex")
        || !command_available("bibtex")
        || !command_available("biber")
        || !tex_file_available("biblatex.sty")
    {
        eprintln!(
            "skipping BibLaTeX backend switch test; pdflatex, bibtex, biber, or biblatex.sty is unavailable"
        );
        return;
    }

    let _guard = BIBER_TEST_LOCK.lock().expect("Biber test lock poisoned");
    let root = unique_temp_dir("texpilot-biblatex-backend-switch-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, BIBER_DOC).expect("failed to write Biber document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial Biber build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(
        out_dir.join("main.bcf").exists(),
        "Biber run should leave a .bcf"
    );

    fs::write(&main, BIBLATEX_BIBTEX_DOC).expect("failed to switch BibLaTeX backend");
    let switched = build(&options(&main, &out_dir)).expect("BibTeX backend build failed");
    assert_eq!(switched.bibliography_runs, 1, "{switched:#?}");
    assert!(switched.tex_runs >= 1, "{switched:#?}");
    let bbl = fs::read_to_string(out_dir.join("main.bbl"))
        .expect("failed to read switched bibliography output");
    assert!(
        !bbl.contains("biber as required"),
        "stale Biber output was reused: {bbl}"
    );
    assert!(bbl.contains("The TeXbook"), "{bbl}");

    let cached = build(&options(&main, &out_dir)).expect("cached BibTeX backend build failed");
    assert!(cached.skipped, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_ignores_stale_bcf_when_current_tex_run_has_no_biber_control_file() {
    if !command_available("pdflatex")
        || !command_available("biber")
        || !tex_file_available("biblatex.sty")
    {
        eprintln!(
            "skipping stale Biber control test; pdflatex, biber, or biblatex.sty is unavailable"
        );
        return;
    }

    let _guard = BIBER_TEST_LOCK.lock().expect("Biber test lock poisoned");
    let root = unique_temp_dir("texpilot-stale-bcf-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, BIBER_DOC).expect("failed to write Biber document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial Biber build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(
        out_dir.join("main.bcf").exists(),
        "Biber run should leave a stale .bcf candidate"
    );

    fs::write(&main, NO_BIBLIOGRAPHY_DOC).expect("failed to remove bibliography from document");
    fs::remove_file(out_dir.join("main.aux")).expect("failed to remove stale BibLaTeX aux file");
    let removed = build(&options(&main, &out_dir)).expect("no-bibliography build failed");
    assert_eq!(removed.bibliography_runs, 0, "{removed:#?}");
    assert_eq!(removed.index_runs, 0, "{removed:#?}");

    let cached = build(&options(&main, &out_dir)).expect("cached no-bibliography build failed");
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

fn tex_file_available(name: &str) -> bool {
    Command::new("kpsewhich")
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

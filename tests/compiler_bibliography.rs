use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use texpilot::compiler::{
    BibMode, BuildOptions, DraftPrepass, Engine, Runner, build, build_dependency_paths,
};

const MAIN_TEX: &str = r#"\documentclass{article}
\begin{document}
\include{chapter}
\end{document}
"#;

const SUBDIR_MAIN_TEX: &str = r#"\documentclass{article}
\begin{document}
\include{sections/chapter}
\end{document}
"#;

const MULTI_MAIN_TEX: &str = r#"\documentclass{article}
\begin{document}
\include{chapter_a}
\include{chapter_b}
\end{document}
"#;

const INCLUDEONLY_CHAPTER_A_TEX: &str = r#"\documentclass{article}
\includeonly{chapter_a}
\begin{document}
\include{chapter_a}
\include{chapter_b}
\end{document}
"#;

const CHAPTER_TEX: &str = r#"A citation \cite{knuth1984}.
\bibliographystyle{plain}
\bibliography{refs}
"#;

const CHAPTER_A_TEX: &str = r#"First citation \cite{knuth1984}.
\bibliographystyle{plain}
\bibliography{refs}
"#;

const CHAPTER_B_TEX: &str = r#"Second citation \cite{lamport1994}.
\bibliographystyle{plain}
\bibliography{refs}
"#;

const REFS_BIB: &str = r#"@book{knuth1984,
  author = {Donald E. Knuth},
  title = {The TeXbook},
  year = {1984},
  publisher = {Addison-Wesley}
}
"#;

const MULTI_REFS_BIB: &str = r#"@book{knuth1984,
  author = {Donald E. Knuth},
  title = {The TeXbook},
  year = {1984},
  publisher = {Addison-Wesley}
}

@book{lamport1994,
  author = {Leslie Lamport},
  title = {LaTeX: A Document Preparation System},
  year = {1994},
  publisher = {Addison-Wesley}
}
"#;

const GRAPHIC_MAIN_TEX: &str = r#"\documentclass{article}
\usepackage[draft]{graphicx}
\begin{document}
\includegraphics{pixel.png}
A citation \cite{knuth1984}.
\bibliographystyle{plain}
\bibliography{refs}
\end{document}
"#;

const TEXT_ONLY_BIB_MAIN_TEX: &str = r#"\documentclass{article}
\begin{document}
A citation \cite{knuth1984}.
\bibliographystyle{plain}
\bibliography{refs}
\end{document}
"#;

const SIMPLE_MAIN_TEX: &str = r#"\documentclass{article}
\begin{document}
Hello.
\end{document}
"#;

const BACKREF_GRAPHIC_MAIN_TEX: &str = r#"\documentclass{article}
\usepackage{graphicx}
\usepackage[numbers]{natbib}
\usepackage[pagebackref=true]{hyperref}
\renewcommand*{\backref}[1]{}
\begin{document}
\includegraphics[width=1cm]{example-image}
A citation \citep{knuth1984}.
\bibliographystyle{plainnat}
\bibliography{refs}
\end{document}
"#;

const EXTERNAL_BIB_TEX: &str = r#"\documentclass{article}
\begin{document}
A citation \cite{external2026}.
\bibliographystyle{plain}
\bibliography{externalrefs}
\end{document}
"#;

const HIDDEN_AUX_TEX: &str = r#"\documentclass{article}
\begin{document}
\newwrite\hiddenaux
\immediate\openout\hiddenaux=hidden.aux
\immediate\write\hiddenaux{\string\relax}
\immediate\write\hiddenaux{\string\citation{knuth1984}}
\immediate\write\hiddenaux{\string\bibstyle{plain}}
\immediate\write\hiddenaux{\string\bibdata{refs}}
\immediate\closeout\hiddenaux
Hidden bibliography:
\IfFileExists{hidden.bbl}{\input{hidden.bbl}}{Missing hidden bibliography.}
\end{document}
"#;

const EXTERNAL_REFS_BIB: &str = r#"@book{external2026,
  author = {Search Path},
  title = {External Bibliography},
  year = {2026},
  publisher = {Configured Path Press}
}
"#;

const BIBLATEX_BIBTEX8_TEX: &str = r#"\documentclass{article}
\usepackage[backend=bibtex8]{biblatex}
\addbibresource{refs.bib}
\begin{document}
A citation \cite{knuth1984}.
\printbibliography
\end{document}
"#;

static BIBINPUTS_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn direct_runner_builds_bibliographies_from_included_aux_files() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping included-aux BibTeX test; pdflatex or bibtex is not available");
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-bib-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let chapter = root.join("chapter.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, MAIN_TEX).expect("failed to write main document");
    fs::write(&chapter, CHAPTER_TEX).expect("failed to write chapter document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial bibliography build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let bbl_path = out_dir.join("chapter.bbl");
    let bbl = fs::read_to_string(&bbl_path).expect("failed to read included bibliography");
    assert!(bbl.contains("The TeXbook"), "{bbl}");

    let cached = build(&options(&main, &out_dir)).expect("cached bibliography build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.bibliography_runs, 0, "{cached:#?}");

    fs::write(
        &chapter,
        format!("{CHAPTER_TEX}\n% source-only edit that leaves citations unchanged\n"),
    )
    .expect("failed to update chapter");
    let text_edit = build(&options(&main, &out_dir)).expect("text edit build failed");
    assert_eq!(text_edit.bibliography_runs, 0, "{text_edit:#?}");

    fs::write(
        &refs,
        REFS_BIB.replace("The TeXbook", "The TeXbook Updated"),
    )
    .expect("failed to update bibliography");
    let bib_edit = build(&options(&main, &out_dir)).expect("bibliography edit build failed");
    assert_eq!(bib_edit.bibliography_runs, 1, "{bib_edit:#?}");
    assert_eq!(bib_edit.tex_runs, 1, "{bib_edit:#?}");
    let bbl = fs::read_to_string(&bbl_path).expect("failed to read regenerated bibliography");
    assert!(bbl.contains("The TeXbook Updated"), "{bbl}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_builds_multiple_included_bibliographies() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping multi-aux BibTeX test; pdflatex or bibtex is not available");
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-multi-bib-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let chapter_a = root.join("chapter_a.tex");
    let chapter_b = root.join("chapter_b.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, MULTI_MAIN_TEX).expect("failed to write main document");
    fs::write(&chapter_a, CHAPTER_A_TEX).expect("failed to write first chapter");
    fs::write(&chapter_b, CHAPTER_B_TEX).expect("failed to write second chapter");
    fs::write(&refs, MULTI_REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial multi-bibliography build failed");
    assert_eq!(first.bibliography_runs, 2, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let bbl_a = fs::read_to_string(out_dir.join("chapter_a.bbl"))
        .expect("failed to read first bibliography");
    let bbl_b = fs::read_to_string(out_dir.join("chapter_b.bbl"))
        .expect("failed to read second bibliography");
    assert!(bbl_a.contains("The TeXbook"), "{bbl_a}");
    assert!(bbl_b.contains("Document Preparation System"), "{bbl_b}");

    let cached = build(&options(&main, &out_dir)).expect("cached multi-bibliography build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.bibliography_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_ignores_stale_bibtex_aux_excluded_by_includeonly() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!(
            "skipping includeonly stale-aux BibTeX test; pdflatex or bibtex is not available"
        );
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-includeonly-stale-bib-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let chapter_a = root.join("chapter_a.tex");
    let chapter_b = root.join("chapter_b.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, MULTI_MAIN_TEX).expect("failed to write main document");
    fs::write(&chapter_a, CHAPTER_A_TEX).expect("failed to write first chapter");
    fs::write(&chapter_b, CHAPTER_B_TEX).expect("failed to write second chapter");
    fs::write(&refs, MULTI_REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial multi-bibliography build failed");
    assert_eq!(first.bibliography_runs, 2, "{first:#?}");
    assert!(out_dir.join("chapter_b.bbl").exists());

    fs::write(
        out_dir.join("chapter_b.aux"),
        "\\relax\n\\citation{missing2026}\n\\bibstyle{plain}\n\\bibdata{missingrefs}\n",
    )
    .expect("failed to poison stale excluded aux file");
    fs::write(&main, INCLUDEONLY_CHAPTER_A_TEX).expect("failed to write includeonly document");

    let includeonly = build(&options(&main, &out_dir)).expect("includeonly build failed");
    assert_eq!(includeonly.bibliography_runs, 0, "{includeonly:#?}");
    assert!(includeonly.tex_runs > 0, "{includeonly:#?}");
    assert!(out_dir.join("main.pdf").exists());

    let dependency_paths =
        build_dependency_paths(&options(&main, &out_dir)).expect("failed to read dependencies");
    assert!(
        !dependency_paths
            .iter()
            .any(|path| path.file_name().and_then(|name| name.to_str()) == Some("missingrefs.bib")),
        "{dependency_paths:#?}"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_discovers_recorded_bibtex_aux_not_linked_from_root_aux() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping recorded hidden-aux BibTeX test; pdflatex or bibtex is unavailable");
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-recorded-hidden-bib-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, HIDDEN_AUX_TEX).expect("failed to write hidden-aux document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial hidden-aux build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let root_aux = fs::read_to_string(out_dir.join("main.aux")).expect("failed to read root aux");
    assert!(!root_aux.contains(r"\bibdata"), "{root_aux}");
    let bbl_path = out_dir.join("hidden.bbl");
    let bbl = fs::read_to_string(&bbl_path).expect("failed to read hidden bibliography");
    assert!(bbl.contains("The TeXbook"), "{bbl}");

    let dependency_paths =
        build_dependency_paths(&options(&main, &out_dir)).expect("failed to read dependencies");
    assert!(
        dependency_paths
            .iter()
            .any(|path| path == &refs.canonicalize().unwrap()),
        "{dependency_paths:#?}"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached hidden-aux build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.bibliography_runs, 0, "{cached:#?}");

    fs::write(
        &refs,
        REFS_BIB.replace("The TeXbook", "The TeXbook Hidden Aux Updated"),
    )
    .expect("failed to update hidden-aux bibliography");
    let bib_edit = build(&options(&main, &out_dir)).expect("hidden-aux bibliography edit failed");
    assert_eq!(bib_edit.bibliography_runs, 1, "{bib_edit:#?}");
    assert_eq!(bib_edit.tex_runs, 1, "{bib_edit:#?}");
    assert!(bib_edit.aux_preflight_used, "{bib_edit:#?}");
    let bbl = fs::read_to_string(&bbl_path).expect("failed to read regenerated bibliography");
    assert!(bbl.contains("The TeXbook Hidden Aux Updated"), "{bbl}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_preserves_existing_bibinputs_and_tracks_external_bibs() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping BIBINPUTS BibTeX test; pdflatex or bibtex is not available");
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-bibinputs-test");
    let bib_root = unique_temp_dir("texpilot-bibinputs-external");
    let alternate_bib_root = unique_temp_dir("texpilot-bibinputs-alternate");
    fs::create_dir_all(&root).expect("failed to create test directory");
    fs::create_dir_all(&bib_root).expect("failed to create external bibliography directory");
    fs::create_dir_all(&alternate_bib_root)
        .expect("failed to create alternate bibliography directory");
    let main = root.join("main.tex");
    let refs = bib_root.join("externalrefs.bib");
    let alternate_refs = alternate_bib_root.join("externalrefs.bib");
    let out_dir = root.join("out");
    fs::write(&main, EXTERNAL_BIB_TEX).expect("failed to write main document");
    fs::write(&refs, EXTERNAL_REFS_BIB).expect("failed to write external bibliography");
    fs::write(
        &alternate_refs,
        EXTERNAL_REFS_BIB.replace("External Bibliography", "Alternate Bibliography"),
    )
    .expect("failed to write alternate bibliography");

    let mut env = EnvVarGuard::set(
        "BIBINPUTS",
        OsString::from(format!("{}//:", bib_root.display())),
    );

    let first = build(&options(&main, &out_dir)).expect("initial BIBINPUTS build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let dependency_paths =
        build_dependency_paths(&options(&main, &out_dir)).expect("failed to read dependencies");
    assert!(
        dependency_paths
            .iter()
            .any(|path| path == &refs.canonicalize().unwrap()),
        "{dependency_paths:#?}"
    );
    let bbl_path = out_dir.join("main.bbl");
    let bbl = fs::read_to_string(&bbl_path).expect("failed to read external bibliography output");
    assert!(bbl.contains("External Bibliography"), "{bbl}");

    let cached = build(&options(&main, &out_dir)).expect("cached BIBINPUTS build failed");
    assert!(cached.skipped, "{cached:#?}");

    fs::write(
        &refs,
        EXTERNAL_REFS_BIB.replace("External Bibliography", "External Bibliography Updated"),
    )
    .expect("failed to update external bibliography");
    let bib_edit = build(&options(&main, &out_dir)).expect("external bibliography edit failed");
    assert_eq!(bib_edit.bibliography_runs, 1, "{bib_edit:#?}");
    assert_eq!(bib_edit.tex_runs, 1, "{bib_edit:#?}");
    let bbl = fs::read_to_string(&bbl_path).expect("failed to read regenerated bibliography");
    assert!(bbl.contains("External Bibliography Updated"), "{bbl}");

    env.replace(OsString::from(format!(
        "{}//:",
        alternate_bib_root.display()
    )));
    let search_path_edit =
        build(&options(&main, &out_dir)).expect("BIBINPUTS search path change build failed");
    assert!(!search_path_edit.skipped, "{search_path_edit:#?}");
    assert_eq!(
        search_path_edit.bibliography_runs, 1,
        "{search_path_edit:#?}"
    );
    assert!(search_path_edit.tex_runs >= 1, "{search_path_edit:#?}");
    let dependency_paths =
        build_dependency_paths(&options(&main, &out_dir)).expect("failed to read dependencies");
    assert!(
        dependency_paths
            .iter()
            .any(|path| path == &alternate_refs.canonicalize().unwrap()),
        "{dependency_paths:#?}"
    );
    let bbl = fs::read_to_string(&bbl_path).expect("failed to read search-path bibliography");
    assert!(bbl.contains("Alternate Bibliography"), "{bbl}");

    let _ = fs::remove_dir_all(root);
    let _ = fs::remove_dir_all(bib_root);
    let _ = fs::remove_dir_all(alternate_bib_root);
}

#[test]
fn direct_runner_honors_biblatex_logreq_bibtex8_command() {
    if !command_available("pdflatex")
        || !command_available("bibtex8")
        || !tex_file_available("biblatex.sty")
    {
        eprintln!(
            "skipping biblatex bibtex8 test; pdflatex, bibtex8, or biblatex.sty is unavailable"
        );
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-bibtex8-logreq-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, BIBLATEX_BIBTEX8_TEX).expect("failed to write biblatex document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial bibtex8 build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let blg = fs::read_to_string(out_dir.join("main.blg")).expect("failed to read BibTeX log");
    assert!(blg.contains("8-bit Big BibTeX"), "{blg}");

    let dependency_paths =
        build_dependency_paths(&options(&main, &out_dir)).expect("failed to read dependencies");
    assert!(
        dependency_paths
            .iter()
            .any(|path| path.file_name().and_then(|name| name.to_str()) == Some("main-blx.bib")),
        "{dependency_paths:#?}"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached bibtex8 build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.bibliography_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_skips_tex_when_bibtex_preflight_output_is_unchanged() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping unchanged BibTeX output test; pdflatex or bibtex is not available");
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-bib-unchanged-output-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let chapter = root.join("chapter.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, MAIN_TEX).expect("failed to write main document");
    fs::write(&chapter, CHAPTER_TEX).expect("failed to write chapter document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial bibliography build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let bbl_path = out_dir.join("chapter.bbl");
    let original_bbl = fs::read_to_string(&bbl_path).expect("failed to read bibliography");

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
    let updated_bbl = fs::read_to_string(&bbl_path).expect("failed to read bibliography");
    assert_eq!(updated_bbl, original_bbl);

    let cached = build(&options(&main, &out_dir)).expect("cached bibliography build failed");
    assert!(cached.skipped, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_does_not_add_tex_pass_when_post_tex_bibtex_output_is_unchanged() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!(
            "skipping post-TeX unchanged BibTeX output test; pdflatex or bibtex is not available"
        );
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-bib-post-tex-unchanged-output-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let chapter = root.join("chapter.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, MAIN_TEX).expect("failed to write main document");
    fs::write(&chapter, CHAPTER_TEX).expect("failed to write chapter document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial bibliography build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let bbl_path = out_dir.join("chapter.bbl");
    let original_bbl = fs::read_to_string(&bbl_path).expect("failed to read bibliography");

    fs::write(
        &chapter,
        format!("{CHAPTER_TEX}\nAdditional visible text.\n"),
    )
    .expect("failed to update chapter");
    fs::write(
        &refs,
        format!(
            "{REFS_BIB}\n@book{{unused2026,\n  author = {{Ignored Author}},\n  title = {{Unused Entry}},\n  year = {{2026}}\n}}\n"
        ),
    )
    .expect("failed to update bibliography");
    let edit =
        build(&options(&main, &out_dir)).expect("source plus unused bibliography edit failed");
    assert_eq!(edit.bibliography_runs, 0, "{edit:#?}");
    assert_eq!(edit.tex_runs, 1, "{edit:#?}");
    assert!(!edit.aux_preflight_used, "{edit:#?}");
    let updated_bbl = fs::read_to_string(&bbl_path).expect("failed to read bibliography");
    assert_eq!(updated_bbl, original_bbl);

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_builds_bibliographies_from_included_aux_files_in_subdirectories() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping included-subdir BibTeX test; pdflatex or bibtex is not available");
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-subdir-bib-test");
    let sections = root.join("sections");
    fs::create_dir_all(&sections).expect("failed to create section directory");
    let main = root.join("main.tex");
    let chapter = sections.join("chapter.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, SUBDIR_MAIN_TEX).expect("failed to write main document");
    fs::write(&chapter, CHAPTER_TEX).expect("failed to write chapter document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options(&main, &out_dir)).expect("initial subdir bibliography build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let bbl_path = out_dir.join("sections").join("chapter.bbl");
    let bbl = fs::read_to_string(&bbl_path).expect("failed to read included bibliography");
    assert!(bbl.contains("The TeXbook"), "{bbl}");

    let cached = build(&options(&main, &out_dir)).expect("cached subdir bibliography build failed");
    assert!(cached.skipped, "{cached:#?}");

    fs::write(
        &refs,
        REFS_BIB.replace("The TeXbook", "The TeXbook In Sections"),
    )
    .expect("failed to update bibliography");
    let bib_edit = build(&options(&main, &out_dir)).expect("subdir bibliography edit failed");
    assert_eq!(bib_edit.bibliography_runs, 1, "{bib_edit:#?}");
    assert_eq!(bib_edit.tex_runs, 1, "{bib_edit:#?}");
    let bbl = fs::read_to_string(bbl_path).expect("failed to read regenerated bibliography");
    assert!(bbl.contains("The TeXbook In Sections"), "{bbl}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn auto_draft_prepass_does_not_add_passes_to_bibliography_edit_rebuilds() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping auto-prepass BibTeX edit test; pdflatex or bibtex is not available");
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-auto-prepass-bib-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, GRAPHIC_MAIN_TEX).expect("failed to write main document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options_auto(&main, &out_dir)).expect("initial auto build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(first.draft_tex_runs > 0, "{first:#?}");
    assert_eq!(first.pdf_tex_runs, 1, "{first:#?}");
    assert!(first.final_tex_runs >= first.pdf_tex_runs, "{first:#?}");
    assert_eq!(
        first.tex_runs,
        first.draft_tex_runs + first.final_tex_runs,
        "{first:#?}"
    );
    assert_eq!(first.passes.len(), first.tex_runs, "{first:#?}");
    assert!(first.passes.iter().any(|pass| pass.draft), "{first:#?}");
    assert_eq!(
        first.passes.iter().filter(|pass| pass.pdf_output).count(),
        first.pdf_tex_runs,
        "{first:#?}"
    );
    assert!(
        first
            .passes
            .iter()
            .any(|pass| !pass.rerun_reasons.is_empty()),
        "{first:#?}"
    );
    assert!(out_dir.join("main.pdf").exists());

    fs::write(
        &refs,
        REFS_BIB.replace("The TeXbook", "The TeXbook Auto Updated"),
    )
    .expect("failed to update bibliography");
    let bib_edit =
        build(&options_auto(&main, &out_dir)).expect("auto bibliography edit build failed");
    assert_eq!(bib_edit.bibliography_runs, 1, "{bib_edit:#?}");
    assert_eq!(bib_edit.tex_runs, 1, "{bib_edit:#?}");
    let bbl = fs::read_to_string(out_dir.join("main.bbl")).expect("failed to read bibliography");
    assert!(bbl.contains("The TeXbook Auto Updated"), "{bbl}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn auto_no_pdf_prepass_converges_text_only_multipass_documents() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping auto no-pdf BibTeX test; pdflatex or bibtex is not available");
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-auto-no-pdf-bib-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, TEXT_ONLY_BIB_MAIN_TEX).expect("failed to write main document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&options_auto(&main, &out_dir)).expect("initial auto build failed");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert_eq!(first.draft_tex_runs, 0, "{first:#?}");
    assert_eq!(first.pdf_tex_runs, 1, "{first:#?}");
    assert!(first.final_tex_runs > first.pdf_tex_runs, "{first:#?}");
    assert_eq!(first.passes.len(), first.tex_runs, "{first:#?}");
    assert!(
        first
            .passes
            .iter()
            .take(first.passes.len() - 1)
            .all(|pass| !pass.pdf_output),
        "{first:#?}"
    );
    assert!(
        first.passes.last().is_some_and(|pass| pass.pdf_output),
        "{first:#?}"
    );
    assert!(
        first.passes.iter().all(|pass| {
            pass.pdf_output
                || pass.aux_outputs_changed
                || pass.generated_outputs_changed
                || pass.generated_inputs_unread
                || !pass.rerun_reasons.is_empty()
        }),
        "settled no-PDF passes should be promoted to PDF output: {first:#?}"
    );
    assert!(out_dir.join("main.pdf").exists());

    let cached = build(&options_auto(&main, &out_dir)).expect("cached auto build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn auto_no_pdf_prepass_does_not_expand_simple_one_pass_documents() {
    if !command_available("pdflatex") {
        eprintln!("skipping simple auto no-pdf test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("texpilot-auto-no-pdf-simple-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, SIMPLE_MAIN_TEX).expect("failed to write main document");

    let first = build(&options_auto(&main, &out_dir)).expect("simple auto build failed");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert_eq!(first.draft_tex_runs, 0, "{first:#?}");
    assert_eq!(first.final_tex_runs, 1, "{first:#?}");
    assert_eq!(first.pdf_tex_runs, 1, "{first:#?}");
    assert!(
        first.passes.first().is_some_and(|pass| pass.pdf_output),
        "{first:#?}"
    );
    assert!(out_dir.join("main.pdf").exists());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn auto_draft_prepass_switches_to_final_after_backref_outputs_stabilize() {
    if !command_available("pdflatex")
        || !command_available("bibtex")
        || !tex_file_available("natbib.sty")
        || !tex_file_available("example-image.pdf")
    {
        eprintln!(
            "skipping auto-prepass backref test; pdflatex, bibtex, natbib, or example-image is unavailable"
        );
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-auto-prepass-backref-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, BACKREF_GRAPHIC_MAIN_TEX).expect("failed to write main document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&BuildOptions {
        max_runs: 4,
        ..options_auto(&main, &out_dir)
    })
    .expect("auto backref build should settle without exhausting draft reruns");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert!(first.draft_tex_runs > 0, "{first:#?}");
    assert!(first.final_tex_runs > 0, "{first:#?}");
    assert_eq!(first.pdf_tex_runs, 1, "{first:#?}");
    assert!(first.pdf_tex_runs <= first.final_tex_runs, "{first:#?}");
    assert_eq!(
        first.tex_runs,
        first.draft_tex_runs + first.final_tex_runs,
        "{first:#?}"
    );
    assert_eq!(first.passes.len(), first.tex_runs, "{first:#?}");
    assert!(first.passes.iter().any(|pass| pass.draft), "{first:#?}");
    assert!(first.passes.iter().any(|pass| !pass.draft), "{first:#?}");
    assert_eq!(
        first.passes.iter().filter(|pass| pass.pdf_output).count(),
        first.pdf_tex_runs,
        "{first:#?}"
    );
    assert!(
        !first.passes[0]
            .rerun_reasons
            .iter()
            .any(|reason| reason == "citations-changed"),
        "{first:#?}"
    );
    assert!(out_dir.join("main.pdf").exists());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn forced_build_reuses_bibtex_session_freshness_within_single_build() {
    if !command_available("pdflatex")
        || !command_available("bibtex")
        || !tex_file_available("natbib.sty")
        || !tex_file_available("example-image.pdf")
    {
        eprintln!(
            "skipping forced BibTeX session-cache test; pdflatex, bibtex, natbib, or example-image is unavailable"
        );
        return;
    }

    let _env_guard = bibinputs_test_guard();
    let root = unique_temp_dir("texpilot-force-bibtex-session-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, BACKREF_GRAPHIC_MAIN_TEX).expect("failed to write main document");
    fs::write(&refs, REFS_BIB).expect("failed to write bibliography");

    let first = build(&BuildOptions {
        force: true,
        max_runs: 4,
        ..options_auto(&main, &out_dir)
    })
    .expect("forced auto backref build should settle");
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(first.tex_runs > 1, "{first:#?}");
    assert_eq!(
        first
            .passes
            .iter()
            .filter(|pass| pass.bibliography_runs > 0)
            .count(),
        0,
        "{first:#?}"
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

fn options_auto(main: &Path, out_dir: &Path) -> BuildOptions {
    BuildOptions {
        draft_prepass: DraftPrepass::Auto,
        ..options(main, out_dir)
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

fn bibinputs_test_guard() -> std::sync::MutexGuard<'static, ()> {
    BIBINPUTS_TEST_LOCK
        .lock()
        .expect("BIBINPUTS test lock poisoned")
}

struct EnvVarGuard {
    name: &'static str,
    previous: Option<OsString>,
}

impl EnvVarGuard {
    fn set(name: &'static str, value: OsString) -> Self {
        let previous = std::env::var_os(name);
        unsafe {
            std::env::set_var(name, value);
        }
        Self { name, previous }
    }

    fn replace(&mut self, value: OsString) {
        unsafe {
            std::env::set_var(self.name, value);
        }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        unsafe {
            if let Some(previous) = &self.previous {
                std::env::set_var(self.name, previous);
            } else {
                std::env::remove_var(self.name);
            }
        }
    }
}

use std::fs;
use std::path::{Path, PathBuf};

use tekai::compiler::{
    BibMode, BuildOptions, DraftPrepass, Engine, Runner, build, build_dependency_paths,
};

const INDEX_DOC: &str = r#"\documentclass{article}
\usepackage{makeidx}
\makeindex
\begin{document}
Alpha\index{alpha}
Beta\index{beta}
\printindex
\end{document}
"#;

const NO_INDEX_DOC: &str = r#"\documentclass{article}
\begin{document}
No index now.
\end{document}
"#;

const IMAKEIDX_STYLED_DOC: &str = r#"\documentclass{article}
\usepackage{imakeidx}
\makeindex[name=people,options={-s custom.ist -l}]
\begin{document}
Sea lion\index[people]{sea lion}
Seal\index[people]{seal}
Sea-lion\index[people]{sea-lion}
\printindex[people]
\end{document}
"#;

const IMAKEIDX_XINDY_DOC: &str = r#"\documentclass{article}
\usepackage[xindy]{imakeidx}
\makeindex[name=people,options=-L english]
\begin{document}
Alpha\index[people]{alpha}
\printindex[people]
\end{document}
"#;

const SPLITIDX_DOC: &str = r#"\documentclass{article}
\usepackage[makeindex]{splitidx}
\newindex[General]{idx}
\newindex[People]{per}
\newindex[Concepts]{cpt}
\begin{document}
Ada\sindex[per]{Ada}
Lambda\sindex[cpt]{lambda}
General\sindex{general}
\printindices
\end{document}
"#;

const CUSTOM_INDEX_STYLE: &str = r#"delim_0 " STYLE "
"#;

const UPDATED_CUSTOM_INDEX_STYLE: &str = r#"delim_0 " RESTYLE "
"#;

#[test]
fn direct_runner_builds_and_caches_makeindex_output() {
    if !command_available("pdflatex") || !command_available("makeindex") {
        eprintln!("skipping MakeIndex build test; pdflatex or makeindex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-index-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, INDEX_DOC).expect("failed to write test document");

    let first = build(&options(&main, &out_dir)).expect("initial index build failed");
    assert_eq!(first.index_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let ind_path = out_dir.join("main.ind");
    let ind = fs::read_to_string(&ind_path).expect("failed to read generated index");
    assert!(ind.contains("alpha"), "{ind}");
    assert!(ind.contains("beta"), "{ind}");

    let cached = build(&options(&main, &out_dir)).expect("cached index build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.index_runs, 0, "{cached:#?}");

    fs::write(
        &main,
        format!("{INDEX_DOC}\n% source-only edit that leaves index entries unchanged\n"),
    )
    .expect("failed to update test document");
    let text_edit = build(&options(&main, &out_dir)).expect("text edit build failed");
    assert_eq!(text_edit.index_runs, 0, "{text_edit:#?}");

    fs::write(&main, INDEX_DOC.replace(r"\index{alpha}", r"\index{gamma}"))
        .expect("failed to update index entry");
    let index_edit = build(&options(&main, &out_dir)).expect("index edit build failed");
    assert_eq!(index_edit.index_runs, 1, "{index_edit:#?}");
    let ind = fs::read_to_string(ind_path).expect("failed to read regenerated index");
    assert!(ind.contains("gamma"), "{ind}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_ignores_stale_index_files_not_recorded_by_latest_tex_run() {
    if !command_available("pdflatex") || !command_available("makeindex") {
        eprintln!("skipping stale index artifact test; pdflatex or makeindex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-stale-index-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, INDEX_DOC).expect("failed to write indexed document");

    let first = build(&options(&main, &out_dir)).expect("initial index build failed");
    assert_eq!(first.index_runs, 1, "{first:#?}");
    let stale_idx = out_dir.join("main.idx");
    assert!(stale_idx.exists(), "initial index file should exist");

    fs::write(&main, NO_INDEX_DOC).expect("failed to remove index from document");
    let removed = build(&options(&main, &out_dir)).expect("no-index build failed");
    assert_eq!(removed.index_runs, 0, "{removed:#?}");

    fs::write(&stale_idx, "\\indexentry{stale}{1}\n").expect("failed to mutate stale index file");
    fs::write(
        &main,
        format!("{NO_INDEX_DOC}\n% text edit after stale index mutation\n"),
    )
    .expect("failed to edit no-index document");
    let text_edit = build(&options(&main, &out_dir)).expect("text edit build failed");
    assert_eq!(text_edit.index_runs, 0, "{text_edit:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_uses_xindy_requested_by_imakeidx_log_command() {
    if !command_available("pdflatex")
        || !command_available("texindy")
        || !command_available("xindy")
        || !tex_file_available("imakeidx.sty")
    {
        eprintln!(
            "skipping imakeidx xindy test; pdflatex, texindy/xindy, or imakeidx.sty is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("tekai-imakeidx-xindy-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, IMAKEIDX_XINDY_DOC).expect("failed to write imakeidx xindy document");

    let first = build(&options(&main, &out_dir)).expect("initial imakeidx xindy build failed");
    assert_eq!(first.index_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let ind_path = out_dir.join("people.ind");
    let ind = fs::read_to_string(&ind_path).expect("failed to read imakeidx xindy output");
    assert!(ind.contains(r"\lettergroup{A}"), "{ind}");
    assert!(ind.contains("alpha"), "{ind}");

    let cached = build(&options(&main, &out_dir)).expect("cached imakeidx xindy build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.index_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_uses_makeindex_style_requested_by_imakeidx_log_command() {
    if !command_available("pdflatex")
        || !command_available("makeindex")
        || !tex_file_available("imakeidx.sty")
    {
        eprintln!(
            "skipping imakeidx style test; pdflatex, makeindex, or imakeidx.sty is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("tekai-imakeidx-style-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let style = root.join("custom.ist");
    let out_dir = root.join("out");
    fs::write(&main, IMAKEIDX_STYLED_DOC).expect("failed to write imakeidx document");
    fs::write(&style, CUSTOM_INDEX_STYLE).expect("failed to write custom index style");

    let first = build(&options(&main, &out_dir)).expect("initial imakeidx build failed");
    assert_eq!(first.index_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let ind_path = out_dir.join("people.ind");
    let ind = fs::read_to_string(&ind_path).expect("failed to read imakeidx output");
    assert!(ind.contains("sea-lion STYLE 1"), "{ind}");
    assert!(ind.contains("seal STYLE 1"), "{ind}");
    assert!(ind.contains("sea lion STYLE 1"), "{ind}");
    assert_sorted_by_letter_order(&ind, "STYLE");

    let dependency_paths =
        build_dependency_paths(&options(&main, &out_dir)).expect("failed to read dependencies");
    assert!(
        dependency_paths
            .iter()
            .any(|path| path == &style.canonicalize().unwrap()),
        "{dependency_paths:#?}"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached imakeidx build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.index_runs, 0, "{cached:#?}");

    fs::write(&style, UPDATED_CUSTOM_INDEX_STYLE).expect("failed to update custom index style");
    let style_edit = build(&options(&main, &out_dir)).expect("style edit build failed");
    assert_eq!(style_edit.index_runs, 1, "{style_edit:#?}");
    assert_eq!(style_edit.tex_runs, 1, "{style_edit:#?}");
    assert!(style_edit.aux_preflight_used, "{style_edit:#?}");
    let ind = fs::read_to_string(ind_path).expect("failed to read regenerated imakeidx output");
    assert!(ind.contains("sea-lion RESTYLE 1"), "{ind}");
    assert_sorted_by_letter_order(&ind, "RESTYLE");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_splits_splitidx_raw_index_before_makeindex() {
    if !command_available("pdflatex")
        || !command_available("splitindex")
        || !command_available("makeindex")
        || !tex_file_available("splitidx.sty")
    {
        eprintln!(
            "skipping splitidx test; pdflatex, splitindex, makeindex, or splitidx.sty is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("tekai-splitidx-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, SPLITIDX_DOC).expect("failed to write splitidx document");

    let first = build(&options(&main, &out_dir)).expect("initial splitidx build failed");
    assert_eq!(first.index_runs, 4, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main-per.idx").exists());
    assert!(out_dir.join("main-cpt.idx").exists());
    assert!(out_dir.join("main-idx.idx").exists());
    let people = fs::read_to_string(out_dir.join("main-per.ind"))
        .expect("failed to read people split index");
    let concepts = fs::read_to_string(out_dir.join("main-cpt.ind"))
        .expect("failed to read concepts split index");
    let general = fs::read_to_string(out_dir.join("main-idx.ind"))
        .expect("failed to read general split index");
    assert!(people.contains("Ada"), "{people}");
    assert!(concepts.contains("lambda"), "{concepts}");
    assert!(general.contains("general"), "{general}");
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    assert!(!log.contains("No file main-per.ind"), "{log}");
    assert!(!log.contains("No file main-cpt.ind"), "{log}");
    assert!(!log.contains("No file main-idx.ind"), "{log}");

    let cached = build(&options(&main, &out_dir)).expect("cached splitidx build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.index_runs, 0, "{cached:#?}");

    fs::write(
        &main,
        format!("{SPLITIDX_DOC}\n% source-only edit that leaves split indices unchanged\n"),
    )
    .expect("failed to update splitidx document");
    let text_edit = build(&options(&main, &out_dir)).expect("splitidx text edit build failed");
    assert_eq!(text_edit.index_runs, 0, "{text_edit:#?}");

    fs::write(&main, SPLITIDX_DOC.replace("lambda", "omega"))
        .expect("failed to update splitidx entry");
    let index_edit = build(&options(&main, &out_dir)).expect("splitidx index edit build failed");
    assert_eq!(index_edit.index_runs, 2, "{index_edit:#?}");
    let concepts = fs::read_to_string(out_dir.join("main-cpt.ind"))
        .expect("failed to read updated split index");
    assert!(concepts.contains("omega"), "{concepts}");

    let _ = fs::remove_dir_all(root);
}

fn assert_sorted_by_letter_order(ind: &str, delimiter: &str) {
    let hyphenated = ind
        .find(&format!("sea-lion {delimiter} 1"))
        .unwrap_or_else(|| panic!("missing hyphenated entry in {ind}"));
    let compact = ind
        .find(&format!("seal {delimiter} 1"))
        .unwrap_or_else(|| panic!("missing compact entry in {ind}"));
    let spaced = ind
        .find(&format!("sea lion {delimiter} 1"))
        .unwrap_or_else(|| panic!("missing spaced entry in {ind}"));

    assert!(
        hyphenated < compact && compact < spaced,
        "index output did not use letter-order sorting: {ind}"
    );
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

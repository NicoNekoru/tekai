use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use texpilot::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const GLOSSARY_DOC: &str = r#"\documentclass{article}
\usepackage{glossaries}
\makeglossaries
\newglossaryentry{sample}{name={Sample term},description={A generated glossary entry}}
\begin{document}
Use \gls{sample}.
\printglossaries
\end{document}
"#;

const XINDY_GLOSSARY_DOC: &str = r#"\documentclass{article}
\usepackage[xindy]{glossaries}
\makeglossaries
\newglossaryentry{sample}{name={Sample term},description={A generated glossary entry}}
\begin{document}
Use \gls{sample}.
\printglossaries
\end{document}
"#;

#[test]
fn direct_runner_builds_and_caches_glossary_makeindex_output() {
    if !command_available("pdflatex")
        || !command_available("makeindex")
        || !tex_file_available("glossaries.sty")
    {
        eprintln!(
            "skipping glossary build test; pdflatex, makeindex, or glossaries.sty is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("texpilot-glossary-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, GLOSSARY_DOC).expect("failed to write glossary document");

    let first = build(&options(&main, &out_dir)).expect("initial glossary build failed");
    assert_eq!(first.index_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let gls_path = out_dir.join("main.gls");
    let gls = fs::read_to_string(&gls_path).expect("failed to read generated glossary");
    assert!(gls.contains(r"\glossentry{sample}"), "{gls}");

    let cached = build(&options(&main, &out_dir)).expect("cached glossary build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.index_runs, 0, "{cached:#?}");

    fs::write(
        &main,
        format!("{GLOSSARY_DOC}\n% source-only edit that leaves glossary entries unchanged\n"),
    )
    .expect("failed to update glossary document");
    let text_edit = build(&options(&main, &out_dir)).expect("text edit build failed");
    assert_eq!(text_edit.index_runs, 0, "{text_edit:#?}");

    fs::write(&main, GLOSSARY_DOC.replace("sample", "changed"))
        .expect("failed to update glossary entry");
    let glossary_edit = build(&options(&main, &out_dir)).expect("glossary edit build failed");
    assert_eq!(glossary_edit.index_runs, 1, "{glossary_edit:#?}");
    let gls = fs::read_to_string(gls_path).expect("failed to read regenerated glossary");
    assert!(gls.contains(r"\glossentry{changed}"), "{gls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_builds_and_caches_xindy_glossaries_via_makeglossaries() {
    if !command_available("pdflatex")
        || !command_available("makeglossaries")
        || !command_available("xindy")
        || !tex_file_available("glossaries.sty")
    {
        eprintln!(
            "skipping Xindy glossary build test; pdflatex, makeglossaries, xindy, or glossaries.sty is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("texpilot-xindy-glossary-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, XINDY_GLOSSARY_DOC).expect("failed to write Xindy glossary document");

    let first = build(&options(&main, &out_dir)).expect("initial Xindy glossary build failed");
    assert_eq!(first.index_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let gls_path = out_dir.join("main.gls");
    let gls = fs::read_to_string(&gls_path).expect("failed to read generated Xindy glossary");
    assert!(gls.contains(r"\glossentry{sample}"), "{gls}");
    let glg = String::from_utf8_lossy(
        &fs::read(out_dir.join("main.glg")).expect("failed to read Xindy glossary transcript"),
    )
    .into_owned();
    assert!(glg.to_ascii_lowercase().contains("xindy"), "{glg}");

    let cached = build(&options(&main, &out_dir)).expect("cached Xindy glossary build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.index_runs, 0, "{cached:#?}");

    fs::write(
        &main,
        format!(
            "{XINDY_GLOSSARY_DOC}\n% source-only edit that leaves glossary entries unchanged\n"
        ),
    )
    .expect("failed to update Xindy glossary document");
    let text_edit = build(&options(&main, &out_dir)).expect("Xindy text edit build failed");
    assert_eq!(text_edit.index_runs, 0, "{text_edit:#?}");

    fs::write(&main, XINDY_GLOSSARY_DOC.replace("sample", "changed"))
        .expect("failed to update Xindy glossary entry");
    let glossary_edit = build(&options(&main, &out_dir)).expect("Xindy glossary edit build failed");
    assert_eq!(glossary_edit.index_runs, 1, "{glossary_edit:#?}");
    let gls = fs::read_to_string(gls_path).expect("failed to read regenerated Xindy glossary");
    assert!(gls.contains(r"\glossentry{changed}"), "{gls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_ignores_stale_xdy_when_current_glossary_uses_makeindex() {
    if !command_available("pdflatex")
        || !command_available("makeindex")
        || !command_available("makeglossaries")
        || !command_available("xindy")
        || !tex_file_available("glossaries.sty")
    {
        eprintln!(
            "skipping stale Xindy artifact test; pdflatex, makeindex, makeglossaries, xindy, or glossaries.sty is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("texpilot-stale-xdy-glossary-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, XINDY_GLOSSARY_DOC).expect("failed to write Xindy glossary document");

    let first = build(&options(&main, &out_dir)).expect("initial Xindy glossary build failed");
    assert_eq!(first.index_runs, 1, "{first:#?}");
    assert!(
        out_dir.join("main.xdy").exists(),
        "Xindy style should exist"
    );

    fs::write(&main, GLOSSARY_DOC).expect("failed to switch glossary backend");
    let switched = build(&options(&main, &out_dir)).expect("default glossary build failed");
    assert_eq!(switched.index_runs, 1, "{switched:#?}");
    let glg = String::from_utf8_lossy(
        &fs::read(out_dir.join("main.glg")).expect("failed to read glossary transcript"),
    )
    .into_owned();
    assert!(
        !glg.to_ascii_lowercase().contains("xindy"),
        "stale Xindy transcript was reused: {glg}"
    );
    assert!(glg.to_ascii_lowercase().contains("makeindex"), "{glg}");

    let cached = build(&options(&main, &out_dir)).expect("cached default glossary build failed");
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

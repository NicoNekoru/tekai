use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

static AUX_CACHE_TEST_LOCK: Mutex<()> = Mutex::new(());

const AUX_DOC: &str = r#"\documentclass{article}
\begin{document}
\tableofcontents
\section{Intro}
\label{sec:intro}
See Section~\ref{sec:intro}.
\end{document}
"#;

#[test]
fn generated_output_dir_artifacts_do_not_invalidate_noop_cache() {
    let _test_guard = super::lock();
    let _environment = bundled_environment();

    let root = unique_temp_dir("tekai-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, AUX_DOC).expect("failed to write test document");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    // A legacy state cannot establish a current hit. A valid JSON state can
    // coexist with it, and malformed JSON must rebuild rather than fall back.
    let state_path = out_dir.join(".tekai-main.state.json");
    let legacy_path = out_dir.join(".tekai-main.state.toml");
    let state: serde_json::Value = serde_json::from_slice(&fs::read(&state_path).unwrap()).unwrap();
    let mut legacy_fields = toml::map::Map::new();
    legacy_fields.insert(
        "version".into(),
        toml::Value::Integer(state["version"].as_i64().unwrap()),
    );
    for field in ["mode_key", "pdf_path"] {
        legacy_fields.insert(
            field.into(),
            toml::Value::String(state[field].as_str().unwrap().into()),
        );
    }
    legacy_fields.insert("inputs".into(), toml::Value::Array(vec![]));
    let legacy = toml::to_string(&legacy_fields).unwrap();
    fs::write(&legacy_path, &legacy).unwrap();
    fs::remove_file(&state_path).unwrap();
    let migrated = build(&options(&main, &out_dir)).expect("legacy state rebuild failed");
    assert!(!migrated.skipped, "{migrated:#?}");
    assert!(migrated.tex_runs > 0, "{migrated:#?}");
    assert!(build(&options(&main, &out_dir)).unwrap().skipped);
    fs::write(&state_path, "{").unwrap();
    let recovered = build(&options(&main, &out_dir)).expect("malformed JSON rebuild failed");
    assert!(!recovered.skipped, "{recovered:#?}");
    assert!(recovered.tex_runs > 0, "{recovered:#?}");
    assert_eq!(fs::read_to_string(&legacy_path).unwrap(), legacy);

    fs::write(out_dir.join("main.aux"), "% mutated generated aux\n")
        .expect("failed to mutate generated aux");
    fs::write(out_dir.join("main.toc"), "% mutated generated toc\n")
        .expect("failed to mutate generated toc");

    let cached = build(&options(&main, &out_dir)).expect("cached build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn scheduler_policy_changes_do_not_invalidate_settled_final_cache() {
    let _test_guard = super::lock();
    let _environment = bundled_environment();

    let root = unique_temp_dir("tekai-scheduler-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, AUX_DOC).expect("failed to write test document");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    let more_runs = build(&BuildOptions {
        max_runs: 16,
        ..options(&main, &out_dir)
    })
    .expect("max-runs cache check failed");
    assert!(more_runs.skipped, "{more_runs:#?}");
    assert_eq!(more_runs.tex_runs, 0, "{more_runs:#?}");

    let auto_prepass = build(&BuildOptions {
        draft_prepass: DraftPrepass::Auto,
        ..options(&main, &out_dir)
    })
    .expect("draft-prepass cache check failed");
    assert!(auto_prepass.skipped, "{auto_prepass:#?}");
    assert_eq!(auto_prepass.tex_runs, 0, "{auto_prepass:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fresh_output_dir_restores_settled_pdf_artifact_from_shared_cache() {
    let _test_guard = super::lock();
    let _environment = bundled_environment();

    let _env_lock = AUX_CACHE_TEST_LOCK
        .lock()
        .expect("aux cache test lock poisoned");
    let root = unique_temp_dir("tekai-shared-artifact-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let _cache_guard =
        EnvVarGuard::set("TEKAI_AUX_CACHE", root.join("cache").display().to_string());
    let main = root.join("main.tex");
    let first_out = root.join("first-out");
    let fresh_out = root.join("fresh-out");
    let force_out = root.join("force-out");
    fs::create_dir_all(&first_out).unwrap();
    fs::create_dir_all(&fresh_out).unwrap();
    fs::create_dir_all(&force_out).unwrap();
    fs::write(&main, AUX_DOC).expect("failed to write test document");

    let first = build(&options(&main, &first_out)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(first.tex_runs > 0, "{first:#?}");
    let first_pdf = fs::read(first_out.join("main.pdf")).expect("failed to read first pdf");

    let restored = build(&options(&main, &fresh_out)).expect("fresh output cache restore failed");
    assert!(restored.skipped, "{restored:#?}");
    assert_eq!(restored.tex_runs, 0, "{restored:#?}");
    assert_eq!(
        fs::read(fresh_out.join("main.pdf")).expect("failed to read restored pdf"),
        first_pdf
    );
    assert!(fresh_out.join(".tekai-main.state.json").exists());

    let local_cached = build(&options(&main, &fresh_out)).expect("local cache check failed");
    assert!(local_cached.skipped, "{local_cached:#?}");
    assert_eq!(local_cached.tex_runs, 0, "{local_cached:#?}");

    let forced = build(&BuildOptions {
        force: true,
        ..options(&main, &force_out)
    })
    .expect("forced build failed");
    assert!(!forced.skipped, "{forced:#?}");
    assert!(forced.tex_runs > 0, "{forced:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn source_edits_rebuild_after_apparent_execution_boundaries() {
    let _test_guard = super::lock();
    let _environment = bundled_environment();
    for (name, root_source, included_source) in [
        (
            "inactive-root-end",
            "\\documentclass{article}\n\\begin{document}\n\\iffalse\\end{document}\\fi\nOLD-CONTENT\\end{document}\n",
            None,
        ),
        (
            "macro-root-end",
            "\\documentclass{article}\n\\def\\finish{\\end{document}}\n\\begin{document}\nOLD-CONTENT\\finish\n",
            None,
        ),
        (
            "inactive-input-end",
            "\\documentclass{article}\n\\begin{document}\\input{section}\\end{document}\n",
            Some("\\iffalse\\endinput\\fi\nOLD-CONTENT\n"),
        ),
        (
            "indirect-catcode",
            "\\documentclass{article}\n\\let\\setcode\\catcode\n\\begin{document}\n\\setcode37=12\n% OLD-CONTENT\n\\setcode37=14\n\\end{document}\n",
            None,
        ),
    ] {
        let root = unique_temp_dir(name);
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).unwrap();
        let main = root.join("main.tex");
        fs::write(&main, root_source).unwrap();
        let changed = if let Some(source) = included_source {
            let section = root.join("section.tex");
            fs::write(&section, source).unwrap();
            section
        } else {
            main.clone()
        };
        let options = BuildOptions {
            external_tools: false,
            ..options(&main, &out_dir)
        };
        let first = build(&options).unwrap();
        assert!(!first.skipped);
        assert!(build(&options).unwrap().skipped, "{name}");
        let initial_pdf = fs::read(out_dir.join("main.pdf")).unwrap();
        let source = fs::read_to_string(&changed).unwrap();
        fs::write(&changed, source.replace("OLD-CONTENT", "NEW-CONTENT")).unwrap();
        let next = build(&options).unwrap();
        assert!(!next.skipped, "{name}: {next:#?}");
        assert!(next.tex_runs > 0);
        assert_ne!(
            initial_pdf,
            fs::read(out_dir.join("main.pdf")).unwrap(),
            "{name}"
        );
        assert!(build(&options).unwrap().skipped, "{name}");
        assert_pdf_text_when_available(&out_dir.join("main.pdf"), "NEW-CONTENT", "OLD-CONTENT");
        fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn new_higher_priority_inputs_rebuild_without_forcing() {
    let _test_guard = super::lock();
    let _environment = bundled_environment();
    let root = unique_temp_dir("tekai-cache-lookup-precedence");
    let out_dir = root.join("out");
    fs::create_dir_all(root.join("project/a")).unwrap();
    fs::create_dir_all(root.join("project/z")).unwrap();
    fs::create_dir_all(&out_dir).unwrap();
    let main = root.join("main.tex");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\\input{choice}\\end{document}\n",
    )
    .unwrap();
    fs::write(root.join("project/z/choice.tex"), "OLD-CHOICE\n").unwrap();
    let _inputs = EnvVarGuard::set(
        "TEXINPUTS",
        format!("{}//:", root.join("project").display()),
    );
    let _program_inputs = EnvVarGuard::set(
        "TEXINPUTS_pdflatex",
        format!("{}//:", root.join("project").display()),
    );
    let options = BuildOptions {
        external_tools: false,
        ..options(&main, &out_dir)
    };
    assert!(!build(&options).unwrap().skipped);
    assert!(build(&options).unwrap().skipped);
    fs::write(root.join("project/a/choice.tex"), "NEW-CHOICE\n").unwrap();
    let next = build(&options).unwrap();
    assert!(!next.skipped, "{next:#?}");
    assert!(next.tex_runs > 0);
    assert!(build(&options).unwrap().skipped);
    assert_pdf_text_when_available(&out_dir.join("main.pdf"), "NEW-CHOICE", "OLD-CHOICE");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn implicit_generated_namespaces_are_ignored_but_explicit_search_remains_available() {
    let _test_guard = super::lock();
    let _environment = bundled_environment();
    let root = unique_temp_dir("tekai-generated-lookup-policy");
    let out_dir = root.join("a-output");
    let cache_dir = root.join("b-cache");
    fs::create_dir_all(root.join("z-source")).unwrap();
    fs::create_dir_all(out_dir.join("nested")).unwrap();
    fs::create_dir_all(cache_dir.join("nested")).unwrap();
    let _cache = EnvVarGuard::set("TEKAI_AUX_CACHE", cache_dir.display().to_string());
    let main = root.join("main.tex");
    fs::write(&main, "\\documentclass{article}\n\\usepackage{choice}\n\\begin{document}\\choicecontent\\end{document}\n").unwrap();
    let package =
        |word| format!("\\ProvidesPackage{{choice}}\n\\newcommand{{\\choicecontent}}{{{word}}}\n");
    fs::write(root.join("z-source/choice.sty"), package("SOURCE-CHOICE")).unwrap();
    let options = BuildOptions {
        external_tools: false,
        ..options(&main, &out_dir)
    };
    assert!(!build(&options).unwrap().skipped);
    assert!(build(&options).unwrap().skipped);
    fs::write(out_dir.join("nested/choice.sty"), package("OUTPUT-CHOICE")).unwrap();
    fs::write(cache_dir.join("nested/choice.sty"), package("CACHE-CHOICE")).unwrap();
    assert!(build(&options).unwrap().skipped);
    assert!(
        !build(&BuildOptions {
            force: true,
            ..options.clone()
        })
        .unwrap()
        .skipped
    );
    assert_pdf_text_when_available(&out_dir.join("main.pdf"), "SOURCE-CHOICE", "OUTPUT-CHOICE");
    let route = format!("{}//:", out_dir.join("nested").display());
    let _generic = EnvVarGuard::set("TEXINPUTS", route.clone());
    let _program = EnvVarGuard::set("TEXINPUTS_pdflatex", route);
    // Explicit routes retain their resolver semantics, but cannot reuse an
    // inventory which intentionally excludes those mutable generated trees.
    for _ in 0..2 {
        assert!(!build(&options).unwrap().skipped);
        assert!(!out_dir.join(".tekai-main.state.json").exists());
    }
    assert_pdf_text_when_available(&out_dir.join("main.pdf"), "OUTPUT-CHOICE", "SOURCE-CHOICE");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn explicit_output_tree_sources_and_pdfs_are_real_cache_inputs() {
    let _test_guard = super::lock();
    let _environment = bundled_environment();
    let root = unique_temp_dir("tekai-explicit-generated-inputs");
    let out_dir = root.join("out");
    fs::create_dir_all(&out_dir).unwrap();
    let main = root.join("main.tex");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\\input{out/user.tex}\\end{document}\n",
    )
    .unwrap();
    fs::write(out_dir.join("user.tex"), "OLD-TEX\n").unwrap();
    let options = BuildOptions {
        external_tools: false,
        ..options(&main, &out_dir)
    };
    assert!(!build(&options).unwrap().skipped);
    assert!(build(&options).unwrap().skipped);
    fs::write(out_dir.join("user.tex"), "NEW-TEX\n").unwrap();
    assert!(!build(&options).unwrap().skipped);
    assert!(build(&options).unwrap().skipped);
    assert_pdf_text_when_available(&out_dir.join("main.pdf"), "NEW-TEX", "OLD-TEX");

    let fixture = root.join("fixture.tex");
    let fixture_source =
        |word| format!("\\documentclass{{article}}\n\\begin{{document}}{word}\\end{{document}}\n");
    fs::write(&fixture, fixture_source("OLD-PDF")).unwrap();
    build(&options_for_fixture(&fixture, &out_dir)).unwrap();
    fs::copy(out_dir.join("fixture.pdf"), out_dir.join("user.pdf")).unwrap();
    fs::write(&main, "\\documentclass{article}\n\\usepackage{graphicx}\n\\begin{document}\\includegraphics[width=5cm]{out/user.pdf}\\end{document}\n").unwrap();
    assert!(!build(&options).unwrap().skipped);
    assert!(build(&options).unwrap().skipped);
    fs::write(&fixture, fixture_source("NEW-PDF")).unwrap();
    build(&options_for_fixture(&fixture, &out_dir)).unwrap();
    fs::copy(out_dir.join("fixture.pdf"), out_dir.join("user.pdf")).unwrap();
    assert!(!build(&options).unwrap().skipped);
    assert!(build(&options).unwrap().skipped);
    assert_pdf_text_when_available(&out_dir.join("main.pdf"), "NEW-PDF", "OLD-PDF");
    fs::remove_dir_all(root).unwrap();
}

fn options_for_fixture(main: &Path, out_dir: &Path) -> BuildOptions {
    BuildOptions {
        external_tools: false,
        ..options(main, out_dir)
    }
}

#[test]
fn edits_during_compilation_never_publish_a_fresh_cache_state() {
    let _test_guard = super::lock();
    let _environment = bundled_environment();
    for once in [false, true] {
        let root = unique_temp_dir("tekai-cache-input-generation");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).unwrap();
        let main = root.join("main.tex");
        let source = "\\documentclass{article}\n\\newwrite\\markerfile\\newcount\\loopcount\n\\begin{document}\nOLD-CONTENT\\par\n\\immediate\\openout\\markerfile=consumed.marker\n\\immediate\\write\\markerfile{consumed}\\immediate\\closeout\\markerfile\n\\loop\\advance\\loopcount by1\\ifnum\\loopcount<1000000\\repeat\n\\end{document}\n";
        fs::write(&main, source).unwrap();
        let marker = out_dir.join("consumed.marker");
        let edited_main = main.clone();
        let editor = std::thread::spawn(move || {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
            while !marker.is_file() {
                assert!(
                    std::time::Instant::now() < deadline,
                    "engine never consumed the old input"
                );
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            fs::write(edited_main, source.replace("OLD-CONTENT", "NEW-CONTENT")).unwrap();
        });
        let options = BuildOptions {
            once,
            external_tools: false,
            ..options(&main, &out_dir)
        };
        let first = build(&options);
        editor.join().unwrap();
        let error = first.expect_err("changed input must reject publication");
        assert!(
            error.to_string().contains("changed during compilation"),
            "{error:#}"
        );
        assert!(!out_dir.join(".tekai-main.state.json").exists());
        let next = build(&options).unwrap();
        assert!(!next.skipped);
        assert!(next.tex_runs > 0);
        assert!(build(&options).unwrap().skipped);
        assert_pdf_text_when_available(&out_dir.join("main.pdf"), "NEW-CONTENT", "OLD-CONTENT");
        fs::remove_dir_all(root).unwrap();
    }
}

fn assert_pdf_text_when_available(pdf: &Path, expected: &str, stale: &str) {
    let output = std::process::Command::new("pdftotext")
        .arg(pdf)
        .arg("-")
        .output();
    if let Ok(output) = output {
        assert!(output.status.success());
        let text = String::from_utf8_lossy(&output.stdout);
        assert!(text.contains(expected), "{text}");
        assert!(!text.contains(stale), "{text}");
    }
}

#[test]
fn concurrent_library_builds_publish_once_then_reuse_the_cache() {
    let _test_guard = super::lock();
    let _environment = bundled_environment();
    let root = unique_temp_dir("tekai-concurrent-library-builds");
    fs::create_dir_all(&root).unwrap();
    let main = root.join("main.tex");
    fs::write(&main, "\\documentclass{article}\n\\newcount\\loopcount\n\\begin{document}\nConcurrent content.\\loop\\advance\\loopcount by1\\ifnum\\loopcount<1000000\\repeat\n\\end{document}\n").unwrap();
    let out_dir = root.join("out");
    let options = BuildOptions {
        external_tools: false,
        ..options(&main, &out_dir)
    };
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
    let builds = (0..2)
        .map(|_| {
            let options = options.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                build(&options).unwrap()
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    let reports = builds
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        reports.iter().filter(|report| !report.skipped).count(),
        1,
        "{reports:#?}"
    );
    assert_eq!(
        reports.iter().filter(|report| report.skipped).count(),
        1,
        "{reports:#?}"
    );
    assert!(build(&options).unwrap().skipped);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn distinct_output_builds_serialize_shared_cache_save_and_restore() {
    let _test_guard = super::lock();
    let _environment = bundled_environment();
    let root = unique_temp_dir("tekai-concurrent-shared-cache");
    let generated = root.join("artifacts");
    let first_out = generated.join("first");
    let second_out = generated.join("second");
    fs::create_dir_all(&first_out).unwrap();
    fs::create_dir_all(&second_out).unwrap();
    let _cache = EnvVarGuard::set("TEKAI_AUX_CACHE", generated.display().to_string());
    let main = root.join("main.tex");
    fs::write(&main, AUX_DOC).unwrap();
    let first = BuildOptions {
        external_tools: false,
        ..options(&main, &first_out)
    };
    let second = BuildOptions {
        external_tools: false,
        ..options(&main, &second_out)
    };
    build(&first).unwrap();
    let prior_pdf = fs::read(first_out.join("main.pdf")).unwrap();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
    let writer = {
        let barrier = barrier.clone();
        let options = BuildOptions {
            force: true,
            ..first.clone()
        };
        std::thread::spawn(move || {
            barrier.wait();
            build(&options).unwrap()
        })
    };
    let reader = {
        let barrier = barrier.clone();
        let options = second.clone();
        std::thread::spawn(move || {
            barrier.wait();
            build(&options).unwrap()
        })
    };
    barrier.wait();
    assert!(!writer.join().unwrap().skipped);
    let restored = reader.join().unwrap();
    assert!(restored.skipped, "{restored:#?}");
    assert_eq!(restored.tex_runs, 0);
    let saved_pdf = fs::read(first_out.join("main.pdf")).unwrap();
    let restored_pdf = fs::read(second_out.join("main.pdf")).unwrap();
    // Restore can own the shared namespace before or after the force-save.
    // It must accept one complete generation, never a mixed publication.
    assert!(restored_pdf == prior_pdf || restored_pdf == saved_pdf);
    assert!(build(&first).unwrap().skipped);
    assert!(build(&second).unwrap().skipped);
    fs::remove_dir_all(root).unwrap();
}

pub(super) struct BundledEnvironment {
    _guards: [EnvVarGuard; 4],
}

#[cfg(unix)]
#[test]
fn absolute_bibliography_source_alias_retarget_rebuilds_and_repairs_owned_state() {
    use std::os::unix::fs::symlink;

    let _test_guard = super::lock();
    let _environment = bundled_environment();
    let root = unique_temp_dir("tekai-bibliography-source-alias");
    let project = root.join("project");
    let external = root.join("external");
    let old = external.join("old");
    let new = external.join("new");
    let alias = external.join("selected");
    let out = project.join("out");
    fs::create_dir_all(&out).unwrap();
    fs::create_dir_all(&old).unwrap();
    fs::create_dir_all(&new).unwrap();
    for (directory, title) in [(&old, "OldAliasTitle"), (&new, "NewAliasTitle")] {
        fs::write(
            directory.join("refs.bib"),
            format!("@book{{x, author={{A. Author}}, title={{{title}}}, year={{2026}}}}\n"),
        )
        .unwrap();
    }
    symlink(&old, &alias).unwrap();
    let _bib_cache = EnvVarGuard::set(
        "TEKAI_BIBTEX_CACHE",
        root.join("bibliography-cache").display().to_string(),
    );
    let _aux_cache = EnvVarGuard::set(
        "TEKAI_AUX_CACHE",
        root.join("aux-cache").display().to_string(),
    );
    let main = project.join("main.tex");
    fs::write(
        &main,
        format!(
            "\\documentclass{{article}}\n\\begin{{document}}\\cite{{x}}\n\\bibliographystyle{{plain}}\n\\bibliography{{{}/refs}}\n\\end{{document}}\n",
            alias.display()
        ),
    )
    .unwrap();
    let options = BuildOptions {
        external_tools: false,
        ..options(&main, &out)
    };
    let first = build(&options).unwrap();
    assert!(!first.skipped);
    assert_eq!(first.bibliography_runs, 1, "{first:#?}");
    assert!(
        fs::read_to_string(out.join("main.bbl"))
            .unwrap()
            .contains("OldAliasTitle")
    );
    assert!(build(&options).unwrap().skipped);
    let state = fs::read_to_string(out.join(".tekai-main.state.json")).unwrap();
    assert!(state.contains("tekai-input-alias:"));
    fs::remove_file(&alias).unwrap();
    symlink(&new, &alias).unwrap();
    let next = build(&options).unwrap();
    assert!(!next.skipped, "{next:#?}");
    assert_eq!(next.bibliography_runs, 1, "{next:#?}");
    let bibliography = fs::read_to_string(out.join("main.bbl")).unwrap();
    assert!(bibliography.contains("NewAliasTitle"), "{bibliography}");
    assert!(!bibliography.contains("OldAliasTitle"), "{bibliography}");
    assert!(build(&options).unwrap().skipped);

    // Interrupted owned metadata must not prevent a forced build from repair.
    fs::write(out.join(".tekai-main.state.json"), b"\xff").unwrap();
    fs::write(out.join(".tekai-main.bibstate.toml"), "version = [").unwrap();
    let repaired = build(&BuildOptions {
        force: true,
        ..options.clone()
    })
    .unwrap();
    assert!(!repaired.skipped);
    assert_eq!(repaired.bibliography_runs, 1, "{repaired:#?}");
    assert!(build(&options).unwrap().skipped);
    let state: toml::Value =
        toml::from_str(&fs::read_to_string(out.join(".tekai-main.bibstate.toml")).unwrap())
            .unwrap();
    assert!(state["inputs"].as_array().unwrap().iter().any(|input| {
        input["path"]
            .as_str()
            .unwrap()
            .starts_with("tekai-input-alias:")
    }));
    fs::remove_dir_all(root).unwrap();
}

pub(super) fn bundled_environment() -> BundledEnvironment {
    BundledEnvironment {
        _guards: [
            EnvVarGuard::set(
                tekai::compiler::EMBEDDED_ENGINE_RUNNER_ENV,
                env!("CARGO_BIN_EXE_tekai").to_string(),
            ),
            EnvVarGuard::set("TEKAI_TEXMF_MODE", "bundled".to_string()),
            EnvVarGuard::set("TEXINPUTS", String::new()),
            EnvVarGuard::set("TEXINPUTS_pdflatex", String::new()),
        ],
    }
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
        external_tools: true,
        quiet: true,
        print_command: false,
    }
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

struct EnvVarGuard {
    name: &'static str,
    previous: Option<OsString>,
}

impl EnvVarGuard {
    fn set(name: &'static str, value: String) -> Self {
        let previous = std::env::var_os(name);
        unsafe {
            std::env::set_var(name, value);
        }
        Self { name, previous }
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

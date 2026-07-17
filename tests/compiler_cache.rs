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
    if !command_available("pdflatex") {
        eprintln!("skipping cache test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, AUX_DOC).expect("failed to write test document");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

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
    if !command_available("pdflatex") {
        eprintln!("skipping scheduler policy cache test; pdflatex is not available");
        return;
    }

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
    if !command_available("pdflatex") {
        eprintln!("skipping shared artifact cache test; pdflatex is not available");
        return;
    }

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
    assert!(fresh_out.join(".tekai-main.state.toml").exists());

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
fn trailing_content_after_end_document_does_not_invalidate_root_cache() {
    if !command_available("pdflatex") {
        eprintln!("skipping root effective-source cache test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-effective-root-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, AUX_DOC).expect("failed to write test document");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    fs::write(
        &main,
        format!("{AUX_DOC}\n% trailing notebook text ignored after end{{document}}\n"),
    )
    .expect("failed to append trailing content");
    let trailing = build(&options(&main, &out_dir)).expect("trailing-content build failed");
    assert!(trailing.skipped, "{trailing:#?}");
    assert_eq!(trailing.tex_runs, 0, "{trailing:#?}");

    fs::write(&main, AUX_DOC.replace("Intro", "Changed Intro"))
        .expect("failed to edit effective document body");
    let body_edit = build(&options(&main, &out_dir)).expect("body edit build failed");
    assert!(!body_edit.skipped, "{body_edit:#?}");
    assert!(body_edit.tex_runs > 0, "{body_edit:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn trailing_content_after_endinput_does_not_invalidate_tex_input_cache() {
    if !command_available("pdflatex") {
        eprintln!("skipping endinput cache test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-effective-endinput-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let section = root.join("section.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\n\\input{section}\n\\end{document}\n",
    )
    .expect("failed to write test document");
    fs::write(
        &section,
        "Visible section text.\n\\endinput same-line text stays visible\nignored tail\n",
    )
    .expect("failed to write section");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    fs::write(
        &section,
        "Visible section text.\n\\endinput same-line text stays visible\nchanged ignored tail\n",
    )
    .expect("failed to edit ignored section tail");
    let trailing = build(&options(&main, &out_dir)).expect("endinput-tail build failed");
    assert!(trailing.skipped, "{trailing:#?}");
    assert_eq!(trailing.tex_runs, 0, "{trailing:#?}");

    fs::write(
        &section,
        "Visible section text.\n\\endinput changed same-line text stays visible\nchanged ignored tail\n",
    )
    .expect("failed to edit effective endinput line");
    let same_line = build(&options(&main, &out_dir)).expect("endinput-line build failed");
    assert!(!same_line.skipped, "{same_line:#?}");
    assert!(same_line.tex_runs > 0, "{same_line:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn physical_trailing_spaces_do_not_invalidate_tex_cache() {
    if !command_available("pdflatex") {
        eprintln!("skipping trailing-space cache test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-trailing-space-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let section = root.join("section.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\n\\input{section}\n\\end{document}\n",
    )
    .expect("failed to write test document");
    fs::write(&section, "Visible section text.\n").expect("failed to write section");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    fs::write(
        &main,
        "\\documentclass{article}   \n\\begin{document}   \n\\input{section}   \n\\end{document}   \n",
    )
    .expect("failed to add trailing spaces to root");
    fs::write(&section, "Visible section text.   \n").expect("failed to edit section");
    let whitespace = build(&options(&main, &out_dir)).expect("trailing-space build failed");
    assert!(whitespace.skipped, "{whitespace:#?}");
    assert_eq!(whitespace.tex_runs, 0, "{whitespace:#?}");

    fs::write(&section, "Visible  section text.   \n").expect("failed to edit effective text");
    let body_edit = build(&options(&main, &out_dir)).expect("body edit build failed");
    assert!(!body_edit.skipped, "{body_edit:#?}");
    assert!(body_edit.tex_runs > 0, "{body_edit:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn column_zero_comment_text_does_not_invalidate_tex_cache() {
    if !command_available("pdflatex") {
        eprintln!("skipping full-line comment cache test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-full-line-comment-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let section = root.join("section.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n% root note before body\n\\begin{document}\n\\input{section}\n\\end{document}\n",
    )
    .expect("failed to write test document");
    fs::write(&section, "% section note\nVisible section text.\n")
        .expect("failed to write section");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    fs::write(
        &main,
        "\\documentclass{article}\n% changed root note before body\n\\begin{document}\n\\input{section}\n\\end{document}\n",
    )
    .expect("failed to edit root comment");
    fs::write(&section, "% changed section note\nVisible section text.\n")
        .expect("failed to edit section comment");
    let comment_edit = build(&options(&main, &out_dir)).expect("comment edit build failed");
    assert!(comment_edit.skipped, "{comment_edit:#?}");
    assert_eq!(comment_edit.tex_runs, 0, "{comment_edit:#?}");

    fs::write(
        &section,
        "  % indented comment can contribute spacing\nVisible section text.\n",
    )
    .expect("failed to edit effective section spacing");
    let indented_comment = build(&options(&main, &out_dir)).expect("indented comment build failed");
    assert!(!indented_comment.skipped, "{indented_comment:#?}");
    assert!(indented_comment.tex_runs > 0, "{indented_comment:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn ordinary_comment_text_does_not_invalidate_tex_cache() {
    if !command_available("pdflatex") {
        eprintln!("skipping ordinary comment cache test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-comment-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let section = root.join("section.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nVisible root text. % root note\n\\input{section}\n\\end{document}\n",
    )
    .expect("failed to write test document");
    fs::write(&section, "Visible section text. % section note\n").expect("failed to write section");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nVisible root text. % changed root note\n% added full-line note\n\\input{section}\n\\end{document}\n",
    )
    .expect("failed to edit root comments");
    fs::write(&section, "Visible section text. % changed section note\n")
        .expect("failed to edit section comment");
    let comment_edit = build(&options(&main, &out_dir)).expect("comment edit build failed");
    assert!(comment_edit.skipped, "{comment_edit:#?}");
    assert_eq!(comment_edit.tex_runs, 0, "{comment_edit:#?}");

    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nVisible root text.    % changed root note plus editor padding\n% added full-line note\n\\input{section}\n\\end{document}\n",
    )
    .expect("failed to edit inline comment padding");
    fs::write(
        &section,
        "Visible section text.    % changed section note plus editor padding\n",
    )
    .expect("failed to edit section inline comment padding");
    let padded_comment_edit =
        build(&options(&main, &out_dir)).expect("comment padding edit build failed");
    assert!(padded_comment_edit.skipped, "{padded_comment_edit:#?}");
    assert_eq!(padded_comment_edit.tex_runs, 0, "{padded_comment_edit:#?}");

    fs::write(
        &section,
        "Visible section text.% removed effective spacing before comment\n",
    )
    .expect("failed to remove effective section comment spacing");
    let tight_comment_edit =
        build(&options(&main, &out_dir)).expect("tight comment edit build failed");
    assert!(!tight_comment_edit.skipped, "{tight_comment_edit:#?}");
    assert!(tight_comment_edit.tex_runs > 0, "{tight_comment_edit:#?}");

    fs::write(
        &section,
        "Visible changed section text. % changed section note\n",
    )
    .expect("failed to edit effective section text");
    let body_edit = build(&options(&main, &out_dir)).expect("body edit build failed");
    assert!(!body_edit.skipped, "{body_edit:#?}");
    assert!(body_edit.tex_runs > 0, "{body_edit:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn verbatim_percent_text_does_invalidate_tex_cache() {
    if !command_available("pdflatex") {
        eprintln!("skipping verbatim percent cache test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-verbatim-percent-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\n\\begin{verbatim}\n% visible one\n\\end{verbatim}\n\\end{document}\n",
    )
    .expect("failed to write test document");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\n\\begin{verbatim}\n% visible two\n\\end{verbatim}\n\\end{document}\n",
    )
    .expect("failed to edit verbatim percent text");
    let percent_edit = build(&options(&main, &out_dir)).expect("verbatim edit build failed");
    assert!(!percent_edit.skipped, "{percent_edit:#?}");
    assert!(percent_edit.tex_runs > 0, "{percent_edit:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn inline_verb_does_not_disable_safe_effective_tex_cache() {
    if !command_available("pdflatex") {
        eprintln!("skipping inline verb cache test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-inline-verb-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nInline \\verb|% literal \\end{document}| text. % note\n\\end{document}\n",
    )
    .expect("failed to write test document");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nInline \\verb|% literal \\end{document}| text.    % edited note\n\\end{document}\n% trailing notebook text\n",
    )
    .expect("failed to edit ignored comment and trailing content");
    let comment_edit = build(&options(&main, &out_dir)).expect("inline verb comment edit failed");
    assert!(comment_edit.skipped, "{comment_edit:#?}");
    assert_eq!(comment_edit.tex_runs, 0, "{comment_edit:#?}");

    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nInline \\verb|% changed literal \\end{document}| text.    % edited note\n\\end{document}\n% trailing notebook text\n",
    )
    .expect("failed to edit inline verb literal");
    let literal_edit = build(&options(&main, &out_dir)).expect("inline verb literal edit failed");
    assert!(!literal_edit.skipped, "{literal_edit:#?}");
    assert!(literal_edit.tex_runs > 0, "{literal_edit:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn lstinline_does_not_disable_safe_effective_tex_cache() {
    if !command_available("pdflatex") || !tex_file_available("listings.sty") {
        eprintln!("skipping lstinline cache test; pdflatex or listings.sty is not available");
        return;
    }

    let root = unique_temp_dir("tekai-lstinline-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\usepackage{listings}\n\\begin{document}\nInline \\lstinline|% literal \\end{document}| text. % note\n\\end{document}\n",
    )
    .expect("failed to write test document");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    fs::write(
        &main,
        "\\documentclass{article}\n\\usepackage{listings}\n\\begin{document}\nInline \\lstinline|% literal \\end{document}| text.    % edited note\n\\end{document}\n% trailing notebook text\n",
    )
    .expect("failed to edit ignored comment and trailing content");
    let comment_edit = build(&options(&main, &out_dir)).expect("lstinline comment edit failed");
    assert!(comment_edit.skipped, "{comment_edit:#?}");
    assert_eq!(comment_edit.tex_runs, 0, "{comment_edit:#?}");

    fs::write(
        &main,
        "\\documentclass{article}\n\\usepackage{listings}\n\\begin{document}\nInline \\lstinline|% changed literal \\end{document}| text.    % edited note\n\\end{document}\n% trailing notebook text\n",
    )
    .expect("failed to edit lstinline literal");
    let literal_edit = build(&options(&main, &out_dir)).expect("lstinline literal edit failed");
    assert!(!literal_edit.skipped, "{literal_edit:#?}");
    assert!(literal_edit.tex_runs > 0, "{literal_edit:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn percent_lines_with_explicit_catcode_do_invalidate_tex_cache() {
    if !command_available("pdflatex") {
        eprintln!("skipping percent catcode cache test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-percent-catcode-cache-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\n\\catcode37=12\n% visible percent text one\n\\catcode37=14\n\\end{document}\n",
    )
    .expect("failed to write test document");

    let first = build(&options(&main, &out_dir)).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());

    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\n\\catcode37=12\n% visible percent text two\n\\catcode37=14\n\\end{document}\n",
    )
    .expect("failed to edit percent text");
    let percent_edit = build(&options(&main, &out_dir)).expect("percent edit build failed");
    assert!(!percent_edit.skipped, "{percent_edit:#?}");
    assert!(percent_edit.tex_runs > 0, "{percent_edit:#?}");

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
        .output()
        .is_ok_and(|output| output.status.success())
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

use std::fs;
use std::path::{Path, PathBuf};

use tekai::lint::{LintConfig, lint_paths, lint_source};

#[test]
fn bracket_math_is_accepted() {
    let diagnostics = lint_source(
        Path::new("paper.tex"),
        "\\[\n  E = mc^2\n\\]\nText \\(x + y\\).\n",
        &LintConfig::default(),
    );
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
}

#[test]
fn dollar_math_is_reported() {
    let diagnostics = lint_source(
        Path::new("paper.tex"),
        "Text $x$.\n$$y$$\n",
        &LintConfig::default(),
    );
    assert!(diagnostics.iter().any(|d| d.rule == "math/inline-dollar"));
    assert!(diagnostics.iter().any(|d| d.rule == "math/display-dollar"));
}

#[test]
fn inline_verbatim_is_not_linted_as_math_or_environment_syntax() {
    let diagnostics = lint_source(
        Path::new("paper.tex"),
        r"\verb|$x$ \begin{proof}| and \lstinline!\end{theorem} $$y$$! then $z$.",
        &LintConfig::default(),
    );

    assert_eq!(
        diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.rule == "math/inline-dollar")
            .count(),
        2,
        "{diagnostics:#?}"
    );
    assert!(!diagnostics.iter().any(|diagnostic| matches!(
        diagnostic.rule,
        "math/display-dollar"
            | "env/unmatched-end"
            | "env/unclosed"
            | "env/mismatch"
            | "indent/size"
    )));
}

#[test]
fn indentation_tracks_environments() {
    let diagnostics = lint_source(
        Path::new("paper.tex"),
        "\\begin{theorem}\n  Claim.\n\\end{theorem}\n",
        &LintConfig::default(),
    );
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");
}

#[test]
fn lint_paths_descends_into_directories() {
    let root = unique_temp_dir("tekai-lint-paths");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let source = root.join("paper.tex");
    fs::write(
        &source,
        "\\documentclass{article}\n\\begin{document}\n\tTabbed text.\n\\end{document}\n",
    )
    .expect("failed to write TeX source");

    let diagnostics =
        lint_paths(std::slice::from_ref(&root), &LintConfig::default()).expect("lint failed");
    assert!(diagnostics.iter().any(|d| d.rule == "indent/tabs"));

    let _ = fs::remove_dir_all(root);
}

#[test]
fn lint_paths_collects_tex_like_extensions_case_insensitively() {
    let root = unique_temp_dir("tekai-lint-extension-case");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let source = root.join("PAPER.TEX");
    fs::write(&source, "Text $x$.\n").expect("failed to write TeX source");

    let diagnostics =
        lint_paths(std::slice::from_ref(&root), &LintConfig::default()).expect("lint failed");

    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.rule == "math/inline-dollar"),
        "{diagnostics:#?}"
    );

    let _ = fs::remove_dir_all(root);
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

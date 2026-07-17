use std::fs;
use std::path::{Path, PathBuf};

use tekai::lint::{IndentStyle, LintConfig, fix_paths, lint_paths, lint_source};

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

#[test]
fn lint_paths_skip_style_files() {
    let root = unique_temp_dir("tekai-lint-skip-style");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    fs::write(root.join("paper.tex"), "Text \\(x\\).\n").expect("failed to write TeX source");
    fs::write(root.join("package.sty"), "Text $x$.\n").expect("failed to write style source");

    let diagnostics =
        lint_paths(std::slice::from_ref(&root), &LintConfig::default()).expect("lint failed");
    assert!(diagnostics.is_empty(), "{diagnostics:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fix_paths_rewrites_safe_issues_and_skips_style_files() {
    let root = unique_temp_dir("tekai-lint-fix");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let source = root.join("paper.tex");
    let style = root.join("package.sty");
    fs::write(
        &source,
        "\\begin{itemize}\n\t\\item Inline café $x$\n\\end{itemize}\n$$y$$\n",
    )
    .expect("failed to write TeX source");
    fs::write(&style, "Style $z$.\n").expect("failed to write style source");

    let report =
        fix_paths(std::slice::from_ref(&root), &LintConfig::default()).expect("fix failed");

    assert_eq!(report.files_changed, vec![source.clone()]);
    assert_eq!(
        fs::read_to_string(&source).expect("failed to read fixed source"),
        "\\begin{itemize}\n  \\item Inline café \\(x\\)\n\\end{itemize}\n\\[y\\]\n"
    );
    assert_eq!(
        fs::read_to_string(&style).expect("failed to read style source"),
        "Style $z$.\n"
    );
    assert!(
        lint_paths(std::slice::from_ref(&root), &LintConfig::default())
            .expect("lint failed")
            .is_empty()
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fix_paths_leave_suppressed_math_unchanged() {
    let root = unique_temp_dir("tekai-lint-fix-suppressed");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let source = root.join("paper.tex");
    let contents = "Legacy $x$. % tekai-ignore-line math/inline-dollar\n";
    fs::write(&source, contents).expect("failed to write TeX source");

    let report =
        fix_paths(std::slice::from_ref(&source), &LintConfig::default()).expect("fix failed");

    assert_eq!(report.fixes_applied, 0);
    assert_eq!(
        fs::read_to_string(&source).expect("failed to read source"),
        contents
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fix_paths_do_not_rewrite_structurally_invalid_math() {
    let root = unique_temp_dir("tekai-lint-fix-invalid-math");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let source = root.join("paper.tex");
    let contents = "Nested \\($x$\\).\n";
    fs::write(&source, contents).expect("failed to write TeX source");

    let report =
        fix_paths(std::slice::from_ref(&source), &LintConfig::default()).expect("fix failed");

    assert_eq!(report.fixes_applied, 0);
    assert_eq!(
        fs::read_to_string(&source).expect("failed to read source"),
        contents
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fix_paths_preserve_tabs_inside_verbatim_content() {
    let root = unique_temp_dir("tekai-lint-fix-verbatim");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let source = root.join("paper.tex");
    let contents = "\\begin{verbatim}\n\tliteral tab\n\\end{verbatim}\n";
    fs::write(&source, contents).expect("failed to write TeX source");

    let report =
        fix_paths(std::slice::from_ref(&source), &LintConfig::default()).expect("fix failed");

    assert_eq!(report.fixes_applied, 0);
    assert_eq!(
        fs::read_to_string(&source).expect("failed to read source"),
        contents
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fix_paths_convert_space_indentation_to_tabs() {
    let root = unique_temp_dir("tekai-lint-fix-tab-indentation");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let source = root.join("paper.tex");
    fs::write(
        &source,
        "\\begin{itemize}\n  \\item First\n  \\begin{enumerate}\n    \\item Nested\n  \\end{enumerate}\n\\end{itemize}\n",
    )
    .expect("failed to write TeX source");
    let config = LintConfig {
        indent_style: IndentStyle::Tabs,
        ..LintConfig::default()
    };

    let report = fix_paths(std::slice::from_ref(&source), &config).expect("fix failed");

    assert!(report.fixes_applied > 0, "{report:#?}");
    assert_eq!(
        fs::read_to_string(&source).expect("failed to read fixed source"),
        "\\begin{itemize}\n\t\\item First\n\t\\begin{enumerate}\n\t\t\\item Nested\n\t\\end{enumerate}\n\\end{itemize}\n"
    );
    assert!(
        lint_paths(std::slice::from_ref(&source), &config)
            .expect("lint failed")
            .is_empty()
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fix_paths_round_odd_space_indentation_to_tabs() {
    let root = unique_temp_dir("tekai-lint-fix-odd-tab-indentation");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let source = root.join("paper.tex");
    fs::write(&source, "   manually aligned text\n").expect("failed to write TeX source");
    let config = LintConfig {
        indent_style: IndentStyle::Tabs,
        indent_environments: false,
        ..LintConfig::default()
    };

    fix_paths(std::slice::from_ref(&source), &config).expect("fix failed");

    assert_eq!(
        fs::read_to_string(&source).expect("failed to read fixed source"),
        "\t\tmanually aligned text\n"
    );
    assert!(
        lint_paths(std::slice::from_ref(&source), &config)
            .expect("lint failed")
            .is_empty()
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

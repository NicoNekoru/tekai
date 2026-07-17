use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

const FINAL_GRAPHICX_MISSING_IMAGE_DOC: &str = r#"\documentclass{article}
\usepackage[final]{graphicx}
\begin{document}
Before \includegraphics[width=1cm]{missing-image} After.
\end{document}
"#;

const MINIMAL_DOC: &str = r#"\documentclass{article}
\begin{document}
Hello.
\end{document}
"#;

const SHARED_PACKAGE_DOC: &str = r#"\documentclass{article}
\usepackage{sharedpkg}
\begin{document}
\sharedword
\end{document}
"#;

#[test]
fn build_uses_configured_fast_preview_defaults() {
    if !command_available("pdflatex") {
        eprintln!("skipping build config CLI test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-cli-build-config-fast");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    fs::write(root.join("main.tex"), FINAL_GRAPHICX_MISSING_IMAGE_DOC)
        .expect("failed to write TeX source");
    fs::write(
        root.join("tekai.toml"),
        r#"
        [build]
        out_dir = "configured-out"
        fast = true
        once = true
        quiet = true
        "#,
    )
    .expect("failed to write config");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .current_dir(&root)
        .args([
            "build",
            "main.tex",
            "--config",
            "tekai.toml",
            "--report-json",
        ])
        .output()
        .expect("failed to run tekai build");

    assert_success(&output, "configured fast preview build");
    assert!(root.join("configured-out/main.pdf").exists());

    let report: Value =
        serde_json::from_slice(&output.stdout).expect("build stdout should be valid JSON");
    assert_eq!(report["tex_runs"], 1);
    assert!(
        report["pdf_path"]
            .as_str()
            .is_some_and(|path| path.ends_with("configured-out/main.pdf")),
        "{report:#}"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn build_uses_configured_texinputs_environment() {
    if !command_available("pdflatex") {
        eprintln!("skipping build config env CLI test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-cli-build-config-env");
    let shared = root.join("shared-tex");
    fs::create_dir_all(&shared).expect("failed to create shared tex directory");
    fs::write(root.join("main.tex"), SHARED_PACKAGE_DOC).expect("failed to write TeX source");
    fs::write(
        shared.join("sharedpkg.sty"),
        r#"\ProvidesPackage{sharedpkg}
\newcommand{\sharedword}{Shared package loaded.}
"#,
    )
    .expect("failed to write shared package");
    let separator = if cfg!(windows) { ";" } else { ":" };
    let texinputs = format!("{}//{}", shared.display(), separator);
    fs::write(
        root.join("tekai.toml"),
        format!(
            r#"
            [build]
            out_dir = "configured-out"
            once = true
            quiet = true

            [build.env]
            TEXINPUTS = "{}"
            "#,
            toml_basic_string_escape(&texinputs)
        ),
    )
    .expect("failed to write config");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .current_dir(&root)
        .args([
            "build",
            "main.tex",
            "--config",
            "tekai.toml",
            "--report-json",
        ])
        .output()
        .expect("failed to run tekai build");

    assert_success(&output, "configured TEXINPUTS build");
    assert!(root.join("configured-out/main.pdf").exists());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn explicit_build_flags_override_config_defaults() {
    if !command_available("pdflatex") {
        eprintln!("skipping build config CLI test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-cli-build-config-override");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    fs::write(root.join("main.tex"), MINIMAL_DOC).expect("failed to write TeX source");
    fs::write(
        root.join("tekai.toml"),
        r#"
        [build]
        out_dir = "configured-out"
        once = true
        quiet = true
        "#,
    )
    .expect("failed to write config");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .current_dir(&root)
        .args([
            "build",
            "main.tex",
            "--config",
            "tekai.toml",
            "--out-dir",
            "cli-out",
            "--report-json",
        ])
        .output()
        .expect("failed to run tekai build");

    assert_success(&output, "CLI override build");
    assert!(root.join("cli-out/main.pdf").exists());
    assert!(!root.join("configured-out/main.pdf").exists());

    let _ = fs::remove_dir_all(root);
}

fn assert_success(output: &std::process::Output, label: &str) {
    assert!(
        output.status.success(),
        "{label} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn command_available(name: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {name} >/dev/null 2>&1"))
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn toml_basic_string_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
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

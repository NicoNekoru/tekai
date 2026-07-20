use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

#[test]
fn check_fix_rewrites_before_reporting_remaining_diagnostics() {
    let root = unique_temp_dir("tekai-cli-check-fix");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let source = root.join("paper.tex");
    fs::write(&source, "Text $x$.\n\\begin{proof}\n").expect("failed to write TeX source");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .arg("check")
        .arg("--fix")
        .arg(&source)
        .output()
        .expect("failed to run tekai check --fix");

    assert_eq!(output.status.code(), Some(1), "{output:#?}");
    assert_eq!(
        fs::read_to_string(&source).expect("failed to read fixed source"),
        "Text \\(x\\).\n\\begin{proof}\n"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("fixed 2 issue(s) in 1 file(s)"),
        "stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("env/unclosed"),
        "stdout:\n{}",
        String::from_utf8_lossy(&output.stdout)
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn check_lints_only_the_referenced_source_graph() {
    let root = unique_temp_dir("tekai-cli-check-source-graph");
    let chapters = root.join("chapters");
    fs::create_dir_all(&chapters).expect("failed to create source tree");
    let main = root.join("main.tex");
    let referenced = chapters.join("referenced.tex");
    let unrelated = root.join("unrelated.tex");
    fs::write(&main, "\\input{chapters/referenced}\n").expect("failed to write root TeX source");
    fs::write(&referenced, "Referenced $x$.\n").expect("failed to write referenced TeX source");
    fs::write(&unrelated, "Unrelated $y$.\n").expect("failed to write unrelated TeX source");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .arg("check")
        .arg(&main)
        .output()
        .expect("failed to run tekai check");

    assert_eq!(output.status.code(), Some(1), "{output:#?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(&referenced.display().to_string()),
        "referenced source was not linted:\n{stdout}"
    );
    assert!(
        !stdout.contains(&unrelated.display().to_string()),
        "unreferenced sibling was linted:\n{stdout}"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn check_always_lints_the_explicit_root() {
    let root = unique_temp_dir("tekai-cli-check-explicit-root");
    let ignored_name = root.join("build");
    fs::create_dir_all(&ignored_name).expect("failed to create source directory");
    let main = ignored_name.join("main.tex");
    fs::write(&main, "Explicit root $x$.\n").expect("failed to write root TeX source");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .arg("check")
        .arg(&main)
        .output()
        .expect("failed to run tekai check");

    assert_eq!(output.status.code(), Some(1), "{output:#?}");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(&main.display().to_string()),
        "explicit root was not linted:\n{stdout}"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn check_report_json_contains_the_diagnostics_that_gate_the_build() {
    let root = unique_temp_dir("tekai-cli-check-json-diagnostics");
    fs::create_dir_all(&root).expect("failed to create source directory");
    let main = root.join("main.tex");
    fs::write(&main, "\\begin{proof}\n").expect("failed to write root TeX source");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .args(["check", "--report-json"])
        .arg(&main)
        .output()
        .expect("failed to run tekai check --report-json");

    assert_eq!(output.status.code(), Some(1), "{output:#?}");
    assert!(
        output.stderr.is_empty(),
        "stderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value =
        serde_json::from_slice(&output.stdout).expect("check stdout should be valid JSON");
    assert_eq!(report["error_count"], 1);
    assert!(
        report.get("elapsed_ms").is_none(),
        "lint-blocked checks must not claim a build: {report:#}"
    );
    assert!(
        report["diagnostics"].as_array().is_some_and(|diagnostics| {
            diagnostics.iter().any(|diagnostic| {
                diagnostic["path"]
                    == main
                        .canonicalize()
                        .expect("failed to canonicalize root")
                        .display()
                        .to_string()
                    && diagnostic["rule"] == "env/unclosed"
            })
        }),
        "{report:#}"
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

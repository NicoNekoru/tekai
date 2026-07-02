use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

#[test]
fn clean_uses_configured_output_directory() {
    let root = unique_temp_dir("texpilot-cli-clean-config");
    let out_dir = root.join("configured-out");
    fs::create_dir_all(&out_dir).expect("failed to create output directory");
    fs::write(out_dir.join("main.aux"), "\\relax\n").expect("failed to write generated file");
    fs::write(
        root.join("texpilot.toml"),
        r#"
        [build]
        out_dir = "configured-out"
        "#,
    )
    .expect("failed to write config");

    let output = Command::new(env!("CARGO_BIN_EXE_texpilot"))
        .current_dir(&root)
        .args(["clean", "--config", "texpilot.toml"])
        .output()
        .expect("failed to run texpilot clean");

    assert_success(&output, "configured clean");
    assert!(
        !out_dir.exists(),
        "configured output directory should be removed"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn clean_dry_run_report_json_does_not_remove_output_directory() {
    let root = unique_temp_dir("texpilot-cli-clean-dry-run");
    let out_dir = root.join("out");
    fs::create_dir_all(&out_dir).expect("failed to create output directory");
    fs::write(out_dir.join("main.log"), "log\n").expect("failed to write generated file");

    let output = Command::new(env!("CARGO_BIN_EXE_texpilot"))
        .current_dir(&root)
        .args(["clean", "--out-dir", "out", "--dry-run", "--report-json"])
        .output()
        .expect("failed to run texpilot clean");

    assert_success(&output, "dry-run clean");
    assert!(
        out_dir.exists(),
        "dry-run should leave output directory intact"
    );

    let report: Value =
        serde_json::from_slice(&output.stdout).expect("clean stdout should be valid JSON");
    assert_eq!(report["existed"], true);
    assert_eq!(report["removed"], false);
    assert_eq!(report["dry_run"], true);
    assert!(
        report["path"]
            .as_str()
            .is_some_and(|path| path.ends_with("out")),
        "{report:#}"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn clean_refuses_current_directory() {
    let root = unique_temp_dir("texpilot-cli-clean-current-dir");
    fs::create_dir_all(&root).expect("failed to create temp directory");

    let output = Command::new(env!("CARGO_BIN_EXE_texpilot"))
        .current_dir(&root)
        .args(["clean", "--out-dir", "."])
        .output()
        .expect("failed to run texpilot clean");

    assert!(
        !output.status.success(),
        "clean should refuse current directory\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("refusing to clean"),
        "stderr should explain refusal: {}",
        String::from_utf8_lossy(&output.stderr)
    );

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

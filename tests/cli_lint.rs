use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

#[test]
fn lint_report_json_emits_machine_readable_diagnostics() {
    let root = unique_temp_dir("texpilot-cli-lint-json");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let source = root.join("paper.tex");
    fs::write(&source, "Text $x$.\n").expect("failed to write TeX source");

    let output = Command::new(env!("CARGO_BIN_EXE_texpilot"))
        .arg("lint")
        .arg("--report-json")
        .arg("--allow-warnings")
        .arg(&source)
        .output()
        .expect("failed to run texpilot lint");

    assert!(
        output.status.success(),
        "lint command failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "JSON lint mode should not print text diagnostics to stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report: Value =
        serde_json::from_slice(&output.stdout).expect("lint stdout should be valid JSON");
    assert_eq!(report["warning_count"], 2);
    assert_eq!(report["error_count"], 0);
    let diagnostics = report["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array");
    assert!(
        diagnostics.iter().any(|diagnostic| {
            diagnostic["rule"] == "math/inline-dollar"
                && diagnostic["severity"] == "warning"
                && diagnostic["line"] == 1
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

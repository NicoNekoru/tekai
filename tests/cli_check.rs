use std::fs;
use std::path::PathBuf;
use std::process::Command;

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

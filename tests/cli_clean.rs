use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use serde_json::Value;

#[test]
fn clean_uses_configured_output_directory() {
    let root = unique_temp_dir("tekai-cli-clean-config");
    let out_dir = root.join("configured-out");
    fs::create_dir_all(&out_dir).expect("failed to create output directory");
    fs::write(out_dir.join("main.aux"), "\\relax\n").expect("failed to write generated file");
    fs::write(
        root.join("tekai.toml"),
        r#"
        [build]
        out_dir = "configured-out"
        "#,
    )
    .expect("failed to write config");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .current_dir(&root)
        .args(["clean", "--config", "tekai.toml"])
        .output()
        .expect("failed to run tekai clean");

    assert_success(&output, "configured clean");
    assert!(
        !out_dir.exists(),
        "configured output directory should be removed"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn clean_dry_run_report_json_does_not_remove_output_directory() {
    let root = unique_temp_dir("tekai-cli-clean-dry-run");
    let out_dir = root.join("out");
    fs::create_dir_all(&out_dir).expect("failed to create output directory");
    fs::write(out_dir.join("main.log"), "log\n").expect("failed to write generated file");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .current_dir(&root)
        .args(["clean", "--out-dir", "out", "--dry-run", "--report-json"])
        .output()
        .expect("failed to run tekai clean");

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
    let root = unique_temp_dir("tekai-cli-clean-current-dir");
    fs::create_dir_all(&root).expect("failed to create temp directory");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .current_dir(&root)
        .args(["clean", "--out-dir", "."])
        .output()
        .expect("failed to run tekai clean");

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

#[test]
fn clean_waits_for_build_ownership_before_removing_outputs() {
    let root = unique_temp_dir("tekai-cli-clean-owned");
    let out_dir = root.join("out");
    fs::create_dir_all(&out_dir).unwrap();
    let artifact = out_dir.join("main.pdf");
    fs::write(&artifact, b"fixture artifact").unwrap();
    let lock = tekai::compiler::BuildOutputLock::acquire(&out_dir).unwrap();
    let mut process = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .current_dir(&root)
        .args(["clean", "--out-dir", "out", "--quiet"])
        .spawn()
        .unwrap();
    std::thread::sleep(Duration::from_millis(100));
    assert!(process.try_wait().unwrap().is_none());
    assert!(artifact.exists());
    drop(lock);
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(status) = process.try_wait().unwrap() {
            assert!(status.success());
            break;
        }
        if Instant::now() >= deadline {
            process.kill().unwrap();
            process.wait().unwrap();
            panic!("clean did not finish after ownership released");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(!out_dir.exists());
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn clean_refuses_an_ancestor_alias_back_to_the_current_project() {
    let root = unique_temp_dir("tekai-cli-clean-alias");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("source.tex"), "preserve this input").unwrap();
    std::os::unix::fs::symlink(root.parent().unwrap(), root.join("alias")).unwrap();
    let alias = PathBuf::from("alias").join(root.file_name().unwrap());
    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .current_dir(&root)
        .args(["clean", "--out-dir"])
        .arg(alias)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(root.join("source.tex").is_file());
    assert!(String::from_utf8_lossy(&output.stderr).contains("refusing to clean"));
    fs::remove_dir_all(root).unwrap();
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

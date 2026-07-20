use std::fs;
use std::path::PathBuf;
use std::process::Command;

const MINIMAL_DOC: &str = r#"\documentclass{article}
\begin{document}
SyncTeX from the embedded engine.
\end{document}
"#;

#[test]
fn embedded_engine_accepts_synctex_and_writes_the_sidecar() {
    let root = unique_temp_dir("tekai-cli-embedded-synctex");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    fs::write(root.join("main.tex"), MINIMAL_DOC).expect("failed to write TeX source");

    let output = run_synctex_build(&root);
    assert_success(&output, "embedded SyncTeX build");
    assert!(root.join("build/main.pdf").exists());

    let synctex_path = root.join("build/main.synctex.gz");
    let bytes = fs::read(&synctex_path).expect("SyncTeX sidecar should be readable");
    assert!(
        bytes.starts_with(&[0x1f, 0x8b]),
        "sidecar should be gzip data"
    );
    assert!(bytes.len() > 20, "sidecar should not be empty");
    assert!(
        !String::from_utf8_lossy(&output.stderr).contains("unrecognized option"),
        "embedded engine rejected -synctex=1\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_file(&synctex_path).expect("failed to remove SyncTeX sidecar");
    let rebuilt = run_synctex_build(&root);
    assert_success(&rebuilt, "missing SyncTeX sidecar rebuild");
    assert!(
        synctex_path.exists(),
        "a cached PDF must not hide a missing SyncTeX sidecar"
    );

    let _ = fs::remove_dir_all(root);
}

fn run_synctex_build(root: &PathBuf) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tekai"))
        .current_dir(root)
        .args([
            "build",
            "main.tex",
            "--out-dir",
            "build",
            "--synctex",
            "--once",
            "--quiet",
        ])
        .output()
        .expect("failed to run tekai build")
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

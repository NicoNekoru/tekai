use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use tekai::compiler::EMBEDDED_ENGINE_SUBCOMMAND;

const MINIMAL_DOC: &str = r#"\documentclass{article}
\begin{document}
SyncTeX from the embedded engine.
\end{document}
"#;

const OPTION_DOC: &str = r#"\documentclass{article}
\typeout{TEKAI-SYNCTEX=\the\synctex}
\typeout{TEKAI-EPOCH=\the\year-\the\month-\the\day,\the\time}
\begin{document}
Integer option parsing from the embedded engine.
\end{document}
"#;

#[test]
fn embedded_engine_accepts_synctex_and_writes_the_sidecar() {
    let fixture = Project::new();
    let root = &fixture.0;
    fs::write(root.join("main.tex"), MINIMAL_DOC).expect("failed to write TeX source");

    let output = run_synctex_build(&fixture);
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
    let rebuilt = run_synctex_build(&fixture);
    assert_success(&rebuilt, "missing SyncTeX sidecar rebuild");
    assert!(
        synctex_path.exists(),
        "a cached PDF must not hide a missing SyncTeX sidecar"
    );
}

#[test]
fn embedded_integer_options_preserve_prefix_epoch_and_sidecar_outcomes() {
    #[derive(Clone, Copy, Debug)]
    enum Sidecar {
        Gzip,
        Plain,
        Absent,
    }

    let fixture = Project::new();
    let cases = [
        ("decimal", "1", "0", Some((1, Sidecar::Gzip, "1970-1-1,0"))),
        (
            "decimal-prefix",
            "10tail",
            "42",
            Some((11, Sidecar::Gzip, "1970-1-1,0")),
        ),
        (
            "octal-empty-epoch",
            "010",
            "",
            Some((9, Sidecar::Gzip, "1970-1-1,0")),
        ),
        (
            "hex-negative-epoch",
            "0x4",
            "-1",
            Some((5, Sidecar::Gzip, "1969-12-31,1439")),
        ),
        (
            "negative-hex-max-epoch",
            "-0x4",
            "18446744073709551615",
            Some((-3, Sidecar::Plain, "1969-12-31,1439")),
        ),
        (
            "no-digits-c-whitespace",
            "garbage",
            "\x0b+42",
            Some((0, Sidecar::Absent, "1970-1-1,0")),
        ),
        (
            "empty-option",
            "",
            "0",
            Some((0, Sidecar::Absent, "1970-1-1,0")),
        ),
        (
            "zero-option",
            "0",
            "0",
            Some((0, Sidecar::Absent, "1970-1-1,0")),
        ),
        ("epoch-no-digits", "1", "\x0b", None),
        ("epoch-trailing-whitespace", "1", "42 ", None),
        ("epoch-hex-is-not-decimal", "1", "0x10", None),
        ("epoch-positive-overflow", "1", "18446744073709551616", None),
        (
            "epoch-negative-overflow",
            "1",
            "-18446744073709551616",
            None,
        ),
    ];

    for (label, synctex, epoch, expected) in cases {
        let root = fixture.0.join(label);
        fs::create_dir(&root).unwrap();
        fs::create_dir(root.join("build")).unwrap();
        fs::write(root.join("main.tex"), OPTION_DOC).unwrap();
        let output = fixture
            .command(&root)
            .env("SOURCE_DATE_EPOCH", epoch)
            .env("FORCE_SOURCE_DATE", "1")
            .env("TZ", "UTC")
            .arg(EMBEDDED_ENGINE_SUBCOMMAND)
            .args([
                "-interaction=nonstopmode",
                "-halt-on-error",
                "-no-shell-escape",
                "-fmt=pdflatex",
                "-output-directory=build",
            ])
            .arg(format!("-synctex={synctex}"))
            .arg("main.tex")
            .output()
            .expect("failed to run isolated embedded engine");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let plain = root.join("build/main.synctex");
        let gzip = root.join("build/main.synctex.gz");
        let pdf = root.join("build/main.pdf");

        let Some((value, sidecar, date)) = expected else {
            assert_eq!(output.status.code(), Some(1), "{label}\n{stdout}\n{stderr}");
            assert!(stderr.contains("fatal: (kpathsea)"), "{label}\n{stderr}");
            assert!(
                stderr.contains("invalid epoch-seconds-timezone value for environment variable $SOURCE_DATE_EPOCH:"),
                "{label}\n{stderr}"
            );
            assert!(
                !pdf.exists() && !plain.exists() && !gzip.exists(),
                "{label}"
            );
            continue;
        };

        assert_success(&output, label);
        let value_marker = format!("TEKAI-SYNCTEX={value}");
        let date_marker = format!("TEKAI-EPOCH={date}");
        assert!(
            stdout.lines().any(|line| line == value_marker),
            "{label}\n{stdout}"
        );
        assert!(
            stdout.lines().any(|line| line == date_marker),
            "{label}\n{stdout}"
        );
        assert!(fs::read(pdf).unwrap().starts_with(b"%PDF-"), "{label}");
        assert!(!stderr.contains("unrecognized option"), "{label}\n{stderr}");

        match sidecar {
            Sidecar::Gzip => {
                assert!(
                    !plain.exists(),
                    "{label} unexpectedly wrote a plain sidecar"
                );
                let bytes = fs::read(&gzip).unwrap();
                assert!(bytes.starts_with(&[0x1f, 0x8b, 8]), "{label}");
                assert!(bytes.len() > 20, "{label} wrote an empty gzip sidecar");
                let size = u32::from_le_bytes(bytes[bytes.len() - 4..].try_into().unwrap());
                assert!(size > 100, "{label} lacks a completed SyncTeX payload");
            }
            Sidecar::Plain => {
                assert!(!gzip.exists(), "{label} unexpectedly wrote a gzip sidecar");
                let text = fs::read_to_string(&plain).unwrap();
                assert!(text.starts_with("SyncTeX Version:1\n"), "{label}");
                assert!(
                    text.lines()
                        .any(|line| line.starts_with("Input:") && line.ends_with("main.tex")),
                    "{label}\n{text}"
                );
                assert!(
                    text.contains("Output:pdf\n") && text.contains("Content:\n"),
                    "{label}\n{text}"
                );
                assert!(
                    text.contains("Postamble:\nCount:") && text.ends_with("Post scriptum:\n"),
                    "{label}\n{text}"
                );
            }
            Sidecar::Absent => {
                assert!(
                    !plain.exists() && !gzip.exists(),
                    "{label} unexpectedly enabled SyncTeX"
                );
            }
        }
    }
}

fn run_synctex_build(fixture: &Project) -> std::process::Output {
    fixture
        .command(&fixture.0)
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

struct Project(PathBuf);

impl Project {
    fn new() -> Self {
        static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);
        for _ in 0..128 {
            let root = std::env::temp_dir().join(format!(
                "tekai-cli-synctex-{}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed),
            ));
            match fs::create_dir(&root) {
                Ok(()) => {
                    let project = Self(root);
                    fs::create_dir(project.0.join("empty-bin")).unwrap();
                    return project;
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("cannot reserve fixture {}: {error}", root.display()),
            }
        }
        panic!("cannot reserve SyncTeX fixture after 128 attempts")
    }

    fn command(&self, directory: &Path) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_tekai"));
        command
            .current_dir(directory)
            .env_clear()
            .env("PATH", self.0.join("empty-bin"))
            .env("TEKAI_TEXMF_MODE", "bundled")
            .env("TEKAI_ENGINE_CACHE", self.0.join("cache/engine"))
            .env("TEKAI_FORMAT_CACHE", self.0.join("cache/formats"))
            .env("TEKAI_AUX_CACHE", self.0.join("cache/aux"))
            .env("TEKAI_BIBTEX_CACHE", self.0.join("cache/bibtex"))
            .env("TEXMFDIST", self.0.join("missing-texlive"))
            .env("TEXMFVAR", self.0.join("missing-texlive"))
            .env("TEXMFCONFIG", self.0.join("missing-texlive"));
        command
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

use serde_json::Value;

struct Fixture(PathBuf);

impl Fixture {
    fn new() -> Self {
        let unique = format!(
            "tekai-cli-format-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let root = std::env::temp_dir().join(unique);
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }

    fn write(&self, name: &str, source: &str) {
        let path = self.0.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, source).unwrap();
    }

    fn read(&self, name: &str) -> String {
        fs::read_to_string(self.0.join(name)).unwrap()
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_tekai"))
            .current_dir(&self.0)
            .arg("format")
            .args(args)
            .output()
            .unwrap()
    }

    fn json(&self, args: &[&str], code: i32) -> Value {
        let output = self.run(args);
        assert_eq!(output.status.code(), Some(code), "{output:#?}");
        assert!(output.stderr.is_empty(), "{output:#?}");
        serde_json::from_slice(&output.stdout).expect("format stdout must contain only JSON")
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn format_reuses_safe_fixes_without_building_and_is_idempotent() {
    let fixture = Fixture::new();
    fixture.write(
        "paper.tex",
        "\\begin{itemize}\n\t\\item Inline café $x$\n\\end{itemize}\n$$y$$\n",
    );
    let report = fixture.json(&["paper.tex", "--report-json"], 0);
    assert_eq!(
        fixture.read("paper.tex"),
        "\\begin{itemize}\n  \\item Inline café \\(x\\)\n\\end{itemize}\n\\[y\\]\n"
    );
    assert_eq!(report["check"], false);
    assert_eq!(report["fixes_applied"], 5);
    assert_eq!(report["files_changed"], serde_json::json!(["paper.tex"]));
    assert_eq!(report["fixes_available"], 0);
    assert_eq!(report["files_would_change"], serde_json::json!([]));
    assert_eq!(report["diagnostics"], serde_json::json!([]));
    assert!(report.get("pdf_path").is_none());
    assert!(!fixture.0.join("build").exists());

    let modified = fs::metadata(fixture.0.join("paper.tex"))
        .unwrap()
        .modified()
        .unwrap();
    let again = fixture.json(&["paper.tex", "--report-json"], 0);
    assert_eq!(again["fixes_applied"], 0);
    assert_eq!(again["files_changed"], serde_json::json!([]));
    assert_eq!(
        fs::metadata(fixture.0.join("paper.tex"))
            .unwrap()
            .modified()
            .unwrap(),
        modified
    );
}

#[test]
fn format_runs_without_external_tools_or_tex_installation() {
    let fixture = Fixture::new();
    fixture.write("paper.tex", "$x$\n");
    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .current_dir(&fixture.0)
        .env_clear()
        .env("PATH", "")
        .args(["format", "paper.tex", "--report-json"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:#?}");
    assert!(output.stderr.is_empty());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["fixes_applied"], 2);
    assert_eq!(fixture.read("paper.tex"), "\\(x\\)\n");
}

#[test]
fn format_explicit_file_in_ignored_directory_is_still_processed() {
    let fixture = Fixture::new();
    fixture.write("build/source.tex", "$x$\n");
    fixture.json(&["build/source.tex", "--report-json"], 0);
    assert_eq!(fixture.read("build/source.tex"), "\\(x\\)\n");
}

#[test]
fn format_defaults_to_current_directory_and_its_config() {
    let fixture = Fixture::new();
    fixture.write(
        "tekai.toml",
        "[lint]\nindent_size = 2\nindent_style = \"tabs\"\n[lint.rules]\n\"math/inline-dollar\" = \"off\"\n",
    );
    fixture.write(
        "sections/chapter.tex",
        "\\begin{itemize}\n  \\item $x$.\n\\end{itemize}\n",
    );
    let report = fixture.json(&["--report-json"], 0);
    assert_eq!(report["fixes_applied"], 1);
    assert_eq!(
        fixture.read("sections/chapter.tex"),
        "\\begin{itemize}\n\t\\item $x$.\n\\end{itemize}\n"
    );
}

#[test]
fn format_explicit_config_overrides_current_directory_config() {
    let fixture = Fixture::new();
    fixture.write("tekai.toml", "[lint]\nindent_size = 4\n");
    fixture.write(
        "config/tabs.toml",
        "[lint]\nindent_size = 2\nindent_style = \"tabs\"\n",
    );
    fixture.write("paper.tex", "\\begin{proof}\n  Claim.\n\\end{proof}\n");
    fixture.json(
        &["paper.tex", "--config", "config/tabs.toml", "--report-json"],
        0,
    );
    assert_eq!(
        fixture.read("paper.tex"),
        "\\begin{proof}\n\tClaim.\n\\end{proof}\n"
    );
}

#[test]
fn format_scans_supported_extensions_and_skips_generated_and_style_files() {
    let fixture = Fixture::new();
    for name in [
        "a.TEX",
        "nested/b.ltx",
        "c.cls",
        "package.sty",
        "notes.txt",
        "build/generated.tex",
        "target/generated.tex",
        ".git/hidden.tex",
        ".latexmk/generated.tex",
        ".tekai/generated.tex",
    ] {
        fixture.write(name, "$x$\n");
    }
    let preview = fixture.json(&[".", "a.TEX", "--check", "--report-json"], 1);
    assert_eq!(preview["fixes_available"], 6);
    assert_eq!(preview["files_would_change"].as_array().unwrap().len(), 3);
    let report = fixture.json(&[".", "a.TEX", "--report-json"], 0);
    assert_eq!(report["fixes_applied"], 6);
    assert_eq!(report["files_changed"].as_array().unwrap().len(), 3);
    for name in ["a.TEX", "nested/b.ltx", "c.cls"] {
        assert_eq!(fixture.read(name), "\\(x\\)\n");
    }
    for name in [
        "package.sty",
        "notes.txt",
        "build/generated.tex",
        "target/generated.tex",
        ".git/hidden.tex",
        ".latexmk/generated.tex",
        ".tekai/generated.tex",
    ] {
        assert_eq!(fixture.read(name), "$x$\n");
    }
}

#[test]
fn format_check_never_writes_and_allow_warnings_does_not_hide_required_changes() {
    let fixture = Fixture::new();
    fixture.write("paper.tex", "$x$\n");
    let modified = fs::metadata(fixture.0.join("paper.tex"))
        .unwrap()
        .modified()
        .unwrap();
    let report = fixture.json(
        &["paper.tex", "--check", "--report-json", "--allow-warnings"],
        1,
    );
    assert_eq!(report["check"], true);
    assert_eq!(report["fixes_applied"], 0);
    assert_eq!(report["files_changed"], serde_json::json!([]));
    assert_eq!(report["fixes_available"], 2);
    assert_eq!(
        report["files_would_change"],
        serde_json::json!(["paper.tex"])
    );
    assert_eq!(report["warning_count"], 2);
    assert_eq!(fixture.read("paper.tex"), "$x$\n");
    assert_eq!(
        fs::metadata(fixture.0.join("paper.tex"))
            .unwrap()
            .modified()
            .unwrap(),
        modified
    );

    let text = fixture.run(&["paper.tex", "--check"]);
    assert_eq!(text.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&text.stderr).contains("would format paper.tex"));
    fixture.json(&["paper.tex", "--report-json"], 0);
    let clean = fixture.json(&["paper.tex", "--check", "--report-json"], 0);
    assert_eq!(clean["fixes_available"], 0);
}

#[test]
fn format_reports_unfixable_warnings_and_uses_lint_exit_policy() {
    for (flags, code) in [
        (vec![], 1),
        (vec!["--allow-warnings"], 0),
        (vec!["--fail-on-warnings"], 1),
    ] {
        let fixture = Fixture::new();
        fixture.write("tekai.toml", "[lint]\nmax_line_length = 12\n");
        fixture.write("paper.tex", "$x$\nA deliberately long prose line.\n");
        let mut args = vec!["paper.tex", "--report-json"];
        args.extend(flags);
        let report = fixture.json(&args, code);
        assert_eq!(report["fixes_applied"], 2);
        assert_eq!(report["warning_count"], 1);
        assert_eq!(report["diagnostics"][0]["rule"], "line/length");
        assert_eq!(
            fixture.read("paper.tex"),
            "\\(x\\)\nA deliberately long prose line.\n"
        );
        // Check mode keeps the same lint policy even when no safe edits remain.
        args.push("--check");
        assert_eq!(fixture.json(&args, code)["fixes_available"], 0);
    }
}

#[test]
fn format_applies_safe_fixes_but_remaining_errors_always_fail() {
    let fixture = Fixture::new();
    fixture.write("paper.tex", "$x$\n\\begin{proof}\n");
    let report = fixture.json(&["paper.tex", "--report-json", "--allow-warnings"], 1);
    assert_eq!(report["fixes_applied"], 2);
    assert_eq!(report["error_count"], 1);
    assert_eq!(report["diagnostics"][0]["rule"], "env/unclosed");
    assert_eq!(fixture.read("paper.tex"), "\\(x\\)\n\\begin{proof}\n");
}

#[test]
fn format_preserves_suppressions_verbatim_and_ambiguous_math() {
    let fixture = Fixture::new();
    let suppressed = "Legacy $x$. % tekai-ignore-line math/inline-dollar\n";
    let verbatim = "\\begin{verbatim}\n\t$x$\n\\end{verbatim}\n";
    let ambiguous = "Nested \\($x$\\).\n";
    fixture.write("suppressed.tex", suppressed);
    fixture.write("verbatim.tex", verbatim);
    fixture.write("ambiguous.tex", ambiguous);
    let report = fixture.json(&["--report-json", "--allow-warnings"], 1);
    assert_eq!(report["fixes_applied"], 0);
    assert_eq!(fixture.read("suppressed.tex"), suppressed);
    assert_eq!(fixture.read("verbatim.tex"), verbatim);
    assert_eq!(fixture.read("ambiguous.tex"), ambiguous);
}

#[test]
fn format_file_targets_do_not_follow_inputs_or_touch_siblings() {
    let fixture = Fixture::new();
    fixture.write("main.tex", "\\input{chapter}\n$x$\n");
    fixture.write("chapter.tex", "$y$\n");
    fixture.write("unrelated.tex", "$z$\n");
    fixture.json(&["main.tex", "--report-json"], 0);
    assert_eq!(fixture.read("main.tex"), "\\input{chapter}\n\\(x\\)\n");
    assert_eq!(fixture.read("chapter.tex"), "$y$\n");
    assert_eq!(fixture.read("unrelated.tex"), "$z$\n");
}

#[test]
fn format_preserves_unicode_line_endings_and_missing_final_newline() {
    let fixture = Fixture::new();
    fixture.write("paper.tex", "Café $x$.\r\nFin $y$.");
    fixture.json(&["paper.tex", "--report-json"], 0);
    assert_eq!(fixture.read("paper.tex"), "Café \\(x\\).\r\nFin \\(y\\).");
}

#[test]
fn format_quiet_suppresses_text_but_not_json_or_failure_status() {
    let fixture = Fixture::new();
    fixture.write("paper.tex", "$x$\n\\begin{proof}\n");
    let output = fixture.run(&["paper.tex", "--quiet"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(output.stderr.is_empty());
    assert_eq!(
        fixture.json(&["paper.tex", "--quiet", "--report-json"], 1)["error_count"],
        1
    );
}

#[test]
fn format_invalid_config_or_missing_target_fails_before_writing() {
    let fixture = Fixture::new();
    fixture.write("paper.tex", "$x$\n");
    fixture.write("invalid.toml", "[lint\n");
    for args in [
        vec!["paper.tex", "--config", "invalid.toml"],
        vec!["paper.tex", "missing.tex"],
    ] {
        let output = fixture.run(&args);
        assert_eq!(output.status.code(), Some(1), "{output:#?}");
        assert!(!output.stderr.is_empty());
        assert_eq!(fixture.read("paper.tex"), "$x$\n");
    }
}

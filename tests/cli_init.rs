use std::fs;
use std::path::PathBuf;
use std::process::Command;

use tekai::compiler::{BibMode, DraftPrepass, Engine, Runner};
use tekai::config::{DEFAULT_CONFIG, load_project_config};
use tekai::lint::IndentStyle;

#[test]
fn init_writes_a_complete_loadable_default_config() {
    let root = unique_temp_dir("tekai-cli-init");
    fs::create_dir_all(&root).expect("failed to create temp directory");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .arg("init")
        .current_dir(&root)
        .output()
        .expect("failed to run tekai init");

    assert!(output.status.success(), "{output:#?}");
    let path = root.join("tekai.toml");
    assert_eq!(
        fs::read_to_string(&path).expect("failed to read initialized config"),
        DEFAULT_CONFIG
    );

    let config = load_project_config(Some(&path)).expect("initialized config should load");
    assert_eq!(config.build.engine, Some(Engine::PdfLatex));
    assert_eq!(config.build.runner, Some(Runner::Direct));
    assert_eq!(config.build.bib_mode, Some(BibMode::Auto));
    assert_eq!(
        config.build.out_dir.as_deref(),
        Some(std::path::Path::new("build"))
    );
    assert_eq!(config.build.job_name, None);
    assert_eq!(config.build.fast, Some(false));
    assert_eq!(config.build.draft_prepass, Some(DraftPrepass::Auto));
    assert_eq!(config.build.once, Some(false));
    assert_eq!(config.build.max_runs, Some(8));
    assert_eq!(config.build.force, Some(false));
    assert_eq!(config.build.precompile_preamble, Some(false));
    assert_eq!(config.build.synctex, Some(false));
    assert_eq!(config.build.shell_escape, Some(false));
    assert_eq!(config.build.print_command, Some(false));
    assert_eq!(config.build.quiet, Some(false));
    assert!(config.build.env.is_empty());
    assert_eq!(config.lint.indent_size, 2);
    assert_eq!(config.lint.indent_style, IndentStyle::Spaces);
    assert!(config.lint.indent_environments);
    assert!(config.lint.indent_display_math);
    assert_eq!(config.lint.ignored_indent_environments, ["document"]);
    assert!(config.lint.prefer_paren_inline_math);
    assert!(config.lint.prefer_bracket_display_math);
    assert!(!config.lint.prefer_prime_command);
    assert!(config.lint.check_environment_stack);
    assert_eq!(config.lint.max_line_length, Some(120));
    assert_eq!(config.lint.prose_wrap, None);
    assert!(config.lint.rule_levels.is_empty());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn init_preserves_an_existing_config_unless_forced() {
    let root = unique_temp_dir("tekai-cli-init-force");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let path = root.join("custom.toml");
    fs::write(&path, "keep me\n").expect("failed to write existing config");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .args(["init", "custom.toml"])
        .current_dir(&root)
        .output()
        .expect("failed to run tekai init");
    assert_eq!(output.status.code(), Some(1), "{output:#?}");
    assert_eq!(fs::read_to_string(&path).unwrap(), "keep me\n");

    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .args(["init", "custom.toml", "--force"])
        .current_dir(&root)
        .output()
        .expect("failed to run tekai init --force");
    assert!(output.status.success(), "{output:#?}");
    assert_eq!(fs::read_to_string(&path).unwrap(), DEFAULT_CONFIG);

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

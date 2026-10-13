use std::process::Command;

use clap::CommandFactory;
use tekai::cli::Cli;

#[test]
fn sole_version_flags_match_canonical_parser_without_profile_records() {
    for flag in ["--version", "-V"] {
        for opt_in in [false, true] {
            assert_parser_output(&[flag], None, opt_in);
            #[cfg(unix)]
            assert_parser_output(&[flag], Some("renamed-tekai"), opt_in);
        }
    }
}

#[test]
fn combined_and_invalid_arguments_retain_canonical_parser_behavior() {
    for args in [
        &["--version", "--help"][..],
        &["--help", "--version"],
        &["--version", "extra"],
        &["extra", "--version"],
        &["--version", "--unknown"],
        &["--unknown", "--version"],
        &["--version", "--version"],
        &["--", "--version"],
        &["--version=extra"],
        &["-Vh"],
        &["build", "--version"],
        &["build", "-V"],
    ] {
        assert_parser_output(args, None, false);
        #[cfg(unix)]
        assert_parser_output(args, Some("renamed-tekai"), false);
    }
}

fn assert_parser_output(args: &[&str], argv0: Option<&str>, opt_in: bool) {
    let executable = env!("CARGO_BIN_EXE_tekai");
    let expected = Cli::command()
        .color(clap::ColorChoice::Never)
        .try_get_matches_from(
            std::iter::once(argv0.unwrap_or(executable)).chain(args.iter().copied()),
        )
        .expect_err("these argument sequences must exit inside Clap");
    let message = expected.to_string().into_bytes();
    let mut command = Command::new(executable);
    command
        .args(args)
        .env("NO_COLOR", "1")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("TEKAI_DIAGNOSTIC_PROFILE");
    if opt_in {
        command.env("TEKAI_DIAGNOSTIC_PROFILE", "1");
    }
    #[cfg(unix)]
    if let Some(argv0) = argv0 {
        use std::os::unix::process::CommandExt;
        command.arg0(argv0);
    }
    let output = command
        .output()
        .expect("CLI parser comparison did not start");
    let context = format!("arguments {args:?}, argv0 {argv0:?}, profile {opt_in}");
    assert_eq!(
        output.status.code(),
        Some(expected.exit_code()),
        "{context}"
    );
    if expected.use_stderr() {
        assert!(output.stdout.is_empty(), "{context}");
        assert_eq!(output.stderr, message, "{context}");
    } else {
        assert_eq!(output.stdout, message, "{context}");
        assert!(output.stderr.is_empty(), "{context}");
    }
}

#[test]
fn every_command_has_long_help_and_examples() {
    let command_names = Cli::command()
        .get_subcommands()
        .map(|command| command.get_name().to_string())
        .filter(|command| command != "help")
        .collect::<Vec<_>>();
    assert!(!command_names.is_empty(), "tekai has no subcommands");

    for command in command_names {
        assert_help(&command, &[&command, "--help"]);
        assert_help(&command, &["help", &command]);
    }
}

fn assert_help(command: &str, args: &[&str]) {
    let output = Command::new(env!("CARGO_BIN_EXE_tekai"))
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("failed to run help for {command}: {error}"));
    assert!(
        output.status.success(),
        "help for {command} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(&format!("Usage: tekai {command}")),
        "help for {command} has no command-specific usage:\n{stdout}"
    );
    assert!(
        stdout.contains("EXAMPLES:"),
        "help for {command} has no examples:\n{stdout}"
    );
}

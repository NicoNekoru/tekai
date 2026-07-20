use std::process::Command;

use clap::CommandFactory;
use tekai::cli::Cli;

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

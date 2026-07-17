use std::ffi::{CString, OsStr};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use clap::parser::ValueSource;
use clap::{ArgMatches, CommandFactory, FromArgMatches};
use serde::Serialize;
use tekai::cli::{BuildArgs, CheckArgs, CleanArgs, Cli, Command, LintArgs, WatchArgs};
use tekai::compiler::{
    BuildOptions, EMBEDDED_ENGINE_RUNNER_ENV, EMBEDDED_ENGINE_SUBCOMMAND, build,
};
use tekai::config::{BuildConfig, load_build_config, load_lint_config, load_project_config};
use tekai::lint::{Diagnostic, Severity, format_diagnostic, has_errors, lint_paths};
use tekai::watch::{WatchOptions, watch};

fn main() -> Result<()> {
    if std::env::args_os().nth(1).as_deref() == Some(OsStr::new(EMBEDDED_ENGINE_SUBCOMMAND)) {
        return run_embedded_engine();
    }
    enable_embedded_engine_runner()?;

    let matches = Cli::command().get_matches();
    let build_flag_sources = BuildFlagSources::from_root_matches(&matches);
    let cli = Cli::from_arg_matches(&matches)?;
    match cli.command {
        Command::Build(args) => run_build(args, build_flag_sources),
        Command::Clean(args) => run_clean(args),
        Command::Lint(args) => run_lint(args),
        Command::Check(args) => run_check(args, build_flag_sources),
        Command::Watch(args) => run_watch(args, build_flag_sources),
    }
}

fn enable_embedded_engine_runner() -> Result<()> {
    if std::env::var_os(EMBEDDED_ENGINE_RUNNER_ENV).is_none() {
        let executable = std::env::current_exe().context("failed to locate tekai executable")?;
        // SAFETY: this is initialized before watcher threads or child processes start.
        unsafe {
            std::env::set_var(EMBEDDED_ENGINE_RUNNER_ENV, executable);
        }
    }
    Ok(())
}

fn run_embedded_engine() -> Result<()> {
    let mut args = Vec::new();
    args.push(CString::new("pdflatex").expect("static pdfTeX program name contains no NUL"));
    for arg in std::env::args_os().skip(2) {
        args.push(CString::new(os_arg_bytes(&arg)).context("pdfTeX argument contained NUL")?);
    }
    let mut argv = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut core::ffi::c_char)
        .collect::<Vec<_>>();
    argv.push(std::ptr::null_mut());

    let code =
        unsafe { tekai_engine::run_from_c_args(args.len() as core::ffi::c_int, argv.as_mut_ptr()) };
    std::process::exit(code)
}

#[cfg(unix)]
fn os_arg_bytes(arg: &OsStr) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    arg.as_bytes().to_vec()
}

#[cfg(not(unix))]
fn os_arg_bytes(arg: &OsStr) -> Vec<u8> {
    arg.to_string_lossy().into_owned().into_bytes()
}

fn run_clean(args: CleanArgs) -> Result<()> {
    let build_config = load_build_config(args.config.as_deref())?;
    let out_dir = args
        .out_dir
        .or(build_config.out_dir)
        .unwrap_or_else(|| PathBuf::from("build"));
    let target = clean_target_path(&out_dir)?;
    let existed = target.exists();

    if existed && !args.dry_run {
        let metadata = fs::symlink_metadata(&target)
            .with_context(|| format!("failed to stat clean target {}", target.display()))?;
        if metadata.file_type().is_symlink() {
            bail!("refusing to clean symlink {}", target.display());
        }
        if !metadata.is_dir() {
            bail!("refusing to clean non-directory {}", target.display());
        }
        fs::remove_dir_all(&target)
            .with_context(|| format!("failed to remove {}", target.display()))?;
    }

    let report = CleanReport {
        path: target.display().to_string(),
        existed,
        removed: existed && !args.dry_run,
        dry_run: args.dry_run,
    };
    if args.report_json {
        print_clean_report_json(&report)?;
    } else if !args.quiet {
        print_clean_report(&report);
    }
    Ok(())
}

fn clean_target_path(out_dir: &Path) -> Result<PathBuf> {
    if out_dir.as_os_str().is_empty() {
        bail!("refusing to clean an empty output directory path");
    }

    let cwd = std::env::current_dir()
        .context("failed to read current directory")?
        .canonicalize()
        .context("failed to canonicalize current directory")?;
    let candidate = if out_dir.is_absolute() {
        out_dir.to_path_buf()
    } else {
        cwd.join(out_dir)
    };
    let target = normalize_path_lexically(&candidate);
    if target == cwd || cwd.starts_with(&target) {
        bail!(
            "refusing to clean {} because it is the current directory or an ancestor",
            target.display()
        );
    }
    Ok(target)
}

fn normalize_path_lexically(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(_) | Component::RootDir | Component::Prefix(_) => {
                normalized.push(component.as_os_str());
            }
        }
    }
    normalized
}

#[derive(Serialize)]
struct CleanReport {
    path: String,
    existed: bool,
    removed: bool,
    dry_run: bool,
}

fn print_clean_report_json(report: &CleanReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}

fn print_clean_report(report: &CleanReport) {
    if report.dry_run {
        if report.existed {
            eprintln!("would remove {}", report.path);
        } else {
            eprintln!("nothing to clean at {}", report.path);
        }
    } else if report.removed {
        eprintln!("removed {}", report.path);
    } else {
        eprintln!("nothing to clean at {}", report.path);
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct BuildFlagSources {
    engine: bool,
    bib: bool,
    runner: bool,
    out_dir: bool,
    job_name: bool,
    fast: bool,
    no_images: bool,
    draft_prepass: bool,
    once: bool,
    max_runs: bool,
    force: bool,
    precompile_preamble: bool,
    synctex: bool,
    shell_escape: bool,
    quiet: bool,
    print_command: bool,
}

impl BuildFlagSources {
    fn from_root_matches(matches: &ArgMatches) -> Self {
        let Some((name, subcommand)) = matches.subcommand() else {
            return Self::default();
        };
        if !matches!(name, "build" | "check" | "watch") {
            return Self::default();
        }
        Self {
            engine: is_command_line_arg(subcommand, "engine"),
            bib: is_command_line_arg(subcommand, "bib"),
            runner: is_command_line_arg(subcommand, "runner"),
            out_dir: is_command_line_arg(subcommand, "out_dir"),
            job_name: is_command_line_arg(subcommand, "job_name"),
            fast: is_command_line_arg(subcommand, "fast"),
            no_images: is_command_line_arg(subcommand, "no_images"),
            draft_prepass: is_command_line_arg(subcommand, "draft_prepass"),
            once: is_command_line_arg(subcommand, "once"),
            max_runs: is_command_line_arg(subcommand, "max_runs"),
            force: is_command_line_arg(subcommand, "force"),
            precompile_preamble: is_command_line_arg(subcommand, "precompile_preamble"),
            synctex: is_command_line_arg(subcommand, "synctex"),
            shell_escape: is_command_line_arg(subcommand, "shell_escape"),
            quiet: is_command_line_arg(subcommand, "quiet"),
            print_command: is_command_line_arg(subcommand, "print_command"),
        }
    }
}

fn is_command_line_arg(matches: &ArgMatches, id: &str) -> bool {
    matches.value_source(id) == Some(ValueSource::CommandLine)
}

fn run_build(args: BuildArgs, flag_sources: BuildFlagSources) -> Result<()> {
    let report_json = args.report_json;
    let build_config = load_build_config(args.config.as_deref())?;
    apply_build_env(&build_config);
    let mut options = build_options(args.main, args.flags, &build_config, flag_sources);
    if report_json {
        options.quiet = true;
        options.print_command = false;
    }
    let report = build(&options)?;
    if report_json {
        print_build_report_json(&report)?;
    } else if let Some(ref pdf) = report.pdf_path {
        print_build_report(pdf, &report);
    } else {
        eprintln!("build finished in {:.2?}", report.elapsed);
    }
    Ok(())
}

fn run_lint(args: LintArgs) -> Result<()> {
    let config = load_lint_config(args.config.as_deref())?;
    let diagnostics = lint_paths(&args.paths, &config)?;
    if args.report_json {
        print_lint_report_json(&diagnostics)?;
    } else {
        print_lint_diagnostics(&diagnostics, false);
    }
    if has_errors(&diagnostics) || (args.flags.should_fail_on_warnings() && !diagnostics.is_empty())
    {
        std::process::exit(1);
    }
    Ok(())
}

fn run_check(args: CheckArgs, flag_sources: BuildFlagSources) -> Result<()> {
    let report_json = args.report_json;
    let config = load_project_config(args.config.as_deref())?;
    let lint_target = vec![document_root(&args.main)];
    let diagnostics = lint_paths(&lint_target, &config.lint)?;
    print_lint_diagnostics(&diagnostics, report_json);
    if has_errors(&diagnostics)
        || (args.lint_flags.should_fail_on_warnings() && !diagnostics.is_empty())
    {
        std::process::exit(1);
    }
    apply_build_env(&config.build);
    let mut options = build_options(args.main, args.flags, &config.build, flag_sources);
    if report_json {
        options.quiet = true;
        options.print_command = false;
    }
    let report = build(&options)?;
    if report_json {
        print_build_report_json(&report)?;
    } else if let Some(ref pdf) = report.pdf_path {
        print_build_report(pdf, &report);
    }
    Ok(())
}

fn print_lint_diagnostics(diagnostics: &[tekai::lint::Diagnostic], stderr: bool) {
    for diagnostic in diagnostics {
        if stderr {
            eprintln!("{}", format_diagnostic(diagnostic));
        } else {
            println!("{}", format_diagnostic(diagnostic));
        }
    }
}

fn run_watch(args: WatchArgs, flag_sources: BuildFlagSources) -> Result<()> {
    let config = load_project_config(args.config.as_deref())?;
    apply_build_env(&config.build);
    let root = args
        .root
        .clone()
        .unwrap_or_else(|| document_root(&args.main));
    let mut build_options =
        build_options(args.main.clone(), args.flags, &config.build, flag_sources);
    let final_build_options = args.final_after_idle_ms.map(|_| build_options.clone());
    if args.preview {
        build_options.once = true;
        build_options.fast = true;
        build_options.precompile_preamble = true;
    }
    watch(WatchOptions {
        main: args.main,
        root,
        build_options,
        final_build_options,
        final_after_idle: args.final_after_idle_ms.map(Duration::from_millis),
        lint_config: config.lint,
        lint: !args.no_lint,
        fail_on_warnings: args.lint_flags.should_fail_on_warnings(),
    })
}

fn apply_build_env(config: &BuildConfig) {
    for (key, value) in &config.env {
        // SAFETY: tekai applies project build environment before starting
        // watcher threads or launching child TeX/tool processes.
        unsafe {
            std::env::set_var(key, value);
        }
    }
}

fn build_options(
    main: PathBuf,
    flags: tekai::cli::BuildFlags,
    config: &BuildConfig,
    sources: BuildFlagSources,
) -> BuildOptions {
    let mut options = BuildOptions::for_main(main);
    apply_build_config(&mut options, config);
    apply_build_flags(&mut options, flags, sources);
    options
}

fn apply_build_config(options: &mut BuildOptions, config: &BuildConfig) {
    if let Some(value) = config.engine {
        options.engine = value;
    }
    if let Some(value) = config.runner {
        options.runner = value;
    }
    if let Some(value) = config.bib_mode {
        options.bib_mode = value;
    }
    if let Some(value) = &config.out_dir {
        options.out_dir = value.clone();
    }
    if let Some(value) = &config.job_name {
        options.job_name = Some(value.clone());
    }
    if let Some(value) = config.fast {
        options.fast = value;
    }
    if let Some(value) = config.draft_prepass {
        options.draft_prepass = value;
    }
    if let Some(value) = config.once {
        options.once = value;
    }
    if let Some(value) = config.max_runs {
        options.max_runs = value;
    }
    if let Some(value) = config.force {
        options.force = value;
    }
    if let Some(value) = config.precompile_preamble {
        options.precompile_preamble = value;
    }
    if let Some(value) = config.synctex {
        options.synctex = value;
    }
    if let Some(value) = config.shell_escape {
        options.shell_escape = value;
    }
    if let Some(value) = config.quiet {
        options.quiet = value;
    }
    if let Some(value) = config.print_command {
        options.print_command = value;
    }
}

fn apply_build_flags(
    options: &mut BuildOptions,
    flags: tekai::cli::BuildFlags,
    sources: BuildFlagSources,
) {
    if sources.engine {
        options.engine = flags.engine.into();
    }
    if sources.runner {
        options.runner = flags.runner.into();
    }
    if sources.bib {
        options.bib_mode = flags.bib.into();
    }
    if sources.out_dir {
        options.out_dir = flags.out_dir;
    }
    if sources.job_name {
        options.job_name = flags.job_name;
    }
    if sources.fast || sources.no_images {
        options.fast = flags.fast || flags.no_images;
    }
    if sources.draft_prepass {
        options.draft_prepass = flags.draft_prepass.into();
    }
    if sources.once {
        options.once = flags.once;
    }
    if sources.max_runs {
        options.max_runs = flags.max_runs;
    }
    if sources.force {
        options.force = flags.force;
    }
    if sources.precompile_preamble {
        options.precompile_preamble = flags.precompile_preamble;
    }
    if sources.synctex {
        options.synctex = flags.synctex;
    }
    if sources.shell_escape {
        options.shell_escape = flags.shell_escape;
    }
    if sources.quiet {
        options.quiet = flags.quiet;
    }
    if sources.print_command {
        options.print_command = flags.print_command;
    }
}

fn print_build_report(pdf: &std::path::Path, report: &tekai::compiler::BuildReport) {
    if report.skipped {
        eprintln!(
            "cached {} in {:.2?} (inputs unchanged)",
            pdf.display(),
            report.elapsed
        );
        return;
    }
    if report.tex_runs == 0 {
        eprintln!("built {} in {:.2?}", pdf.display(), report.elapsed);
    } else {
        eprintln!(
            "built {} in {:.2?} ({} TeX run{}{}, {} bibliography run{}, {} index run{}, {} external run{}{})",
            pdf.display(),
            report.elapsed,
            report.tex_runs,
            if report.tex_runs == 1 { "" } else { "s" },
            tex_run_detail(report),
            report.bibliography_runs,
            if report.bibliography_runs == 1 {
                ""
            } else {
                "s"
            },
            report.index_runs,
            if report.index_runs == 1 { "" } else { "s" },
            report.external_runs,
            if report.external_runs == 1 { "" } else { "s" },
            build_report_detail(report)
        );
    }
}

fn tex_run_detail(report: &tekai::compiler::BuildReport) -> String {
    if report.draft_tex_runs == 0 && report.final_tex_runs == 0 {
        String::new()
    } else {
        format!(
            ": {} draft, {} final-layout, {} pdf",
            report.draft_tex_runs, report.final_tex_runs, report.pdf_tex_runs
        )
    }
}

fn build_report_detail(report: &tekai::compiler::BuildReport) -> String {
    let mut details = Vec::new();
    if report.draft_prepass_used {
        details.push("draft prepass");
    }
    if report.aux_preflight_used {
        details.push("aux preflight");
    }
    if report.preamble_format_used {
        if report.preamble_format_built {
            details.push("preamble format built");
        } else {
            details.push("preamble format cached");
        }
    }
    if details.is_empty() {
        String::new()
    } else {
        format!("; {}", details.join(", "))
    }
}

#[derive(Serialize)]
struct JsonBuildReport {
    pdf_path: Option<String>,
    elapsed_ms: f64,
    tex_runs: usize,
    draft_tex_runs: usize,
    final_tex_runs: usize,
    pdf_tex_runs: usize,
    passes: Vec<JsonTexPassReport>,
    bibliography_runs: usize,
    index_runs: usize,
    external_runs: usize,
    skipped: bool,
    draft_prepass_used: bool,
    aux_preflight_used: bool,
    preamble_format_used: bool,
    preamble_format_built: bool,
}

#[derive(Serialize)]
struct JsonTexPassReport {
    draft: bool,
    pdf_output: bool,
    elapsed_ms: f64,
    tex_elapsed_ms: f64,
    aux_elapsed_ms: f64,
    rerun_reasons: Vec<String>,
    aux_outputs_changed: bool,
    generated_outputs_changed: bool,
    generated_inputs_unread: bool,
    preamble_format_used: bool,
    preamble_format_built: bool,
    bibliography_runs: usize,
    index_runs: usize,
    external_runs: usize,
}

impl From<&tekai::compiler::BuildReport> for JsonBuildReport {
    fn from(report: &tekai::compiler::BuildReport) -> Self {
        Self {
            pdf_path: report
                .pdf_path
                .as_ref()
                .map(|path| path.display().to_string()),
            elapsed_ms: report.elapsed.as_secs_f64() * 1000.0,
            tex_runs: report.tex_runs,
            draft_tex_runs: report.draft_tex_runs,
            final_tex_runs: report.final_tex_runs,
            pdf_tex_runs: report.pdf_tex_runs,
            passes: report
                .passes
                .iter()
                .map(|pass| JsonTexPassReport {
                    draft: pass.draft,
                    pdf_output: pass.pdf_output,
                    elapsed_ms: pass.elapsed.as_secs_f64() * 1000.0,
                    tex_elapsed_ms: pass.tex_elapsed.as_secs_f64() * 1000.0,
                    aux_elapsed_ms: pass.aux_elapsed.as_secs_f64() * 1000.0,
                    rerun_reasons: pass.rerun_reasons.clone(),
                    aux_outputs_changed: pass.aux_outputs_changed,
                    generated_outputs_changed: pass.generated_outputs_changed,
                    generated_inputs_unread: pass.generated_inputs_unread,
                    preamble_format_used: pass.preamble_format_used,
                    preamble_format_built: pass.preamble_format_built,
                    bibliography_runs: pass.bibliography_runs,
                    index_runs: pass.index_runs,
                    external_runs: pass.external_runs,
                })
                .collect(),
            bibliography_runs: report.bibliography_runs,
            index_runs: report.index_runs,
            external_runs: report.external_runs,
            skipped: report.skipped,
            draft_prepass_used: report.draft_prepass_used,
            aux_preflight_used: report.aux_preflight_used,
            preamble_format_used: report.preamble_format_used,
            preamble_format_built: report.preamble_format_built,
        }
    }
}

fn print_build_report_json(report: &tekai::compiler::BuildReport) -> Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&JsonBuildReport::from(report))?
    );
    Ok(())
}

#[derive(Serialize)]
struct JsonLintReport {
    diagnostics: Vec<JsonDiagnostic>,
    error_count: usize,
    warning_count: usize,
}

#[derive(Serialize)]
struct JsonDiagnostic {
    path: String,
    line: usize,
    column: usize,
    severity: &'static str,
    rule: &'static str,
    message: String,
    help: Option<String>,
}

impl From<&Diagnostic> for JsonDiagnostic {
    fn from(diagnostic: &Diagnostic) -> Self {
        Self {
            path: diagnostic.path.display().to_string(),
            line: diagnostic.line,
            column: diagnostic.column,
            severity: match diagnostic.severity {
                Severity::Warning => "warning",
                Severity::Error => "error",
            },
            rule: diagnostic.rule,
            message: diagnostic.message.clone(),
            help: diagnostic.help.clone(),
        }
    }
}

impl From<&[Diagnostic]> for JsonLintReport {
    fn from(diagnostics: &[Diagnostic]) -> Self {
        Self {
            diagnostics: diagnostics.iter().map(JsonDiagnostic::from).collect(),
            error_count: diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.severity == Severity::Error)
                .count(),
            warning_count: diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.severity == Severity::Warning)
                .count(),
        }
    }
}

fn print_lint_report_json(diagnostics: &[Diagnostic]) -> Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&JsonLintReport::from(diagnostics))?
    );
    Ok(())
}

fn document_root(main: &std::path::Path) -> PathBuf {
    main.parent()
        .map(std::path::Path::to_path_buf)
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| PathBuf::from("."))
}

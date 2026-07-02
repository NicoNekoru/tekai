use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::compiler::{BibMode, DraftPrepass, Engine, Runner};

#[derive(Debug, Parser)]
#[command(name = "texpilot")]
#[command(about = "Fast LaTeX build orchestration and opinionated TeX linting")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Compile a root TeX document with the direct runner by default.
    Build(BuildArgs),
    /// Remove the configured build output directory.
    Clean(CleanArgs),
    /// Lint TeX sources for math delimiter, indentation, and style issues.
    Lint(LintArgs),
    /// Run lint first, then compile if lint passes.
    Check(CheckArgs),
    /// Watch a project and rebuild after TeX-related file changes.
    Watch(WatchArgs),
}

#[derive(Debug, Args, Clone)]
pub struct CleanArgs {
    /// Optional texpilot.toml path.
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Output directory to remove. Defaults to [build].out_dir or build.
    #[arg(long)]
    pub out_dir: Option<PathBuf>,

    /// Show what would be removed without deleting files.
    #[arg(long)]
    pub dry_run: bool,

    /// Emit a machine-readable JSON clean report to stdout.
    #[arg(long)]
    pub report_json: bool,

    /// Suppress text output.
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Debug, Args, Clone)]
pub struct BuildArgs {
    /// Root .tex file.
    pub main: PathBuf,

    /// Optional texpilot.toml path.
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Emit a machine-readable JSON build report to stdout.
    #[arg(long)]
    pub report_json: bool,

    #[command(flatten)]
    pub flags: BuildFlags,
}

#[derive(Debug, Args, Clone)]
pub struct CheckArgs {
    /// Root .tex file.
    pub main: PathBuf,

    /// Optional texpilot.toml path.
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Emit a machine-readable JSON build report to stdout after lint passes.
    #[arg(long)]
    pub report_json: bool,

    #[command(flatten)]
    pub flags: BuildFlags,

    #[command(flatten)]
    pub lint_flags: LintFlags,
}

#[derive(Debug, Args, Clone)]
pub struct WatchArgs {
    /// Root .tex file.
    pub main: PathBuf,

    /// Optional texpilot.toml path.
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Directory to watch. Defaults to the root document's directory.
    #[arg(long)]
    pub root: Option<PathBuf>,

    /// Rebuild without running the linter.
    #[arg(long)]
    pub no_lint: bool,

    /// Use the fastest edit-loop mode: one TeX pass with images disabled.
    #[arg(long)]
    pub preview: bool,

    /// In preview watch mode, run a final-quality build after this many idle milliseconds.
    #[arg(long, value_name = "MS", requires = "preview")]
    pub final_after_idle_ms: Option<u64>,

    #[command(flatten)]
    pub flags: BuildFlags,

    #[command(flatten)]
    pub lint_flags: LintFlags,
}

#[derive(Debug, Args, Clone)]
pub struct LintArgs {
    /// Files or directories to lint. Defaults to the current directory.
    pub paths: Vec<PathBuf>,

    /// Optional texpilot.toml path.
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Emit a machine-readable JSON lint report to stdout.
    #[arg(long)]
    pub report_json: bool,

    #[command(flatten)]
    pub flags: LintFlags,
}

#[derive(Debug, Args, Clone)]
pub struct LintFlags {
    /// Return exit code 1 when warnings are found.
    #[arg(long)]
    pub fail_on_warnings: bool,

    /// Allow warnings without failing lint/check.
    #[arg(long, conflicts_with = "fail_on_warnings")]
    pub allow_warnings: bool,
}

impl LintFlags {
    pub fn should_fail_on_warnings(&self) -> bool {
        self.fail_on_warnings || !self.allow_warnings
    }
}

#[derive(Debug, Args, Clone)]
pub struct BuildFlags {
    /// TeX engine to use.
    #[arg(long, value_enum, default_value_t = EngineArg::PdfLatex)]
    pub engine: EngineArg,

    /// Bibliography runner policy.
    #[arg(long, value_enum, default_value_t = BibArg::Auto)]
    pub bib: BibArg,

    /// Build orchestrator to use. `direct` avoids latexmk.
    #[arg(long, value_enum, default_value_t = RunnerArg::Direct)]
    pub runner: RunnerArg,

    /// Output directory for generated files.
    #[arg(long, default_value = "build")]
    pub out_dir: PathBuf,

    /// Override TeX's output job name, like pdflatex -jobname.
    #[arg(long)]
    pub job_name: Option<String>,

    /// Skip image inclusion, TikZ externalization, and minted highlighting.
    #[arg(long)]
    pub fast: bool,

    /// Alias for --fast: compile with image and highlighting placeholders.
    #[arg(long)]
    pub no_images: bool,

    /// First-pass no-image policy for faster aux discovery.
    #[arg(long, value_enum, default_value_t = DraftPrepassArg::Auto)]
    pub draft_prepass: DraftPrepassArg,

    /// Run exactly one TeX pass and skip bibliography/rerun convergence.
    #[arg(long)]
    pub once: bool,

    /// Cache a precompiled preamble format for direct pdfLaTeX builds.
    #[arg(long)]
    pub precompile_preamble: bool,

    /// Maximum TeX passes for the direct runner.
    #[arg(long, default_value_t = 8)]
    pub max_runs: usize,

    /// Ignore cached dependency state and run the compiler anyway.
    #[arg(long)]
    pub force: bool,

    /// Enable SyncTeX output.
    #[arg(long)]
    pub synctex: bool,

    /// Enable shell escape for packages that need it.
    #[arg(long)]
    pub shell_escape: bool,

    /// Print the underlying command before running it.
    #[arg(long)]
    pub print_command: bool,

    /// Reduce latexmk output.
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum EngineArg {
    PdfLatex,
    XeLatex,
    LuaLatex,
    Tectonic,
    TexpilotPdftex,
}

impl From<EngineArg> for Engine {
    fn from(value: EngineArg) -> Self {
        match value {
            EngineArg::PdfLatex => Engine::PdfLatex,
            EngineArg::XeLatex => Engine::XeLatex,
            EngineArg::LuaLatex => Engine::LuaLatex,
            EngineArg::Tectonic => Engine::Tectonic,
            EngineArg::TexpilotPdftex => Engine::TexpilotPdftex,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum BibArg {
    Auto,
    Bibtex,
    Biber,
    None,
}

impl From<BibArg> for BibMode {
    fn from(value: BibArg) -> Self {
        match value {
            BibArg::Auto => BibMode::Auto,
            BibArg::Bibtex => BibMode::BibTex,
            BibArg::Biber => BibMode::Biber,
            BibArg::None => BibMode::None,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum DraftPrepassArg {
    Auto,
    Always,
    Never,
}

impl From<DraftPrepassArg> for DraftPrepass {
    fn from(value: DraftPrepassArg) -> Self {
        match value {
            DraftPrepassArg::Auto => DraftPrepass::Auto,
            DraftPrepassArg::Always => DraftPrepass::Always,
            DraftPrepassArg::Never => DraftPrepass::Never,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum RunnerArg {
    Direct,
    Latexmk,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_accepts_no_images_alias() {
        let cli = Cli::try_parse_from(["texpilot", "build", "main.tex", "--no-images"])
            .expect("build --no-images should parse");
        let Command::Build(args) = cli.command else {
            panic!("expected build command");
        };

        assert!(args.flags.no_images);
        assert!(!args.flags.fast);
    }

    #[test]
    fn watch_accepts_no_images_alias_in_build_flags() {
        let cli = Cli::try_parse_from(["texpilot", "watch", "main.tex", "--no-images"])
            .expect("watch --no-images should parse");
        let Command::Watch(args) = cli.command else {
            panic!("expected watch command");
        };

        assert!(args.flags.no_images);
        assert!(!args.preview);
    }

    #[test]
    fn build_accepts_precompile_preamble_flag() {
        let cli = Cli::try_parse_from([
            "texpilot",
            "build",
            "main.tex",
            "--fast",
            "--once",
            "--precompile-preamble",
        ])
        .expect("build --precompile-preamble should parse");
        let Command::Build(args) = cli.command else {
            panic!("expected build command");
        };

        assert!(args.flags.fast);
        assert!(args.flags.once);
        assert!(args.flags.precompile_preamble);
    }

    #[test]
    fn build_accepts_experimental_texpilot_pdftex_engine() {
        let cli = Cli::try_parse_from([
            "texpilot",
            "build",
            "main.tex",
            "--engine",
            "texpilot-pdftex",
        ])
        .expect("build --engine texpilot-pdftex should parse");
        let Command::Build(args) = cli.command else {
            panic!("expected build command");
        };

        assert!(matches!(args.flags.engine, EngineArg::TexpilotPdftex));
    }
}

impl From<RunnerArg> for Runner {
    fn from(value: RunnerArg) -> Self {
        match value {
            RunnerArg::Direct => Runner::Direct,
            RunnerArg::Latexmk => Runner::Latexmk,
        }
    }
}

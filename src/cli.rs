use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::compiler::{BibMode, DraftPrepass, Engine, Runner};

#[derive(Debug, Parser)]
#[command(
    name = "tekai",
    about = "Fast, fidelity-preserving LaTeX builds and live previews",
    long_about = "tekai builds, watches, checks, and lints LaTeX projects. Its self-contained engine converges references and auxiliary tools, preserves final PDF rendering, and caches settled builds for fast repeat runs.",
    version,
    propagate_version = true,
    arg_required_else_help = true,
    after_help = "QUICK START:\n  tekai build main.tex\n  tekai watch main.tex --preview --allow-warnings\n  tekai check main.tex --allow-warnings\n\nProject defaults can be stored in ./tekai.toml. Run `tekai <command> --help` for command-specific examples."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Build a root TeX document to a settled final PDF.
    Build(BuildArgs),
    /// Safely remove the configured build output directory.
    Clean(CleanArgs),
    /// Lint TeX sources for structural and style issues.
    Lint(LintArgs),
    /// Lint a document tree, then build when lint passes.
    Check(CheckArgs),
    /// Watch dependencies and rebuild after relevant changes.
    Watch(WatchArgs),
}

#[derive(Debug, Args, Clone)]
#[command(
    after_help = "EXAMPLES:\n  tekai clean --dry-run\n  tekai clean --out-dir build\n  tekai clean --report-json"
)]
pub struct CleanArgs {
    /// Configuration file. Defaults to ./tekai.toml when it exists.
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Output directory to remove. Overrides [build].out_dir.
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
#[command(
    after_help = "EXAMPLES:\n  tekai build main.tex\n  tekai build main.tex --report-json\n  tekai build main.tex --runner latexmk\n  tekai build main.tex --once --fast"
)]
pub struct BuildArgs {
    /// Root TeX document to compile.
    pub main: PathBuf,

    /// Configuration file. Defaults to ./tekai.toml when it exists.
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Emit a machine-readable JSON build report to stdout.
    #[arg(long)]
    pub report_json: bool,

    #[command(flatten)]
    pub flags: BuildFlags,
}

#[derive(Debug, Args, Clone)]
#[command(
    after_help = "EXAMPLES:\n  tekai check main.tex --allow-warnings\n  tekai check main.tex --report-json --allow-warnings"
)]
pub struct CheckArgs {
    /// Root TeX document to lint and compile.
    pub main: PathBuf,

    /// Configuration file. Defaults to ./tekai.toml when it exists.
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
#[command(
    after_help = "EXAMPLES:\n  tekai watch main.tex --allow-warnings\n  tekai watch main.tex --preview --allow-warnings\n  tekai watch main.tex --preview --final-after-idle-ms 1500 --allow-warnings"
)]
pub struct WatchArgs {
    /// Root TeX document to rebuild.
    pub main: PathBuf,

    /// Configuration file. Defaults to ./tekai.toml when it exists.
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Directory to watch. Defaults to the root document's directory.
    #[arg(long)]
    pub root: Option<PathBuf>,

    /// Rebuild without running the linter.
    #[arg(long)]
    pub no_lint: bool,

    /// Use the low-latency, non-final edit loop with placeholder content.
    #[arg(long)]
    pub preview: bool,

    /// Also run an exact final build after the edit stream is idle for MS.
    #[arg(long, value_name = "MS", requires = "preview")]
    pub final_after_idle_ms: Option<u64>,

    #[command(flatten)]
    pub flags: BuildFlags,

    #[command(flatten)]
    pub lint_flags: LintFlags,
}

#[derive(Debug, Args, Clone)]
#[command(
    after_help = "EXAMPLES:\n  tekai lint\n  tekai lint paper --allow-warnings\n  tekai lint main.tex --report-json --allow-warnings"
)]
pub struct LintArgs {
    /// Files or directories to lint. Defaults to the current directory.
    pub paths: Vec<PathBuf>,

    /// Configuration file. Defaults to ./tekai.toml when it exists.
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
    /// Return exit code 1 when warnings are found. This is the default.
    #[arg(long)]
    pub fail_on_warnings: bool,

    /// Return success when only warnings are found.
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
    /// Typesetting engine. tekai-engine is the self-contained exact default.
    #[arg(long, value_enum, default_value_t = EngineArg::TekaiEngine)]
    pub engine: EngineArg,

    /// Bibliography policy: detect automatically, force a tool, or disable it.
    #[arg(long, value_enum, default_value_t = BibArg::Auto)]
    pub bib: BibArg,

    /// Build scheduler. direct is native; latexmk is the compatibility baseline.
    #[arg(long, value_enum, default_value_t = RunnerArg::Direct)]
    pub runner: RunnerArg,

    /// Directory for the PDF, auxiliary files, and build state.
    #[arg(long, default_value = "build")]
    pub out_dir: PathBuf,

    /// Override TeX's output job name, like pdflatex -jobname.
    #[arg(long)]
    pub job_name: Option<String>,

    /// Replace expensive external content with placeholders. Output is not final.
    #[arg(long)]
    pub fast: bool,

    /// Alias for --fast.
    #[arg(long)]
    pub no_images: bool,

    /// Draft policy for intermediate convergence passes; final output stays exact.
    #[arg(long, value_enum, default_value_t = DraftPrepassArg::Auto)]
    pub draft_prepass: DraftPrepassArg,

    /// Run one TeX pass without auxiliary tools or convergence. Output may be incomplete.
    #[arg(long)]
    pub once: bool,

    /// Reuse a compatible precompiled preamble for direct tekai-engine builds.
    #[arg(long)]
    pub precompile_preamble: bool,

    /// Fail after this many unsettled TeX passes.
    #[arg(long, default_value_t = 8)]
    pub max_runs: usize,

    /// Ignore cached dependency state and run the compiler anyway.
    #[arg(long)]
    pub force: bool,

    /// Enable SyncTeX output.
    #[arg(long)]
    pub synctex: bool,

    /// Pass shell escape to TeX. Use only with trusted documents.
    #[arg(long)]
    pub shell_escape: bool,

    /// Print each engine or auxiliary command before it runs.
    #[arg(long)]
    pub print_command: bool,

    /// Suppress TeX engine and auxiliary-tool output.
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum EngineArg {
    TekaiEngine,
    XeLatex,
    LuaLatex,
    Tectonic,
    TekaiPdftex,
    TekaiPdftexCertified,
}

impl From<EngineArg> for Engine {
    fn from(value: EngineArg) -> Self {
        match value {
            EngineArg::TekaiEngine => Engine::PdfLatex,
            EngineArg::XeLatex => Engine::XeLatex,
            EngineArg::LuaLatex => Engine::LuaLatex,
            EngineArg::Tectonic => Engine::Tectonic,
            EngineArg::TekaiPdftex => Engine::TekaiPdftex,
            EngineArg::TekaiPdftexCertified => Engine::TekaiPdftexCertified,
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

impl From<RunnerArg> for Runner {
    fn from(value: RunnerArg) -> Self {
        match value {
            RunnerArg::Direct => Runner::Direct,
            RunnerArg::Latexmk => Runner::Latexmk,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn build_accepts_no_images_alias() {
        let cli = Cli::try_parse_from(["tekai", "build", "main.tex", "--no-images"])
            .expect("build --no-images should parse");
        let Command::Build(args) = cli.command else {
            panic!("expected build command");
        };

        assert!(args.flags.no_images);
        assert!(!args.flags.fast);
    }

    #[test]
    fn watch_accepts_no_images_alias_in_build_flags() {
        let cli = Cli::try_parse_from(["tekai", "watch", "main.tex", "--no-images"])
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
            "tekai",
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
    fn build_defaults_to_tekai_engine() {
        let cli = Cli::try_parse_from(["tekai", "build", "main.tex"])
            .expect("default build should parse");
        let Command::Build(args) = cli.command else {
            panic!("expected build command");
        };

        assert!(matches!(args.flags.engine, EngineArg::TekaiEngine));
    }

    #[test]
    fn build_rejects_legacy_exact_engine_names() {
        for legacy in ["pdf-latex", "pdflatex"] {
            assert!(
                Cli::try_parse_from(["tekai", "build", "main.tex", "--engine", legacy]).is_err(),
                "legacy engine name {legacy} should be rejected"
            );
        }
    }

    #[test]
    fn build_accepts_experimental_tekai_pdftex_engine() {
        let cli = Cli::try_parse_from(["tekai", "build", "main.tex", "--engine", "tekai-pdftex"])
            .expect("build --engine tekai-pdftex should parse");
        let Command::Build(args) = cli.command else {
            panic!("expected build command");
        };

        assert!(matches!(args.flags.engine, EngineArg::TekaiPdftex));
    }

    #[test]
    fn build_accepts_certified_tekai_pdftex_engine() {
        let cli = Cli::try_parse_from([
            "tekai",
            "build",
            "main.tex",
            "--engine",
            "tekai-pdftex-certified",
        ])
        .expect("build --engine tekai-pdftex-certified should parse");
        let Command::Build(args) = cli.command else {
            panic!("expected build command");
        };

        assert!(matches!(args.flags.engine, EngineArg::TekaiPdftexCertified));
    }

    #[test]
    fn top_level_help_uses_public_name_and_examples() {
        let help = Cli::command().render_long_help().to_string();
        assert!(help.contains("self-contained engine"));
        assert!(help.contains("tekai watch main.tex --preview"));
        assert!(
            !help
                .to_ascii_lowercase()
                .contains(&["tex", "pilot"].concat())
        );
    }
}

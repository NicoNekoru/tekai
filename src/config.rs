use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::compiler::{BibMode, DraftPrepass, Engine, Runner};
use crate::lint::{IndentStyle, LintConfig, ProseWrap, RuleLevel};

#[derive(Debug, Default, Deserialize)]
struct RawConfig {
    build: Option<RawBuildConfig>,
    lint: Option<RawLintConfig>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BuildConfig {
    pub engine: Option<Engine>,
    pub runner: Option<Runner>,
    pub bib_mode: Option<BibMode>,
    pub out_dir: Option<PathBuf>,
    pub job_name: Option<String>,
    pub fast: Option<bool>,
    pub draft_prepass: Option<DraftPrepass>,
    pub once: Option<bool>,
    pub max_runs: Option<usize>,
    pub force: Option<bool>,
    pub precompile_preamble: Option<bool>,
    pub synctex: Option<bool>,
    pub shell_escape: Option<bool>,
    pub quiet: Option<bool>,
    pub print_command: Option<bool>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub build: BuildConfig,
    pub lint: LintConfig,
}

#[derive(Debug, Default, Deserialize)]
struct RawBuildConfig {
    engine: Option<String>,
    runner: Option<String>,
    bib: Option<String>,
    bibliography: Option<String>,
    out_dir: Option<PathBuf>,
    job_name: Option<String>,
    fast: Option<bool>,
    no_images: Option<bool>,
    draft_prepass: Option<String>,
    once: Option<bool>,
    max_runs: Option<usize>,
    force: Option<bool>,
    precompile_preamble: Option<bool>,
    synctex: Option<bool>,
    shell_escape: Option<bool>,
    quiet: Option<bool>,
    print_command: Option<bool>,
    env: Option<HashMap<String, String>>,
}

#[derive(Debug, Default, Deserialize)]
struct RawLintConfig {
    indent_size: Option<usize>,
    indent_style: Option<String>,
    indent_environments: Option<bool>,
    indent_display_math: Option<bool>,
    ignored_indent_environments: Option<Vec<String>>,
    prefer_paren_inline_math: Option<bool>,
    prefer_bracket_display_math: Option<bool>,
    prefer_prime_command: Option<bool>,
    check_environment_stack: Option<bool>,
    max_line_length: Option<usize>,
    prose_wrap: Option<String>,
    rules: Option<HashMap<String, String>>,
}

pub fn load_project_config(path: Option<&Path>) -> Result<ProjectConfig> {
    let Some(raw) = load_raw_config(path)? else {
        return Ok(ProjectConfig {
            build: BuildConfig::default(),
            lint: LintConfig::default(),
        });
    };
    Ok(ProjectConfig {
        build: apply_build_config(raw.build)?,
        lint: apply_lint_config(LintConfig::default(), raw.lint)?,
    })
}

pub fn load_build_config(path: Option<&Path>) -> Result<BuildConfig> {
    let Some(raw) = load_raw_config(path)? else {
        return Ok(BuildConfig::default());
    };
    apply_build_config(raw.build)
}

pub fn load_lint_config(path: Option<&Path>) -> Result<LintConfig> {
    let Some(raw) = load_raw_config(path)? else {
        return Ok(LintConfig::default());
    };
    apply_lint_config(LintConfig::default(), raw.lint)
}

fn load_raw_config(path: Option<&Path>) -> Result<Option<RawConfig>> {
    let Some(path) = path.map(Path::to_path_buf).or_else(default_config_path) else {
        return Ok(None);
    };
    let source = fs::read_to_string(&path)
        .with_context(|| format!("failed to read config {}", path.display()))?;
    let raw: RawConfig = toml::from_str(&source)
        .with_context(|| format!("failed to parse config {}", path.display()))?;
    Ok(Some(raw))
}

fn default_config_path() -> Option<PathBuf> {
    let candidate = std::env::current_dir().ok()?.join("tekai.toml");
    candidate.exists().then_some(candidate)
}

fn apply_lint_config(mut config: LintConfig, raw: Option<RawLintConfig>) -> Result<LintConfig> {
    let Some(raw) = raw else {
        return Ok(config);
    };
    if let Some(value) = raw.indent_size {
        config.indent_size = value;
    }
    if let Some(value) = raw.indent_style {
        config.indent_style = value
            .parse::<IndentStyle>()
            .map_err(|message| anyhow::anyhow!("invalid lint.indent_style value: {message}"))?;
    }
    if let Some(value) = raw.indent_environments {
        config.indent_environments = value;
    }
    if let Some(value) = raw.indent_display_math {
        config.indent_display_math = value;
    }
    if let Some(value) = raw.ignored_indent_environments {
        config.ignored_indent_environments = value;
    }
    if let Some(value) = raw.prefer_paren_inline_math {
        config.prefer_paren_inline_math = value;
    }
    if let Some(value) = raw.prefer_bracket_display_math {
        config.prefer_bracket_display_math = value;
    }
    if let Some(value) = raw.prefer_prime_command {
        config.prefer_prime_command = value;
    }
    if let Some(value) = raw.check_environment_stack {
        config.check_environment_stack = value;
    }
    if let Some(value) = raw.max_line_length {
        config.max_line_length = Some(value);
    }
    if let Some(value) = raw.prose_wrap {
        config.prose_wrap = Some(
            value
                .parse::<ProseWrap>()
                .map_err(|message| anyhow::anyhow!("invalid lint.prose_wrap value: {message}"))?,
        );
    }
    if let Some(rules) = raw.rules {
        for (rule, level) in rules {
            let level = level
                .parse::<RuleLevel>()
                .map_err(|message| anyhow::anyhow!("invalid lint.rules.{rule} value: {message}"))?;
            config.rule_levels.insert(rule, level);
        }
    }
    Ok(config)
}

fn apply_build_config(raw: Option<RawBuildConfig>) -> Result<BuildConfig> {
    let Some(raw) = raw else {
        return Ok(BuildConfig::default());
    };

    let bib_value = match (raw.bib, raw.bibliography) {
        (Some(_), Some(_)) => {
            anyhow::bail!("build.bib and build.bibliography cannot both be set")
        }
        (bib, bibliography) => bib.or(bibliography),
    };
    let fast = match (raw.fast, raw.no_images) {
        (Some(fast), Some(no_images)) if fast != no_images => {
            anyhow::bail!("build.fast and build.no_images cannot disagree")
        }
        (Some(fast), _) => Some(fast),
        (_, no_images) => no_images,
    };

    let env = raw.env.unwrap_or_default();
    validate_env_config(&env)?;

    Ok(BuildConfig {
        engine: raw
            .engine
            .as_deref()
            .map(parse_engine)
            .transpose()
            .context("invalid build.engine")?,
        runner: raw
            .runner
            .as_deref()
            .map(parse_runner)
            .transpose()
            .context("invalid build.runner")?,
        bib_mode: bib_value
            .as_deref()
            .map(parse_bib_mode)
            .transpose()
            .context("invalid build.bib")?,
        out_dir: raw.out_dir,
        job_name: raw.job_name,
        fast,
        draft_prepass: raw
            .draft_prepass
            .as_deref()
            .map(parse_draft_prepass)
            .transpose()
            .context("invalid build.draft_prepass")?,
        once: raw.once,
        max_runs: raw.max_runs,
        force: raw.force,
        precompile_preamble: raw.precompile_preamble,
        synctex: raw.synctex,
        shell_escape: raw.shell_escape,
        quiet: raw.quiet,
        print_command: raw.print_command,
        env,
    })
}

fn validate_env_config(env: &HashMap<String, String>) -> Result<()> {
    for (key, value) in env {
        if key.is_empty() {
            anyhow::bail!("build.env contains an empty variable name");
        }
        if key.contains('=') {
            anyhow::bail!("build.env variable name {key:?} contains '='");
        }
        if key.contains('\0') {
            anyhow::bail!("build.env variable name {key:?} contains NUL");
        }
        if value.contains('\0') {
            anyhow::bail!("build.env.{key} contains NUL");
        }
    }
    Ok(())
}

fn parse_engine(value: &str) -> Result<Engine> {
    match normalize_choice(value).as_str() {
        "tekai-engine" => Ok(Engine::PdfLatex),
        "xelatex" | "xe-latex" => Ok(Engine::XeLatex),
        "lualatex" | "lua-latex" => Ok(Engine::LuaLatex),
        "tectonic" => Ok(Engine::Tectonic),
        "tekai-pdftex" | "tekai_pdftex" => Ok(Engine::TekaiPdftex),
        "tekai-pdftex-certified" | "tekai_pdftex_certified" | "tekai-pdftex-oracle" => {
            Ok(Engine::TekaiPdftexCertified)
        }
        _ => {
            anyhow::bail!(
                "expected one of tekai-engine, xelatex, lualatex, tectonic, tekai-pdftex, tekai-pdftex-certified"
            )
        }
    }
}

fn parse_runner(value: &str) -> Result<Runner> {
    match normalize_choice(value).as_str() {
        "direct" => Ok(Runner::Direct),
        "latexmk" => Ok(Runner::Latexmk),
        _ => anyhow::bail!("expected one of direct, latexmk"),
    }
}

fn parse_bib_mode(value: &str) -> Result<BibMode> {
    match normalize_choice(value).as_str() {
        "auto" => Ok(BibMode::Auto),
        "bibtex" | "bib-tex" => Ok(BibMode::BibTex),
        "biber" => Ok(BibMode::Biber),
        "none" | "off" | "false" => Ok(BibMode::None),
        _ => anyhow::bail!("expected one of auto, bibtex, biber, none"),
    }
}

fn parse_draft_prepass(value: &str) -> Result<DraftPrepass> {
    match normalize_choice(value).as_str() {
        "auto" => Ok(DraftPrepass::Auto),
        "always" | "on" | "true" => Ok(DraftPrepass::Always),
        "never" | "off" | "false" => Ok(DraftPrepass::Never),
        _ => anyhow::bail!("expected one of auto, always, never"),
    }
}

fn normalize_choice(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('_', "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_lint_config_parses_rule_levels() {
        let rules = HashMap::from([
            ("math/inline-dollar".to_string(), "off".to_string()),
            ("math/display-dollar".to_string(), "error".to_string()),
            ("line/length".to_string(), "warn".to_string()),
        ]);

        let config = apply_lint_config(
            LintConfig::default(),
            Some(RawLintConfig {
                rules: Some(rules),
                ..RawLintConfig::default()
            }),
        )
        .expect("rule levels should parse");

        assert_eq!(
            config.rule_levels.get("math/inline-dollar"),
            Some(&RuleLevel::Off)
        );
        assert_eq!(
            config.rule_levels.get("math/display-dollar"),
            Some(&RuleLevel::Error)
        );
        assert_eq!(
            config.rule_levels.get("line/length"),
            Some(&RuleLevel::Warning)
        );
    }

    #[test]
    fn apply_lint_config_parses_display_math_indentation_toggle() {
        let config = apply_lint_config(
            LintConfig::default(),
            Some(RawLintConfig {
                indent_display_math: Some(false),
                ..RawLintConfig::default()
            }),
        )
        .expect("display math indentation config should parse");

        assert!(!config.indent_display_math);
    }

    #[test]
    fn apply_lint_config_parses_tab_indentation_style() {
        let config = apply_lint_config(
            LintConfig::default(),
            Some(RawLintConfig {
                indent_style: Some("tabs".to_string()),
                ..RawLintConfig::default()
            }),
        )
        .expect("tab indentation config should parse");

        assert_eq!(config.indent_style, IndentStyle::Tabs);
    }

    #[test]
    fn apply_lint_config_rejects_unknown_indentation_style() {
        let error = apply_lint_config(
            LintConfig::default(),
            Some(RawLintConfig {
                indent_style: Some("mixed".to_string()),
                ..RawLintConfig::default()
            }),
        )
        .expect_err("unknown indentation style should fail");

        assert!(
            error
                .to_string()
                .contains("invalid lint.indent_style value"),
            "{error:#}"
        );
    }

    #[test]
    fn apply_lint_config_parses_prose_wrap_modes() {
        for (raw_value, expected) in [
            ("hardwrap", ProseWrap::Hard),
            ("unwrapped", ProseWrap::Unwrapped),
        ] {
            let config = apply_lint_config(
                LintConfig::default(),
                Some(RawLintConfig {
                    prose_wrap: Some(raw_value.to_string()),
                    ..RawLintConfig::default()
                }),
            )
            .expect("prose wrap config should parse");

            assert_eq!(config.prose_wrap, Some(expected));
        }
    }

    #[test]
    fn apply_lint_config_rejects_unknown_prose_wrap_mode() {
        let error = apply_lint_config(
            LintConfig::default(),
            Some(RawLintConfig {
                prose_wrap: Some("sometimes".to_string()),
                ..RawLintConfig::default()
            }),
        )
        .expect_err("unknown prose wrap mode should fail");

        assert!(
            error.to_string().contains("invalid lint.prose_wrap value"),
            "{error:#}"
        );
    }

    #[test]
    fn apply_lint_config_rejects_unknown_rule_level() {
        let error = apply_lint_config(
            LintConfig::default(),
            Some(RawLintConfig {
                rules: Some(HashMap::from([(
                    "math/inline-dollar".to_string(),
                    "sometimes".to_string(),
                )])),
                ..RawLintConfig::default()
            }),
        )
        .expect_err("unknown rule level should fail");

        assert!(
            error
                .to_string()
                .contains("invalid lint.rules.math/inline-dollar"),
            "{error:#}"
        );
    }

    #[test]
    fn load_project_config_parses_build_defaults() {
        let raw: RawConfig = toml::from_str(
            r#"
            [build]
            engine = "lua-latex"
            runner = "direct"
            bib = "biber"
            out_dir = "paper-build"
            job_name = "draft"
            no_images = true
            draft_prepass = "always"
            once = true
            max_runs = 3
            force = true
            precompile_preamble = true
            synctex = true
            shell_escape = true
            quiet = true
            print_command = true

            [build.env]
            TEXINPUTS = "tex//:"
            BIBINPUTS = "bib//:"
            "#,
        )
        .expect("raw config should parse");

        let config = apply_build_config(raw.build).expect("build config should apply");

        assert_eq!(config.engine, Some(Engine::LuaLatex));
        assert_eq!(config.runner, Some(Runner::Direct));
        assert_eq!(config.bib_mode, Some(BibMode::Biber));
        assert_eq!(config.out_dir.as_deref(), Some(Path::new("paper-build")));
        assert_eq!(config.job_name.as_deref(), Some("draft"));
        assert_eq!(config.fast, Some(true));
        assert_eq!(config.draft_prepass, Some(DraftPrepass::Always));
        assert_eq!(config.once, Some(true));
        assert_eq!(config.max_runs, Some(3));
        assert_eq!(config.force, Some(true));
        assert_eq!(config.precompile_preamble, Some(true));
        assert_eq!(config.synctex, Some(true));
        assert_eq!(config.shell_escape, Some(true));
        assert_eq!(config.quiet, Some(true));
        assert_eq!(config.print_command, Some(true));
        assert_eq!(
            config.env.get("TEXINPUTS").map(String::as_str),
            Some("tex//:")
        );
        assert_eq!(
            config.env.get("BIBINPUTS").map(String::as_str),
            Some("bib//:")
        );
    }

    #[test]
    fn apply_build_config_parses_experimental_tekai_pdftex_engine() {
        let raw: RawConfig = toml::from_str(
            r#"
            [build]
            engine = "tekai-pdftex"
            "#,
        )
        .expect("raw config should parse");

        let config = apply_build_config(raw.build).expect("build config should apply");

        assert_eq!(config.engine, Some(Engine::TekaiPdftex));
    }

    #[test]
    fn apply_build_config_parses_tekai_engine() {
        let raw: RawConfig = toml::from_str(
            r#"
            [build]
            engine = "tekai-engine"
            "#,
        )
        .expect("raw config should parse");

        let config = apply_build_config(raw.build).expect("build config should apply");
        assert_eq!(config.engine, Some(Engine::PdfLatex));
    }

    #[test]
    fn apply_build_config_rejects_legacy_exact_engine_names() {
        for engine in ["pdf-latex", "pdflatex"] {
            let raw: RawConfig = toml::from_str(&format!(
                r#"
                [build]
                engine = "{engine}"
                "#,
            ))
            .expect("raw config should parse");

            assert!(
                apply_build_config(raw.build).is_err(),
                "legacy engine name {engine} should be rejected"
            );
        }
    }

    #[test]
    fn apply_build_config_parses_certified_tekai_pdftex_engine() {
        let raw: RawConfig = toml::from_str(
            r#"
            [build]
            engine = "tekai-pdftex-certified"
            "#,
        )
        .expect("raw config should parse");

        let config = apply_build_config(raw.build).expect("build config should apply");

        assert_eq!(config.engine, Some(Engine::TekaiPdftexCertified));
    }

    #[test]
    fn apply_build_config_rejects_conflicting_bibliography_keys() {
        let error = apply_build_config(Some(RawBuildConfig {
            bib: Some("bibtex".to_string()),
            bibliography: Some("biber".to_string()),
            ..RawBuildConfig::default()
        }))
        .expect_err("conflicting bibliography keys should fail");

        assert!(
            error
                .to_string()
                .contains("build.bib and build.bibliography"),
            "{error:#}"
        );
    }

    #[test]
    fn apply_build_config_rejects_conflicting_fast_aliases() {
        let error = apply_build_config(Some(RawBuildConfig {
            fast: Some(true),
            no_images: Some(false),
            ..RawBuildConfig::default()
        }))
        .expect_err("conflicting fast aliases should fail");

        assert!(
            error.to_string().contains("build.fast and build.no_images"),
            "{error:#}"
        );
    }

    #[test]
    fn apply_build_config_rejects_invalid_env_names() {
        let error = apply_build_config(Some(RawBuildConfig {
            env: Some(HashMap::from([(
                "BAD=NAME".to_string(),
                "value".to_string(),
            )])),
            ..RawBuildConfig::default()
        }))
        .expect_err("invalid env key should fail");

        assert!(error.to_string().contains("contains '='"), "{error:#}");
    }
}

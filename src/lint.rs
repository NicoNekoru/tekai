use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub struct LintConfig {
    pub indent_size: usize,
    pub indent_environments: bool,
    pub indent_display_math: bool,
    pub ignored_indent_environments: Vec<String>,
    pub prefer_paren_inline_math: bool,
    pub prefer_bracket_display_math: bool,
    pub prefer_prime_command: bool,
    pub check_environment_stack: bool,
    pub max_line_length: Option<usize>,
    pub rule_levels: HashMap<String, RuleLevel>,
}

impl Default for LintConfig {
    fn default() -> Self {
        Self {
            indent_size: 2,
            indent_environments: true,
            indent_display_math: true,
            ignored_indent_environments: vec!["document".to_string()],
            prefer_paren_inline_math: true,
            prefer_bracket_display_math: true,
            prefer_prime_command: false,
            check_environment_stack: true,
            max_line_length: Some(120),
            rule_levels: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RuleLevel {
    Off,
    Warning,
    Error,
}

impl std::str::FromStr for RuleLevel {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" | "allow" | "ignore" => Ok(Self::Off),
            "warn" | "warning" => Ok(Self::Warning),
            "error" | "deny" => Ok(Self::Error),
            _ => Err(format!(
                "invalid rule level '{value}'; expected off, warn, or error"
            )),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Diagnostic {
    pub path: PathBuf,
    pub line: usize,
    pub column: usize,
    pub severity: Severity,
    pub rule: &'static str,
    pub message: String,
    pub help: Option<String>,
}

impl Diagnostic {
    fn warning(
        path: &Path,
        line: usize,
        column: usize,
        rule: &'static str,
        message: impl Into<String>,
        help: impl Into<Option<String>>,
    ) -> Self {
        Self {
            path: path.to_path_buf(),
            line,
            column,
            severity: Severity::Warning,
            rule,
            message: message.into(),
            help: help.into(),
        }
    }

    fn error(
        path: &Path,
        line: usize,
        column: usize,
        rule: &'static str,
        message: impl Into<String>,
        help: impl Into<Option<String>>,
    ) -> Self {
        Self {
            path: path.to_path_buf(),
            line,
            column,
            severity: Severity::Error,
            rule,
            message: message.into(),
            help: help.into(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum MathMode {
    InlineDollar {
        line: usize,
        column: usize,
    },
    DisplayDollar {
        line: usize,
        column: usize,
    },
    InlineParen {
        line: usize,
        column: usize,
    },
    DisplayBracket {
        line: usize,
        column: usize,
    },
    Environment {
        stack: Vec<String>,
        line: usize,
        column: usize,
    },
}

impl MathMode {
    fn is_math(&self) -> bool {
        true
    }

    fn start(&self) -> (usize, usize) {
        match self {
            Self::InlineDollar { line, column }
            | Self::DisplayDollar { line, column }
            | Self::InlineParen { line, column }
            | Self::DisplayBracket { line, column }
            | Self::Environment { line, column, .. } => (*line, *column),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct MathFenceFrame {
    family: MathFenceFamily,
    token: String,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct MathFence {
    family: MathFenceFamily,
    token: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum MathFenceFamily {
    Invisible,
    Paren,
    Bracket,
    Brace,
    Angle,
    Floor,
    Ceiling,
    Vert,
    DoubleVert,
    Other(String),
}

#[derive(Debug)]
struct LintState {
    math_mode: Option<MathMode>,
    math_fence_stack: Vec<MathFenceFrame>,
    nested_math_env_stack: Vec<EnvFrame>,
    env_stack: Vec<EnvFrame>,
    indent_level: usize,
    verbatim_env: Option<String>,
}

#[derive(Debug, Clone)]
struct EnvFrame {
    name: String,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum EnvKind {
    Begin,
    End,
}

#[derive(Debug, Clone)]
struct EnvEvent {
    kind: EnvKind,
    name: String,
    column: usize,
}

#[derive(Copy, Clone)]
struct LintLine<'a> {
    source: &'a str,
    scan: &'a str,
    math_mode_before: Option<&'a MathMode>,
}

pub fn lint_paths(paths: &[PathBuf], config: &LintConfig) -> Result<Vec<Diagnostic>> {
    let targets = if paths.is_empty() {
        vec![std::env::current_dir()?]
    } else {
        paths.to_vec()
    };
    let mut files = Vec::new();
    for target in targets {
        collect_tex_files(&target, &mut files)?;
    }
    files.sort();
    files.dedup();

    let mut diagnostics = Vec::new();
    for file in files {
        let source = fs::read_to_string(&file)
            .with_context(|| format!("failed to read TeX source {}", file.display()))?;
        diagnostics.extend(lint_source(&file, &source, config));
    }
    Ok(diagnostics)
}

pub fn lint_source(path: &Path, source: &str, config: &LintConfig) -> Vec<Diagnostic> {
    let suppressions = lint_suppressions(source);
    let ignored: HashSet<&str> = config
        .ignored_indent_environments
        .iter()
        .map(String::as_str)
        .collect();
    let mut diagnostics = Vec::new();
    let mut state = LintState {
        math_mode: None,
        math_fence_stack: Vec::new(),
        nested_math_env_stack: Vec::new(),
        env_stack: Vec::new(),
        indent_level: 0,
        verbatim_env: None,
    };

    for (index, raw_line) in source.lines().enumerate() {
        let line_no = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        lint_line_length(path, line, line_no, config, &mut diagnostics);
        lint_tabs(path, line, line_no, &mut diagnostics);
        if state.verbatim_env.is_some() {
            lint_verbatim_line(
                path,
                line,
                line_no,
                config,
                &ignored,
                &mut state,
                &mut diagnostics,
            );
            continue;
        }

        let masked = mask_inline_verbatim(line);
        let uncommented = strip_comment(&masked);
        let math_mode_before = state.math_mode.clone();
        lint_math(
            path,
            uncommented,
            line_no,
            config,
            &mut state,
            &mut diagnostics,
        );
        lint_environments_and_indent(
            path,
            LintLine {
                source: line,
                scan: uncommented,
                math_mode_before: math_mode_before.as_ref(),
            },
            line_no,
            config,
            &ignored,
            &mut state,
            &mut diagnostics,
        );
        update_verbatim_state(uncommented, &mut state);
    }

    if let Some(mode) = state.math_mode.take() {
        flush_unclosed_nested_math_environments(path, &mut state, &mut diagnostics);
        flush_unclosed_math_fences(path, &mut state, &mut diagnostics);
        let (line, column) = mode.start();
        diagnostics.push(Diagnostic::error(
            path,
            line,
            column,
            "math/unclosed",
            "math mode delimiter is not closed",
            Some("Close the math span with the matching delimiter.".to_string()),
        ));
    }
    if config.check_environment_stack {
        for frame in state.env_stack {
            diagnostics.push(Diagnostic::error(
                path,
                frame.line,
                frame.column,
                "env/unclosed",
                format!("environment '{}' is not closed", frame.name),
                Some(format!("Add \\end{{{}}}.", frame.name)),
            ));
        }
    }
    apply_rule_levels(&mut diagnostics, config);
    diagnostics.retain(|diagnostic| !suppressions.suppresses(diagnostic.line, diagnostic.rule));
    diagnostics
}

fn apply_rule_levels(diagnostics: &mut Vec<Diagnostic>, config: &LintConfig) {
    diagnostics.retain_mut(|diagnostic| match config.rule_levels.get(diagnostic.rule) {
        Some(RuleLevel::Off) => false,
        Some(RuleLevel::Warning) => {
            diagnostic.severity = Severity::Warning;
            true
        }
        Some(RuleLevel::Error) => {
            diagnostic.severity = Severity::Error;
            true
        }
        None => true,
    });
}

pub fn format_diagnostic(diagnostic: &Diagnostic) -> String {
    let severity = match diagnostic.severity {
        Severity::Warning => "warning",
        Severity::Error => "error",
    };
    let mut out = format!(
        "{}:{}:{}: {}[{}]: {}",
        diagnostic.path.display(),
        diagnostic.line,
        diagnostic.column,
        severity,
        diagnostic.rule,
        diagnostic.message
    );
    if let Some(help) = &diagnostic.help {
        out.push_str("\n  help: ");
        out.push_str(help);
    }
    out
}

pub fn has_errors(diagnostics: &[Diagnostic]) -> bool {
    diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == Severity::Error)
}

fn collect_tex_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    if path.is_file() {
        if is_tex_like(path) {
            files.push(path.to_path_buf());
        }
        return Ok(());
    }
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|entry| !is_ignored_dir(entry))
    {
        let entry = entry?;
        if entry.file_type().is_file() && is_tex_like(entry.path()) {
            files.push(entry.path().to_path_buf());
        }
    }
    Ok(())
}

#[derive(Debug, Default)]
struct LintSuppressions {
    all_by_line: HashSet<usize>,
    rules_by_line: HashMap<usize, HashSet<String>>,
}

impl LintSuppressions {
    fn suppress_all(&mut self, line: usize) {
        self.all_by_line.insert(line);
    }

    fn suppress_rules(&mut self, line: usize, rules: Vec<String>) {
        if rules.is_empty() {
            self.suppress_all(line);
            return;
        }
        self.rules_by_line.entry(line).or_default().extend(rules);
    }

    fn suppresses(&self, line: usize, rule: &str) -> bool {
        self.all_by_line.contains(&line)
            || self
                .rules_by_line
                .get(&line)
                .is_some_and(|rules| rules.contains(rule))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum LintSuppressionTarget {
    Line,
    NextLine,
}

fn lint_suppressions(source: &str) -> LintSuppressions {
    let mut suppressions = LintSuppressions::default();
    for (index, raw_line) in source.lines().enumerate() {
        let line_no = index + 1;
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let masked = mask_inline_verbatim(line);
        let Some(comment) = tex_comment_text(&masked) else {
            continue;
        };
        let Some((target, rules)) = parse_lint_suppression(comment) else {
            continue;
        };
        let target_line = match target {
            LintSuppressionTarget::Line => line_no,
            LintSuppressionTarget::NextLine => line_no + 1,
        };
        suppressions.suppress_rules(target_line, rules);
    }
    suppressions
}

fn parse_lint_suppression(comment: &str) -> Option<(LintSuppressionTarget, Vec<String>)> {
    let comment = comment.trim_start();
    if let Some(rest) = comment.strip_prefix("texpilot-ignore-line") {
        return Some((LintSuppressionTarget::Line, parse_suppression_rules(rest)));
    }
    if let Some(rest) = comment.strip_prefix("texpilot-ignore-next-line") {
        return Some((
            LintSuppressionTarget::NextLine,
            parse_suppression_rules(rest),
        ));
    }
    None
}

fn parse_suppression_rules(rest: &str) -> Vec<String> {
    rest.trim_start_matches(|ch: char| ch == ':' || ch.is_ascii_whitespace())
        .split(|ch: char| ch == ',' || ch.is_ascii_whitespace())
        .filter(|rule| !rule.is_empty())
        .map(str::to_string)
        .collect()
}

fn is_ignored_dir(entry: &DirEntry) -> bool {
    if !entry.file_type().is_dir() {
        return false;
    }
    let name = entry.file_name().to_string_lossy();
    matches!(
        name.as_ref(),
        ".git" | "target" | "build" | ".latexmk" | ".texpilot"
    )
}

fn is_tex_like(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            ["tex", "ltx", "sty", "cls"]
                .iter()
                .any(|candidate| ext.eq_ignore_ascii_case(candidate))
        })
}

fn lint_line_length(
    path: &Path,
    line: &str,
    line_no: usize,
    config: &LintConfig,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(limit) = config.max_line_length else {
        return;
    };
    let len = line.chars().count();
    if len > limit {
        diagnostics.push(Diagnostic::warning(
            path,
            line_no,
            limit + 1,
            "line/length",
            format!("line is {len} characters; configured maximum is {limit}"),
            None,
        ));
    }
}

fn lint_tabs(path: &Path, line: &str, line_no: usize, diagnostics: &mut Vec<Diagnostic>) {
    if let Some(column) = line.chars().position(|ch| ch == '\t').map(|idx| idx + 1) {
        diagnostics.push(Diagnostic::warning(
            path,
            line_no,
            column,
            "indent/tabs",
            "tab indentation is not allowed",
            Some("Use spaces for deterministic TeX diffs.".to_string()),
        ));
    }
}

fn lint_math(
    path: &Path,
    line: &str,
    line_no: usize,
    config: &LintConfig,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let bytes = line.as_bytes();
    let mut byte = 0;
    while byte < bytes.len() {
        let column = column_for_byte(line, byte);
        if bytes[byte] == b'\\' {
            if config.prefer_prime_command
                && state.math_mode.as_ref().is_some_and(MathMode::is_math)
                && let Some(end) = math_text_command_payload_end(line, byte)
            {
                byte = end;
                continue;
            }
            if starts_with_at(bytes, byte, br"\begin{")
                && let Some((name, end)) = read_braced_name(line, byte + br"\begin{".len())
            {
                if is_math_environment(&name) {
                    open_math_environment(path, line_no, column, name, state, diagnostics);
                }
                byte = end;
                continue;
            }
            if starts_with_at(bytes, byte, br"\end{")
                && let Some((name, end)) = read_braced_name(line, byte + br"\end{".len())
            {
                if is_math_environment(&name) {
                    close_math_environment(path, line_no, column, &name, state, diagnostics);
                }
                byte = end;
                continue;
            }
            if state.math_mode.as_ref().is_some_and(MathMode::is_math)
                && starts_with_latex_command(bytes, byte, br"\left")
                && let Some((fence, end)) = math_fence_after_command(line, byte + br"\left".len())
            {
                state.math_fence_stack.push(MathFenceFrame {
                    family: fence.family,
                    token: fence.token,
                    line: line_no,
                    column,
                });
                byte = end;
                continue;
            }
            if state.math_mode.as_ref().is_some_and(MathMode::is_math)
                && starts_with_latex_command(bytes, byte, br"\right")
                && let Some((fence, end)) = math_fence_after_command(line, byte + br"\right".len())
            {
                close_math_fence(path, line_no, column, fence, state, diagnostics);
                byte = end;
                continue;
            }
            if starts_with_at(bytes, byte, br"\(") {
                open_command_math(
                    path,
                    line_no,
                    column,
                    MathMode::InlineParen {
                        line: line_no,
                        column,
                    },
                    state,
                    diagnostics,
                );
                byte += 2;
                continue;
            }
            if starts_with_at(bytes, byte, br"\)") {
                close_command_math(
                    path,
                    line_no,
                    column,
                    "math/unmatched-paren",
                    |mode| matches!(mode, MathMode::InlineParen { .. }),
                    state,
                    diagnostics,
                );
                byte += 2;
                continue;
            }
            if starts_with_at(bytes, byte, br"\[") {
                open_command_math(
                    path,
                    line_no,
                    column,
                    MathMode::DisplayBracket {
                        line: line_no,
                        column,
                    },
                    state,
                    diagnostics,
                );
                byte += 2;
                continue;
            }
            if starts_with_at(bytes, byte, br"\]") {
                close_command_math(
                    path,
                    line_no,
                    column,
                    "math/unmatched-bracket",
                    |mode| matches!(mode, MathMode::DisplayBracket { .. }),
                    state,
                    diagnostics,
                );
                byte += 2;
                continue;
            }
        }

        if bytes[byte] == b'$' && !is_escaped(bytes, byte) {
            if byte + 1 < bytes.len() && bytes[byte + 1] == b'$' {
                if config.prefer_bracket_display_math {
                    diagnostics.push(Diagnostic::warning(
                        path,
                        line_no,
                        column,
                        "math/display-dollar",
                        "display math uses $$ delimiter",
                        Some("Use \\[ ... \\] for LaTeX display math.".to_string()),
                    ));
                }
                toggle_dollar_math(
                    path,
                    line_no,
                    column,
                    MathMode::DisplayDollar {
                        line: line_no,
                        column,
                    },
                    state,
                    diagnostics,
                );
                byte += 2;
                continue;
            }
            if config.prefer_paren_inline_math {
                diagnostics.push(Diagnostic::warning(
                    path,
                    line_no,
                    column,
                    "math/inline-dollar",
                    "inline math uses $ delimiter",
                    Some("Use \\( ... \\) for LaTeX inline math.".to_string()),
                ));
            }
            toggle_dollar_math(
                path,
                line_no,
                column,
                MathMode::InlineDollar {
                    line: line_no,
                    column,
                },
                state,
                diagnostics,
            );
            byte += 1;
            continue;
        }

        if bytes[byte] == b'\''
            && config.prefer_prime_command
            && state.math_mode.as_ref().is_some_and(MathMode::is_math)
            && !is_escaped(bytes, byte)
        {
            diagnostics.push(Diagnostic::warning(
                path,
                line_no,
                column,
                "math/prime-command",
                "ASCII prime mark used in math mode",
                Some(
                    "Use ^{\\prime} or a project-approved prime macro for explicitness."
                        .to_string(),
                ),
            ));
        }

        byte += char_width_at(line, byte);
    }
}

fn open_command_math(
    path: &Path,
    line: usize,
    column: usize,
    mode: MathMode,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if state.math_mode.is_some() {
        diagnostics.push(Diagnostic::error(
            path,
            line,
            column,
            "math/nested",
            "new math delimiter opened before closing the previous math span",
            Some("Close the current math span before opening another one.".to_string()),
        ));
    } else {
        state.math_mode = Some(mode);
    }
}

fn close_command_math<F>(
    path: &Path,
    line: usize,
    column: usize,
    rule: &'static str,
    matches_open: F,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) where
    F: FnOnce(&MathMode) -> bool,
{
    match state.math_mode.take() {
        Some(mode) if matches_open(&mode) => {
            flush_unclosed_nested_math_environments(path, state, diagnostics);
            flush_unclosed_math_fences(path, state, diagnostics);
        }
        Some(mode) => {
            state.math_mode = Some(mode);
            diagnostics.push(Diagnostic::error(
                path,
                line,
                column,
                rule,
                "math delimiter closes a different delimiter kind",
                Some("Use the delimiter matching the currently open math span.".to_string()),
            ));
        }
        None => diagnostics.push(Diagnostic::error(
            path,
            line,
            column,
            rule,
            "closing math delimiter has no opener",
            None,
        )),
    }
}

fn open_math_environment(
    _path: &Path,
    line: usize,
    column: usize,
    name: String,
    state: &mut LintState,
    _diagnostics: &mut Vec<Diagnostic>,
) {
    match &mut state.math_mode {
        None => {
            state.math_mode = Some(MathMode::Environment {
                stack: vec![name],
                line,
                column,
            });
        }
        Some(MathMode::Environment { stack, .. }) => {
            stack.push(name);
        }
        Some(_) => state
            .nested_math_env_stack
            .push(EnvFrame { name, line, column }),
    }
}

fn close_math_environment(
    path: &Path,
    line: usize,
    column: usize,
    name: &str,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match state.math_mode.take() {
        Some(MathMode::Environment {
            mut stack,
            line: start_line,
            column: start_column,
        }) => match stack.pop() {
            Some(open_name) if open_name == name => {
                if !stack.is_empty() {
                    state.math_mode = Some(MathMode::Environment {
                        stack,
                        line: start_line,
                        column: start_column,
                    });
                } else {
                    flush_unclosed_math_fences(path, state, diagnostics);
                }
            }
            Some(open_name) => {
                stack.push(open_name);
                state.math_mode = Some(MathMode::Environment {
                    stack,
                    line: start_line,
                    column: start_column,
                });
                diagnostics.push(Diagnostic::error(
                    path,
                    line,
                    column,
                    "math/mixed-delimiters",
                    format!("\\end{{{name}}} closes a different math environment"),
                    Some(
                        "Close the current math environment before ending another one.".to_string(),
                    ),
                ));
            }
            None => {
                state.math_mode = Some(MathMode::Environment {
                    stack,
                    line: start_line,
                    column: start_column,
                });
                diagnostics.push(Diagnostic::error(
                    path,
                    line,
                    column,
                    "math/unmatched-environment",
                    format!("\\end{{{name}}} has no matching math opener"),
                    None,
                ));
            }
        },
        Some(mode) => {
            state.math_mode = Some(mode);
            close_nested_math_environment(path, line, column, name, state, diagnostics);
        }
        None => diagnostics.push(Diagnostic::error(
            path,
            line,
            column,
            "math/unmatched-environment",
            format!("\\end{{{name}}} has no matching math opener"),
            None,
        )),
    }
}

fn toggle_dollar_math(
    path: &Path,
    line: usize,
    column: usize,
    opener: MathMode,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match (&state.math_mode, &opener) {
        (None, _) => state.math_mode = Some(opener),
        (Some(MathMode::InlineDollar { .. }), MathMode::InlineDollar { .. })
        | (Some(MathMode::DisplayDollar { .. }), MathMode::DisplayDollar { .. }) => {
            flush_unclosed_nested_math_environments(path, state, diagnostics);
            flush_unclosed_math_fences(path, state, diagnostics);
            state.math_mode = None;
        }
        (Some(_), _) => diagnostics.push(Diagnostic::error(
            path,
            line,
            column,
            "math/mixed-delimiters",
            "dollar delimiter appears inside another math span",
            Some("Close the current math span before changing delimiter styles.".to_string()),
        )),
    }
}

fn close_nested_math_environment(
    path: &Path,
    line: usize,
    column: usize,
    name: &str,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) {
    match state.nested_math_env_stack.pop() {
        Some(frame) if frame.name == name => {}
        Some(frame) => {
            diagnostics.push(Diagnostic::error(
                path,
                line,
                column,
                "math/mixed-delimiters",
                format!(
                    "\\end{{{name}}} closes \\begin{{{}}} from line {}",
                    frame.name, frame.line
                ),
                Some(format!(
                    "Use \\end{{{}}} or close '{}' first.",
                    frame.name, name
                )),
            ));
            state.nested_math_env_stack.push(frame);
        }
        None => diagnostics.push(Diagnostic::error(
            path,
            line,
            column,
            "math/unmatched-environment",
            format!("\\end{{{name}}} has no matching math opener"),
            None,
        )),
    }
}

fn flush_unclosed_nested_math_environments(
    path: &Path,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for frame in state.nested_math_env_stack.drain(..) {
        diagnostics.push(Diagnostic::error(
            path,
            frame.line,
            frame.column,
            "math/unclosed-environment",
            format!("math environment '{}' is not closed", frame.name),
            Some(format!(
                "Add \\end{{{}}} before closing the math span.",
                frame.name
            )),
        ));
    }
}

fn close_math_fence(
    path: &Path,
    line: usize,
    column: usize,
    right: MathFence,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(left) = state.math_fence_stack.pop() else {
        diagnostics.push(Diagnostic::error(
            path,
            line,
            column,
            "math/left-right",
            "\\right delimiter has no matching \\left",
            Some("Add a matching \\left delimiter or remove the \\right.".to_string()),
        ));
        return;
    };

    if !math_fences_match(&left.family, &right.family) {
        diagnostics.push(Diagnostic::error(
            path,
            line,
            column,
            "math/left-right",
            format!(
                "\\right{} does not match \\left{} from line {}",
                right.token, left.token, left.line
            ),
            Some("Use a matching \\right delimiter for the active \\left.".to_string()),
        ));
    }
}

fn flush_unclosed_math_fences(
    path: &Path,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for frame in state.math_fence_stack.drain(..) {
        diagnostics.push(Diagnostic::error(
            path,
            frame.line,
            frame.column,
            "math/left-right",
            format!("\\left{} has no matching \\right", frame.token),
            Some("Close the scalable delimiter with a matching \\right.".to_string()),
        ));
    }
}

fn math_fences_match(left: &MathFenceFamily, right: &MathFenceFamily) -> bool {
    matches!(left, MathFenceFamily::Invisible)
        || matches!(right, MathFenceFamily::Invisible)
        || left == right
}

fn math_fence_after_command(line: &str, start: usize) -> Option<(MathFence, usize)> {
    let mut byte = skip_ascii_whitespace(line, start);
    if byte >= line.len() {
        return None;
    }
    let token_start = byte;
    let token = if line.as_bytes()[byte] == b'\\' {
        byte += 1;
        if byte >= line.len() {
            return None;
        }
        if line.as_bytes()[byte].is_ascii_alphabetic() {
            while byte < line.len() && line.as_bytes()[byte].is_ascii_alphabetic() {
                byte += 1;
            }
        } else {
            byte += char_width_at(line, byte);
        }
        &line[token_start..byte]
    } else {
        byte += char_width_at(line, byte);
        &line[token_start..byte]
    };
    Some((
        MathFence {
            family: math_fence_family(token),
            token: token.to_string(),
        },
        byte,
    ))
}

fn math_fence_family(token: &str) -> MathFenceFamily {
    match token {
        "." => MathFenceFamily::Invisible,
        "(" | ")" => MathFenceFamily::Paren,
        "[" | "]" => MathFenceFamily::Bracket,
        r"\{" | r"\}" | r"\lbrace" | r"\rbrace" => MathFenceFamily::Brace,
        r"\langle" | r"\rangle" => MathFenceFamily::Angle,
        r"\lfloor" | r"\rfloor" => MathFenceFamily::Floor,
        r"\lceil" | r"\rceil" => MathFenceFamily::Ceiling,
        "|" | r"\vert" | r"\lvert" | r"\rvert" | r"\|" => MathFenceFamily::Vert,
        r"\Vert" | r"\lVert" | r"\rVert" => MathFenceFamily::DoubleVert,
        _ => MathFenceFamily::Other(token.to_string()),
    }
}

fn is_math_environment(name: &str) -> bool {
    matches!(
        name,
        "equation"
            | "equation*"
            | "align"
            | "align*"
            | "aligned"
            | "alignedat"
            | "alignat"
            | "alignat*"
            | "flalign"
            | "flalign*"
            | "gather"
            | "gather*"
            | "gathered"
            | "multline"
            | "multline*"
            | "split"
            | "matrix"
            | "pmatrix"
            | "bmatrix"
            | "Bmatrix"
            | "vmatrix"
            | "Vmatrix"
            | "smallmatrix"
            | "cases"
            | "dcases"
            | "array"
    )
}

fn lint_environments_and_indent(
    path: &Path,
    line: LintLine<'_>,
    line_no: usize,
    config: &LintConfig,
    ignored: &HashSet<&str>,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let trimmed = line.scan.trim_start_matches(' ');
    if trimmed.is_empty() {
        return;
    }

    let events = env_events(line.scan);
    let display_math_indent = display_math_indent_extra(line.math_mode_before, line.scan, config);
    if state
        .math_mode
        .as_ref()
        .is_some_and(|mode| !matches!(mode, MathMode::Environment { .. }))
        && events.is_empty()
        && display_math_indent.is_none()
    {
        return;
    }
    if config.indent_environments {
        let prefix_closes = leading_closing_indent_events(&events, ignored);
        let expected_level =
            state.indent_level.saturating_sub(prefix_closes) + display_math_indent.unwrap_or(0);
        let expected = expected_level * config.indent_size;
        let actual = line.source.len() - line.source.trim_start_matches(' ').len();
        if actual != expected {
            diagnostics.push(Diagnostic::warning(
                path,
                line_no,
                1,
                "indent/size",
                format!("expected {expected} leading spaces, found {actual}"),
                Some(format!(
                    "Indent nested environments by {} spaces.",
                    config.indent_size
                )),
            ));
        }
    }
    if state.math_mode.is_some() && events.is_empty() {
        return;
    }

    for event in events {
        match event.kind {
            EnvKind::Begin => {
                if config.check_environment_stack {
                    state.env_stack.push(EnvFrame {
                        name: event.name.clone(),
                        line: line_no,
                        column: event.column,
                    });
                }
                if config.indent_environments && !ignored.contains(event.name.as_str()) {
                    state.indent_level += 1;
                }
            }
            EnvKind::End => {
                if config.check_environment_stack {
                    match state.env_stack.pop() {
                        Some(frame) if frame.name == event.name => {}
                        Some(frame) => {
                            diagnostics.push(Diagnostic::error(
                                path,
                                line_no,
                                event.column,
                                "env/mismatch",
                                format!(
                                    "\\end{{{}}} closes \\begin{{{}}} from line {}",
                                    event.name, frame.name, frame.line
                                ),
                                Some(format!(
                                    "Use \\end{{{}}} or close '{}' first.",
                                    frame.name, event.name
                                )),
                            ));
                            state.env_stack.push(frame);
                        }
                        None => diagnostics.push(Diagnostic::error(
                            path,
                            line_no,
                            event.column,
                            "env/unmatched-end",
                            format!("\\end{{{}}} has no matching \\begin", event.name),
                            None,
                        )),
                    }
                }
                if config.indent_environments && !ignored.contains(event.name.as_str()) {
                    state.indent_level = state.indent_level.saturating_sub(1);
                }
            }
        }
    }
}

fn lint_verbatim_line(
    path: &Path,
    line: &str,
    line_no: usize,
    config: &LintConfig,
    ignored: &HashSet<&str>,
    state: &mut LintState,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(verbatim_env) = state.verbatim_env.clone() else {
        return;
    };
    if env_events(line)
        .iter()
        .any(|event| event.kind == EnvKind::End && event.name == verbatim_env)
    {
        lint_environments_and_indent(
            path,
            LintLine {
                source: line,
                scan: line,
                math_mode_before: None,
            },
            line_no,
            config,
            ignored,
            state,
            diagnostics,
        );
        state.verbatim_env = None;
    }
}

fn update_verbatim_state(line: &str, state: &mut LintState) {
    let mut active = None;
    for event in env_events(line) {
        match event.kind {
            EnvKind::Begin if is_verbatim_environment(&event.name) => {
                active = Some(event.name);
            }
            EnvKind::End if active.as_deref() == Some(event.name.as_str()) => {
                active = None;
            }
            _ => {}
        }
    }
    if active.is_some() {
        state.verbatim_env = active;
    }
}

fn is_verbatim_environment(name: &str) -> bool {
    matches!(
        name,
        "verbatim"
            | "verbatim*"
            | "Verbatim"
            | "BVerbatim"
            | "LVerbatim"
            | "lstlisting"
            | "minted"
            | "filecontents"
            | "filecontents*"
            | "comment"
            | "tcblisting"
    )
}

fn display_math_indent_extra(
    math_mode_before: Option<&MathMode>,
    line: &str,
    config: &LintConfig,
) -> Option<usize> {
    if !config.indent_display_math {
        return None;
    }
    match math_mode_before {
        Some(MathMode::DisplayBracket { .. }) => {
            Some(usize::from(!line_starts_with_display_bracket_close(line)))
        }
        Some(MathMode::DisplayDollar { .. }) => {
            Some(usize::from(!line_starts_with_display_dollar(line)))
        }
        _ => {
            if line_starts_with_display_bracket_open(line) || line_starts_with_display_dollar(line)
            {
                Some(0)
            } else {
                None
            }
        }
    }
}

fn line_starts_with_display_bracket_open(line: &str) -> bool {
    line.trim_start_matches(' ').starts_with(r"\[")
}

fn line_starts_with_display_bracket_close(line: &str) -> bool {
    line.trim_start_matches(' ').starts_with(r"\]")
}

fn line_starts_with_display_dollar(line: &str) -> bool {
    let trimmed = line.trim_start_matches(' ');
    trimmed.starts_with("$$")
}

fn mask_inline_verbatim(line: &str) -> String {
    let mut chars: Vec<char> = line.chars().collect();
    let mut index = 0;
    while index < chars.len() {
        if chars[index] == '\\'
            && !is_escaped_chars(&chars, index)
            && let Some(end) = inline_verbatim_span_end(&chars, index)
        {
            for ch in &mut chars[index..end] {
                *ch = ' ';
            }
            index = end;
            continue;
        }
        index += 1;
    }
    chars.into_iter().collect()
}

fn inline_verbatim_span_end(chars: &[char], start: usize) -> Option<usize> {
    if starts_with_command_chars(chars, start, r"\verb") {
        let mut delimiter = start + r"\verb".chars().count();
        if chars.get(delimiter) == Some(&'*') {
            delimiter += 1;
        }
        return delimited_inline_span_end(chars, delimiter);
    }

    if starts_with_command_chars(chars, start, r"\lstinline") {
        let mut delimiter = start + r"\lstinline".chars().count();
        delimiter = skip_ascii_whitespace_chars(chars, delimiter);
        if chars.get(delimiter) == Some(&'[') {
            delimiter = bracketed_chars_end(chars, delimiter)?;
            delimiter = skip_ascii_whitespace_chars(chars, delimiter);
        }
        return delimited_inline_span_end(chars, delimiter);
    }

    for command in [r"\mintinline", r"\mint"] {
        if starts_with_command_chars(chars, start, command) {
            let mut delimiter = start + command.chars().count();
            delimiter = skip_ascii_whitespace_chars(chars, delimiter);
            if chars.get(delimiter) == Some(&'[') {
                delimiter = bracketed_chars_end(chars, delimiter)?;
                delimiter = skip_ascii_whitespace_chars(chars, delimiter);
            }
            delimiter = braced_chars_end(chars, delimiter)?;
            delimiter = skip_ascii_whitespace_chars(chars, delimiter);
            return delimited_inline_span_end(chars, delimiter);
        }
    }

    None
}

fn delimited_inline_span_end(chars: &[char], delimiter_index: usize) -> Option<usize> {
    let delimiter = *chars.get(delimiter_index)?;
    if delimiter.is_ascii_alphanumeric() || delimiter == '\\' {
        return None;
    }
    chars[delimiter_index + 1..]
        .iter()
        .position(|ch| *ch == delimiter)
        .map_or(Some(chars.len()), |offset| {
            Some(delimiter_index + 1 + offset + 1)
        })
}

fn starts_with_command_chars(chars: &[char], index: usize, command: &str) -> bool {
    let mut cursor = index;
    for expected in command.chars() {
        if chars.get(cursor) != Some(&expected) {
            return false;
        }
        cursor += 1;
    }
    chars.get(cursor).is_none_or(|ch| !ch.is_ascii_alphabetic())
}

fn skip_ascii_whitespace_chars(chars: &[char], mut index: usize) -> usize {
    while chars.get(index).is_some_and(|ch| ch.is_ascii_whitespace()) {
        index += 1;
    }
    index
}

fn bracketed_chars_end(chars: &[char], open: usize) -> Option<usize> {
    balanced_chars_end(chars, open, '[', ']')
}

fn braced_chars_end(chars: &[char], open: usize) -> Option<usize> {
    balanced_chars_end(chars, open, '{', '}')
}

fn balanced_chars_end(chars: &[char], open: usize, left: char, right: char) -> Option<usize> {
    if chars.get(open) != Some(&left) {
        return None;
    }
    let mut depth = 0_u32;
    for (index, ch) in chars.iter().enumerate().skip(open) {
        if *ch == left && !is_escaped_chars(chars, index) {
            depth += 1;
        } else if *ch == right && !is_escaped_chars(chars, index) {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(index + 1);
            }
        }
    }
    None
}

fn leading_closing_indent_events(events: &[EnvEvent], ignored: &HashSet<&str>) -> usize {
    events
        .iter()
        .take_while(|event| event.kind == EnvKind::End && !ignored.contains(event.name.as_str()))
        .count()
}

fn env_events(line: &str) -> Vec<EnvEvent> {
    let bytes = line.as_bytes();
    let mut events = Vec::new();
    let mut byte = 0;
    while byte < bytes.len() {
        if bytes[byte] == b'\\' && !is_escaped(bytes, byte) {
            if starts_with_at(bytes, byte, br"\begin{")
                && let Some((name, end)) = read_braced_name(line, byte + br"\begin{".len())
            {
                events.push(EnvEvent {
                    kind: EnvKind::Begin,
                    name,
                    column: column_for_byte(line, byte),
                });
                byte = end;
                continue;
            }
            if starts_with_at(bytes, byte, br"\end{")
                && let Some((name, end)) = read_braced_name(line, byte + br"\end{".len())
            {
                events.push(EnvEvent {
                    kind: EnvKind::End,
                    name,
                    column: column_for_byte(line, byte),
                });
                byte = end;
                continue;
            }
        }
        byte += char_width_at(line, byte);
    }
    events
}

fn read_braced_name(line: &str, start: usize) -> Option<(String, usize)> {
    let rest = line.get(start..)?;
    let close = rest.find('}')?;
    let name = &rest[..close];
    if name.is_empty() {
        return None;
    }
    Some((name.to_string(), start + close + 1))
}

fn math_text_command_payload_end(line: &str, command_start: usize) -> Option<usize> {
    let commands = [
        r"\text",
        r"\textnormal",
        r"\textrm",
        r"\textit",
        r"\textbf",
        r"\textsf",
        r"\texttt",
        r"\mbox",
        r"\hbox",
    ];
    for command in commands {
        let command_bytes = command.as_bytes();
        let bytes = line.as_bytes();
        if !starts_with_at(bytes, command_start, command_bytes) {
            continue;
        }
        let after_command = command_start + command_bytes.len();
        if line[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            continue;
        }
        let payload_start = skip_ascii_whitespace(line, after_command);
        return balanced_braced_end(line, payload_start);
    }
    None
}

fn skip_ascii_whitespace(line: &str, mut byte: usize) -> usize {
    while line[byte..]
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_whitespace())
    {
        byte += char_width_at(line, byte);
    }
    byte
}

fn balanced_braced_end(line: &str, open: usize) -> Option<usize> {
    if !line[open..].starts_with('{') {
        return None;
    }
    let bytes = line.as_bytes();
    let mut depth = 0_u32;
    let mut byte = open;
    while byte < bytes.len() {
        match bytes[byte] {
            b'{' if !is_escaped(bytes, byte) => depth += 1,
            b'}' if !is_escaped(bytes, byte) => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(byte + 1);
                }
            }
            _ => {}
        }
        byte += char_width_at(line, byte);
    }
    None
}

fn strip_comment(line: &str) -> &str {
    comment_start_byte(line).map_or(line, |byte| &line[..byte])
}

fn tex_comment_text(line: &str) -> Option<&str> {
    let byte = comment_start_byte(line)?;
    line.get(byte + 1..)
}

fn comment_start_byte(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut byte = 0;
    while byte < bytes.len() {
        if bytes[byte] == b'%' && !is_escaped(bytes, byte) {
            return Some(byte);
        }
        byte += char_width_at(line, byte);
    }
    None
}

fn starts_with_at(haystack: &[u8], index: usize, needle: &[u8]) -> bool {
    haystack
        .get(index..index + needle.len())
        .is_some_and(|slice| slice == needle)
}

fn starts_with_latex_command(haystack: &[u8], index: usize, command: &[u8]) -> bool {
    starts_with_at(haystack, index, command)
        && haystack
            .get(index + command.len())
            .is_none_or(|byte| !byte.is_ascii_alphabetic())
}

fn is_escaped(bytes: &[u8], index: usize) -> bool {
    let mut count = 0;
    let mut cursor = index;
    while cursor > 0 && bytes[cursor - 1] == b'\\' {
        count += 1;
        cursor -= 1;
    }
    count % 2 == 1
}

fn is_escaped_chars(chars: &[char], index: usize) -> bool {
    let mut count = 0;
    let mut cursor = index;
    while cursor > 0 && chars[cursor - 1] == '\\' {
        count += 1;
        cursor -= 1;
    }
    count % 2 == 1
}

fn char_width_at(line: &str, byte: usize) -> usize {
    line[byte..].chars().next().map(char::len_utf8).unwrap_or(1)
}

fn column_for_byte(line: &str, byte: usize) -> usize {
    line[..byte].chars().count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catches_dollar_math_and_ascii_prime() {
        let config = LintConfig {
            prefer_prime_command: true,
            ..LintConfig::default()
        };
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            r"Let $f'(x)$ and $$x^2$$ be old style.",
            &config,
        );
        assert!(diagnostics.iter().any(|d| d.rule == "math/inline-dollar"));
        assert!(diagnostics.iter().any(|d| d.rule == "math/display-dollar"));
        assert!(diagnostics.iter().any(|d| d.rule == "math/prime-command"));
    }

    #[test]
    fn catches_ascii_prime_inside_math_environments() {
        let config = LintConfig {
            prefer_prime_command: true,
            ..LintConfig::default()
        };
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{align}\n  f'(x) &= x\n\\end{align}\n\\begin{equation}g'(x)=1\\end{equation}\n",
            &config,
        );

        assert_eq!(
            diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.rule == "math/prime-command")
                .count(),
            2,
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn checks_indentation_inside_math_environments() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{align}\nx &= y\n\\end{align}\n",
            &LintConfig::default(),
        );

        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic.rule == "indent/size"
                    && diagnostic.line == 2
                    && diagnostic.message.contains("expected 2 leading spaces")
            }),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn accepts_indentation_inside_math_environments() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{align}\n  x &= y\n\\end{align}\n",
            &LintConfig::default(),
        );

        assert!(
            !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "indent/size"),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn checks_indentation_inside_display_math_delimiters() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\[\nx = y\n\\]\n",
            &LintConfig::default(),
        );

        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic.rule == "indent/size"
                    && diagnostic.line == 2
                    && diagnostic.message.contains("expected 2 leading spaces")
            }),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn accepts_indentation_inside_display_math_delimiters() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{theorem}\n  \\[\n    x = y\n  \\]\n\\end{theorem}\n",
            &LintConfig::default(),
        );

        assert!(
            !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "indent/size"),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn display_math_indentation_can_be_disabled() {
        let config = LintConfig {
            indent_display_math: false,
            ..LintConfig::default()
        };
        let diagnostics = lint_source(Path::new("sample.tex"), "\\[\nx = y\n\\]\n", &config);

        assert!(
            !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "indent/size"),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn checks_display_math_closer_indentation() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\[\n  x = y\n  \\]\n",
            &LintConfig::default(),
        );

        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic.rule == "indent/size"
                    && diagnostic.line == 3
                    && diagnostic.message.contains("expected 0 leading spaces")
            }),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn allows_aligned_environment_inside_display_math_delimiters() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\[\n  \\begin{aligned}\n    x &= y\n  \\end{aligned}\n\\]\n",
            &LintConfig::default(),
        );

        assert!(
            !diagnostics.iter().any(|diagnostic| matches!(
                diagnostic.rule,
                "indent/size" | "math/nested" | "math/mixed-delimiters"
            )),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn catches_unclosed_nested_math_environment_inside_display_math() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\[\n  \\begin{aligned}\n    x &= y\n\\]\n",
            &LintConfig::default(),
        );

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/unclosed-environment"
                    && diagnostic.line == 2),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn ignores_ascii_apostrophe_inside_math_text_commands() {
        let config = LintConfig {
            prefer_prime_command: true,
            ..LintConfig::default()
        };
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            r"\( \text{Alice's note} + f'(x) + \mbox{Bob's label} \)",
            &config,
        );

        assert_eq!(
            diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.rule == "math/prime-command")
                .count(),
            1,
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn does_not_treat_text_environments_as_math() {
        let config = LintConfig {
            prefer_prime_command: true,
            ..LintConfig::default()
        };
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{quote}\nAlice's text.\n\\end{quote}\n",
            &config,
        );

        assert!(
            !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/prime-command"),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn ignores_math_and_environment_syntax_inside_verbatim_blocks() {
        let config = LintConfig {
            prefer_prime_command: true,
            ..LintConfig::default()
        };
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{verbatim}\n\
             $f'(x)$ and \\begin{proof}\n\
             \\end{verbatim}\n\
             \\begin{lstlisting}\n\
             $$y$$ and \\end{theorem}\n\
             \\end{lstlisting}\n",
            &config,
        );

        assert!(
            !diagnostics.iter().any(|diagnostic| matches!(
                diagnostic.rule,
                "math/inline-dollar"
                    | "math/display-dollar"
                    | "math/prime-command"
                    | "env/mismatch"
                    | "env/unclosed"
                    | "env/unmatched-end"
            )),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn ignores_math_and_environment_syntax_inside_inline_verbatim() {
        let config = LintConfig {
            prefer_prime_command: true,
            ..LintConfig::default()
        };
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            r"\verb|$f'(x)$ \begin{proof}| and \lstinline!\end{theorem} $$x$$! and \mintinline{tex}|\[x'\]|",
            &config,
        );

        assert!(
            !diagnostics.iter().any(|diagnostic| matches!(
                diagnostic.rule,
                "math/inline-dollar"
                    | "math/display-dollar"
                    | "math/prime-command"
                    | "env/mismatch"
                    | "env/unclosed"
                    | "env/unmatched-end"
            )),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn inline_verbatim_percent_does_not_hide_later_lintable_text() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            r"Literal \verb|% not a comment| then $x$.",
            &LintConfig::default(),
        );

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/inline-dollar"),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn suppression_comments_can_target_specific_rules() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "% texpilot-ignore-next-line math/inline-dollar\n\
             Text $x$ and $$y$$.\n",
            &LintConfig::default(),
        );

        assert!(
            !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/inline-dollar"),
            "{diagnostics:#?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/display-dollar"),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn suppression_comments_without_rules_suppress_the_target_line() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "Text $x$ and $$y$$. % texpilot-ignore-line\n",
            &LintConfig::default(),
        );

        assert!(diagnostics.is_empty(), "{diagnostics:#?}");
    }

    #[test]
    fn suppression_comments_are_ignored_inside_inline_verbatim() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            r"\verb|% texpilot-ignore-line| then $x$.",
            &LintConfig::default(),
        );

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/inline-dollar"),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn rule_levels_can_disable_promote_and_downgrade_diagnostics() {
        let config = LintConfig {
            rule_levels: HashMap::from([
                ("math/inline-dollar".to_string(), RuleLevel::Off),
                ("math/display-dollar".to_string(), RuleLevel::Error),
                ("env/unclosed".to_string(), RuleLevel::Warning),
            ]),
            ..LintConfig::default()
        };
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{proof}\nText $x$ and $$y$$.\n",
            &config,
        );

        assert!(
            !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/inline-dollar"),
            "{diagnostics:#?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/display-dollar"
                    && diagnostic.severity == Severity::Error),
            "{diagnostics:#?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "env/unclosed"
                    && diagnostic.severity == Severity::Warning),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn allows_nested_math_environments() {
        let config = LintConfig {
            prefer_prime_command: true,
            ..LintConfig::default()
        };
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{equation}\n\\begin{split}\n  f'(x) &= x\n\\end{split}\n\\end{equation}\n",
            &config,
        );

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/prime-command"),
            "{diagnostics:#?}"
        );
        assert!(
            !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/nested"
                    || diagnostic.rule == "math/mixed-delimiters"),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn checks_left_right_delimiter_pairs_inside_math() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            r"\[ \left( x + \left\langle y \right\rangle \right) \]",
            &LintConfig::default(),
        );

        assert!(
            !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/left-right"),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn catches_mismatched_left_right_delimiters() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            r"\( \left( x \right] \)",
            &LintConfig::default(),
        );

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/left-right"
                    && diagnostic.message.contains("\\right]")),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn catches_unmatched_right_delimiter() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            r"\( x \right) \)",
            &LintConfig::default(),
        );

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/left-right"
                    && diagnostic.message.contains("no matching \\left")),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn catches_unclosed_left_delimiter_at_math_close() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{equation}\n  \\left( x\n\\end{equation}\n",
            &LintConfig::default(),
        );

        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "math/left-right"
                    && diagnostic.line == 2
                    && diagnostic.message.contains("has no matching \\right")),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn catches_environment_mismatch_and_indent() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{proof}\n\\begin{align}\n  x &= y\n\\end{proof}\n",
            &LintConfig::default(),
        );
        assert!(diagnostics.iter().any(|d| d.rule == "indent/size"));
        assert!(diagnostics.iter().any(|d| d.rule == "env/mismatch"));
    }

    #[test]
    fn environment_stack_check_can_be_disabled_without_disabling_indentation() {
        let config = LintConfig {
            check_environment_stack: false,
            ..LintConfig::default()
        };
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            "\\begin{proof}\nBad indent.\n\\end{quote}\n",
            &config,
        );

        assert!(
            !diagnostics.iter().any(|diagnostic| matches!(
                diagnostic.rule,
                "env/mismatch" | "env/unclosed" | "env/unmatched-end"
            )),
            "{diagnostics:#?}"
        );
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.rule == "indent/size"),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn ignores_escaped_dollars_and_comments() {
        let diagnostics = lint_source(
            Path::new("sample.tex"),
            r"Cost is \$5. % $not math$",
            &LintConfig::default(),
        );
        assert!(diagnostics.is_empty());
    }
}

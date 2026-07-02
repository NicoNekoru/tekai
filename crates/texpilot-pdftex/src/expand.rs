use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Local};
use md5::{Digest, Md5};

use crate::token::{CatCode, CatCodeTable, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandError {
    message: String,
}

impl ExpandError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ExpandError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ExpandError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroDefinition {
    arity: usize,
    optional_default: Option<Vec<Token>>,
    replacement: Vec<Token>,
    protected: bool,
}

impl MacroDefinition {
    pub fn new(arity: usize, replacement: Vec<Token>) -> Self {
        Self {
            arity,
            optional_default: None,
            replacement,
            protected: false,
        }
    }

    pub fn with_optional_default(
        arity: usize,
        optional_default: Vec<Token>,
        replacement: Vec<Token>,
    ) -> Self {
        Self {
            arity,
            optional_default: Some(optional_default),
            replacement,
            protected: false,
        }
    }

    pub fn protected(mut self) -> Self {
        self.protected = true;
        self
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn optional_default(&self) -> Option<&[Token]> {
        self.optional_default.as_deref()
    }

    pub fn replacement(&self) -> &[Token] {
        &self.replacement
    }

    pub fn is_protected(&self) -> bool {
        self.protected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DefinitionPolicy {
    Always,
    IfUndefined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReplacementExpansion {
    Deferred,
    ExpandNow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AssignmentScope {
    Local,
    Global,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumericRelation {
    Less,
    Equal,
    Greater,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CountRegisterKey {
    Named(String),
    Numbered(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DimensionRegisterKey {
    Named(String),
    Numbered(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SkipRegisterKey {
    Named(String),
    Numbered(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TokenRegisterKey {
    Named(String),
    Numbered(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GlueValue {
    width: i64,
    stretch: i64,
    shrink: i64,
}

impl GlueValue {
    fn add(&self, other: &Self) -> Result<Self, ExpandError> {
        Ok(Self {
            width: self
                .width
                .checked_add(other.width)
                .ok_or_else(|| ExpandError::new("glue width is too large"))?,
            stretch: self
                .stretch
                .checked_add(other.stretch)
                .ok_or_else(|| ExpandError::new("glue stretch is too large"))?,
            shrink: self
                .shrink
                .checked_add(other.shrink)
                .ok_or_else(|| ExpandError::new("glue shrink is too large"))?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RegisterTarget {
    Count(CountRegisterKey),
    Dimension(DimensionRegisterKey),
    Skip(SkipRegisterKey),
}

#[derive(Debug, Clone)]
struct ExpansionFileContext {
    root_dir: PathBuf,
    job_name: String,
}

#[derive(Debug, Clone)]
pub struct ExpansionEngine<'a> {
    tokenizer: StreamingTokenizer<'a>,
    pending: Vec<Token>,
    macros: HashMap<String, MacroDefinition>,
    conditionals: HashMap<String, bool>,
    count_registers: HashMap<CountRegisterKey, i64>,
    count_aliases: HashMap<String, CountRegisterKey>,
    dimension_registers: HashMap<DimensionRegisterKey, i64>,
    dimension_aliases: HashMap<String, DimensionRegisterKey>,
    skip_registers: HashMap<SkipRegisterKey, GlueValue>,
    skip_aliases: HashMap<String, SkipRegisterKey>,
    token_registers: HashMap<TokenRegisterKey, Vec<Token>>,
    token_aliases: HashMap<String, TokenRegisterKey>,
    integer_constants: HashMap<String, i64>,
    next_count_register: i64,
    next_dimension_register: i64,
    next_skip_register: i64,
    next_token_register: i64,
    scopes: Vec<ExpansionScopeSnapshot>,
    file_context: Option<ExpansionFileContext>,
    creation_time: SystemTime,
}

#[derive(Debug, Clone)]
struct ExpansionScopeSnapshot {
    state: Option<ExpansionStateSnapshot>,
    aftergroup_tokens: Vec<Token>,
}

#[derive(Debug, Clone)]
struct ExpansionStateSnapshot {
    macros: HashMap<String, MacroDefinition>,
    conditionals: HashMap<String, bool>,
    count_registers: HashMap<CountRegisterKey, i64>,
    count_aliases: HashMap<String, CountRegisterKey>,
    dimension_registers: HashMap<DimensionRegisterKey, i64>,
    dimension_aliases: HashMap<String, DimensionRegisterKey>,
    skip_registers: HashMap<SkipRegisterKey, GlueValue>,
    skip_aliases: HashMap<String, SkipRegisterKey>,
    token_registers: HashMap<TokenRegisterKey, Vec<Token>>,
    token_aliases: HashMap<String, TokenRegisterKey>,
    integer_constants: HashMap<String, i64>,
    next_count_register: i64,
    next_dimension_register: i64,
    next_skip_register: i64,
    next_token_register: i64,
    catcodes: CatCodeTable,
}

#[derive(Debug, Clone)]
struct ConditionalBranches {
    then_branch: Vec<Token>,
    else_branch: Vec<Token>,
}

#[derive(Debug, Clone)]
struct CaseBranches {
    branches: Vec<Vec<Token>>,
    else_branch: Vec<Token>,
}

impl<'a> ExpansionEngine<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            tokenizer: StreamingTokenizer::new(source),
            pending: Vec::new(),
            macros: HashMap::new(),
            conditionals: HashMap::new(),
            count_registers: default_count_registers(),
            count_aliases: default_count_aliases(),
            dimension_registers: default_dimension_registers(),
            dimension_aliases: default_dimension_aliases(),
            skip_registers: default_skip_registers(),
            skip_aliases: default_skip_aliases(),
            token_registers: default_token_registers(),
            token_aliases: default_token_aliases(),
            integer_constants: default_integer_constants(),
            next_count_register: FIRST_ALLOCATED_COUNT_REGISTER,
            next_dimension_register: FIRST_ALLOCATED_DIMENSION_REGISTER,
            next_skip_register: FIRST_ALLOCATED_SKIP_REGISTER,
            next_token_register: FIRST_ALLOCATED_TOKEN_REGISTER,
            scopes: Vec::new(),
            file_context: None,
            creation_time: SystemTime::now(),
        }
    }

    pub fn with_catcodes(source: &'a str, catcodes: CatCodeTable) -> Self {
        Self {
            tokenizer: StreamingTokenizer::with_catcodes(source, catcodes),
            pending: Vec::new(),
            macros: HashMap::new(),
            conditionals: HashMap::new(),
            count_registers: default_count_registers(),
            count_aliases: default_count_aliases(),
            dimension_registers: default_dimension_registers(),
            dimension_aliases: default_dimension_aliases(),
            skip_registers: default_skip_registers(),
            skip_aliases: default_skip_aliases(),
            token_registers: default_token_registers(),
            token_aliases: default_token_aliases(),
            integer_constants: default_integer_constants(),
            next_count_register: FIRST_ALLOCATED_COUNT_REGISTER,
            next_dimension_register: FIRST_ALLOCATED_DIMENSION_REGISTER,
            next_skip_register: FIRST_ALLOCATED_SKIP_REGISTER,
            next_token_register: FIRST_ALLOCATED_TOKEN_REGISTER,
            scopes: Vec::new(),
            file_context: None,
            creation_time: SystemTime::now(),
        }
    }

    pub fn with_file_context(
        source: &'a str,
        root_dir: impl Into<PathBuf>,
        job_name: impl Into<String>,
    ) -> Self {
        let mut engine = Self::new(source);
        let root_dir = root_dir.into();
        let root_dir = fs::canonicalize(&root_dir).unwrap_or(root_dir);
        engine.file_context = Some(ExpansionFileContext {
            root_dir,
            job_name: job_name.into(),
        });
        engine
    }

    pub fn define_macro(&mut self, name: impl Into<String>, definition: MacroDefinition) {
        self.define_macro_with_scope(name, definition, AssignmentScope::Local);
    }

    pub fn macro_definition(&self, name: &str) -> Option<&MacroDefinition> {
        self.macros.get(name)
    }

    pub fn expand_all(&mut self) -> Result<Vec<Token>, ExpandError> {
        let mut expanded = Vec::new();
        while let Some(token) = self.next_unexpanded() {
            match token {
                Token::ControlSequence(name) if name == "def" => {
                    self.read_tex_definition(
                        "\\def",
                        ReplacementExpansion::Deferred,
                        AssignmentScope::Local,
                        false,
                    )?;
                }
                Token::ControlSequence(name) if name == "gdef" => {
                    self.read_tex_definition(
                        "\\gdef",
                        ReplacementExpansion::Deferred,
                        AssignmentScope::Global,
                        false,
                    )?;
                }
                Token::ControlSequence(name) if name == "edef" => {
                    self.read_tex_definition(
                        "\\edef",
                        ReplacementExpansion::ExpandNow,
                        AssignmentScope::Local,
                        false,
                    )?;
                }
                Token::ControlSequence(name) if name == "xdef" => {
                    self.read_tex_definition(
                        "\\xdef",
                        ReplacementExpansion::ExpandNow,
                        AssignmentScope::Global,
                        false,
                    )?;
                }
                Token::ControlSequence(name) if name == "protected@edef" => {
                    self.read_tex_definition(
                        "\\protected@edef",
                        ReplacementExpansion::ExpandNow,
                        AssignmentScope::Local,
                        false,
                    )?;
                }
                Token::ControlSequence(name) if name == "protected" => {
                    self.read_protected_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "newcommand" => {
                    self.read_latex_command_definition(
                        "\\newcommand",
                        DefinitionPolicy::Always,
                        AssignmentScope::Local,
                        false,
                    )?;
                }
                Token::ControlSequence(name) if name == "renewcommand" => {
                    self.read_latex_command_definition(
                        "\\renewcommand",
                        DefinitionPolicy::Always,
                        AssignmentScope::Local,
                        false,
                    )?;
                }
                Token::ControlSequence(name) if name == "providecommand" => {
                    self.read_latex_command_definition(
                        "\\providecommand",
                        DefinitionPolicy::IfUndefined,
                        AssignmentScope::Local,
                        false,
                    )?;
                }
                Token::ControlSequence(name) if name == "DeclareRobustCommand" => {
                    self.read_latex_command_definition(
                        "\\DeclareRobustCommand",
                        DefinitionPolicy::Always,
                        AssignmentScope::Local,
                        true,
                    )?;
                }
                Token::ControlSequence(name) if name == "DeclareMathOperator" => {
                    self.read_declare_math_operator_definition(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "csname" => {
                    let token = self.read_csname_control_sequence()?;
                    if let Token::ControlSequence(name) = token.clone() {
                        self.expand_or_emit_macro_token(&mut expanded, name, token)?;
                    }
                }
                Token::ControlSequence(name) if name == "expandafter" => {
                    let tokens = self.expand_after_once()?;
                    self.push_front(tokens);
                }
                Token::ControlSequence(name) if name == "futurelet" => {
                    self.read_futurelet_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "aftergroup" => {
                    self.read_aftergroup_assignment()?;
                }
                Token::ControlSequence(name) if name == "relax" => {}
                Token::ControlSequence(name) if name == "iftrue" => {
                    self.read_conditional(true)?;
                }
                Token::ControlSequence(name) if name == "iffalse" => {
                    self.read_conditional(false)?;
                }
                Token::ControlSequence(name) if name == "ifx" => {
                    let condition = self.read_ifx_condition()?;
                    self.read_conditional(condition)?;
                }
                Token::ControlSequence(name) if name == "if" => {
                    let condition = self.read_if_condition()?;
                    self.read_conditional(condition)?;
                }
                Token::ControlSequence(name) if name == "ifcat" => {
                    let condition = self.read_ifcat_condition()?;
                    self.read_conditional(condition)?;
                }
                Token::ControlSequence(name) if name == "ifdefined" => {
                    let condition = self.read_ifdefined_condition()?;
                    self.read_conditional(condition)?;
                }
                Token::ControlSequence(name) if name == "ifcsname" => {
                    let condition = self.read_ifcsname_condition()?;
                    self.read_conditional(condition)?;
                }
                Token::ControlSequence(name) if name == "ifpdfprimitive" => {
                    let condition = self.read_ifpdfprimitive_condition()?;
                    self.read_conditional(condition)?;
                }
                Token::ControlSequence(name) if name == "ifnum" => {
                    let condition = self.read_ifnum_condition()?;
                    self.read_conditional(condition)?;
                }
                Token::ControlSequence(name) if name == "ifdim" => {
                    let condition = self.read_ifdim_condition()?;
                    self.read_conditional(condition)?;
                }
                Token::ControlSequence(name) if name == "ifodd" => {
                    let condition = self.read_integer()? % 2 != 0;
                    self.skip_spaces();
                    self.read_conditional(condition)?;
                }
                Token::ControlSequence(name) if name == "ifcase" => {
                    let case = self.read_integer()?;
                    self.skip_spaces();
                    self.read_case_conditional(case)?;
                }
                Token::ControlSequence(name) if name == "unless" => {
                    self.read_unless_conditional()?;
                }
                Token::ControlSequence(name) if name == "newif" => {
                    self.read_newif_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "newcount" => {
                    self.read_newcount_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "countdef" => {
                    self.read_countdef_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "newdimen" => {
                    self.read_newdimen_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "dimendef" => {
                    self.read_dimendef_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "newskip" => {
                    self.read_newskip_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "skipdef" => {
                    self.read_skipdef_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "newtoks" => {
                    self.read_newtoks_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "toksdef" => {
                    self.read_toksdef_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "chardef" || name == "mathchardef" => {
                    self.read_integer_constant_assignment(
                        &format!("\\{name}"),
                        AssignmentScope::Local,
                    )?;
                }
                Token::ControlSequence(name) if name == "advance" => {
                    self.read_advance_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "number" => {
                    let value = self.read_integer()?;
                    expanded.extend(integer_tokens(value));
                }
                Token::ControlSequence(name) if name == "romannumeral" => {
                    let value = self.read_integer()?;
                    expanded.extend(roman_numeral_tokens(value));
                }
                Token::ControlSequence(name) if name == "string" => {
                    expanded.extend(self.read_string_primitive_tokens()?);
                }
                Token::ControlSequence(name) if name == "meaning" => {
                    expanded.extend(self.read_meaning_primitive_tokens()?);
                }
                Token::ControlSequence(name) if name == "detokenize" => {
                    expanded.extend(self.read_detokenize_primitive_tokens()?);
                }
                Token::ControlSequence(name) if name == "unexpanded" => {
                    expanded.extend(self.read_unexpanded_primitive_tokens()?);
                }
                Token::ControlSequence(name) if name == "expanded" => {
                    let tokens = self.read_expanded_primitive_tokens()?;
                    self.push_front(tokens);
                }
                Token::ControlSequence(name) if name == "noexpand" => {
                    expanded.extend(self.read_noexpand_primitive_tokens()?);
                }
                Token::ControlSequence(name) if name == "jobname" => {
                    expanded.extend(self.jobname_tokens());
                }
                Token::ControlSequence(name) if name == "pdfprimitive" => {
                    expanded.extend(self.read_pdfprimitive_tokens()?);
                }
                Token::ControlSequence(name) if name == "pdfcreationdate" => {
                    expanded.extend(tokens_from_ascii_other(&pdf_date_string(
                        self.creation_time,
                    )));
                }
                Token::ControlSequence(name) if name == "pdffilesize" => {
                    expanded.extend(self.read_pdffilesize_tokens()?);
                }
                Token::ControlSequence(name) if name == "pdffilemoddate" => {
                    expanded.extend(self.read_pdffilemoddate_tokens()?);
                }
                Token::ControlSequence(name) if name == "pdffiledump" => {
                    expanded.extend(self.read_pdffiledump_tokens()?);
                }
                Token::ControlSequence(name) if name == "pdfstrcmp" => {
                    expanded.extend(self.read_pdfstrcmp_tokens()?);
                }
                Token::ControlSequence(name) if name == "pdfescapehex" => {
                    expanded.extend(self.read_pdfescapehex_tokens()?);
                }
                Token::ControlSequence(name) if name == "pdfunescapehex" => {
                    expanded.extend(self.read_pdfunescapehex_tokens()?);
                }
                Token::ControlSequence(name) if name == "pdfescapestring" => {
                    expanded.extend(self.read_pdfescapestring_tokens()?);
                }
                Token::ControlSequence(name) if name == "pdfescapename" => {
                    expanded.extend(self.read_pdfescapename_tokens()?);
                }
                Token::ControlSequence(name) if name == "pdfmdfivesum" => {
                    expanded.extend(self.read_pdfmdfivesum_tokens()?);
                }
                Token::ControlSequence(name) if name == "the" => {
                    let tokens = self.read_the_quantity_tokens()?;
                    self.push_front(tokens);
                }
                Token::ControlSequence(name) if name == "let" => {
                    self.read_let_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "global" => {
                    self.read_global_assignment()?;
                }
                Token::ControlSequence(name) if name == "catcode" => {
                    self.read_catcode_assignment(AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "makeatletter" => {
                    self.set_ascii_catcode(b'@', CatCode::Letter, AssignmentScope::Local);
                }
                Token::ControlSequence(name) if name == "makeatother" => {
                    self.set_ascii_catcode(b'@', CatCode::Other, AssignmentScope::Local);
                }
                Token::ControlSequence(name)
                    if self.conditional_value_from_control_name(&name).is_some() =>
                {
                    let condition = self
                        .conditional_value_from_control_name(&name)
                        .expect("guard checked condition");
                    self.read_conditional(condition)?;
                }
                Token::ControlSequence(name)
                    if self
                        .conditional_assignment_from_control_name(&name)
                        .is_some() =>
                {
                    let (conditional, value) = self
                        .conditional_assignment_from_control_name(&name)
                        .expect("guard checked assignment");
                    self.set_conditional(conditional, value, AssignmentScope::Local);
                }
                Token::ControlSequence(name)
                    if self.resolve_count_register_key(&name).is_some()
                        && self.next_non_space_starts_register_assignment() =>
                {
                    let key = self
                        .resolve_count_register_key(&name)
                        .expect("guard checked count register");
                    self.read_count_assignment(key, AssignmentScope::Local)?;
                }
                Token::ControlSequence(name)
                    if name == "count" && self.next_numbered_register_starts_assignment() =>
                {
                    let key = self.read_numbered_count_register_key("\\count")?;
                    self.read_count_assignment(key, AssignmentScope::Local)?;
                }
                Token::ControlSequence(name)
                    if self.resolve_dimension_register_key(&name).is_some()
                        && self.next_non_space_starts_register_assignment() =>
                {
                    let key = self
                        .resolve_dimension_register_key(&name)
                        .expect("guard checked dimension register");
                    self.read_dimension_assignment(key, AssignmentScope::Local)?;
                }
                Token::ControlSequence(name)
                    if name == "dimen" && self.next_numbered_register_starts_assignment() =>
                {
                    let key = self.read_numbered_dimension_register_key("\\dimen")?;
                    self.read_dimension_assignment(key, AssignmentScope::Local)?;
                }
                Token::ControlSequence(name)
                    if self.resolve_skip_register_key(&name).is_some()
                        && self.next_non_space_starts_register_assignment() =>
                {
                    let key = self
                        .resolve_skip_register_key(&name)
                        .expect("guard checked skip register");
                    self.read_skip_assignment(key, AssignmentScope::Local)?;
                }
                Token::ControlSequence(name)
                    if name == "skip" && self.next_numbered_register_starts_assignment() =>
                {
                    let key = self.read_numbered_skip_register_key("\\skip")?;
                    self.read_skip_assignment(key, AssignmentScope::Local)?;
                }
                Token::ControlSequence(name)
                    if self.resolve_token_register_key(&name).is_some()
                        && self.next_non_space_starts_token_assignment() =>
                {
                    let key = self
                        .resolve_token_register_key(&name)
                        .expect("guard checked token register");
                    self.read_token_assignment(key, AssignmentScope::Local)?;
                }
                Token::ControlSequence(name)
                    if name == "toks" && self.next_numbered_register_starts_assignment() =>
                {
                    let key = self.read_numbered_token_register_key("\\toks")?;
                    self.read_token_assignment(key, AssignmentScope::Local)?;
                }
                Token::ControlSequence(name) if name == "begingroup" => {
                    self.enter_group();
                }
                Token::ControlSequence(name) if name == "endgroup" => {
                    self.leave_group()?;
                }
                Token::Character {
                    catcode: CatCode::BeginGroup,
                    ..
                } => {
                    self.enter_group();
                    expanded.push(token);
                }
                Token::Character {
                    catcode: CatCode::EndGroup,
                    ..
                } => {
                    self.leave_group()?;
                    expanded.push(token);
                }
                Token::ControlSequence(name) => {
                    self.expand_or_emit_macro_token(
                        &mut expanded,
                        name.clone(),
                        Token::ControlSequence(name),
                    )?;
                }
                Token::ControlSymbol(symbol) => {
                    self.expand_or_emit_macro_token(
                        &mut expanded,
                        symbol.to_string(),
                        Token::ControlSymbol(symbol),
                    )?;
                }
                token => expanded.push(token),
            }
        }
        Ok(expanded)
    }

    fn enter_group(&mut self) {
        self.scopes.push(ExpansionScopeSnapshot {
            state: None,
            aftergroup_tokens: Vec::new(),
        });
    }

    fn leave_group(&mut self) -> Result<(), ExpandError> {
        let Some(snapshot) = self.scopes.pop() else {
            return Err(ExpandError::new(
                "group ended without a matching begin group",
            ));
        };
        let aftergroup_tokens = snapshot.aftergroup_tokens;
        if let Some(state) = snapshot.state {
            self.restore_expansion_state(state);
        }
        self.push_front(aftergroup_tokens);
        Ok(())
    }

    fn ensure_current_scope_snapshot(&mut self) {
        let needs_snapshot = self
            .scopes
            .last()
            .is_some_and(|scope| scope.state.is_none());
        if !needs_snapshot {
            return;
        }
        let state = self.current_expansion_state();
        if let Some(scope) = self.scopes.last_mut() {
            scope.state = Some(state);
        }
    }

    fn current_expansion_state(&self) -> ExpansionStateSnapshot {
        ExpansionStateSnapshot {
            macros: self.macros.clone(),
            conditionals: self.conditionals.clone(),
            count_registers: self.count_registers.clone(),
            count_aliases: self.count_aliases.clone(),
            dimension_registers: self.dimension_registers.clone(),
            dimension_aliases: self.dimension_aliases.clone(),
            skip_registers: self.skip_registers.clone(),
            skip_aliases: self.skip_aliases.clone(),
            token_registers: self.token_registers.clone(),
            token_aliases: self.token_aliases.clone(),
            integer_constants: self.integer_constants.clone(),
            next_count_register: self.next_count_register,
            next_dimension_register: self.next_dimension_register,
            next_skip_register: self.next_skip_register,
            next_token_register: self.next_token_register,
            catcodes: self.tokenizer.catcodes(),
        }
    }

    fn restore_expansion_state(&mut self, state: ExpansionStateSnapshot) {
        self.macros = state.macros;
        self.conditionals = state.conditionals;
        self.count_registers = state.count_registers;
        self.count_aliases = state.count_aliases;
        self.dimension_registers = state.dimension_registers;
        self.dimension_aliases = state.dimension_aliases;
        self.skip_registers = state.skip_registers;
        self.skip_aliases = state.skip_aliases;
        self.token_registers = state.token_registers;
        self.token_aliases = state.token_aliases;
        self.integer_constants = state.integer_constants;
        self.next_count_register = state.next_count_register;
        self.next_dimension_register = state.next_dimension_register;
        self.next_skip_register = state.next_skip_register;
        self.next_token_register = state.next_token_register;
        self.tokenizer.set_catcodes(state.catcodes);
    }

    fn define_macro_with_scope(
        &mut self,
        name: impl Into<String>,
        definition: MacroDefinition,
        scope: AssignmentScope,
    ) {
        let name = name.into();
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.macros.insert(name.clone(), definition.clone());
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.macros.insert(name, definition);
    }

    fn set_conditional(&mut self, name: String, value: bool, scope: AssignmentScope) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.conditionals.insert(name.clone(), value);
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.conditionals.insert(name, value);
    }

    fn set_count_register(&mut self, key: CountRegisterKey, value: i64, scope: AssignmentScope) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.count_registers.insert(key.clone(), value);
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.count_registers.insert(key, value);
    }

    fn define_count_alias(&mut self, name: String, key: CountRegisterKey, scope: AssignmentScope) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.count_aliases.insert(name.clone(), key.clone());
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.count_aliases.insert(name, key);
    }

    fn set_dimension_register(
        &mut self,
        key: DimensionRegisterKey,
        value: i64,
        scope: AssignmentScope,
    ) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.dimension_registers.insert(key.clone(), value);
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.dimension_registers.insert(key, value);
    }

    fn define_dimension_alias(
        &mut self,
        name: String,
        key: DimensionRegisterKey,
        scope: AssignmentScope,
    ) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.dimension_aliases.insert(name.clone(), key.clone());
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.dimension_aliases.insert(name, key);
    }

    fn set_skip_register(
        &mut self,
        key: SkipRegisterKey,
        value: GlueValue,
        scope: AssignmentScope,
    ) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.skip_registers.insert(key.clone(), value.clone());
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.skip_registers.insert(key, value);
    }

    fn define_skip_alias(&mut self, name: String, key: SkipRegisterKey, scope: AssignmentScope) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.skip_aliases.insert(name.clone(), key.clone());
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.skip_aliases.insert(name, key);
    }

    fn set_token_register(
        &mut self,
        key: TokenRegisterKey,
        value: Vec<Token>,
        scope: AssignmentScope,
    ) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.token_registers.insert(key.clone(), value.clone());
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.token_registers.insert(key, value);
    }

    fn define_token_alias(&mut self, name: String, key: TokenRegisterKey, scope: AssignmentScope) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.token_aliases.insert(name.clone(), key.clone());
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.token_aliases.insert(name, key);
    }

    fn define_integer_constant(&mut self, name: String, value: i64, scope: AssignmentScope) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.integer_constants.insert(name.clone(), value);
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.integer_constants.insert(name, value);
    }

    fn allocate_count_register(&mut self, scope: AssignmentScope) -> CountRegisterKey {
        if scope == AssignmentScope::Local {
            self.ensure_current_scope_snapshot();
        }
        let key = CountRegisterKey::Numbered(self.next_count_register);
        self.next_count_register += 1;
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.next_count_register = self.next_count_register;
                }
            }
        }
        key
    }

    fn allocate_skip_register(&mut self, scope: AssignmentScope) -> SkipRegisterKey {
        if scope == AssignmentScope::Local {
            self.ensure_current_scope_snapshot();
        }
        let key = SkipRegisterKey::Numbered(self.next_skip_register);
        self.next_skip_register += 1;
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.next_skip_register = self.next_skip_register;
                }
            }
        }
        key
    }

    fn allocate_token_register(&mut self, scope: AssignmentScope) -> TokenRegisterKey {
        if scope == AssignmentScope::Local {
            self.ensure_current_scope_snapshot();
        }
        let key = TokenRegisterKey::Numbered(self.next_token_register);
        self.next_token_register += 1;
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.next_token_register = self.next_token_register;
                }
            }
        }
        key
    }

    fn allocate_dimension_register(&mut self, scope: AssignmentScope) -> DimensionRegisterKey {
        if scope == AssignmentScope::Local {
            self.ensure_current_scope_snapshot();
        }
        let key = DimensionRegisterKey::Numbered(self.next_dimension_register);
        self.next_dimension_register += 1;
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.next_dimension_register = self.next_dimension_register;
                }
            }
        }
        key
    }

    fn set_ascii_catcode(&mut self, ch: u8, catcode: CatCode, scope: AssignmentScope) {
        if scope == AssignmentScope::Global {
            for snapshot in &mut self.scopes {
                if let Some(state) = &mut snapshot.state {
                    state.catcodes.set_ascii(ch, catcode);
                }
            }
        } else {
            self.ensure_current_scope_snapshot();
        }
        self.tokenizer.set_ascii_catcode(ch, catcode);
    }

    fn expand_or_emit_macro_token(
        &mut self,
        expanded: &mut Vec<Token>,
        name: String,
        original: Token,
    ) -> Result<(), ExpandError> {
        if let Some(definition) = self.macros.get(&name).cloned() {
            let arguments = self.read_macro_arguments(&definition)?;
            let replacement = substitute_macro_arguments(&definition, &arguments)?;
            self.push_front(replacement);
        } else {
            expanded.push(original);
        }
        Ok(())
    }

    fn expand_after_once(&mut self) -> Result<Vec<Token>, ExpandError> {
        let first = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\expandafter ended before a first token"))?;
        let second = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\expandafter ended before a second token"))?;
        let mut tokens = vec![first];
        tokens.extend(self.expand_one_token(second)?);
        Ok(tokens)
    }

    fn read_conditional(&mut self, condition: bool) -> Result<(), ExpandError> {
        let ConditionalBranches {
            then_branch,
            else_branch,
        } = self.read_conditional_branches()?;
        if condition {
            self.push_front(then_branch);
        } else {
            self.push_front(else_branch);
        }
        Ok(())
    }

    fn read_case_conditional(&mut self, case: i64) -> Result<(), ExpandError> {
        let CaseBranches {
            branches,
            else_branch,
        } = self.read_case_branches()?;
        if case >= 0 && (case as usize) < branches.len() {
            let branch = branches
                .into_iter()
                .nth(case as usize)
                .expect("case index was bounds checked");
            self.push_front(branch);
        } else {
            self.push_front(else_branch);
        }
        Ok(())
    }

    fn read_unless_conditional(&mut self) -> Result<(), ExpandError> {
        self.skip_spaces();
        let conditional = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\unless ended before a conditional"))?;
        let condition = self.read_boolean_condition_from_token(conditional)?;
        self.read_conditional(!condition)
    }

    fn read_boolean_condition_from_token(&mut self, token: Token) -> Result<bool, ExpandError> {
        match token {
            Token::ControlSequence(name) if name == "iftrue" => Ok(true),
            Token::ControlSequence(name) if name == "iffalse" => Ok(false),
            Token::ControlSequence(name) if name == "ifx" => self.read_ifx_condition(),
            Token::ControlSequence(name) if name == "if" => self.read_if_condition(),
            Token::ControlSequence(name) if name == "ifcat" => self.read_ifcat_condition(),
            Token::ControlSequence(name) if name == "ifdefined" => self.read_ifdefined_condition(),
            Token::ControlSequence(name) if name == "ifcsname" => self.read_ifcsname_condition(),
            Token::ControlSequence(name) if name == "ifpdfprimitive" => {
                self.read_ifpdfprimitive_condition()
            }
            Token::ControlSequence(name) if name == "ifnum" => self.read_ifnum_condition(),
            Token::ControlSequence(name) if name == "ifdim" => self.read_ifdim_condition(),
            Token::ControlSequence(name) if name == "ifodd" => {
                let condition = self.read_integer()? % 2 != 0;
                self.skip_spaces();
                Ok(condition)
            }
            Token::ControlSequence(name) => {
                if let Some(condition) = self.conditional_value_from_control_name(&name) {
                    Ok(condition)
                } else {
                    Err(ExpandError::new(format!(
                        "\\unless requires a conditional, got \\{name}"
                    )))
                }
            }
            token => Err(ExpandError::new(format!(
                "\\unless requires a conditional, got {token:?}"
            ))),
        }
    }

    fn read_ifx_condition(&mut self) -> Result<bool, ExpandError> {
        let left = self
            .next_raw_non_space()
            .ok_or_else(|| ExpandError::new("\\ifx ended before a left token"))?;
        let right = self
            .next_raw_non_space()
            .ok_or_else(|| ExpandError::new("\\ifx ended before a right token"))?;
        Ok(self.tokens_have_same_meaning(&left, &right))
    }

    fn read_if_condition(&mut self) -> Result<bool, ExpandError> {
        let left = self.read_expanded_comparison_token("\\if")?;
        let right = self.read_expanded_comparison_token("\\if")?;
        self.skip_spaces();
        Ok(tokens_have_same_character_code(&left, &right))
    }

    fn read_ifcat_condition(&mut self) -> Result<bool, ExpandError> {
        let left = self.read_expanded_comparison_token("\\ifcat")?;
        let right = self.read_expanded_comparison_token("\\ifcat")?;
        self.skip_spaces();
        Ok(tokens_have_same_category(&left, &right))
    }

    fn read_ifdefined_condition(&mut self) -> Result<bool, ExpandError> {
        let token = self
            .next_raw_non_space()
            .ok_or_else(|| ExpandError::new("\\ifdefined ended before a token"))?;
        Ok(macro_key(&token).is_some_and(|name| self.is_defined_control_name(&name)))
    }

    fn read_ifcsname_condition(&mut self) -> Result<bool, ExpandError> {
        let Token::ControlSequence(name) = self.read_csname_control_sequence_named("\\ifcsname")?
        else {
            unreachable!("csname helper always returns a control sequence");
        };
        Ok(self.is_defined_control_name(&name))
    }

    fn read_ifpdfprimitive_condition(&mut self) -> Result<bool, ExpandError> {
        let token = self
            .next_raw_non_space()
            .ok_or_else(|| ExpandError::new("\\ifpdfprimitive ended before a token"))?;
        Ok(macro_key(&token).is_some_and(|name| primitive_meaning(&name).is_some()))
    }

    fn read_ifnum_condition(&mut self) -> Result<bool, ExpandError> {
        let left = self.read_integer()?;
        self.skip_spaces();
        let relation = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\ifnum ended before a relation"))?;
        let relation = match relation {
            Token::Character { ch: '<', .. } => NumericRelation::Less,
            Token::Character { ch: '=', .. } => NumericRelation::Equal,
            Token::Character { ch: '>', .. } => NumericRelation::Greater,
            token => {
                return Err(ExpandError::new(format!(
                    "\\ifnum relation must be one of <, =, or >, got {token:?}"
                )));
            }
        };
        let right = self.read_integer()?;
        self.skip_spaces();
        Ok(match relation {
            NumericRelation::Less => left < right,
            NumericRelation::Equal => left == right,
            NumericRelation::Greater => left > right,
        })
    }

    fn read_ifdim_condition(&mut self) -> Result<bool, ExpandError> {
        let left = self.read_dimension()?;
        self.skip_spaces();
        let relation = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\ifdim ended before a relation"))?;
        let relation = match relation {
            Token::Character { ch: '<', .. } => NumericRelation::Less,
            Token::Character { ch: '=', .. } => NumericRelation::Equal,
            Token::Character { ch: '>', .. } => NumericRelation::Greater,
            token => {
                return Err(ExpandError::new(format!(
                    "\\ifdim relation must be one of <, =, or >, got {token:?}"
                )));
            }
        };
        let right = self.read_dimension()?;
        self.skip_spaces();
        Ok(match relation {
            NumericRelation::Less => left < right,
            NumericRelation::Equal => left == right,
            NumericRelation::Greater => left > right,
        })
    }

    fn read_integer(&mut self) -> Result<i64, ExpandError> {
        self.skip_spaces();
        let sign = self.read_optional_integer_sign();
        self.skip_spaces();
        let token = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("integer expression ended before a value"))?;
        match token {
            Token::Character { ch, .. } if ch.is_ascii_digit() => {
                let mut digits = String::from(ch);
                while let Some(Token::Character { ch, .. }) = self.peek_unexpanded() {
                    if !ch.is_ascii_digit() {
                        break;
                    }
                    digits.push(ch);
                    let _ = self.next_unexpanded();
                }
                digits
                    .parse::<i64>()
                    .map(|value| sign * value)
                    .map_err(|_| ExpandError::new("integer expression is too large"))
            }
            Token::ControlSequence(name) if name == "number" || name == "the" => {
                Ok(sign * self.read_integer()?)
            }
            Token::ControlSequence(name) if name == "numexpr" => {
                Ok(sign * self.read_integer_expression()?)
            }
            Token::ControlSequence(name) => {
                if name == "count" {
                    let key = self.read_numbered_count_register_key("\\count")?;
                    Ok(sign * self.count_register_value(&key))
                } else if let Some(key) = self.resolve_count_register_key(&name) {
                    let value = self.count_register_value(&key);
                    Ok(sign * value)
                } else if let Some(value) = self.integer_constants.get(&name).copied() {
                    Ok(sign * value)
                } else if is_expandable_primitive_name(&name) {
                    let tokens = self.expand_one_token(Token::ControlSequence(name))?;
                    if tokens.is_empty() {
                        return Ok(0);
                    }
                    self.push_front(tokens);
                    Ok(sign * self.read_integer()?)
                } else if let Some(definition) = self.macros.get(&name).cloned() {
                    let arguments = self.read_macro_arguments(&definition)?;
                    let replacement = substitute_macro_arguments(&definition, &arguments)?;
                    self.push_front(replacement);
                    Ok(sign * self.read_integer()?)
                } else {
                    Err(ExpandError::new(format!(
                        "integer expression expected digits or a count register, got \\{name}"
                    )))
                }
            }
            Token::ControlSymbol(symbol) => {
                let key = symbol.to_string();
                if let Some(key) = self.resolve_count_register_key(&key) {
                    let value = self.count_register_value(&key);
                    Ok(sign * value)
                } else if let Some(value) = self.integer_constants.get(&key).copied() {
                    Ok(sign * value)
                } else {
                    Err(ExpandError::new(format!(
                        "integer expression expected digits or a count register, got \\{symbol}"
                    )))
                }
            }
            token => Err(ExpandError::new(format!(
                "integer expression expected digits or a count register, got {token:?}"
            ))),
        }
    }

    fn read_integer_expression(&mut self) -> Result<i64, ExpandError> {
        let mut value = self.read_integer_product()?;
        loop {
            self.skip_spaces();
            match self.peek_unexpanded() {
                Some(Token::ControlSequence(name)) if name == "relax" => {
                    let _ = self.next_unexpanded();
                    return Ok(value);
                }
                Some(Token::Character { ch: '+', .. }) => {
                    let _ = self.next_unexpanded();
                    let delta = self.read_integer_product()?;
                    value = value
                        .checked_add(delta)
                        .ok_or_else(|| ExpandError::new("integer expression is too large"))?;
                }
                Some(Token::Character { ch: '-', .. }) => {
                    let _ = self.next_unexpanded();
                    let delta = self.read_integer_product()?;
                    value = value
                        .checked_sub(delta)
                        .ok_or_else(|| ExpandError::new("integer expression is too large"))?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn read_integer_product(&mut self) -> Result<i64, ExpandError> {
        let mut value = self.read_integer()?;
        loop {
            self.skip_spaces();
            match self.peek_unexpanded() {
                Some(Token::Character { ch: '*', .. }) => {
                    let _ = self.next_unexpanded();
                    let factor = self.read_integer()?;
                    value = value
                        .checked_mul(factor)
                        .ok_or_else(|| ExpandError::new("integer expression is too large"))?;
                }
                Some(Token::Character { ch: '/', .. }) => {
                    let _ = self.next_unexpanded();
                    let divisor = self.read_integer()?;
                    if divisor == 0 {
                        return Err(ExpandError::new("integer expression divided by zero"));
                    }
                    value /= divisor;
                }
                _ => return Ok(value),
            }
        }
    }

    fn read_dimension(&mut self) -> Result<i64, ExpandError> {
        self.skip_spaces();
        let sign = self.read_optional_integer_sign();
        self.skip_spaces();
        let token = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("dimension expression ended before a value"))?;
        match token {
            Token::Character { ch, .. } if ch.is_ascii_digit() || ch == '.' => {
                let magnitude = self.read_decimal_number(ch)?;
                let unit = self.read_dimension_unit()?;
                decimal_dimension_to_sp(sign as f64 * magnitude, &unit)
            }
            Token::ControlSequence(name) if name == "the" => Ok(sign * self.read_dimension()?),
            Token::ControlSequence(name) if name == "dimexpr" => {
                Ok(sign * self.read_dimension_expression()?)
            }
            Token::ControlSequence(name) => {
                if name == "dimen" {
                    let key = self.read_numbered_dimension_register_key("\\dimen")?;
                    Ok(sign * self.dimension_register_value(&key))
                } else if let Some(key) = self.resolve_dimension_register_key(&name) {
                    Ok(sign * self.dimension_register_value(&key))
                } else if let Some(definition) = self.macros.get(&name).cloned() {
                    let arguments = self.read_macro_arguments(&definition)?;
                    let replacement = substitute_macro_arguments(&definition, &arguments)?;
                    self.push_front(replacement);
                    Ok(sign * self.read_dimension()?)
                } else {
                    Err(ExpandError::new(format!(
                        "dimension expression expected a dimension or register, got \\{name}"
                    )))
                }
            }
            Token::ControlSymbol(symbol) => {
                let name = symbol.to_string();
                if let Some(key) = self.resolve_dimension_register_key(&name) {
                    Ok(sign * self.dimension_register_value(&key))
                } else {
                    Err(ExpandError::new(format!(
                        "dimension expression expected a dimension or register, got \\{symbol}"
                    )))
                }
            }
            token => Err(ExpandError::new(format!(
                "dimension expression expected a dimension or register, got {token:?}"
            ))),
        }
    }

    fn read_dimension_expression(&mut self) -> Result<i64, ExpandError> {
        let mut value = self.read_dimension()?;
        loop {
            self.skip_spaces();
            match self.peek_unexpanded() {
                Some(Token::ControlSequence(name)) if name == "relax" => {
                    let _ = self.next_unexpanded();
                    return Ok(value);
                }
                Some(Token::Character { ch: '+', .. }) => {
                    let _ = self.next_unexpanded();
                    let delta = self.read_dimension()?;
                    value = value
                        .checked_add(delta)
                        .ok_or_else(|| ExpandError::new("dimension expression is too large"))?;
                }
                Some(Token::Character { ch: '-', .. }) => {
                    let _ = self.next_unexpanded();
                    let delta = self.read_dimension()?;
                    value = value
                        .checked_sub(delta)
                        .ok_or_else(|| ExpandError::new("dimension expression is too large"))?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn read_decimal_number(&mut self, first: char) -> Result<f64, ExpandError> {
        let mut literal = String::from(first);
        let mut seen_dot = first == '.';
        while let Some(Token::Character { ch, .. }) = self.peek_unexpanded() {
            if ch.is_ascii_digit() {
                literal.push(ch);
                let _ = self.next_unexpanded();
            } else if ch == '.' && !seen_dot {
                seen_dot = true;
                literal.push(ch);
                let _ = self.next_unexpanded();
            } else {
                break;
            }
        }
        if literal == "." {
            return Err(ExpandError::new("dimension number requires digits"));
        }
        literal
            .parse::<f64>()
            .map_err(|_| ExpandError::new("invalid dimension number"))
    }

    fn read_dimension_unit(&mut self) -> Result<String, ExpandError> {
        self.skip_spaces();
        let mut unit = String::new();
        while let Some(Token::Character { ch, .. }) = self.peek_unexpanded() {
            if !ch.is_ascii_alphabetic() {
                break;
            }
            unit.push(ch.to_ascii_lowercase());
            let _ = self.next_unexpanded();
        }
        if unit.is_empty() {
            return Err(ExpandError::new("dimension requires a unit"));
        }
        if unit == "true" {
            self.skip_spaces();
            unit.clear();
            while let Some(Token::Character { ch, .. }) = self.peek_unexpanded() {
                if !ch.is_ascii_alphabetic() {
                    break;
                }
                unit.push(ch.to_ascii_lowercase());
                let _ = self.next_unexpanded();
            }
            if unit.is_empty() {
                return Err(ExpandError::new("true dimension requires a physical unit"));
            }
        }
        Ok(unit)
    }

    fn read_the_quantity_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        self.skip_spaces();
        if self.next_token_starts_token_quantity() {
            self.read_token_register_tokens()
        } else if self.next_token_starts_skip_quantity() {
            let value = self.read_glue()?;
            Ok(glue_tokens(&value))
        } else if self.next_token_starts_dimension_quantity() {
            let value = self.read_dimension()?;
            Ok(dimension_tokens(value))
        } else {
            let value = self.read_integer()?;
            Ok(integer_tokens(value))
        }
    }

    fn read_glue(&mut self) -> Result<GlueValue, ExpandError> {
        self.skip_spaces();
        let token = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("glue expression ended before a value"))?;
        match token {
            Token::ControlSequence(name) if name == "the" => self.read_glue(),
            Token::ControlSequence(name) if name == "skip" => {
                let key = self.read_numbered_skip_register_key("\\skip")?;
                Ok(self.skip_register_value(&key))
            }
            Token::ControlSequence(name) => {
                if let Some(key) = self.resolve_skip_register_key(&name) {
                    Ok(self.skip_register_value(&key))
                } else if let Some(definition) = self.macros.get(&name).cloned() {
                    let arguments = self.read_macro_arguments(&definition)?;
                    let replacement = substitute_macro_arguments(&definition, &arguments)?;
                    self.push_front(replacement);
                    self.read_glue()
                } else {
                    self.push_front(vec![Token::ControlSequence(name)]);
                    self.read_glue_literal()
                }
            }
            Token::ControlSymbol(symbol) => {
                let name = symbol.to_string();
                if let Some(key) = self.resolve_skip_register_key(&name) {
                    Ok(self.skip_register_value(&key))
                } else {
                    self.push_front(vec![Token::ControlSymbol(symbol)]);
                    self.read_glue_literal()
                }
            }
            token => {
                self.push_front(vec![token]);
                self.read_glue_literal()
            }
        }
    }

    fn read_glue_literal(&mut self) -> Result<GlueValue, ExpandError> {
        let width = self.read_dimension()?;
        let stretch = if self.consume_optional_keyword("plus") {
            self.read_dimension()?
        } else {
            0
        };
        let shrink = if self.consume_optional_keyword("minus") {
            self.read_dimension()?
        } else {
            0
        };
        Ok(GlueValue {
            width,
            stretch,
            shrink,
        })
    }

    fn read_string_primitive_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let token = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\string ended before a token"))?;
        Ok(string_tokens(&token))
    }

    fn read_meaning_primitive_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let token = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\meaning ended before a token"))?;
        Ok(meaning_tokens(self, &token))
    }

    fn read_detokenize_primitive_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let body = self.read_required_group_body("\\detokenize", "a braced token list")?;
        Ok(detokenized_tokens(&body))
    }

    fn read_unexpanded_primitive_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        self.read_required_group_body("\\unexpanded", "a braced token list")
    }

    fn read_expanded_primitive_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let body = self.read_required_group_body("\\expanded", "a braced token list")?;
        self.expand_known_macros_in_tokens(body)
    }

    fn read_noexpand_primitive_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let token = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\noexpand ended before a token"))?;
        Ok(vec![token])
    }

    fn jobname_tokens(&self) -> Vec<Token> {
        tokens_from_ascii_other(self.job_name())
    }

    fn read_pdfprimitive_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        self.skip_spaces();
        let token = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\pdfprimitive ended before a token"))?;
        let Some(name) = macro_key(&token) else {
            return Ok(Vec::new());
        };

        if is_expandable_primitive_name(&name) {
            self.expand_one_token(Token::ControlSequence(name))
        } else if primitive_meaning(&name).is_some() {
            Ok(vec![
                Token::ControlSequence("pdfprimitive".to_string()),
                token,
            ])
        } else {
            Ok(Vec::new())
        }
    }

    fn read_pdffilesize_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let file_name = self.read_expanded_file_name("\\pdffilesize")?;
        let Some(path) = self.resolve_pdftex_file(&file_name, "\\pdffilesize")? else {
            return Ok(Vec::new());
        };
        let size = fs::metadata(&path)
            .map_err(|error| {
                ExpandError::new(format!(
                    "\\pdffilesize could not read metadata for `{}`: {error}",
                    path.display()
                ))
            })?
            .len();
        Ok(tokens_from_ascii_other(&size.to_string()))
    }

    fn read_pdffilemoddate_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let file_name = self.read_expanded_file_name("\\pdffilemoddate")?;
        let Some(path) = self.resolve_pdftex_file(&file_name, "\\pdffilemoddate")? else {
            return Ok(Vec::new());
        };
        let modified = fs::metadata(&path)
            .and_then(|metadata| metadata.modified())
            .map_err(|error| {
                ExpandError::new(format!(
                    "\\pdffilemoddate could not read modification time for `{}`: {error}",
                    path.display()
                ))
            })?;
        Ok(tokens_from_ascii_other(&pdf_date_string(modified)))
    }

    fn read_pdffiledump_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let mut offset = 0_usize;
        let mut length = None;
        if self.consume_optional_keyword("offset") {
            offset = self.read_non_negative_usize("\\pdffiledump offset")?;
        }
        if self.consume_optional_keyword("length") {
            length = Some(self.read_non_negative_usize("\\pdffiledump length")?);
        }
        let file_name = self.read_expanded_file_name("\\pdffiledump")?;
        Ok(tokens_from_ascii_other(
            &self.pdf_file_dump(&file_name, offset, length)?,
        ))
    }

    fn read_pdfstrcmp_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let left = self.read_expanded_group_text("\\pdfstrcmp")?;
        let right = self.read_expanded_group_text("\\pdfstrcmp")?;
        Ok(integer_tokens(pdfstrcmp_value(&left, &right)))
    }

    fn read_pdfescapehex_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let text = self.read_expanded_group_text("\\pdfescapehex")?;
        Ok(tokens_from_ascii_other(&pdf_escape_hex(&text)))
    }

    fn read_pdfunescapehex_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let text = self.read_expanded_group_text("\\pdfunescapehex")?;
        Ok(tokens_from_ascii_other(&pdf_unescape_hex(&text)?))
    }

    fn read_pdfescapestring_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let text = self.read_expanded_group_text("\\pdfescapestring")?;
        Ok(tokens_from_ascii_other(&pdf_escape_string(&text)))
    }

    fn read_pdfescapename_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        let text = self.read_expanded_group_text("\\pdfescapename")?;
        Ok(tokens_from_ascii_other(&pdf_escape_name(&text)))
    }

    fn read_pdfmdfivesum_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        if self.consume_optional_keyword("file") {
            let file_name = self.read_expanded_file_name("\\pdfmdfivesum file")?;
            return Ok(tokens_from_ascii_other(&self.pdf_file_md5_sum(&file_name)?));
        }
        let text = self.read_expanded_group_text("\\pdfmdfivesum")?;
        Ok(tokens_from_ascii_other(&pdf_md5_sum(&text)))
    }

    fn read_expanded_group_text(&mut self, primitive: &str) -> Result<String, ExpandError> {
        let body = self.read_required_group_body(primitive, "a braced token list")?;
        let expanded = self.expand_known_macros_in_tokens(body)?;
        Ok(tokens_to_text(&expanded))
    }

    fn read_expanded_file_name(&mut self, primitive: &str) -> Result<String, ExpandError> {
        let name = self.read_expanded_group_text(primitive)?;
        Ok(self.normalize_pdftex_file_name(&name))
    }

    fn normalize_pdftex_file_name(&self, file_name: &str) -> String {
        file_name
            .trim()
            .replace("\\jobname", self.job_name())
            .replace("\\relax", "")
            .trim()
            .to_string()
    }

    fn resolve_pdftex_file(
        &self,
        file_name: &str,
        primitive: &str,
    ) -> Result<Option<PathBuf>, ExpandError> {
        let file_name = self.normalize_pdftex_file_name(file_name);
        if file_name.is_empty() {
            return Ok(None);
        }
        if file_name.starts_with('|') {
            return Err(ExpandError::new(format!(
                "{primitive} does not support shell-pipe file names"
            )));
        }
        let path = Path::new(&file_name);
        if path.is_absolute() {
            return Err(ExpandError::new(format!(
                "{primitive} only supports local relative file names, got `{file_name}`"
            )));
        }
        let Some(context) = &self.file_context else {
            return Ok(None);
        };
        let candidate = context.root_dir.join(path);
        let existing = if candidate.exists() {
            Some(candidate)
        } else if candidate.extension().is_none() {
            let tex_candidate = candidate.with_extension("tex");
            tex_candidate.exists().then_some(tex_candidate)
        } else {
            None
        };
        let Some(existing) = existing else {
            return Ok(None);
        };
        let canonical = fs::canonicalize(&existing).map_err(|error| {
            ExpandError::new(format!(
                "{primitive} could not canonicalize `{}`: {error}",
                existing.display()
            ))
        })?;
        if !canonical.starts_with(&context.root_dir) {
            return Err(ExpandError::new(format!(
                "{primitive} file `{file_name}` resolves outside the native project root"
            )));
        }
        Ok(Some(canonical))
    }

    fn pdf_file_md5_sum(&self, file_name: &str) -> Result<String, ExpandError> {
        let Some(path) = self.resolve_pdftex_file(file_name, "\\pdfmdfivesum file")? else {
            return Ok(String::new());
        };
        let bytes = fs::read(&path).map_err(|error| {
            ExpandError::new(format!(
                "\\pdfmdfivesum file could not read `{}`: {error}",
                path.display()
            ))
        })?;
        Ok(pdf_md5_sum_bytes(&bytes))
    }

    fn pdf_file_dump(
        &self,
        file_name: &str,
        offset: usize,
        length: Option<usize>,
    ) -> Result<String, ExpandError> {
        let Some(length) = length else {
            return Ok(String::new());
        };
        if length == 0 {
            return Ok(String::new());
        }
        let Some(path) = self.resolve_pdftex_file(file_name, "\\pdffiledump")? else {
            return Ok(String::new());
        };
        let bytes = fs::read(&path).map_err(|error| {
            ExpandError::new(format!(
                "\\pdffiledump could not read `{}`: {error}",
                path.display()
            ))
        })?;
        if offset >= bytes.len() {
            return Ok(String::new());
        }
        let end = offset.saturating_add(length).min(bytes.len());
        Ok(pdf_escape_hex_bytes(&bytes[offset..end]))
    }

    fn job_name(&self) -> &str {
        self.file_context
            .as_ref()
            .map(|context| context.job_name.as_str())
            .unwrap_or("texput")
    }

    fn read_non_negative_usize(&mut self, primitive: &str) -> Result<usize, ExpandError> {
        let value = self.read_integer()?;
        usize::try_from(value).map_err(|_| {
            ExpandError::new(format!(
                "{primitive} requires a non-negative integer, got {value}"
            ))
        })
    }

    fn read_token_register_tokens(&mut self) -> Result<Vec<Token>, ExpandError> {
        self.skip_spaces();
        let token = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("token register expansion ended before a value"))?;
        match token {
            Token::ControlSequence(name) if name == "toks" => {
                let key = self.read_numbered_token_register_key("\\toks")?;
                Ok(self.token_register_value(&key))
            }
            Token::ControlSequence(name) => {
                if let Some(key) = self.resolve_token_register_key(&name) {
                    Ok(self.token_register_value(&key))
                } else {
                    Err(ExpandError::new(format!(
                        "\\the expected a token register, got \\{name}"
                    )))
                }
            }
            Token::ControlSymbol(symbol) => {
                let name = symbol.to_string();
                if let Some(key) = self.resolve_token_register_key(&name) {
                    Ok(self.token_register_value(&key))
                } else {
                    Err(ExpandError::new(format!(
                        "\\the expected a token register, got \\{symbol}"
                    )))
                }
            }
            token => Err(ExpandError::new(format!(
                "\\the expected a token register, got {token:?}"
            ))),
        }
    }

    fn read_optional_integer_sign(&mut self) -> i64 {
        let mut sign = 1_i64;
        loop {
            match self.peek_unexpanded() {
                Some(Token::Character { ch: '+', .. }) => {
                    let _ = self.next_unexpanded();
                }
                Some(Token::Character { ch: '-', .. }) => {
                    let _ = self.next_unexpanded();
                    sign = -sign;
                }
                _ => return sign,
            }
            self.skip_spaces();
        }
    }

    fn read_newif_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let conditional = self.read_control_name("\\newif")?;
        let Some(name) = conditional.strip_prefix("if") else {
            return Err(ExpandError::new(format!(
                "\\newif requires a conditional name beginning with \\if, got \\{conditional}"
            )));
        };
        if name.is_empty() {
            return Err(ExpandError::new(
                "\\newif requires a non-empty conditional name",
            ));
        }
        self.set_conditional(name.to_string(), false, scope);
        Ok(())
    }

    fn read_newcount_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let name = self.read_control_name("\\newcount")?;
        let key = self.allocate_count_register(scope);
        self.define_count_alias(name, key.clone(), scope);
        self.set_count_register(key, 0, scope);
        Ok(())
    }

    fn read_countdef_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let name = self.read_control_name("\\countdef")?;
        self.skip_spaces();
        if matches!(
            self.peek_unexpanded(),
            Some(Token::Character { ch: '=', .. })
        ) {
            let _ = self.next_unexpanded();
        }
        let register = self.read_count_register_index("\\countdef")?;
        let key = CountRegisterKey::Numbered(register);
        self.define_count_alias(name, key.clone(), scope);
        self.ensure_count_register(key, scope);
        Ok(())
    }

    fn read_newdimen_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let name = self.read_control_name("\\newdimen")?;
        let key = self.allocate_dimension_register(scope);
        self.define_dimension_alias(name, key.clone(), scope);
        self.set_dimension_register(key, 0, scope);
        Ok(())
    }

    fn read_dimendef_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let name = self.read_control_name("\\dimendef")?;
        self.skip_spaces();
        if matches!(
            self.peek_unexpanded(),
            Some(Token::Character { ch: '=', .. })
        ) {
            let _ = self.next_unexpanded();
        }
        let register = self.read_dimension_register_index("\\dimendef")?;
        let key = DimensionRegisterKey::Numbered(register);
        self.define_dimension_alias(name, key.clone(), scope);
        self.ensure_dimension_register(key, scope);
        Ok(())
    }

    fn read_newskip_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let name = self.read_control_name("\\newskip")?;
        let key = self.allocate_skip_register(scope);
        self.define_skip_alias(name, key.clone(), scope);
        self.set_skip_register(key, zero_glue(), scope);
        Ok(())
    }

    fn read_skipdef_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let name = self.read_control_name("\\skipdef")?;
        self.skip_spaces();
        if matches!(
            self.peek_unexpanded(),
            Some(Token::Character { ch: '=', .. })
        ) {
            let _ = self.next_unexpanded();
        }
        let register = self.read_skip_register_index("\\skipdef")?;
        let key = SkipRegisterKey::Numbered(register);
        self.define_skip_alias(name, key.clone(), scope);
        self.ensure_skip_register(key, scope);
        Ok(())
    }

    fn read_newtoks_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let name = self.read_control_name("\\newtoks")?;
        let key = self.allocate_token_register(scope);
        self.define_token_alias(name, key.clone(), scope);
        self.set_token_register(key, Vec::new(), scope);
        Ok(())
    }

    fn read_toksdef_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let name = self.read_control_name("\\toksdef")?;
        self.skip_spaces();
        if matches!(
            self.peek_unexpanded(),
            Some(Token::Character { ch: '=', .. })
        ) {
            let _ = self.next_unexpanded();
        }
        let register = self.read_token_register_index("\\toksdef")?;
        let key = TokenRegisterKey::Numbered(register);
        self.define_token_alias(name, key.clone(), scope);
        self.ensure_token_register(key, scope);
        Ok(())
    }

    fn read_integer_constant_assignment(
        &mut self,
        primitive: &str,
        scope: AssignmentScope,
    ) -> Result<(), ExpandError> {
        self.skip_spaces();
        let name = self.read_control_name(primitive)?;
        self.skip_spaces();
        if matches!(
            self.peek_unexpanded(),
            Some(Token::Character { ch: '=', .. })
        ) {
            let _ = self.next_unexpanded();
        }
        let value = self.read_integer()?;
        self.define_integer_constant(name, value, scope);
        Ok(())
    }

    fn read_count_assignment(
        &mut self,
        key: CountRegisterKey,
        scope: AssignmentScope,
    ) -> Result<(), ExpandError> {
        self.skip_spaces();
        self.consume_optional_equals();
        let value = self.read_integer()?;
        self.set_count_register(key, value, scope);
        Ok(())
    }

    fn read_dimension_assignment(
        &mut self,
        key: DimensionRegisterKey,
        scope: AssignmentScope,
    ) -> Result<(), ExpandError> {
        self.skip_spaces();
        self.consume_optional_equals();
        let value = self.read_dimension()?;
        self.set_dimension_register(key, value, scope);
        Ok(())
    }

    fn read_skip_assignment(
        &mut self,
        key: SkipRegisterKey,
        scope: AssignmentScope,
    ) -> Result<(), ExpandError> {
        self.skip_spaces();
        self.consume_optional_equals();
        let value = self.read_glue()?;
        self.set_skip_register(key, value, scope);
        Ok(())
    }

    fn read_token_assignment(
        &mut self,
        key: TokenRegisterKey,
        scope: AssignmentScope,
    ) -> Result<(), ExpandError> {
        self.skip_spaces();
        self.consume_optional_equals();
        let value = self.read_token_assignment_value()?;
        self.set_token_register(key, value, scope);
        Ok(())
    }

    fn consume_optional_equals(&mut self) {
        self.skip_spaces();
        if matches!(
            self.peek_unexpanded(),
            Some(Token::Character { ch: '=', .. })
        ) {
            let _ = self.next_unexpanded();
        }
    }

    fn read_token_assignment_value(&mut self) -> Result<Vec<Token>, ExpandError> {
        self.skip_spaces();
        match self.next_unexpanded() {
            Some(Token::Character {
                catcode: CatCode::BeginGroup,
                ..
            }) => self.read_balanced_group_body(),
            Some(token) => Err(ExpandError::new(format!(
                "token register assignment requires a braced token list, got {token:?}"
            ))),
            None => Err(ExpandError::new(
                "token register assignment ended before a braced token list",
            )),
        }
    }

    fn read_advance_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let target = self.read_register_target("\\advance")?;
        self.skip_spaces();
        self.consume_optional_keyword("by");
        match target {
            RegisterTarget::Count(key) => {
                let delta = self.read_integer()?;
                let current = self.count_register_value(&key);
                self.set_count_register(key, current + delta, scope);
            }
            RegisterTarget::Dimension(key) => {
                let delta = self.read_dimension()?;
                let current = self.dimension_register_value(&key);
                self.set_dimension_register(key, current + delta, scope);
            }
            RegisterTarget::Skip(key) => {
                let delta = self.read_glue()?;
                let current = self.skip_register_value(&key);
                self.set_skip_register(key, current.add(&delta)?, scope);
            }
        }
        Ok(())
    }

    fn read_register_target(&mut self, primitive: &str) -> Result<RegisterTarget, ExpandError> {
        match self.next_raw_non_space() {
            Some(Token::ControlSequence(name)) if name == "count" => Ok(RegisterTarget::Count(
                self.read_numbered_count_register_key("\\count")?,
            )),
            Some(Token::ControlSequence(name)) if name == "dimen" => Ok(RegisterTarget::Dimension(
                self.read_numbered_dimension_register_key("\\dimen")?,
            )),
            Some(Token::ControlSequence(name)) if name == "skip" => Ok(RegisterTarget::Skip(
                self.read_numbered_skip_register_key("\\skip")?,
            )),
            Some(Token::ControlSequence(name)) => {
                if let Some(key) = self.resolve_skip_register_key(&name) {
                    Ok(RegisterTarget::Skip(key))
                } else if let Some(key) = self.resolve_dimension_register_key(&name) {
                    Ok(RegisterTarget::Dimension(key))
                } else {
                    Ok(RegisterTarget::Count(
                        self.resolve_count_register_key(&name)
                            .unwrap_or(CountRegisterKey::Named(name)),
                    ))
                }
            }
            Some(Token::ControlSymbol(symbol)) => {
                let name = symbol.to_string();
                if let Some(key) = self.resolve_skip_register_key(&name) {
                    Ok(RegisterTarget::Skip(key))
                } else if let Some(key) = self.resolve_dimension_register_key(&name) {
                    Ok(RegisterTarget::Dimension(key))
                } else {
                    Ok(RegisterTarget::Count(
                        self.resolve_count_register_key(&name)
                            .unwrap_or(CountRegisterKey::Named(name)),
                    ))
                }
            }
            Some(token) => Err(ExpandError::new(format!(
                "{primitive} requires a register target, got {token:?}"
            ))),
            None => Err(ExpandError::new(format!(
                "{primitive} ended before a register target"
            ))),
        }
    }

    fn read_numbered_count_register_key(
        &mut self,
        primitive: &str,
    ) -> Result<CountRegisterKey, ExpandError> {
        Ok(CountRegisterKey::Numbered(
            self.read_count_register_index(primitive)?,
        ))
    }

    fn read_count_register_index(&mut self, primitive: &str) -> Result<i64, ExpandError> {
        let register = self.read_integer()?;
        if register < 0 {
            return Err(ExpandError::new(format!(
                "{primitive} requires a non-negative count register number, got {register}"
            )));
        }
        Ok(register)
    }

    fn read_numbered_dimension_register_key(
        &mut self,
        primitive: &str,
    ) -> Result<DimensionRegisterKey, ExpandError> {
        Ok(DimensionRegisterKey::Numbered(
            self.read_dimension_register_index(primitive)?,
        ))
    }

    fn read_dimension_register_index(&mut self, primitive: &str) -> Result<i64, ExpandError> {
        let register = self.read_integer()?;
        if register < 0 {
            return Err(ExpandError::new(format!(
                "{primitive} requires a non-negative dimension register number, got {register}"
            )));
        }
        Ok(register)
    }

    fn read_numbered_skip_register_key(
        &mut self,
        primitive: &str,
    ) -> Result<SkipRegisterKey, ExpandError> {
        Ok(SkipRegisterKey::Numbered(
            self.read_skip_register_index(primitive)?,
        ))
    }

    fn read_skip_register_index(&mut self, primitive: &str) -> Result<i64, ExpandError> {
        let register = self.read_integer()?;
        if register < 0 {
            return Err(ExpandError::new(format!(
                "{primitive} requires a non-negative skip register number, got {register}"
            )));
        }
        Ok(register)
    }

    fn read_numbered_token_register_key(
        &mut self,
        primitive: &str,
    ) -> Result<TokenRegisterKey, ExpandError> {
        Ok(TokenRegisterKey::Numbered(
            self.read_token_register_index(primitive)?,
        ))
    }

    fn read_token_register_index(&mut self, primitive: &str) -> Result<i64, ExpandError> {
        let register = self.read_integer()?;
        if register < 0 {
            return Err(ExpandError::new(format!(
                "{primitive} requires a non-negative token register number, got {register}"
            )));
        }
        Ok(register)
    }

    fn resolve_count_register_key(&self, name: &str) -> Option<CountRegisterKey> {
        self.count_aliases.get(name).cloned().or_else(|| {
            self.count_registers
                .contains_key(&CountRegisterKey::Named(name.to_string()))
                .then(|| CountRegisterKey::Named(name.to_string()))
        })
    }

    fn resolve_dimension_register_key(&self, name: &str) -> Option<DimensionRegisterKey> {
        self.dimension_aliases.get(name).cloned().or_else(|| {
            self.dimension_registers
                .contains_key(&DimensionRegisterKey::Named(name.to_string()))
                .then(|| DimensionRegisterKey::Named(name.to_string()))
        })
    }

    fn resolve_skip_register_key(&self, name: &str) -> Option<SkipRegisterKey> {
        self.skip_aliases.get(name).cloned().or_else(|| {
            self.skip_registers
                .contains_key(&SkipRegisterKey::Named(name.to_string()))
                .then(|| SkipRegisterKey::Named(name.to_string()))
        })
    }

    fn resolve_token_register_key(&self, name: &str) -> Option<TokenRegisterKey> {
        self.token_aliases.get(name).cloned().or_else(|| {
            self.token_registers
                .contains_key(&TokenRegisterKey::Named(name.to_string()))
                .then(|| TokenRegisterKey::Named(name.to_string()))
        })
    }

    fn count_register_value(&self, key: &CountRegisterKey) -> i64 {
        self.count_registers.get(key).copied().unwrap_or(0)
    }

    fn dimension_register_value(&self, key: &DimensionRegisterKey) -> i64 {
        self.dimension_registers.get(key).copied().unwrap_or(0)
    }

    fn skip_register_value(&self, key: &SkipRegisterKey) -> GlueValue {
        self.skip_registers
            .get(key)
            .cloned()
            .unwrap_or_else(zero_glue)
    }

    fn token_register_value(&self, key: &TokenRegisterKey) -> Vec<Token> {
        self.token_registers.get(key).cloned().unwrap_or_default()
    }

    fn ensure_count_register(&mut self, key: CountRegisterKey, scope: AssignmentScope) {
        if !self.count_registers.contains_key(&key) {
            self.set_count_register(key, 0, scope);
        }
    }

    fn ensure_dimension_register(&mut self, key: DimensionRegisterKey, scope: AssignmentScope) {
        if !self.dimension_registers.contains_key(&key) {
            self.set_dimension_register(key, 0, scope);
        }
    }

    fn ensure_skip_register(&mut self, key: SkipRegisterKey, scope: AssignmentScope) {
        if !self.skip_registers.contains_key(&key) {
            self.set_skip_register(key, zero_glue(), scope);
        }
    }

    fn ensure_token_register(&mut self, key: TokenRegisterKey, scope: AssignmentScope) {
        if !self.token_registers.contains_key(&key) {
            self.set_token_register(key, Vec::new(), scope);
        }
    }

    fn next_token_starts_token_quantity(&mut self) -> bool {
        match self.peek_unexpanded() {
            Some(Token::ControlSequence(name)) if name == "toks" => true,
            Some(Token::ControlSequence(name)) => self.resolve_token_register_key(&name).is_some(),
            Some(Token::ControlSymbol(symbol)) => self
                .resolve_token_register_key(&symbol.to_string())
                .is_some(),
            _ => false,
        }
    }

    fn next_token_starts_skip_quantity(&mut self) -> bool {
        match self.peek_unexpanded() {
            Some(Token::ControlSequence(name)) if name == "skip" => true,
            Some(Token::ControlSequence(name)) => self.resolve_skip_register_key(&name).is_some(),
            Some(Token::ControlSymbol(symbol)) => self
                .resolve_skip_register_key(&symbol.to_string())
                .is_some(),
            _ => false,
        }
    }

    fn next_token_starts_dimension_quantity(&mut self) -> bool {
        match self.peek_unexpanded() {
            Some(Token::ControlSequence(name)) if name == "dimexpr" => true,
            Some(Token::ControlSequence(name)) if name == "dimen" => true,
            Some(Token::ControlSequence(name)) => {
                self.resolve_dimension_register_key(&name).is_some()
            }
            Some(Token::ControlSymbol(symbol)) => self
                .resolve_dimension_register_key(&symbol.to_string())
                .is_some(),
            _ => false,
        }
    }

    fn next_non_space_starts_register_assignment(&mut self) -> bool {
        let mut consumed = Vec::new();
        let mut starts_assignment = false;
        while let Some(token) = self.next_unexpanded() {
            let is_space = is_space_like(&token);
            starts_assignment = matches!(token, Token::Character { ch: '=', .. });
            consumed.push(token);
            if !is_space {
                break;
            }
        }
        self.push_front(consumed);
        starts_assignment
    }

    fn next_non_space_starts_token_assignment(&mut self) -> bool {
        let mut consumed = Vec::new();
        let mut starts_assignment = false;
        while let Some(token) = self.next_unexpanded() {
            let is_space = is_space_like(&token);
            starts_assignment = matches!(
                token,
                Token::Character { ch: '=', .. }
                    | Token::Character {
                        catcode: CatCode::BeginGroup,
                        ..
                    }
            );
            consumed.push(token);
            if !is_space {
                break;
            }
        }
        self.push_front(consumed);
        starts_assignment
    }

    fn next_numbered_register_starts_assignment(&mut self) -> bool {
        let mut consumed = Vec::new();
        let mut saw_digit = false;
        let mut starts_assignment = false;

        while let Some(token) = self.next_unexpanded() {
            if is_space_like(&token) {
                consumed.push(token);
                continue;
            }
            match &token {
                Token::Character { ch, .. } if ch.is_ascii_digit() => {
                    saw_digit = true;
                    consumed.push(token);
                }
                token => {
                    consumed.push(token.clone());
                    break;
                }
            }
            break;
        }

        if saw_digit {
            loop {
                let Some(token) = self.next_unexpanded() else {
                    break;
                };
                match &token {
                    Token::Character { ch, .. } if ch.is_ascii_digit() => {
                        consumed.push(token);
                    }
                    _ if is_space_like(&token) => {
                        consumed.push(token);
                    }
                    _ => {
                        starts_assignment = matches!(token, Token::Character { ch: '=', .. });
                        consumed.push(token);
                        break;
                    }
                }
            }
        }

        self.push_front(consumed);
        saw_digit && starts_assignment
    }

    fn read_conditional_branches(&mut self) -> Result<ConditionalBranches, ExpandError> {
        let mut then_branch = Vec::new();
        let mut else_branch = Vec::new();
        let mut in_else_branch = false;
        let mut nested_conditionals = 0_usize;

        while let Some(token) = self.next_unexpanded() {
            match &token {
                Token::ControlSequence(name)
                    if nested_conditionals == 0 && name == "else" && !in_else_branch =>
                {
                    in_else_branch = true;
                    continue;
                }
                Token::ControlSequence(name) if nested_conditionals == 0 && name == "fi" => {
                    return Ok(ConditionalBranches {
                        then_branch,
                        else_branch,
                    });
                }
                Token::ControlSequence(name) if self.is_conditional_start_name(name) => {
                    nested_conditionals += 1;
                }
                Token::ControlSequence(name) if name == "fi" && nested_conditionals > 0 => {
                    nested_conditionals -= 1;
                }
                _ => {}
            }

            if in_else_branch {
                else_branch.push(token);
            } else {
                then_branch.push(token);
            }
        }

        Err(ExpandError::new("conditional ended before a matching \\fi"))
    }

    fn read_case_branches(&mut self) -> Result<CaseBranches, ExpandError> {
        let mut branches = vec![Vec::new()];
        let mut else_branch = Vec::new();
        let mut in_else_branch = false;
        let mut nested_conditionals = 0_usize;

        while let Some(token) = self.next_unexpanded() {
            match &token {
                Token::ControlSequence(name)
                    if nested_conditionals == 0 && name == "or" && !in_else_branch =>
                {
                    branches.push(Vec::new());
                    continue;
                }
                Token::ControlSequence(name)
                    if nested_conditionals == 0 && name == "else" && !in_else_branch =>
                {
                    in_else_branch = true;
                    continue;
                }
                Token::ControlSequence(name) if nested_conditionals == 0 && name == "fi" => {
                    return Ok(CaseBranches {
                        branches,
                        else_branch,
                    });
                }
                Token::ControlSequence(name) if self.is_conditional_start_name(name) => {
                    nested_conditionals += 1;
                }
                Token::ControlSequence(name) if name == "fi" && nested_conditionals > 0 => {
                    nested_conditionals -= 1;
                }
                _ => {}
            }

            if in_else_branch {
                else_branch.push(token);
            } else if let Some(branch) = branches.last_mut() {
                branch.push(token);
            }
        }

        Err(ExpandError::new("\\ifcase ended before a matching \\fi"))
    }

    fn tokens_have_same_meaning(&self, left: &Token, right: &Token) -> bool {
        match (macro_key(left), macro_key(right)) {
            (Some(left_name), Some(right_name)) => {
                match (self.macros.get(&left_name), self.macros.get(&right_name)) {
                    (Some(left_definition), Some(right_definition)) => {
                        left_definition == right_definition
                    }
                    (None, None) => {
                        self.conditional_value_from_control_name(&left_name)
                            == self.conditional_value_from_control_name(&right_name)
                            && self
                                .conditional_assignment_from_control_name(&left_name)
                                .map(|(_, value)| value)
                                == self
                                    .conditional_assignment_from_control_name(&right_name)
                                    .map(|(_, value)| value)
                            && self.resolve_count_register_key(&left_name)
                                == self.resolve_count_register_key(&right_name)
                            && self.resolve_dimension_register_key(&left_name)
                                == self.resolve_dimension_register_key(&right_name)
                            && self.resolve_skip_register_key(&left_name)
                                == self.resolve_skip_register_key(&right_name)
                            && self.resolve_token_register_key(&left_name)
                                == self.resolve_token_register_key(&right_name)
                            && self.integer_constants.get(&left_name)
                                == self.integer_constants.get(&right_name)
                            && primitive_meaning(&left_name) == primitive_meaning(&right_name)
                    }
                    _ => false,
                }
            }
            (None, None) => left == right,
            _ => false,
        }
    }

    fn is_defined_control_name(&self, name: &str) -> bool {
        self.macros.contains_key(name)
            || self.conditional_value_from_control_name(name).is_some()
            || self
                .conditional_assignment_from_control_name(name)
                .is_some()
            || self.resolve_count_register_key(name).is_some()
            || self.resolve_dimension_register_key(name).is_some()
            || self.resolve_skip_register_key(name).is_some()
            || self.resolve_token_register_key(name).is_some()
            || self.integer_constants.contains_key(name)
            || primitive_meaning(name).is_some()
    }

    fn conditional_value_from_control_name(&self, name: &str) -> Option<bool> {
        name.strip_prefix("if")
            .and_then(|conditional| self.conditionals.get(conditional).copied())
    }

    fn conditional_assignment_from_control_name(&self, name: &str) -> Option<(String, bool)> {
        if let Some(conditional) = name.strip_suffix("true")
            && !conditional.is_empty()
            && self.conditionals.contains_key(conditional)
        {
            return Some((conditional.to_string(), true));
        }
        if let Some(conditional) = name.strip_suffix("false")
            && !conditional.is_empty()
            && self.conditionals.contains_key(conditional)
        {
            return Some((conditional.to_string(), false));
        }
        None
    }

    fn is_conditional_start_name(&self, name: &str) -> bool {
        matches!(
            name,
            "iftrue"
                | "iffalse"
                | "ifx"
                | "if"
                | "ifcat"
                | "ifdefined"
                | "ifcsname"
                | "ifpdfprimitive"
                | "ifnum"
                | "ifdim"
                | "ifodd"
                | "ifcase"
        ) || self.conditional_value_from_control_name(name).is_some()
    }

    fn read_expanded_comparison_token(&mut self, primitive: &str) -> Result<Token, ExpandError> {
        self.skip_spaces();
        loop {
            let token = self
                .next_unexpanded()
                .ok_or_else(|| ExpandError::new(format!("{primitive} ended before a token")))?;
            let expanded = self.expand_one_token(token)?;
            if expanded.is_empty() {
                continue;
            }
            if expanded
                .first()
                .is_some_and(|token| self.token_is_expandable(token))
            {
                self.push_front(expanded);
                continue;
            }

            let mut expanded = expanded.into_iter();
            let first = expanded
                .next()
                .expect("expanded token list was checked non-empty");
            let remaining = expanded.collect::<Vec<_>>();
            self.push_front(remaining);
            return Ok(first);
        }
    }

    fn token_is_expandable(&self, token: &Token) -> bool {
        macro_key(token).is_some_and(|name| {
            (name != "pdfprimitive" && is_expandable_primitive_name(&name))
                || self.macros.contains_key(&name)
        })
    }

    fn expand_one_token(&mut self, token: Token) -> Result<Vec<Token>, ExpandError> {
        match token {
            Token::ControlSequence(name) if name == "csname" => {
                Ok(vec![self.read_csname_control_sequence()?])
            }
            Token::ControlSequence(name) if name == "expandafter" => self.expand_after_once(),
            Token::ControlSequence(name) if name == "number" => {
                Ok(integer_tokens(self.read_integer()?))
            }
            Token::ControlSequence(name) if name == "the" => self.read_the_quantity_tokens(),
            Token::ControlSequence(name) if name == "romannumeral" => {
                Ok(roman_numeral_tokens(self.read_integer()?))
            }
            Token::ControlSequence(name) if name == "string" => self.read_string_primitive_tokens(),
            Token::ControlSequence(name) if name == "meaning" => {
                self.read_meaning_primitive_tokens()
            }
            Token::ControlSequence(name) if name == "detokenize" => {
                self.read_detokenize_primitive_tokens()
            }
            Token::ControlSequence(name) if name == "unexpanded" => {
                self.read_unexpanded_primitive_tokens()
            }
            Token::ControlSequence(name) if name == "expanded" => {
                self.read_expanded_primitive_tokens()
            }
            Token::ControlSequence(name) if name == "noexpand" => {
                self.read_noexpand_primitive_tokens()
            }
            Token::ControlSequence(name) if name == "jobname" => Ok(self.jobname_tokens()),
            Token::ControlSequence(name) if name == "pdfprimitive" => {
                self.read_pdfprimitive_tokens()
            }
            Token::ControlSequence(name) if name == "pdfcreationdate" => Ok(
                tokens_from_ascii_other(&pdf_date_string(self.creation_time)),
            ),
            Token::ControlSequence(name) if name == "pdffilesize" => self.read_pdffilesize_tokens(),
            Token::ControlSequence(name) if name == "pdffilemoddate" => {
                self.read_pdffilemoddate_tokens()
            }
            Token::ControlSequence(name) if name == "pdffiledump" => self.read_pdffiledump_tokens(),
            Token::ControlSequence(name) if name == "pdfstrcmp" => self.read_pdfstrcmp_tokens(),
            Token::ControlSequence(name) if name == "pdfescapehex" => {
                self.read_pdfescapehex_tokens()
            }
            Token::ControlSequence(name) if name == "pdfunescapehex" => {
                self.read_pdfunescapehex_tokens()
            }
            Token::ControlSequence(name) if name == "pdfescapestring" => {
                self.read_pdfescapestring_tokens()
            }
            Token::ControlSequence(name) if name == "pdfescapename" => {
                self.read_pdfescapename_tokens()
            }
            Token::ControlSequence(name) if name == "pdfmdfivesum" => {
                self.read_pdfmdfivesum_tokens()
            }
            Token::ControlSequence(name) => {
                if let Some(definition) = self.macros.get(&name).cloned() {
                    let arguments = self.read_macro_arguments(&definition)?;
                    substitute_macro_arguments(&definition, &arguments)
                } else {
                    Ok(vec![Token::ControlSequence(name)])
                }
            }
            Token::ControlSymbol(symbol) => {
                if let Some(definition) = self.macros.get(&symbol.to_string()).cloned() {
                    let arguments = self.read_macro_arguments(&definition)?;
                    substitute_macro_arguments(&definition, &arguments)
                } else {
                    Ok(vec![Token::ControlSymbol(symbol)])
                }
            }
            token => Ok(vec![token]),
        }
    }

    fn read_csname_control_sequence(&mut self) -> Result<Token, ExpandError> {
        self.read_csname_control_sequence_named("\\csname")
    }

    fn read_csname_control_sequence_named(
        &mut self,
        primitive: &str,
    ) -> Result<Token, ExpandError> {
        let mut name = String::new();
        loop {
            let Some(token) = self.next_unexpanded() else {
                return Err(ExpandError::new(format!(
                    "{primitive} ended before a matching \\endcsname"
                )));
            };
            if is_end_csname_token(&token) {
                return Ok(Token::ControlSequence(name));
            }
            let expanded = self.expand_one_token(token)?;
            for token in expanded {
                if is_end_csname_token(&token) {
                    return Ok(Token::ControlSequence(name));
                }
                append_csname_part(&mut name, &token);
            }
        }
    }

    fn read_tex_definition(
        &mut self,
        primitive: &str,
        expansion: ReplacementExpansion,
        scope: AssignmentScope,
        protected: bool,
    ) -> Result<(), ExpandError> {
        self.skip_spaces();
        let name = self.read_control_name(primitive)?;

        let mut arity = 0_usize;
        loop {
            let Some(token) = self.next_unexpanded() else {
                return Err(ExpandError::new(format!(
                    "{primitive} ended before replacement text"
                )));
            };
            match token {
                Token::Character {
                    catcode: CatCode::BeginGroup,
                    ..
                } => {
                    let replacement = match expansion {
                        ReplacementExpansion::Deferred => self.read_balanced_group_body()?,
                        ReplacementExpansion::ExpandNow => {
                            let replacement = self.read_balanced_group_body()?;
                            self.expand_known_macros_in_tokens(replacement)?
                        }
                    };
                    let mut definition = MacroDefinition::new(arity, replacement);
                    if protected {
                        definition = definition.protected();
                    }
                    self.define_macro_with_scope(name, definition, scope);
                    return Ok(());
                }
                Token::Character {
                    ch: '#',
                    catcode: CatCode::Parameter,
                } => {
                    let parameter = self.next_unexpanded().ok_or_else(|| {
                        ExpandError::new(format!(
                            "{primitive} parameter marker ended before a digit"
                        ))
                    })?;
                    arity = arity.max(parameter_number(&parameter)?);
                }
                Token::Character {
                    catcode: CatCode::Space,
                    ..
                } => {}
                _ => {
                    return Err(ExpandError::new(
                        "only simple positional macro parameter text is supported",
                    ));
                }
            }
        }
    }

    fn expand_known_macros_in_tokens(&self, tokens: Vec<Token>) -> Result<Vec<Token>, ExpandError> {
        let mut pending = tokens.into_iter().rev().collect::<Vec<_>>();
        let mut output = Vec::new();
        while let Some(token) = pending.pop() {
            let Some(name) = macro_key(&token) else {
                output.push(token);
                continue;
            };
            if name == "csname" {
                let control = read_csname_control_sequence_from_pending(self, &mut pending)?;
                pending.push(control);
                continue;
            }
            if name == "expandafter" {
                let tokens = expand_after_once_from_pending(self, &mut pending)?;
                pending.extend(tokens.into_iter().rev());
                continue;
            }
            if name == "noexpand" {
                let token = pending
                    .pop()
                    .ok_or_else(|| ExpandError::new("\\noexpand ended before a token"))?;
                output.push(token);
                continue;
            }
            if name == "unexpanded" {
                let body = read_required_group_argument_from_pending(&mut pending, "\\unexpanded")?;
                output.extend(body);
                continue;
            }
            if let Some(tokens) = expand_primitive_from_pending(self, &name, &mut pending)? {
                pending.extend(tokens.into_iter().rev());
                continue;
            }
            let Some(definition) = self.macros.get(&name).cloned() else {
                output.push(token);
                continue;
            };
            if definition.is_protected() {
                output.push(token);
                continue;
            }
            let arguments = read_macro_arguments_from_pending(&mut pending, &definition)?;
            let replacement = substitute_macro_arguments(&definition, &arguments)?;
            pending.extend(replacement.into_iter().rev());
        }
        Ok(output)
    }

    fn read_latex_command_definition(
        &mut self,
        primitive: &str,
        policy: DefinitionPolicy,
        scope: AssignmentScope,
        protected: bool,
    ) -> Result<(), ExpandError> {
        self.skip_spaces();
        self.consume_optional_star();
        self.skip_spaces();
        let name = self.read_latex_command_name(primitive)?;

        self.skip_spaces();
        let arity = self
            .read_optional_bracket_body()?
            .map(|tokens| parse_usize_tokens(&tokens, primitive))
            .transpose()?
            .unwrap_or(0);
        if arity > 9 {
            return Err(ExpandError::new(format!(
                "{primitive} supports at most 9 arguments, got {arity}"
            )));
        }

        self.skip_spaces();
        let optional_default = if arity == 0 {
            None
        } else {
            self.read_optional_bracket_body()?
        };
        self.skip_spaces();
        let replacement = self.read_required_group_body(primitive, "a replacement braced group")?;

        let mut definition = match optional_default {
            Some(default) => MacroDefinition::with_optional_default(arity, default, replacement),
            None => MacroDefinition::new(arity, replacement),
        };
        if protected {
            definition = definition.protected();
        }
        if policy == DefinitionPolicy::Always || !self.macros.contains_key(&name) {
            self.define_macro_with_scope(name, definition, scope);
        }
        Ok(())
    }

    fn read_declare_math_operator_definition(
        &mut self,
        scope: AssignmentScope,
    ) -> Result<(), ExpandError> {
        self.skip_spaces();
        self.consume_optional_star();
        self.skip_spaces();
        let name = self.read_latex_command_name("\\DeclareMathOperator")?;
        let replacement =
            self.read_required_group_body("\\DeclareMathOperator", "an operator braced group")?;
        self.define_macro_with_scope(name, MacroDefinition::new(0, replacement), scope);
        Ok(())
    }

    fn read_latex_command_name(&mut self, primitive: &str) -> Result<String, ExpandError> {
        match self.next_raw_non_space() {
            Some(Token::Character {
                catcode: CatCode::BeginGroup,
                ..
            }) => {
                let body = self.read_balanced_group_body()?;
                let significant_tokens = body
                    .iter()
                    .filter(|token| !is_space_like(token))
                    .collect::<Vec<_>>();
                if significant_tokens.len() != 1 {
                    return Err(ExpandError::new(format!(
                        "{primitive} requires a single control sequence name"
                    )));
                }
                macro_key(significant_tokens[0]).ok_or_else(|| {
                    ExpandError::new(format!(
                        "{primitive} requires a control sequence name, got {:?}",
                        significant_tokens[0]
                    ))
                })
            }
            Some(token) => macro_key(&token).ok_or_else(|| {
                ExpandError::new(format!(
                    "{primitive} requires a control sequence name, got {token:?}"
                ))
            }),
            None => Err(ExpandError::new(format!(
                "{primitive} ended before a control sequence name"
            ))),
        }
    }

    fn read_required_group_body(
        &mut self,
        primitive: &str,
        description: &str,
    ) -> Result<Vec<Token>, ExpandError> {
        self.skip_spaces();
        match self.next_unexpanded() {
            Some(Token::Character {
                catcode: CatCode::BeginGroup,
                ..
            }) => self.read_balanced_group_body(),
            Some(token) => Err(ExpandError::new(format!(
                "{primitive} requires {description}, got {token:?}"
            ))),
            None => Err(ExpandError::new(format!(
                "{primitive} ended before {description}"
            ))),
        }
    }

    fn consume_optional_star(&mut self) -> bool {
        if matches!(
            self.peek_unexpanded(),
            Some(Token::Character { ch: '*', .. })
        ) {
            let _ = self.next_unexpanded();
            true
        } else {
            false
        }
    }

    fn read_futurelet_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let target = self.read_control_name("\\futurelet")?;
        let first = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\futurelet ended before an execution token"))?;
        let second = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\futurelet ended before a lookahead token"))?;
        self.assign_let_target_to_source(target, second.clone(), scope);
        self.push_front(vec![first, second]);
        Ok(())
    }

    fn read_aftergroup_assignment(&mut self) -> Result<(), ExpandError> {
        let token = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\aftergroup ended before a token"))?;
        let Some(scope) = self.scopes.last_mut() else {
            return Err(ExpandError::new(
                "\\aftergroup requires an active group in this expansion engine",
            ));
        };
        scope.aftergroup_tokens.push(token);
        Ok(())
    }

    fn read_let_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        let target = self.read_control_name("\\let")?;
        self.skip_spaces();
        if matches!(
            self.peek_unexpanded(),
            Some(Token::Character { ch: '=', .. })
        ) {
            let _ = self.next_unexpanded();
        }
        self.skip_spaces();
        let source = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\let ended before a source token"))?;
        self.assign_let_target_to_source(target, source, scope);
        Ok(())
    }

    fn assign_let_target_to_source(
        &mut self,
        target: String,
        source: Token,
        scope: AssignmentScope,
    ) {
        if let Some(key) =
            macro_key(&source).and_then(|name| self.resolve_count_register_key(&name))
        {
            self.define_count_alias(target, key, scope);
            return;
        }
        if let Some(key) =
            macro_key(&source).and_then(|name| self.resolve_dimension_register_key(&name))
        {
            self.define_dimension_alias(target, key, scope);
            return;
        }
        if let Some(key) = macro_key(&source).and_then(|name| self.resolve_skip_register_key(&name))
        {
            self.define_skip_alias(target, key, scope);
            return;
        }
        if let Some(key) =
            macro_key(&source).and_then(|name| self.resolve_token_register_key(&name))
        {
            self.define_token_alias(target, key, scope);
            return;
        }
        if let Some(value) =
            macro_key(&source).and_then(|name| self.integer_constants.get(&name).copied())
        {
            self.define_integer_constant(target, value, scope);
            return;
        }
        let definition = match macro_key(&source).and_then(|name| self.macros.get(&name).cloned()) {
            Some(definition) => definition,
            None => MacroDefinition::new(0, vec![source]),
        };
        self.define_macro_with_scope(target, definition, scope);
    }

    fn read_protected_assignment(
        &mut self,
        default_scope: AssignmentScope,
    ) -> Result<(), ExpandError> {
        self.skip_spaces();
        let assignment = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\protected ended before a definition"))?;
        match assignment {
            Token::ControlSequence(name) if name == "def" => self.read_tex_definition(
                "\\protected\\def",
                ReplacementExpansion::Deferred,
                default_scope,
                true,
            ),
            Token::ControlSequence(name) if name == "gdef" => self.read_tex_definition(
                "\\protected\\gdef",
                ReplacementExpansion::Deferred,
                AssignmentScope::Global,
                true,
            ),
            Token::ControlSequence(name) if name == "edef" => self.read_tex_definition(
                "\\protected\\edef",
                ReplacementExpansion::ExpandNow,
                default_scope,
                true,
            ),
            Token::ControlSequence(name) if name == "xdef" => self.read_tex_definition(
                "\\protected\\xdef",
                ReplacementExpansion::ExpandNow,
                AssignmentScope::Global,
                true,
            ),
            token => Err(ExpandError::new(format!(
                "\\protected only supports TeX definition primitives, got {token:?}"
            ))),
        }
    }

    fn read_global_assignment(&mut self) -> Result<(), ExpandError> {
        self.skip_spaces();
        let assignment = self
            .next_unexpanded()
            .ok_or_else(|| ExpandError::new("\\global ended before an assignment"))?;
        match assignment {
            Token::ControlSequence(name) if name == "def" || name == "gdef" => self
                .read_tex_definition(
                    "\\global\\def",
                    ReplacementExpansion::Deferred,
                    AssignmentScope::Global,
                    false,
                ),
            Token::ControlSequence(name)
                if name == "edef" || name == "xdef" || name == "protected@edef" =>
            {
                self.read_tex_definition(
                    "\\global\\edef",
                    ReplacementExpansion::ExpandNow,
                    AssignmentScope::Global,
                    false,
                )
            }
            Token::ControlSequence(name) if name == "protected" => {
                self.read_protected_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "newcommand" || name == "renewcommand" => self
                .read_latex_command_definition(
                    "\\global\\newcommand",
                    DefinitionPolicy::Always,
                    AssignmentScope::Global,
                    false,
                ),
            Token::ControlSequence(name) if name == "newif" => {
                self.read_newif_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "newcount" => {
                self.read_newcount_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "countdef" => {
                self.read_countdef_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "newdimen" => {
                self.read_newdimen_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "dimendef" => {
                self.read_dimendef_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "newskip" => {
                self.read_newskip_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "skipdef" => {
                self.read_skipdef_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "newtoks" => {
                self.read_newtoks_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "toksdef" => {
                self.read_toksdef_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "chardef" || name == "mathchardef" => self
                .read_integer_constant_assignment(
                    &format!("\\global\\{name}"),
                    AssignmentScope::Global,
                ),
            Token::ControlSequence(name) if name == "advance" => {
                self.read_advance_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "providecommand" => self
                .read_latex_command_definition(
                    "\\global\\providecommand",
                    DefinitionPolicy::IfUndefined,
                    AssignmentScope::Global,
                    false,
                ),
            Token::ControlSequence(name) if name == "DeclareRobustCommand" => self
                .read_latex_command_definition(
                    "\\global\\DeclareRobustCommand",
                    DefinitionPolicy::Always,
                    AssignmentScope::Global,
                    true,
                ),
            Token::ControlSequence(name) if name == "DeclareMathOperator" => {
                self.read_declare_math_operator_definition(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "let" => {
                self.read_let_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "futurelet" => {
                self.read_futurelet_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "catcode" => {
                self.read_catcode_assignment(AssignmentScope::Global)
            }
            Token::ControlSequence(name)
                if self
                    .conditional_assignment_from_control_name(&name)
                    .is_some() =>
            {
                let (conditional, value) = self
                    .conditional_assignment_from_control_name(&name)
                    .expect("guard checked conditional assignment");
                self.set_conditional(conditional, value, AssignmentScope::Global);
                Ok(())
            }
            Token::ControlSequence(name) if self.resolve_count_register_key(&name).is_some() => {
                let key = self
                    .resolve_count_register_key(&name)
                    .expect("guard checked count register");
                self.read_count_assignment(key, AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "count" => {
                let key = self.read_numbered_count_register_key("\\count")?;
                self.read_count_assignment(key, AssignmentScope::Global)
            }
            Token::ControlSequence(name)
                if self.resolve_dimension_register_key(&name).is_some() =>
            {
                let key = self
                    .resolve_dimension_register_key(&name)
                    .expect("guard checked dimension register");
                self.read_dimension_assignment(key, AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "dimen" => {
                let key = self.read_numbered_dimension_register_key("\\dimen")?;
                self.read_dimension_assignment(key, AssignmentScope::Global)
            }
            Token::ControlSequence(name) if self.resolve_skip_register_key(&name).is_some() => {
                let key = self
                    .resolve_skip_register_key(&name)
                    .expect("guard checked skip register");
                self.read_skip_assignment(key, AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "skip" => {
                let key = self.read_numbered_skip_register_key("\\skip")?;
                self.read_skip_assignment(key, AssignmentScope::Global)
            }
            Token::ControlSequence(name) if self.resolve_token_register_key(&name).is_some() => {
                let key = self
                    .resolve_token_register_key(&name)
                    .expect("guard checked token register");
                self.read_token_assignment(key, AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "toks" => {
                let key = self.read_numbered_token_register_key("\\toks")?;
                self.read_token_assignment(key, AssignmentScope::Global)
            }
            Token::ControlSequence(name) if name == "makeatletter" => {
                self.set_ascii_catcode(b'@', CatCode::Letter, AssignmentScope::Global);
                Ok(())
            }
            Token::ControlSequence(name) if name == "makeatother" => {
                self.set_ascii_catcode(b'@', CatCode::Other, AssignmentScope::Global);
                Ok(())
            }
            token => Err(ExpandError::new(format!(
                "\\global only supports assignment primitives already implemented by the expansion engine, got {token:?}"
            ))),
        }
    }

    fn read_control_name(&mut self, primitive: &str) -> Result<String, ExpandError> {
        match self.next_raw_non_space() {
            Some(Token::ControlSequence(name)) => Ok(name),
            Some(Token::ControlSymbol(symbol)) => Ok(symbol.to_string()),
            Some(token) => Err(ExpandError::new(format!(
                "{primitive} requires a control sequence name, got {token:?}"
            ))),
            None => Err(ExpandError::new(format!(
                "{primitive} ended before a control sequence name"
            ))),
        }
    }

    fn read_catcode_assignment(&mut self, scope: AssignmentScope) -> Result<(), ExpandError> {
        self.skip_spaces();
        match self.next_raw_non_space() {
            Some(Token::Character { ch: '`', .. }) => {}
            Some(token) => {
                return Err(ExpandError::new(format!(
                    "\\catcode requires a backtick character target, got {token:?}"
                )));
            }
            None => {
                return Err(ExpandError::new(
                    "\\catcode ended before a target character",
                ));
            }
        }

        let target = match self.next_unexpanded() {
            Some(Token::ControlSymbol(symbol)) => symbol,
            Some(Token::ControlSequence(name)) if name.chars().count() == 1 => {
                name.chars().next().expect("single-character name")
            }
            Some(Token::Character { ch, .. }) => ch,
            Some(token) => {
                return Err(ExpandError::new(format!(
                    "\\catcode target must be a character or control symbol, got {token:?}"
                )));
            }
            None => {
                return Err(ExpandError::new(
                    "\\catcode ended before a target character",
                ));
            }
        };
        if !target.is_ascii() {
            return Err(ExpandError::new(format!(
                "\\catcode only supports ASCII targets, got {target:?}"
            )));
        }

        self.skip_spaces();
        if matches!(
            self.peek_unexpanded(),
            Some(Token::Character { ch: '=', .. })
        ) {
            let _ = self.next_unexpanded();
        }
        self.skip_spaces();

        let mut digits = String::new();
        while let Some(Token::Character { ch, .. }) = self.peek_unexpanded() {
            if !ch.is_ascii_digit() {
                break;
            }
            digits.push(ch);
            let _ = self.next_unexpanded();
        }
        if digits.is_empty() {
            return Err(ExpandError::new("\\catcode requires a numeric value"));
        }
        let value = digits
            .parse::<u8>()
            .map_err(|_| ExpandError::new("invalid \\catcode numeric value"))?;
        let catcode = catcode_from_number(value)?;
        self.set_ascii_catcode(target as u8, catcode, scope);
        Ok(())
    }

    fn read_macro_arguments(
        &mut self,
        definition: &MacroDefinition,
    ) -> Result<Vec<Vec<Token>>, ExpandError> {
        let arity = definition.arity();
        let mut arguments = Vec::with_capacity(arity);
        if let Some(default) = definition.optional_default() {
            self.skip_spaces();
            if let Some(argument) = self.read_optional_bracket_body()? {
                arguments.push(argument);
            } else {
                arguments.push(default.to_vec());
            }
        }

        for _ in arguments.len()..arity {
            arguments.push(self.read_required_macro_argument()?);
        }
        Ok(arguments)
    }

    fn read_required_macro_argument(&mut self) -> Result<Vec<Token>, ExpandError> {
        self.skip_spaces();
        let Some(token) = self.next_unexpanded() else {
            return Err(ExpandError::new(
                "macro invocation ended before all arguments",
            ));
        };
        if matches!(
            token,
            Token::Character {
                catcode: CatCode::BeginGroup,
                ..
            }
        ) {
            self.read_balanced_group_body()
        } else {
            Ok(vec![token])
        }
    }

    fn read_optional_bracket_body(&mut self) -> Result<Option<Vec<Token>>, ExpandError> {
        if !matches!(
            self.peek_unexpanded(),
            Some(Token::Character { ch: '[', .. })
        ) {
            return Ok(None);
        }
        let _ = self.next_unexpanded();

        let mut body = Vec::new();
        let mut depth = 1_usize;
        while let Some(token) = self.next_unexpanded() {
            match token {
                Token::Character { ch: '[', .. } => {
                    depth += 1;
                    body.push(token);
                }
                Token::Character { ch: ']', .. } => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(Some(body));
                    }
                    body.push(token);
                }
                token => body.push(token),
            }
        }
        Err(ExpandError::new(
            "optional argument ended before a matching `]`",
        ))
    }

    fn read_balanced_group_body(&mut self) -> Result<Vec<Token>, ExpandError> {
        let mut body = Vec::new();
        let mut depth = 1_usize;
        while let Some(token) = self.next_unexpanded() {
            match token {
                Token::Character {
                    catcode: CatCode::BeginGroup,
                    ..
                } => {
                    depth += 1;
                    body.push(token);
                }
                Token::Character {
                    catcode: CatCode::EndGroup,
                    ..
                } => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(body);
                    }
                    body.push(token);
                }
                token => body.push(token),
            }
        }
        Err(ExpandError::new("group ended before a matching `}`"))
    }

    fn skip_spaces(&mut self) {
        while self
            .peek_unexpanded()
            .is_some_and(|token| is_space_like(&token))
        {
            let _ = self.next_unexpanded();
        }
    }

    fn consume_optional_keyword(&mut self, keyword: &str) -> bool {
        let mut consumed = Vec::new();
        self.skip_spaces();
        for expected in keyword.chars() {
            let Some(token) = self.next_unexpanded() else {
                self.push_front(consumed);
                return false;
            };
            let matches_expected = matches!(
                token,
                Token::Character {
                    ch,
                    catcode: CatCode::Letter | CatCode::Other,
                } if ch == expected
            );
            consumed.push(token);
            if !matches_expected {
                self.push_front(consumed);
                return false;
            }
        }
        true
    }

    fn next_raw_non_space(&mut self) -> Option<Token> {
        loop {
            let token = self.next_unexpanded()?;
            if !is_space_like(&token) {
                return Some(token);
            }
        }
    }

    fn peek_unexpanded(&mut self) -> Option<Token> {
        let token = self.next_unexpanded()?;
        self.pending.push(token.clone());
        Some(token)
    }

    fn next_unexpanded(&mut self) -> Option<Token> {
        self.pending.pop().or_else(|| self.tokenizer.next_token())
    }

    fn push_front(&mut self, tokens: Vec<Token>) {
        self.pending.extend(tokens.into_iter().rev());
    }
}

pub fn expand_to_tokens(source: &str) -> Result<Vec<Token>, ExpandError> {
    ExpansionEngine::new(source).expand_all()
}

pub fn expand_to_tokens_with_file_context(
    source: &str,
    root_dir: impl Into<PathBuf>,
    job_name: impl Into<String>,
) -> Result<Vec<Token>, ExpandError> {
    ExpansionEngine::with_file_context(source, root_dir, job_name).expand_all()
}

pub fn expand_to_text(source: &str) -> Result<String, ExpandError> {
    Ok(tokens_to_text(&expand_to_tokens(source)?))
}

pub fn expand_to_text_with_file_context(
    source: &str,
    root_dir: impl Into<PathBuf>,
    job_name: impl Into<String>,
) -> Result<String, ExpandError> {
    Ok(tokens_to_text(&expand_to_tokens_with_file_context(
        source, root_dir, job_name,
    )?))
}

pub fn expand_to_source(source: &str) -> Result<String, ExpandError> {
    Ok(tokens_to_source(&expand_to_tokens(source)?))
}

pub fn expand_to_source_with_file_context(
    source: &str,
    root_dir: impl Into<PathBuf>,
    job_name: impl Into<String>,
) -> Result<String, ExpandError> {
    Ok(tokens_to_source(&expand_to_tokens_with_file_context(
        source, root_dir, job_name,
    )?))
}

pub fn tokens_to_text(tokens: &[Token]) -> String {
    let mut out = String::new();
    for token in tokens {
        match token {
            Token::ControlSequence(name) => {
                out.push('\\');
                out.push_str(name);
            }
            Token::ControlSymbol(symbol) => {
                out.push('\\');
                out.push(*symbol);
            }
            Token::Character { ch, catcode } => {
                if *catcode == CatCode::EndOfLine {
                    out.push(' ');
                } else {
                    out.push(*ch);
                }
            }
        }
    }
    out
}

pub fn tokens_to_source(tokens: &[Token]) -> String {
    let mut out = String::new();
    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::ControlSequence(name) => {
                out.push('\\');
                out.push_str(name);
                if matches!(
                    tokens.get(index + 1),
                    Some(Token::Character {
                        catcode: CatCode::Letter,
                        ..
                    })
                ) {
                    out.push(' ');
                }
            }
            Token::ControlSymbol(symbol) => {
                out.push('\\');
                out.push(*symbol);
            }
            Token::Character { ch, .. } => out.push(*ch),
        }
    }
    out
}

fn substitute_macro_arguments(
    definition: &MacroDefinition,
    arguments: &[Vec<Token>],
) -> Result<Vec<Token>, ExpandError> {
    let mut output = Vec::new();
    let mut index = 0_usize;
    while index < definition.replacement().len() {
        let token = &definition.replacement()[index];
        if matches!(
            token,
            Token::Character {
                ch: '#',
                catcode: CatCode::Parameter,
            }
        ) {
            index += 1;
            let Some(parameter) = definition.replacement().get(index) else {
                return Err(ExpandError::new("macro replacement ended after `#`"));
            };
            if matches!(
                parameter,
                Token::Character {
                    ch: '#',
                    catcode: CatCode::Parameter,
                }
            ) {
                output.push(token.clone());
            } else {
                let number = parameter_number(parameter)?;
                let argument = arguments.get(number - 1).ok_or_else(|| {
                    ExpandError::new(format!("macro replacement references missing #{number}"))
                })?;
                output.extend(argument.clone());
            }
        } else {
            output.push(token.clone());
        }
        index += 1;
    }
    Ok(output)
}

fn read_macro_arguments_from_pending(
    pending: &mut Vec<Token>,
    definition: &MacroDefinition,
) -> Result<Vec<Vec<Token>>, ExpandError> {
    let arity = definition.arity();
    let mut arguments = Vec::with_capacity(arity);
    if let Some(default) = definition.optional_default() {
        skip_space_like_pending(pending);
        if let Some(argument) = read_optional_bracket_body_from_pending(pending)? {
            arguments.push(argument);
        } else {
            arguments.push(default.to_vec());
        }
    }

    for _ in arguments.len()..arity {
        arguments.push(read_required_macro_argument_from_pending(pending)?);
    }
    Ok(arguments)
}

fn expand_after_once_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
) -> Result<Vec<Token>, ExpandError> {
    let first = pending
        .pop()
        .ok_or_else(|| ExpandError::new("\\expandafter ended before a first token"))?;
    let second = pending
        .pop()
        .ok_or_else(|| ExpandError::new("\\expandafter ended before a second token"))?;
    let mut tokens = vec![first];
    tokens.extend(expand_one_token_from_pending(engine, second, pending)?);
    Ok(tokens)
}

fn expand_one_token_from_pending(
    engine: &ExpansionEngine<'_>,
    token: Token,
    pending: &mut Vec<Token>,
) -> Result<Vec<Token>, ExpandError> {
    match token {
        Token::ControlSequence(name) if name == "csname" => {
            Ok(vec![read_csname_control_sequence_from_pending(
                engine, pending,
            )?])
        }
        Token::ControlSequence(name) if name == "expandafter" => {
            expand_after_once_from_pending(engine, pending)
        }
        Token::ControlSequence(name) if is_expandable_primitive_name(&name) => {
            Ok(expand_primitive_from_pending(engine, &name, pending)?
                .unwrap_or_else(|| vec![Token::ControlSequence(name)]))
        }
        Token::ControlSequence(name) => {
            if let Some(definition) = engine.macros.get(&name).cloned() {
                let arguments = read_macro_arguments_from_pending(pending, &definition)?;
                substitute_macro_arguments(&definition, &arguments)
            } else {
                Ok(vec![Token::ControlSequence(name)])
            }
        }
        Token::ControlSymbol(symbol) => {
            if let Some(definition) = engine.macros.get(&symbol.to_string()).cloned() {
                let arguments = read_macro_arguments_from_pending(pending, &definition)?;
                substitute_macro_arguments(&definition, &arguments)
            } else {
                Ok(vec![Token::ControlSymbol(symbol)])
            }
        }
        token => Ok(vec![token]),
    }
}

fn read_csname_control_sequence_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
) -> Result<Token, ExpandError> {
    let mut name = String::new();
    loop {
        let Some(token) = pending.pop() else {
            return Err(ExpandError::new(
                "\\csname ended before a matching \\endcsname",
            ));
        };
        if is_end_csname_token(&token) {
            return Ok(Token::ControlSequence(name));
        }
        let expanded = expand_one_token_from_pending(engine, token, pending)?;
        for token in expanded {
            if is_end_csname_token(&token) {
                return Ok(Token::ControlSequence(name));
            }
            append_csname_part(&mut name, &token);
        }
    }
}

fn expand_primitive_from_pending(
    engine: &ExpansionEngine<'_>,
    name: &str,
    pending: &mut Vec<Token>,
) -> Result<Option<Vec<Token>>, ExpandError> {
    if name == "pdfprimitive" {
        return expand_pdfprimitive_from_pending(engine, pending);
    }

    let tokens = match name {
        "number" => integer_tokens(read_integer_from_pending(engine, pending)?),
        "the" => read_the_quantity_tokens_from_pending(engine, pending)?,
        "romannumeral" => roman_numeral_tokens(read_integer_from_pending(engine, pending)?),
        "string" => {
            let token = pending
                .pop()
                .ok_or_else(|| ExpandError::new("\\string ended before a token"))?;
            string_tokens(&token)
        }
        "meaning" => {
            let token = pending
                .pop()
                .ok_or_else(|| ExpandError::new("\\meaning ended before a token"))?;
            meaning_tokens(engine, &token)
        }
        "detokenize" => {
            let body = read_required_group_argument_from_pending(pending, "\\detokenize")?;
            detokenized_tokens(&body)
        }
        "unexpanded" => read_required_group_argument_from_pending(pending, "\\unexpanded")?,
        "expanded" => {
            let body = read_required_group_argument_from_pending(pending, "\\expanded")?;
            engine.expand_known_macros_in_tokens(body)?
        }
        "noexpand" => {
            let token = pending
                .pop()
                .ok_or_else(|| ExpandError::new("\\noexpand ended before a token"))?;
            vec![token]
        }
        "jobname" => engine.jobname_tokens(),
        "pdfcreationdate" => tokens_from_ascii_other(&pdf_date_string(engine.creation_time)),
        "pdffilesize" => {
            let file_name = read_expanded_file_name_from_pending(engine, pending, "\\pdffilesize")?;
            let Some(path) = engine.resolve_pdftex_file(&file_name, "\\pdffilesize")? else {
                return Ok(Some(Vec::new()));
            };
            let size = fs::metadata(&path)
                .map_err(|error| {
                    ExpandError::new(format!(
                        "\\pdffilesize could not read metadata for `{}`: {error}",
                        path.display()
                    ))
                })?
                .len();
            tokens_from_ascii_other(&size.to_string())
        }
        "pdffilemoddate" => {
            let file_name =
                read_expanded_file_name_from_pending(engine, pending, "\\pdffilemoddate")?;
            let Some(path) = engine.resolve_pdftex_file(&file_name, "\\pdffilemoddate")? else {
                return Ok(Some(Vec::new()));
            };
            let modified = fs::metadata(&path)
                .and_then(|metadata| metadata.modified())
                .map_err(|error| {
                    ExpandError::new(format!(
                        "\\pdffilemoddate could not read modification time for `{}`: {error}",
                        path.display()
                    ))
                })?;
            tokens_from_ascii_other(&pdf_date_string(modified))
        }
        "pdffiledump" => {
            let (offset, length) = read_pdffiledump_options_from_pending(engine, pending)?;
            let file_name = read_expanded_file_name_from_pending(engine, pending, "\\pdffiledump")?;
            tokens_from_ascii_other(&engine.pdf_file_dump(&file_name, offset, length)?)
        }
        "pdfstrcmp" => {
            let left = read_expanded_group_text_from_pending(engine, pending, "\\pdfstrcmp")?;
            let right = read_expanded_group_text_from_pending(engine, pending, "\\pdfstrcmp")?;
            integer_tokens(pdfstrcmp_value(&left, &right))
        }
        "pdfescapehex" => {
            let text = read_expanded_group_text_from_pending(engine, pending, "\\pdfescapehex")?;
            tokens_from_ascii_other(&pdf_escape_hex(&text))
        }
        "pdfunescapehex" => {
            let text = read_expanded_group_text_from_pending(engine, pending, "\\pdfunescapehex")?;
            tokens_from_ascii_other(&pdf_unescape_hex(&text)?)
        }
        "pdfescapestring" => {
            let text = read_expanded_group_text_from_pending(engine, pending, "\\pdfescapestring")?;
            tokens_from_ascii_other(&pdf_escape_string(&text))
        }
        "pdfescapename" => {
            let text = read_expanded_group_text_from_pending(engine, pending, "\\pdfescapename")?;
            tokens_from_ascii_other(&pdf_escape_name(&text))
        }
        "pdfmdfivesum" => {
            if consume_optional_keyword_from_pending(pending, "file") {
                let file_name =
                    read_expanded_file_name_from_pending(engine, pending, "\\pdfmdfivesum file")?;
                return Ok(Some(tokens_from_ascii_other(
                    &engine.pdf_file_md5_sum(&file_name)?,
                )));
            }
            let text = read_expanded_group_text_from_pending(engine, pending, "\\pdfmdfivesum")?;
            tokens_from_ascii_other(&pdf_md5_sum(&text))
        }
        _ => return Ok(None),
    };
    Ok(Some(tokens))
}

fn expand_pdfprimitive_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
) -> Result<Option<Vec<Token>>, ExpandError> {
    skip_space_like_pending(pending);
    let Some(token) = pending.last().cloned() else {
        return Err(ExpandError::new("\\pdfprimitive ended before a token"));
    };
    let Some(name) = macro_key(&token) else {
        let _ = pending.pop();
        return Ok(Some(Vec::new()));
    };

    if is_expandable_primitive_name(&name) {
        let _ = pending.pop();
        if let Some(tokens) = expand_primitive_from_pending(engine, &name, pending)? {
            return Ok(Some(tokens));
        }
        return Ok(Some(vec![Token::ControlSequence(name)]));
    }

    if primitive_meaning(&name).is_some() {
        return Ok(None);
    }

    let _ = pending.pop();
    Ok(Some(Vec::new()))
}

fn read_the_quantity_tokens_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
) -> Result<Vec<Token>, ExpandError> {
    skip_space_like_pending(pending);
    match pending.last().cloned() {
        Some(Token::ControlSequence(name)) if name == "toks" => {
            let _ = pending.pop();
            let key = TokenRegisterKey::Numbered(read_non_negative_register_from_pending(
                engine, pending, "\\toks",
            )?);
            Ok(engine.token_register_value(&key))
        }
        Some(Token::ControlSequence(name)) if name == "skip" => {
            let _ = pending.pop();
            let key = SkipRegisterKey::Numbered(read_non_negative_register_from_pending(
                engine, pending, "\\skip",
            )?);
            Ok(glue_tokens(&engine.skip_register_value(&key)))
        }
        Some(Token::ControlSequence(name)) if name == "dimen" => {
            let _ = pending.pop();
            let key = DimensionRegisterKey::Numbered(read_non_negative_register_from_pending(
                engine, pending, "\\dimen",
            )?);
            Ok(dimension_tokens(engine.dimension_register_value(&key)))
        }
        Some(Token::ControlSequence(name)) => {
            if let Some(key) = engine.resolve_token_register_key(&name) {
                let _ = pending.pop();
                Ok(engine.token_register_value(&key))
            } else if let Some(key) = engine.resolve_skip_register_key(&name) {
                let _ = pending.pop();
                Ok(glue_tokens(&engine.skip_register_value(&key)))
            } else if let Some(key) = engine.resolve_dimension_register_key(&name) {
                let _ = pending.pop();
                Ok(dimension_tokens(engine.dimension_register_value(&key)))
            } else {
                Ok(integer_tokens(read_integer_from_pending(engine, pending)?))
            }
        }
        Some(Token::ControlSymbol(symbol)) => {
            let name = symbol.to_string();
            if let Some(key) = engine.resolve_token_register_key(&name) {
                let _ = pending.pop();
                Ok(engine.token_register_value(&key))
            } else if let Some(key) = engine.resolve_skip_register_key(&name) {
                let _ = pending.pop();
                Ok(glue_tokens(&engine.skip_register_value(&key)))
            } else if let Some(key) = engine.resolve_dimension_register_key(&name) {
                let _ = pending.pop();
                Ok(dimension_tokens(engine.dimension_register_value(&key)))
            } else {
                Ok(integer_tokens(read_integer_from_pending(engine, pending)?))
            }
        }
        _ => Ok(integer_tokens(read_integer_from_pending(engine, pending)?)),
    }
}

fn read_integer_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
) -> Result<i64, ExpandError> {
    skip_space_like_pending(pending);
    let sign = read_optional_integer_sign_from_pending(pending);
    skip_space_like_pending(pending);
    let token = pending
        .pop()
        .ok_or_else(|| ExpandError::new("integer expression ended before a value"))?;
    match token {
        Token::Character { ch, .. } if ch.is_ascii_digit() => {
            let mut digits = String::from(ch);
            while let Some(Token::Character { ch, .. }) = pending.last() {
                if !ch.is_ascii_digit() {
                    break;
                }
                digits.push(*ch);
                let _ = pending.pop();
            }
            digits
                .parse::<i64>()
                .map(|value| sign * value)
                .map_err(|_| ExpandError::new("integer expression is too large"))
        }
        Token::ControlSequence(name) if name == "number" || name == "the" => {
            Ok(sign * read_integer_from_pending(engine, pending)?)
        }
        Token::ControlSequence(name) if name == "numexpr" => {
            Ok(sign * read_integer_expression_from_pending(engine, pending)?)
        }
        Token::ControlSequence(name) if name == "count" => {
            let register = read_non_negative_register_from_pending(engine, pending, "\\count")?;
            Ok(sign * engine.count_register_value(&CountRegisterKey::Numbered(register)))
        }
        Token::ControlSequence(name) => {
            if let Some(key) = engine.resolve_count_register_key(&name) {
                Ok(sign * engine.count_register_value(&key))
            } else if let Some(value) = engine.integer_constants.get(&name).copied() {
                Ok(sign * value)
            } else if is_expandable_primitive_name(&name) {
                let tokens = expand_primitive_from_pending(engine, &name, pending)?
                    .unwrap_or_else(|| vec![Token::ControlSequence(name)]);
                if tokens.is_empty() {
                    return Ok(0);
                }
                pending.extend(tokens.into_iter().rev());
                Ok(sign * read_integer_from_pending(engine, pending)?)
            } else if let Some(definition) = engine.macros.get(&name).cloned() {
                let arguments = read_macro_arguments_from_pending(pending, &definition)?;
                let replacement = substitute_macro_arguments(&definition, &arguments)?;
                pending.extend(replacement.into_iter().rev());
                Ok(sign * read_integer_from_pending(engine, pending)?)
            } else {
                Err(ExpandError::new(format!(
                    "integer expression expected digits or a count register, got \\{name}"
                )))
            }
        }
        Token::ControlSymbol(symbol) => {
            let name = symbol.to_string();
            if let Some(key) = engine.resolve_count_register_key(&name) {
                Ok(sign * engine.count_register_value(&key))
            } else if let Some(value) = engine.integer_constants.get(&name).copied() {
                Ok(sign * value)
            } else {
                Err(ExpandError::new(format!(
                    "integer expression expected digits or a count register, got \\{symbol}"
                )))
            }
        }
        token => Err(ExpandError::new(format!(
            "integer expression expected digits or a count register, got {token:?}"
        ))),
    }
}

fn read_integer_expression_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
) -> Result<i64, ExpandError> {
    let mut value = read_integer_product_from_pending(engine, pending)?;
    loop {
        skip_space_like_pending(pending);
        match pending.last() {
            Some(Token::ControlSequence(name)) if name == "relax" => {
                let _ = pending.pop();
                return Ok(value);
            }
            Some(Token::Character { ch: '+', .. }) => {
                let _ = pending.pop();
                let delta = read_integer_product_from_pending(engine, pending)?;
                value = value
                    .checked_add(delta)
                    .ok_or_else(|| ExpandError::new("integer expression is too large"))?;
            }
            Some(Token::Character { ch: '-', .. }) => {
                let _ = pending.pop();
                let delta = read_integer_product_from_pending(engine, pending)?;
                value = value
                    .checked_sub(delta)
                    .ok_or_else(|| ExpandError::new("integer expression is too large"))?;
            }
            _ => return Ok(value),
        }
    }
}

fn read_integer_product_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
) -> Result<i64, ExpandError> {
    let mut value = read_integer_from_pending(engine, pending)?;
    loop {
        skip_space_like_pending(pending);
        match pending.last() {
            Some(Token::Character { ch: '*', .. }) => {
                let _ = pending.pop();
                let factor = read_integer_from_pending(engine, pending)?;
                value = value
                    .checked_mul(factor)
                    .ok_or_else(|| ExpandError::new("integer expression is too large"))?;
            }
            Some(Token::Character { ch: '/', .. }) => {
                let _ = pending.pop();
                let divisor = read_integer_from_pending(engine, pending)?;
                if divisor == 0 {
                    return Err(ExpandError::new("integer expression divided by zero"));
                }
                value /= divisor;
            }
            _ => return Ok(value),
        }
    }
}

fn read_optional_integer_sign_from_pending(pending: &mut Vec<Token>) -> i64 {
    let mut sign = 1_i64;
    loop {
        match pending.last() {
            Some(Token::Character { ch: '+', .. }) => {
                let _ = pending.pop();
            }
            Some(Token::Character { ch: '-', .. }) => {
                let _ = pending.pop();
                sign = -sign;
            }
            _ => return sign,
        }
        skip_space_like_pending(pending);
    }
}

fn read_non_negative_register_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
    primitive: &str,
) -> Result<i64, ExpandError> {
    let register = read_integer_from_pending(engine, pending)?;
    if register < 0 {
        return Err(ExpandError::new(format!(
            "{primitive} requires a non-negative register number, got {register}"
        )));
    }
    Ok(register)
}

fn read_non_negative_usize_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
    primitive: &str,
) -> Result<usize, ExpandError> {
    let value = read_integer_from_pending(engine, pending)?;
    usize::try_from(value).map_err(|_| {
        ExpandError::new(format!(
            "{primitive} requires a non-negative integer, got {value}"
        ))
    })
}

fn read_required_group_argument_from_pending(
    pending: &mut Vec<Token>,
    primitive: &str,
) -> Result<Vec<Token>, ExpandError> {
    skip_space_like_pending(pending);
    match pending.pop() {
        Some(Token::Character {
            catcode: CatCode::BeginGroup,
            ..
        }) => read_balanced_group_body_from_pending(pending),
        Some(token) => Err(ExpandError::new(format!(
            "{primitive} requires a braced token list, got {token:?}"
        ))),
        None => Err(ExpandError::new(format!(
            "{primitive} ended before a braced token list"
        ))),
    }
}

fn read_expanded_group_text_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
    primitive: &str,
) -> Result<String, ExpandError> {
    let body = read_required_group_argument_from_pending(pending, primitive)?;
    let expanded = engine.expand_known_macros_in_tokens(body)?;
    Ok(tokens_to_text(&expanded))
}

fn read_expanded_file_name_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
    primitive: &str,
) -> Result<String, ExpandError> {
    let file_name = read_expanded_group_text_from_pending(engine, pending, primitive)?;
    Ok(engine.normalize_pdftex_file_name(&file_name))
}

fn read_pdffiledump_options_from_pending(
    engine: &ExpansionEngine<'_>,
    pending: &mut Vec<Token>,
) -> Result<(usize, Option<usize>), ExpandError> {
    let mut offset = 0_usize;
    let mut length = None;
    if consume_optional_keyword_from_pending(pending, "offset") {
        offset = read_non_negative_usize_from_pending(engine, pending, "\\pdffiledump offset")?;
    }
    if consume_optional_keyword_from_pending(pending, "length") {
        length = Some(read_non_negative_usize_from_pending(
            engine,
            pending,
            "\\pdffiledump length",
        )?);
    }
    Ok((offset, length))
}

fn read_required_macro_argument_from_pending(
    pending: &mut Vec<Token>,
) -> Result<Vec<Token>, ExpandError> {
    skip_space_like_pending(pending);
    let Some(token) = pending.pop() else {
        return Err(ExpandError::new(
            "macro invocation ended before all arguments",
        ));
    };
    if matches!(
        token,
        Token::Character {
            catcode: CatCode::BeginGroup,
            ..
        }
    ) {
        read_balanced_group_body_from_pending(pending)
    } else {
        Ok(vec![token])
    }
}

fn read_optional_bracket_body_from_pending(
    pending: &mut Vec<Token>,
) -> Result<Option<Vec<Token>>, ExpandError> {
    if !matches!(pending.last(), Some(Token::Character { ch: '[', .. })) {
        return Ok(None);
    }
    let _ = pending.pop();

    let mut body = Vec::new();
    let mut depth = 1_usize;
    while let Some(token) = pending.pop() {
        match token {
            Token::Character { ch: '[', .. } => {
                depth += 1;
                body.push(token);
            }
            Token::Character { ch: ']', .. } => {
                depth -= 1;
                if depth == 0 {
                    return Ok(Some(body));
                }
                body.push(token);
            }
            token => body.push(token),
        }
    }
    Err(ExpandError::new(
        "optional argument ended before a matching `]`",
    ))
}

fn read_balanced_group_body_from_pending(
    pending: &mut Vec<Token>,
) -> Result<Vec<Token>, ExpandError> {
    let mut body = Vec::new();
    let mut depth = 1_usize;
    while let Some(token) = pending.pop() {
        match token {
            Token::Character {
                catcode: CatCode::BeginGroup,
                ..
            } => {
                depth += 1;
                body.push(token);
            }
            Token::Character {
                catcode: CatCode::EndGroup,
                ..
            } => {
                depth -= 1;
                if depth == 0 {
                    return Ok(body);
                }
                body.push(token);
            }
            token => body.push(token),
        }
    }
    Err(ExpandError::new("group ended before a matching `}`"))
}

fn skip_space_like_pending(pending: &mut Vec<Token>) {
    while pending.last().is_some_and(is_space_like) {
        let _ = pending.pop();
    }
}

fn consume_optional_keyword_from_pending(pending: &mut Vec<Token>, keyword: &str) -> bool {
    let mut consumed = Vec::new();
    skip_space_like_pending(pending);
    for expected in keyword.chars() {
        let Some(token) = pending.pop() else {
            pending.extend(consumed.into_iter().rev());
            return false;
        };
        let matches_expected = matches!(
            token,
            Token::Character {
                ch,
                catcode: CatCode::Letter | CatCode::Other,
            } if ch == expected
        );
        consumed.push(token);
        if !matches_expected {
            pending.extend(consumed.into_iter().rev());
            return false;
        }
    }
    true
}

fn parameter_number(token: &Token) -> Result<usize, ExpandError> {
    match token {
        Token::Character { ch, .. } if ('1'..='9').contains(ch) => Ok((*ch as u8 - b'0') as usize),
        _ => Err(ExpandError::new(format!(
            "macro parameter marker must be followed by 1..9, got {token:?}"
        ))),
    }
}

fn parse_usize_tokens(tokens: &[Token], primitive: &str) -> Result<usize, ExpandError> {
    let text = tokens_to_text(tokens).trim().to_string();
    if text.is_empty() || !text.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(ExpandError::new(format!(
            "{primitive} argument count must be numeric, got `{text}`"
        )));
    }
    text.parse::<usize>()
        .map_err(|_| ExpandError::new(format!("{primitive} argument count is too large")))
}

fn catcode_from_number(value: u8) -> Result<CatCode, ExpandError> {
    match value {
        0 => Ok(CatCode::Escape),
        1 => Ok(CatCode::BeginGroup),
        2 => Ok(CatCode::EndGroup),
        3 => Ok(CatCode::MathShift),
        4 => Ok(CatCode::AlignmentTab),
        5 => Ok(CatCode::EndOfLine),
        6 => Ok(CatCode::Parameter),
        7 => Ok(CatCode::Superscript),
        8 => Ok(CatCode::Subscript),
        9 => Ok(CatCode::Ignored),
        10 => Ok(CatCode::Space),
        11 => Ok(CatCode::Letter),
        12 => Ok(CatCode::Other),
        13 => Ok(CatCode::Active),
        14 => Ok(CatCode::Comment),
        15 => Ok(CatCode::Invalid),
        _ => Err(ExpandError::new(format!("invalid catcode value {value}"))),
    }
}

const FIRST_ALLOCATED_COUNT_REGISTER: i64 = 10;
const FIRST_ALLOCATED_DIMENSION_REGISTER: i64 = 10;
const FIRST_ALLOCATED_SKIP_REGISTER: i64 = 10;
const FIRST_ALLOCATED_TOKEN_REGISTER: i64 = 10;
const SCRATCH_COUNT_REGISTER: i64 = 255;
const SCRATCH_DIMENSION_REGISTER: i64 = 255;
const SCRATCH_SKIP_REGISTER: i64 = 255;
const SCRATCH_TOKEN_REGISTER: i64 = 255;
const SP_PER_PT: i64 = 65_536;

fn default_count_registers() -> HashMap<CountRegisterKey, i64> {
    HashMap::from([
        (CountRegisterKey::Numbered(SCRATCH_COUNT_REGISTER), 0),
        (CountRegisterKey::Named("pdfoutput".to_string()), 1),
        (CountRegisterKey::Named("pdfminorversion".to_string()), 7),
        (CountRegisterKey::Named("pdfcompresslevel".to_string()), 9),
        (
            CountRegisterKey::Named("pdfobjcompresslevel".to_string()),
            2,
        ),
        (CountRegisterKey::Named("pdfdraftmode".to_string()), 0),
        (CountRegisterKey::Named("pdfshellescape".to_string()), 0),
    ])
}

fn default_count_aliases() -> HashMap<String, CountRegisterKey> {
    HashMap::from([(
        "count@".to_string(),
        CountRegisterKey::Numbered(SCRATCH_COUNT_REGISTER),
    )])
}

fn default_dimension_registers() -> HashMap<DimensionRegisterKey, i64> {
    HashMap::from([
        (
            DimensionRegisterKey::Numbered(SCRATCH_DIMENSION_REGISTER),
            0,
        ),
        (DimensionRegisterKey::Named("z@".to_string()), 0),
        (DimensionRegisterKey::Named("p@".to_string()), SP_PER_PT),
        (
            DimensionRegisterKey::Named("pdfpageheight".to_string()),
            default_dimension_sp(297.0, "truemm"),
        ),
        (
            DimensionRegisterKey::Named("pdfpagewidth".to_string()),
            default_dimension_sp(210.0, "truemm"),
        ),
        (
            DimensionRegisterKey::Named("pdfhorigin".to_string()),
            default_dimension_sp(1.0, "truein"),
        ),
        (
            DimensionRegisterKey::Named("pdfvorigin".to_string()),
            default_dimension_sp(1.0, "truein"),
        ),
    ])
}

fn default_dimension_aliases() -> HashMap<String, DimensionRegisterKey> {
    HashMap::from([
        (
            "dimen@".to_string(),
            DimensionRegisterKey::Numbered(SCRATCH_DIMENSION_REGISTER),
        ),
        (
            "z@".to_string(),
            DimensionRegisterKey::Named("z@".to_string()),
        ),
        (
            "p@".to_string(),
            DimensionRegisterKey::Named("p@".to_string()),
        ),
    ])
}

fn default_skip_registers() -> HashMap<SkipRegisterKey, GlueValue> {
    HashMap::from([
        (
            SkipRegisterKey::Numbered(SCRATCH_SKIP_REGISTER),
            zero_glue(),
        ),
        (SkipRegisterKey::Named("z@skip".to_string()), zero_glue()),
    ])
}

fn default_skip_aliases() -> HashMap<String, SkipRegisterKey> {
    HashMap::from([
        (
            "skip@".to_string(),
            SkipRegisterKey::Numbered(SCRATCH_SKIP_REGISTER),
        ),
        (
            "z@skip".to_string(),
            SkipRegisterKey::Named("z@skip".to_string()),
        ),
    ])
}

fn default_token_registers() -> HashMap<TokenRegisterKey, Vec<Token>> {
    HashMap::from([
        (
            TokenRegisterKey::Numbered(SCRATCH_TOKEN_REGISTER),
            Vec::new(),
        ),
        (
            TokenRegisterKey::Named("pdfpageattr".to_string()),
            Vec::new(),
        ),
        (
            TokenRegisterKey::Named("pdfpageresources".to_string()),
            Vec::new(),
        ),
    ])
}

fn default_token_aliases() -> HashMap<String, TokenRegisterKey> {
    HashMap::from([(
        "toks@".to_string(),
        TokenRegisterKey::Numbered(SCRATCH_TOKEN_REGISTER),
    )])
}

fn default_integer_constants() -> HashMap<String, i64> {
    HashMap::from([
        ("m@ne".to_string(), -1),
        ("@ne".to_string(), 1),
        ("tw@".to_string(), 2),
        ("thr@@".to_string(), 3),
        ("sixt@@n".to_string(), 16),
        ("pdftexversion".to_string(), 140),
        ("pdftexrevision".to_string(), 29),
    ])
}

fn default_dimension_sp(value: f64, unit: &str) -> i64 {
    decimal_dimension_to_sp(value, unit).expect("valid pdfTeX default dimension")
}

fn pdfstrcmp_value(left: &str, right: &str) -> i64 {
    match left.as_bytes().cmp(right.as_bytes()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

fn pdf_escape_hex(text: &str) -> String {
    pdf_escape_hex_bytes(text.as_bytes())
}

fn pdf_escape_hex_bytes(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        write!(&mut output, "{byte:02X}").expect("writing to a string cannot fail");
    }
    output
}

fn pdf_unescape_hex(text: &str) -> Result<String, ExpandError> {
    let hex = text
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect::<String>();
    if hex.len() % 2 != 0 {
        return Err(ExpandError::new(
            "\\pdfunescapehex requires an even number of hex digits",
        ));
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut chars = hex.chars();
    while let Some(high) = chars.next() {
        let low = chars
            .next()
            .expect("hex string length was checked to be even");
        let Some(high) = high.to_digit(16) else {
            return Err(ExpandError::new(
                "\\pdfunescapehex encountered a non-hex digit",
            ));
        };
        let Some(low) = low.to_digit(16) else {
            return Err(ExpandError::new(
                "\\pdfunescapehex encountered a non-hex digit",
            ));
        };
        bytes.push(((high << 4) | low) as u8);
    }
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn pdf_escape_string(text: &str) -> String {
    let mut output = String::new();
    for byte in text.as_bytes() {
        match *byte {
            b'(' => output.push_str("\\("),
            b')' => output.push_str("\\)"),
            b'\\' => output.push_str("\\\\"),
            b'\n' => output.push_str("\\012"),
            b'\r' => output.push_str("\\015"),
            b'\t' => output.push_str("\\011"),
            0x20 | 0x00..=0x1f | 0x7f..=0xff => {
                use std::fmt::Write as _;
                write!(&mut output, "\\{byte:03o}").expect("writing to a string cannot fail");
            }
            _ => output.push(*byte as char),
        }
    }
    output
}

fn pdf_escape_name(text: &str) -> String {
    let mut output = String::new();
    for byte in text.as_bytes() {
        if *byte <= 0x20
            || *byte >= 0x7f
            || matches!(
                *byte,
                b'#' | b'%' | b'(' | b')' | b'/' | b'<' | b'>' | b'[' | b']' | b'{' | b'}'
            )
        {
            use std::fmt::Write as _;
            write!(&mut output, "#{byte:02X}").expect("writing to a string cannot fail");
        } else {
            output.push(*byte as char);
        }
    }
    output
}

fn pdf_md5_sum(text: &str) -> String {
    pdf_md5_sum_bytes(text.as_bytes())
}

fn pdf_md5_sum_bytes(bytes: &[u8]) -> String {
    let digest = Md5::digest(bytes);
    let mut output = String::with_capacity(32);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut output, "{byte:02X}").expect("writing to a string cannot fail");
    }
    output
}

fn pdf_date_string(time: SystemTime) -> String {
    let date_time: DateTime<Local> = time.into();
    let offset = date_time.format("%z").to_string();
    let zone = if offset == "+0000" || offset == "-0000" {
        "Z".to_string()
    } else {
        format!("{}'{}'", &offset[..3], &offset[3..])
    };
    format!("D:{}{}", date_time.format("%Y%m%d%H%M%S"), zone)
}

fn decimal_dimension_to_sp(value: f64, unit: &str) -> Result<i64, ExpandError> {
    let unit = unit.strip_prefix("true").unwrap_or(unit);
    let scale = match unit {
        "sp" => 1.0,
        "pt" => SP_PER_PT as f64,
        "pc" => 12.0 * SP_PER_PT as f64,
        "in" => 72.27 * SP_PER_PT as f64,
        "bp" => (72.27 / 72.0) * SP_PER_PT as f64,
        "cm" => (72.27 / 2.54) * SP_PER_PT as f64,
        "mm" => (72.27 / 25.4) * SP_PER_PT as f64,
        "dd" => (1238.0 / 1157.0) * SP_PER_PT as f64,
        "cc" => 12.0 * (1238.0 / 1157.0) * SP_PER_PT as f64,
        "em" => 10.0 * SP_PER_PT as f64,
        "ex" => 5.0 * SP_PER_PT as f64,
        _ => {
            return Err(ExpandError::new(format!(
                "unsupported dimension unit `{unit}`"
            )));
        }
    };
    let scaled = (value * scale).round();
    if !scaled.is_finite() || scaled < i64::MIN as f64 || scaled > i64::MAX as f64 {
        return Err(ExpandError::new("dimension value is too large"));
    }
    Ok(scaled as i64)
}

fn zero_glue() -> GlueValue {
    GlueValue {
        width: 0,
        stretch: 0,
        shrink: 0,
    }
}

fn integer_tokens(value: i64) -> Vec<Token> {
    value
        .to_string()
        .chars()
        .map(|ch| Token::Character {
            ch,
            catcode: CatCode::Other,
        })
        .collect()
}

fn dimension_tokens(value: i64) -> Vec<Token> {
    tokens_from_ascii(&format_dimension(value))
}

fn glue_tokens(value: &GlueValue) -> Vec<Token> {
    tokens_from_ascii(&format_glue(value))
}

fn roman_numeral_tokens(value: i64) -> Vec<Token> {
    tokens_from_ascii(&format_roman_numeral(value))
}

fn format_glue(value: &GlueValue) -> String {
    let mut text = format_dimension(value.width);
    if value.stretch != 0 {
        text.push_str(" plus ");
        text.push_str(&format_dimension(value.stretch));
    }
    if value.shrink != 0 {
        text.push_str(" minus ");
        text.push_str(&format_dimension(value.shrink));
    }
    text
}

fn format_dimension(value: i64) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let absolute = value.unsigned_abs();
    let whole = absolute / SP_PER_PT as u64;
    let fractional_sp = absolute % SP_PER_PT as u64;
    let fractional = ((fractional_sp as f64 / SP_PER_PT as f64) * 100_000.0).round() as u64;
    if fractional == 0 {
        return format!("{sign}{whole}.0pt");
    }
    let mut fraction = format!("{fractional:05}");
    while fraction.ends_with('0') {
        fraction.pop();
    }
    format!("{sign}{whole}.{fraction}pt")
}

fn format_roman_numeral(value: i64) -> String {
    if value <= 0 {
        return String::new();
    }
    let mut value = value.min(100_000);
    let mut output = String::new();
    for (unit, text) in [
        (1000, "m"),
        (900, "cm"),
        (500, "d"),
        (400, "cd"),
        (100, "c"),
        (90, "xc"),
        (50, "l"),
        (40, "xl"),
        (10, "x"),
        (9, "ix"),
        (5, "v"),
        (4, "iv"),
        (1, "i"),
    ] {
        while value >= unit {
            output.push_str(text);
            value -= unit;
        }
    }
    output
}

fn tokens_from_ascii(value: &str) -> Vec<Token> {
    value
        .chars()
        .map(|ch| Token::Character {
            ch,
            catcode: if ch.is_ascii_alphabetic() {
                CatCode::Letter
            } else {
                CatCode::Other
            },
        })
        .collect()
}

fn tokens_from_ascii_other(value: &str) -> Vec<Token> {
    value
        .chars()
        .map(|ch| Token::Character {
            ch,
            catcode: if ch == ' ' {
                CatCode::Space
            } else {
                CatCode::Other
            },
        })
        .collect()
}

fn string_tokens(token: &Token) -> Vec<Token> {
    tokens_from_ascii_other(&stringify_token(token))
}

fn meaning_tokens(engine: &ExpansionEngine<'_>, token: &Token) -> Vec<Token> {
    tokens_from_ascii_other(&meaning_text(engine, token))
}

fn detokenized_tokens(tokens: &[Token]) -> Vec<Token> {
    tokens_from_ascii_other(&tokens_to_source(tokens))
}

fn stringify_token(token: &Token) -> String {
    match token {
        Token::ControlSequence(name) => format!("\\{name}"),
        Token::ControlSymbol(symbol) => format!("\\{symbol}"),
        Token::Character { ch, .. } => ch.to_string(),
    }
}

fn meaning_text(engine: &ExpansionEngine<'_>, token: &Token) -> String {
    if let Some(name) = macro_key(token) {
        if let Some(definition) = engine.macros.get(&name) {
            if definition.is_protected() {
                return format!(
                    "protected macro:->{}",
                    tokens_to_source(definition.replacement())
                );
            }
            return format!("macro:->{}", tokens_to_source(definition.replacement()));
        }
        if let Some(meaning) = primitive_meaning(&name) {
            return format!("\\{meaning}");
        }
        if engine.is_defined_control_name(&name) {
            return format!("defined control sequence \\{name}");
        }
        return "undefined".to_string();
    }
    match token {
        Token::Character { ch, catcode } => format!("{catcode:?} character {ch}"),
        Token::ControlSequence(_) | Token::ControlSymbol(_) => unreachable!("handled by macro_key"),
    }
}

fn is_expandable_primitive_name(name: &str) -> bool {
    matches!(
        name,
        "csname"
            | "expandafter"
            | "number"
            | "the"
            | "romannumeral"
            | "string"
            | "meaning"
            | "detokenize"
            | "unexpanded"
            | "expanded"
            | "noexpand"
            | "jobname"
            | "pdfprimitive"
            | "pdfcreationdate"
            | "pdffilesize"
            | "pdffilemoddate"
            | "pdffiledump"
            | "pdfstrcmp"
            | "pdfescapehex"
            | "pdfunescapehex"
            | "pdfescapestring"
            | "pdfescapename"
            | "pdfmdfivesum"
    )
}

fn primitive_meaning(name: &str) -> Option<&'static str> {
    match name {
        "relax" => Some("relax"),
        "def" => Some("def"),
        "gdef" => Some("gdef"),
        "edef" => Some("edef"),
        "xdef" => Some("xdef"),
        "protected@edef" => Some("protected@edef"),
        "protected" => Some("protected"),
        "newcommand" => Some("newcommand"),
        "renewcommand" => Some("renewcommand"),
        "providecommand" => Some("providecommand"),
        "DeclareRobustCommand" => Some("DeclareRobustCommand"),
        "DeclareMathOperator" => Some("DeclareMathOperator"),
        "futurelet" => Some("futurelet"),
        "aftergroup" => Some("aftergroup"),
        "iftrue" => Some("iftrue"),
        "iffalse" => Some("iffalse"),
        "ifx" => Some("ifx"),
        "if" => Some("if"),
        "ifcat" => Some("ifcat"),
        "ifdefined" => Some("ifdefined"),
        "ifcsname" => Some("ifcsname"),
        "ifpdfprimitive" => Some("ifpdfprimitive"),
        "ifnum" => Some("ifnum"),
        "ifdim" => Some("ifdim"),
        "ifodd" => Some("ifodd"),
        "ifcase" => Some("ifcase"),
        "unless" => Some("unless"),
        "newif" => Some("newif"),
        "newcount" => Some("newcount"),
        "countdef" => Some("countdef"),
        "newdimen" => Some("newdimen"),
        "dimendef" => Some("dimendef"),
        "newskip" => Some("newskip"),
        "skipdef" => Some("skipdef"),
        "newtoks" => Some("newtoks"),
        "toksdef" => Some("toksdef"),
        "chardef" => Some("chardef"),
        "mathchardef" => Some("mathchardef"),
        "advance" => Some("advance"),
        "number" => Some("number"),
        "the" => Some("the"),
        "romannumeral" => Some("romannumeral"),
        "string" => Some("string"),
        "meaning" => Some("meaning"),
        "detokenize" => Some("detokenize"),
        "unexpanded" => Some("unexpanded"),
        "expanded" => Some("expanded"),
        "noexpand" => Some("noexpand"),
        "jobname" => Some("jobname"),
        "pdfprimitive" => Some("pdfprimitive"),
        "pdfcreationdate" => Some("pdfcreationdate"),
        "pdffilesize" => Some("pdffilesize"),
        "pdffilemoddate" => Some("pdffilemoddate"),
        "pdffiledump" => Some("pdffiledump"),
        "pdfstrcmp" => Some("pdfstrcmp"),
        "pdfescapehex" => Some("pdfescapehex"),
        "pdfunescapehex" => Some("pdfunescapehex"),
        "pdfescapestring" => Some("pdfescapestring"),
        "pdfescapename" => Some("pdfescapename"),
        "pdfmdfivesum" => Some("pdfmdfivesum"),
        "pdfoutput" => Some("pdfoutput"),
        "pdfminorversion" => Some("pdfminorversion"),
        "pdfcompresslevel" => Some("pdfcompresslevel"),
        "pdfobjcompresslevel" => Some("pdfobjcompresslevel"),
        "pdfdraftmode" => Some("pdfdraftmode"),
        "pdfshellescape" => Some("pdfshellescape"),
        "pdfpageheight" => Some("pdfpageheight"),
        "pdfpagewidth" => Some("pdfpagewidth"),
        "pdfhorigin" => Some("pdfhorigin"),
        "pdfvorigin" => Some("pdfvorigin"),
        "pdfpageattr" => Some("pdfpageattr"),
        "pdfpageresources" => Some("pdfpageresources"),
        "pdftexversion" => Some("pdftexversion"),
        "pdftexrevision" => Some("pdftexrevision"),
        "let" => Some("let"),
        "global" => Some("global"),
        "catcode" => Some("catcode"),
        "makeatletter" => Some("makeatletter"),
        "makeatother" => Some("makeatother"),
        _ => None,
    }
}

fn macro_key(token: &Token) -> Option<String> {
    match token {
        Token::ControlSequence(name) => Some(name.clone()),
        Token::ControlSymbol(symbol) => Some(symbol.to_string()),
        Token::Character { .. } => None,
    }
}

fn tokens_have_same_character_code(left: &Token, right: &Token) -> bool {
    match (left, right) {
        (Token::Character { ch: left, .. }, Token::Character { ch: right, .. }) => left == right,
        (Token::ControlSymbol(left), Token::ControlSymbol(right)) => left == right,
        _ => false,
    }
}

fn tokens_have_same_category(left: &Token, right: &Token) -> bool {
    token_category(left) == token_category(right)
}

fn token_category(token: &Token) -> TokenCategory {
    match token {
        Token::Character { catcode, .. } => TokenCategory::Character(*catcode),
        Token::ControlSequence(_) | Token::ControlSymbol(_) => TokenCategory::ControlSequence,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenCategory {
    Character(CatCode),
    ControlSequence,
}

fn is_end_csname_token(token: &Token) -> bool {
    matches!(token, Token::ControlSequence(name) if name == "endcsname")
}

fn append_csname_part(name: &mut String, token: &Token) {
    match token {
        Token::ControlSequence(part) => name.push_str(part),
        Token::ControlSymbol(symbol) => name.push(*symbol),
        Token::Character { ch, .. } if !is_space_like(token) => name.push(*ch),
        Token::Character { .. } => {}
    }
}

fn is_space_like(token: &Token) -> bool {
    matches!(
        token,
        Token::Character {
            catcode: CatCode::Space | CatCode::EndOfLine,
            ..
        }
    )
}

#[derive(Debug, Clone)]
struct StreamingTokenizer<'a> {
    source: std::str::Chars<'a>,
    catcodes: CatCodeTable,
    skipping_comment: bool,
}

impl<'a> StreamingTokenizer<'a> {
    fn new(source: &'a str) -> Self {
        Self::with_catcodes(source, CatCodeTable::default())
    }

    fn with_catcodes(source: &'a str, catcodes: CatCodeTable) -> Self {
        Self {
            source: source.chars(),
            catcodes,
            skipping_comment: false,
        }
    }

    fn set_ascii_catcode(&mut self, ch: u8, catcode: CatCode) {
        self.catcodes.set_ascii(ch, catcode);
    }

    fn catcodes(&self) -> CatCodeTable {
        self.catcodes.clone()
    }

    fn set_catcodes(&mut self, catcodes: CatCodeTable) {
        self.catcodes = catcodes;
    }

    fn next_token(&mut self) -> Option<Token> {
        loop {
            let ch = self.source.next()?;
            let catcode = self.catcodes.get(ch);
            if self.skipping_comment {
                if catcode == CatCode::EndOfLine {
                    self.skipping_comment = false;
                    return Some(Token::Character {
                        ch,
                        catcode: CatCode::EndOfLine,
                    });
                }
                continue;
            }

            match catcode {
                CatCode::Escape => return Some(self.read_control()),
                CatCode::Comment => {
                    self.skipping_comment = true;
                    continue;
                }
                CatCode::Ignored => continue,
                CatCode::EndOfLine => {
                    return Some(Token::Character {
                        ch,
                        catcode: CatCode::EndOfLine,
                    });
                }
                _ => return Some(Token::Character { ch, catcode }),
            }
        }
    }

    fn read_control(&mut self) -> Token {
        let Some(next) = self.source.next() else {
            return Token::ControlSymbol(' ');
        };
        if self.catcodes.get(next) != CatCode::Letter {
            return Token::ControlSymbol(next);
        }

        let mut name = String::from(next);
        let mut tail = self.source.clone();
        while let Some(ch) = tail.next() {
            if self.catcodes.get(ch) != CatCode::Letter {
                break;
            }
            name.push(ch);
            self.source = tail.clone();
        }

        let mut tail = self.source.clone();
        while let Some(ch) = tail.next() {
            if self.catcodes.get(ch) != CatCode::Space {
                break;
            }
            self.source = tail.clone();
        }

        Token::ControlSequence(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_expand_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "texpilot-pdftex-expand-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&path);
        path
    }

    #[test]
    fn expands_simple_definitions() {
        assert_eq!(expand_to_text(r"\def\hello{Hi}\hello").unwrap(), "Hi");
    }

    #[test]
    fn expands_global_definitions_like_definitions() {
        assert_eq!(expand_to_text(r"\gdef\hello{Hi}\hello").unwrap(), "Hi");
    }

    #[test]
    fn local_definitions_are_restored_after_brace_groups() {
        assert_eq!(
            expand_to_text(r"\def\a{outer}{\def\a{inner}\a}\a").unwrap(),
            "{inner}outer"
        );
    }

    #[test]
    fn local_definitions_are_restored_after_begingroup() {
        assert_eq!(
            expand_to_text(r"\def\a{outer}\begingroup\def\a{inner}\a\endgroup\a").unwrap(),
            "innerouter"
        );
    }

    #[test]
    fn global_definitions_survive_group_restoration() {
        assert_eq!(
            expand_to_text(r"\begingroup\gdef\a{global}\endgroup\a").unwrap(),
            "global"
        );
        assert_eq!(
            expand_to_text(r"\begingroup\global\def\a{global}\endgroup\a").unwrap(),
            "global"
        );
        assert_eq!(
            expand_to_text(r"\def\a{A}\begingroup\global\edef\b{\a}\def\a{B}\endgroup\b").unwrap(),
            "A"
        );
    }

    #[test]
    fn global_let_survives_group_restoration() {
        assert_eq!(
            expand_to_text(r"\def\a{A}\begingroup\global\let\b=\a\endgroup\b").unwrap(),
            "A"
        );
    }

    #[test]
    fn conditionals_select_the_active_branch() {
        assert_eq!(
            expand_to_text(r"\iftrue yes\else no\fi|\iffalse yes\else no\fi").unwrap(),
            "yes|no"
        );
    }

    #[test]
    fn conditionals_skip_inactive_branch_side_effects() {
        assert_eq!(
            expand_to_text(r"\def\a{old}\iffalse\def\a{bad}\else\def\b{good}\fi\a\b").unwrap(),
            "oldgood"
        );
    }

    #[test]
    fn conditionals_track_nested_conditionals_when_selecting_branches() {
        assert_eq!(
            expand_to_text(r"\iffalse A\iftrue bad\else bad\fi\else B\iffalse bad\else C\fi\fi")
                .unwrap(),
            "BC"
        );
    }

    #[test]
    fn ifx_compares_macro_meanings_without_expanding_operands() {
        assert_eq!(
            expand_to_text(
                r"\def\a{A}\let\b=\a\def\c{C}\ifx\a\b same\else different\fi|\ifx\a\c same\else different\fi"
            )
            .unwrap(),
            "same|different"
        );
    }

    #[test]
    fn futurelet_assigns_the_following_token_meaning_without_consuming_tokens() {
        assert_eq!(
            expand_to_text(
                r"\def\b{B}\def\probe#1{\ifx\next\b yes:#1\else no:#1\fi}\futurelet\next\probe\b"
            )
            .unwrap(),
            "yes:B"
        );
        assert_eq!(
            expand_to_text(
                r"\def\b{B}\begingroup\global\futurelet\next\relax\b\endgroup\ifx\next\b yes\else no\fi"
            )
            .unwrap(),
            "Byes"
        );
    }

    #[test]
    fn aftergroup_inserts_tokens_after_group_restoration() {
        assert_eq!(
            expand_to_text(r"\def\a{A}\begingroup\def\a{local}\aftergroup\a\endgroup\a").unwrap(),
            "AA"
        );
        assert_eq!(
            expand_to_text(r"\begingroup\aftergroup A\aftergroup B\endgroup").unwrap(),
            "AB"
        );
    }

    #[test]
    fn relax_is_defined_and_inert() {
        assert_eq!(
            expand_to_text(
                r"\ifdefined\relax yes\else no\fi|\ifx\relax\missing same\else distinct\fi|\relax done"
            )
            .unwrap(),
            "yes|distinct|done"
        );
    }

    #[test]
    fn if_compares_expanded_character_codes() {
        assert_eq!(
            expand_to_text(
                r"\def\a{x}\if\a x same\else different\fi|\if ab same\else different\fi"
            )
            .unwrap(),
            "same|different"
        );
        assert_eq!(
            expand_to_text(r"\def\a{\b}\def\b{z}\if\a z nested\else bad\fi").unwrap(),
            "nested"
        );
    }

    #[test]
    fn ifcat_compares_expanded_token_categories() {
        assert_eq!(
            expand_to_text(r"\ifcat a b letters\else bad\fi|\ifcat a1 bad\else different\fi")
                .unwrap(),
            "letters|different"
        );
        assert_eq!(
            expand_to_text(r"\def\a{1}\ifcat\a 2 others\else bad\fi").unwrap(),
            "others"
        );
    }

    #[test]
    fn ifdefined_detects_implemented_assignment_meanings() {
        assert_eq!(
            expand_to_text(
                r"\ifdefined\a before\else missing\fi|\def\a{A}\ifdefined\a after\else missing\fi"
            )
            .unwrap(),
            "missing|after"
        );
    }

    #[test]
    fn ifcsname_detects_defined_names_without_defining_missing_ones() {
        assert_eq!(
            expand_to_text(
                r"\def\a{A}\ifcsname a\endcsname yes\else no\fi|\ifcsname missing\endcsname bad\else missing\fi|\ifdefined\missing bad\else clean\fi"
            )
            .unwrap(),
            "yes|missing|clean"
        );
        assert_eq!(
            expand_to_text(
                r"\def\stem{target}\def\target{T}\ifcsname \stem\endcsname found\else missing\fi"
            )
            .unwrap(),
            "found"
        );
    }

    #[test]
    fn unless_inverts_boolean_conditionals() {
        assert_eq!(
            expand_to_text(
                r"\unless\ifdefined\missing yes\else no\fi|\def\a{A}\unless\ifcsname a\endcsname bad\else ok\fi"
            )
            .unwrap(),
            "yes|ok"
        );
        assert_eq!(
            expand_to_text(r"\newif\ifdraft\unless\ifdraft off\else on\fi\drafttrue\unless\ifdraft bad\else active\fi")
                .unwrap(),
            "offactive"
        );
    }

    #[test]
    fn newif_booleans_are_scoped_and_assignable() {
        assert_eq!(
            expand_to_text(
                r"\newif\ifdraft\ifdraft on\else off\fi\drafttrue\ifdraft on\else off\fi"
            )
            .unwrap(),
            "offon"
        );
        assert_eq!(
            expand_to_text(
                r"\newif\ifdraft\begingroup\drafttrue\ifdraft local\else bad\fi\endgroup\ifdraft bad\else restored\fi"
            )
            .unwrap(),
            "localrestored"
        );
        assert_eq!(
            expand_to_text(
                r"\newif\ifdraft\begingroup\global\drafttrue\endgroup\ifdraft global\else bad\fi"
            )
            .unwrap(),
            "global"
        );
    }

    #[test]
    fn count_registers_expand_through_number_and_the() {
        assert_eq!(
            expand_to_text(r"\newcount\n\n=3\number\n|\the\n").unwrap(),
            "3|3"
        );
        assert_eq!(
            expand_to_text(r"\makeatletter\count@=12\number\count@\makeatother").unwrap(),
            "12"
        );
    }

    #[test]
    fn pdftex_count_registers_are_defined_assignable_and_branchable() {
        assert_eq!(
            expand_to_text(
                r"\ifx\pdfoutput\undefined no\else yes\fi|\ifnum\pdfoutput>0 pdf\else dvi\fi|\pdfoutput=0\ifnum\pdfoutput=0 dvi\else pdf\fi|\begingroup\pdfminorversion=5\the\pdfminorversion\endgroup\the\pdfminorversion|\begingroup\global\pdfcompresslevel 0\endgroup\the\pdfcompresslevel"
            )
            .unwrap(),
            "yes|pdf|dvi|57|0"
        );
    }

    #[test]
    fn count_registers_support_advance_and_signed_values() {
        assert_eq!(
            expand_to_text(r"\newcount\n\n=3\advance\n by 4\number\n").unwrap(),
            "7"
        );
        assert_eq!(
            expand_to_text(r"\newcount\n\n=-3\advance\n by +1\number\n").unwrap(),
            "-2"
        );
    }

    #[test]
    fn numexpr_supports_simple_integer_arithmetic() {
        assert_eq!(
            expand_to_text(r"\number\numexpr1+2*3-4/2\relax").unwrap(),
            "5"
        );
        assert_eq!(
            expand_to_text(r"\newcount\n\n=5\n=\numexpr\n*2+1\relax\number\n").unwrap(),
            "11"
        );
        assert_eq!(
            expand_to_text(r"\ifnum\numexpr3*4\relax>10 yes\else no\fi").unwrap(),
            "yes"
        );
    }

    #[test]
    fn expandable_primitives_render_strings_and_roman_numerals() {
        assert_eq!(
            expand_to_text(r"\romannumeral14|\romannumeral0|\string\alpha").unwrap(),
            r"xiv||\alpha"
        );
        assert_eq!(
            expand_to_text(r"\def\a{A}\meaning\a|\meaning\missing").unwrap(),
            "macro:->A|undefined"
        );
        assert_eq!(
            expand_to_text(r"\detokenize{\alpha{x}}").unwrap(),
            r"\alpha{x}"
        );
        assert_eq!(expand_to_text(r"\def\a{A}\expanded{\a B}").unwrap(), "AB");
        assert_eq!(
            expand_to_text(r"\protected\def\a{A}\expanded{\a}").unwrap(),
            "A"
        );
    }

    #[test]
    fn pdftex_expandable_string_primitives_match_common_pdftex_outputs() {
        assert_eq!(
            expand_to_text(r"\def\foo{A B}\pdfstrcmp{\foo}{A B}|\pdfstrcmp{a}{b}|\pdfstrcmp{b}{a}")
                .unwrap(),
            "0|-1|1"
        );
        assert_eq!(
            expand_to_text(r"\pdfescapehex{A ()#}|\pdfunescapehex{4120282923}").unwrap(),
            "4120282923|A ()#"
        );
        assert_eq!(
            expand_to_text(r"\pdfescapestring{A (x) y}").unwrap(),
            r"A\040\(x\)\040y"
        );
        assert_eq!(
            expand_to_text(r"\pdfescapename{A /()}").unwrap(),
            "A#20#2F#28#29"
        );
        assert_eq!(
            expand_to_text(r"\pdfmdfivesum{abc}").unwrap(),
            "900150983CD24FB0D6963F7D28E17F72"
        );
        assert_eq!(
            expand_to_text(r"\ifnum\pdfstrcmp{a}{b}<0 yes\else no\fi|\the\pdfshellescape").unwrap(),
            "yes|0"
        );
    }

    #[test]
    fn pdftex_expandable_string_primitives_work_inside_edef() {
        assert_eq!(
            expand_to_text(
                r"\def\foo{A B}\edef\a{\pdfstrcmp{\foo}{A B}|\pdfescapehex{\foo}|\pdfunescapehex{412042}|\pdfmdfivesum{abc}}\a"
            )
            .unwrap(),
            "0|412042|A B|900150983CD24FB0D6963F7D28E17F72"
        );
    }

    #[test]
    fn pdftex_primitive_probe_primitives_match_common_pdftex_paths() {
        assert_eq!(
            expand_to_text(
                r"\ifpdfprimitive\pdfstrcmp yes\else no\fi|\ifpdfprimitive\missing bad\else missing\fi|\let\alias\pdfstrcmp\ifpdfprimitive\alias bad\else alias-no\fi"
            )
            .unwrap(),
            "yes|missing|alias-no"
        );
        assert_eq!(
            expand_to_text(
                r"\ifpdfprimitive\pdfoutput count\else bad\fi|\ifpdfprimitive\pdfpagewidth dimen\else bad\fi|\ifpdfprimitive\pdfpageattr toks\else bad\fi|\ifpdfprimitive\pdftexversion version\else bad\fi"
            )
            .unwrap(),
            "count|dimen|toks|version"
        );
        assert_eq!(
            expand_to_text(
                r"\meaning\pdfstrcmp|\meaning\pdfoutput|\meaning\pdftexversion|\meaning\pdfprimitive|\meaning\ifpdfprimitive"
            )
            .unwrap(),
            r"\pdfstrcmp|\pdfoutput|\pdftexversion|\pdfprimitive|\ifpdfprimitive"
        );
        assert_eq!(
            expand_to_text(r"\number\pdftexversion|\number\pdftexrevision").unwrap(),
            "140|29"
        );
    }

    #[test]
    fn pdfprimitive_expands_named_expandable_primitives_not_aliases() {
        assert_eq!(
            expand_to_text(
                r"\pdfprimitive\pdfstrcmp{a}{b}|\edef\x{\pdfprimitive\pdfescapehex{A}}\x|\ifnum\pdfprimitive\pdfstrcmp{a}{b}<0 yes\else no\fi"
            )
            .unwrap(),
            "-1|41|yes"
        );
        assert_eq!(
            expand_to_text(
                r"\let\prim\pdfprimitive\prim\pdfstrcmp{a}{a}|\let\alias\pdfstrcmp\pdfprimitive\alias{a}{a}"
            )
            .unwrap(),
            "0|{a}{a}"
        );
    }

    #[test]
    fn pdftex_file_primitives_use_native_file_context() {
        let root = temp_expand_dir("pdftex-file-primitives");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("data.txt"), b"abc").unwrap();
        fs::write(root.join("main.tex"), b"abcd").unwrap();

        let expanded = expand_to_text_with_file_context(
            r"\jobname|\pdffilesize{data.txt}|\pdffilesize{main}|\pdffilemoddate{data.txt}|\pdfmdfivesum file {data.txt}|\edef\h{\pdfmdfivesum file {data.txt}}\h|\pdffiledump offset 1 length 2 {data.txt}|\def\filedump#1#2#3{\pdffiledump offset#1 length#2{#3}}\edef\d{\filedump{1}{2}{data.txt}}\d|\pdffiledump offset 1 length 2 {missing.txt}|\ifnum\pdffilesize{missing.txt}=0 missing\else bad\fi|\ifpdfprimitive\pdffilesize primitive\else bad\fi|\ifpdfprimitive\pdfcreationdate creation\else bad\fi|\ifpdfprimitive\pdffiledump dump\else bad\fi",
            &root,
            "main",
        )
        .unwrap();

        let parts = expanded.split('|').collect::<Vec<_>>();
        assert_eq!(parts[0], "main");
        assert_eq!(parts[1], "3");
        assert_eq!(parts[2], "4");
        assert!(
            parts[3].starts_with("D:") && parts[3].len() >= "D:YYYYMMDDhhmmssZ".len(),
            "{expanded}"
        );
        assert_eq!(parts[4], "900150983CD24FB0D6963F7D28E17F72");
        assert_eq!(parts[5], "900150983CD24FB0D6963F7D28E17F72");
        assert_eq!(parts[6], "6263");
        assert_eq!(parts[7], "6263");
        assert_eq!(parts[8], "");
        assert_eq!(parts[9], "missing");
        assert_eq!(parts[10], "primitive");
        assert_eq!(parts[11], "creation");
        assert_eq!(parts[12], "dump");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn edef_expands_primitives_and_preserves_noexpand_tokens() {
        assert_eq!(
            expand_to_text(
                r"\newcount\n\n=7\edef\a{\number\n|\the\n|\romannumeral\numexpr\n+2\relax}\a"
            )
            .unwrap(),
            "7|7|ix"
        );
        assert_eq!(
            expand_to_text(r"\def\a{A}\edef\b{\string\a|\detokenize{\a}}\def\a{B}\b").unwrap(),
            r"\a|\a"
        );
        assert_eq!(
            expand_to_text(r"\def\a{A}\edef\b{\noexpand\a|\unexpanded{\a}}\def\a{B}\b").unwrap(),
            "B|B"
        );
        assert_eq!(
            expand_to_text(r"\def\a{A}\edef\b{\expanded{\a}}\def\a{B}\b").unwrap(),
            "A"
        );
        assert_eq!(
            expand_to_text(r"\protected\def\a{A}\edef\b{\expanded{\a}}\def\a{B}\b").unwrap(),
            "B"
        );
    }

    #[test]
    fn numbered_count_registers_and_countdef_aliases_expand() {
        assert_eq!(
            expand_to_text(
                r"\count0=3\number\count0|\countdef\score=0\advance\score by 4\the\score"
            )
            .unwrap(),
            "3|7"
        );
        assert_eq!(
            expand_to_text(r"\countdef\score=12\score=5\let\alias=\score\advance\alias by 2\number\score|\number\count12")
                .unwrap(),
            "7|7"
        );
    }

    #[test]
    fn skip_registers_expand_through_the_and_glue_components() {
        assert_eq!(
            expand_to_text(r"\newskip\s\s=1pt plus 2pt minus .5pt\advance\s by 3pt plus 1pt\the\s")
                .unwrap(),
            "4.0pt plus 3.0pt minus 0.5pt"
        );
        assert_eq!(
            expand_to_text(r"\skip0=2pt\skipdef\pad=0\advance\pad by 1pt\the\pad|\the\skip0")
                .unwrap(),
            "3.0pt|3.0pt"
        );
    }

    #[test]
    fn skip_register_assignments_are_scoped_and_globalizable() {
        assert_eq!(
            expand_to_text(r"\newskip\s\s=1pt\begingroup\s=2pt\the\s\endgroup\the\s").unwrap(),
            "2.0pt1.0pt"
        );
        assert_eq!(
            expand_to_text(r"\newskip\s\s=1pt\begingroup\global\s=4pt\endgroup\the\s").unwrap(),
            "4.0pt"
        );
        assert_eq!(
            expand_to_text(r"\begingroup\global\newskip\s\global\s=6pt\endgroup\the\s").unwrap(),
            "6.0pt"
        );
    }

    #[test]
    fn skip_register_aliases_participate_in_let_and_ifdefined() {
        assert_eq!(
            expand_to_text(r"\newskip\gap\gap=2pt\let\alias=\gap\advance\alias by 3pt\the\gap")
                .unwrap(),
            "5.0pt"
        );
        assert_eq!(
            expand_to_text(r"\ifdefined\gap missing\else before\fi|\newskip\gap\ifdefined\gap after\else missing\fi")
                .unwrap(),
            "before|after"
        );
        assert_eq!(
            expand_to_text(r"\makeatletter\the\z@skip|\skip@=1pt\the\skip@").unwrap(),
            "0.0pt|1.0pt"
        );
    }

    #[test]
    fn token_registers_expand_through_the_and_aliases() {
        assert_eq!(
            expand_to_text(r"\def\a{A}\newtoks\t\t={\a B}\the\t").unwrap(),
            "AB"
        );
        assert_eq!(
            expand_to_text(r"\toks0={raw}\toksdef\bag=0\the\bag|\the\toks0").unwrap(),
            "raw|raw"
        );
        assert_eq!(
            expand_to_text(r"\newtoks\t\t={old}\let\alias=\t\alias={new}\the\t").unwrap(),
            "new"
        );
    }

    #[test]
    fn pdftex_token_registers_are_defined_and_assignable_without_equals() {
        assert_eq!(
            expand_to_text(
                r"\ifdefined\pdfpageattr yes\else no\fi|\the\pdfpageattr|\pdfpageattr{/Rotate 90}\the\pdfpageattr|\begingroup\global\pdfpageresources{/CS}\endgroup\the\pdfpageresources"
            )
            .unwrap(),
            "yes||/Rotate 90|/CS"
        );
    }

    #[test]
    fn token_register_assignments_are_scoped_and_globalizable() {
        assert_eq!(
            expand_to_text(r"\newtoks\t\t={outer}\begingroup\t={inner}\the\t\endgroup\the\t")
                .unwrap(),
            "innerouter"
        );
        assert_eq!(
            expand_to_text(r"\newtoks\t\t={outer}\begingroup\global\t={global}\endgroup\the\t")
                .unwrap(),
            "global"
        );
        assert_eq!(
            expand_to_text(r"\begingroup\global\newtoks\t\global\t={G}\endgroup\the\t").unwrap(),
            "G"
        );
    }

    #[test]
    fn token_register_aliases_participate_in_ifdefined_and_scratch_defaults() {
        assert_eq!(
            expand_to_text(
                r"\ifdefined\t missing\else before\fi|\newtoks\t\ifdefined\t after\else missing\fi"
            )
            .unwrap(),
            "before|after"
        );
        assert_eq!(
            expand_to_text(r"\makeatletter\toks@={scratch}\the\toks@").unwrap(),
            "scratch"
        );
    }

    #[test]
    fn dimension_registers_expand_through_the_and_units() {
        assert_eq!(
            expand_to_text(r"\newdimen\d\d=1.5pt\advance\d by .5pt\the\d").unwrap(),
            "2.0pt"
        );
        assert_eq!(
            expand_to_text(r"\dimen0=3pt\dimendef\gap=0\advance\gap by 2pt\the\gap|\the\dimen0")
                .unwrap(),
            "5.0pt|5.0pt"
        );
    }

    #[test]
    fn pdftex_dimension_registers_are_defined_and_accept_true_units() {
        assert_eq!(
            expand_to_text(
                r"\ifdefined\pdfpagewidth yes\else no\fi|\ifdim\pdfpagewidth>200truemm wide\else narrow\fi|\pdfpagewidth=8.5truein\ifdim\pdfpagewidth>8truein wide\else narrow\fi|\pdfpageheight=297 true mm\ifdim\pdfpageheight>800pt tall\else short\fi"
            )
            .unwrap(),
            "yes|wide|wide|tall"
        );
    }

    #[test]
    fn dimension_register_assignments_are_scoped_and_globalizable() {
        assert_eq!(
            expand_to_text(r"\newdimen\d\d=1pt\begingroup\d=2pt\the\d\endgroup\the\d").unwrap(),
            "2.0pt1.0pt"
        );
        assert_eq!(
            expand_to_text(r"\newdimen\d\d=1pt\begingroup\global\d=4pt\endgroup\the\d").unwrap(),
            "4.0pt"
        );
        assert_eq!(
            expand_to_text(r"\begingroup\global\newdimen\d\global\d=6pt\endgroup\the\d").unwrap(),
            "6.0pt"
        );
    }

    #[test]
    fn dimension_register_aliases_participate_in_let_and_ifdefined() {
        assert_eq!(
            expand_to_text(r"\newdimen\gap\gap=2pt\let\alias=\gap\advance\alias by 3pt\the\gap")
                .unwrap(),
            "5.0pt"
        );
        assert_eq!(
            expand_to_text(r"\ifdefined\gap missing\else before\fi|\newdimen\gap\ifdefined\gap after\else missing\fi")
                .unwrap(),
            "before|after"
        );
    }

    #[test]
    fn count_register_assignments_are_scoped_and_globalizable() {
        assert_eq!(
            expand_to_text(r"\newcount\n\n=1\begingroup\n=2\number\n\endgroup\number\n").unwrap(),
            "21"
        );
        assert_eq!(
            expand_to_text(r"\newcount\n\n=1\begingroup\global\n=5\endgroup\number\n").unwrap(),
            "5"
        );
        assert_eq!(
            expand_to_text(r"\begingroup\global\newcount\n\global\n=8\endgroup\number\n").unwrap(),
            "8"
        );
        assert_eq!(
            expand_to_text(
                r"\begingroup\global\countdef\score=14\global\score=9\endgroup\number\count14"
            )
            .unwrap(),
            "9"
        );
    }

    #[test]
    fn ifnum_and_ifodd_select_numeric_branches() {
        assert_eq!(
            expand_to_text(
                r"\newcount\n\n=4\ifnum\n>3yes\else no\fi|\ifnum\n=5 yes\else no\fi|\ifodd\n odd\else even\fi"
            )
            .unwrap(),
            "yes|no|even"
        );
        assert_eq!(
            expand_to_text(r"\def\threshold{7}\ifnum\threshold>5high\else low\fi").unwrap(),
            "high"
        );
    }

    #[test]
    fn ifcase_selects_or_and_else_branches() {
        assert_eq!(
            expand_to_text(
                r"\ifcase0 zero\or one\else many\fi|\ifcase2 zero\or one\or two\else many\fi|\ifcase5 zero\or one\else many\fi"
            )
            .unwrap(),
            "zero|two|many"
        );
        assert_eq!(
            expand_to_text(r"\ifcase1 bad\or \ifnum2>1 nested\else bad\fi\else bad\fi").unwrap(),
            "nested"
        );
    }

    #[test]
    fn ifdim_selects_dimension_branches() {
        assert_eq!(
            expand_to_text(
                r"\newdimen\d\d=2pt\ifdim\d>1.5pt yes\else no\fi|\ifdim\d<1pt yes\else no\fi"
            )
            .unwrap(),
            "yes|no"
        );
        assert_eq!(
            expand_to_text(
                r"\makeatletter\ifdim\p@=1pt one\else bad\fi|\ifdim\z@=0pt zero\else bad\fi"
            )
            .unwrap(),
            "one|zero"
        );
    }

    #[test]
    fn dimexpr_supports_simple_dimension_arithmetic() {
        assert_eq!(
            expand_to_text(r"\the\dimexpr1pt+2pt-.5pt\relax").unwrap(),
            "2.5pt"
        );
        assert_eq!(
            expand_to_text(
                r"\newdimen\d\d=\dimexpr1pt+3pt\relax\ifdim\dimexpr\d-1pt=3pt yes\else no\fi"
            )
            .unwrap(),
            "yes"
        );
    }

    #[test]
    fn chardef_constants_scan_as_integers() {
        assert_eq!(
            expand_to_text(
                r"\chardef\one=1\number\one|\ifnum\one=1 yes\else no\fi|\let\uno=\one\ifx\one\uno same\else different\fi"
            )
            .unwrap(),
            "1|yes|same"
        );
        assert_eq!(
            expand_to_text(r"\begingroup\global\mathchardef\answer=42\endgroup\number\answer")
                .unwrap(),
            "42"
        );
        assert_eq!(
            expand_to_text(r"\makeatletter\number\@ne|\ifcase\tw@ zero\or one\or two\fi").unwrap(),
            "1|two"
        );
    }

    #[test]
    fn expands_positional_macro_arguments() {
        assert_eq!(
            expand_to_text(r"\def\wrap#1{[#1:#1]}\wrap{ok}").unwrap(),
            "[ok:ok]"
        );
    }

    #[test]
    fn edef_expands_replacement_when_defined() {
        assert_eq!(
            expand_to_text(r"\def\a{A}\edef\b{\a}\def\a{B}\b\a").unwrap(),
            "AB"
        );
    }

    #[test]
    fn xdef_expands_replacement_when_defined() {
        assert_eq!(
            expand_to_text(r"\def\a#1{<#1>}\xdef\b{\a{X}}\def\a#1{[#1]}\b\a{Y}").unwrap(),
            "<X>[Y]"
        );
    }

    #[test]
    fn protected_at_edef_expands_with_letter_at_catcode() {
        assert_eq!(
            expand_to_text(r"\makeatletter\def\a{A}\protected@edef\b{\a}\def\a{B}\b\a\makeatother")
                .unwrap(),
            "AB"
        );
    }

    #[test]
    fn protected_definitions_expand_normally_but_survive_edef() {
        assert_eq!(expand_to_text(r"\protected\def\a{A}\a").unwrap(), "A");
        assert_eq!(
            expand_to_text(r"\protected\def\a{A}\edef\b{\a}\def\a{B}\b").unwrap(),
            "B"
        );
        assert_eq!(
            expand_to_text(r"\protected\def\a#1{[#1]}\edef\b{\a{X}}\def\a#1{<#1>}\b").unwrap(),
            "<X>"
        );
        assert_eq!(
            expand_to_text(r"\begingroup\global\protected\def\a{A}\endgroup\edef\b{\a}\def\a{B}\b")
                .unwrap(),
            "B"
        );
        assert_eq!(
            expand_to_text(r"\begingroup\protected\gdef\a{A}\endgroup\edef\b{\a}\def\a{B}\b")
                .unwrap(),
            "B"
        );
    }

    #[test]
    fn expands_nested_macro_output() {
        assert_eq!(
            expand_to_text(r"\def\a{A}\def\b#1{\a#1}\b{B}").unwrap(),
            "AB"
        );
    }

    #[test]
    fn expands_control_symbol_macros() {
        assert_eq!(expand_to_text(r"\def\!{bang}\!").unwrap(), "bang");
    }

    #[test]
    fn expands_newcommand_without_arguments() {
        assert_eq!(
            expand_to_text(r"\newcommand{\hello}{Hi}\hello").unwrap(),
            "Hi"
        );
    }

    #[test]
    fn expands_newcommand_with_required_arguments() {
        assert_eq!(
            expand_to_text(r"\newcommand{\wrap}[2]{<#1:#2>}\wrap{A}{B}").unwrap(),
            "<A:B>"
        );
    }

    #[test]
    fn expands_newcommand_with_optional_default() {
        assert_eq!(
            expand_to_text(r"\newcommand{\pair}[2][x]{#1/#2}\pair{y}|\pair[z]{y}").unwrap(),
            "x/y|z/y"
        );
    }

    #[test]
    fn expands_direct_renewcommand_names() {
        assert_eq!(
            expand_to_text(r"\newcommand\name{Old}\renewcommand*\name{New}\name").unwrap(),
            "New"
        );
    }

    #[test]
    fn providecommand_only_defines_missing_macros() {
        assert_eq!(
            expand_to_text(
                r"\providecommand{\first}{One}\first|\newcommand{\second}{Old}\providecommand{\second}{New}\second"
            )
            .unwrap(),
            "One|Old"
        );
    }

    #[test]
    fn expands_declare_robust_command_like_newcommand() {
        assert_eq!(
            expand_to_text(r"\DeclareRobustCommand{\wrap}[1]{(#1)}\wrap{ok}").unwrap(),
            "(ok)"
        );
        assert_eq!(
            expand_to_text(
                r"\DeclareRobustCommand{\wrap}[1]{(#1)}\edef\a{\wrap{ok}}\renewcommand{\wrap}[1]{[#1]}\a"
            )
            .unwrap(),
            "[ok]"
        );
    }

    #[test]
    fn expands_declare_math_operator_definitions() {
        assert_eq!(
            expand_to_text(r"\DeclareMathOperator*{\argmax}{arg\,max}\argmax_x").unwrap(),
            r"arg\,max_x"
        );
    }

    #[test]
    fn macro_expansion_can_emit_definition_assignments() {
        assert_eq!(
            expand_to_text(
                r"\makeatletter\newcommand{\settitle}[1]{\gdef\@title{#1}}\settitle{Hi}\@title\makeatother"
            )
            .unwrap(),
            "Hi"
        );
    }

    #[test]
    fn let_aliases_existing_macro_definitions() {
        assert_eq!(
            expand_to_text(r"\def\a#1{<#1>}\let\b=\a\b{X}").unwrap(),
            "<X>"
        );
    }

    #[test]
    fn csname_expands_dynamic_control_sequence_names() {
        assert_eq!(
            expand_to_text(r"\def\hello{Hi}\csname hello\endcsname").unwrap(),
            "Hi"
        );
    }

    #[test]
    fn csname_expands_macros_inside_control_sequence_names() {
        assert_eq!(
            expand_to_text(r"\def\stem{hello}\def\hello{Hi}\csname \stem\endcsname").unwrap(),
            "Hi"
        );
    }

    #[test]
    fn expandafter_defines_dynamic_control_sequence_names() {
        assert_eq!(
            expand_to_text(r"\expandafter\def\csname hello\endcsname{Hi}\hello").unwrap(),
            "Hi"
        );
    }

    #[test]
    fn expandafter_lets_dynamic_control_sequence_names() {
        assert_eq!(
            expand_to_text(r"\def\source{Hi}\expandafter\let\csname alias\endcsname=\source\alias")
                .unwrap(),
            "Hi"
        );
    }

    #[test]
    fn edef_expands_dynamic_control_sequence_names() {
        assert_eq!(
            expand_to_text(
                r"\def\hello{Hi}\edef\saved{\csname hello\endcsname}\def\hello{Bye}\saved\hello"
            )
            .unwrap(),
            "HiBye"
        );
    }

    #[test]
    fn catcode_assignment_changes_subsequent_tokenization() {
        assert_eq!(
            expand_to_text(r"\catcode`\@=11\def\make@letter{OK}\make@letter").unwrap(),
            "OK"
        );
        assert_eq!(
            expand_to_text(r"\catcode`\@=11\def\make@letter{OK}\catcode`\@=12").unwrap(),
            ""
        );
    }

    #[test]
    fn local_catcode_assignments_are_restored_after_groups() {
        assert_eq!(
            expand_to_text(r"\begingroup\catcode`\@=11\def\name@x{local}\name@x\endgroup\name@x")
                .unwrap(),
            r"local\name@x"
        );
        assert_eq!(
            expand_to_text(r"\begingroup\global\catcode`\@=11\endgroup\def\name@x{OK}\name@x")
                .unwrap(),
            "OK"
        );
    }

    #[test]
    fn makeatletter_and_makeatother_change_at_catcode() {
        assert_eq!(
            expand_to_text(r"\makeatletter\def\internal@name{OK}\internal@name\makeatother")
                .unwrap(),
            "OK"
        );
        assert_eq!(
            expand_to_tokens(r"\makeatletter\def\internal@name{OK}\makeatother\internal@name")
                .unwrap(),
            vec![
                Token::ControlSequence("internal".to_string()),
                Token::Character {
                    ch: '@',
                    catcode: CatCode::Other,
                },
                Token::Character {
                    ch: 'n',
                    catcode: CatCode::Letter,
                },
                Token::Character {
                    ch: 'a',
                    catcode: CatCode::Letter,
                },
                Token::Character {
                    ch: 'm',
                    catcode: CatCode::Letter,
                },
                Token::Character {
                    ch: 'e',
                    catcode: CatCode::Letter,
                },
            ]
        );
    }

    #[test]
    fn comments_are_ignored_during_streaming_expansion() {
        assert_eq!(expand_to_text("a% hidden\nb").unwrap(), "a b");
    }

    #[test]
    fn source_serialization_preserves_control_word_boundaries() {
        assert_eq!(
            expand_to_source(r"\LaTeX document").unwrap(),
            r"\LaTeX document"
        );
        assert_eq!(
            expand_to_source(r"\section{Title}").unwrap(),
            r"\section{Title}"
        );
        assert_eq!(expand_to_source("a\n\nb").unwrap(), "a\n\nb");
    }
}

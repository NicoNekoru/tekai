use std::collections::{HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow, bail};
use glob::{MatchOptions, glob_with};
use serde::{Deserialize, Serialize};

const BUILD_STATE_VERSION: u32 = 34;
const BIB_STATE_VERSION: u32 = 10;
const INDEX_STATE_VERSION: u32 = 10;
const SPLIT_INDEX_STATE_VERSION: u32 = 1;
const EXTERNAL_TOOL_STATE_VERSION: u32 = 1;
const PREAMBLE_FORMAT_STATE_VERSION: u32 = 1;
const SETTLED_AUX_CACHE_STATE_VERSION: u32 = 3;

#[cfg(windows)]
const KPATHSEA_PATH_SEPARATOR: &str = ";";
#[cfg(not(windows))]
const KPATHSEA_PATH_SEPARATOR: &str = ":";

const BUILD_ENV_VARS: &[&str] = &[
    "TEXINPUTS",
    "BIBINPUTS",
    "BSTINPUTS",
    "INDEXSTYLE",
    "TEXINDEXSTYLE",
    "TEXMFCNF",
    "TEXMF",
    "TEXMFHOME",
    "TEXMFLOCAL",
    "TEXMFCONFIG",
    "TEXMFVAR",
    "TEXMFSYSCONFIG",
    "TEXMFSYSVAR",
];
const BIB_ENV_VARS: &[&str] = &[
    "BIBINPUTS",
    "BSTINPUTS",
    "TEXINPUTS",
    "TEXMFCNF",
    "TEXMF",
    "TEXMFHOME",
    "TEXMFLOCAL",
];
const TEX_ROOT_EFFECTIVE_HASH_PREFIX: &str = "tex-root-effective:";
const TEX_INPUT_EFFECTIVE_HASH_PREFIX: &str = "tex-input-effective:";
const TEX_PREAMBLE_EFFECTIVE_HASH_PREFIX: &str = "tex-preamble-effective:";
const BIB_CITED_EFFECTIVE_HASH_PREFIX: &str = "bib-cited-effective:";
const BIBER_GLOB_FINGERPRINT_PATH_PREFIX: &str = "texpilot-biber-glob:";
const BIBER_GLOB_MATCHES_HASH_PREFIX: &str = "biber-glob-matches:";
const BIBER_CONFIG_FINGERPRINT_PATH_PREFIX: &str = "texpilot-biber-config:";
const BIBER_CONFIG_CHOICE_HASH_PREFIX: &str = "biber-config-choice:";
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Engine {
    PdfLatex,
    XeLatex,
    LuaLatex,
    Tectonic,
    TexpilotPdftex,
    TexpilotPdftexCertified,
}

fn is_texpilot_pdftex_engine(engine: Engine) -> bool {
    matches!(
        engine,
        Engine::TexpilotPdftex | Engine::TexpilotPdftexCertified
    )
}

fn is_certified_texpilot_pdftex_engine(engine: Engine) -> bool {
    matches!(engine, Engine::TexpilotPdftexCertified)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Runner {
    Direct,
    Latexmk,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BibMode {
    Auto,
    BibTex,
    Biber,
    None,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DraftPrepass {
    Auto,
    Always,
    Never,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DraftGraphicsStrategy {
    None,
    UntilSettled,
}

#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub main: PathBuf,
    pub job_name: Option<String>,
    pub engine: Engine,
    pub runner: Runner,
    pub bib_mode: BibMode,
    pub out_dir: PathBuf,
    pub fast: bool,
    pub draft_prepass: DraftPrepass,
    pub once: bool,
    pub max_runs: usize,
    pub force: bool,
    pub precompile_preamble: bool,
    pub synctex: bool,
    pub shell_escape: bool,
    pub quiet: bool,
    pub print_command: bool,
}

#[derive(Debug, Clone)]
pub struct BuildReport {
    pub elapsed: Duration,
    pub pdf_path: Option<PathBuf>,
    pub tex_runs: usize,
    pub draft_tex_runs: usize,
    pub final_tex_runs: usize,
    pub pdf_tex_runs: usize,
    pub passes: Vec<TexPassReport>,
    pub bibliography_runs: usize,
    pub index_runs: usize,
    pub external_runs: usize,
    pub skipped: bool,
    pub draft_prepass_used: bool,
    pub aux_preflight_used: bool,
    pub preamble_format_used: bool,
    pub preamble_format_built: bool,
}

#[derive(Debug, Clone)]
pub struct TexPassReport {
    pub draft: bool,
    pub pdf_output: bool,
    pub elapsed: Duration,
    pub tex_elapsed: Duration,
    pub aux_elapsed: Duration,
    pub rerun_reasons: Vec<String>,
    pub aux_outputs_changed: bool,
    pub generated_outputs_changed: bool,
    pub generated_inputs_unread: bool,
    pub preamble_format_used: bool,
    pub preamble_format_built: bool,
    pub bibliography_runs: usize,
    pub index_runs: usize,
    pub external_runs: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BuildState {
    version: u32,
    mode_key: String,
    pdf_path: String,
    inputs: Vec<FileFingerprint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PreambleFormatState {
    version: u32,
    mode_key: String,
    inputs: Vec<FileFingerprint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SettledAuxCacheState {
    version: u32,
    mode_key: String,
    inputs: Vec<FileFingerprint>,
    #[serde(default)]
    out_dir: String,
    #[serde(default)]
    accept_stale_final_pdf: bool,
    files: Vec<String>,
}

#[derive(Debug, Clone)]
struct SettledAuxCacheFile {
    relative: String,
    bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
struct RestoredSettledAuxCache {
    state: BuildState,
    accept_stale_final_pdf: bool,
    restored_pdf: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
struct FileFingerprint {
    path: String,
    len: u64,
    modified_ns: u64,
    #[serde(default)]
    hash: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct GeneratedOutputFingerprint {
    path: String,
    len: u64,
    hash: u64,
}

#[derive(Debug, Clone, Copy)]
struct TexRunMode {
    draft_graphics: bool,
    suppress_pdf_output: bool,
    force_pgf_list_and_make: bool,
}

#[derive(Debug, Default)]
struct AuxToolSessionCache {
    fresh_bibtex_jobs: Mutex<HashSet<BibtexSessionKey>>,
    root_includeonly_filter: Mutex<Option<Option<HashSet<String>>>>,
    source_read_cache: TexSourceReadCache,
    source_external_tools_checked: Mutex<bool>,
    source_external_tool_jobs: Mutex<Option<SourceExternalToolJobs>>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct BibtexSessionKey {
    state_path: PathBuf,
    bbl_path: PathBuf,
    signature: String,
}

#[derive(Debug, Clone, Default)]
struct SourceExternalToolJobs {
    eps: Vec<EpsConversionJob>,
    svg: Vec<SvgConversionJob>,
}

impl AuxToolSessionCache {
    fn bibtex_job_is_fresh(&self, key: &BibtexSessionKey) -> bool {
        self.fresh_bibtex_jobs
            .lock()
            .expect("BibTeX session cache mutex poisoned")
            .contains(key)
    }

    fn mark_bibtex_job_fresh(&self, key: BibtexSessionKey) {
        self.fresh_bibtex_jobs
            .lock()
            .expect("BibTeX session cache mutex poisoned")
            .insert(key);
    }

    fn root_includeonly_filter(&self, main: &Path) -> Result<Option<HashSet<String>>> {
        if let Some(includeonly) = self
            .root_includeonly_filter
            .lock()
            .expect("root includeonly cache mutex poisoned")
            .as_ref()
            .cloned()
        {
            return Ok(includeonly);
        }

        let includeonly = includeonly_filter_for_root(main)?;
        let mut cached = self
            .root_includeonly_filter
            .lock()
            .expect("root includeonly cache mutex poisoned");
        if let Some(cached) = cached.as_ref() {
            return Ok(cached.clone());
        }
        *cached = Some(includeonly.clone());
        Ok(includeonly)
    }

    fn source_read_cache(&self) -> &TexSourceReadCache {
        &self.source_read_cache
    }

    fn source_external_tools_were_checked(&self) -> bool {
        *self
            .source_external_tools_checked
            .lock()
            .expect("source external-tool session cache mutex poisoned")
    }

    fn mark_source_external_tools_checked(&self) {
        *self
            .source_external_tools_checked
            .lock()
            .expect("source external-tool session cache mutex poisoned") = true;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BibState {
    version: u32,
    signature: String,
    bbl_path: String,
    inputs: Vec<FileFingerprint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IndexState {
    version: u32,
    signature: String,
    output_path: String,
    inputs: Vec<FileFingerprint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SplitIndexState {
    version: u32,
    signature: String,
    outputs: Vec<String>,
    inputs: Vec<FileFingerprint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ExternalToolState {
    version: u32,
    signature: String,
    output_path: String,
    #[serde(default)]
    output_paths: Vec<String>,
    inputs: Vec<FileFingerprint>,
}

#[derive(Debug, Clone, Deserialize)]
struct MintedCacheIndex {
    #[serde(default)]
    cachefiles: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PythontexDependencyReport {
    #[serde(default = "default_pythontex_workingdir")]
    workingdir: String,
    #[serde(default)]
    dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
struct BibtexJob {
    program: BibtexProgram,
    command_options: Vec<String>,
    command_arg: String,
    bbl_path: PathBuf,
    request_inputs: Vec<PathBuf>,
    state_path: PathBuf,
}

#[derive(Debug, Clone)]
struct AsymptoteJob {
    input_path: PathBuf,
    output_path: PathBuf,
    input_paths: Vec<PathBuf>,
    state_path: PathBuf,
}

#[derive(Debug, Clone)]
struct PythontexJob {
    code_path: PathBuf,
    command_arg: String,
    macro_path: PathBuf,
    output_paths: Vec<PathBuf>,
    state_path: PathBuf,
}

#[derive(Debug, Clone, Default)]
struct MetapostJob {
    input_path: PathBuf,
    output_paths: Vec<PathBuf>,
    input_paths: Vec<PathBuf>,
    state_path: PathBuf,
}

#[derive(Debug, Clone, Default)]
struct Bib2GlsJob {
    doc_dir: PathBuf,
    aux_path: PathBuf,
    command_arg: String,
    output_paths: Vec<PathBuf>,
    resource_inputs: Vec<PathBuf>,
    state_path: PathBuf,
}

#[derive(Debug, Clone, Default)]
struct Bib2GlsResource {
    options: String,
    output_stem: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct EpsConversionJob {
    input_path: PathBuf,
    output_path: PathBuf,
    state_path: PathBuf,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SvgConversionJob {
    inkscape_executable: String,
    input_path: PathBuf,
    output_path: PathBuf,
    output_tex_path: Option<PathBuf>,
    state_path: PathBuf,
    area: SvgExportArea,
    dpi: Option<String>,
}

#[derive(Debug, Clone)]
struct GnuplottexJob {
    script_path: PathBuf,
    output_path: PathBuf,
    input_paths: Vec<PathBuf>,
    state_path: PathBuf,
}

#[derive(Debug, Clone, Default)]
struct PgfExternalJob {
    makefile_path: PathBuf,
    make_targets: Vec<OsString>,
    output_paths: Vec<PathBuf>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SvgExportArea {
    Drawing,
    Page,
}

#[derive(Copy, Clone)]
struct DirectContext<'a> {
    doc_dir: &'a Path,
    out_dir: &'a Path,
    job_name: &'a str,
    main: &'a Path,
    options: &'a BuildOptions,
    aux_session_cache: &'a AuxToolSessionCache,
}

#[derive(Debug, Clone, Default)]
struct BibtexCommandSpec {
    program: BibtexProgram,
    options: Vec<String>,
    request_inputs: Vec<PathBuf>,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum BibtexProgram {
    #[default]
    Bibtex,
    Bibtex8,
    Bibtexu,
    Pbibtex,
    Upbibtex,
}

impl BibtexProgram {
    fn executable(self) -> &'static str {
        match self {
            Self::Bibtex => "bibtex",
            Self::Bibtex8 => "bibtex8",
            Self::Bibtexu => "bibtexu",
            Self::Pbibtex => "pbibtex",
            Self::Upbibtex => "upbibtex",
        }
    }
}

#[derive(Debug, Clone)]
struct MakeIndexJob {
    tool: IndexTool,
    program: Option<IndexCommandProgram>,
    input_path: PathBuf,
    output_path: PathBuf,
    transcript_path: PathBuf,
    style_path: Option<PathBuf>,
    command_options: Vec<String>,
    style_is_build_output: bool,
    state_path: PathBuf,
}

#[derive(Debug, Clone)]
struct SplitIndexJob {
    input_path: PathBuf,
    output_paths: Vec<PathBuf>,
    state_path: PathBuf,
}

#[derive(Debug, Clone, Default)]
struct MakeIndexCommandSpec {
    program: IndexCommandProgram,
    style_path: Option<PathBuf>,
    options: Vec<String>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
struct ParsedMakeIndexCommand {
    program: IndexCommandProgram,
    style: Option<String>,
    options: Vec<String>,
}

struct MakeIndexStyleRequest<'a> {
    doc_dir: &'a Path,
    out_dir: &'a Path,
    job_name: &'a str,
    input_path: &'a Path,
    kind: MakeIndexKind,
    tool: IndexTool,
    xdy_style_path: Option<&'a Path>,
    command_spec: Option<&'a MakeIndexCommandSpec>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MakeIndexKind {
    Index,
    Glossary,
    Acronym,
    Nomenclature,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum IndexTool {
    MakeIndex,
    Xindy,
    MakeGlossaries,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum IndexCommandProgram {
    #[default]
    MakeIndex,
    Xindy,
    Texindy,
}

impl IndexCommandProgram {
    fn executable(self) -> &'static str {
        match self {
            Self::MakeIndex => "makeindex",
            Self::Xindy => "xindy",
            Self::Texindy => "texindy",
        }
    }
}

pub fn build(options: &BuildOptions) -> Result<BuildReport> {
    match options.runner {
        Runner::Direct if is_certified_texpilot_pdftex_engine(options.engine) => {
            texpilot_pdftex_direct_build(options)
        }
        Runner::Direct if options.engine != Engine::Tectonic => direct_build(options),
        Runner::Direct | Runner::Latexmk => latexmk_or_tectonic_build(options),
    }
}

pub fn build_dependency_paths(options: &BuildOptions) -> Result<Vec<PathBuf>> {
    if options.runner != Runner::Direct || options.engine == Engine::Tectonic {
        return Ok(Vec::new());
    }
    let main = options
        .main
        .canonicalize()
        .with_context(|| format!("cannot find root TeX file {}", options.main.display()))?;
    let job_name = build_job_name(options, &main)?;
    let out_dir = absolute_from_cwd(&options.out_dir)?;
    let state_path = out_dir.join(format!(".texpilot-{job_name}.state.toml"));
    let mut paths = source_seed_dependency_paths(&main)?;
    if !state_path.exists() {
        return Ok(paths);
    }
    let source = fs::read_to_string(&state_path)
        .with_context(|| format!("failed to read build state {}", state_path.display()))?;
    let state: BuildState = toml::from_str(&source)
        .with_context(|| format!("failed to parse build state {}", state_path.display()))?;
    let mode_key = direct_mode_key(options, &main);
    if state.version != BUILD_STATE_VERSION || state.mode_key != mode_key {
        return Ok(paths);
    }
    paths.extend(
        state
            .inputs
            .into_iter()
            .filter(|input| !is_virtual_fingerprint_path(&input.path))
            .map(|input| PathBuf::from(input.path)),
    );
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn latexmk_or_tectonic_build(options: &BuildOptions) -> Result<BuildReport> {
    let main = options
        .main
        .canonicalize()
        .with_context(|| format!("cannot find root TeX file {}", options.main.display()))?;
    let doc_dir = main
        .parent()
        .context("root TeX file has no parent directory")?
        .to_path_buf();
    let file_name = main
        .file_name()
        .context("root TeX file has no filename")?
        .to_os_string();
    let job_name = build_job_name(options, &main)?;
    let out_dir = absolute_from_cwd(&options.out_dir)?;
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("cannot create output directory {}", out_dir.display()))?;
    if options.engine == Engine::Tectonic && options.job_name.is_some() {
        bail!("--job-name is not supported with the tectonic engine");
    }
    if is_texpilot_pdftex_engine(options.engine) {
        bail!("the experimental texpilot-pdftex engine only supports the direct runner");
    }

    let mut command = match options.engine {
        Engine::Tectonic => tectonic_command(&doc_dir, &file_name, &out_dir, options),
        Engine::PdfLatex | Engine::XeLatex | Engine::LuaLatex => {
            latexmk_command(&doc_dir, &file_name, &job_name, &out_dir, options)
        }
        Engine::TexpilotPdftex | Engine::TexpilotPdftexCertified => {
            unreachable!("texpilot-pdftex was rejected above")
        }
    };
    if options.engine == Engine::Tectonic && options.fast {
        eprintln!(
            "warning: --fast no-image mode is implemented through latexmk/graphicx; tectonic will compile normally"
        );
    }
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);

    let started = Instant::now();
    let status = command.status().context("failed to launch TeX compiler")?;
    let elapsed = started.elapsed();
    if !status.success() {
        bail!("TeX build failed with status {status}");
    }

    let pdf_path = Some(job_output_path(&out_dir, &job_name, "pdf"));
    Ok(BuildReport {
        elapsed,
        pdf_path,
        tex_runs: 0,
        draft_tex_runs: 0,
        final_tex_runs: 0,
        pdf_tex_runs: 0,
        passes: Vec::new(),
        bibliography_runs: 0,
        index_runs: 0,
        external_runs: 0,
        skipped: false,
        draft_prepass_used: false,
        aux_preflight_used: false,
        preamble_format_used: false,
        preamble_format_built: false,
    })
}

fn texpilot_pdftex_direct_build(options: &BuildOptions) -> Result<BuildReport> {
    if options.max_runs == 0 {
        bail!("--max-runs must be at least 1");
    }

    let main = options
        .main
        .canonicalize()
        .with_context(|| format!("cannot find root TeX file {}", options.main.display()))?;
    let doc_dir = main
        .parent()
        .context("root TeX file has no parent directory")?
        .to_path_buf();
    let job_name = build_job_name(options, &main)?;
    let out_dir = absolute_from_cwd(&options.out_dir)?;
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("cannot create output directory {}", out_dir.display()))?;
    let pdf_path = job_output_path(&out_dir, &job_name, "pdf");
    let state_path = out_dir.join(format!(".texpilot-{job_name}.state.toml"));
    let mode_key = direct_mode_key(options, &main);

    let started = Instant::now();
    let mut previous_build_state = read_build_state_if_exists(&state_path)?;
    if !options.force
        && !pdf_path.exists()
        && let Some(restored) =
            restore_settled_aux_cache_if_fresh(&doc_dir, &out_dir, &main, &mode_key, &pdf_path)?
    {
        if restored.restored_pdf {
            previous_build_state = Some(write_build_state_from_fingerprints(
                &state_path,
                &mode_key,
                &pdf_path,
                restored.state.inputs,
            )?);
        } else {
            previous_build_state = Some(restored.state);
        }
    }
    let compatible_previous_build_state = previous_build_state
        .as_ref()
        .filter(|state| build_state_is_compatible(state, &mode_key, &pdf_path));
    let mut build_state_input_freshness = HashMap::new();
    if !options.force
        && pdf_path.exists()
        && let Some(state) = compatible_previous_build_state
        && build_state_inputs_are_fresh(state, &mut build_state_input_freshness)?
    {
        return Ok(BuildReport {
            elapsed: started.elapsed(),
            pdf_path: Some(pdf_path),
            tex_runs: 0,
            draft_tex_runs: 0,
            final_tex_runs: 0,
            pdf_tex_runs: 0,
            passes: Vec::new(),
            bibliography_runs: 0,
            index_runs: 0,
            external_runs: 0,
            skipped: true,
            draft_prepass_used: false,
            aux_preflight_used: false,
            preamble_format_used: false,
            preamble_format_built: false,
        });
    }

    if is_certified_texpilot_pdftex_engine(options.engine) {
        return texpilot_pdftex_certified_direct_build(
            options, started, &job_name, &out_dir, &main,
        );
    }

    let run_mode = TexRunMode {
        draft_graphics: false,
        suppress_pdf_output: false,
        force_pgf_list_and_make: false,
    };
    let pass_started = Instant::now();
    let native_started = Instant::now();
    match run_texpilot_pdftex_native(&job_name, &out_dir, &main, options, run_mode)? {
        TexpilotPdftexRun::Native { input_paths } => {
            let native_elapsed = native_started.elapsed();
            write_native_build_state(
                &state_path,
                &mode_key,
                &pdf_path,
                &doc_dir,
                &out_dir,
                &main,
                &input_paths,
                previous_build_state.as_ref(),
            )?;
            Ok(BuildReport {
                elapsed: started.elapsed(),
                pdf_path: Some(pdf_path),
                tex_runs: 1,
                draft_tex_runs: 0,
                final_tex_runs: 1,
                pdf_tex_runs: 1,
                passes: vec![TexPassReport {
                    draft: false,
                    pdf_output: true,
                    elapsed: pass_started.elapsed(),
                    tex_elapsed: native_elapsed,
                    aux_elapsed: Duration::ZERO,
                    rerun_reasons: Vec::new(),
                    aux_outputs_changed: false,
                    generated_outputs_changed: false,
                    generated_inputs_unread: false,
                    preamble_format_used: false,
                    preamble_format_built: false,
                    bibliography_runs: 0,
                    index_runs: 0,
                    external_runs: 0,
                }],
                bibliography_runs: 0,
                index_runs: 0,
                external_runs: 0,
                skipped: false,
                draft_prepass_used: false,
                aux_preflight_used: false,
                preamble_format_used: false,
                preamble_format_built: false,
            })
        }
        TexpilotPdftexRun::Fallback(reason) => {
            if !options.quiet {
                eprintln!(
                    "warning: texpilot-pdftex native backend unsupported ({reason}); falling back to pdflatex"
                );
            }
            let mut fallback_options = options.clone();
            fallback_options.engine = Engine::PdfLatex;
            direct_build(&fallback_options)
        }
    }
}

fn texpilot_pdftex_certified_direct_build(
    options: &BuildOptions,
    started: Instant,
    job_name: &str,
    out_dir: &Path,
    main: &Path,
) -> Result<BuildReport> {
    let run_mode = TexRunMode {
        draft_graphics: false,
        suppress_pdf_output: false,
        force_pgf_list_and_make: false,
    };
    let native_started = Instant::now();
    let native_result = run_texpilot_pdftex_native(job_name, out_dir, main, options, run_mode)?;
    let native_elapsed = native_started.elapsed();
    if !options.quiet {
        match &native_result {
            TexpilotPdftexRun::Native { .. } => {
                eprintln!(
                    "warning: certified texpilot-pdftex uses pdfTeX as the final artifact oracle"
                );
            }
            TexpilotPdftexRun::Fallback(reason) => {
                eprintln!(
                    "warning: texpilot-pdftex native backend unsupported ({reason}); certified final artifact will be generated by pdfTeX"
                );
            }
        }
    }

    let mut report = direct_build(options)?;
    append_texpilot_pdftex_certification_trace(out_dir, job_name, &native_result, native_elapsed)?;
    report.elapsed = started.elapsed();
    Ok(report)
}

fn append_texpilot_pdftex_certification_trace(
    out_dir: &Path,
    job_name: &str,
    native_result: &TexpilotPdftexRun,
    native_elapsed: Duration,
) -> Result<()> {
    let trace_path = out_dir.join(format!("{job_name}.texpilot-pdftex.trace"));
    let mut trace = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&trace_path)
        .with_context(|| {
            format!(
                "failed to append texpilot-pdftex certification trace {}",
                trace_path.display()
            )
        })?;
    let native_status = match native_result {
        TexpilotPdftexRun::Native { .. } => "native-produced",
        TexpilotPdftexRun::Fallback(_) => "native-unsupported",
    };
    writeln!(trace, "certification_policy\tpdftex-final-oracle")
        .context("failed to write texpilot-pdftex certification policy")?;
    writeln!(trace, "certification_native_status\t{native_status}")
        .context("failed to write texpilot-pdftex certification native status")?;
    writeln!(
        trace,
        "certification_native_ms\t{}",
        native_elapsed.as_millis()
    )
    .context("failed to write texpilot-pdftex certification timing")?;
    writeln!(trace, "certification_final_pdf\tpdflatex")
        .context("failed to write texpilot-pdftex certification final artifact")?;
    Ok(())
}

fn direct_build(options: &BuildOptions) -> Result<BuildReport> {
    if options.max_runs == 0 {
        bail!("--max-runs must be at least 1");
    }

    let main = options
        .main
        .canonicalize()
        .with_context(|| format!("cannot find root TeX file {}", options.main.display()))?;
    let doc_dir = main
        .parent()
        .context("root TeX file has no parent directory")?
        .to_path_buf();
    let file_name = main
        .file_name()
        .context("root TeX file has no filename")?
        .to_os_string();
    let job_name = build_job_name(options, &main)?;
    let out_dir = absolute_from_cwd(&options.out_dir)?;
    fs::create_dir_all(&out_dir)
        .with_context(|| format!("cannot create output directory {}", out_dir.display()))?;
    let pdf_path = job_output_path(&out_dir, &job_name, "pdf");
    let state_path = out_dir.join(format!(".texpilot-{job_name}.state.toml"));
    let mode_key = direct_mode_key(options, &main);

    let started = Instant::now();
    let mut previous_build_state = read_build_state_if_exists(&state_path)?;
    let mut restored_aux_cache_accepts_stale_final_pdf = false;
    if !options.force
        && !pdf_path.exists()
        && let Some(restored) =
            restore_settled_aux_cache_if_fresh(&doc_dir, &out_dir, &main, &mode_key, &pdf_path)?
    {
        restored_aux_cache_accepts_stale_final_pdf = restored.accept_stale_final_pdf;
        if restored.restored_pdf {
            previous_build_state = Some(write_build_state_from_fingerprints(
                &state_path,
                &mode_key,
                &pdf_path,
                restored.state.inputs,
            )?);
        } else {
            previous_build_state = Some(restored.state);
        }
    }
    let compatible_previous_build_state = previous_build_state
        .as_ref()
        .filter(|state| build_state_is_compatible(state, &mode_key, &pdf_path));
    let mut build_state_input_freshness = HashMap::new();
    if !options.force
        && pdf_path.exists()
        && let Some(state) = compatible_previous_build_state
        && build_state_inputs_are_fresh(state, &mut build_state_input_freshness)?
    {
        return Ok(BuildReport {
            elapsed: started.elapsed(),
            pdf_path: Some(pdf_path),
            tex_runs: 0,
            draft_tex_runs: 0,
            final_tex_runs: 0,
            pdf_tex_runs: 0,
            passes: Vec::new(),
            bibliography_runs: 0,
            index_runs: 0,
            external_runs: 0,
            skipped: true,
            draft_prepass_used: false,
            aux_preflight_used: false,
            preamble_format_used: false,
            preamble_format_built: false,
        });
    }

    let mut tex_runs = 0;
    let mut draft_tex_runs = 0;
    let mut final_tex_runs = 0;
    let mut pdf_tex_runs = 0;
    let mut passes = Vec::new();
    let mut bibliography_runs = 0;
    let mut index_runs = 0;
    let mut external_runs = 0;
    let mut preamble_format_used = false;
    let mut preamble_format_built = false;
    let mut preamble_format_inputs = Vec::new();
    let mut pending_preamble_format: Option<PreambleFormatBuildHandle> = None;
    let mut last_generated_outputs: Option<Vec<GeneratedOutputFingerprint>> = None;
    let mut stable_full_layout_no_pdf_rerun_passes = 0_usize;
    let mut stable_standard_file_churn_no_pdf_rerun_passes = 0_usize;
    let mut accept_stale_final_pdf_after_draft = restored_aux_cache_accepts_stale_final_pdf;
    let mut settled_aux_cache_snapshot_for_pdf: Option<Vec<SettledAuxCacheFile>> = None;
    let mut settled_aux_cache_accepts_stale_final_pdf = false;
    let aux_session_cache = AuxToolSessionCache::default();
    let includeonly = aux_session_cache.root_includeonly_filter(&main)?;
    let source_cache = aux_session_cache.source_read_cache();
    let compatible_build_state = compatible_previous_build_state.is_some();
    let source_features_needed =
        options.draft_prepass == DraftPrepass::Auto && !compatible_build_state;
    let source_preflight = source_preflight_scan(
        &doc_dir,
        &out_dir,
        &main,
        includeonly.as_ref(),
        source_cache,
        SourcePreflightOptions {
            collect_features: source_features_needed,
            collect_pgf_externalize: !options.fast && !options.shell_escape,
            prepare_output_subdirs: true,
        },
    )?;
    let source_features = if source_features_needed {
        Some(source_preflight.features)
    } else {
        None
    };
    let draft_graphics_strategy = draft_graphics_strategy(
        options,
        &doc_dir,
        &main,
        compatible_build_state,
        source_features.as_ref(),
    )?;
    let force_pgf_list_and_make = source_preflight.pgf_externalize.uses_externalize
        && !source_preflight.pgf_externalize.has_explicit_mode;
    let mut draft_graphics_phase =
        matches!(draft_graphics_strategy, DraftGraphicsStrategy::UntilSettled)
            && !options.fast
            && !options.once;
    let can_suppress_pdf_output = nonfinal_output_mode_arg(options.engine).is_some();
    let mut full_layout_no_pdf_phase = should_start_full_layout_no_pdf_phase(
        options,
        source_features.as_ref(),
        compatible_build_state,
        can_suppress_pdf_output,
    );
    let mut draft_prepass_used = false;
    let direct = DirectContext {
        doc_dir: &doc_dir,
        out_dir: &out_dir,
        job_name: &job_name,
        main: &main,
        options,
        aux_session_cache: &aux_session_cache,
    };
    let source_bibtex_preflight = run_source_bibtex_preflight_if_possible(
        &doc_dir,
        &out_dir,
        &job_name,
        &main,
        includeonly.as_ref(),
        options,
        source_cache,
        &aux_session_cache,
        compatible_build_state,
    )?;
    let source_bibtex_preflight_used =
        source_bibtex_preflight.bibliography_runs > 0 || source_bibtex_preflight.bibcite_seeded;
    bibliography_runs += source_bibtex_preflight.bibliography_runs;
    let aux_preflight_used = !options.force
        && !options.once
        && can_preflight_aux_tools(
            compatible_previous_build_state,
            direct,
            &mut build_state_input_freshness,
        )?;

    if aux_preflight_used {
        let aux_outputs_before = aux_output_snapshot(
            &doc_dir,
            &out_dir,
            &job_name,
            &main,
            options,
            &aux_session_cache,
        )?;
        let (bibliography_run_count, indexes_run, external_run_count) = run_aux_tools_if_needed(
            &doc_dir,
            &out_dir,
            &job_name,
            &main,
            options,
            &aux_session_cache,
        )?;
        bibliography_runs += bibliography_run_count;
        index_runs += indexes_run;
        external_runs += external_run_count;
        if bibliography_runs + index_runs + external_runs > 0 {
            let aux_outputs_after = aux_output_snapshot(
                &doc_dir,
                &out_dir,
                &job_name,
                &main,
                options,
                &aux_session_cache,
            )?;
            if aux_outputs_before == aux_outputs_after {
                write_build_state(
                    &state_path,
                    &mode_key,
                    &pdf_path,
                    direct,
                    previous_build_state.as_ref(),
                    &[],
                )?;
                return Ok(BuildReport {
                    elapsed: started.elapsed(),
                    pdf_path: Some(pdf_path),
                    tex_runs,
                    draft_tex_runs,
                    final_tex_runs,
                    pdf_tex_runs,
                    passes,
                    bibliography_runs,
                    index_runs,
                    external_runs,
                    skipped: false,
                    draft_prepass_used,
                    aux_preflight_used,
                    preamble_format_used: false,
                    preamble_format_built: false,
                });
            }
        }
    }

    if !aux_preflight_used {
        external_runs += run_source_external_tools_if_needed_once(
            &doc_dir,
            &out_dir,
            &main,
            options,
            &aux_session_cache,
        )?;
    }

    let build_settled = loop {
        let draft_graphics = !options.fast
            && !options.once
            && match draft_graphics_strategy {
                DraftGraphicsStrategy::None => false,
                DraftGraphicsStrategy::UntilSettled => draft_graphics_phase,
            };
        let promote_full_layout_pdf = !draft_graphics
            && !options.fast
            && !options.once
            && full_layout_no_pdf_phase
            && stable_full_layout_no_pdf_rerun_passes
                >= full_layout_pdf_promotion_threshold(
                    draft_prepass_used,
                    stable_standard_file_churn_no_pdf_rerun_passes,
                    source_bibtex_preflight.bibcite_seeded,
                );
        let full_layout_no_pdf = !promote_full_layout_pdf
            && !draft_graphics
            && !options.fast
            && !options.once
            && full_layout_no_pdf_phase;
        if promote_full_layout_pdf {
            full_layout_no_pdf_phase = false;
            stable_full_layout_no_pdf_rerun_passes = 0;
            stable_standard_file_churn_no_pdf_rerun_passes = 0;
        }
        let suppress_pdf_output = draft_graphics || full_layout_no_pdf;
        let run_mode = TexRunMode {
            draft_graphics,
            suppress_pdf_output,
            force_pgf_list_and_make,
        };
        if draft_graphics && pending_preamble_format.is_none() {
            let final_preamble_mode = TexRunMode {
                draft_graphics: false,
                suppress_pdf_output: false,
                force_pgf_list_and_make,
            };
            if let Some(format_kind) = preamble_format_background_kind_for_run(
                options,
                &doc_dir,
                &main,
                final_preamble_mode,
            )? {
                pending_preamble_format = Some(spawn_preamble_format_build(
                    &doc_dir,
                    &file_name,
                    &out_dir,
                    &main,
                    options,
                    format_kind,
                ));
            }
        }
        draft_prepass_used |= draft_graphics;
        let pass_started = Instant::now();
        let track_standard_rerun_outputs =
            full_layout_no_pdf || (!draft_graphics && !suppress_pdf_output);
        let standard_rerun_outputs_before = if track_standard_rerun_outputs {
            Some(standard_rerun_output_snapshot(&out_dir)?)
        } else {
            None
        };
        let sidecar_cache_before_pdf =
            if !draft_graphics && !suppress_pdf_output && accept_stale_final_pdf_after_draft {
                Some(capture_settled_aux_cache_files(&out_dir)?)
            } else {
                None
            };
        let tex_started = Instant::now();
        let preamble_override = if !draft_graphics && !force_pgf_list_and_make {
            match pending_preamble_format.take() {
                Some(handle) => match join_preamble_format_build(handle)? {
                    Some(precompiled) => PreambleFormatOverride::Prepared(precompiled),
                    None => PreambleFormatOverride::Disabled,
                },
                None => PreambleFormatOverride::Auto,
            }
        } else {
            PreambleFormatOverride::Auto
        };
        let tex_invocation = run_tex_direct(
            &doc_dir,
            &file_name,
            &job_name,
            &out_dir,
            &main,
            options,
            run_mode,
            preamble_override,
        )?;
        let tex_elapsed = tex_started.elapsed();
        preamble_format_used |= tex_invocation.preamble_format_used;
        preamble_format_built |= tex_invocation.preamble_format_built;
        if !tex_invocation.preamble_format_inputs.is_empty() {
            preamble_format_inputs.extend(tex_invocation.preamble_format_inputs.clone());
            preamble_format_inputs.sort_by(|left, right| left.path.cmp(&right.path));
            preamble_format_inputs.dedup_by(|left, right| left.path == right.path);
        }
        tex_runs += 1;
        if draft_graphics {
            draft_tex_runs += 1;
        } else {
            final_tex_runs += 1;
            if !suppress_pdf_output {
                pdf_tex_runs += 1;
            }
        }
        let generated_outputs_changed = {
            let current = generated_output_snapshot(&out_dir, &doc_dir, &job_name)?;
            let changed = last_generated_outputs
                .as_ref()
                .is_some_and(|previous| previous != &current);
            last_generated_outputs = Some(current);
            changed
        };

        if options.once {
            passes.push(TexPassReport {
                draft: draft_graphics,
                pdf_output: !suppress_pdf_output,
                elapsed: pass_started.elapsed(),
                tex_elapsed,
                aux_elapsed: Duration::ZERO,
                rerun_reasons: Vec::new(),
                aux_outputs_changed: false,
                generated_outputs_changed,
                generated_inputs_unread: false,
                preamble_format_used: tex_invocation.preamble_format_used,
                preamble_format_built: tex_invocation.preamble_format_built,
                bibliography_runs: 0,
                index_runs: 0,
                external_runs: 0,
            });
            break true;
        }

        let aux_outputs_before = aux_output_snapshot(
            &doc_dir,
            &out_dir,
            &job_name,
            &main,
            options,
            &aux_session_cache,
        )?;
        let aux_started = Instant::now();
        let (bibliography_run_count, indexes_run, external_run_count) = run_aux_tools_if_needed(
            &doc_dir,
            &out_dir,
            &job_name,
            &main,
            options,
            &aux_session_cache,
        )?;
        let aux_elapsed = aux_started.elapsed();
        bibliography_runs += bibliography_run_count;
        index_runs += indexes_run;
        external_runs += external_run_count;
        let aux_outputs_changed =
            if bibliography_run_count > 0 || indexes_run > 0 || external_run_count > 0 {
                let aux_outputs_after = aux_output_snapshot(
                    &doc_dir,
                    &out_dir,
                    &job_name,
                    &main,
                    options,
                    &aux_session_cache,
                )?;
                aux_outputs_before != aux_outputs_after
            } else {
                false
            };

        let rerun_reasons = tex_rerun_reasons(&out_dir.join(format!("{job_name}.log")))?;
        let needs_rerun = !rerun_reasons.is_empty();
        let standard_rerun_outputs_changed = standard_rerun_outputs_before
            .map(|before| standard_rerun_output_snapshot(&out_dir).map(|after| before != after))
            .transpose()?
            .unwrap_or(false);
        let generated_inputs_unread =
            generated_inputs_unread_from_latest_run(&doc_dir, &out_dir, &job_name)?;
        passes.push(TexPassReport {
            draft: draft_graphics,
            pdf_output: !suppress_pdf_output,
            elapsed: pass_started.elapsed(),
            tex_elapsed,
            aux_elapsed,
            rerun_reasons: rerun_reasons.clone(),
            aux_outputs_changed,
            generated_outputs_changed,
            generated_inputs_unread,
            preamble_format_used: tex_invocation.preamble_format_used,
            preamble_format_built: tex_invocation.preamble_format_built,
            bibliography_runs: bibliography_run_count,
            index_runs: indexes_run,
            external_runs: external_run_count,
        });

        if aux_outputs_changed {
            stable_full_layout_no_pdf_rerun_passes = 0;
            stable_standard_file_churn_no_pdf_rerun_passes = 0;
            if tex_runs >= options.max_runs {
                break false;
            }
            continue;
        }

        if draft_graphics {
            if matches!(draft_graphics_strategy, DraftGraphicsStrategy::UntilSettled)
                && !generated_outputs_changed
                && !generated_inputs_unread
            {
                // Draft logs can keep asking for reruns for backref/outfile churn that the
                // final full-image pass will resolve. Switch once generated inputs settle.
                draft_graphics_phase = false;
                accept_stale_final_pdf_after_draft =
                    should_accept_stale_final_pdf_after_stable_draft(
                        source_features.as_ref(),
                        &rerun_reasons,
                        source_bibtex_preflight_used,
                    );
                full_layout_no_pdf_phase = false;
                stable_full_layout_no_pdf_rerun_passes = 0;
                stable_standard_file_churn_no_pdf_rerun_passes = 0;
                last_generated_outputs = None;
            }
            if tex_runs >= options.max_runs {
                break false;
            }
            continue;
        }

        if full_layout_no_pdf {
            if !generated_outputs_changed && !generated_inputs_unread && !needs_rerun {
                full_layout_no_pdf_phase = false;
                stable_full_layout_no_pdf_rerun_passes = 0;
                stable_standard_file_churn_no_pdf_rerun_passes = 0;
                last_generated_outputs = None;
            } else if !generated_outputs_changed && !generated_inputs_unread && needs_rerun {
                stable_full_layout_no_pdf_rerun_passes += 1;
                if !standard_rerun_outputs_changed
                    && rerun_reasons.iter().any(|reason| reason == "file-changed")
                {
                    stable_standard_file_churn_no_pdf_rerun_passes += 1;
                } else {
                    stable_standard_file_churn_no_pdf_rerun_passes = 0;
                }
            } else {
                stable_full_layout_no_pdf_rerun_passes = 0;
                stable_standard_file_churn_no_pdf_rerun_passes = 0;
            }
            if tex_runs >= options.max_runs {
                break false;
            }
            continue;
        }

        let final_outputs_stable = !generated_outputs_changed && !generated_inputs_unread;
        let final_pdf_settled = final_outputs_stable && !needs_rerun;
        let final_pdf_accepted_stale = final_outputs_stable
            && needs_rerun
            && can_accept_final_pdf_with_stale_rerun_warnings(
                draft_graphics,
                suppress_pdf_output,
                generated_outputs_changed,
                generated_inputs_unread,
                standard_rerun_outputs_changed,
                &rerun_reasons,
                accept_stale_final_pdf_after_draft,
            );
        if final_pdf_settled || final_pdf_accepted_stale {
            if final_pdf_accepted_stale {
                settled_aux_cache_snapshot_for_pdf = sidecar_cache_before_pdf;
                settled_aux_cache_accepts_stale_final_pdf =
                    settled_aux_cache_snapshot_for_pdf.is_some();
            } else {
                settled_aux_cache_snapshot_for_pdf = None;
                settled_aux_cache_accepts_stale_final_pdf = false;
            }
            break true;
        }
        accept_stale_final_pdf_after_draft = false;

        if tex_runs >= options.max_runs {
            break false;
        }
    };

    if let Some(handle) = pending_preamble_format.take() {
        let _ = join_preamble_format_build(handle);
    }

    if !build_settled {
        bail!(
            "direct runner did not settle after {} TeX run{}; increase --max-runs or use --once for an intentionally incomplete preview",
            options.max_runs,
            if options.max_runs == 1 { "" } else { "s" }
        );
    }

    let elapsed = started.elapsed();
    if !options.once {
        refresh_bibtex_state_if_available(&doc_dir, &out_dir, &job_name)?;
        refresh_biber_state_if_available(&doc_dir, &out_dir, &job_name)?;
        refresh_index_states_if_available(&doc_dir, &out_dir, &job_name)?;
    }
    let final_build_state = write_build_state(
        &state_path,
        &mode_key,
        &pdf_path,
        direct,
        previous_build_state.as_ref(),
        &preamble_format_inputs,
    )?;
    if !options.once
        && (!settled_aux_cache_accepts_stale_final_pdf
            || settled_aux_cache_snapshot_for_pdf.is_some())
    {
        save_settled_aux_cache(
            &doc_dir,
            &out_dir,
            &main,
            &mode_key,
            &final_build_state,
            (!options.synctex).then_some(pdf_path.as_path()),
            settled_aux_cache_snapshot_for_pdf.as_deref(),
            settled_aux_cache_accepts_stale_final_pdf,
        )?;
    }
    Ok(BuildReport {
        elapsed,
        pdf_path: Some(pdf_path),
        tex_runs,
        draft_tex_runs,
        final_tex_runs,
        pdf_tex_runs,
        passes,
        bibliography_runs,
        index_runs,
        external_runs,
        skipped: false,
        draft_prepass_used,
        aux_preflight_used,
        preamble_format_used,
        preamble_format_built,
    })
}

fn run_tex_direct(
    doc_dir: &Path,
    file_name: &OsString,
    job_name: &str,
    out_dir: &Path,
    main: &Path,
    options: &BuildOptions,
    mode: TexRunMode,
    preamble_override: PreambleFormatOverride,
) -> Result<TexInvocationReport> {
    if !tex_run_records_files(options, mode) {
        let _ = fs::remove_file(out_dir.join(format!("{job_name}.fls")));
    }

    let precompiled = match preamble_override {
        PreambleFormatOverride::Auto => {
            prepare_preamble_format(doc_dir, file_name, out_dir, main, options, mode)?
        }
        PreambleFormatOverride::Prepared(precompiled) => Some(precompiled),
        PreambleFormatOverride::Disabled => None,
    };
    if let Some(precompiled) = precompiled {
        let mut command = tex_direct_base_command(doc_dir, job_name, out_dir, options, mode);
        command.env("TEXFORMATS", texformats_env(&precompiled.format_dir));
        command.arg(format!("-fmt={}", precompiled.format_name));
        command.arg(file_name);

        if options.print_command {
            eprintln!("{}", display_command(&command));
        }
        configure_output(&mut command, options);
        match command.status() {
            Ok(status) if status.success() => {
                return Ok(TexInvocationReport {
                    preamble_format_used: true,
                    preamble_format_built: precompiled.built,
                    preamble_format_inputs: precompiled.inputs,
                });
            }
            Ok(status) => {
                if !options.quiet {
                    eprintln!(
                        "warning: precompiled preamble format failed with status {status}; falling back to normal TeX"
                    );
                }
                let _ = fs::remove_file(&precompiled.format_path);
                let _ = fs::remove_file(&precompiled.state_path);
            }
            Err(error) => {
                if !options.quiet {
                    eprintln!(
                        "warning: failed to launch precompiled preamble format: {error}; falling back to normal TeX"
                    );
                }
            }
        }
    }

    let mut command = tex_direct_base_command(doc_dir, job_name, out_dir, options, mode);
    if options.engine == Engine::TexpilotPdftex {
        command.arg("-fmt=pdflatex");
    }
    add_tex_direct_input(&mut command, doc_dir, file_name, options, mode);

    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch TeX engine")?;
    if !status.success() {
        bail!("TeX engine failed with status {status}");
    }
    Ok(TexInvocationReport::default())
}

enum TexpilotPdftexRun {
    Native { input_paths: Vec<PathBuf> },
    Fallback(String),
}

fn run_texpilot_pdftex_native(
    job_name: &str,
    out_dir: &Path,
    main: &Path,
    options: &BuildOptions,
    mode: TexRunMode,
) -> Result<TexpilotPdftexRun> {
    let run = texpilot_pdftex::run_native_pdf_only(&texpilot_pdftex::NativeEngineOptions {
        main: main.to_path_buf(),
        output_dir: out_dir.to_path_buf(),
        job_name: job_name.to_string(),
        mode: texpilot_pdftex::RunMode {
            suppress_pdf_output: mode.suppress_pdf_output,
            draft_graphics: mode.draft_graphics,
        },
        shell_escape: options.shell_escape,
        synctex: options.synctex,
    })
    .context("failed to run texpilot-pdftex native backend")?;

    match run.status {
        texpilot_pdftex::NativeEngineStatus::Native => Ok(TexpilotPdftexRun::Native {
            input_paths: run.input_paths,
        }),
        texpilot_pdftex::NativeEngineStatus::Unsupported(unsupported) => {
            Ok(TexpilotPdftexRun::Fallback(unsupported.reason))
        }
    }
}

#[derive(Debug, Clone, Default)]
struct TexInvocationReport {
    preamble_format_used: bool,
    preamble_format_built: bool,
    preamble_format_inputs: Vec<FileFingerprint>,
}

#[derive(Debug, Clone)]
struct PreambleFormatPreparation {
    format_name: String,
    format_dir: PathBuf,
    format_path: PathBuf,
    state_path: PathBuf,
    built: bool,
    inputs: Vec<FileFingerprint>,
}

enum PreambleFormatOverride {
    Auto,
    Prepared(PreambleFormatPreparation),
    Disabled,
}

type PreambleFormatBuildHandle = thread::JoinHandle<Result<Option<PreambleFormatPreparation>>>;

fn spawn_preamble_format_build(
    doc_dir: &Path,
    file_name: &OsString,
    out_dir: &Path,
    main: &Path,
    options: &BuildOptions,
    format_kind: PreambleFormatKind,
) -> PreambleFormatBuildHandle {
    let doc_dir = doc_dir.to_path_buf();
    let file_name = file_name.clone();
    let out_dir = out_dir.to_path_buf();
    let main = main.to_path_buf();
    let options = options.clone();
    thread::spawn(move || {
        prepare_preamble_format_for_kind(
            &doc_dir,
            &file_name,
            &out_dir,
            &main,
            &options,
            format_kind,
        )
    })
}

fn join_preamble_format_build(
    handle: PreambleFormatBuildHandle,
) -> Result<Option<PreambleFormatPreparation>> {
    handle
        .join()
        .map_err(|_| anyhow!("preamble format builder thread panicked"))?
}

fn tex_direct_base_command(
    doc_dir: &Path,
    job_name: &str,
    out_dir: &Path,
    options: &BuildOptions,
    mode: TexRunMode,
) -> Command {
    let mut command = tex_engine_command(options.engine);
    command.current_dir(doc_dir);
    command.env("TEXINPUTS", texinputs_env(doc_dir, out_dir));
    command
        .arg("-interaction=nonstopmode")
        .arg("-halt-on-error")
        .arg("-file-line-error");
    if tex_run_records_files(options, mode) {
        command.arg("-recorder");
    }
    command
        .arg(format!("-jobname={job_name}"))
        .arg(format!("-output-directory={}", out_dir.display()));

    if options.synctex {
        command.arg("-synctex=1");
    }
    if options.shell_escape {
        command.arg("-shell-escape");
    }
    if mode.suppress_pdf_output
        && let Some(output_mode_arg) = nonfinal_output_mode_arg(options.engine)
    {
        command.arg(output_mode_arg);
    }
    command
}

fn tex_run_records_files(options: &BuildOptions, mode: TexRunMode) -> bool {
    !(mode.draft_graphics
        && mode.suppress_pdf_output
        && !mode.force_pgf_list_and_make
        && !options.shell_escape)
}

fn add_tex_direct_input(
    command: &mut Command,
    doc_dir: &Path,
    file_name: &OsString,
    options: &BuildOptions,
    mode: TexRunMode,
) {
    if options.fast {
        command.arg(format!(
            r"{}\input{{{}}}",
            fast_preview_pretex(doc_dir),
            file_name.to_string_lossy()
        ));
    } else if mode.draft_graphics && mode.force_pgf_list_and_make {
        command.arg(format!(
            r"\PassOptionsToPackage{{draft}}{{graphicx}}\AtBeginDocument{{\tikzset{{external/mode=list and make}}}}\input{{{}}}",
            file_name.to_string_lossy()
        ));
    } else if mode.draft_graphics {
        command.arg(format!(
            r"\PassOptionsToPackage{{draft}}{{graphicx}}\input{{{}}}",
            file_name.to_string_lossy()
        ));
    } else if mode.force_pgf_list_and_make {
        command.arg(format!(
            r"\AtBeginDocument{{\tikzset{{external/mode=list and make}}}}\input{{{}}}",
            file_name.to_string_lossy()
        ));
    } else {
        command.arg(file_name);
    }
}

fn prepare_preamble_format(
    doc_dir: &Path,
    file_name: &OsString,
    out_dir: &Path,
    main: &Path,
    options: &BuildOptions,
    mode: TexRunMode,
) -> Result<Option<PreambleFormatPreparation>> {
    if let Some(format_kind) = preamble_format_kind_for_run(options, mode) {
        return prepare_preamble_format_for_kind(
            doc_dir,
            file_name,
            out_dir,
            main,
            options,
            format_kind,
        );
    }
    let Some(format_kind) = opportunistic_preamble_format_kind_for_run(options, mode) else {
        return Ok(None);
    };
    if !automatic_preamble_format_is_safe_for_root(main)? {
        return Ok(None);
    }
    prepare_preamble_format_for_kind_with_policy(
        doc_dir,
        file_name,
        out_dir,
        main,
        options,
        format_kind,
        false,
    )
}

fn prepare_preamble_format_for_kind(
    doc_dir: &Path,
    file_name: &OsString,
    out_dir: &Path,
    main: &Path,
    options: &BuildOptions,
    format_kind: PreambleFormatKind,
) -> Result<Option<PreambleFormatPreparation>> {
    prepare_preamble_format_for_kind_with_policy(
        doc_dir,
        file_name,
        out_dir,
        main,
        options,
        format_kind,
        true,
    )
}

fn prepare_preamble_format_for_kind_with_policy(
    doc_dir: &Path,
    file_name: &OsString,
    out_dir: &Path,
    main: &Path,
    options: &BuildOptions,
    format_kind: PreambleFormatKind,
    build_if_missing: bool,
) -> Result<Option<PreambleFormatPreparation>> {
    let mode_key = preamble_format_mode_key(doc_dir, main, options, format_kind)?;
    let format_name = format!(
        "texpilot-fastfmt-{:016x}",
        content_hash(mode_key.as_bytes())
    );
    let format_dir = preamble_format_cache_dir(doc_dir, main);
    fs::create_dir_all(&format_dir).with_context(|| {
        format!(
            "failed to create preamble format cache directory {}",
            format_dir.display()
        )
    })?;
    let format_path = format_dir.join(format!("{format_name}.fmt"));
    let state_path = format_dir.join(format!("{format_name}.state.toml"));
    let fls_path = format_dir.join(format!("{format_name}.fls"));

    if format_path.exists()
        && let Some(state) = read_preamble_format_state_if_exists(&state_path)?
        && state.version == PREAMBLE_FORMAT_STATE_VERSION
        && state.mode_key == mode_key
        && preamble_format_inputs_are_fresh(&state.inputs)?
    {
        return Ok(Some(PreambleFormatPreparation {
            format_name,
            format_dir,
            format_path,
            state_path,
            built: false,
            inputs: state.inputs,
        }));
    }

    if !build_if_missing {
        return Ok(None);
    }

    let previous_inputs = read_preamble_format_state_if_exists(&state_path)?
        .filter(|state| state.version == PREAMBLE_FORMAT_STATE_VERSION)
        .map(|state| fingerprint_map(state.inputs))
        .unwrap_or_default();

    let _ = fs::remove_file(&format_path);
    let _ = fs::remove_file(&fls_path);
    ensure_preamble_format_sidecar_inputs(&format_dir, &format_name)?;
    let mut command = tex_engine_command(options.engine);
    command.current_dir(doc_dir);
    command.env(
        "TEXINPUTS",
        texinputs_env_with_format_dir(doc_dir, out_dir, &format_dir),
    );
    command
        .arg("-ini")
        .arg("-recorder")
        .arg("-interaction=nonstopmode")
        .arg("-halt-on-error")
        .arg(format!("-jobname={format_name}"))
        .arg(format!("-output-directory={}", format_dir.display()))
        .arg("&pdflatex")
        .arg("mylatexformat.ltx")
        .arg(preamble_format_source(doc_dir, file_name, format_kind));
    if options.shell_escape {
        command.arg("-shell-escape");
    }
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = match command.status() {
        Ok(status) => status,
        Err(error) => {
            if !options.quiet {
                eprintln!(
                    "warning: failed to launch precompiled preamble format builder: {error}; falling back to normal TeX"
                );
            }
            return Ok(None);
        }
    };
    if !status.success() || !format_path.exists() || !fls_path.exists() {
        if !options.quiet {
            eprintln!(
                "warning: precompiled preamble format build failed; falling back to normal TeX"
            );
        }
        let _ = fs::remove_file(&format_path);
        let _ = fs::remove_file(&state_path);
        return Ok(None);
    }

    let inputs =
        recorded_preamble_format_inputs(&fls_path, doc_dir, out_dir, main, &previous_inputs)?;
    let state = PreambleFormatState {
        version: PREAMBLE_FORMAT_STATE_VERSION,
        mode_key,
        inputs: inputs.clone(),
    };
    let source = toml::to_string(&state).context("failed to serialize preamble format state")?;
    fs::write(&state_path, source).with_context(|| {
        format!(
            "failed to write preamble format state {}",
            state_path.display()
        )
    })?;

    Ok(Some(PreambleFormatPreparation {
        format_name,
        format_dir,
        format_path,
        state_path,
        built: true,
        inputs,
    }))
}

fn preamble_format_cache_dir(doc_dir: &Path, main: &Path) -> PathBuf {
    let root = cache_root_from_env("TEXPILOT_FORMAT_CACHE", default_preamble_format_cache_root);
    root.join(document_cache_key(doc_dir, main))
}

fn settled_aux_cache_dir(doc_dir: &Path, main: &Path, mode_key: &str) -> PathBuf {
    let root = cache_root_from_env("TEXPILOT_AUX_CACHE", default_settled_aux_cache_root);
    root.join(document_cache_key(doc_dir, main))
        .join(format!("{:016x}", content_hash(mode_key.as_bytes())))
}

fn cache_root_from_env(var: &str, default: fn() -> PathBuf) -> PathBuf {
    let Some(value) = std::env::var_os(var).filter(|value| !value.is_empty()) else {
        return default();
    };
    let path = PathBuf::from(value);
    if path.is_absolute() {
        path
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(path)
    }
}

fn document_cache_key(doc_dir: &Path, main: &Path) -> String {
    format!(
        "{:016x}",
        content_hash(
            format!(
                "{}\n{}",
                canonical_or_original(doc_dir).display(),
                canonical_or_original(main).display()
            )
            .as_bytes()
        )
    )
}

fn default_preamble_format_cache_root() -> PathBuf {
    default_texpilot_cache_root("formats")
}

fn default_settled_aux_cache_root() -> PathBuf {
    default_texpilot_cache_root("settled-aux")
}

fn default_bibtex_cache_root() -> PathBuf {
    default_texpilot_cache_root("bibtex")
}

fn default_texpilot_cache_root(kind: &str) -> PathBuf {
    if let Some(value) = std::env::var_os("XDG_CACHE_HOME").filter(|value| !value.is_empty()) {
        return PathBuf::from(value).join("texpilot").join(kind);
    }
    if cfg!(target_os = "macos")
        && let Some(home) = home_dir()
    {
        return home
            .join("Library")
            .join("Caches")
            .join("texpilot")
            .join(kind);
    }
    if cfg!(windows)
        && let Some(value) = std::env::var_os("LOCALAPPDATA")
            .or_else(|| std::env::var_os("APPDATA"))
            .filter(|value| !value.is_empty())
    {
        return PathBuf::from(value).join("texpilot").join(kind);
    }
    if let Some(home) = home_dir() {
        return home.join(".cache").join("texpilot").join(kind);
    }
    std::env::temp_dir().join("texpilot").join(kind)
}

fn restore_settled_aux_cache_if_fresh(
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    mode_key: &str,
    pdf_path: &Path,
) -> Result<Option<RestoredSettledAuxCache>> {
    let cache_dir = settled_aux_cache_dir(doc_dir, main, mode_key);
    let state_path = cache_dir.join("state.toml");
    let Some(source) = read_optional_text_file(&state_path, "settled aux cache state")? else {
        return Ok(None);
    };
    let cache_state: SettledAuxCacheState = toml::from_str(&source).with_context(|| {
        format!(
            "failed to parse settled aux cache state {}",
            state_path.display()
        )
    })?;
    if cache_state.version != SETTLED_AUX_CACHE_STATE_VERSION || cache_state.mode_key != mode_key {
        return Ok(None);
    }
    let restored_state = BuildState {
        version: BUILD_STATE_VERSION,
        mode_key: mode_key.to_string(),
        pdf_path: pdf_path.display().to_string(),
        inputs: cache_state.inputs,
    };
    let mut freshness = HashMap::new();
    if !build_state_inputs_are_fresh(&restored_state, &mut freshness)? {
        return Ok(None);
    }

    let files_dir = cache_dir.join("files");
    let mut files = Vec::with_capacity(cache_state.files.len());
    let cached_out_dir = cache_state.out_dir;
    let restored_out_dir = out_dir.display().to_string();
    let mut restored_pdf = false;
    for file in cache_state.files {
        let relative = PathBuf::from(&file);
        if !safe_relative_path(&relative) {
            return Ok(None);
        }
        let cached = files_dir.join(&relative);
        if !cached.is_file() {
            return Ok(None);
        }
        let destination = out_dir.join(&relative);
        let restores_pdf = destination == pdf_path;
        let rewrite_output_paths = is_aux_tool_state_cache_file(&relative);
        files.push((cached, destination, rewrite_output_paths, restores_pdf));
    }

    for (cached, destination, rewrite_output_paths, restores_pdf) in files {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create restored aux sidecar directory {}",
                    parent.display()
                )
            })?;
        }
        if restores_pdf {
            restored_pdf = true;
        }
        if rewrite_output_paths && !cached_out_dir.is_empty() {
            let source = fs::read_to_string(&cached).with_context(|| {
                format!("failed to read cached aux tool state {}", cached.display())
            })?;
            let source = source.replace(&cached_out_dir, &restored_out_dir);
            fs::write(&destination, source).with_context(|| {
                format!(
                    "failed to restore cached aux tool state {} to {}",
                    cached.display(),
                    destination.display()
                )
            })?;
        } else {
            fs::copy(&cached, &destination).with_context(|| {
                format!(
                    "failed to restore cached aux sidecar {} to {}",
                    cached.display(),
                    destination.display()
                )
            })?;
        }
    }
    Ok(Some(RestoredSettledAuxCache {
        state: restored_state,
        accept_stale_final_pdf: cache_state.accept_stale_final_pdf,
        restored_pdf,
    }))
}

fn save_settled_aux_cache(
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    mode_key: &str,
    build_state: &BuildState,
    pdf_path: Option<&Path>,
    sidecars: Option<&[SettledAuxCacheFile]>,
    accept_stale_final_pdf: bool,
) -> Result<()> {
    let mut cache_files = if let Some(sidecars) = sidecars {
        sidecars.to_vec()
    } else {
        capture_settled_aux_cache_files(out_dir)?
    };
    if let Some(pdf_path) = pdf_path
        && let Some(pdf_artifact) = capture_settled_cache_file(out_dir, pdf_path)?
    {
        cache_files.push(pdf_artifact);
    }
    if cache_files.is_empty() {
        return Ok(());
    }

    let cache_dir = settled_aux_cache_dir(doc_dir, main, mode_key);
    let files_dir = cache_dir.join("files");
    let stale_files_dir = cache_dir.join("files.stale");
    let state_path = cache_dir.join("state.toml");
    let tmp_state_path = cache_dir.join("state.toml.tmp");
    fs::create_dir_all(&cache_dir).with_context(|| {
        format!(
            "failed to create settled aux cache directory {}",
            cache_dir.display()
        )
    })?;
    let _ = fs::remove_dir_all(&stale_files_dir);
    if files_dir.exists() {
        let _ = fs::rename(&files_dir, &stale_files_dir);
    }
    fs::create_dir_all(&files_dir).with_context(|| {
        format!(
            "failed to create settled aux cache files directory {}",
            files_dir.display()
        )
    })?;

    let mut files = Vec::new();
    for sidecar in &cache_files {
        let relative = Path::new(&sidecar.relative);
        if !safe_relative_path(relative) {
            continue;
        }
        let destination = files_dir.join(relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create settled aux cache sidecar directory {}",
                    parent.display()
                )
            })?;
        }
        fs::write(&destination, &sidecar.bytes).with_context(|| {
            format!(
                "failed to cache aux sidecar {} to {}",
                sidecar.relative,
                destination.display()
            )
        })?;
        files.push(sidecar.relative.clone());
    }
    files.sort();
    files.dedup();

    let cache_state = SettledAuxCacheState {
        version: SETTLED_AUX_CACHE_STATE_VERSION,
        mode_key: mode_key.to_string(),
        inputs: build_state.inputs.clone(),
        out_dir: out_dir.display().to_string(),
        accept_stale_final_pdf,
        files,
    };
    let source =
        toml::to_string(&cache_state).context("failed to serialize settled aux cache state")?;
    fs::write(&tmp_state_path, source).with_context(|| {
        format!(
            "failed to write settled aux cache state {}",
            tmp_state_path.display()
        )
    })?;
    fs::rename(&tmp_state_path, &state_path).with_context(|| {
        format!(
            "failed to publish settled aux cache state {}",
            state_path.display()
        )
    })?;
    let _ = fs::remove_dir_all(stale_files_dir);
    Ok(())
}

fn capture_settled_aux_cache_files(out_dir: &Path) -> Result<Vec<SettledAuxCacheFile>> {
    let mut sidecars = Vec::new();
    for path in settled_aux_cache_file_paths(out_dir)? {
        if let Some(sidecar) = capture_settled_cache_file(out_dir, &path)? {
            sidecars.push(sidecar);
        }
    }
    sidecars.sort_by(|left, right| left.relative.cmp(&right.relative));
    sidecars.dedup_by(|left, right| left.relative == right.relative);
    Ok(sidecars)
}

fn capture_settled_cache_file(out_dir: &Path, path: &Path) -> Result<Option<SettledAuxCacheFile>> {
    let Ok(relative) = path.strip_prefix(out_dir) else {
        return Ok(None);
    };
    if !safe_relative_path(relative) {
        return Ok(None);
    }
    let bytes = fs::read(path)
        .with_context(|| format!("failed to read aux cache file {}", path.display()))?;
    Ok(Some(SettledAuxCacheFile {
        relative: cache_relative_path(relative),
        bytes,
    }))
}

fn settled_aux_cache_file_paths(out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    collect_settled_aux_cache_file_paths(out_dir, &mut paths)?;
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn collect_settled_aux_cache_file_paths(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries {
        let entry = entry.with_context(|| format!("failed to read {}", dir.display()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to inspect {}", path.display()))?;
        if file_type.is_dir() {
            collect_settled_aux_cache_file_paths(&path, paths)?;
        } else if file_type.is_file() && is_settled_aux_cache_sidecar(&path) {
            paths.push(path);
        }
    }
    Ok(())
}

fn is_settled_aux_cache_sidecar(path: &Path) -> bool {
    is_aux_tool_state_cache_file(path)
        || path_extension_is_any(
            path,
            &[
                "aux", "out", "toc", "lof", "lot", "lol", "brf", "nav", "snm", "vrb", "thm", "bbl",
                "ind", "gls", "acr", "nls", "maf", "mtc", "mtc0",
            ],
        )
}

fn is_aux_tool_state_cache_file(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    name.starts_with(".texpilot-")
        && [
            ".bibstate.toml",
            ".biberstate.toml",
            ".indexstate.toml",
            ".splitindexstate.toml",
            ".epspdfstate.toml",
            ".svgstate.toml",
            ".asystate.toml",
            ".pythontexstate.toml",
            ".mpoststate.toml",
            ".gnuplotstate.toml",
            ".bib2glsstate.toml",
        ]
        .iter()
        .any(|suffix| name.ends_with(suffix))
}

fn cache_relative_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum PreambleFormatKind {
    Final,
    FastPreview,
}

impl PreambleFormatKind {
    fn key(self) -> &'static str {
        match self {
            PreambleFormatKind::Final => "final",
            PreambleFormatKind::FastPreview => "fast-preview",
        }
    }
}

fn preamble_format_kind_for_run(
    options: &BuildOptions,
    mode: TexRunMode,
) -> Option<PreambleFormatKind> {
    if !options.precompile_preamble
        || !matches!(options.engine, Engine::PdfLatex | Engine::TexpilotPdftex)
    {
        return None;
    }
    if options.fast {
        return options.once.then_some(PreambleFormatKind::FastPreview);
    }
    if mode.draft_graphics || mode.force_pgf_list_and_make {
        return None;
    }
    Some(PreambleFormatKind::Final)
}

fn preamble_format_background_kind_for_run(
    options: &BuildOptions,
    doc_dir: &Path,
    main: &Path,
    mode: TexRunMode,
) -> Result<Option<PreambleFormatKind>> {
    if let Some(kind) = preamble_format_kind_for_run(options, mode) {
        return Ok(Some(kind));
    }
    let Some(kind) = opportunistic_preamble_format_kind_for_run(options, mode) else {
        return Ok(None);
    };
    if !automatic_preamble_format_is_safe_for_root(main)? {
        return Ok(None);
    }
    if resolve_kpathsea_input(doc_dir, "mylatexformat", "ltx")?.is_none() {
        return Ok(None);
    }
    Ok(Some(kind))
}

fn opportunistic_preamble_format_kind_for_run(
    options: &BuildOptions,
    mode: TexRunMode,
) -> Option<PreambleFormatKind> {
    if !matches!(options.engine, Engine::TexpilotPdftex)
        || options.fast
        || options.once
        || options.shell_escape
        || mode.draft_graphics
        || mode.suppress_pdf_output
        || mode.force_pgf_list_and_make
    {
        return None;
    }
    Some(PreambleFormatKind::Final)
}

fn automatic_preamble_format_is_safe_for_root(main: &Path) -> Result<bool> {
    let source = fs::read_to_string(main)
        .with_context(|| format!("failed to read TeX source {}", main.display()))?;
    Ok(!preamble_contains_input_like_dependency(&source))
}

fn preamble_contains_input_like_dependency(source: &str) -> bool {
    let stripped = tex_comment_stripped_source(source);
    let preamble = source_before_begin_document(&stripped);
    !tex_include_source_dependencies_stripped(preamble).is_empty()
        || !tex_input_like_source_dependencies_stripped(preamble).is_empty()
}

fn source_before_begin_document(source: &str) -> &str {
    for line in source.split_inclusive('\n') {
        if let Some(end) = begin_document_end_offset_bytes(line.as_bytes()) {
            let begin = line.as_ptr() as usize - source.as_ptr() as usize;
            return &source[..begin + end];
        }
    }
    source
}

fn preamble_format_source(doc_dir: &Path, file_name: &OsStr, kind: PreambleFormatKind) -> String {
    match kind {
        PreambleFormatKind::Final => format!(r"\input{{{}}}", file_name.to_string_lossy()),
        PreambleFormatKind::FastPreview => {
            format!(
                r"{}\input{{{}}}",
                fast_preview_pretex(doc_dir),
                file_name.to_string_lossy()
            )
        }
    }
}

fn preamble_format_mode_key(
    doc_dir: &Path,
    main: &Path,
    options: &BuildOptions,
    kind: PreambleFormatKind,
) -> Result<String> {
    let file_name = main.file_name().context("root TeX file has no filename")?;
    let mylatexformat = resolve_kpathsea_input(doc_dir, "mylatexformat", "ltx")?
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "missing".to_string());
    Ok(format!(
        "v{};kind={};main={};job={};engine={:?};shell_escape={};source={:016x};mylatexformat={};env={}",
        PREAMBLE_FORMAT_STATE_VERSION,
        kind.key(),
        main.display(),
        options.job_name.as_deref().unwrap_or("<default>"),
        options.engine,
        options.shell_escape,
        content_hash(preamble_format_source(doc_dir, file_name, kind).as_bytes()),
        mylatexformat,
        environment_signature(BUILD_ENV_VARS)
    ))
}

fn read_preamble_format_state_if_exists(path: &Path) -> Result<Option<PreambleFormatState>> {
    let Some(source) = read_optional_text_file(path, "preamble format state")? else {
        return Ok(None);
    };
    let state: PreambleFormatState = toml::from_str(&source)
        .with_context(|| format!("failed to parse preamble format state {}", path.display()))?;
    Ok(Some(state))
}

fn preamble_format_inputs_are_fresh(inputs: &[FileFingerprint]) -> Result<bool> {
    let mut freshness = HashMap::new();
    for input in inputs {
        if !input_fingerprint_is_fresh_cached(input, &mut freshness)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn ensure_preamble_format_sidecar_inputs(format_dir: &Path, format_name: &str) -> Result<()> {
    let mtc0_path = format_dir.join(format!("{format_name}.mtc0"));
    if !mtc0_path.exists() {
        fs::File::create(&mtc0_path).with_context(|| {
            format!(
                "failed to create preamble format sidecar {}",
                mtc0_path.display()
            )
        })?;
    }
    Ok(())
}

fn nonfinal_output_mode_arg(engine: Engine) -> Option<&'static str> {
    match engine {
        Engine::PdfLatex
        | Engine::LuaLatex
        | Engine::TexpilotPdftex
        | Engine::TexpilotPdftexCertified => Some("-draftmode"),
        Engine::XeLatex => Some("-no-pdf"),
        Engine::Tectonic => None,
    }
}

fn fast_preview_pretex(doc_dir: &Path) -> String {
    let minted_option = fast_minted_package_option(doc_dir);
    format!(
        r"\PassOptionsToPackage{{demo}}{{graphicx}}\PassOptionsToPackage{{{minted_option}}}{{minted}}\makeatletter\def\texpilot@fastplaceholder{{\begingroup\fbox{{\rule{{2cm}}{{0pt}}\rule{{0pt}}{{1cm}}}}\endgroup}}\AtBeginDocument{{\@ifpackageloaded{{svg}}{{\renewcommand*\includesvg[2][]{{\texpilot@fastplaceholder}}}}{{}}\@ifpackageloaded{{pdfpages}}{{\renewcommand*\includepdf[2][]{{\texpilot@fastplaceholder}}\renewcommand*\includepdfmerge[2][]{{\texpilot@fastplaceholder}}}}{{}}\@ifpackageloaded{{minted}}{{\renewcommand*\inputminted[3][]{{\texpilot@fastplaceholder}}}}{{}}\@ifpackageloaded{{animate}}{{\renewcommand*\animategraphics[5][]{{\texpilot@fastplaceholder}}}}{{}}\@ifpackageloaded{{standalone}}{{\renewcommand*\includestandalone[2][]{{\texpilot@fastplaceholder}}}}{{}}\@ifpackageloaded{{media9}}{{\renewcommand*\includemedia[3][]{{\texpilot@fastplaceholder}}}}{{}}\@ifpackageloaded{{attachfile2}}{{\renewcommand*\attachfile[2][]{{\texpilot@fastplaceholder}}\renewcommand*\textattachfile[3][]{{\texpilot@fastplaceholder}}\renewcommand*\notextattachfile[2][]{{\texpilot@fastplaceholder}}\renewcommand*\noattachfile[1][]{{\texpilot@fastplaceholder}}}}{{}}\@ifpackageloaded{{attachfile}}{{\renewcommand*\attachfile[2][]{{\texpilot@fastplaceholder}}\renewcommand*\textattachfile[3][]{{\texpilot@fastplaceholder}}\renewcommand*\notextattachfile[2][]{{\texpilot@fastplaceholder}}\renewcommand*\noattachfile[1][]{{\texpilot@fastplaceholder}}}}{{}}\@ifundefined{{tikzexternaldisable}}{{}}{{\tikzexternaldisable}}}}\makeatother"
    )
}

fn fast_minted_package_option(doc_dir: &Path) -> &'static str {
    if let Ok(Some(path)) = resolve_kpathsea_input(doc_dir, "minted", "sty")
        && let Ok(source) = fs::read_to_string(path)
        && source.contains("placeholder/.is if=minted@placeholder")
    {
        return "placeholder";
    }
    "draft"
}

fn draft_graphics_strategy(
    options: &BuildOptions,
    doc_dir: &Path,
    main: &Path,
    compatible_build_state: bool,
    precomputed_features: Option<&SourceFeatures>,
) -> Result<DraftGraphicsStrategy> {
    Ok(match options.draft_prepass {
        DraftPrepass::Never => DraftGraphicsStrategy::None,
        DraftPrepass::Always => DraftGraphicsStrategy::UntilSettled,
        DraftPrepass::Auto => {
            if compatible_build_state {
                return Ok(DraftGraphicsStrategy::None);
            }
            let scanned_features;
            let features = if let Some(features) = precomputed_features {
                features
            } else {
                scanned_features = source_features(doc_dir, main)?;
                &scanned_features
            };
            if features.has_graphics && features.has_multipass_signal {
                DraftGraphicsStrategy::UntilSettled
            } else {
                DraftGraphicsStrategy::None
            }
        }
    })
}

fn should_start_full_layout_no_pdf_phase(
    options: &BuildOptions,
    source_features: Option<&SourceFeatures>,
    compatible_build_state: bool,
    can_suppress_pdf_output: bool,
) -> bool {
    if options.draft_prepass != DraftPrepass::Auto
        || options.fast
        || options.once
        || compatible_build_state
        || !can_suppress_pdf_output
        || options.max_runs < 5
    {
        return false;
    }
    source_features.is_some_and(|features| features.has_multipass_signal && !features.has_graphics)
}

fn should_accept_stale_final_pdf_after_stable_draft(
    source_features: Option<&SourceFeatures>,
    rerun_reasons: &[String],
    source_bibtex_preflight_used: bool,
) -> bool {
    if source_features.is_some_and(|features| features.has_backref_signal) {
        return true;
    }
    !source_bibtex_preflight_used && rerun_reasons.iter().any(|reason| reason == "file-changed")
}

fn full_layout_pdf_promotion_threshold(
    draft_prepass_used: bool,
    stable_standard_file_churn_no_pdf_rerun_passes: usize,
    bibcite_preflight_seeded: bool,
) -> usize {
    if !draft_prepass_used || stable_standard_file_churn_no_pdf_rerun_passes > 0 {
        1
    } else if bibcite_preflight_seeded {
        1
    } else {
        2
    }
}

fn can_accept_final_pdf_with_stale_rerun_warnings(
    draft_graphics: bool,
    suppress_pdf_output: bool,
    generated_outputs_changed: bool,
    generated_inputs_unread: bool,
    standard_rerun_outputs_changed: bool,
    rerun_reasons: &[String],
    accept_stale_after_stable_draft: bool,
) -> bool {
    !draft_graphics
        && !suppress_pdf_output
        && !generated_outputs_changed
        && !generated_inputs_unread
        && (!standard_rerun_outputs_changed || accept_stale_after_stable_draft)
        && !rerun_reasons.is_empty()
        && rerun_reasons
            .iter()
            .all(|reason| is_stale_standard_rerun_reason(reason))
}

fn is_stale_standard_rerun_reason(reason: &str) -> bool {
    matches!(reason, "file-changed" | "rerun-to-get-cross-references")
}

#[derive(Debug, Clone, Default)]
struct PgfExternalizeScan {
    uses_externalize: bool,
    has_explicit_mode: bool,
}

impl PgfExternalizeScan {
    fn merge(&mut self, other: PgfExternalizeScan) {
        self.uses_externalize |= other.uses_externalize;
        self.has_explicit_mode |= other.has_explicit_mode;
    }
}

#[cfg(test)]
fn pgf_externalize_scan_from_source(
    doc_dir: &Path,
    source_path: &Path,
    visited: &mut HashSet<PathBuf>,
    includeonly: Option<&HashSet<String>>,
) -> Result<PgfExternalizeScan> {
    let source_cache = TexSourceReadCache::default();
    pgf_externalize_scan_from_source_with_cache(
        doc_dir,
        source_path,
        visited,
        includeonly,
        &source_cache,
    )
}

#[cfg(test)]
fn pgf_externalize_scan_from_source_with_cache(
    doc_dir: &Path,
    source_path: &Path,
    visited: &mut HashSet<PathBuf>,
    includeonly: Option<&HashSet<String>>,
    source_cache: &TexSourceReadCache,
) -> Result<PgfExternalizeScan> {
    let source_path = canonical_tex_source_path(source_path)?;
    if !visited.insert(source_path.clone()) {
        return Ok(PgfExternalizeScan::default());
    }
    let source = read_cached_tex_source(source_cache, &source_path)?;
    let mut scan = pgf_externalize_scan(&source);

    for include in active_include_source_dependencies(&source, includeonly) {
        if let Some(path) = resolve_local_tex_source_dependency(doc_dir, &include)? {
            scan.merge(pgf_externalize_scan_from_source_with_cache(
                doc_dir,
                &path,
                visited,
                includeonly,
                source_cache,
            )?);
        }
    }

    for input in tex_input_like_source_dependencies(&source) {
        if let Some(path) = resolve_local_tex_source_dependency(doc_dir, &input)? {
            scan.merge(pgf_externalize_scan_from_source_with_cache(
                doc_dir,
                &path,
                visited,
                includeonly,
                source_cache,
            )?);
        }
    }

    Ok(scan)
}

#[cfg(test)]
fn pgf_externalize_scan(source: &str) -> PgfExternalizeScan {
    let source = tex_comment_stripped_source(source);
    pgf_externalize_scan_stripped(&source)
}

fn pgf_externalize_scan_stripped(source: &str) -> PgfExternalizeScan {
    let mut scan = PgfExternalizeScan::default();
    for externalize in tikz_externalize_refs_stripped(source) {
        scan.uses_externalize = true;
        scan.has_explicit_mode |= tex_options_define_pgf_external_mode(&externalize.options);
    }
    for options in tex_command_balanced_payloads_stripped(source, "tikzset") {
        scan.has_explicit_mode |= tex_options_define_pgf_external_mode(&options);
    }
    scan
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TikzExternalizeRef {
    options: String,
}

fn tikz_externalize_refs_stripped(source: &str) -> Vec<TikzExternalizeRef> {
    let command = r"\tikzexternalize";
    let mut refs = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find(command) {
        let after_command = cursor + offset + command.len();
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let payload_start = skip_tex_whitespace(source, after_command);
        let (options, end) = if source[payload_start..].starts_with('[') {
            match bracketed_tex_argument_payload_at(source, payload_start) {
                Some((options, options_end)) => (options, options_end),
                None => (String::new(), after_command),
            }
        } else {
            (String::new(), after_command)
        };
        refs.push(TikzExternalizeRef { options });
        cursor = end;
    }
    refs
}

fn tex_options_define_pgf_external_mode(options: &str) -> bool {
    split_tex_keyvals(options).into_iter().any(|option| {
        let key = option.key.to_ascii_lowercase().replace([' ', '_'], "");
        key == "mode" || key == "external/mode"
    })
}

#[derive(Debug, Clone, Default)]
struct SourceFeatures {
    has_graphics: bool,
    has_multipass_signal: bool,
    has_backref_signal: bool,
    graphic_command_count: usize,
}

#[derive(Debug, Clone, Copy)]
struct SourcePreflightOptions {
    collect_features: bool,
    collect_pgf_externalize: bool,
    prepare_output_subdirs: bool,
}

#[derive(Clone, Copy)]
struct SourcePreflightContext<'a> {
    doc_dir: &'a Path,
    out_dir: &'a Path,
    includeonly: Option<&'a HashSet<String>>,
    source_cache: &'a TexSourceReadCache,
    options: SourcePreflightOptions,
}

#[derive(Debug, Default)]
struct SourcePreflightScan {
    features: SourceFeatures,
    pgf_externalize: PgfExternalizeScan,
}

fn source_preflight_scan(
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    includeonly: Option<&HashSet<String>>,
    source_cache: &TexSourceReadCache,
    options: SourcePreflightOptions,
) -> Result<SourcePreflightScan> {
    let mut scan = SourcePreflightScan::default();
    let mut visited = HashSet::new();
    let context = SourcePreflightContext {
        doc_dir,
        out_dir,
        includeonly,
        source_cache,
        options,
    };
    source_preflight_scan_from_source(context, main, &mut visited, &mut scan)?;
    Ok(scan)
}

fn source_seed_dependency_paths(main: &Path) -> Result<Vec<PathBuf>> {
    let doc_dir = main
        .parent()
        .context("root TeX file has no parent directory")?;
    let includeonly = includeonly_filter_for_root(main)?;
    let source_cache = TexSourceReadCache::default();
    let context = SourceSeedContext {
        doc_dir,
        includeonly: includeonly.as_ref(),
        source_cache: &source_cache,
    };
    let mut state = SourceSeedState::default();
    collect_source_seed_dependency_paths(context, main, &mut state)?;
    let mut paths = state.paths;
    paths.sort();
    paths.dedup();
    Ok(paths)
}

#[derive(Clone, Copy)]
struct SourceSeedContext<'a> {
    doc_dir: &'a Path,
    includeonly: Option<&'a HashSet<String>>,
    source_cache: &'a TexSourceReadCache,
}

struct SourceSeedState {
    visited: HashSet<PathBuf>,
    aux_visited: HashSet<PathBuf>,
    paths: Vec<PathBuf>,
    graphic_paths: Vec<PathBuf>,
    graphic_extensions: Option<Vec<String>>,
    svg_paths: Vec<PathBuf>,
    svg_settings: SvgIncludeSettings,
}

impl Default for SourceSeedState {
    fn default() -> Self {
        Self {
            visited: HashSet::new(),
            aux_visited: HashSet::new(),
            paths: Vec::new(),
            graphic_paths: Vec::new(),
            graphic_extensions: None,
            svg_paths: Vec::new(),
            svg_settings: default_svg_include_settings(),
        }
    }
}

fn collect_source_seed_dependency_paths(
    context: SourceSeedContext<'_>,
    source_path: &Path,
    state: &mut SourceSeedState,
) -> Result<()> {
    let source_path = canonical_tex_source_path(source_path)?;
    if !state.visited.insert(source_path.clone()) {
        return Ok(());
    }
    state.paths.push(source_path.clone());

    let analysis = read_cached_tex_source_analysis(context.source_cache, &source_path)?;
    let original_graphic_path_count = state.graphic_paths.len();
    state.graphic_paths.extend(
        analysis
            .graphicspath_entries
            .iter()
            .cloned()
            .map(PathBuf::from),
    );
    let original_svg_path_count = state.svg_paths.len();
    state.svg_paths.extend(
        analysis
            .graphicspath_entries
            .iter()
            .cloned()
            .map(PathBuf::from),
    );
    state
        .svg_paths
        .extend(analysis.svgpath_entries.iter().cloned().map(PathBuf::from));
    let original_graphic_extensions = state.graphic_extensions.clone();
    if let Some(declared) = &analysis.declared_graphics_extensions {
        state.graphic_extensions = Some(declared.clone());
    }
    let original_svg_settings = state.svg_settings.clone();

    for payload in &analysis.class_payloads {
        if let Some(path) = resolve_source_class_input(context.doc_dir, payload)? {
            collect_resolved_source_seed_dependency_path(context, &path, state, "document class")?;
        }
    }
    for payload in &analysis.package_payloads {
        if let Some(path) = resolve_source_package_input(context.doc_dir, payload)? {
            collect_resolved_source_seed_dependency_path(context, &path, state, "package")?;
        }
    }
    for payload in &analysis.bibliography_payloads {
        if let Some(path) = resolve_source_bibliography_input(context.doc_dir, payload)? {
            state.paths.push(path.canonicalize().with_context(|| {
                format!(
                    "failed to canonicalize bibliography dependency {}",
                    path.display()
                )
            })?);
        }
    }
    for payload in &analysis.bibliography_style_payloads {
        if let Some(path) = resolve_source_bibliography_style_input(context.doc_dir, payload)? {
            state.paths.push(path.canonicalize().with_context(|| {
                format!(
                    "failed to canonicalize bibliography style dependency {}",
                    path.display()
                )
            })?);
        }
    }
    for graphic in &analysis.includegraphics_payloads {
        if let Some(path) = resolve_source_graphic_input(
            context.doc_dir,
            &state.graphic_paths,
            graphic,
            state.graphic_extensions.as_deref(),
        )? {
            state.paths.push(path.canonicalize().with_context(|| {
                format!(
                    "failed to canonicalize graphic dependency {}",
                    path.display()
                )
            })?);
        }
    }
    for animation in &analysis.animategraphics_refs {
        for graphic in animategraphics_frame_payloads(animation) {
            if let Some(path) = resolve_source_graphic_input(
                context.doc_dir,
                &state.graphic_paths,
                &graphic,
                state.graphic_extensions.as_deref(),
            )? {
                state.paths.push(path.canonicalize().with_context(|| {
                    format!(
                        "failed to canonicalize animated graphic dependency {}",
                        path.display()
                    )
                })?);
            }
        }
    }
    for pdf in &analysis.pdfpages_payloads {
        if let Some(path) = resolve_source_pdf_input(context.doc_dir, &state.graphic_paths, pdf)? {
            state.paths.push(path.canonicalize().with_context(|| {
                format!("failed to canonicalize PDF dependency {}", path.display())
            })?);
        }
    }
    for payload in &analysis.source_file_payloads {
        if let Some(path) = resolve_source_file_input(context.doc_dir, payload)? {
            if path_extension_is_any(&path, &["aux"]) {
                collect_source_seed_aux_dependency_paths(context, &path, state)?;
            } else {
                state.paths.push(path.canonicalize().with_context(|| {
                    format!(
                        "failed to canonicalize source file dependency {}",
                        path.display()
                    )
                })?);
            }
        }
    }
    for svg in &analysis.includesvg_refs {
        let Some(settings) =
            svg_settings_for_include(&state.svg_settings, &analysis.svg_setup_refs, svg)
        else {
            continue;
        };
        if let Some(path) =
            resolve_source_svg_input(context.doc_dir, &state.svg_paths, svg, &settings)?
        {
            state.paths.push(path.canonicalize().with_context(|| {
                format!("failed to canonicalize SVG dependency {}", path.display())
            })?);
        }
    }
    let inherited_svg_settings =
        svg_settings_after_setups(&state.svg_settings, &analysis.svg_setup_refs).unwrap_or_else(
            || {
                let mut settings = state.svg_settings.clone();
                settings.inkscape_enabled = false;
                settings
            },
        );
    for dependency in active_include_dependencies(&analysis, context.includeonly)
        .into_iter()
        .chain(analysis.input_dependencies.iter().cloned())
    {
        if let Some(path) = resolve_local_tex_source_dependency(context.doc_dir, &dependency)? {
            let dependency_graphic_path_count = state.graphic_paths.len();
            let dependency_svg_path_count = state.svg_paths.len();
            let dependency_svg_settings = state.svg_settings.clone();
            if let Some(local_graphic_path) = dependency.local_graphic_path {
                state.graphic_paths.push(local_graphic_path.clone());
                state.svg_paths.push(local_graphic_path);
            }
            state.svg_settings = inherited_svg_settings.clone();
            collect_resolved_source_seed_dependency_path(context, &path, state, "TeX dependency")?;
            state.graphic_paths.truncate(dependency_graphic_path_count);
            state.svg_paths.truncate(dependency_svg_path_count);
            state.svg_settings = dependency_svg_settings;
        }
    }

    state.graphic_paths.truncate(original_graphic_path_count);
    state.svg_paths.truncate(original_svg_path_count);
    state.graphic_extensions = original_graphic_extensions;
    state.svg_settings = original_svg_settings;
    Ok(())
}

fn collect_source_seed_aux_dependency_paths(
    context: SourceSeedContext<'_>,
    aux_path: &Path,
    state: &mut SourceSeedState,
) -> Result<()> {
    let aux_path = aux_path.canonicalize().with_context(|| {
        format!(
            "failed to canonicalize external aux dependency {}",
            aux_path.display()
        )
    })?;
    if !state.aux_visited.insert(aux_path.clone()) {
        return Ok(());
    }
    state.paths.push(aux_path.clone());

    let source = fs::read_to_string(&aux_path).with_context(|| {
        format!(
            "failed to read external aux dependency {}",
            aux_path.display()
        )
    })?;
    let base_dir = aux_path.parent().unwrap_or(context.doc_dir);
    for payload in source_aux_input_payloads(&source) {
        let input_path = Path::new(&payload);
        if !safe_relative_path(input_path) {
            continue;
        }
        let nested = base_dir.join(input_path);
        if nested.is_file() {
            collect_source_seed_aux_dependency_paths(context, &nested, state)?;
        }
    }
    Ok(())
}

fn collect_resolved_source_seed_dependency_path(
    context: SourceSeedContext<'_>,
    path: &Path,
    state: &mut SourceSeedState,
    description: &str,
) -> Result<()> {
    let path = path.canonicalize().with_context(|| {
        format!(
            "failed to canonicalize {description} dependency {}",
            path.display()
        )
    })?;
    if source_seed_dependency_is_recursable(context.doc_dir, &path) {
        collect_source_seed_dependency_paths(context, &path, state)?;
    } else {
        state.paths.push(path);
    }
    Ok(())
}

fn source_seed_dependency_is_recursable(doc_dir: &Path, path: &Path) -> bool {
    path_extension_is_any(
        path,
        &["tex", "ltx", "sty", "cls", "def", "cfg", "clo", "dtx"],
    ) && (path_is_under(path, doc_dir)
        || texinputs_source_seed_roots(doc_dir)
            .iter()
            .any(|root| path_is_under(path, root)))
}

fn texinputs_source_seed_roots(doc_dir: &Path) -> Vec<PathBuf> {
    let Some(value) = std::env::var_os("TEXINPUTS") else {
        return Vec::new();
    };
    std::env::split_paths(&value)
        .filter_map(|entry| kpathsea_path_entry_root(doc_dir, entry))
        .collect()
}

fn kpathsea_path_entry_root(doc_dir: &Path, entry: PathBuf) -> Option<PathBuf> {
    let entry = entry.to_string_lossy();
    let entry = entry.strip_prefix("!!").unwrap_or(&entry);
    let entry = entry.strip_suffix("//").unwrap_or(entry);
    if entry.is_empty() {
        return None;
    }
    let path = PathBuf::from(entry);
    let path = if path.is_absolute() {
        path
    } else {
        doc_dir.join(path)
    };
    path.is_dir().then(|| path.canonicalize().unwrap_or(path))
}

fn resolve_source_package_input(doc_dir: &Path, payload: &str) -> Result<Option<PathBuf>> {
    resolve_source_kpathsea_input(doc_dir, payload, "sty")
}

fn resolve_source_class_input(doc_dir: &Path, payload: &str) -> Result<Option<PathBuf>> {
    resolve_source_kpathsea_input(doc_dir, payload, "cls")
}

fn resolve_source_bibliography_input(doc_dir: &Path, payload: &str) -> Result<Option<PathBuf>> {
    resolve_source_kpathsea_input(doc_dir, payload, "bib")
}

fn resolve_source_bibliography_style_input(
    doc_dir: &Path,
    payload: &str,
) -> Result<Option<PathBuf>> {
    resolve_source_kpathsea_input(doc_dir, payload, "bst")
}

fn resolve_source_graphic_input(
    doc_dir: &Path,
    graphic_paths: &[PathBuf],
    payload: &str,
    declared_extensions: Option<&[String]>,
) -> Result<Option<PathBuf>> {
    let requested = Path::new(payload);
    if !safe_relative_path(requested) {
        return Ok(None);
    }

    let search_dirs = std::iter::once(PathBuf::new())
        .chain(graphic_paths.iter().cloned())
        .collect::<Vec<_>>();
    if let Some(extension) = requested
        .extension()
        .and_then(|extension| extension.to_str())
    {
        for directory in search_dirs {
            let candidate = doc_dir.join(directory).join(requested);
            if candidate.is_file() {
                return Ok(Some(candidate));
            }
        }
        return resolve_kpathsea_input(doc_dir, payload, extension);
    }

    let default_extensions = [
        "pdf".to_string(),
        "png".to_string(),
        "jpg".to_string(),
        "jpeg".to_string(),
        "mps".to_string(),
        "eps".to_string(),
    ];
    let extensions = declared_extensions.unwrap_or(&default_extensions);
    for directory in search_dirs {
        let base = doc_dir.join(directory).join(requested);
        for extension in extensions {
            let candidate = base.with_extension(extension);
            if candidate.is_file() {
                return Ok(Some(candidate));
            }
        }
    }
    for directory in std::iter::once(PathBuf::new()).chain(graphic_paths.iter().cloned()) {
        let payload = directory.join(requested).to_string_lossy().to_string();
        for extension in extensions {
            if let Some(path) = resolve_kpathsea_input(doc_dir, &payload, extension)? {
                return Ok(Some(path));
            }
        }
    }
    Ok(None)
}

fn resolve_source_svg_input(
    doc_dir: &Path,
    svg_paths: &[PathBuf],
    svg: &IncludeSvgRef,
    settings: &SvgIncludeSettings,
) -> Result<Option<PathBuf>> {
    if !settings.inkscape_enabled || !matches!(settings.format.as_str(), "pdf" | "png") {
        return Ok(None);
    }
    resolve_svg_input(doc_dir, svg_paths, &svg.payload, &settings.source_extension)
}

fn resolve_source_pdf_input(
    doc_dir: &Path,
    graphic_paths: &[PathBuf],
    payload: &str,
) -> Result<Option<PathBuf>> {
    let requested = Path::new(payload.trim());
    if !safe_relative_path(requested) {
        return Ok(None);
    }
    if requested
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| !extension.eq_ignore_ascii_case("pdf"))
    {
        return Ok(None);
    }

    let search_dirs = std::iter::once(PathBuf::new())
        .chain(graphic_paths.iter().cloned())
        .collect::<Vec<_>>();
    for directory in search_dirs {
        let candidate = doc_dir.join(directory).join(requested);
        if candidate.is_file() {
            return Ok(Some(candidate));
        }
        if candidate.extension().is_none() {
            let pdf_candidate = candidate.with_extension("pdf");
            if pdf_candidate.is_file() {
                return Ok(Some(pdf_candidate));
            }
        }
    }
    for directory in std::iter::once(PathBuf::new()).chain(graphic_paths.iter().cloned()) {
        let payload = directory.join(requested).to_string_lossy().to_string();
        if let Some(path) = resolve_kpathsea_input(doc_dir, &payload, "pdf")? {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

fn resolve_source_file_input(doc_dir: &Path, payload: &str) -> Result<Option<PathBuf>> {
    let payload = payload.trim();
    if payload.is_empty() || payload.contains("://") || payload.starts_with('\\') {
        return Ok(None);
    }
    let Some(path) = safe_relative_payload_path(payload) else {
        return Ok(None);
    };
    let candidate = doc_dir.join(path);
    if candidate.is_file() {
        return Ok(Some(candidate));
    }
    let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
        return Ok(None);
    };
    resolve_kpathsea_input(doc_dir, payload, extension)
}

fn resolve_source_kpathsea_input(
    doc_dir: &Path,
    payload: &str,
    extension: &str,
) -> Result<Option<PathBuf>> {
    let payload = payload.trim();
    if payload.is_empty() || payload.contains("://") {
        return Ok(None);
    }
    if payload.starts_with('\\') {
        return Ok(None);
    }
    resolve_kpathsea_input(doc_dir, payload, extension)
}

fn source_preflight_scan_from_source(
    context: SourcePreflightContext<'_>,
    source_path: &Path,
    visited: &mut HashSet<PathBuf>,
    scan: &mut SourcePreflightScan,
) -> Result<()> {
    let source_path = canonical_tex_source_path(source_path)?;
    if !visited.insert(source_path.clone()) {
        return Ok(());
    }
    let analysis = read_cached_tex_source_analysis(context.source_cache, &source_path)?;

    if context.options.collect_features {
        scan.features.merge(analysis.features.clone());
    }
    if context.options.collect_pgf_externalize {
        scan.pgf_externalize.merge(analysis.pgf_externalize.clone());
    }

    for include in active_include_dependencies(&analysis, context.includeonly) {
        if context.options.prepare_output_subdirs
            && let Some(parent) = safe_relative_parent(Path::new(&include.payload))
        {
            fs::create_dir_all(context.out_dir.join(parent)).with_context(|| {
                format!(
                    "failed to create output include directory for {}",
                    include.payload
                )
            })?;
        }
        if let Some(path) = resolve_local_tex_source_dependency(context.doc_dir, &include)? {
            source_preflight_scan_from_source(context, &path, visited, scan)?;
        }
    }

    for input in &analysis.input_dependencies {
        if let Some(path) = resolve_local_tex_source_dependency(context.doc_dir, input)? {
            source_preflight_scan_from_source(context, &path, visited, scan)?;
        }
    }

    Ok(())
}

fn source_features(doc_dir: &Path, main: &Path) -> Result<SourceFeatures> {
    let includeonly = includeonly_filter_for_root(main)?;
    let source_cache = TexSourceReadCache::default();
    source_features_with_includeonly(doc_dir, main, includeonly.as_ref(), &source_cache)
}

fn source_features_with_includeonly(
    doc_dir: &Path,
    main: &Path,
    includeonly: Option<&HashSet<String>>,
    source_cache: &TexSourceReadCache,
) -> Result<SourceFeatures> {
    let mut visited = HashSet::new();
    source_features_from_source(doc_dir, main, &mut visited, includeonly, source_cache)
}

fn source_features_from_source(
    doc_dir: &Path,
    source_path: &Path,
    visited: &mut HashSet<PathBuf>,
    includeonly: Option<&HashSet<String>>,
    source_cache: &TexSourceReadCache,
) -> Result<SourceFeatures> {
    let source_path = canonical_tex_source_path(source_path)?;
    if !visited.insert(source_path.clone()) {
        return Ok(SourceFeatures::default());
    }
    let analysis = read_cached_tex_source_analysis(source_cache, &source_path)?;

    let mut features = analysis.features.clone();
    for include in active_include_dependencies(&analysis, includeonly) {
        if let Some(path) = resolve_local_tex_source_dependency(doc_dir, &include)? {
            features.merge(source_features_from_source(
                doc_dir,
                &path,
                visited,
                includeonly,
                source_cache,
            )?);
        }
    }

    for input in &analysis.input_dependencies {
        if let Some(path) = resolve_local_tex_source_dependency(doc_dir, input)? {
            features.merge(source_features_from_source(
                doc_dir,
                &path,
                visited,
                includeonly,
                source_cache,
            )?);
        }
    }

    Ok(features)
}

fn source_features_in_stripped_source_with_graphics_count(
    scan_source: &str,
    graphics_count: usize,
) -> SourceFeatures {
    let mut features = SourceFeatures::default();
    for line in scan_source.lines() {
        features.has_graphics |= tex_command_present(line, "includegraphics");
        features.has_graphics |= tex_command_present(line, "includesvg");
        features.has_graphics |= tex_command_present(line, "includepdf");
        features.has_graphics |= tex_command_present(line, "includepdfmerge");
        features.has_multipass_signal |= has_multipass_signal(line);
        features.has_backref_signal |= has_backref_signal(line);
    }
    if graphics_count > 0 {
        features.graphic_command_count += graphics_count;
    } else if features.has_graphics {
        features.graphic_command_count += 1;
    }
    features
}

impl SourceFeatures {
    fn merge(&mut self, other: SourceFeatures) {
        self.has_graphics |= other.has_graphics;
        self.has_multipass_signal |= other.has_multipass_signal;
        self.has_backref_signal |= other.has_backref_signal;
        self.graphic_command_count += other.graphic_command_count;
    }
}

fn has_backref_signal(line: &str) -> bool {
    line.contains("pagebackref")
        || tex_command_present(line, "backref")
        || tex_command_present(line, "backrefalt")
}

fn has_multipass_signal(line: &str) -> bool {
    const MULTIPASS_COMMANDS: &[&str] = &[
        "addbibresource",
        "autocite",
        "autoref",
        "bibliography",
        "Cref",
        "cite",
        "citealp",
        "citeauthor",
        "citep",
        "citet",
        "citeyear",
        "cref",
        "eqref",
        "footcite",
        "gls",
        "include",
        "label",
        "listoffigures",
        "listoftables",
        "makeglossaries",
        "makeindex",
        "nameref",
        "nomenclature",
        "pageref",
        "parencite",
        "printbibliography",
        "printglossary",
        "printindex",
        "ref",
        "supercite",
        "tableofcontents",
        "textcite",
    ];
    MULTIPASS_COMMANDS
        .iter()
        .copied()
        .any(|command| tex_command_present(line, command))
}

fn tex_command_present(line: &str, command: &str) -> bool {
    let mut cursor = 0;
    while let Some(offset) = line[cursor..].find('\\') {
        let command_start = cursor + offset;
        let name_start = command_start + '\\'.len_utf8();
        if !line[name_start..].starts_with(command) {
            cursor = name_start;
            continue;
        }
        let after_command = name_start + command.len();
        if line[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }
        return true;
    }
    false
}

#[cfg(test)]
fn tex_command_payloads(source: &str, command: &str) -> Vec<String> {
    tex_command_balanced_payloads(source, command)
}

#[cfg(test)]
fn source_bibliography_payloads(source: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    source_bibliography_payloads_stripped(&source)
}

fn source_bibliography_payloads_stripped(source: &str) -> Vec<String> {
    let mut payloads = Vec::new();
    for payload in tex_command_balanced_payloads_stripped(source, "bibliography") {
        payloads.extend(
            split_tex_top_level(&payload, ',')
                .into_iter()
                .map(|entry| entry.trim().to_string())
                .filter(|entry| !entry.is_empty()),
        );
    }
    for command in ["addbibresource", "addglobalbib", "addsectionbib"] {
        payloads.extend(tex_command_optional_braced_payloads_stripped(
            source, command,
        ));
    }
    payloads.sort();
    payloads.dedup();
    payloads
}

#[cfg(test)]
fn source_package_payloads(source: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    source_package_payloads_stripped(&source)
}

fn source_package_payloads_stripped(source: &str) -> Vec<String> {
    source_tex_resource_payloads_stripped(
        source,
        &["usepackage", "RequirePackage", "RequirePackageWithOptions"],
        true,
    )
}

#[cfg(test)]
fn source_class_payloads(source: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    source_class_payloads_stripped(&source)
}

fn source_class_payloads_stripped(source: &str) -> Vec<String> {
    source_tex_resource_payloads_stripped(
        source,
        &["documentclass", "LoadClass", "LoadClassWithOptions"],
        false,
    )
}

fn source_tex_resource_payloads_stripped(
    source: &str,
    commands: &[&str],
    split_commas: bool,
) -> Vec<String> {
    let mut payloads = Vec::new();
    for command in commands {
        for payload in tex_command_optional_braced_payloads_stripped(source, command) {
            if split_commas {
                payloads.extend(
                    split_tex_top_level(&payload, ',')
                        .into_iter()
                        .map(|entry| entry.trim().to_string())
                        .filter(|entry| !entry.is_empty()),
                );
            } else {
                let payload = payload.trim();
                if !payload.is_empty() {
                    payloads.push(payload.to_string());
                }
            }
        }
    }
    payloads.sort();
    payloads.dedup();
    payloads
}

#[cfg(test)]
fn source_bibliography_style_payloads(source: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    source_bibliography_style_payloads_stripped(&source)
}

fn source_bibliography_style_payloads_stripped(source: &str) -> Vec<String> {
    let mut payloads = tex_command_balanced_payloads_stripped(source, "bibliographystyle")
        .into_iter()
        .map(|payload| payload.trim().to_string())
        .filter(|payload| !payload.is_empty())
        .collect::<Vec<_>>();
    payloads.sort();
    payloads.dedup();
    payloads
}

#[cfg(test)]
fn source_pdfpages_payloads(source: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    source_pdfpages_payloads_stripped(&source)
}

fn source_pdfpages_payloads_stripped(source: &str) -> Vec<String> {
    let mut payloads = tex_command_optional_braced_payloads_stripped(source, "includepdf")
        .into_iter()
        .map(|payload| payload.trim().to_string())
        .filter(|payload| !payload.is_empty())
        .collect::<Vec<_>>();
    for payload in tex_command_optional_braced_payloads_stripped(source, "includepdfmerge") {
        payloads.extend(
            split_tex_top_level(&payload, ',')
                .into_iter()
                .step_by(2)
                .map(|entry| entry.trim().to_string())
                .filter(|entry| !entry.is_empty()),
        );
    }
    payloads.sort();
    payloads.dedup();
    payloads
}

#[cfg(test)]
fn source_file_payloads(source: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    source_file_payloads_stripped(&source)
}

fn source_file_payloads_stripped(source: &str) -> Vec<String> {
    let mut payloads = Vec::new();
    for command in ["lstinputlisting", "verbatiminput", "VerbatimInput"] {
        payloads.extend(tex_command_optional_braced_payloads_stripped(
            source, command,
        ));
    }
    for command in ["attachfile", "textattachfile", "notextattachfile"] {
        payloads.extend(tex_command_optional_braced_payloads_stripped(
            source, command,
        ));
    }
    payloads.extend(tex_command_optional_nth_braced_payloads_stripped(
        source,
        "inputminted",
        2,
    ));
    payloads.extend(tex_command_optional_nth_braced_payloads_stripped(
        source,
        "includemedia",
        2,
    ));
    for command in ["DTLloaddb", "DTLloadrawdb", "DTLloadtexdb"] {
        payloads.extend(tex_command_optional_nth_braced_payloads_stripped(
            source, command, 2,
        ));
    }
    for command in [
        "csvreader",
        "csvautotabular",
        "csvautobooktabular",
        "csvautolongtable",
    ] {
        payloads.extend(tex_command_optional_braced_payloads_stripped(
            source, command,
        ));
    }
    payloads.extend(tex_command_optional_braced_payloads_stripped(
        source,
        "pgfplotstableread",
    ));
    payloads.extend(pgfplots_addplot_table_payloads_stripped(source));
    payloads.extend(source_external_aux_payloads_stripped(source));
    payloads.sort();
    payloads.dedup();
    payloads
}

fn source_external_aux_payloads_stripped(source: &str) -> Vec<String> {
    let mut payloads = Vec::new();
    for command in ["externaldocument", "externalcitedocument"] {
        payloads.extend(
            tex_command_optional_braced_payloads_stripped(source, command)
                .into_iter()
                .filter_map(|payload| external_aux_payload(&payload)),
        );
    }
    payloads.extend(
        tex_command_optional_star_braced_payloads_stripped(source, "zexternaldocument")
            .into_iter()
            .filter_map(|payload| external_aux_payload(&payload)),
    );
    payloads
}

fn external_aux_payload(payload: &str) -> Option<String> {
    let payload = payload.trim();
    if payload.is_empty() || payload.contains("://") || payload.starts_with('\\') {
        return None;
    }
    let path = safe_relative_payload_path(payload)?;
    path.file_name()?;
    if path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("aux"))
    {
        Some(payload.to_string())
    } else {
        Some(format!("{payload}.aux"))
    }
}

fn source_aux_input_payloads(source: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|line| braced_payload(line.trim_start(), r"\@input"))
        .map(str::trim)
        .filter(|payload| !payload.is_empty())
        .map(str::to_string)
        .collect()
}

fn pgfplots_addplot_table_payloads_stripped(source: &str) -> Vec<String> {
    let command = r"\addplot";
    let mut payloads = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find(command) {
        let command_start = cursor + offset;
        let mut after_command = command_start + command.len();
        if source[after_command..].starts_with('3') {
            after_command += 1;
        }
        if source[after_command..].starts_with('+') {
            after_command += 1;
        }
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let mut table_start = skip_tex_whitespace(source, after_command);
        while source[table_start..].starts_with('[') {
            if let Some(argument_end) = bracketed_tex_argument_end(source, table_start) {
                table_start = skip_tex_whitespace(source, argument_end);
            } else {
                break;
            }
        }
        if !source[table_start..].starts_with("table")
            || source[table_start + "table".len()..]
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let mut payload_start = skip_tex_whitespace(source, table_start + "table".len());
        while source[payload_start..].starts_with('[') {
            if let Some(argument_end) = bracketed_tex_argument_end(source, payload_start) {
                payload_start = skip_tex_whitespace(source, argument_end);
            } else {
                break;
            }
        }
        let Some((payload, payload_end)) = balanced_braced_payload_at(source, payload_start) else {
            cursor = table_start + "table".len();
            continue;
        };
        if !payload.trim().is_empty() {
            payloads.push(payload.trim().to_string());
        }
        cursor = payload_end;
    }
    payloads
}

fn includeonly_filter_for_root(main: &Path) -> Result<Option<HashSet<String>>> {
    let source = fs::read_to_string(main)
        .with_context(|| format!("failed to read TeX source {}", main.display()))?;
    Ok(includeonly_filter(&source))
}

fn includeonly_filter(source: &str) -> Option<HashSet<String>> {
    let payload = tex_command_balanced_payloads(source, "includeonly")
        .into_iter()
        .last()?;
    Some(
        split_tex_top_level(&payload, ',')
            .into_iter()
            .filter_map(|entry| normalized_include_name(&entry))
            .collect(),
    )
}

fn normalized_include_name(payload: &str) -> Option<String> {
    let name = payload.trim();
    if name.is_empty()
        || Path::new(name)
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        return None;
    }
    Some(strip_tex_extension_ignore_ascii_case(name).to_string())
}

fn strip_tex_extension_ignore_ascii_case(name: &str) -> &str {
    const TEX_EXTENSION: &str = ".tex";
    let bytes = name.as_bytes();
    if bytes.len() >= TEX_EXTENSION.len()
        && bytes[bytes.len() - TEX_EXTENSION.len()..].eq_ignore_ascii_case(TEX_EXTENSION.as_bytes())
    {
        &name[..name.len() - TEX_EXTENSION.len()]
    } else {
        name
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct TexSourceDependency {
    payload: String,
    local_graphic_path: Option<PathBuf>,
    tex_like_only: bool,
    obey_includeonly: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TexTwoArgPathRef {
    directory: String,
    payload: String,
}

#[cfg(test)]
fn tex_include_source_dependencies(source: &str) -> Vec<TexSourceDependency> {
    let source = tex_comment_stripped_source(source);
    tex_include_source_dependencies_stripped(&source)
}

fn tex_include_source_dependencies_stripped(source: &str) -> Vec<TexSourceDependency> {
    let mut dependencies = Vec::new();
    dependencies.extend(
        tex_command_payloads_stripped(source, "include")
            .into_iter()
            .map(tex_includeonly_source_dependency),
    );
    dependencies.extend(
        tex_command_payloads_stripped(source, "subfileinclude")
            .into_iter()
            .map(tex_includeonly_source_dependency),
    );
    for command in ["includefrom", "subincludefrom"] {
        dependencies.extend(
            tex_two_arg_path_refs_stripped(source, command)
                .into_iter()
                .map(tex_two_arg_includeonly_source_dependency),
        );
    }
    dedup_source_dependencies(dependencies)
}

#[cfg(test)]
fn active_include_source_dependencies(
    source: &str,
    includeonly: Option<&HashSet<String>>,
) -> Vec<TexSourceDependency> {
    tex_include_source_dependencies(source)
        .into_iter()
        .filter(|dependency| include_dependency_is_active(dependency, includeonly))
        .collect()
}

fn include_dependency_is_active(
    dependency: &TexSourceDependency,
    includeonly: Option<&HashSet<String>>,
) -> bool {
    if !dependency.obey_includeonly {
        return true;
    }
    let Some(includeonly) = includeonly else {
        return true;
    };
    normalized_include_name(&dependency.payload)
        .as_ref()
        .is_some_and(|name| includeonly.contains(name))
}

#[cfg(test)]
fn tex_include_payloads(source: &str) -> Vec<String> {
    dedup_payloads(
        tex_include_source_dependencies(source)
            .into_iter()
            .map(|dependency| dependency.payload)
            .collect(),
    )
}

#[cfg(test)]
fn tex_input_like_source_dependencies(source: &str) -> Vec<TexSourceDependency> {
    let source = tex_comment_stripped_source(source);
    tex_input_like_source_dependencies_stripped(&source)
}

fn tex_input_like_source_dependencies_stripped(source: &str) -> Vec<TexSourceDependency> {
    let mut dependencies = Vec::new();
    dependencies.extend(
        tex_input_payloads_stripped(source)
            .into_iter()
            .map(tex_source_dependency),
    );
    for command in ["subfile", "InputIfFileExists"] {
        dependencies.extend(
            tex_command_payloads_stripped(source, command)
                .into_iter()
                .map(tex_source_dependency),
        );
    }
    dependencies.extend(
        tex_command_optional_braced_payloads_stripped(source, "includestandalone")
            .into_iter()
            .map(tex_source_dependency),
    );
    dependencies.extend(
        tex_command_payloads_stripped(source, "IfFileExists")
            .into_iter()
            .map(tex_like_probe_source_dependency),
    );
    for command in ["import", "subimport", "inputfrom", "subinputfrom"] {
        dependencies.extend(
            tex_two_arg_path_refs_stripped(source, command)
                .into_iter()
                .map(tex_two_arg_source_dependency),
        );
    }
    dedup_source_dependencies(dependencies)
}

#[cfg(test)]
fn tex_input_like_payloads(source: &str) -> Vec<String> {
    dedup_payloads(
        tex_input_like_source_dependencies(source)
            .into_iter()
            .map(|dependency| dependency.payload)
            .collect(),
    )
}

fn tex_source_dependency(payload: String) -> TexSourceDependency {
    TexSourceDependency {
        payload,
        local_graphic_path: None,
        tex_like_only: false,
        obey_includeonly: false,
    }
}

fn tex_includeonly_source_dependency(payload: String) -> TexSourceDependency {
    TexSourceDependency {
        payload,
        local_graphic_path: None,
        tex_like_only: false,
        obey_includeonly: true,
    }
}

fn tex_like_probe_source_dependency(payload: String) -> TexSourceDependency {
    TexSourceDependency {
        payload,
        local_graphic_path: None,
        tex_like_only: true,
        obey_includeonly: false,
    }
}

fn tex_two_arg_source_dependency(source_ref: TexTwoArgPathRef) -> TexSourceDependency {
    let local_graphic_path = local_graphic_path_from_tex_directory(&source_ref.directory);
    TexSourceDependency {
        payload: source_ref.payload,
        local_graphic_path,
        tex_like_only: false,
        obey_includeonly: false,
    }
}

fn tex_two_arg_includeonly_source_dependency(source_ref: TexTwoArgPathRef) -> TexSourceDependency {
    let local_graphic_path = local_graphic_path_from_tex_directory(&source_ref.directory);
    TexSourceDependency {
        payload: source_ref.payload,
        local_graphic_path,
        tex_like_only: false,
        obey_includeonly: true,
    }
}

fn local_graphic_path_from_tex_directory(directory: &str) -> Option<PathBuf> {
    let path = Path::new(directory);
    if safe_relative_path(path) {
        Some(path.to_path_buf())
    } else {
        None
    }
}

fn tex_two_arg_path_refs_stripped(source: &str, command: &str) -> Vec<TexTwoArgPathRef> {
    let command = format!(r"\{command}");
    let mut refs = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find(&command) {
        let command_start = cursor + offset;
        let after_command = command_start + command.len();
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let first_start = skip_tex_whitespace(source, after_command);
        let Some((directory, first_end)) = balanced_braced_payload_at(source, first_start) else {
            cursor = after_command;
            continue;
        };
        let second_start = skip_tex_whitespace(source, first_end);
        let Some((file, second_end)) = balanced_braced_payload_at(source, second_start) else {
            cursor = first_end;
            continue;
        };
        if let Some(payload) = join_tex_path_payloads(&directory, &file) {
            refs.push(TexTwoArgPathRef {
                directory: directory.trim().to_string(),
                payload,
            });
        }
        cursor = second_end;
    }
    refs
}

fn join_tex_path_payloads(directory: &str, file: &str) -> Option<String> {
    let directory = directory.trim();
    let file = file.trim();
    if file.is_empty() {
        return None;
    }
    if directory.is_empty() {
        return Some(file.to_string());
    }
    let separator = if directory.ends_with('/') || file.starts_with('/') {
        ""
    } else {
        "/"
    };
    Some(format!("{directory}{separator}{file}"))
}

fn dedup_source_dependencies(dependencies: Vec<TexSourceDependency>) -> Vec<TexSourceDependency> {
    let mut seen = HashSet::new();
    dependencies
        .into_iter()
        .filter(|dependency| seen.insert(dependency.clone()))
        .collect()
}

#[cfg(test)]
fn dedup_payloads(payloads: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    payloads
        .into_iter()
        .filter(|payload| seen.insert(payload.clone()))
        .collect()
}

#[cfg(test)]
fn tex_input_payloads(source: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    tex_input_payloads_stripped(&source)
}

fn tex_input_payloads_stripped(source: &str) -> Vec<String> {
    let command = r"\input";
    let mut payloads = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find(command) {
        let command_start = cursor + offset;
        let after_command = command_start + command.len();
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let payload_start = skip_tex_whitespace(source, after_command);
        if source[payload_start..].starts_with('{') {
            let Some((payload, payload_end)) = balanced_braced_payload_at(source, payload_start)
            else {
                break;
            };
            if !payload.is_empty() {
                payloads.push(payload);
            }
            cursor = payload_end;
            continue;
        }

        let Some((payload, payload_end)) = unbraced_input_payload(source, payload_start) else {
            cursor = after_command;
            continue;
        };
        payloads.push(payload.to_string());
        cursor = payload_end;
    }
    payloads
}

fn skip_tex_whitespace(line: &str, mut cursor: usize) -> usize {
    while line[cursor..]
        .chars()
        .next()
        .is_some_and(|ch| ch.is_whitespace())
    {
        cursor += line[cursor..]
            .chars()
            .next()
            .map(char::len_utf8)
            .unwrap_or(1);
    }
    cursor
}

fn unbraced_input_payload(line: &str, start: usize) -> Option<(&str, usize)> {
    let mut end = start;
    for (offset, ch) in line[start..].char_indices() {
        if ch.is_whitespace() || matches!(ch, '\\' | '{' | '}') {
            break;
        }
        end = start + offset + ch.len_utf8();
    }
    if end == start {
        return None;
    }
    Some((line[start..end].trim(), end))
}

fn strip_tex_comment(line: &str) -> &str {
    let comment_start = tex_comment_start_bytes(line.as_bytes()).unwrap_or(line.len());
    &line[..comment_start]
}

fn resolve_local_tex_source_dependency(
    doc_dir: &Path,
    dependency: &TexSourceDependency,
) -> Result<Option<PathBuf>> {
    if dependency.tex_like_only {
        resolve_local_tex_like_source(doc_dir, &dependency.payload)
    } else {
        resolve_local_tex_source(doc_dir, &dependency.payload)
    }
}

fn resolve_local_tex_source(doc_dir: &Path, payload: &str) -> Result<Option<PathBuf>> {
    let Some(path) = safe_relative_payload_path(payload) else {
        return Ok(None);
    };
    let candidate = doc_dir.join(path);
    if candidate.is_file() {
        return Ok(Some(candidate));
    }
    if candidate.extension().is_none() {
        let tex_candidate = candidate.with_extension("tex");
        if tex_candidate.is_file() {
            return Ok(Some(tex_candidate));
        }
        return resolve_kpathsea_input(doc_dir, payload, "tex");
    }
    let extension = candidate
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    resolve_kpathsea_input(doc_dir, payload, &extension)
}

fn resolve_local_tex_like_source(doc_dir: &Path, payload: &str) -> Result<Option<PathBuf>> {
    let Some(path) = safe_relative_payload_path(payload) else {
        return Ok(None);
    };
    let candidate = doc_dir.join(path);
    if candidate.extension().is_none() {
        let tex_candidate = candidate.with_extension("tex");
        if tex_candidate.is_file() {
            return Ok(Some(tex_candidate));
        }
        return resolve_kpathsea_input(doc_dir, payload, "tex");
    }
    if is_tex_source_extension(&candidate) && candidate.is_file() {
        Ok(Some(candidate))
    } else if is_tex_source_extension(&candidate) {
        let extension = candidate
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        resolve_kpathsea_input(doc_dir, payload, &extension)
    } else {
        Ok(None)
    }
}

fn safe_relative_payload_path(payload: &str) -> Option<&Path> {
    let path = Path::new(payload);
    if path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        return None;
    }
    Some(path)
}

fn is_tex_source_extension(path: &Path) -> bool {
    path_extension_is_any(path, &["tex", "ltx", "sty", "cls", "dtx"])
}

fn path_extension_is_any(path: &Path, extensions: &[&str]) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extensions
                .iter()
                .any(|candidate| extension.eq_ignore_ascii_case(candidate))
        })
}

fn ends_with_ignore_ascii_case(value: &str, suffix: &str) -> bool {
    let bytes = value.as_bytes();
    let suffix = suffix.as_bytes();
    bytes.len() >= suffix.len() && bytes[bytes.len() - suffix.len()..].eq_ignore_ascii_case(suffix)
}

fn safe_relative_parent(path: &Path) -> Option<&Path> {
    if path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        return None;
    }
    path.parent()
        .filter(|parent| !parent.as_os_str().is_empty())
}

fn run_aux_tools_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    main: &Path,
    options: &BuildOptions,
    aux_session_cache: &AuxToolSessionCache,
) -> Result<(usize, usize, usize)> {
    thread::scope(|scope| {
        let bibliography = scope.spawn(move || {
            run_bibliographies_if_needed(doc_dir, out_dir, job_name, options, aux_session_cache)
        });
        let indexes =
            scope.spawn(move || run_indexes_if_needed(doc_dir, out_dir, job_name, options));
        let external = scope.spawn(move || {
            run_external_tools_if_needed(
                doc_dir,
                out_dir,
                job_name,
                main,
                options,
                aux_session_cache,
            )
        });

        let bibliography_runs = bibliography
            .join()
            .map_err(|_| anyhow!("bibliography worker panicked"))?;
        let index_runs = indexes
            .join()
            .map_err(|_| anyhow!("index worker panicked"))?;
        let external_runs = external
            .join()
            .map_err(|_| anyhow!("external-tool worker panicked"))?;

        Ok((bibliography_runs?, index_runs?, external_runs?))
    })
}

fn run_external_tools_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    main: &Path,
    options: &BuildOptions,
    aux_session_cache: &AuxToolSessionCache,
) -> Result<usize> {
    thread::scope(|scope| {
        let source = scope.spawn(move || {
            run_source_external_tools_if_needed_once(
                doc_dir,
                out_dir,
                main,
                options,
                aux_session_cache,
            )
        });
        let asymptote =
            scope.spawn(move || run_asymptote_if_needed(doc_dir, out_dir, job_name, options));
        let pythontex =
            scope.spawn(move || run_pythontex_if_needed(doc_dir, out_dir, job_name, options));
        let metapost =
            scope.spawn(move || run_metapost_if_needed(doc_dir, out_dir, job_name, options));
        let gnuplottex =
            scope.spawn(move || run_gnuplottex_if_needed(doc_dir, out_dir, job_name, options));
        let pgf_external =
            scope.spawn(move || run_pgf_external_if_needed(doc_dir, out_dir, job_name, options));
        let bib2gls =
            scope.spawn(move || run_bib2gls_if_needed(doc_dir, out_dir, job_name, options));

        let source_runs = source
            .join()
            .map_err(|_| anyhow!("source external-tool worker panicked"))?;
        let asymptote_runs = asymptote
            .join()
            .map_err(|_| anyhow!("Asymptote worker panicked"))?;
        let pythontex_runs = pythontex
            .join()
            .map_err(|_| anyhow!("PythonTeX worker panicked"))?;
        let metapost_runs = metapost
            .join()
            .map_err(|_| anyhow!("MetaPost worker panicked"))?;
        let gnuplottex_runs = gnuplottex
            .join()
            .map_err(|_| anyhow!("gnuplottex worker panicked"))?;
        let pgf_external_runs = pgf_external
            .join()
            .map_err(|_| anyhow!("PGF externalization worker panicked"))?;
        let bib2gls_runs = bib2gls
            .join()
            .map_err(|_| anyhow!("Bib2Gls worker panicked"))?;

        Ok(source_runs?
            + asymptote_runs?
            + pythontex_runs?
            + metapost_runs?
            + gnuplottex_runs?
            + pgf_external_runs?
            + bib2gls_runs?)
    })
}

fn run_source_external_tools_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    options: &BuildOptions,
    aux_session_cache: &AuxToolSessionCache,
) -> Result<usize> {
    let jobs = source_external_tool_jobs(doc_dir, out_dir, main, options, aux_session_cache)?;
    let eps_runs = run_eps_conversion_jobs_if_needed(&jobs.eps, options)?;
    let svg_runs = run_svg_conversion_jobs_if_needed(&jobs.svg, options)?;
    Ok(eps_runs + svg_runs)
}

fn run_source_external_tools_if_needed_once(
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    options: &BuildOptions,
    aux_session_cache: &AuxToolSessionCache,
) -> Result<usize> {
    if aux_session_cache.source_external_tools_were_checked() {
        return Ok(0);
    }
    let runs =
        run_source_external_tools_if_needed(doc_dir, out_dir, main, options, aux_session_cache)?;
    aux_session_cache.mark_source_external_tools_checked();
    Ok(runs)
}

fn source_external_tool_jobs(
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    options: &BuildOptions,
    aux_session_cache: &AuxToolSessionCache,
) -> Result<SourceExternalToolJobs> {
    if let Some(jobs) = aux_session_cache
        .source_external_tool_jobs
        .lock()
        .expect("source external-tool job cache mutex poisoned")
        .clone()
    {
        return Ok(jobs);
    }

    let source_cache = aux_session_cache.source_read_cache();
    let includeonly = aux_session_cache.root_includeonly_filter(main)?;
    let context = SourceConversionContext {
        doc_dir,
        out_dir,
        includeonly: includeonly.as_ref(),
        source_cache,
    };
    let jobs = SourceExternalToolJobs {
        eps: if !options.fast && uses_pdftex_graphics_pipeline(options.engine) {
            eps_conversion_jobs_with_context(context, main)?
        } else {
            Vec::new()
        },
        svg: if !options.fast {
            svg_conversion_jobs_with_context(context, main)?
        } else {
            Vec::new()
        },
    };

    let mut cached = aux_session_cache
        .source_external_tool_jobs
        .lock()
        .expect("source external-tool job cache mutex poisoned");
    if let Some(cached) = cached.as_ref() {
        return Ok(cached.clone());
    }
    *cached = Some(jobs.clone());
    Ok(jobs)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum SourceBibtexEvent {
    Citation(String),
    BibData(String),
    BibStyle(String),
}

#[derive(Debug, Default)]
struct SourceBibtexPreflight {
    events: Vec<SourceBibtexEvent>,
    unsupported: bool,
}

#[derive(Debug, Clone, Default)]
struct SourceBibtexPreflightReport {
    bibliography_runs: usize,
    bibcite_seeded: bool,
}

fn run_source_bibtex_preflight_if_possible(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    main: &Path,
    includeonly: Option<&HashSet<String>>,
    options: &BuildOptions,
    source_cache: &TexSourceReadCache,
    aux_session_cache: &AuxToolSessionCache,
    compatible_build_state: bool,
) -> Result<SourceBibtexPreflightReport> {
    if options.once
        || options.fast
        || compatible_build_state
        || !matches!(options.bib_mode, BibMode::Auto | BibMode::BibTex)
    {
        return Ok(SourceBibtexPreflightReport::default());
    }

    let aux_path = out_dir.join(format!("{job_name}.aux"));
    if aux_path.exists() {
        return Ok(SourceBibtexPreflightReport::default());
    }

    let mut visited = HashSet::new();
    let mut preflight = SourceBibtexPreflight::default();
    collect_ordered_source_bibtex_preflight(
        doc_dir,
        main,
        includeonly,
        source_cache,
        &mut visited,
        &mut preflight,
        true,
    )?;
    if preflight.unsupported || !source_bibtex_preflight_is_runnable(&preflight.events) {
        return Ok(SourceBibtexPreflightReport::default());
    }

    let aux_source = source_bibtex_preflight_aux_source(&preflight.events);
    fs::write(&aux_path, aux_source).with_context(|| {
        format!(
            "failed to write source BibTeX preflight aux {}",
            aux_path.display()
        )
    })?;
    let job = bibtex_job(out_dir, &aux_path, None);
    let bibliography_runs = if run_bibtex_job_if_stale(
        doc_dir,
        out_dir,
        &aux_path,
        &job,
        options,
        aux_session_cache,
    )? {
        1
    } else {
        0
    };
    let bibcite_seeded =
        seed_natbib_bibcite_preflight_if_possible(&aux_path, &job.bbl_path, &preflight.events)?;
    Ok(SourceBibtexPreflightReport {
        bibliography_runs,
        bibcite_seeded,
    })
}

fn source_bibtex_preflight_is_runnable(events: &[SourceBibtexEvent]) -> bool {
    events
        .iter()
        .any(|event| matches!(event, SourceBibtexEvent::Citation(_)))
        && events
            .iter()
            .any(|event| matches!(event, SourceBibtexEvent::BibData(_)))
        && events
            .iter()
            .any(|event| matches!(event, SourceBibtexEvent::BibStyle(_)))
}

fn source_bibtex_preflight_aux_source(events: &[SourceBibtexEvent]) -> String {
    let mut source = String::from("\\relax\n");
    for event in events {
        match event {
            SourceBibtexEvent::Citation(payload) => {
                let _ = writeln!(&mut source, "\\citation{{{payload}}}");
            }
            SourceBibtexEvent::BibData(payload) => {
                let _ = writeln!(&mut source, "\\bibdata{{{payload}}}");
            }
            SourceBibtexEvent::BibStyle(payload) => {
                let _ = writeln!(&mut source, "\\bibstyle{{{payload}}}");
            }
        }
    }
    source
}

fn seed_natbib_bibcite_preflight_if_possible(
    aux_path: &Path,
    bbl_path: &Path,
    events: &[SourceBibtexEvent],
) -> Result<bool> {
    if !source_bibtex_preflight_uses_natbib_style(events) || !bbl_path.exists() {
        return Ok(false);
    }
    let aux_source = fs::read_to_string(aux_path)
        .with_context(|| format!("failed to read aux file {}", aux_path.display()))?;
    if aux_source.contains(r"\bibcite") {
        return Ok(false);
    }
    let Some(citation_keys) = bibtex_citation_keys(&aux_source) else {
        return Ok(false);
    };
    if citation_keys.is_empty() {
        return Ok(false);
    }

    let bbl_source = fs::read_to_string(bbl_path)
        .with_context(|| format!("failed to read bibliography file {}", bbl_path.display()))?;
    let seeds = natbib_bibcite_seeds_from_bbl(&bbl_source);
    if seeds.is_empty() {
        return Ok(false);
    }
    let seed_keys = seeds
        .iter()
        .map(|seed| seed.key.as_str())
        .collect::<HashSet<_>>();
    if citation_keys
        .iter()
        .any(|citation| !seed_keys.contains(citation.as_str()))
    {
        return Ok(false);
    }

    let mut seeded_aux = aux_source;
    if !seeded_aux.ends_with('\n') {
        seeded_aux.push('\n');
    }
    for seed in seeds {
        seeded_aux.push_str(&seed.line);
        seeded_aux.push('\n');
    }
    fs::write(aux_path, seeded_aux).with_context(|| {
        format!(
            "failed to write source BibTeX preflight aux {}",
            aux_path.display()
        )
    })?;
    Ok(true)
}

fn source_bibtex_preflight_uses_natbib_style(events: &[SourceBibtexEvent]) -> bool {
    events.iter().any(|event| {
        let SourceBibtexEvent::BibStyle(style) = event else {
            return false;
        };
        style
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or(style)
            .trim_end_matches(".bst")
            .to_ascii_lowercase()
            .ends_with("nat")
    })
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct NatbibBibciteSeed {
    key: String,
    line: String,
}

fn natbib_bibcite_seeds_from_bbl(source: &str) -> Vec<NatbibBibciteSeed> {
    let mut seeds = Vec::new();
    let mut cursor = 0;
    let mut number = 1_usize;
    while let Some(offset) = source[cursor..].find(r"\bibitem") {
        let item_start = cursor + offset;
        let mut after_command = item_start + r"\bibitem".len();
        after_command = skip_tex_whitespace(source, after_command);
        let Some((label, after_label)) = bracketed_tex_argument_payload_at(source, after_command)
        else {
            return Vec::new();
        };
        let after_label = skip_tex_whitespace(source, after_label);
        let Some((key, end)) = balanced_braced_payload_at(source, after_label) else {
            return Vec::new();
        };
        let key = key.trim();
        if key.is_empty() {
            return Vec::new();
        }
        let Some((short_author, year, long_author)) = parse_natbib_bibitem_label(&label) else {
            return Vec::new();
        };
        seeds.push(NatbibBibciteSeed {
            key: key.to_string(),
            line: natbib_bibcite_seed_line(key, number, &short_author, &year, &long_author),
        });
        number += 1;
        cursor = end;
    }
    seeds
}

fn parse_natbib_bibitem_label(label: &str) -> Option<(String, String, String)> {
    let open = label.find('(')?;
    let close = label[open + 1..].find(')')? + open + 1;
    let short_author = label[..open].trim();
    let year = label[open + 1..close].trim();
    let long_author = label[close + 1..].trim();
    if short_author.is_empty() || year.is_empty() {
        return None;
    }
    Some((
        short_author.to_string(),
        year.to_string(),
        long_author.to_string(),
    ))
}

fn natbib_bibcite_seed_line(
    key: &str,
    number: usize,
    short_author: &str,
    year: &str,
    long_author: &str,
) -> String {
    let mut line = String::new();
    line.push_str(r"\bibcite{");
    line.push_str(key);
    line.push_str("}{");
    line.push('{');
    let _ = write!(&mut line, "{number}");
    line.push('}');
    line.push('{');
    line.push_str(year);
    line.push('}');
    line.push_str("{{");
    line.push_str(short_author);
    line.push_str("}}{{");
    line.push_str(long_author);
    line.push_str("}}}");
    line
}

fn collect_ordered_source_bibtex_preflight(
    doc_dir: &Path,
    source_path: &Path,
    includeonly: Option<&HashSet<String>>,
    source_cache: &TexSourceReadCache,
    visited: &mut HashSet<PathBuf>,
    preflight: &mut SourceBibtexPreflight,
    is_root: bool,
) -> Result<()> {
    let source_path = canonical_tex_source_path(source_path)?;
    if !visited.insert(source_path.clone()) {
        return Ok(());
    }
    let source = read_cached_tex_source(source_cache, &source_path)?;
    for event in ordered_source_bibtex_events(&source) {
        match event {
            OrderedSourceBibtexEvent::Citation(payload) => {
                preflight.events.push(SourceBibtexEvent::Citation(payload));
            }
            OrderedSourceBibtexEvent::BibData(payload) => {
                if !is_root {
                    preflight.unsupported = true;
                }
                preflight.events.push(SourceBibtexEvent::BibData(payload));
            }
            OrderedSourceBibtexEvent::BibStyle(payload) => {
                if !is_root {
                    preflight.unsupported = true;
                }
                preflight.events.push(SourceBibtexEvent::BibStyle(payload));
            }
            OrderedSourceBibtexEvent::Input(dependency) => {
                if !include_dependency_is_active(&dependency, includeonly) {
                    continue;
                }
                if let Some(path) = resolve_local_tex_source_dependency(doc_dir, &dependency)? {
                    collect_ordered_source_bibtex_preflight(
                        doc_dir,
                        &path,
                        includeonly,
                        source_cache,
                        visited,
                        preflight,
                        false,
                    )?;
                }
            }
            OrderedSourceBibtexEvent::Unsupported => {
                preflight.unsupported = true;
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum OrderedSourceBibtexEvent {
    Citation(String),
    BibData(String),
    BibStyle(String),
    Input(TexSourceDependency),
    Unsupported,
}

fn ordered_source_bibtex_events(source: &str) -> Vec<OrderedSourceBibtexEvent> {
    let source = tex_comment_stripped_source(source);
    let mut events = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find('\\') {
        let command_start = cursor + offset;
        let name_start = command_start + 1;
        let mut name_end = name_start;
        while source[name_end..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            name_end += source[name_end..].chars().next().unwrap().len_utf8();
        }
        if name_end == name_start {
            cursor = name_start;
            continue;
        }
        let command = &source[name_start..name_end];
        let mut after_command = name_end;
        if source[after_command..].starts_with('*') {
            after_command += 1;
        }

        match command {
            "input" | "include" => {
                if let Some((payload, end)) =
                    tex_required_payload_after_optional_args(&source, after_command)
                {
                    let dependency = if command == "include" {
                        tex_includeonly_source_dependency(payload)
                    } else {
                        tex_source_dependency(payload)
                    };
                    events.push(OrderedSourceBibtexEvent::Input(dependency));
                    cursor = end;
                    continue;
                }
            }
            "bibliography" => {
                if let Some((payload, end)) =
                    tex_required_payload_after_optional_args(&source, after_command)
                {
                    if let Some(payload) = normalized_bibtex_comma_payload(&payload) {
                        events.push(OrderedSourceBibtexEvent::BibData(payload));
                    }
                    cursor = end;
                    continue;
                }
            }
            "bibliographystyle" => {
                if let Some((payload, end)) =
                    tex_required_payload_after_optional_args(&source, after_command)
                {
                    let payload = payload.trim();
                    if !payload.is_empty() {
                        events.push(OrderedSourceBibtexEvent::BibStyle(payload.to_string()));
                    }
                    cursor = end;
                    continue;
                }
            }
            "nocite" | "cite" | "citep" | "citet" | "citealp" | "citealt" | "citeauthor"
            | "citeyear" | "citeyearpar" | "Citep" | "Citet" => {
                if let Some((payload, end)) =
                    tex_required_payload_after_optional_args(&source, after_command)
                {
                    if let Some(payload) = normalized_bibtex_comma_payload(&payload) {
                        events.push(OrderedSourceBibtexEvent::Citation(payload));
                    }
                    cursor = end;
                    continue;
                }
            }
            "addbibresource" | "addglobalbib" | "addsectionbib" | "printbibliography" => {
                events.push(OrderedSourceBibtexEvent::Unsupported);
            }
            _ => {}
        }

        cursor = after_command;
    }
    if source_package_payloads_stripped(&source)
        .iter()
        .any(|payload| payload == "biblatex")
    {
        events.push(OrderedSourceBibtexEvent::Unsupported);
    }
    events
}

fn tex_required_payload_after_optional_args(source: &str, start: usize) -> Option<(String, usize)> {
    let mut cursor = skip_tex_whitespace(source, start);
    while source[cursor..].starts_with('[') {
        cursor = bracketed_tex_argument_end(source, cursor)?;
        cursor = skip_tex_whitespace(source, cursor);
    }
    let (payload, end) = balanced_braced_payload_at(source, cursor)?;
    Some((payload, end))
}

fn normalized_bibtex_comma_payload(payload: &str) -> Option<String> {
    let entries = split_tex_top_level(payload, ',')
        .into_iter()
        .map(|entry| entry.split_whitespace().collect::<String>())
        .filter(|entry| !entry.is_empty())
        .collect::<Vec<_>>();
    (!entries.is_empty()).then(|| entries.join(","))
}

fn run_bibliographies_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
    aux_session_cache: &AuxToolSessionCache,
) -> Result<usize> {
    match options.bib_mode {
        BibMode::None => return Ok(0),
        BibMode::Biber => return run_biber_if_stale(doc_dir, out_dir, job_name, options),
        BibMode::BibTex => {
            return run_bibtex_jobs_if_stale(
                doc_dir,
                out_dir,
                job_name,
                options,
                aux_session_cache,
            );
        }
        BibMode::Auto => {}
    }

    let bibtex_jobs = bibtex_jobs_for_run(doc_dir, out_dir, job_name)?;
    let has_biber_control_file =
        biber_control_file_from_latest_run(doc_dir, out_dir, job_name)?.is_some();
    if !bibtex_jobs.is_empty() {
        if has_biber_control_file {
            ensure_bibliography_outputs_are_disjoint(&bibtex_jobs, out_dir, job_name)?;
            return thread::scope(|scope| {
                let bibtex = scope.spawn(move || {
                    run_bibtex_jobs_if_stale_for_jobs(
                        doc_dir,
                        out_dir,
                        &bibtex_jobs,
                        options,
                        aux_session_cache,
                    )
                });
                let biber =
                    scope.spawn(move || run_biber_if_stale(doc_dir, out_dir, job_name, options));

                let bibtex_runs = bibtex
                    .join()
                    .map_err(|_| anyhow!("BibTeX worker panicked"))?;
                let biber_runs = biber.join().map_err(|_| anyhow!("Biber worker panicked"))?;
                Ok(bibtex_runs? + biber_runs?)
            });
        }
        return run_bibtex_jobs_if_stale_for_jobs(
            doc_dir,
            out_dir,
            &bibtex_jobs,
            options,
            aux_session_cache,
        );
    }

    if has_biber_control_file {
        return run_biber_if_stale(doc_dir, out_dir, job_name, options);
    }

    Ok(0)
}

fn ensure_bibliography_outputs_are_disjoint(
    bibtex_jobs: &[(PathBuf, BibtexJob)],
    out_dir: &Path,
    job_name: &str,
) -> Result<()> {
    let biber_bbl_path = out_dir.join(format!("{job_name}.bbl"));
    for (_, job) in bibtex_jobs {
        if output_paths_conflict(&job.bbl_path, &biber_bbl_path) {
            bail!(
                "auto bibliography mode found both BibTeX and Biber jobs writing {}; choose --bib bibtex or --bib biber to resolve the conflict",
                biber_bbl_path.display()
            );
        }
    }
    Ok(())
}

fn output_paths_conflict(left: &Path, right: &Path) -> bool {
    canonical_or_original(left) == canonical_or_original(right)
}

fn run_bibtex_jobs_if_stale(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
    aux_session_cache: &AuxToolSessionCache,
) -> Result<usize> {
    let jobs = bibtex_jobs_for_run(doc_dir, out_dir, job_name)?;
    run_bibtex_jobs_if_stale_for_jobs(doc_dir, out_dir, &jobs, options, aux_session_cache)
}

fn run_bibtex_jobs_if_stale_for_jobs(
    doc_dir: &Path,
    out_dir: &Path,
    jobs: &[(PathBuf, BibtexJob)],
    options: &BuildOptions,
    aux_session_cache: &AuxToolSessionCache,
) -> Result<usize> {
    if jobs.len() <= 1 {
        let mut runs = 0;
        for (aux_path, job) in jobs {
            if run_bibtex_job_if_stale(doc_dir, out_dir, aux_path, job, options, aux_session_cache)?
            {
                runs += 1;
            }
        }
        return Ok(runs);
    }

    thread::scope(|scope| {
        let mut handles = Vec::new();
        for (aux_path, job) in jobs {
            handles.push(scope.spawn(move || {
                run_bibtex_job_if_stale(doc_dir, out_dir, aux_path, job, options, aux_session_cache)
            }));
        }

        let mut runs = 0;
        for handle in handles {
            if handle
                .join()
                .map_err(|_| anyhow!("BibTeX worker panicked"))??
            {
                runs += 1;
            }
        }
        Ok(runs)
    })
}

fn run_makeindex_jobs_if_stale(jobs: &[MakeIndexJob], options: &BuildOptions) -> Result<usize> {
    if jobs.len() <= 1 {
        let mut runs = 0;
        for job in jobs {
            if run_makeindex_if_stale(job, options)? {
                runs += 1;
            }
        }
        return Ok(runs);
    }

    thread::scope(|scope| {
        let mut handles = Vec::new();
        for job in jobs {
            handles.push(scope.spawn(move || run_makeindex_if_stale(job, options)));
        }

        let mut runs = 0;
        for handle in handles {
            if handle
                .join()
                .map_err(|_| anyhow!("MakeIndex worker panicked"))??
            {
                runs += 1;
            }
        }
        Ok(runs)
    })
}

fn run_splitindex_jobs_if_stale(jobs: &[SplitIndexJob], options: &BuildOptions) -> Result<usize> {
    if jobs.len() <= 1 {
        let mut runs = 0;
        for job in jobs {
            if run_splitindex_if_stale(job, options)? {
                runs += 1;
            }
        }
        return Ok(runs);
    }

    thread::scope(|scope| {
        let mut handles = Vec::new();
        for job in jobs {
            handles.push(scope.spawn(move || run_splitindex_if_stale(job, options)));
        }

        let mut runs = 0;
        for handle in handles {
            if handle
                .join()
                .map_err(|_| anyhow!("SplitIndex worker panicked"))??
            {
                runs += 1;
            }
        }
        Ok(runs)
    })
}

fn run_asymptote_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
) -> Result<usize> {
    let jobs = asymptote_jobs_from_latest_run(doc_dir, out_dir, job_name)?;
    if jobs.len() <= 1 {
        let mut runs = 0;
        for job in jobs {
            if run_asymptote_if_stale(&job, options)? {
                runs += 1;
            }
        }
        return Ok(runs);
    }

    thread::scope(|scope| {
        let mut handles = Vec::new();
        for job in &jobs {
            handles.push(scope.spawn(move || run_asymptote_if_stale(job, options)));
        }

        let mut runs = 0;
        for handle in handles {
            if handle
                .join()
                .map_err(|_| anyhow!("Asymptote worker panicked"))??
            {
                runs += 1;
            }
        }
        Ok(runs)
    })
}

fn run_asymptote_if_stale(job: &AsymptoteJob, options: &BuildOptions) -> Result<bool> {
    let signature = asymptote_signature(job)?;
    if !options.force
        && external_tool_cache_is_fresh(&job.state_path, &signature, &job.output_path)?
    {
        return Ok(false);
    }

    run_asymptote(job, options)?;
    write_asymptote_state(job, &signature)?;
    Ok(true)
}

fn run_asymptote(job: &AsymptoteJob, options: &BuildOptions) -> Result<()> {
    let asy_dir = job
        .input_path
        .parent()
        .context("Asymptote file has no parent directory")?;
    let input_name = job
        .input_path
        .file_name()
        .context("Asymptote file has no filename")?;
    let mut command = Command::new("asy");
    command.current_dir(asy_dir).arg(input_name);
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch Asymptote")?;
    if !status.success() {
        bail!("Asymptote failed with status {status}");
    }
    Ok(())
}

fn run_pythontex_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
) -> Result<usize> {
    let jobs = pythontex_jobs_from_latest_run(doc_dir, out_dir, job_name)?;
    if jobs.len() <= 1 {
        let mut runs = 0;
        for job in jobs {
            if run_pythontex_if_stale(&job, options)? {
                runs += 1;
            }
        }
        return Ok(runs);
    }

    thread::scope(|scope| {
        let mut handles = Vec::new();
        for job in &jobs {
            handles.push(scope.spawn(move || run_pythontex_if_stale(job, options)));
        }

        let mut runs = 0;
        for handle in handles {
            if handle
                .join()
                .map_err(|_| anyhow!("PythonTeX worker panicked"))??
            {
                runs += 1;
            }
        }
        Ok(runs)
    })
}

fn run_pythontex_if_stale(job: &PythontexJob, options: &BuildOptions) -> Result<bool> {
    let signature = pythontex_signature(job)?;
    if !options.force
        && external_tool_cache_is_fresh_for_outputs(&job.state_path, &signature, &job.output_paths)?
    {
        return Ok(false);
    }

    run_pythontex(job, options)?;
    write_pythontex_state(job, &signature)?;
    Ok(true)
}

fn run_pythontex(job: &PythontexJob, options: &BuildOptions) -> Result<()> {
    let code_dir = job
        .code_path
        .parent()
        .context("PythonTeX code file has no parent directory")?;
    let mut command = Command::new("pythontex");
    command.current_dir(code_dir).arg(&job.command_arg);
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch PythonTeX")?;
    if !status.success() {
        bail!("PythonTeX failed with status {status}");
    }
    Ok(())
}

fn run_metapost_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
) -> Result<usize> {
    let jobs = metapost_jobs_from_latest_run(doc_dir, out_dir, job_name)?;
    if jobs.len() <= 1 {
        let mut runs = 0;
        for job in jobs {
            if run_metapost_if_stale(&job, options)? {
                runs += 1;
            }
        }
        return Ok(runs);
    }

    thread::scope(|scope| {
        let mut handles = Vec::new();
        for job in &jobs {
            handles.push(scope.spawn(move || run_metapost_if_stale(job, options)));
        }

        let mut runs = 0;
        for handle in handles {
            if handle
                .join()
                .map_err(|_| anyhow!("MetaPost worker panicked"))??
            {
                runs += 1;
            }
        }
        Ok(runs)
    })
}

fn run_metapost_if_stale(job: &MetapostJob, options: &BuildOptions) -> Result<bool> {
    let signature = metapost_signature(job)?;
    if !options.force
        && external_tool_cache_is_fresh_for_outputs(&job.state_path, &signature, &job.output_paths)?
    {
        return Ok(false);
    }

    run_metapost(job, options)?;
    write_metapost_state(job, &signature)?;
    Ok(true)
}

fn run_metapost(job: &MetapostJob, options: &BuildOptions) -> Result<()> {
    let mp_dir = job
        .input_path
        .parent()
        .context("MetaPost file has no parent directory")?;
    let input_name = job
        .input_path
        .file_name()
        .context("MetaPost file has no filename")?;
    let mut command = Command::new("mpost");
    command.current_dir(mp_dir).arg(input_name);
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch MetaPost")?;
    if !status.success() {
        bail!("MetaPost failed with status {status}");
    }
    Ok(())
}

fn run_gnuplottex_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
) -> Result<usize> {
    let jobs = gnuplottex_jobs_from_latest_run(doc_dir, out_dir, job_name)?;
    if jobs.len() <= 1 {
        let mut runs = 0;
        for job in jobs {
            if run_gnuplottex_if_stale(&job, out_dir, options)? {
                runs += 1;
            }
        }
        return Ok(runs);
    }

    thread::scope(|scope| {
        let mut handles = Vec::new();
        for job in &jobs {
            handles.push(scope.spawn(move || run_gnuplottex_if_stale(job, out_dir, options)));
        }

        let mut runs = 0;
        for handle in handles {
            if handle
                .join()
                .map_err(|_| anyhow!("gnuplottex worker panicked"))??
            {
                runs += 1;
            }
        }
        Ok(runs)
    })
}

fn run_gnuplottex_if_stale(
    job: &GnuplottexJob,
    out_dir: &Path,
    options: &BuildOptions,
) -> Result<bool> {
    let signature = gnuplottex_signature(job)?;
    if !options.force
        && external_tool_cache_is_fresh(&job.state_path, &signature, &job.output_path)?
    {
        return Ok(false);
    }

    run_gnuplottex(job, out_dir, options)?;
    write_gnuplottex_state(job, &signature)?;
    Ok(true)
}

fn run_gnuplottex(job: &GnuplottexJob, out_dir: &Path, options: &BuildOptions) -> Result<()> {
    if let Some(parent) = job.output_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create gnuplottex output directory {}",
                parent.display()
            )
        })?;
    }
    let script_arg = job
        .script_path
        .strip_prefix(out_dir)
        .unwrap_or(&job.script_path);
    let mut command = Command::new("gnuplot");
    command.current_dir(out_dir).arg(script_arg);
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch gnuplot")?;
    if !status.success() {
        bail!("gnuplot failed with status {status}");
    }
    if !job.output_path.exists() {
        bail!(
            "gnuplot did not create expected output {}",
            job.output_path.display()
        );
    }
    Ok(())
}

fn run_pgf_external_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
) -> Result<usize> {
    let Some(job) = pgf_external_job_from_latest_run(out_dir, job_name)? else {
        return Ok(0);
    };
    if run_pgf_external_if_stale(doc_dir, out_dir, &job, options)? {
        Ok(1)
    } else {
        Ok(0)
    }
}

fn run_pgf_external_if_stale(
    doc_dir: &Path,
    out_dir: &Path,
    job: &PgfExternalJob,
    options: &BuildOptions,
) -> Result<bool> {
    match pgf_external_make_status(doc_dir, out_dir, job, options)? {
        MakeStatus::UpToDate => Ok(false),
        MakeStatus::NeedsRun => {
            run_pgf_external_make(doc_dir, out_dir, job, options)?;
            Ok(true)
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MakeStatus {
    UpToDate,
    NeedsRun,
}

fn pgf_external_make_status(
    doc_dir: &Path,
    out_dir: &Path,
    job: &PgfExternalJob,
    options: &BuildOptions,
) -> Result<MakeStatus> {
    let mut command = Command::new("make");
    command
        .current_dir(out_dir)
        .env("TEXINPUTS", texinputs_env(doc_dir, out_dir))
        .arg("-q")
        .arg("-f")
        .arg(
            job.makefile_path
                .file_name()
                .context("PGF externalization makefile has no filename")?,
        );
    for target in &job.make_targets {
        command.arg(target);
    }
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command
        .status()
        .context("failed to launch make for PGF externalization")?;
    match status.code() {
        Some(0) => Ok(MakeStatus::UpToDate),
        Some(1) => Ok(MakeStatus::NeedsRun),
        _ => bail!("PGF externalization make freshness check failed with status {status}"),
    }
}

fn run_pgf_external_make(
    doc_dir: &Path,
    out_dir: &Path,
    job: &PgfExternalJob,
    options: &BuildOptions,
) -> Result<()> {
    let mut command = Command::new("make");
    command
        .current_dir(out_dir)
        .env("TEXINPUTS", texinputs_env(doc_dir, out_dir))
        .args(pgf_external_make_parallel_args(job.make_targets.len()))
        .arg("-f")
        .arg(
            job.makefile_path
                .file_name()
                .context("PGF externalization makefile has no filename")?,
        );
    for target in &job.make_targets {
        command.arg(target);
    }
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command
        .status()
        .context("failed to launch make for PGF externalization")?;
    if !status.success() {
        bail!("PGF externalization make failed with status {status}");
    }
    Ok(())
}

fn pgf_external_make_parallel_args(target_count: usize) -> Vec<OsString> {
    if target_count <= 1 {
        return Vec::new();
    }
    let jobs = thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
        .min(target_count);
    if jobs <= 1 {
        Vec::new()
    } else {
        vec![OsString::from("-j"), OsString::from(jobs.to_string())]
    }
}

fn run_bib2gls_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
) -> Result<usize> {
    let Some(job) = bib2gls_job_from_latest_run(doc_dir, out_dir, job_name)? else {
        return Ok(0);
    };
    if run_bib2gls_if_stale(&job, options)? {
        Ok(1)
    } else {
        Ok(0)
    }
}

fn run_bib2gls_if_stale(job: &Bib2GlsJob, options: &BuildOptions) -> Result<bool> {
    let signature = bib2gls_signature(job)?;
    if !options.force
        && external_tool_cache_is_fresh_for_outputs(&job.state_path, &signature, &job.output_paths)?
    {
        return Ok(false);
    }

    run_bib2gls(job, options)?;
    write_bib2gls_state(job, &signature)?;
    Ok(true)
}

fn run_bib2gls(job: &Bib2GlsJob, options: &BuildOptions) -> Result<()> {
    let aux_dir = job
        .aux_path
        .parent()
        .context("Bib2Gls aux file has no parent directory")?;
    let mut command = Command::new("bib2gls");
    command
        .current_dir(aux_dir)
        .arg(&job.command_arg)
        .env("BIBINPUTS", kpathsea_env("BIBINPUTS", &job.doc_dir))
        .env("BSTINPUTS", kpathsea_env("BSTINPUTS", &job.doc_dir))
        .env("TEXINPUTS", kpathsea_env("TEXINPUTS", &job.doc_dir));
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch Bib2Gls")?;
    if !status.success() {
        bail!("Bib2Gls failed with status {status}");
    }
    Ok(())
}

fn run_eps_conversion_jobs_if_needed(
    jobs: &[EpsConversionJob],
    options: &BuildOptions,
) -> Result<usize> {
    if jobs.len() <= 1 {
        let mut runs = 0;
        for job in jobs {
            if run_eps_conversion_if_stale(job, options)? {
                runs += 1;
            }
        }
        return Ok(runs);
    }

    thread::scope(|scope| {
        let mut handles = Vec::new();
        for job in jobs {
            handles.push(scope.spawn(move || run_eps_conversion_if_stale(job, options)));
        }

        let mut runs = 0;
        for handle in handles {
            if handle
                .join()
                .map_err(|_| anyhow!("EPS conversion worker panicked"))??
            {
                runs += 1;
            }
        }
        Ok(runs)
    })
}

fn run_eps_conversion_if_stale(job: &EpsConversionJob, options: &BuildOptions) -> Result<bool> {
    let signature = eps_conversion_signature(job)?;
    if !options.force
        && external_tool_cache_is_fresh(&job.state_path, &signature, &job.output_path)?
    {
        return Ok(false);
    }

    run_eps_conversion(job, options)?;
    write_eps_conversion_state(job, &signature)?;
    Ok(true)
}

fn run_eps_conversion(job: &EpsConversionJob, options: &BuildOptions) -> Result<()> {
    if let Some(parent) = job.output_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create EPS conversion output directory {}",
                parent.display()
            )
        })?;
    }
    let mut command = Command::new("epstopdf");
    command
        .arg(&job.input_path)
        .arg(format!("--outfile={}", job.output_path.display()));
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch epstopdf")?;
    if !status.success() {
        bail!("epstopdf failed with status {status}");
    }
    Ok(())
}

fn run_svg_conversion_jobs_if_needed(
    jobs: &[SvgConversionJob],
    options: &BuildOptions,
) -> Result<usize> {
    if jobs.len() <= 1 {
        let mut runs = 0;
        for job in jobs {
            if run_svg_conversion_if_stale(job, options)? {
                runs += 1;
            }
        }
        return Ok(runs);
    }

    thread::scope(|scope| {
        let mut handles = Vec::new();
        for job in jobs {
            handles.push(scope.spawn(move || run_svg_conversion_if_stale(job, options)));
        }

        let mut runs = 0;
        for handle in handles {
            if handle
                .join()
                .map_err(|_| anyhow!("SVG conversion worker panicked"))??
            {
                runs += 1;
            }
        }
        Ok(runs)
    })
}

fn run_svg_conversion_if_stale(job: &SvgConversionJob, options: &BuildOptions) -> Result<bool> {
    let signature = svg_conversion_signature(job)?;
    if !options.force
        && external_tool_cache_is_fresh_for_outputs(
            &job.state_path,
            &signature,
            &job.output_paths(),
        )?
    {
        return Ok(false);
    }

    run_svg_conversion(job, options)?;
    write_svg_conversion_state(job, &signature)?;
    Ok(true)
}

fn run_svg_conversion(job: &SvgConversionJob, options: &BuildOptions) -> Result<()> {
    if let Some(parent) = job.output_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create SVG conversion output directory {}",
                parent.display()
            )
        })?;
    }
    let mut command = Command::new(&job.inkscape_executable);
    command.arg(&job.input_path);
    match job.area {
        SvgExportArea::Drawing => {
            command.arg("--export-area-drawing");
        }
        SvgExportArea::Page => {
            command.arg("--export-area-page");
        }
    }
    if let Some(dpi) = &job.dpi {
        command.arg(format!("--export-dpi={dpi}"));
    }
    if job.output_tex_path.is_some() {
        command.arg("--export-latex");
    }
    command.arg(format!("--export-filename={}", job.output_path.display()));
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch Inkscape")?;
    if !status.success() {
        bail!("Inkscape failed with status {status}");
    }
    for output_path in job.output_paths() {
        if !output_path.exists() {
            bail!(
                "Inkscape did not create expected SVG output {}",
                output_path.display()
            );
        }
    }
    Ok(())
}

impl SvgConversionJob {
    fn output_paths(&self) -> Vec<PathBuf> {
        let mut output_paths = vec![self.output_path.clone()];
        if let Some(output_tex_path) = &self.output_tex_path {
            output_paths.push(output_tex_path.clone());
        }
        output_paths
    }
}

fn run_bibtex_job_if_stale(
    doc_dir: &Path,
    out_dir: &Path,
    aux_path: &Path,
    job: &BibtexJob,
    options: &BuildOptions,
    aux_session_cache: &AuxToolSessionCache,
) -> Result<bool> {
    let source = fs::read_to_string(aux_path)
        .with_context(|| format!("failed to read aux file {}", aux_path.display()))?;
    if !source.contains(r"\bibdata") {
        return Ok(false);
    }

    let signature = bibtex_aux_signature_from_source(&source, job);
    let session_key = bibtex_session_key(job, &signature);

    if job.bbl_path.exists() && aux_session_cache.bibtex_job_is_fresh(&session_key) {
        return Ok(false);
    }

    if !options.force && bibtex_cache_is_fresh(&job.state_path, &signature, &job.bbl_path)? {
        aux_session_cache.mark_bibtex_job_fresh(session_key);
        return Ok(false);
    }

    if !options.force
        && restore_global_bibtex_cache_if_fresh(doc_dir, out_dir, job, &signature, &source)?
    {
        aux_session_cache.mark_bibtex_job_fresh(session_key);
        return Ok(false);
    }

    run_bibtex(doc_dir, out_dir, job, options)?;
    write_bibtex_state_from_source(job, &signature, &source, doc_dir, out_dir)?;
    save_global_bibtex_cache(doc_dir, out_dir, job, &signature, &source)?;
    aux_session_cache.mark_bibtex_job_fresh(session_key);
    Ok(true)
}

fn bibtex_session_key(job: &BibtexJob, signature: &str) -> BibtexSessionKey {
    BibtexSessionKey {
        state_path: job.state_path.clone(),
        bbl_path: job.bbl_path.clone(),
        signature: signature.to_string(),
    }
}

fn restore_global_bibtex_cache_if_fresh(
    doc_dir: &Path,
    out_dir: &Path,
    job: &BibtexJob,
    signature: &str,
    aux_source: &str,
) -> Result<bool> {
    let cache = global_bibtex_cache_paths(signature);
    if !bibtex_cache_is_fresh(&cache.state_path, signature, &cache.bbl_path)? {
        return Ok(false);
    }
    if let Some(parent) = job.bbl_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create bibliography output directory {}",
                parent.display()
            )
        })?;
    }
    fs::copy(&cache.bbl_path, &job.bbl_path).with_context(|| {
        format!(
            "failed to restore bibliography cache {} to {}",
            cache.bbl_path.display(),
            job.bbl_path.display()
        )
    })?;
    if cache.blg_path.exists() {
        fs::copy(&cache.blg_path, job.bbl_path.with_extension("blg")).with_context(|| {
            format!(
                "failed to restore bibliography log cache {}",
                cache.blg_path.display()
            )
        })?;
    }
    write_bibtex_state_from_source(job, signature, aux_source, doc_dir, out_dir)?;
    Ok(true)
}

fn save_global_bibtex_cache(
    doc_dir: &Path,
    out_dir: &Path,
    job: &BibtexJob,
    signature: &str,
    aux_source: &str,
) -> Result<()> {
    if !job.bbl_path.exists() {
        return Ok(());
    }
    let cache = global_bibtex_cache_paths(signature);
    fs::create_dir_all(&cache.dir).with_context(|| {
        format!(
            "failed to create global bibliography cache {}",
            cache.dir.display()
        )
    })?;
    fs::copy(&job.bbl_path, &cache.bbl_path).with_context(|| {
        format!(
            "failed to save bibliography cache {}",
            cache.bbl_path.display()
        )
    })?;
    let blg_path = job.bbl_path.with_extension("blg");
    if blg_path.exists() {
        fs::copy(&blg_path, &cache.blg_path).with_context(|| {
            format!(
                "failed to save bibliography log cache {}",
                cache.blg_path.display()
            )
        })?;
    }
    let mut cache_job = job.clone();
    cache_job.bbl_path = cache.bbl_path;
    cache_job.state_path = cache.state_path;
    write_bibtex_state_from_source(&cache_job, signature, aux_source, doc_dir, out_dir)
}

#[derive(Debug, Clone)]
struct GlobalBibtexCachePaths {
    dir: PathBuf,
    bbl_path: PathBuf,
    blg_path: PathBuf,
    state_path: PathBuf,
}

fn global_bibtex_cache_paths(signature: &str) -> GlobalBibtexCachePaths {
    let root = cache_root_from_env("TEXPILOT_BIBTEX_CACHE", default_bibtex_cache_root);
    let dir = root.join(format!("{:016x}", content_hash(signature.as_bytes())));
    GlobalBibtexCachePaths {
        bbl_path: dir.join("output.bbl"),
        blg_path: dir.join("output.blg"),
        state_path: dir.join("state.toml"),
        dir,
    }
}

fn refresh_bibtex_state_if_available(doc_dir: &Path, out_dir: &Path, job_name: &str) -> Result<()> {
    for aux_path in bibliography_aux_files(doc_dir, out_dir, job_name)? {
        let job = bibtex_job_for_run(doc_dir, out_dir, job_name, &aux_path)?;
        if !job.bbl_path.exists() {
            continue;
        }
        let source = fs::read_to_string(&aux_path)
            .with_context(|| format!("failed to read aux file {}", aux_path.display()))?;
        if !source.contains(r"\bibdata") {
            continue;
        }
        let signature = bibtex_aux_signature_from_source(&source, &job);
        write_bibtex_state_from_source(&job, &signature, &source, doc_dir, out_dir)?;
    }
    Ok(())
}

fn bibliography_aux_files(doc_dir: &Path, out_dir: &Path, job_name: &str) -> Result<Vec<PathBuf>> {
    let root_aux = out_dir.join(format!("{job_name}.aux"));
    let mut pending = vec![root_aux];
    let mut seen = Vec::<PathBuf>::new();
    let mut result = Vec::<PathBuf>::new();

    let mut latest_aux_output_keys = None;
    let fls_path = out_dir.join(format!("{job_name}.fls"));
    if fls_path.exists() {
        let latest_aux_outputs = recorded_outputs(&fls_path, doc_dir)?
            .into_iter()
            .filter(|path| path_extension_is_any(path, &["aux"]))
            .filter(|path| path.exists())
            .collect::<Vec<_>>();
        latest_aux_output_keys = Some(
            latest_aux_outputs
                .iter()
                .map(|path| canonical_or_original(path))
                .collect::<HashSet<_>>(),
        );
        pending.extend(latest_aux_outputs);
    }

    while let Some(aux_path) = pending.pop() {
        if seen.iter().any(|path| path == &aux_path) {
            continue;
        }
        seen.push(aux_path.clone());
        let Ok(source) = fs::read_to_string(&aux_path) else {
            continue;
        };
        if source.contains(r"\bibdata") {
            result.push(aux_path.clone());
        }
        for line in source.lines() {
            if let Some(input) = braced_payload(line, r"\@input") {
                let input_path = Path::new(input);
                let nested = if input_path.is_absolute() {
                    input_path.to_path_buf()
                } else {
                    out_dir.join(input_path)
                };
                if latest_aux_output_keys
                    .as_ref()
                    .is_none_or(|outputs| outputs.contains(&canonical_or_original(&nested)))
                {
                    pending.push(nested);
                }
            }
        }
    }

    result.sort();
    result.dedup();
    Ok(result)
}

fn bibtex_jobs_for_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Vec<(PathBuf, BibtexJob)>> {
    let specs = bibtex_command_specs_from_logreq(doc_dir, out_dir, job_name)?;
    bibliography_aux_files(doc_dir, out_dir, job_name)?
        .into_iter()
        .map(|aux_path| {
            let key = canonical_or_original(&aux_path).display().to_string();
            let job = bibtex_job(out_dir, &aux_path, specs.get(&key));
            Ok((aux_path, job))
        })
        .collect()
}

fn bibtex_job_for_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    aux_path: &Path,
) -> Result<BibtexJob> {
    let specs = bibtex_command_specs_from_logreq(doc_dir, out_dir, job_name)?;
    let key = canonical_or_original(aux_path).display().to_string();
    Ok(bibtex_job(out_dir, aux_path, specs.get(&key)))
}

fn bibtex_job(
    out_dir: &Path,
    aux_path: &Path,
    command_spec: Option<&BibtexCommandSpec>,
) -> BibtexJob {
    let relative = aux_path.strip_prefix(out_dir).unwrap_or(aux_path);
    let command_path = relative.with_extension("");
    let command_arg = command_path.to_string_lossy().to_string();
    let bbl_path = aux_path.with_extension("bbl");
    let state_name = command_arg
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect::<String>();
    let state_path = out_dir.join(format!(".texpilot-{state_name}.bibstate.toml"));
    BibtexJob {
        program: command_spec
            .map(|spec| spec.program)
            .unwrap_or(BibtexProgram::Bibtex),
        command_options: command_spec
            .map(|spec| spec.options.clone())
            .unwrap_or_default(),
        command_arg,
        bbl_path,
        request_inputs: command_spec
            .map(|spec| spec.request_inputs.clone())
            .unwrap_or_default(),
        state_path,
    }
}

fn bibtex_command_specs_from_logreq(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<HashMap<String, BibtexCommandSpec>> {
    let run_xml_path = out_dir.join(format!("{job_name}.run.xml"));
    let Ok(source) = fs::read_to_string(&run_xml_path) else {
        return Ok(HashMap::new());
    };

    let mut specs = HashMap::new();
    for block in xml_blocks(&source, "external") {
        let Some(generic) = xml_first_tag_value(block, "generic") else {
            continue;
        };
        if generic.trim() != "bibtex" {
            continue;
        }

        let Some(cmdline) = xml_blocks(block, "cmdline").into_iter().next() else {
            continue;
        };
        let Some(infile) = xml_first_tag_value(cmdline, "infile") else {
            continue;
        };
        let Some(aux_path) = resolve_bibtex_logreq_aux(out_dir, &infile) else {
            continue;
        };

        let program = xml_first_tag_value(cmdline, "binary")
            .and_then(|value| bibtex_program(&value))
            .unwrap_or(BibtexProgram::Bibtex);
        let mut options = Vec::new();
        for option in xml_tag_values(cmdline, "option") {
            options.extend(split_shell_words(&option));
        }

        let mut request_inputs = Vec::new();
        for section in xml_blocks(block, "input")
            .into_iter()
            .chain(xml_blocks(block, "requires"))
        {
            for file in xml_tag_values(section, "file") {
                if let Some(path) = resolve_logreq_file(doc_dir, out_dir, &file)? {
                    request_inputs.push(path);
                }
            }
        }
        request_inputs.retain(|path| !same_existing_path(path, &aux_path));
        request_inputs.sort();
        request_inputs.dedup();

        specs.insert(
            canonical_or_original(&aux_path).display().to_string(),
            BibtexCommandSpec {
                program,
                options,
                request_inputs,
            },
        );
    }
    Ok(specs)
}

fn resolve_bibtex_logreq_aux(out_dir: &Path, value: &str) -> Option<PathBuf> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    let path = Path::new(value);
    let path = if path_extension_is_any(path, &["aux"]) {
        path.to_path_buf()
    } else {
        path.with_extension("aux")
    };
    Some(if path.is_absolute() {
        path
    } else {
        out_dir.join(path)
    })
}

fn resolve_logreq_file(doc_dir: &Path, out_dir: &Path, value: &str) -> Result<Option<PathBuf>> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(None);
    }
    let path = Path::new(value);
    if path.is_absolute() {
        return Ok(path.exists().then(|| path.to_path_buf()));
    }

    let out_candidate = out_dir.join(path);
    if out_candidate.exists() {
        return Ok(Some(out_candidate));
    }
    let doc_candidate = doc_dir.join(path);
    if doc_candidate.exists() {
        return Ok(Some(doc_candidate));
    }

    match path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase)
    {
        Some(extension)
            if matches!(
                extension.as_str(),
                "bib" | "bst" | "tex" | "def" | "bbx" | "cbx" | "lbx" | "cfg"
            ) =>
        {
            resolve_kpathsea_input(doc_dir, value, &extension)
        }
        _ => Ok(None),
    }
}

fn bibtex_program(value: &str) -> Option<BibtexProgram> {
    let name = Path::new(value.trim())
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(value)
        .to_ascii_lowercase();
    match name.as_str() {
        "bibtex" | "bibtex.exe" => Some(BibtexProgram::Bibtex),
        "bibtex8" | "bibtex8.exe" => Some(BibtexProgram::Bibtex8),
        "bibtexu" | "bibtexu.exe" => Some(BibtexProgram::Bibtexu),
        "pbibtex" | "pbibtex.exe" => Some(BibtexProgram::Pbibtex),
        "upbibtex" | "upbibtex.exe" => Some(BibtexProgram::Upbibtex),
        _ => None,
    }
}

fn run_bibtex(
    doc_dir: &Path,
    out_dir: &Path,
    job: &BibtexJob,
    options: &BuildOptions,
) -> Result<()> {
    let mut command = Command::new(job.program.executable());
    command
        .current_dir(out_dir)
        .args(&job.command_options)
        .arg(&job.command_arg)
        .env("BIBINPUTS", kpathsea_env("BIBINPUTS", doc_dir))
        .env("BSTINPUTS", kpathsea_env("BSTINPUTS", doc_dir))
        .env("TEXINPUTS", kpathsea_env("TEXINPUTS", doc_dir));
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command
        .status()
        .context("failed to launch BibTeX-family tool")?;
    if !status.success() {
        bail!("BibTeX-family tool failed with status {status}");
    }
    Ok(())
}

fn run_biber(doc_dir: &Path, out_dir: &Path, job_name: &str, options: &BuildOptions) -> Result<()> {
    let mut command = Command::new("biber");
    let out_dir_arg = path_arg_relative_to(doc_dir, out_dir);
    command
        .current_dir(doc_dir)
        .arg("--input-directory")
        .arg(&out_dir_arg)
        .arg("--output-directory")
        .arg(&out_dir_arg)
        .arg(job_name)
        .env("BIBINPUTS", kpathsea_env("BIBINPUTS", doc_dir))
        .env("BSTINPUTS", kpathsea_env("BSTINPUTS", doc_dir))
        .env("TEXINPUTS", kpathsea_env("TEXINPUTS", doc_dir));
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch Biber")?;
    if !status.success() {
        bail!("Biber failed with status {status}");
    }
    Ok(())
}

fn run_biber_if_stale(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
) -> Result<usize> {
    let Some(bcf_path) = biber_control_file_from_latest_run(doc_dir, out_dir, job_name)? else {
        return Ok(0);
    };
    let signature = biber_signature(&bcf_path)?;
    let bbl_path = out_dir.join(format!("{job_name}.bbl"));
    let state_path = out_dir.join(format!(".texpilot-{job_name}.biberstate.toml"));

    if !options.force && bibtex_cache_is_fresh(&state_path, &signature, &bbl_path)? {
        return Ok(0);
    }

    run_biber(doc_dir, out_dir, job_name, options)?;
    write_biber_state(&state_path, &signature, &bbl_path, &bcf_path, doc_dir)?;
    Ok(1)
}

fn refresh_biber_state_if_available(doc_dir: &Path, out_dir: &Path, job_name: &str) -> Result<()> {
    let Some(bcf_path) = biber_control_file_from_latest_run(doc_dir, out_dir, job_name)? else {
        return Ok(());
    };
    let bbl_path = out_dir.join(format!("{job_name}.bbl"));
    if !bbl_path.exists() {
        return Ok(());
    }
    let signature = biber_signature(&bcf_path)?;
    let state_path = out_dir.join(format!(".texpilot-{job_name}.biberstate.toml"));
    write_biber_state(&state_path, &signature, &bbl_path, &bcf_path, doc_dir)
}

fn biber_signature(bcf_path: &Path) -> Result<String> {
    let mut signature = fs::read_to_string(bcf_path)
        .with_context(|| format!("failed to read biber control file {}", bcf_path.display()))?;
    signature.push_str(&environment_signature(BIB_ENV_VARS));
    Ok(signature)
}

fn biber_control_file_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Option<PathBuf>> {
    let bcf_path = out_dir.join(format!("{job_name}.bcf"));
    let fls_path = out_dir.join(format!("{job_name}.fls"));
    if fls_path.exists() {
        let outputs = recorded_outputs(&fls_path, doc_dir)?;
        return Ok(outputs
            .into_iter()
            .any(|path| same_existing_path(&path, &bcf_path))
            .then_some(bcf_path)
            .filter(|path| path.exists()));
    }
    Ok(bcf_path.exists().then_some(bcf_path))
}

fn write_biber_state(
    state_path: &Path,
    signature: &str,
    bbl_path: &Path,
    bcf_path: &Path,
    doc_dir: &Path,
) -> Result<()> {
    let source = fs::read_to_string(bcf_path)
        .with_context(|| format!("failed to read biber control file {}", bcf_path.display()))?;
    let previous = previous_bib_input_map(state_path)?;
    let mut inputs = biber_inputs_from_bcf(&source, doc_dir, &previous)?;
    inputs.extend(biber_config_inputs(doc_dir, &previous)?);
    inputs.sort_by(|left, right| left.path.cmp(&right.path));
    inputs.dedup_by(|left, right| left.path == right.path);
    let state = BibState {
        version: BIB_STATE_VERSION,
        signature: signature.to_string(),
        bbl_path: bbl_path.display().to_string(),
        inputs,
    };
    let state_source = toml::to_string(&state).context("failed to serialize biber state")?;
    fs::write(state_path, state_source)
        .with_context(|| format!("failed to write biber state {}", state_path.display()))
}

fn run_indexes_if_needed(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
) -> Result<usize> {
    let mut runs = 0;
    let splitindex_jobs = splitindex_jobs_from_latest_run(doc_dir, out_dir, job_name)?;
    runs += run_splitindex_jobs_if_stale(&splitindex_jobs, options)?;

    let jobs = makeindex_jobs_from_latest_run(doc_dir, out_dir, job_name)?;
    let makeglossaries_jobs = jobs
        .iter()
        .filter(|job| job.tool == IndexTool::MakeGlossaries)
        .cloned()
        .collect::<Vec<_>>();
    if !makeglossaries_jobs.is_empty()
        && run_makeglossaries_if_stale(doc_dir, out_dir, job_name, &makeglossaries_jobs, options)?
    {
        runs += 1;
    }
    let makeindex_jobs = jobs
        .iter()
        .filter(|job| matches!(job.tool, IndexTool::MakeIndex | IndexTool::Xindy))
        .cloned()
        .collect::<Vec<_>>();
    runs += run_makeindex_jobs_if_stale(&makeindex_jobs, options)?;
    Ok(runs)
}

fn run_makeglossaries_if_stale(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    jobs: &[MakeIndexJob],
    options: &BuildOptions,
) -> Result<bool> {
    let mut stale = false;
    for job in jobs {
        let source = fs::read_to_string(&job.input_path).with_context(|| {
            format!("failed to read glossary file {}", job.input_path.display())
        })?;
        if !makeindex_source_has_entries(&source) {
            continue;
        }
        let signature = makeindex_signature(&source, job)?;
        if options.force || !index_cache_is_fresh(&job.state_path, &signature, job)? {
            stale = true;
            break;
        }
    }

    if !stale {
        return Ok(false);
    }

    run_makeglossaries(doc_dir, out_dir, job_name, options)?;
    for job in jobs {
        if !job.input_path.exists() || !job.output_path.exists() {
            continue;
        }
        let source = fs::read_to_string(&job.input_path).with_context(|| {
            format!("failed to read glossary file {}", job.input_path.display())
        })?;
        if !makeindex_source_has_entries(&source) {
            continue;
        }
        let signature = makeindex_signature(&source, job)?;
        write_index_state(&job.state_path, &signature, job)?;
    }
    Ok(true)
}

fn run_makeindex_if_stale(job: &MakeIndexJob, options: &BuildOptions) -> Result<bool> {
    let source = fs::read_to_string(&job.input_path)
        .with_context(|| format!("failed to read index file {}", job.input_path.display()))?;
    if !makeindex_source_has_entries(&source) {
        return Ok(false);
    }
    let signature = makeindex_signature(&source, job)?;

    if !options.force && index_cache_is_fresh(&job.state_path, &signature, job)? {
        return Ok(false);
    }

    run_makeindex(job, options)?;
    write_index_state(&job.state_path, &signature, job)?;
    Ok(true)
}

fn run_splitindex_if_stale(job: &SplitIndexJob, options: &BuildOptions) -> Result<bool> {
    let source = fs::read_to_string(&job.input_path).with_context(|| {
        format!(
            "failed to read split index file {}",
            job.input_path.display()
        )
    })?;
    let signature = splitindex_signature(&source);

    if !options.force && splitindex_cache_is_fresh(&job.state_path, &signature, &job.output_paths)?
    {
        return Ok(false);
    }

    run_splitindex(job, options)?;
    write_splitindex_state(job, &signature)?;
    Ok(true)
}

fn run_makeindex(job: &MakeIndexJob, options: &BuildOptions) -> Result<()> {
    let index_dir = job
        .input_path
        .parent()
        .context("index file has no parent directory")?;
    let input_name = job
        .input_path
        .file_name()
        .context("index file has no filename")?;
    let program = job
        .program
        .unwrap_or(IndexCommandProgram::MakeIndex)
        .executable();
    let mut command = Command::new(program);
    command.current_dir(index_dir);
    command.args(&job.command_options);
    if job.tool == IndexTool::MakeIndex
        && let Some(style_path) = &job.style_path
    {
        command.arg("-s").arg(style_path);
    }
    command
        .arg("-t")
        .arg(path_arg_relative_to(index_dir, &job.transcript_path))
        .arg("-o")
        .arg(path_arg_relative_to(index_dir, &job.output_path))
        .arg(input_name);
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch index tool")?;
    if !status.success() {
        bail!("index tool failed with status {status}");
    }
    Ok(())
}

fn run_splitindex(job: &SplitIndexJob, options: &BuildOptions) -> Result<()> {
    let index_dir = job
        .input_path
        .parent()
        .context("split index file has no parent directory")?;
    let input_name = job
        .input_path
        .file_name()
        .context("split index file has no filename")?;
    let mut command = Command::new("splitindex");
    command
        .current_dir(index_dir)
        .arg("-m")
        .arg("")
        .arg(input_name);
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command.status().context("failed to launch SplitIndex")?;
    if !status.success() {
        bail!("SplitIndex failed with status {status}");
    }
    Ok(())
}

fn run_makeglossaries(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    options: &BuildOptions,
) -> Result<()> {
    let mut command = Command::new("makeglossaries");
    command.current_dir(doc_dir).arg("-d").arg(out_dir);
    if options.quiet {
        command.arg("-q");
    }
    command.arg(job_name);
    if options.print_command {
        eprintln!("{}", display_command(&command));
    }
    configure_output(&mut command, options);
    let status = command
        .status()
        .context("failed to launch MakeGlossaries")?;
    if !status.success() {
        bail!("MakeGlossaries failed with status {status}");
    }
    Ok(())
}

fn refresh_index_states_if_available(doc_dir: &Path, out_dir: &Path, job_name: &str) -> Result<()> {
    for job in makeindex_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        let source = fs::read_to_string(&job.input_path)
            .with_context(|| format!("failed to read index file {}", job.input_path.display()))?;
        if !makeindex_source_has_entries(&source) {
            continue;
        }
        if !job.output_path.exists() {
            continue;
        }
        let signature = makeindex_signature(&source, &job)?;
        write_index_state(&job.state_path, &signature, &job)?;
    }
    Ok(())
}

fn index_cache_is_fresh(state_path: &Path, signature: &str, job: &MakeIndexJob) -> Result<bool> {
    if !state_path.exists() || !job.output_path.exists() {
        return Ok(false);
    }
    let source = fs::read_to_string(state_path)
        .with_context(|| format!("failed to read index state {}", state_path.display()))?;
    let state: IndexState = toml::from_str(&source)
        .with_context(|| format!("failed to parse index state {}", state_path.display()))?;
    if state.version != INDEX_STATE_VERSION
        || state.signature != signature
        || state.output_path != job.output_path.display().to_string()
    {
        return Ok(false);
    }
    for input in &state.inputs {
        if !input_fingerprint_is_fresh(input)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn splitindex_cache_is_fresh(
    state_path: &Path,
    signature: &str,
    output_paths: &[PathBuf],
) -> Result<bool> {
    if !state_path.exists() || output_paths.iter().any(|path| !path.exists()) {
        return Ok(false);
    }
    let source = fs::read_to_string(state_path)
        .with_context(|| format!("failed to read split index state {}", state_path.display()))?;
    let state: SplitIndexState = toml::from_str(&source)
        .with_context(|| format!("failed to parse split index state {}", state_path.display()))?;
    let outputs = output_paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    if state.version != SPLIT_INDEX_STATE_VERSION
        || state.signature != signature
        || state.outputs != outputs
    {
        return Ok(false);
    }
    for input in &state.inputs {
        if !input_fingerprint_is_fresh(input)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn write_index_state(state_path: &Path, signature: &str, job: &MakeIndexJob) -> Result<()> {
    let previous = previous_index_input_map(state_path)?;
    let mut inputs = Vec::new();
    if !job.style_is_build_output
        && let Some(style_path) = &job.style_path
        && let Some(fingerprint) = fingerprint_path_reusing(style_path, Some(&previous))?
    {
        inputs.push(fingerprint);
    }
    let state = IndexState {
        version: INDEX_STATE_VERSION,
        signature: signature.to_string(),
        output_path: job.output_path.display().to_string(),
        inputs,
    };
    let source = toml::to_string(&state).context("failed to serialize index state")?;
    fs::write(state_path, source)
        .with_context(|| format!("failed to write index state {}", state_path.display()))
}

fn write_splitindex_state(job: &SplitIndexJob, signature: &str) -> Result<()> {
    let state = SplitIndexState {
        version: SPLIT_INDEX_STATE_VERSION,
        signature: signature.to_string(),
        outputs: job
            .output_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        inputs: Vec::new(),
    };
    let source = toml::to_string(&state).context("failed to serialize split index state")?;
    fs::write(&job.state_path, source).with_context(|| {
        format!(
            "failed to write split index state {}",
            job.state_path.display()
        )
    })
}

fn makeindex_command_specs_from_log(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<HashMap<String, MakeIndexCommandSpec>> {
    let log_path = out_dir.join(format!("{job_name}.log"));
    let Ok(source) = fs::read_to_string(&log_path) else {
        return Ok(HashMap::new());
    };

    let mut paths = HashMap::new();
    for command in makeindex_commands_from_log(&source) {
        for (idx_arg, parsed) in parse_makeindex_command(&command) {
            let Some(input_path) = resolve_makeindex_command_input(out_dir, doc_dir, &idx_arg)
            else {
                continue;
            };
            let style_path = if let Some(style) = parsed.style {
                resolve_makeindex_style_argument(doc_dir, out_dir, &style)?
            } else {
                None
            };
            paths.insert(
                canonical_or_original(&input_path).display().to_string(),
                MakeIndexCommandSpec {
                    program: parsed.program,
                    style_path,
                    options: parsed.options,
                },
            );
        }
    }
    Ok(paths)
}

fn makeindex_commands_from_log(source: &str) -> Vec<String> {
    let mut commands = Vec::new();
    for line in source.lines() {
        let mut cursor = 0;
        while let Some(start) = line[cursor..].find("runsystem(") {
            let command_start = cursor + start + "runsystem(".len();
            let Some(end) = line[command_start..].find(")...") else {
                break;
            };
            commands.push(line[command_start..command_start + end].to_string());
            cursor = command_start + end + 1;
        }

        let mut cursor = 0;
        while let Some(start) = line[cursor..].find('`') {
            let command_start = cursor + start + 1;
            let Some(end) = line[command_start..].find('\'') else {
                break;
            };
            commands.push(line[command_start..command_start + end].to_string());
            cursor = command_start + end + 1;
        }
    }
    commands
}

fn parse_makeindex_command(command: &str) -> Vec<(String, ParsedMakeIndexCommand)> {
    let words = split_shell_words(command);
    let Some(program) = words.first() else {
        return Vec::new();
    };
    let Some(program) = index_command_program(program) else {
        return Vec::new();
    };

    let mut parsed = ParsedMakeIndexCommand {
        program,
        ..ParsedMakeIndexCommand::default()
    };
    let mut inputs = Vec::new();
    let mut i = 1;
    while i < words.len() {
        let word = &words[i];
        if word == "-s" {
            if program == IndexCommandProgram::MakeIndex
                && let Some(value) = words.get(i + 1)
            {
                parsed.style = Some(value.clone());
            }
            i += 2;
            continue;
        }
        if program == IndexCommandProgram::MakeIndex
            && let Some(value) = word.strip_prefix("-s").filter(|value| !value.is_empty())
        {
            parsed.style = Some(value.to_string());
            i += 1;
            continue;
        }
        if word == "-p" {
            if let Some(value) = words.get(i + 1) {
                parsed.options.push("-p".to_string());
                parsed.options.push(value.clone());
            }
            i += 2;
            continue;
        }
        if let Some(value) = word.strip_prefix("-p").filter(|value| !value.is_empty()) {
            parsed.options.push("-p".to_string());
            parsed.options.push(value.to_string());
            i += 1;
            continue;
        }
        if matches!(word.as_str(), "-o" | "-t") {
            i += 2;
            continue;
        }
        if word == "--" {
            inputs.extend(
                words[i + 1..]
                    .iter()
                    .filter(|value| ends_with_ignore_ascii_case(value, ".idx"))
                    .cloned(),
            );
            break;
        }
        if word.starts_with('-') {
            match program {
                IndexCommandProgram::MakeIndex => {
                    parsed.options.extend(makeindex_passthrough_options(word));
                }
                IndexCommandProgram::Xindy | IndexCommandProgram::Texindy => {
                    collect_xindy_option(&words, &mut i, &mut parsed.options);
                }
            }
            i += 1;
            continue;
        }
        if ends_with_ignore_ascii_case(word, ".idx") {
            inputs.push(word.clone());
        }
        i += 1;
    }

    if program == IndexCommandProgram::MakeIndex
        && parsed.style.is_none()
        && parsed.options.is_empty()
    {
        return Vec::new();
    }
    inputs
        .into_iter()
        .map(|input| (input, parsed.clone()))
        .collect()
}

fn makeindex_passthrough_options(word: &str) -> Vec<String> {
    let Some(flags) = word.strip_prefix('-') else {
        return Vec::new();
    };
    flags
        .chars()
        .filter(|flag| matches!(flag, 'c' | 'g' | 'l' | 'q' | 'r' | 'L' | 'T'))
        .map(|flag| format!("-{flag}"))
        .collect()
}

fn split_shell_words(command: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    let mut escaped = false;

    for ch in command.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if Some(ch) == quote {
            quote = None;
            continue;
        }
        if quote.is_none() && matches!(ch, '\'' | '"') {
            quote = Some(ch);
            continue;
        }
        if quote.is_none() && ch.is_whitespace() {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
            continue;
        }
        current.push(ch);
    }

    if escaped {
        current.push('\\');
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

fn collect_xindy_option(words: &[String], index: &mut usize, options: &mut Vec<String>) {
    let word = &words[*index];
    if matches!(word.as_str(), "-L" | "-C" | "-M" | "-I" | "-d") {
        if let Some(value) = words.get(*index + 1) {
            options.push(word.clone());
            options.push(value.clone());
            *index += 1;
        }
        return;
    }
    for prefix in ["-L", "-C", "-M", "-I", "-d"] {
        if let Some(value) = word.strip_prefix(prefix).filter(|value| !value.is_empty()) {
            options.push(prefix.to_string());
            options.push(value.to_string());
            return;
        }
    }
    options.extend(
        word.strip_prefix('-')
            .unwrap_or_default()
            .chars()
            .filter(|flag| matches!(flag, 'q' | 'v' | 'g' | 'l' | 'r'))
            .map(|flag| format!("-{flag}")),
    );
}

fn index_command_program(value: &str) -> Option<IndexCommandProgram> {
    Path::new(value)
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| match name {
            "makeindex" | "makeindex.exe" => Some(IndexCommandProgram::MakeIndex),
            "xindy" | "xindy.exe" => Some(IndexCommandProgram::Xindy),
            "texindy" | "texindy.exe" | "truexindy" | "truexindy.exe" => {
                Some(IndexCommandProgram::Texindy)
            }
            _ => None,
        })
}

fn resolve_makeindex_command_input(out_dir: &Path, doc_dir: &Path, value: &str) -> Option<PathBuf> {
    let path = Path::new(value);
    if path.is_absolute() {
        return path.exists().then(|| path.to_path_buf());
    }
    let out_candidate = out_dir.join(path);
    if out_candidate.exists() {
        return Some(out_candidate);
    }
    let doc_candidate = doc_dir.join(path);
    if doc_candidate.exists() {
        return Some(doc_candidate);
    }
    None
}

fn resolve_makeindex_style_argument(
    doc_dir: &Path,
    out_dir: &Path,
    value: &str,
) -> Result<Option<PathBuf>> {
    let path = Path::new(value);
    if path.is_absolute() {
        return Ok(path.exists().then(|| path.to_path_buf()));
    }
    let doc_candidate = doc_dir.join(path);
    if doc_candidate.exists() {
        return Ok(Some(doc_candidate));
    }
    let out_candidate = out_dir.join(path);
    if out_candidate.exists() {
        return Ok(Some(out_candidate));
    }
    resolve_kpathsea_input(doc_dir, value, "ist")
}

fn makeindex_jobs_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Vec<MakeIndexJob>> {
    let (paths, xdy_style_path) =
        makeindex_input_paths_from_latest_run(doc_dir, out_dir, job_name)?;
    let command_specs = makeindex_command_specs_from_log(doc_dir, out_dir, job_name)?;
    let mut expanded_paths = Vec::new();
    for path in paths {
        if path_extension_is_any(&path, &["idx"]) {
            let source = fs::read_to_string(&path)
                .with_context(|| format!("failed to read index file {}", path.display()))?;
            if splitindex_source_needs_splitting(&source) {
                expanded_paths.extend(splitindex_output_paths(&path, &source)?);
                continue;
            }
        }
        expanded_paths.push(path);
    }

    expanded_paths.sort();
    expanded_paths.dedup();
    expanded_paths
        .into_iter()
        .map(|path| {
            let command_spec =
                command_specs.get(&canonical_or_original(&path).display().to_string());
            makeindex_job(
                doc_dir,
                out_dir,
                job_name,
                path,
                xdy_style_path.as_deref(),
                command_spec,
            )
        })
        .collect()
}

fn splitindex_jobs_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Vec<SplitIndexJob>> {
    let (paths, _) = makeindex_input_paths_from_latest_run(doc_dir, out_dir, job_name)?;
    let mut jobs = Vec::new();
    for path in paths {
        if !path_extension_is_any(&path, &["idx"]) {
            continue;
        }
        let source = fs::read_to_string(&path)
            .with_context(|| format!("failed to read index file {}", path.display()))?;
        if !splitindex_source_needs_splitting(&source) {
            continue;
        }
        let output_paths = splitindex_output_paths(&path, &source)?;
        if output_paths.is_empty() {
            continue;
        }
        let state_path = out_dir.join(format!(
            ".texpilot-{}.splitindexstate.toml",
            makeindex_state_name(out_dir, &path)
        ));
        jobs.push(SplitIndexJob {
            input_path: path,
            output_paths,
            state_path,
        });
    }
    jobs.sort_by(|left, right| left.input_path.cmp(&right.input_path));
    Ok(jobs)
}

fn makeindex_input_paths_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<(Vec<PathBuf>, Option<PathBuf>)> {
    let fls_path = out_dir.join(format!("{job_name}.fls"));
    let mut xdy_style_path = None;
    let mut paths = if fls_path.exists() {
        let outputs = recorded_outputs(&fls_path, doc_dir)?;
        let xdy_file_name = format!("{job_name}.xdy");
        xdy_style_path = outputs
            .iter()
            .find(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name == xdy_file_name)
            })
            .filter(|path| path.exists())
            .cloned();
        outputs
            .into_iter()
            .filter(|path| makeindex_input_kind(path).is_some())
            .filter(|path| path.exists())
            .collect::<Vec<_>>()
    } else {
        let mut paths = Vec::new();
        let xdy_candidate = out_dir.join(format!("{job_name}.xdy"));
        if xdy_candidate.exists() {
            xdy_style_path = Some(xdy_candidate);
        }
        for extension in ["idx", "glo", "acn", "nlo"] {
            let candidate = out_dir.join(format!("{job_name}.{extension}"));
            if candidate.exists() {
                paths.push(candidate);
            }
        }
        paths
    };

    paths.sort();
    paths.dedup();
    Ok((paths, xdy_style_path))
}

fn makeindex_job(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    input_path: PathBuf,
    xdy_style_path: Option<&Path>,
    command_spec: Option<&MakeIndexCommandSpec>,
) -> Result<MakeIndexJob> {
    let kind = makeindex_input_kind(&input_path).context("unrecognized MakeIndex input")?;
    let tool = index_tool_for_kind(kind, xdy_style_path, command_spec);
    let output_path = input_path.with_extension(makeindex_output_extension(kind));
    let transcript_path = input_path.with_extension(makeindex_transcript_extension(kind));
    let style_path = makeindex_style_path(MakeIndexStyleRequest {
        doc_dir,
        out_dir,
        job_name,
        input_path: &input_path,
        kind,
        tool,
        xdy_style_path,
        command_spec,
    })?;
    let command_options = command_spec
        .map(|spec| spec.options.clone())
        .unwrap_or_default();
    let style_is_build_output = style_path
        .as_deref()
        .is_some_and(|style_path| style_path.starts_with(out_dir));
    let state_path = out_dir.join(format!(
        ".texpilot-{}.indexstate.toml",
        makeindex_state_name(out_dir, &input_path)
    ));
    Ok(MakeIndexJob {
        tool,
        program: command_spec.map(|spec| spec.program),
        input_path,
        output_path,
        transcript_path,
        style_path,
        command_options,
        style_is_build_output,
        state_path,
    })
}

fn index_tool_for_kind(
    kind: MakeIndexKind,
    xdy_style_path: Option<&Path>,
    command_spec: Option<&MakeIndexCommandSpec>,
) -> IndexTool {
    if matches!(
        command_spec.map(|spec| spec.program),
        Some(IndexCommandProgram::Xindy | IndexCommandProgram::Texindy)
    ) {
        return IndexTool::Xindy;
    }
    if matches!(kind, MakeIndexKind::Glossary | MakeIndexKind::Acronym) && xdy_style_path.is_some()
    {
        IndexTool::MakeGlossaries
    } else {
        IndexTool::MakeIndex
    }
}

fn makeindex_input_kind(path: &Path) -> Option<MakeIndexKind> {
    if path_extension_is_any(path, &["idx"]) {
        Some(MakeIndexKind::Index)
    } else if path_extension_is_any(path, &["glo"]) {
        Some(MakeIndexKind::Glossary)
    } else if path_extension_is_any(path, &["acn"]) {
        Some(MakeIndexKind::Acronym)
    } else if path_extension_is_any(path, &["nlo"]) {
        Some(MakeIndexKind::Nomenclature)
    } else {
        None
    }
}

fn splitindex_source_needs_splitting(source: &str) -> bool {
    source
        .lines()
        .any(|line| splitindex_entry_suffix(line).is_some())
}

fn splitindex_output_paths(input_path: &Path, source: &str) -> Result<Vec<PathBuf>> {
    let parent = input_path
        .parent()
        .context("split index file has no parent directory")?;
    let stem = input_path
        .file_stem()
        .context("split index file has no stem")?
        .to_string_lossy();
    let mut output_paths = Vec::new();
    for line in source.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let suffix = splitindex_entry_suffix(line).unwrap_or("idx");
        output_paths.push(parent.join(format!("{stem}-{suffix}.idx")));
    }
    output_paths.sort();
    output_paths.dedup();
    Ok(output_paths)
}

fn splitindex_entry_suffix(line: &str) -> Option<&str> {
    let rest = line.trim_start().strip_prefix(r"\indexentry[")?;
    let close = rest.find(']')?;
    Some(&rest[..close])
}

fn makeindex_output_extension(kind: MakeIndexKind) -> &'static str {
    match kind {
        MakeIndexKind::Index => "ind",
        MakeIndexKind::Glossary => "gls",
        MakeIndexKind::Acronym => "acr",
        MakeIndexKind::Nomenclature => "nls",
    }
}

fn makeindex_transcript_extension(kind: MakeIndexKind) -> &'static str {
    match kind {
        MakeIndexKind::Index => "ilg",
        MakeIndexKind::Glossary => "glg",
        MakeIndexKind::Acronym => "alg",
        MakeIndexKind::Nomenclature => "nlg",
    }
}

fn makeindex_style_path(request: MakeIndexStyleRequest<'_>) -> Result<Option<PathBuf>> {
    if request.tool == IndexTool::MakeGlossaries {
        return Ok(request.xdy_style_path.map(Path::to_path_buf));
    }

    match request.kind {
        MakeIndexKind::Index => Ok(request
            .command_spec
            .and_then(|spec| spec.style_path.as_ref())
            .cloned()),
        MakeIndexKind::Glossary | MakeIndexKind::Acronym => {
            let job_style = request.out_dir.join(format!("{}.ist", request.job_name));
            if job_style.exists() {
                return Ok(Some(job_style));
            }
            let input_style = request.input_path.with_extension("ist");
            if input_style.exists() {
                return Ok(Some(input_style));
            }
            Ok(None)
        }
        MakeIndexKind::Nomenclature => {
            let job_style = request.out_dir.join(format!("{}.ist", request.job_name));
            if job_style.exists() {
                return Ok(Some(job_style));
            }
            resolve_kpathsea_input(request.doc_dir, "nomencl", "ist")
        }
    }
}

fn makeindex_state_name(out_dir: &Path, input_path: &Path) -> String {
    input_path
        .strip_prefix(out_dir)
        .unwrap_or(input_path)
        .to_string_lossy()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn makeindex_source_has_entries(source: &str) -> bool {
    source
        .lines()
        .map(str::trim)
        .any(|line| !line.is_empty() && !line.starts_with('%'))
}

fn makeindex_signature(source: &str, job: &MakeIndexJob) -> Result<String> {
    let mut signature = String::from(source);
    signature.push_str("\n%% texpilot index tool\n");
    signature.push_str(match job.tool {
        IndexTool::MakeIndex => "makeindex",
        IndexTool::Xindy => job
            .program
            .unwrap_or(IndexCommandProgram::Texindy)
            .executable(),
        IndexTool::MakeGlossaries => "makeglossaries",
    });
    signature.push('\n');
    if !job.command_options.is_empty() {
        signature.push_str("\n%% texpilot makeindex options\n");
        for option in &job.command_options {
            signature.push_str(option);
            signature.push('\n');
        }
    }
    if let Some(style_path) = &job.style_path {
        let bytes = fs::read(style_path)
            .with_context(|| format!("failed to read index style {}", style_path.display()))?;
        signature.push_str("\n%% texpilot style ");
        signature.push_str(&style_path.display().to_string());
        signature.push('\n');
        signature.push_str(&String::from_utf8_lossy(&bytes));
    }
    Ok(signature)
}

fn splitindex_signature(source: &str) -> String {
    let mut signature = String::from(source);
    signature.push_str("\n%% texpilot splitindex command\nsplitindex -m <empty>\n");
    signature
}

fn eps_conversion_jobs_from_source(
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
) -> Result<Vec<EpsConversionJob>> {
    let source_cache = TexSourceReadCache::default();
    let includeonly = includeonly_filter_for_root(main)?;
    let context = SourceConversionContext {
        doc_dir,
        out_dir,
        includeonly: includeonly.as_ref(),
        source_cache: &source_cache,
    };
    eps_conversion_jobs_with_context(context, main)
}

fn eps_conversion_jobs_with_context(
    context: SourceConversionContext<'_>,
    main: &Path,
) -> Result<Vec<EpsConversionJob>> {
    let mut visited = HashSet::new();
    let mut jobs = Vec::new();
    collect_eps_conversion_jobs_from_source(
        context,
        main,
        &mut visited,
        &mut Vec::new(),
        &mut None,
        &mut jobs,
    )?;
    jobs.sort_by(|left, right| {
        left.output_path
            .cmp(&right.output_path)
            .then(left.input_path.cmp(&right.input_path))
    });
    jobs.dedup_by(|left, right| left.output_path == right.output_path);
    Ok(jobs)
}

fn svg_conversion_jobs_from_source(
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
) -> Result<Vec<SvgConversionJob>> {
    let source_cache = TexSourceReadCache::default();
    let includeonly = includeonly_filter_for_root(main)?;
    let context = SourceConversionContext {
        doc_dir,
        out_dir,
        includeonly: includeonly.as_ref(),
        source_cache: &source_cache,
    };
    svg_conversion_jobs_with_context(context, main)
}

fn svg_conversion_jobs_with_context(
    context: SourceConversionContext<'_>,
    main: &Path,
) -> Result<Vec<SvgConversionJob>> {
    let mut visited = HashSet::new();
    let mut jobs = Vec::new();
    collect_svg_conversion_jobs_from_source(
        context,
        main,
        &mut visited,
        &mut Vec::new(),
        &default_svg_include_settings(),
        &mut jobs,
    )?;
    jobs.sort_by(|left, right| {
        left.output_path
            .cmp(&right.output_path)
            .then(left.input_path.cmp(&right.input_path))
    });
    jobs.dedup_by(|left, right| left.output_path == right.output_path);
    Ok(jobs)
}

#[derive(Clone, Copy)]
struct SourceConversionContext<'a> {
    doc_dir: &'a Path,
    out_dir: &'a Path,
    includeonly: Option<&'a HashSet<String>>,
    source_cache: &'a TexSourceReadCache,
}

#[derive(Debug, Default)]
struct TexSourceReadCache {
    sources: Mutex<HashMap<PathBuf, Arc<String>>>,
    analyses: Mutex<HashMap<PathBuf, Arc<TexSourceAnalysis>>>,
}

#[derive(Debug, Clone)]
struct TexSourceAnalysis {
    include_dependencies: Vec<TexSourceDependency>,
    input_dependencies: Vec<TexSourceDependency>,
    package_payloads: Vec<String>,
    class_payloads: Vec<String>,
    bibliography_payloads: Vec<String>,
    bibliography_style_payloads: Vec<String>,
    pdfpages_payloads: Vec<String>,
    source_file_payloads: Vec<String>,
    features: SourceFeatures,
    pgf_externalize: PgfExternalizeScan,
    includegraphics_payloads: Vec<String>,
    animategraphics_refs: Vec<AnimateGraphicsRef>,
    declared_graphics_extensions: Option<Vec<String>>,
    includesvg_refs: Vec<IncludeSvgRef>,
    graphicspath_entries: Vec<String>,
    svgpath_entries: Vec<String>,
    svg_setup_refs: Vec<SvgSetupRef>,
}

fn canonical_tex_source_path(source_path: &Path) -> Result<PathBuf> {
    source_path.canonicalize().with_context(|| {
        format!(
            "failed to canonicalize TeX source {}",
            source_path.display()
        )
    })
}

fn read_cached_tex_source(
    source_cache: &TexSourceReadCache,
    source_path: &Path,
) -> Result<Arc<String>> {
    if let Some(source) = source_cache
        .sources
        .lock()
        .expect("TeX source read cache mutex poisoned")
        .get(source_path)
    {
        return Ok(Arc::clone(source));
    }

    let source = Arc::new(
        fs::read_to_string(source_path)
            .with_context(|| format!("failed to read TeX source {}", source_path.display()))?,
    );
    let mut sources = source_cache
        .sources
        .lock()
        .expect("TeX source read cache mutex poisoned");
    if let Some(cached) = sources.get(source_path) {
        return Ok(Arc::clone(cached));
    }
    sources.insert(source_path.to_path_buf(), Arc::clone(&source));
    Ok(source)
}

fn read_cached_tex_source_analysis(
    source_cache: &TexSourceReadCache,
    source_path: &Path,
) -> Result<Arc<TexSourceAnalysis>> {
    if let Some(analysis) = source_cache
        .analyses
        .lock()
        .expect("TeX source analysis cache mutex poisoned")
        .get(source_path)
    {
        return Ok(Arc::clone(analysis));
    }

    let source = read_cached_tex_source(source_cache, source_path)?;
    let stripped_source = tex_comment_stripped_source(&source);
    let includegraphics_payloads = includegraphics_payloads_stripped(&stripped_source);
    let animategraphics_refs = animategraphics_refs_stripped(&stripped_source);
    let includesvg_refs = includesvg_refs_stripped(&stripped_source);
    let analysis = Arc::new(TexSourceAnalysis {
        include_dependencies: tex_include_source_dependencies_stripped(&stripped_source),
        input_dependencies: tex_input_like_source_dependencies_stripped(&stripped_source),
        package_payloads: source_package_payloads_stripped(&stripped_source),
        class_payloads: source_class_payloads_stripped(&stripped_source),
        bibliography_payloads: source_bibliography_payloads_stripped(&stripped_source),
        bibliography_style_payloads: source_bibliography_style_payloads_stripped(&stripped_source),
        pdfpages_payloads: source_pdfpages_payloads_stripped(&stripped_source),
        source_file_payloads: source_file_payloads_stripped(&stripped_source),
        features: source_features_in_stripped_source_with_graphics_count(
            &stripped_source,
            includegraphics_payloads.len() + animategraphics_refs.len() + includesvg_refs.len(),
        ),
        pgf_externalize: pgf_externalize_scan_stripped(&stripped_source),
        includegraphics_payloads,
        animategraphics_refs,
        declared_graphics_extensions: declared_graphics_extensions_stripped(&stripped_source),
        includesvg_refs,
        graphicspath_entries: graphicspath_entries_stripped(&stripped_source),
        svgpath_entries: svgpath_entries_stripped(&stripped_source),
        svg_setup_refs: svg_setup_refs_stripped(&stripped_source),
    });

    let mut analyses = source_cache
        .analyses
        .lock()
        .expect("TeX source analysis cache mutex poisoned");
    if let Some(cached) = analyses.get(source_path) {
        return Ok(Arc::clone(cached));
    }
    analyses.insert(source_path.to_path_buf(), Arc::clone(&analysis));
    Ok(analysis)
}

fn active_include_dependencies(
    analysis: &TexSourceAnalysis,
    includeonly: Option<&HashSet<String>>,
) -> Vec<TexSourceDependency> {
    analysis
        .include_dependencies
        .iter()
        .filter(|dependency| include_dependency_is_active(dependency, includeonly))
        .cloned()
        .collect()
}

fn collect_eps_conversion_jobs_from_source(
    context: SourceConversionContext<'_>,
    source_path: &Path,
    visited: &mut HashSet<PathBuf>,
    graphic_paths: &mut Vec<PathBuf>,
    graphic_extensions: &mut Option<Vec<String>>,
    jobs: &mut Vec<EpsConversionJob>,
) -> Result<()> {
    let source_path = canonical_tex_source_path(source_path)?;
    if !visited.insert(source_path.clone()) {
        return Ok(());
    }
    let analysis = read_cached_tex_source_analysis(context.source_cache, &source_path)?;

    let original_graphic_path_count = graphic_paths.len();
    graphic_paths.extend(
        analysis
            .graphicspath_entries
            .iter()
            .cloned()
            .map(PathBuf::from),
    );
    let original_graphic_extensions = graphic_extensions.clone();
    if let Some(declared) = &analysis.declared_graphics_extensions {
        *graphic_extensions = Some(declared.clone());
    }

    for graphic in &analysis.includegraphics_payloads {
        push_eps_conversion_job_for_graphic(
            context,
            graphic_paths,
            graphic_extensions.as_deref(),
            graphic,
            jobs,
        )?;
    }
    for animation in &analysis.animategraphics_refs {
        for graphic in animategraphics_frame_payloads(animation) {
            push_eps_conversion_job_for_graphic(
                context,
                graphic_paths,
                graphic_extensions.as_deref(),
                &graphic,
                jobs,
            )?;
        }
    }

    for dependency in active_include_dependencies(&analysis, context.includeonly)
        .into_iter()
        .chain(analysis.input_dependencies.iter().cloned())
    {
        if let Some(path) = resolve_local_tex_source_dependency(context.doc_dir, &dependency)? {
            let dependency_graphic_path_count = graphic_paths.len();
            if let Some(local_graphic_path) = dependency.local_graphic_path {
                graphic_paths.push(local_graphic_path);
            }
            collect_eps_conversion_jobs_from_source(
                context,
                &path,
                visited,
                graphic_paths,
                graphic_extensions,
                jobs,
            )?;
            graphic_paths.truncate(dependency_graphic_path_count);
        }
    }

    graphic_paths.truncate(original_graphic_path_count);
    *graphic_extensions = original_graphic_extensions;
    Ok(())
}

fn push_eps_conversion_job_for_graphic(
    context: SourceConversionContext<'_>,
    graphic_paths: &[PathBuf],
    declared_extensions: Option<&[String]>,
    graphic: &str,
    jobs: &mut Vec<EpsConversionJob>,
) -> Result<()> {
    if let Some((input_path, explicit_eps)) =
        resolve_eps_graphic_input(context.doc_dir, graphic_paths, graphic, declared_extensions)?
    {
        let output_path = eps_conversion_output_path(
            context.doc_dir,
            context.out_dir,
            &input_path,
            explicit_eps,
        )?;
        let state_name = external_tool_state_name(context.out_dir, &output_path);
        let state_path = context
            .out_dir
            .join(format!(".texpilot-{state_name}.epspdfstate.toml"));
        jobs.push(EpsConversionJob {
            input_path,
            output_path,
            state_path,
        });
    }
    Ok(())
}

fn collect_svg_conversion_jobs_from_source(
    context: SourceConversionContext<'_>,
    source_path: &Path,
    visited: &mut HashSet<PathBuf>,
    svg_paths: &mut Vec<PathBuf>,
    inherited_settings: &SvgIncludeSettings,
    jobs: &mut Vec<SvgConversionJob>,
) -> Result<()> {
    let source_path = canonical_tex_source_path(source_path)?;
    if !visited.insert(source_path.clone()) {
        return Ok(());
    }
    let analysis = read_cached_tex_source_analysis(context.source_cache, &source_path)?;

    let original_svg_path_count = svg_paths.len();
    svg_paths.extend(
        analysis
            .graphicspath_entries
            .iter()
            .cloned()
            .map(PathBuf::from),
    );
    svg_paths.extend(analysis.svgpath_entries.iter().cloned().map(PathBuf::from));

    for svg in &analysis.includesvg_refs {
        let Some(settings) =
            svg_settings_for_include(inherited_settings, &analysis.svg_setup_refs, svg)
        else {
            continue;
        };
        if let Some(job) = svg_conversion_job_from_ref(
            context.doc_dir,
            context.out_dir,
            svg_paths,
            svg,
            &settings,
        )? {
            jobs.push(job);
        }
    }
    let inherited_dependency_settings =
        svg_settings_after_setups(inherited_settings, &analysis.svg_setup_refs).unwrap_or_else(
            || {
                let mut settings = inherited_settings.clone();
                settings.inkscape_enabled = false;
                settings
            },
        );

    for dependency in active_include_dependencies(&analysis, context.includeonly)
        .into_iter()
        .chain(analysis.input_dependencies.iter().cloned())
    {
        if let Some(path) = resolve_local_tex_source_dependency(context.doc_dir, &dependency)? {
            let dependency_svg_path_count = svg_paths.len();
            if let Some(local_graphic_path) = dependency.local_graphic_path {
                svg_paths.push(local_graphic_path);
            }
            collect_svg_conversion_jobs_from_source(
                context,
                &path,
                visited,
                svg_paths,
                &inherited_dependency_settings,
                jobs,
            )?;
            svg_paths.truncate(dependency_svg_path_count);
        }
    }

    svg_paths.truncate(original_svg_path_count);
    Ok(())
}

fn svg_conversion_job_from_ref(
    doc_dir: &Path,
    out_dir: &Path,
    svg_paths: &[PathBuf],
    svg: &IncludeSvgRef,
    settings: &SvgIncludeSettings,
) -> Result<Option<SvgConversionJob>> {
    if !settings.inkscape_enabled || !matches!(settings.format.as_str(), "pdf" | "png") {
        return Ok(None);
    }
    let Some(input_path) =
        resolve_svg_input(doc_dir, svg_paths, &svg.payload, &settings.source_extension)?
    else {
        return Ok(None);
    };
    let stem = input_path
        .file_stem()
        .context("SVG input file has no stem")?
        .to_string_lossy();
    let export_latex = settings.export_latex && settings.format == "pdf";
    let suffix = if export_latex {
        format!("_{}-tex", settings.source_extension)
    } else {
        format!("_{}-raw", settings.source_extension)
    };
    let output_name = settings.inkscape_name.as_deref().unwrap_or(stem.as_ref());
    let output_stem = format!("{output_name}{suffix}");
    let output_path = out_dir
        .join("svg-inkscape")
        .join(format!("{output_stem}.{}", settings.format));
    let output_tex_path = export_latex.then(|| output_path.with_extension("pdf_tex"));
    let state_name = external_tool_state_name(out_dir, &output_path);
    let state_path = out_dir.join(format!(".texpilot-{state_name}.svgstate.toml"));
    Ok(Some(SvgConversionJob {
        inkscape_executable: settings.inkscape_executable.clone(),
        input_path,
        output_path,
        output_tex_path,
        state_path,
        area: settings.area,
        dpi: settings.dpi.clone(),
    }))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SvgIncludeSettings {
    inkscape_enabled: bool,
    inkscape_executable: String,
    export_latex: bool,
    format: String,
    source_extension: String,
    area: SvgExportArea,
    inkscape_name: Option<String>,
    dpi: Option<String>,
}

fn default_svg_include_settings() -> SvgIncludeSettings {
    SvgIncludeSettings {
        inkscape_enabled: true,
        inkscape_executable: "inkscape".to_string(),
        export_latex: true,
        format: "pdf".to_string(),
        source_extension: "svg".to_string(),
        area: SvgExportArea::Drawing,
        inkscape_name: None,
        dpi: None,
    }
}

#[cfg(test)]
fn svg_include_settings(options: &str) -> Option<SvgIncludeSettings> {
    svg_include_settings_with_base(&default_svg_include_settings(), options)
}

#[cfg(test)]
fn svg_include_settings_with_base(
    base: &SvgIncludeSettings,
    options: &str,
) -> Option<SvgIncludeSettings> {
    let mut settings = base.clone();
    apply_svg_options(&mut settings, options).then_some(settings)
}

fn svg_settings_for_include(
    inherited_settings: &SvgIncludeSettings,
    setup_refs: &[SvgSetupRef],
    svg: &IncludeSvgRef,
) -> Option<SvgIncludeSettings> {
    let mut settings = inherited_settings.clone();
    for setup in setup_refs
        .iter()
        .filter(|setup| setup.command_start < svg.command_start)
    {
        if !apply_svg_options(&mut settings, &setup.options) {
            return None;
        }
    }
    apply_svg_options(&mut settings, &svg.options).then_some(settings)
}

fn svg_settings_after_setups(
    inherited_settings: &SvgIncludeSettings,
    setup_refs: &[SvgSetupRef],
) -> Option<SvgIncludeSettings> {
    let mut settings = inherited_settings.clone();
    for setup in setup_refs {
        if !apply_svg_options(&mut settings, &setup.options) {
            return None;
        }
    }
    Some(settings)
}

fn apply_svg_options(settings: &mut SvgIncludeSettings, options: &str) -> bool {
    for option in split_tex_keyvals(options) {
        let key = option.key.to_ascii_lowercase().replace([' ', '_'], "");
        let value = option.value.as_deref().map(str::trim);
        match key.as_str() {
            "inkscape" => {
                if let Some(value) = value {
                    match value.to_ascii_lowercase().as_str() {
                        "false" | "off" | "no" => settings.inkscape_enabled = false,
                        "true" | "on" | "yes" | "force" => settings.inkscape_enabled = true,
                        _ => {}
                    }
                }
            }
            "inkscapelatex" | "latex" | "tex" => {
                if let Some(value) = value {
                    match value.to_ascii_lowercase().as_str() {
                        "false" | "off" | "no" => settings.export_latex = false,
                        "true" | "on" | "yes" => settings.export_latex = true,
                        _ => {}
                    }
                }
            }
            "inkscapeformat" => {
                if let Some(value) = value {
                    settings.format = value.trim_start_matches('.').to_ascii_lowercase();
                }
            }
            "svgextension" | "extension" | "ext" => {
                let Some(value) = value else {
                    return false;
                };
                let extension = value.trim_start_matches('.').to_ascii_lowercase();
                if extension.is_empty()
                    || !extension
                        .chars()
                        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
                {
                    return false;
                }
                settings.source_extension = extension;
            }
            "inkscapearea" => {
                if let Some(value) = value {
                    match value.to_ascii_lowercase().as_str() {
                        "page" | "nocrop" => settings.area = SvgExportArea::Page,
                        "drawing" | "crop" => settings.area = SvgExportArea::Drawing,
                        _ => {}
                    }
                }
            }
            "inkscapedpi" | "inkscapedensity" => {
                let Some(value) = value else {
                    settings.dpi = None;
                    continue;
                };
                if value.eq_ignore_ascii_case(r"\relax") {
                    settings.dpi = None;
                    continue;
                }
                let dpi = value
                    .to_ascii_lowercase()
                    .trim()
                    .trim_end_matches("dpi")
                    .trim()
                    .to_string();
                if dpi.is_empty() || !dpi.chars().all(|ch| ch.is_ascii_digit()) {
                    return false;
                }
                settings.dpi = Some(dpi);
            }
            "inkscapepath" => {
                if let Some(value) = value
                    && !matches!(
                        value.to_ascii_lowercase().as_str(),
                        "basesubpath" | "basesubdir" | "jobsubpath" | "jobsubdir"
                    )
                {
                    return false;
                }
            }
            "inkscapename" => {
                let Some(value) = value else {
                    return false;
                };
                if safe_relative_path(Path::new(value)) {
                    settings.inkscape_name = Some(value.to_string());
                } else {
                    return false;
                }
            }
            "inkscapeexe" => {
                let Some(value) = value else {
                    return false;
                };
                if safe_command_name(value) {
                    settings.inkscape_executable = value.to_string();
                } else {
                    return false;
                }
            }
            "inkscapeopt" => return false,
            _ => {}
        }
    }
    true
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TexKeyVal {
    key: String,
    value: Option<String>,
}

fn split_tex_keyvals(options: &str) -> Vec<TexKeyVal> {
    split_tex_top_level(options, ',')
        .into_iter()
        .filter_map(|option| {
            let option = option.trim();
            if option.is_empty() {
                return None;
            }
            let mut depth = 0_u32;
            for (index, ch) in option.char_indices() {
                match ch {
                    '{' | '[' => depth += 1,
                    '}' | ']' => depth = depth.saturating_sub(1),
                    '=' if depth == 0 => {
                        let key = option[..index].trim().to_string();
                        let value = strip_outer_tex_braces(option[index + 1..].trim()).to_string();
                        return (!key.is_empty()).then_some(TexKeyVal {
                            key,
                            value: Some(value),
                        });
                    }
                    _ => {}
                }
            }
            Some(TexKeyVal {
                key: option.to_string(),
                value: None,
            })
        })
        .collect()
}

fn split_tex_top_level(source: &str, delimiter: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0_u32;
    let mut start = 0;
    for (index, ch) in source.char_indices() {
        match ch {
            '{' | '[' => depth += 1,
            '}' | ']' => depth = depth.saturating_sub(1),
            _ if ch == delimiter && depth == 0 => {
                parts.push(source[start..index].to_string());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    parts.push(source[start..].to_string());
    parts
}

fn strip_outer_tex_braces(value: &str) -> &str {
    if value.starts_with('{')
        && value.ends_with('}')
        && let Some((_payload, end)) = balanced_braced_payload_at(value, 0)
        && end == value.len()
    {
        return value[1..value.len() - 1].trim();
    }
    value
}

fn resolve_svg_input(
    doc_dir: &Path,
    svg_paths: &[PathBuf],
    payload: &str,
    source_extension: &str,
) -> Result<Option<PathBuf>> {
    let requested = Path::new(payload);
    if !safe_relative_path(requested) {
        return Ok(None);
    }
    if requested
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| !extension.eq_ignore_ascii_case(source_extension))
    {
        return Ok(None);
    }
    let search_dirs = std::iter::once(PathBuf::new())
        .chain(svg_paths.iter().cloned())
        .collect::<Vec<_>>();
    for directory in search_dirs {
        let candidate = doc_dir.join(directory).join(requested);
        if candidate.is_file() {
            return Ok(Some(candidate));
        }
        if candidate.extension().is_none() {
            let svg_candidate = candidate.with_extension(source_extension);
            if svg_candidate.is_file() {
                return Ok(Some(svg_candidate));
            }
        }
    }
    for directory in std::iter::once(PathBuf::new()).chain(svg_paths.iter().cloned()) {
        let payload = directory.join(requested).to_string_lossy().to_string();
        if let Some(path) = resolve_kpathsea_input(doc_dir, &payload, source_extension)? {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

#[cfg(test)]
fn includegraphics_payloads(source: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    includegraphics_payloads_stripped(&source)
}

fn includegraphics_payloads_stripped(source: &str) -> Vec<String> {
    let command = r"\includegraphics";
    let mut payloads = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find(command) {
        let command_start = cursor + offset;
        let after_command = command_start + command.len();
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let mut payload_start = skip_tex_whitespace(source, after_command);
        if source[payload_start..].starts_with('*') {
            payload_start = skip_tex_whitespace(source, payload_start + 1);
        }
        while source[payload_start..].starts_with('[') {
            if let Some(argument_end) = bracketed_tex_argument_end(source, payload_start) {
                payload_start = skip_tex_whitespace(source, argument_end);
            } else {
                break;
            };
        }
        let Some((payload, payload_end)) = balanced_braced_payload_at(source, payload_start) else {
            cursor = after_command;
            continue;
        };
        if !payload.is_empty() {
            payloads.push(payload);
        }
        cursor = payload_end;
    }
    payloads
}

const MAX_ANIMATEGRAPHICS_FRAME_DEPENDENCIES: u32 = 20_000;

#[derive(Debug, Clone, Eq, PartialEq)]
struct AnimateGraphicsRef {
    prefix: String,
    first: String,
    last: String,
}

#[cfg(test)]
fn animategraphics_refs(source: &str) -> Vec<AnimateGraphicsRef> {
    let source = tex_comment_stripped_source(source);
    animategraphics_refs_stripped(&source)
}

fn animategraphics_refs_stripped(source: &str) -> Vec<AnimateGraphicsRef> {
    let command = r"\animategraphics";
    let mut refs = Vec::new();
    let mut cursor = 0;
    'scan: while let Some(offset) = source[cursor..].find(command) {
        let after_command = cursor + offset + command.len();
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let mut argument_start = skip_tex_whitespace(source, after_command);
        while source[argument_start..].starts_with('[') {
            if let Some(argument_end) = bracketed_tex_argument_end(source, argument_start) {
                argument_start = skip_tex_whitespace(source, argument_end);
            } else {
                cursor = after_command;
                continue 'scan;
            }
        }

        let mut arguments = Vec::with_capacity(4);
        let mut argument_end = after_command;
        for _ in 0..4 {
            let Some((payload, payload_end)) = balanced_braced_payload_at(source, argument_start)
            else {
                arguments.clear();
                break;
            };
            arguments.push(payload.trim().to_string());
            argument_end = payload_end;
            argument_start = skip_tex_whitespace(source, payload_end);
        }

        if arguments.len() == 4 && !arguments[1].is_empty() {
            refs.push(AnimateGraphicsRef {
                prefix: arguments[1].clone(),
                first: arguments[2].clone(),
                last: arguments[3].clone(),
            });
        }
        cursor = argument_end;
    }
    refs
}

fn animategraphics_frame_payloads(reference: &AnimateGraphicsRef) -> Vec<String> {
    let Some((first, last, width)) = animategraphics_frame_range(reference) else {
        return Vec::new();
    };
    (first..=last)
        .map(|frame| {
            if let Some(width) = width {
                format!("{}{:0width$}", reference.prefix, frame, width = width)
            } else {
                format!("{}{}", reference.prefix, frame)
            }
        })
        .collect()
}

fn animategraphics_frame_range(
    reference: &AnimateGraphicsRef,
) -> Option<(u32, u32, Option<usize>)> {
    let first_text = reference.first.trim();
    let last_text = reference.last.trim();
    if !first_text.chars().all(|ch| ch.is_ascii_digit())
        || !last_text.chars().all(|ch| ch.is_ascii_digit())
    {
        return None;
    }

    let first = first_text.parse::<u32>().ok()?;
    let last = last_text.parse::<u32>().ok()?;
    let frame_count = last.checked_sub(first)?.checked_add(1)?;
    if frame_count > MAX_ANIMATEGRAPHICS_FRAME_DEPENDENCIES {
        return None;
    }

    let zero_padded = (first_text.len() > 1 && first_text.starts_with('0'))
        || (last_text.len() > 1 && last_text.starts_with('0'));
    let width = zero_padded.then_some(first_text.len().max(last_text.len()));
    Some((first, last, width))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct IncludeSvgRef {
    command_start: usize,
    options: String,
    payload: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SvgSetupRef {
    command_start: usize,
    options: String,
}

#[cfg(test)]
fn includesvg_refs(source: &str) -> Vec<IncludeSvgRef> {
    let source = tex_comment_stripped_source(source);
    includesvg_refs_stripped(&source)
}

fn includesvg_refs_stripped(source: &str) -> Vec<IncludeSvgRef> {
    let command = r"\includesvg";
    let mut refs = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find(command) {
        let command_start = cursor + offset;
        let after_command = command_start + command.len();
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let mut payload_start = skip_tex_whitespace(source, after_command);
        let options = if source[payload_start..].starts_with('[') {
            if let Some((options, options_end)) =
                bracketed_tex_argument_payload_at(source, payload_start)
            {
                payload_start = skip_tex_whitespace(source, options_end);
                options
            } else {
                cursor = after_command;
                continue;
            }
        } else {
            String::new()
        };
        let Some((payload, payload_end)) = balanced_braced_payload_at(source, payload_start) else {
            cursor = after_command;
            continue;
        };
        if !payload.is_empty() {
            refs.push(IncludeSvgRef {
                command_start,
                options,
                payload,
            });
        }
        cursor = payload_end;
    }
    refs
}

#[cfg(test)]
fn svg_setup_refs(source: &str) -> Vec<SvgSetupRef> {
    let source = tex_comment_stripped_source(source);
    svg_setup_refs_stripped(&source)
}

fn svg_setup_refs_stripped(source: &str) -> Vec<SvgSetupRef> {
    let mut refs = Vec::new();
    for command in ["svgsetup", "setsvg"] {
        refs.extend(
            tex_command_balanced_payload_refs_stripped(source, command)
                .into_iter()
                .map(|payload_ref| SvgSetupRef {
                    command_start: payload_ref.command_start,
                    options: payload_ref.payload,
                }),
        );
    }
    refs.sort_by(|left, right| left.command_start.cmp(&right.command_start));
    refs
}

fn bracketed_tex_argument_payload_at(source: &str, open: usize) -> Option<(String, usize)> {
    let end = bracketed_tex_argument_end(source, open)?;
    Some((source[open + 1..end - 1].trim().to_string(), end))
}

fn bracketed_tex_argument_end(source: &str, open: usize) -> Option<usize> {
    if !source[open..].starts_with('[') {
        return None;
    }
    let mut depth = 0_u32;
    for (offset, ch) in source[open..].char_indices() {
        let index = open + offset;
        match ch {
            '[' => depth += 1,
            ']' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index + ch.len_utf8());
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
fn graphicspath_entries(source: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    graphicspath_entries_stripped(&source)
}

fn graphicspath_entries_stripped(source: &str) -> Vec<String> {
    braced_path_list_entries_stripped(source, "graphicspath")
}

#[cfg(test)]
fn declared_graphics_extensions(source: &str) -> Option<Vec<String>> {
    let source = tex_comment_stripped_source(source);
    declared_graphics_extensions_stripped(&source)
}

fn declared_graphics_extensions_stripped(source: &str) -> Option<Vec<String>> {
    tex_command_balanced_payloads_stripped(source, "DeclareGraphicsExtensions")
        .into_iter()
        .rev()
        .filter_map(|payload| {
            let extensions = split_tex_top_level(&payload, ',')
                .into_iter()
                .filter_map(|entry| {
                    let extension = entry
                        .trim()
                        .trim_start_matches('.')
                        .trim()
                        .to_ascii_lowercase();
                    (!extension.is_empty()
                        && extension.chars().all(|ch| ch.is_ascii_alphanumeric()))
                    .then_some(extension)
                })
                .collect::<Vec<_>>();
            (!extensions.is_empty()).then_some(extensions)
        })
        .next()
}

#[cfg(test)]
fn svgpath_entries(source: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    svgpath_entries_stripped(&source)
}

fn svgpath_entries_stripped(source: &str) -> Vec<String> {
    braced_path_list_entries_stripped(source, "svgpath")
}

fn braced_path_list_entries_stripped(source: &str, command: &str) -> Vec<String> {
    let mut entries = Vec::new();
    for payload in tex_command_balanced_payloads_stripped(source, command) {
        let mut cursor = 0;
        let mut found_nested_entry = false;
        while cursor < payload.len() {
            cursor = skip_tex_whitespace(&payload, cursor);
            if !payload[cursor..].starts_with('{') {
                cursor += payload[cursor..]
                    .chars()
                    .next()
                    .map(char::len_utf8)
                    .unwrap_or(1);
                continue;
            }
            let Some((entry, end)) = balanced_braced_payload_at(&payload, cursor) else {
                break;
            };
            if !entry.is_empty()
                && safe_relative_path(Path::new(&entry))
                && !entries.contains(&entry)
            {
                entries.push(entry);
                found_nested_entry = true;
            }
            cursor = end;
        }
        if !found_nested_entry {
            let entry = payload.trim();
            if !entry.is_empty()
                && safe_relative_path(Path::new(entry))
                && !entries.iter().any(|existing| existing == entry)
            {
                entries.push(entry.to_string());
            }
        }
    }
    entries
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct TexPayloadRef {
    command_start: usize,
    payload: String,
}

fn tex_command_balanced_payloads(source: &str, command: &str) -> Vec<String> {
    let source = tex_comment_stripped_source(source);
    tex_command_balanced_payloads_stripped(&source, command)
}

fn tex_command_payloads_stripped(source: &str, command: &str) -> Vec<String> {
    tex_command_balanced_payloads_stripped(source, command)
}

fn tex_command_balanced_payloads_stripped(source: &str, command: &str) -> Vec<String> {
    tex_command_balanced_payload_refs_stripped(source, command)
        .into_iter()
        .map(|payload_ref| payload_ref.payload)
        .collect()
}

fn tex_command_optional_braced_payloads_stripped(source: &str, command: &str) -> Vec<String> {
    let command = format!(r"\{command}");
    let mut payloads = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find(&command) {
        let command_start = cursor + offset;
        let after_command = command_start + command.len();
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let mut payload_start = skip_tex_whitespace(source, after_command);
        while source[payload_start..].starts_with('[') {
            if let Some(argument_end) = bracketed_tex_argument_end(source, payload_start) {
                payload_start = skip_tex_whitespace(source, argument_end);
            } else {
                break;
            }
        }
        let Some((payload, payload_end)) = balanced_braced_payload_at(source, payload_start) else {
            cursor = after_command;
            continue;
        };
        if !payload.is_empty() {
            payloads.push(payload);
        }
        cursor = payload_end;
    }
    payloads
}

fn tex_command_optional_star_braced_payloads_stripped(source: &str, command: &str) -> Vec<String> {
    let command = format!(r"\{command}");
    let mut payloads = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find(&command) {
        let command_start = cursor + offset;
        let after_command = command_start + command.len();
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let mut payload_start = skip_tex_whitespace(source, after_command);
        if source[payload_start..].starts_with('*') {
            payload_start = skip_tex_whitespace(source, payload_start + 1);
        }
        while source[payload_start..].starts_with('[') {
            if let Some(argument_end) = bracketed_tex_argument_end(source, payload_start) {
                payload_start = skip_tex_whitespace(source, argument_end);
            } else {
                break;
            }
        }
        let Some((payload, payload_end)) = balanced_braced_payload_at(source, payload_start) else {
            cursor = after_command;
            continue;
        };
        if !payload.is_empty() {
            payloads.push(payload);
        }
        cursor = payload_end;
    }
    payloads
}

fn tex_command_optional_nth_braced_payloads_stripped(
    source: &str,
    command: &str,
    nth: usize,
) -> Vec<String> {
    if nth == 0 {
        return Vec::new();
    }

    let command = format!(r"\{command}");
    let mut payloads = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find(&command) {
        let command_start = cursor + offset;
        let after_command = command_start + command.len();
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }

        let mut payload_start = skip_tex_whitespace(source, after_command);
        while source[payload_start..].starts_with('[') {
            if let Some(argument_end) = bracketed_tex_argument_end(source, payload_start) {
                payload_start = skip_tex_whitespace(source, argument_end);
            } else {
                break;
            }
        }
        let mut payload_end = after_command;
        let mut found = None;
        for index in 1..=nth {
            let Some((payload, end)) = balanced_braced_payload_at(source, payload_start) else {
                found = None;
                break;
            };
            payload_end = end;
            if index == nth {
                found = Some(payload);
                break;
            }
            payload_start = skip_tex_whitespace(source, end);
        }
        if let Some(payload) = found
            && !payload.is_empty()
        {
            payloads.push(payload);
        }
        cursor = payload_end;
    }
    payloads
}

fn tex_command_balanced_payload_refs_stripped(source: &str, command: &str) -> Vec<TexPayloadRef> {
    let command = format!(r"\{command}");
    let mut refs = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = source[cursor..].find(&command) {
        let command_start = cursor + offset;
        let after_command = command_start + command.len();
        if source[after_command..]
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            cursor = after_command;
            continue;
        }
        let payload_start = skip_tex_whitespace(source, after_command);
        let Some((payload, payload_end)) = balanced_braced_payload_at(source, payload_start) else {
            cursor = after_command;
            continue;
        };
        if !payload.is_empty() {
            refs.push(TexPayloadRef {
                command_start,
                payload,
            });
        }
        cursor = payload_end;
    }
    refs
}

fn tex_comment_stripped_source(source: &str) -> String {
    let mut stripped = String::with_capacity(source.len());
    for line in source.lines() {
        push_inline_literal_masked_source(strip_tex_comment(line), &mut stripped);
        stripped.push('\n');
    }
    stripped
}

fn push_inline_literal_masked_source(line: &str, output: &mut String) {
    let bytes = line.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor] == b'\\'
            && let Some(end) = inline_literal_span_end_bytes(bytes, cursor)
        {
            for _ in cursor..end {
                output.push(' ');
            }
            cursor = end;
            continue;
        }

        let ch = line[cursor..]
            .chars()
            .next()
            .expect("cursor should stay on a char boundary");
        output.push(ch);
        cursor += ch.len_utf8();
    }
}

fn resolve_eps_graphic_input(
    doc_dir: &Path,
    graphic_paths: &[PathBuf],
    payload: &str,
    declared_extensions: Option<&[String]>,
) -> Result<Option<(PathBuf, bool)>> {
    let requested = Path::new(payload);
    if !safe_relative_path(requested) {
        return Ok(None);
    }

    let search_dirs = std::iter::once(PathBuf::new())
        .chain(graphic_paths.iter().cloned())
        .collect::<Vec<_>>();
    let extension = requested
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);

    if let Some(extension) = extension {
        if extension != "eps" {
            return Ok(None);
        }
        for directory in search_dirs {
            let candidate = doc_dir.join(directory).join(requested);
            if candidate.is_file() {
                return Ok(Some((candidate, true)));
            }
        }
        return Ok(resolve_kpathsea_input(doc_dir, payload, "eps")?.map(|path| (path, true)));
    }

    if let Some(declared_extensions) = declared_extensions {
        for directory in &search_dirs {
            let base = doc_dir.join(directory).join(requested);
            for declared_extension in declared_extensions {
                let candidate = base.with_extension(declared_extension);
                if !candidate.is_file() {
                    continue;
                }
                return Ok((declared_extension == "eps").then_some((candidate, false)));
            }
        }
        for directory in std::iter::once(PathBuf::new()).chain(graphic_paths.iter().cloned()) {
            let payload = directory.join(requested).to_string_lossy().to_string();
            for declared_extension in declared_extensions {
                if let Some(path) = resolve_kpathsea_input(doc_dir, &payload, declared_extension)? {
                    return Ok((declared_extension == "eps").then_some((path, false)));
                }
            }
        }
        return Ok(None);
    }

    for directory in &search_dirs {
        let base = doc_dir.join(directory).join(requested);
        for extension in ["pdf", "png", "jpg", "jpeg", "mps"] {
            if base.with_extension(extension).is_file() {
                return Ok(None);
            }
        }
    }
    for directory in search_dirs {
        let candidate = doc_dir
            .join(directory)
            .join(requested)
            .with_extension("eps");
        if candidate.is_file() {
            return Ok(Some((candidate, false)));
        }
    }
    for directory in std::iter::once(PathBuf::new()).chain(graphic_paths.iter().cloned()) {
        let payload = directory.join(requested).to_string_lossy().to_string();
        for extension in ["pdf", "png", "jpg", "jpeg", "mps"] {
            if resolve_kpathsea_input(doc_dir, &payload, extension)?.is_some() {
                return Ok(None);
            }
        }
    }
    for directory in std::iter::once(PathBuf::new()).chain(graphic_paths.iter().cloned()) {
        let payload = directory.join(requested).to_string_lossy().to_string();
        if let Some(path) = resolve_kpathsea_input(doc_dir, &payload, "eps")? {
            return Ok(Some((path, false)));
        }
    }
    Ok(None)
}

fn eps_conversion_output_path(
    doc_dir: &Path,
    out_dir: &Path,
    input_path: &Path,
    explicit_eps: bool,
) -> Result<PathBuf> {
    let relative = input_path.strip_prefix(doc_dir).unwrap_or(input_path);
    let parent = relative.parent().unwrap_or_else(|| Path::new(""));
    let stem = input_path
        .file_stem()
        .context("EPS input file has no stem")?
        .to_string_lossy();
    let file_name = if explicit_eps {
        format!("{stem}-eps-converted-to.pdf")
    } else {
        format!("{stem}.pdf")
    };
    Ok(out_dir.join(parent).join(file_name))
}

fn safe_relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, std::path::Component::Normal(_)))
}

fn safe_command_name(value: &str) -> bool {
    !value.is_empty()
        && !value.contains(['/', '\\'])
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.')
}

fn eps_conversion_signature(job: &EpsConversionJob) -> Result<String> {
    let _ = fs::metadata(&job.input_path)
        .with_context(|| format!("failed to stat EPS input {}", job.input_path.display()))?;
    Ok(format!(
        "%% texpilot EPS conversion\nepstopdf\n{}\n",
        job.output_path.display()
    ))
}

fn svg_conversion_signature(job: &SvgConversionJob) -> Result<String> {
    let _ = fs::metadata(&job.input_path)
        .with_context(|| format!("failed to stat SVG input {}", job.input_path.display()))?;
    let mut signature = String::from("%% texpilot SVG conversion\n");
    signature.push_str(&job.inkscape_executable);
    signature.push('\n');
    signature.push_str(match job.area {
        SvgExportArea::Drawing => "--export-area-drawing\n",
        SvgExportArea::Page => "--export-area-page\n",
    });
    if let Some(dpi) = &job.dpi {
        signature.push_str("--export-dpi=");
        signature.push_str(dpi);
        signature.push('\n');
    }
    if job.output_tex_path.is_some() {
        signature.push_str("--export-latex\n");
    }
    for output_path in job.output_paths() {
        signature.push_str(&output_path.display().to_string());
        signature.push('\n');
    }
    Ok(signature)
}

fn asymptote_jobs_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Vec<AsymptoteJob>> {
    let fls_path = out_dir.join(format!("{job_name}.fls"));
    let mut paths = if fls_path.exists() {
        recorded_outputs(&fls_path, doc_dir)?
            .into_iter()
            .filter(|path| path_extension_is_any(path, &["asy"]))
            .filter(|path| path.exists())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    paths.sort();
    paths.dedup();
    let mut jobs = Vec::new();
    for input_path in paths {
        let source = fs::read_to_string(&input_path)
            .with_context(|| format!("failed to read Asymptote file {}", input_path.display()))?;
        let output_path = input_path.with_extension("pdf");
        let input_paths = asymptote_input_paths(&input_path, &source);
        let state_name = external_tool_state_name(out_dir, &input_path);
        let state_path = out_dir.join(format!(".texpilot-{state_name}.asystate.toml"));
        jobs.push(AsymptoteJob {
            input_path,
            output_path,
            input_paths,
            state_path,
        });
    }
    Ok(jobs)
}

fn external_tool_state_name(out_dir: &Path, input_path: &Path) -> String {
    input_path
        .strip_prefix(out_dir)
        .unwrap_or(input_path)
        .to_string_lossy()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn asymptote_signature(job: &AsymptoteJob) -> Result<String> {
    let source = fs::read_to_string(&job.input_path)
        .with_context(|| format!("failed to read Asymptote file {}", job.input_path.display()))?;
    let mut signature = source;
    if let Some(pre_path) = asymptote_preamble_path(&job.input_path) {
        let bytes = fs::read(&pre_path)
            .with_context(|| format!("failed to read Asymptote preamble {}", pre_path.display()))?;
        signature.push_str("\n%% texpilot asymptote preamble ");
        signature.push_str(&pre_path.display().to_string());
        signature.push('\n');
        signature.push_str(&String::from_utf8_lossy(&bytes));
    }
    Ok(signature)
}

fn asymptote_preamble_path(input_path: &Path) -> Option<PathBuf> {
    let file_stem = input_path.file_stem()?.to_string_lossy();
    let job_stem = file_stem
        .rsplit_once('-')
        .map_or(file_stem.as_ref(), |(head, _)| head);
    let pre_path = input_path.with_file_name(format!("{job_stem}.pre"));
    pre_path.exists().then_some(pre_path)
}

fn asymptote_input_paths(input_path: &Path, source: &str) -> Vec<PathBuf> {
    let base_dir = input_path.parent().unwrap_or_else(|| Path::new(""));
    let source = strip_asymptote_comments(source);
    let mut paths = Vec::new();
    for statement in source.split(';') {
        let Some(name) = asymptote_input_statement_name(statement) else {
            continue;
        };
        if let Some(path) = asymptote_local_input_path(base_dir, name) {
            paths.push(path);
        }
    }
    paths.sort();
    paths.dedup();
    paths
}

fn asymptote_input_statement_name(statement: &str) -> Option<&str> {
    let statement = statement.trim_start();
    for keyword in ["import", "include", "access"] {
        if let Some(rest) = keyword_argument(statement, keyword) {
            return asymptote_module_name(rest.trim_start()).map(|(name, _)| name);
        }
    }
    let rest = keyword_argument(statement, "from")?.trim_start();
    let (name, consumed) = asymptote_module_name(rest)?;
    keyword_argument(rest[consumed..].trim_start(), "import")?;
    Some(name)
}

fn asymptote_module_name(value: &str) -> Option<(&str, usize)> {
    if value.is_empty() {
        return None;
    }
    let mut chars = value.char_indices();
    let (_, first) = chars.next()?;
    if first == '"' || first == '\'' {
        for (index, ch) in chars {
            if ch == first {
                return Some((&value[first.len_utf8()..index], index + ch.len_utf8()));
            }
        }
        return None;
    }
    let end = value
        .find(|ch: char| ch.is_ascii_whitespace() || matches!(ch, ';' | ',' | '(' | ')'))
        .unwrap_or(value.len());
    (end > 0).then_some((&value[..end], end))
}

fn asymptote_local_input_path(base_dir: &Path, name: &str) -> Option<PathBuf> {
    if name.is_empty() || name.contains("://") || name.starts_with('|') {
        return None;
    }
    let path = Path::new(name);
    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_dir.join(path)
    };
    if candidate.is_file() {
        return Some(candidate);
    }
    if candidate.extension().is_none() {
        let with_asy = candidate.with_extension("asy");
        if with_asy.is_file() {
            return Some(with_asy);
        }
    }
    None
}

fn strip_asymptote_comments(source: &str) -> String {
    let mut stripped = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    let mut quote = None;
    let mut escaped = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while let Some(ch) = chars.next() {
        if in_line_comment {
            if ch == '\n' {
                in_line_comment = false;
                stripped.push(ch);
            }
            continue;
        }
        if in_block_comment {
            if ch == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_block_comment = false;
            } else if ch == '\n' {
                stripped.push(ch);
            }
            continue;
        }
        if let Some(active) = quote {
            stripped.push(ch);
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == active {
                quote = None;
            }
            continue;
        }
        if ch == '"' || ch == '\'' {
            quote = Some(ch);
            stripped.push(ch);
            continue;
        }
        if ch == '/' && chars.peek() == Some(&'/') {
            chars.next();
            in_line_comment = true;
            continue;
        }
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_block_comment = true;
            continue;
        }
        stripped.push(ch);
    }
    stripped
}

fn keyword_argument<'a>(value: &'a str, keyword: &str) -> Option<&'a str> {
    let rest = value.strip_prefix(keyword)?;
    rest.starts_with(char::is_whitespace).then_some(rest)
}

fn pythontex_jobs_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Vec<PythontexJob>> {
    let fls_path = out_dir.join(format!("{job_name}.fls"));
    let mut paths = if fls_path.exists() {
        recorded_outputs(&fls_path, doc_dir)?
            .into_iter()
            .filter(|path| path_extension_is_any(path, &["pytxcode"]))
            .filter(|path| path.exists())
            .collect::<Vec<_>>()
    } else {
        let candidate = out_dir.join(format!("{job_name}.pytxcode"));
        candidate
            .exists()
            .then_some(candidate)
            .into_iter()
            .collect()
    };
    paths.sort();
    paths.dedup();
    paths
        .into_iter()
        .map(|code_path| pythontex_job(out_dir, code_path))
        .collect()
}

fn pythontex_job(out_dir: &Path, code_path: PathBuf) -> Result<PythontexJob> {
    let command_arg = code_path
        .file_stem()
        .context("PythonTeX code file has no stem")?
        .to_string_lossy()
        .to_string();
    let source = fs::read_to_string(&code_path)
        .with_context(|| format!("failed to read PythonTeX code file {}", code_path.display()))?;
    let output_dir_name = pythontex_output_dir_name(&source, &command_arg);
    let code_dir = code_path
        .parent()
        .context("PythonTeX code file has no parent directory")?;
    let output_dir = code_dir.join(output_dir_name);
    let macro_path = output_dir.join(format!("{command_arg}.pytxmcr"));
    let pygments_path = output_dir.join(format!("{command_arg}.pytxpyg"));
    let mut output_paths = vec![macro_path.clone()];
    if pythontex_pygments_enabled(&source) {
        output_paths.push(pygments_path.clone());
    }
    let state_name = external_tool_state_name(out_dir, &code_path);
    let state_path = out_dir.join(format!(".texpilot-{state_name}.pythontexstate.toml"));
    Ok(PythontexJob {
        code_path,
        command_arg,
        macro_path,
        output_paths,
        state_path,
    })
}

fn pythontex_output_dir_name(source: &str, command_arg: &str) -> String {
    source
        .lines()
        .find_map(|line| {
            line.strip_prefix("outputdir=")
                .map(str::trim)
                .filter(|value| !value.is_empty())
        })
        .map(str::to_string)
        .unwrap_or_else(|| format!("pythontex-files-{command_arg}"))
}

fn pythontex_pygments_enabled(source: &str) -> bool {
    !source
        .lines()
        .find_map(|line| line.strip_prefix("pygments=").map(str::trim))
        .is_some_and(|value| value.eq_ignore_ascii_case("false"))
}

fn default_pythontex_workingdir() -> String {
    ".".to_string()
}

fn pythontex_dependency_paths(job: &PythontexJob) -> Result<Vec<PathBuf>> {
    let data_path = pythontex_data_path(job);
    if !data_path.is_file() {
        return Ok(Vec::new());
    }
    let report = pythontex_dependency_report(&data_path)?;
    let code_dir = job
        .code_path
        .parent()
        .context("PythonTeX code file has no parent directory")?;
    let working_dir = pythontex_working_dir(code_dir, &report.workingdir);
    let mut paths = Vec::new();
    for dependency in report.dependencies {
        if let Some(path) = pythontex_local_dependency_path(&working_dir, &dependency) {
            paths.push(path);
        }
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn pythontex_data_path(job: &PythontexJob) -> PathBuf {
    job.macro_path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .join("pythontex_data.pkl")
}

fn pythontex_dependency_report(data_path: &Path) -> Result<PythontexDependencyReport> {
    let script = r#"
import json
import pickle
import sys

with open(sys.argv[1], "rb") as handle:
    data = pickle.load(handle)

settings = data.get("settings") or {}
dependencies = []
for session_dependencies in (data.get("dependencies") or {}).values():
    if isinstance(session_dependencies, dict):
        dependencies.extend(
            path for path in session_dependencies.keys() if isinstance(path, str)
        )

pygments_files = data.get("pygments_files") or {}
if isinstance(pygments_files, dict):
    dependencies.extend(path for path in pygments_files.keys() if isinstance(path, str))

print(json.dumps({
    "workingdir": str(settings.get("workingdir") or "."),
    "dependencies": sorted(set(dependencies)),
}))
"#;
    let mut launch_errors = Vec::new();
    for interpreter in ["python3", "python"] {
        match Command::new(interpreter)
            .arg("-c")
            .arg(script)
            .arg(data_path)
            .output()
        {
            Ok(output) if output.status.success() => {
                return serde_json::from_slice(&output.stdout).with_context(|| {
                    format!(
                        "failed to parse PythonTeX dependency metadata from {}",
                        data_path.display()
                    )
                });
            }
            Ok(output) => {
                launch_errors.push(format!(
                    "{interpreter} exited with status {}: {}",
                    output.status,
                    String::from_utf8_lossy(&output.stderr).trim()
                ));
            }
            Err(error) => {
                launch_errors.push(format!("{interpreter}: {error}"));
            }
        }
    }
    bail!(
        "failed to inspect PythonTeX dependency metadata {} ({})",
        data_path.display(),
        launch_errors.join("; ")
    )
}

fn pythontex_working_dir(code_dir: &Path, workingdir: &str) -> PathBuf {
    let expanded = expand_home_path(workingdir);
    if expanded.is_absolute() {
        expanded
    } else {
        code_dir.join(expanded)
    }
}

fn pythontex_local_dependency_path(base_dir: &Path, dependency: &str) -> Option<PathBuf> {
    if dependency.is_empty() || dependency.contains("://") {
        return None;
    }
    let expanded = expand_home_path(dependency);
    let path = if expanded.is_absolute() {
        expanded
    } else {
        base_dir.join(expanded)
    };
    path.is_file().then_some(path)
}

fn expand_home_path(value: &str) -> PathBuf {
    if (value == "~" || value.starts_with("~/") || value.starts_with("~\\"))
        && let Some(home) = home_dir()
    {
        if value == "~" {
            return home;
        }
        return home.join(&value[2..]);
    }
    PathBuf::from(value)
}

fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
}

fn pythontex_signature(job: &PythontexJob) -> Result<String> {
    let mut signature = fs::read_to_string(&job.code_path).with_context(|| {
        format!(
            "failed to read PythonTeX code file {}",
            job.code_path.display()
        )
    })?;
    signature.push_str("\n%% texpilot pythontex command\n");
    signature.push_str(&job.command_arg);
    signature.push('\n');
    Ok(signature)
}

fn metapost_jobs_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Vec<MetapostJob>> {
    let fls_path = out_dir.join(format!("{job_name}.fls"));
    let mut paths = if fls_path.exists() {
        recorded_outputs(&fls_path, doc_dir)?
            .into_iter()
            .filter(|path| path_extension_is_any(path, &["mp"]))
            .filter(|path| path.exists())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    paths.sort();
    paths.dedup();
    paths
        .into_iter()
        .map(|input_path| metapost_job(out_dir, input_path))
        .collect()
}

fn metapost_job(out_dir: &Path, input_path: PathBuf) -> Result<MetapostJob> {
    let source = fs::read_to_string(&input_path)
        .with_context(|| format!("failed to read MetaPost file {}", input_path.display()))?;
    let output_paths = metapost_output_paths(&input_path, &source)?;
    let input_paths = metapost_input_paths(&input_path, &source);
    let state_name = external_tool_state_name(out_dir, &input_path);
    let state_path = out_dir.join(format!(".texpilot-{state_name}.mpoststate.toml"));
    Ok(MetapostJob {
        input_path,
        output_paths,
        input_paths,
        state_path,
    })
}

fn metapost_output_paths(input_path: &Path, source: &str) -> Result<Vec<PathBuf>> {
    let parent = input_path
        .parent()
        .context("MetaPost file has no parent directory")?;
    let stem = input_path
        .file_stem()
        .context("MetaPost file has no stem")?
        .to_string_lossy();
    let mut numbers = Vec::new();
    for command in ["beginfig", "beginchar"] {
        let pattern = format!("{command}(");
        let mut cursor = 0;
        while let Some(offset) = source[cursor..].find(&pattern) {
            let number_start = cursor + offset + pattern.len();
            let number = source[number_start..]
                .chars()
                .take_while(|ch| ch.is_ascii_digit())
                .collect::<String>();
            if !number.is_empty() {
                numbers.push(number);
            }
            cursor = number_start;
        }
    }
    numbers.sort();
    numbers.dedup();
    Ok(numbers
        .into_iter()
        .map(|number| parent.join(format!("{stem}.{number}")))
        .collect())
}

fn metapost_input_paths(input_path: &Path, source: &str) -> Vec<PathBuf> {
    let base_dir = input_path.parent().unwrap_or_else(|| Path::new(""));
    let mut paths = Vec::new();
    for line in source.lines() {
        let line = strip_metapost_comment(line).trim_start();
        let Some(rest) = line.strip_prefix("input") else {
            continue;
        };
        if !rest.starts_with(char::is_whitespace) {
            continue;
        }
        let Some(name) = metapost_input_name(rest.trim_start()) else {
            continue;
        };
        if let Some(path) = metapost_local_input_path(base_dir, name) {
            paths.push(path);
        }
    }
    paths.sort();
    paths.dedup();
    paths
}

fn metapost_input_name(value: &str) -> Option<&str> {
    if value.is_empty() {
        return None;
    }
    let mut chars = value.char_indices();
    let (_, first) = chars.next()?;
    if first == '"' || first == '\'' {
        for (index, ch) in chars {
            if ch == first {
                return Some(&value[first.len_utf8()..index]);
            }
        }
        return None;
    }
    let end = value
        .find(|ch: char| ch.is_ascii_whitespace() || matches!(ch, ';' | ','))
        .unwrap_or(value.len());
    (end > 0).then_some(&value[..end])
}

fn metapost_local_input_path(base_dir: &Path, name: &str) -> Option<PathBuf> {
    if name.is_empty() || name.contains("://") || name.starts_with('|') {
        return None;
    }
    let path = Path::new(name);
    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_dir.join(path)
    };
    if candidate.is_file() {
        return Some(candidate);
    }
    if candidate.extension().is_none() {
        let with_mp = candidate.with_extension("mp");
        if with_mp.is_file() {
            return Some(with_mp);
        }
    }
    None
}

fn strip_metapost_comment(line: &str) -> &str {
    let mut quote = None;
    for (index, ch) in line.char_indices() {
        if let Some(active) = quote {
            if ch == active {
                quote = None;
            }
            continue;
        }
        if ch == '"' || ch == '\'' {
            quote = Some(ch);
            continue;
        }
        if ch == '%' {
            return &line[..index];
        }
    }
    line
}

fn metapost_signature(job: &MetapostJob) -> Result<String> {
    let mut signature = fs::read_to_string(&job.input_path)
        .with_context(|| format!("failed to read MetaPost file {}", job.input_path.display()))?;
    signature.push_str("\n%% texpilot metapost command\n");
    signature.push_str("mpost ");
    signature.push_str(
        job.input_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default(),
    );
    signature.push('\n');
    Ok(signature)
}

fn gnuplottex_jobs_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Vec<GnuplottexJob>> {
    let fls_path = out_dir.join(format!("{job_name}.fls"));
    let mut paths = if fls_path.exists() {
        recorded_outputs(&fls_path, doc_dir)?
    } else {
        Vec::new()
    };
    paths.sort();
    paths.dedup();

    let mut jobs = Vec::new();
    for script_path in paths.into_iter().filter(|path| is_gnuplottex_script(path)) {
        if !script_path.is_file() {
            continue;
        }
        let source = fs::read_to_string(&script_path).with_context(|| {
            format!("failed to read gnuplottex script {}", script_path.display())
        })?;
        let Some(output_path) = gnuplottex_output_path(&script_path, &source) else {
            continue;
        };
        let input_paths = gnuplottex_input_paths(&script_path, &source);
        let state_name = external_tool_state_name(out_dir, &output_path);
        let state_path = out_dir.join(format!(".texpilot-{state_name}.gnuplotstate.toml"));
        jobs.push(GnuplottexJob {
            script_path,
            output_path,
            input_paths,
            state_path,
        });
    }
    jobs.sort_by(|left, right| {
        left.output_path
            .cmp(&right.output_path)
            .then(left.script_path.cmp(&right.script_path))
    });
    jobs.dedup_by(|left, right| left.output_path == right.output_path);
    Ok(jobs)
}

fn is_gnuplottex_script(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("gnuplot"))
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.contains("-gnuplottex-fig"))
}

fn gnuplottex_output_path(script_path: &Path, source: &str) -> Option<PathBuf> {
    for line in source.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("set output") else {
            continue;
        };
        let value = quoted_or_bare_shell_value(rest.trim())?;
        let path = Path::new(value);
        if path.is_absolute() {
            return Some(path.to_path_buf());
        }
        return Some(
            script_path
                .parent()
                .unwrap_or_else(|| Path::new(""))
                .join(path),
        );
    }
    None
}

fn gnuplottex_input_paths(script_path: &Path, source: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let base_dir = script_path.parent().unwrap_or_else(|| Path::new(""));
    for line in source.lines() {
        let line = strip_gnuplot_comment(line).trim_start();
        if !gnuplot_line_can_reference_input(line) {
            continue;
        }
        for value in quoted_shell_values(line) {
            if let Some(path) = gnuplot_local_input_path(base_dir, value) {
                paths.push(path);
            }
        }
    }
    paths.sort();
    paths.dedup();
    paths
}

fn gnuplot_line_can_reference_input(line: &str) -> bool {
    ["plot", "splot", "replot", "load", "call"]
        .iter()
        .any(|command| {
            line == *command
                || line
                    .strip_prefix(command)
                    .is_some_and(|rest| rest.starts_with(char::is_whitespace))
        })
}

fn gnuplot_local_input_path(base_dir: &Path, value: &str) -> Option<PathBuf> {
    if value.is_empty()
        || matches!(value, "-" | "+")
        || value.contains("://")
        || value.starts_with('<')
        || value.starts_with('|')
    {
        return None;
    }
    let path = Path::new(value);
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_dir.join(path)
    };
    path.is_file().then_some(path)
}

fn strip_gnuplot_comment(line: &str) -> &str {
    let mut quote = None;
    let mut escaped = false;
    for (index, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active) = quote {
            if ch == active {
                quote = None;
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            continue;
        }
        if ch == '#' {
            return &line[..index];
        }
    }
    line
}

fn quoted_shell_values(value: &str) -> Vec<&str> {
    let mut values = Vec::new();
    let mut quote = None;
    let mut start = 0;
    let mut escaped = false;
    for (index, ch) in value.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active) = quote {
            if ch == active {
                values.push(&value[start..index]);
                quote = None;
            }
            continue;
        }
        if ch == '\'' || ch == '"' {
            quote = Some(ch);
            start = index + ch.len_utf8();
        }
    }
    values
}

fn quoted_or_bare_shell_value(value: &str) -> Option<&str> {
    if value.is_empty() {
        return None;
    }
    let mut chars = value.char_indices();
    let (_, first) = chars.next()?;
    if first == '\'' || first == '"' {
        for (index, ch) in chars {
            if ch == first {
                return Some(&value[first.len_utf8()..index]);
            }
        }
        return None;
    }
    value.split_whitespace().next()
}

fn gnuplottex_signature(job: &GnuplottexJob) -> Result<String> {
    let mut signature = fs::read_to_string(&job.script_path).with_context(|| {
        format!(
            "failed to read gnuplottex script {}",
            job.script_path.display()
        )
    })?;
    signature.push_str("\n%% texpilot gnuplottex command\ngnuplot\n");
    signature.push_str(&job.output_path.display().to_string());
    signature.push('\n');
    Ok(signature)
}

fn pgf_external_job_from_latest_run(
    out_dir: &Path,
    job_name: &str,
) -> Result<Option<PgfExternalJob>> {
    let makefile_path = out_dir.join(format!("{job_name}.makefile"));
    let figlist_path = out_dir.join(format!("{job_name}.figlist"));
    if !makefile_path.is_file() || !figlist_path.is_file() {
        return Ok(None);
    }
    let (make_targets, output_paths) = pgf_external_outputs(out_dir, &figlist_path)?;
    if output_paths.is_empty() {
        return Ok(None);
    }
    Ok(Some(PgfExternalJob {
        makefile_path,
        make_targets,
        output_paths,
    }))
}

fn pgf_external_outputs(
    out_dir: &Path,
    figlist_path: &Path,
) -> Result<(Vec<OsString>, Vec<PathBuf>)> {
    let source = fs::read_to_string(figlist_path).with_context(|| {
        format!(
            "failed to read PGF externalization figlist {}",
            figlist_path.display()
        )
    })?;
    let mut make_targets = Vec::new();
    let mut paths = Vec::new();
    for line in source.lines() {
        let stem = line.trim();
        if stem.is_empty() {
            continue;
        }
        let path = Path::new(stem);
        let stem_path = if path.is_absolute() {
            path.to_path_buf()
        } else if safe_relative_path(path) {
            out_dir.join(path)
        } else {
            continue;
        };
        let pdf_path = stem_path.with_extension("pdf");
        if let Ok(target) = pdf_path.strip_prefix(out_dir) {
            make_targets.push(target.as_os_str().to_os_string());
        } else {
            make_targets.push(pdf_path.as_os_str().to_os_string());
        }
        paths.push(pdf_path);
        paths.push(stem_path.with_extension("dpth"));
    }
    make_targets.sort();
    make_targets.dedup();
    paths.sort();
    paths.dedup();
    Ok((make_targets, paths))
}

fn bib2gls_job_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Option<Bib2GlsJob>> {
    let aux_path = out_dir.join(format!("{job_name}.aux"));
    let Ok(source) = fs::read_to_string(&aux_path) else {
        return Ok(None);
    };
    let resources = bib2gls_resources_from_aux(&source);
    if resources.is_empty() {
        return Ok(None);
    }

    let mut output_paths = resources
        .iter()
        .map(|resource| bib2gls_output_path(out_dir, &resource.output_stem))
        .collect::<Vec<_>>();
    output_paths.sort();
    output_paths.dedup();

    let mut resource_inputs = Vec::new();
    for resource in &resources {
        for src in bib2gls_resource_srcs(&resource.options) {
            if let Some(path) = resolve_bibtex_database_input(doc_dir, out_dir, &src)? {
                resource_inputs.push(path);
            }
        }
    }
    resource_inputs.sort();
    resource_inputs.dedup();

    let state_path = out_dir.join(format!(".texpilot-{job_name}.bib2glsstate.toml"));
    Ok(Some(Bib2GlsJob {
        doc_dir: doc_dir.to_path_buf(),
        aux_path,
        command_arg: job_name.to_string(),
        output_paths,
        resource_inputs,
        state_path,
    }))
}

fn bib2gls_resources_from_aux(source: &str) -> Vec<Bib2GlsResource> {
    let mut resources = Vec::new();
    for line in source.lines() {
        let Some(payloads) = braced_payloads_after_prefix(line, r"\glsxtr@resource", 2) else {
            continue;
        };
        resources.push(Bib2GlsResource {
            options: payloads[0].clone(),
            output_stem: payloads[1].clone(),
        });
    }
    resources
}

fn bib2gls_output_path(out_dir: &Path, output_stem: &str) -> PathBuf {
    let path = Path::new(output_stem);
    let path = if path.extension().is_some() {
        path.to_path_buf()
    } else {
        path.with_extension("glstex")
    };
    if path.is_absolute() {
        path
    } else {
        out_dir.join(path)
    }
}

fn bib2gls_resource_srcs(options: &str) -> Vec<String> {
    let mut srcs = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = options[cursor..].find("src") {
        let key_start = cursor + offset;
        let key_end = key_start + "src".len();
        if options[..key_start]
            .chars()
            .next_back()
            .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '-')
            || options[key_end..]
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '-')
        {
            cursor = key_end;
            continue;
        }

        let mut value_start = skip_ascii_whitespace(options, key_end);
        if !options[value_start..].starts_with('=') {
            cursor = key_end;
            continue;
        }
        value_start = skip_ascii_whitespace(options, value_start + 1);
        let (value, value_end) = if options[value_start..].starts_with('{') {
            let Some((payload, end)) = balanced_braced_payload_at(options, value_start) else {
                break;
            };
            (payload, end)
        } else {
            let end = options[value_start..]
                .find(',')
                .map_or(options.len(), |offset| value_start + offset);
            (options[value_start..end].trim().to_string(), end)
        };
        srcs.extend(
            value
                .split(',')
                .map(str::trim)
                .filter(|src| !src.is_empty())
                .map(str::to_string),
        );
        cursor = value_end;
    }
    srcs.sort();
    srcs.dedup();
    srcs
}

fn skip_ascii_whitespace(source: &str, mut cursor: usize) -> usize {
    while source[cursor..]
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_whitespace())
    {
        cursor += source[cursor..]
            .chars()
            .next()
            .map(char::len_utf8)
            .unwrap_or(1);
    }
    cursor
}

fn braced_payloads_after_prefix(line: &str, prefix: &str, count: usize) -> Option<Vec<String>> {
    let mut cursor = line.find(prefix)? + prefix.len();
    let mut payloads = Vec::new();
    for _ in 0..count {
        cursor = skip_ascii_whitespace(line, cursor);
        let (payload, end) = balanced_braced_payload_at(line, cursor)?;
        payloads.push(payload);
        cursor = end;
    }
    Some(payloads)
}

fn balanced_braced_payload_at(source: &str, open: usize) -> Option<(String, usize)> {
    if !source[open..].starts_with('{') {
        return None;
    }
    let mut depth = 0_u32;
    let mut payload_start = None;
    for (offset, ch) in source[open..].char_indices() {
        let index = open + offset;
        match ch {
            '{' => {
                depth += 1;
                if payload_start.is_none() {
                    payload_start = Some(index + ch.len_utf8());
                }
            }
            '}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    let start = payload_start?;
                    return Some((
                        source[start..index].trim().to_string(),
                        index + ch.len_utf8(),
                    ));
                }
            }
            _ => {}
        }
    }
    None
}

fn bib2gls_signature(job: &Bib2GlsJob) -> Result<String> {
    let mut signature = fs::read_to_string(&job.aux_path)
        .with_context(|| format!("failed to read Bib2Gls aux file {}", job.aux_path.display()))?;
    signature.push_str("\n%% texpilot bib2gls command\n");
    signature.push_str(&job.command_arg);
    signature.push('\n');
    signature.push_str(&environment_signature(BIB_ENV_VARS));
    Ok(signature)
}

fn write_pythontex_state(job: &PythontexJob, signature: &str) -> Result<()> {
    let previous = previous_external_tool_input_map(&job.state_path)?;
    let mut inputs = Vec::new();
    if let Some(fingerprint) = fingerprint_path_reusing(&job.code_path, Some(&previous))? {
        inputs.push(fingerprint);
    }
    for path in pythontex_dependency_paths(job)? {
        if let Some(fingerprint) = fingerprint_path_reusing(&path, Some(&previous))? {
            inputs.push(fingerprint);
        }
    }
    inputs.sort_by(|left, right| left.path.cmp(&right.path));
    inputs.dedup_by(|left, right| left.path == right.path);
    let output_paths = job
        .output_paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    let state = ExternalToolState {
        version: EXTERNAL_TOOL_STATE_VERSION,
        signature: signature.to_string(),
        output_path: output_paths.first().cloned().unwrap_or_default(),
        output_paths,
        inputs,
    };
    let source = toml::to_string(&state).context("failed to serialize external tool state")?;
    fs::write(&job.state_path, source).with_context(|| {
        format!(
            "failed to write external tool state {}",
            job.state_path.display()
        )
    })
}

fn external_tool_cache_is_fresh(
    state_path: &Path,
    signature: &str,
    output_path: &Path,
) -> Result<bool> {
    external_tool_cache_is_fresh_for_outputs(state_path, signature, &[output_path.to_path_buf()])
}

fn external_tool_cache_is_fresh_for_outputs(
    state_path: &Path,
    signature: &str,
    output_paths: &[PathBuf],
) -> Result<bool> {
    if output_paths.iter().any(|path| !path.exists()) {
        return Ok(false);
    }
    let Some(source) = read_optional_text_file(state_path, "external tool state")? else {
        return Ok(false);
    };
    let state: ExternalToolState = toml::from_str(&source).with_context(|| {
        format!(
            "failed to parse external tool state {}",
            state_path.display()
        )
    })?;
    let output_path_strings = output_paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    if state.version != EXTERNAL_TOOL_STATE_VERSION
        || state.signature != signature
        || external_tool_state_outputs(&state) != output_path_strings
    {
        return Ok(false);
    }
    for input in &state.inputs {
        if !input_fingerprint_is_fresh(input)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn external_tool_state_outputs(state: &ExternalToolState) -> Vec<String> {
    if state.output_paths.is_empty() {
        vec![state.output_path.clone()]
    } else {
        state.output_paths.clone()
    }
}

fn write_metapost_state(job: &MetapostJob, signature: &str) -> Result<()> {
    let previous = previous_external_tool_input_map(&job.state_path)?;
    let mut inputs = Vec::new();
    if let Some(fingerprint) = fingerprint_path_reusing(&job.input_path, Some(&previous))? {
        inputs.push(fingerprint);
    }
    for path in &job.input_paths {
        if let Some(fingerprint) = fingerprint_path_reusing(path, Some(&previous))? {
            inputs.push(fingerprint);
        }
    }
    inputs.sort_by(|left, right| left.path.cmp(&right.path));
    inputs.dedup_by(|left, right| left.path == right.path);
    let output_paths = job
        .output_paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    let state = ExternalToolState {
        version: EXTERNAL_TOOL_STATE_VERSION,
        signature: signature.to_string(),
        output_path: output_paths.first().cloned().unwrap_or_default(),
        output_paths,
        inputs,
    };
    let source = toml::to_string(&state).context("failed to serialize external tool state")?;
    fs::write(&job.state_path, source).with_context(|| {
        format!(
            "failed to write external tool state {}",
            job.state_path.display()
        )
    })
}

fn write_gnuplottex_state(job: &GnuplottexJob, signature: &str) -> Result<()> {
    let previous = previous_external_tool_input_map(&job.state_path)?;
    let mut inputs = Vec::new();
    if let Some(fingerprint) = fingerprint_path_reusing(&job.script_path, Some(&previous))? {
        inputs.push(fingerprint);
    }
    for path in &job.input_paths {
        if let Some(fingerprint) = fingerprint_path_reusing(path, Some(&previous))? {
            inputs.push(fingerprint);
        }
    }
    inputs.sort_by(|left, right| left.path.cmp(&right.path));
    inputs.dedup_by(|left, right| left.path == right.path);
    let state = ExternalToolState {
        version: EXTERNAL_TOOL_STATE_VERSION,
        signature: signature.to_string(),
        output_path: job.output_path.display().to_string(),
        output_paths: vec![job.output_path.display().to_string()],
        inputs,
    };
    let source = toml::to_string(&state).context("failed to serialize external tool state")?;
    fs::write(&job.state_path, source).with_context(|| {
        format!(
            "failed to write external tool state {}",
            job.state_path.display()
        )
    })
}

fn write_bib2gls_state(job: &Bib2GlsJob, signature: &str) -> Result<()> {
    let previous = previous_external_tool_input_map(&job.state_path)?;
    let mut inputs = Vec::new();
    if let Some(fingerprint) = fingerprint_path_reusing(&job.aux_path, Some(&previous))? {
        inputs.push(fingerprint);
    }
    for path in &job.resource_inputs {
        if let Some(fingerprint) = fingerprint_path_reusing(path, Some(&previous))? {
            inputs.push(fingerprint);
        }
    }
    inputs.sort_by(|left, right| left.path.cmp(&right.path));
    inputs.dedup_by(|left, right| left.path == right.path);
    let output_paths = job
        .output_paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    let state = ExternalToolState {
        version: EXTERNAL_TOOL_STATE_VERSION,
        signature: signature.to_string(),
        output_path: output_paths.first().cloned().unwrap_or_default(),
        output_paths,
        inputs,
    };
    let source = toml::to_string(&state).context("failed to serialize external tool state")?;
    fs::write(&job.state_path, source).with_context(|| {
        format!(
            "failed to write external tool state {}",
            job.state_path.display()
        )
    })
}

fn write_eps_conversion_state(job: &EpsConversionJob, signature: &str) -> Result<()> {
    let previous = previous_external_tool_input_map(&job.state_path)?;
    let mut inputs = Vec::new();
    if let Some(fingerprint) = fingerprint_path_reusing(&job.input_path, Some(&previous))? {
        inputs.push(fingerprint);
    }
    inputs.sort_by(|left, right| left.path.cmp(&right.path));
    inputs.dedup_by(|left, right| left.path == right.path);
    let state = ExternalToolState {
        version: EXTERNAL_TOOL_STATE_VERSION,
        signature: signature.to_string(),
        output_path: job.output_path.display().to_string(),
        output_paths: vec![job.output_path.display().to_string()],
        inputs,
    };
    let source = toml::to_string(&state).context("failed to serialize external tool state")?;
    fs::write(&job.state_path, source).with_context(|| {
        format!(
            "failed to write external tool state {}",
            job.state_path.display()
        )
    })
}

fn write_svg_conversion_state(job: &SvgConversionJob, signature: &str) -> Result<()> {
    let previous = previous_external_tool_input_map(&job.state_path)?;
    let mut inputs = Vec::new();
    if let Some(fingerprint) = fingerprint_path_reusing(&job.input_path, Some(&previous))? {
        inputs.push(fingerprint);
    }
    inputs.sort_by(|left, right| left.path.cmp(&right.path));
    inputs.dedup_by(|left, right| left.path == right.path);
    let output_paths = job
        .output_paths()
        .into_iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    let state = ExternalToolState {
        version: EXTERNAL_TOOL_STATE_VERSION,
        signature: signature.to_string(),
        output_path: output_paths.first().cloned().unwrap_or_default(),
        output_paths,
        inputs,
    };
    let source = toml::to_string(&state).context("failed to serialize external tool state")?;
    fs::write(&job.state_path, source).with_context(|| {
        format!(
            "failed to write external tool state {}",
            job.state_path.display()
        )
    })
}

fn write_asymptote_state(job: &AsymptoteJob, signature: &str) -> Result<()> {
    let previous = previous_external_tool_input_map(&job.state_path)?;
    let mut inputs = Vec::new();
    if let Some(fingerprint) = fingerprint_path_reusing(&job.input_path, Some(&previous))? {
        inputs.push(fingerprint);
    }
    if let Some(pre_path) = asymptote_preamble_path(&job.input_path)
        && let Some(fingerprint) = fingerprint_path_reusing(&pre_path, Some(&previous))?
    {
        inputs.push(fingerprint);
    }
    for path in &job.input_paths {
        if let Some(fingerprint) = fingerprint_path_reusing(path, Some(&previous))? {
            inputs.push(fingerprint);
        }
    }
    inputs.sort_by(|left, right| left.path.cmp(&right.path));
    inputs.dedup_by(|left, right| left.path == right.path);
    let state = ExternalToolState {
        version: EXTERNAL_TOOL_STATE_VERSION,
        signature: signature.to_string(),
        output_path: job.output_path.display().to_string(),
        output_paths: vec![job.output_path.display().to_string()],
        inputs,
    };
    let source = toml::to_string(&state).context("failed to serialize external tool state")?;
    fs::write(&job.state_path, source).with_context(|| {
        format!(
            "failed to write external tool state {}",
            job.state_path.display()
        )
    })
}

fn path_arg_relative_to(cwd: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(cwd).unwrap_or(path).to_path_buf()
}

fn bibtex_aux_signature_from_source(source: &str, job: &BibtexJob) -> String {
    let mut signature = String::new();
    let mut citation_keys = Vec::new();
    let mut seen_citation_keys = HashSet::new();
    for line in source.lines() {
        if line.starts_with(r"\citation") {
            let Some(payload) = braced_payload(line, r"\citation") else {
                continue;
            };
            for key in split_tex_top_level(payload, ',') {
                let key = key.split_whitespace().collect::<String>();
                if !key.is_empty() && seen_citation_keys.insert(key.clone()) {
                    citation_keys.push(key);
                }
            }
        } else if line.starts_with(r"\bibdata") || line.starts_with(r"\bibstyle") {
            signature.push_str(&canonical_bibtex_aux_control_line(line));
            signature.push('\n');
        }
    }
    for key in citation_keys {
        let _ = writeln!(&mut signature, "\\citation{{{key}}}");
    }
    signature.push_str("%% texpilot bibtex program\n");
    signature.push_str(job.program.executable());
    signature.push('\n');
    if !job.command_options.is_empty() {
        signature.push_str("%% texpilot bibtex options\n");
        for option in &job.command_options {
            signature.push_str(option);
            signature.push('\n');
        }
    }
    signature.push_str(&environment_signature(BIB_ENV_VARS));
    signature
}

fn canonical_bibtex_aux_control_line(line: &str) -> String {
    for prefix in [r"\bibdata", r"\bibstyle"] {
        if let Some(payload) = braced_payload(line, prefix)
            && let Some(payload) = normalized_bibtex_comma_payload(payload)
        {
            return format!("{prefix}{{{payload}}}");
        }
    }
    line.trim().to_string()
}

fn bibtex_cache_is_fresh(state_path: &Path, signature: &str, bbl_path: &Path) -> Result<bool> {
    if !state_path.exists() || !bbl_path.exists() {
        return Ok(false);
    }
    let source = fs::read_to_string(state_path)
        .with_context(|| format!("failed to read bibliography state {}", state_path.display()))?;
    let state: BibState = toml::from_str(&source).with_context(|| {
        format!(
            "failed to parse bibliography state {}",
            state_path.display()
        )
    })?;
    if state.version != BIB_STATE_VERSION
        || state.signature != signature
        || state.bbl_path != bbl_path.display().to_string()
    {
        return Ok(false);
    }
    for input in &state.inputs {
        if !input_fingerprint_is_fresh(input)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn write_bibtex_state_from_source(
    job: &BibtexJob,
    signature: &str,
    aux_source: &str,
    doc_dir: &Path,
    out_dir: &Path,
) -> Result<()> {
    let previous = previous_bib_input_map(&job.state_path)?;
    let mut inputs = bibtex_inputs_from_aux(aux_source, doc_dir, out_dir, &previous, job)?;
    inputs.sort_by(|left, right| left.path.cmp(&right.path));
    inputs.dedup_by(|left, right| left.path == right.path);
    let state = BibState {
        version: BIB_STATE_VERSION,
        signature: signature.to_string(),
        bbl_path: job.bbl_path.display().to_string(),
        inputs,
    };
    let state_source = toml::to_string(&state).context("failed to serialize bibliography state")?;
    fs::write(&job.state_path, state_source).with_context(|| {
        format!(
            "failed to write bibliography state {}",
            job.state_path.display()
        )
    })
}

fn bibtex_inputs_from_aux(
    source: &str,
    doc_dir: &Path,
    out_dir: &Path,
    previous: &HashMap<String, FileFingerprint>,
    job: &BibtexJob,
) -> Result<Vec<FileFingerprint>> {
    let mut inputs = Vec::new();
    let citation_keys = bibtex_citation_keys(source);
    for line in source.lines() {
        if let Some(names) = braced_payload(line, r"\bibdata") {
            for name in names
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
            {
                if let Some(path) = resolve_bibtex_database_input(doc_dir, out_dir, name)?
                    && let Some(fingerprint) =
                        fingerprint_bibtex_database_reusing(&path, Some(previous), &citation_keys)?
                {
                    inputs.push(fingerprint);
                }
            }
        }
        if let Some(name) = braced_payload(line, r"\bibstyle") {
            let name = name.trim();
            if !name.is_empty()
                && let Some(path) = resolve_kpathsea_input(doc_dir, name, "bst")?
                && let Some(fingerprint) = fingerprint_path_reusing(&path, Some(previous))?
            {
                inputs.push(fingerprint);
            }
        }
    }
    for path in &job.request_inputs {
        if let Some(fingerprint) = fingerprint_path_reusing(path, Some(previous))? {
            inputs.push(fingerprint);
        }
    }
    Ok(inputs)
}

fn bibtex_citation_keys(source: &str) -> Option<Vec<String>> {
    let mut keys = Vec::new();
    for line in source.lines() {
        let Some(names) = braced_payload(line, r"\citation") else {
            continue;
        };
        for name in split_tex_top_level(names, ',')
            .into_iter()
            .map(|name| name.split_whitespace().collect::<String>())
        {
            if name == "*" {
                return None;
            }
            if !name.is_empty() {
                keys.push(name.to_string());
            }
        }
    }
    keys.sort();
    keys.dedup();
    Some(keys)
}

fn fingerprint_bibtex_database_reusing(
    path: &Path,
    previous: Option<&HashMap<String, FileFingerprint>>,
    citation_keys: &Option<Vec<String>>,
) -> Result<Option<FileFingerprint>> {
    let Some(citation_keys) = citation_keys else {
        return fingerprint_path_reusing(path, previous);
    };
    let Ok(canonical) = path.canonicalize() else {
        return Ok(None);
    };
    let Some((_, modified_ns)) = file_metadata_fingerprint(&canonical)? else {
        return Ok(None);
    };
    let path = canonical.display().to_string();
    let encoded_keys = encode_bibtex_citation_keys(citation_keys);
    let hash_prefix = format!("{BIB_CITED_EFFECTIVE_HASH_PREFIX}{encoded_keys}:");
    if let Some(fingerprint) =
        previous
            .and_then(|previous| previous.get(&path))
            .filter(|fingerprint| {
                fingerprint.modified_ns == modified_ns && fingerprint.hash.starts_with(&hash_prefix)
            })
    {
        return Ok(Some(fingerprint.clone()));
    }

    let bytes = fs::read(&canonical)
        .with_context(|| format!("failed to read input {}", canonical.display()))?;
    let Some(effective) = cited_bibtex_bytes(&bytes, citation_keys) else {
        return fingerprint_path_reusing(&canonical, previous);
    };
    Ok(Some(FileFingerprint {
        path,
        len: effective.len() as u64,
        modified_ns,
        hash: format!("{hash_prefix}{:016x}", content_hash(&effective)),
    }))
}

fn encode_bibtex_citation_keys(keys: &[String]) -> String {
    let mut encoded = String::new();
    for (index, key) in keys.iter().enumerate() {
        if index > 0 {
            encoded.push(',');
        }
        append_hex_bytes(&mut encoded, key.as_bytes());
    }
    encoded
}

fn decode_bibtex_citation_keys(encoded: &str) -> Option<Vec<String>> {
    if encoded.is_empty() {
        return Some(Vec::new());
    }
    encoded
        .split(',')
        .map(|key| {
            let bytes = decode_hex_bytes(key)?;
            String::from_utf8(bytes).ok()
        })
        .collect()
}

fn append_hex_bytes(output: &mut String, bytes: &[u8]) {
    for byte in bytes {
        let _ = write!(output, "{byte:02x}");
    }
}

fn decode_hex_bytes(encoded: &str) -> Option<Vec<u8>> {
    if !encoded.len().is_multiple_of(2) {
        return None;
    }
    let mut bytes = Vec::with_capacity(encoded.len() / 2);
    for pair in encoded.as_bytes().chunks_exact(2) {
        let high = hex_value(pair[0])?;
        let low = hex_value(pair[1])?;
        bytes.push((high << 4) | low);
    }
    Some(bytes)
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn cited_bibtex_bytes(bytes: &[u8], citation_keys: &[String]) -> Option<Vec<u8>> {
    let entries = parse_bibtex_entries(bytes)?;
    let citation_keys = citation_keys
        .iter()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let mut selected = Vec::new();
    for entry in entries {
        if bibtex_entry_kind_eq(entry.kind, b"comment") {
            continue;
        }
        if bibtex_entry_kind_eq(entry.kind, b"string")
            || bibtex_entry_kind_eq(entry.kind, b"preamble")
        {
            selected.extend_from_slice(entry.raw);
            selected.push(b'\n');
            continue;
        }
        let key = entry.key?;
        if citation_keys.contains(std::str::from_utf8(key).ok()?) {
            if bibtex_entry_has_crossref_like_dependency(entry.raw) {
                return None;
            }
            selected.extend_from_slice(entry.raw);
            selected.push(b'\n');
        }
    }
    Some(selected)
}

#[derive(Debug)]
struct ParsedBibtexEntry<'a> {
    kind: &'a [u8],
    key: Option<&'a [u8]>,
    raw: &'a [u8],
}

fn parse_bibtex_entries(bytes: &[u8]) -> Option<Vec<ParsedBibtexEntry<'_>>> {
    let mut entries = Vec::new();
    let mut cursor = 0;
    while let Some(offset) = bytes[cursor..].iter().position(|byte| *byte == b'@') {
        let entry_start = cursor + offset;
        let mut open = entry_start + 1;
        while bytes.get(open).is_some_and(u8::is_ascii_whitespace) {
            open += 1;
        }
        let kind_start = open;
        while bytes
            .get(open)
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || matches!(*byte, b'-' | b'_'))
        {
            open += 1;
        }
        if open == kind_start {
            cursor = entry_start + 1;
            continue;
        }
        let kind = &bytes[kind_start..open];
        while bytes.get(open).is_some_and(u8::is_ascii_whitespace) {
            open += 1;
        }
        let delimiter = *bytes.get(open)?;
        if !matches!(delimiter, b'{' | b'(') {
            cursor = open;
            continue;
        }
        let close = matching_bibtex_entry_end(bytes, open, delimiter)?;
        let body = &bytes[open + 1..close];
        let key = if bibtex_entry_kind_eq(kind, b"string")
            || bibtex_entry_kind_eq(kind, b"preamble")
            || bibtex_entry_kind_eq(kind, b"comment")
        {
            None
        } else {
            bibtex_entry_key(body)
        };
        entries.push(ParsedBibtexEntry {
            kind,
            key,
            raw: &bytes[entry_start..=close],
        });
        cursor = close + 1;
    }
    Some(entries)
}

fn matching_bibtex_entry_end(bytes: &[u8], open: usize, delimiter: u8) -> Option<usize> {
    let close = if delimiter == b'{' { b'}' } else { b')' };
    let mut depth = 1usize;
    let mut cursor = open + 1;
    while cursor < bytes.len() {
        match bytes[cursor] {
            byte if byte == delimiter => depth += 1,
            byte if byte == close => {
                depth -= 1;
                if depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}

fn bibtex_entry_key(body: &[u8]) -> Option<&[u8]> {
    let mut start = 0;
    while body.get(start).is_some_and(u8::is_ascii_whitespace) {
        start += 1;
    }
    let comma = body[start..].iter().position(|byte| *byte == b',')?;
    let mut end = start + comma;
    while end > start && body[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    (end > start).then_some(&body[start..end])
}

fn bibtex_entry_kind_eq(kind: &[u8], expected: &[u8]) -> bool {
    kind.eq_ignore_ascii_case(expected)
}

fn bibtex_entry_has_crossref_like_dependency(raw: &[u8]) -> bool {
    ascii_bytes_contains_case_insensitive(raw, b"crossref")
        || ascii_bytes_contains_case_insensitive(raw, b"xdata")
}

fn ascii_bytes_contains_case_insensitive(bytes: &[u8], needle: &[u8]) -> bool {
    bytes
        .windows(needle.len())
        .any(|window| window.eq_ignore_ascii_case(needle))
}

fn resolve_bibtex_database_input(
    doc_dir: &Path,
    out_dir: &Path,
    name: &str,
) -> Result<Option<PathBuf>> {
    let candidate = if Path::new(name).extension().is_some() {
        name.to_string()
    } else {
        format!("{name}.bib")
    };
    let out_candidate = out_dir.join(&candidate);
    if out_candidate.exists() {
        return Ok(Some(out_candidate));
    }
    let doc_candidate = doc_dir.join(&candidate);
    if doc_candidate.exists() {
        return Ok(Some(doc_candidate));
    }
    resolve_kpathsea_input(doc_dir, name, "bib")
}

fn biber_inputs_from_bcf(
    source: &str,
    doc_dir: &Path,
    previous: &HashMap<String, FileFingerprint>,
) -> Result<Vec<FileFingerprint>> {
    let mut inputs = Vec::new();
    let citation_keys = biber_citation_keys(source);
    for datasource in biber_datasource_specs(source) {
        if datasource.name.contains("://") {
            continue;
        }
        let paths = resolve_biber_datasource_paths(doc_dir, &datasource)?;
        if datasource.glob {
            inputs.push(fingerprint_biber_glob_matches(
                doc_dir,
                &datasource.name,
                &paths,
            )?);
        }
        for path in paths {
            if let Some(fingerprint) = if datasource.supports_cited_entry_fingerprint {
                fingerprint_bibtex_database_reusing(&path, Some(previous), &citation_keys)?
            } else {
                fingerprint_path_reusing(&path, Some(previous))?
            } {
                inputs.push(fingerprint);
            }
        }
    }
    Ok(inputs)
}

#[derive(Debug, Clone)]
struct BiberDatasource {
    name: String,
    glob: bool,
    supports_cited_entry_fingerprint: bool,
}

fn biber_datasource_specs(source: &str) -> Vec<BiberDatasource> {
    let mut datasources = Vec::new();
    let mut cursor = 0;
    while let Some(start) = source[cursor..].find("<bcf:datasource") {
        let tag_start = cursor + start;
        let Some(open_end_offset) = source[tag_start..].find('>') else {
            break;
        };
        let open_tag = &source[tag_start..tag_start + open_end_offset + 1];
        let payload_start = tag_start + open_end_offset + 1;
        let Some(close_offset) = source[payload_start..].find("</bcf:datasource>") else {
            break;
        };
        let payload_end = payload_start + close_offset;
        let payload = xml_unescape(source[payload_start..payload_end].trim());
        if !payload.is_empty() {
            let datatype = xml_attr_value(open_tag, "datatype").unwrap_or_default();
            let glob = xml_attr_value(open_tag, "glob")
                .is_some_and(|value| xml_truthy_value(value.trim()));
            datasources.push(BiberDatasource {
                name: payload,
                glob,
                supports_cited_entry_fingerprint: datatype.eq_ignore_ascii_case("bibtex"),
            });
        }
        cursor = payload_end + "</bcf:datasource>".len();
    }
    datasources.sort_by(|left, right| left.name.cmp(&right.name));
    datasources.dedup_by(|left, right| left.name == right.name);
    datasources
}

fn resolve_biber_datasource_paths(
    doc_dir: &Path,
    datasource: &BiberDatasource,
) -> Result<Vec<PathBuf>> {
    if datasource.glob {
        return resolve_biber_glob_datasource(doc_dir, &datasource.name);
    }
    Ok(resolve_kpathsea_input(doc_dir, &datasource.name, "bib")?
        .into_iter()
        .collect())
}

fn resolve_biber_glob_datasource(doc_dir: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
    let pattern_path = Path::new(pattern);
    let absolute_pattern = if pattern_path.is_absolute() {
        pattern_path.to_path_buf()
    } else {
        doc_dir.join(pattern_path)
    };
    let pattern_string = absolute_pattern.to_string_lossy().into_owned();
    let options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: true,
        require_literal_leading_dot: false,
    };
    let matches = glob_with(&pattern_string, options)
        .with_context(|| format!("invalid Biber datasource glob {pattern}"))?;
    let mut paths = Vec::new();
    for entry in matches {
        match entry {
            Ok(path) if path.is_file() => paths.push(path),
            Ok(_) => {}
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("failed to read Biber datasource glob {pattern}"));
            }
        }
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn fingerprint_biber_glob_matches(
    doc_dir: &Path,
    pattern: &str,
    paths: &[PathBuf],
) -> Result<FileFingerprint> {
    let doc_dir = canonical_or_original(doc_dir);
    let mut bytes = Vec::new();
    bytes.extend_from_slice(doc_dir.display().to_string().as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(pattern.as_bytes());
    bytes.push(0);
    for path in paths {
        let canonical = canonical_or_original(path);
        bytes.extend_from_slice(canonical.display().to_string().as_bytes());
        bytes.push(0);
    }
    Ok(FileFingerprint {
        path: biber_glob_fingerprint_path(&doc_dir, pattern),
        len: paths.len() as u64,
        modified_ns: 0,
        hash: format!(
            "{BIBER_GLOB_MATCHES_HASH_PREFIX}{:016x}",
            content_hash(&bytes)
        ),
    })
}

fn biber_glob_fingerprint_path(doc_dir: &Path, pattern: &str) -> String {
    let mut path = String::from(BIBER_GLOB_FINGERPRINT_PATH_PREFIX);
    append_hex_bytes(&mut path, doc_dir.display().to_string().as_bytes());
    path.push(':');
    append_hex_bytes(&mut path, pattern.as_bytes());
    path
}

fn decode_biber_glob_fingerprint_path(path: &str) -> Option<(PathBuf, String)> {
    let encoded = path.strip_prefix(BIBER_GLOB_FINGERPRINT_PATH_PREFIX)?;
    let (doc_dir, pattern) = encoded.split_once(':')?;
    let doc_dir = String::from_utf8(decode_hex_bytes(doc_dir)?).ok()?;
    let pattern = String::from_utf8(decode_hex_bytes(pattern)?).ok()?;
    Some((PathBuf::from(doc_dir), pattern))
}

fn biber_config_inputs(
    doc_dir: &Path,
    previous: &HashMap<String, FileFingerprint>,
) -> Result<Vec<FileFingerprint>> {
    let config_path = resolve_biber_config_path(doc_dir)?;
    let mut inputs = vec![fingerprint_biber_config_choice(
        doc_dir,
        config_path.as_deref(),
    )];
    if let Some(path) = config_path
        && let Some(fingerprint) = fingerprint_path_reusing(&path, Some(previous))?
    {
        inputs.push(fingerprint);
    }
    Ok(inputs)
}

fn resolve_biber_config_path(doc_dir: &Path) -> Result<Option<PathBuf>> {
    for candidate in biber_config_candidates(doc_dir) {
        if candidate.is_file() {
            return Ok(Some(candidate));
        }
    }
    resolve_kpathsea_input(doc_dir, "biber", "conf")
}

fn biber_config_candidates(doc_dir: &Path) -> Vec<PathBuf> {
    let mut candidates = vec![doc_dir.join("biber.conf"), doc_dir.join(".biber.conf")];
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        candidates.push(home.join(".biber.conf"));
        candidates.push(home.join("biber.conf"));
        candidates.push(home.join(".config").join("biber").join("biber.conf"));
        candidates.push(home.join("Library").join("biber").join("biber.conf"));
    }
    if let Some(config_home) = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from) {
        candidates.push(config_home.join("biber").join("biber.conf"));
    }
    if let Some(appdata) = std::env::var_os("APPDATA").map(PathBuf::from) {
        candidates.push(appdata.join("biber").join("biber.conf"));
    }
    candidates
}

fn fingerprint_biber_config_choice(doc_dir: &Path, config_path: Option<&Path>) -> FileFingerprint {
    let doc_dir = canonical_or_original(doc_dir);
    let config_path = config_path.map(canonical_or_original);
    let mut bytes = Vec::new();
    bytes.extend_from_slice(doc_dir.display().to_string().as_bytes());
    bytes.push(0);
    if let Some(path) = &config_path {
        bytes.extend_from_slice(path.display().to_string().as_bytes());
    } else {
        bytes.extend_from_slice(b"<none>");
    }
    FileFingerprint {
        path: biber_config_fingerprint_path(&doc_dir),
        len: u64::from(config_path.is_some()),
        modified_ns: 0,
        hash: format!(
            "{BIBER_CONFIG_CHOICE_HASH_PREFIX}{:016x}",
            content_hash(&bytes)
        ),
    }
}

fn biber_config_fingerprint_path(doc_dir: &Path) -> String {
    let mut path = String::from(BIBER_CONFIG_FINGERPRINT_PATH_PREFIX);
    append_hex_bytes(&mut path, doc_dir.display().to_string().as_bytes());
    path
}

fn decode_biber_config_fingerprint_path(path: &str) -> Option<PathBuf> {
    let encoded = path.strip_prefix(BIBER_CONFIG_FINGERPRINT_PATH_PREFIX)?;
    String::from_utf8(decode_hex_bytes(encoded)?)
        .ok()
        .map(PathBuf::from)
}

fn biber_citation_keys(source: &str) -> Option<Vec<String>> {
    let mut keys = Vec::new();
    for value in xml_tag_values(source, "bcf:citekey") {
        if value == "*" {
            return None;
        }
        keys.push(value);
    }
    keys.sort();
    keys.dedup();
    Some(keys)
}

fn xml_blocks<'a>(source: &'a str, tag: &str) -> Vec<&'a str> {
    let open_pattern = format!("<{tag}");
    let close_pattern = format!("</{tag}>");
    let mut blocks = Vec::new();
    let mut cursor = 0;
    while let Some(start_offset) = source[cursor..].find(&open_pattern) {
        let tag_start = cursor + start_offset;
        let Some(open_end_offset) = source[tag_start..].find('>') else {
            break;
        };
        let body_start = tag_start + open_end_offset + 1;
        let Some(close_offset) = source[body_start..].find(&close_pattern) else {
            break;
        };
        let body_end = body_start + close_offset;
        blocks.push(&source[body_start..body_end]);
        cursor = body_end + close_pattern.len();
    }
    blocks
}

fn xml_first_tag_value(source: &str, tag: &str) -> Option<String> {
    xml_tag_values(source, tag).into_iter().next()
}

fn xml_tag_values(source: &str, tag: &str) -> Vec<String> {
    let open_pattern = format!("<{tag}");
    let close_pattern = format!("</{tag}>");
    let mut values = Vec::new();
    let mut cursor = 0;
    while let Some(start_offset) = source[cursor..].find(&open_pattern) {
        let tag_start = cursor + start_offset;
        let Some(open_end_offset) = source[tag_start..].find('>') else {
            break;
        };
        let value_start = tag_start + open_end_offset + 1;
        let Some(close_offset) = source[value_start..].find(&close_pattern) else {
            break;
        };
        let value_end = value_start + close_offset;
        values.push(xml_unescape(source[value_start..value_end].trim()));
        cursor = value_end + close_pattern.len();
    }
    values
}

fn xml_attr_value(open_tag: &str, wanted: &str) -> Option<String> {
    let bytes = open_tag.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        while bytes
            .get(cursor)
            .is_some_and(|byte| byte.is_ascii_whitespace() || matches!(*byte, b'<' | b'/'))
        {
            cursor += 1;
        }
        let name_start = cursor;
        while bytes
            .get(cursor)
            .is_some_and(|byte| !byte.is_ascii_whitespace() && !matches!(*byte, b'=' | b'/' | b'>'))
        {
            cursor += 1;
        }
        if cursor == name_start {
            cursor += 1;
            continue;
        }
        let name = &open_tag[name_start..cursor];
        while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
            cursor += 1;
        }
        if bytes.get(cursor) != Some(&b'=') {
            continue;
        }
        cursor += 1;
        while bytes.get(cursor).is_some_and(u8::is_ascii_whitespace) {
            cursor += 1;
        }
        let quote = bytes.get(cursor).copied()?;
        if !matches!(quote, b'\'' | b'"') {
            continue;
        }
        cursor += 1;
        let value_start = cursor;
        while bytes.get(cursor).is_some_and(|byte| *byte != quote) {
            cursor += 1;
        }
        let value = &open_tag[value_start..cursor];
        if name == wanted {
            return Some(xml_unescape(value));
        }
        cursor += 1;
    }
    None
}

fn xml_truthy_value(value: &str) -> bool {
    value == "1" || value.eq_ignore_ascii_case("true")
}

fn xml_unescape(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

fn braced_payload<'a>(line: &'a str, command: &str) -> Option<&'a str> {
    let rest = line.strip_prefix(command)?;
    let rest = rest.strip_prefix('{')?;
    let close = rest.rfind('}')?;
    Some(&rest[..close])
}

fn resolve_kpathsea_input(doc_dir: &Path, name: &str, extension: &str) -> Result<Option<PathBuf>> {
    let candidate = if Path::new(name).extension().is_some() {
        name.to_string()
    } else {
        format!("{name}.{extension}")
    };

    let local = doc_dir.join(&candidate);
    if local.exists() {
        return Ok(Some(local));
    }

    let mut command = Command::new("kpsewhich");
    command.current_dir(doc_dir);
    for (variable, value) in kpathsea_env_overrides_for_extension(extension, doc_dir) {
        command.env(variable, value);
    }
    let output = command
        .arg(&candidate)
        .output()
        .with_context(|| format!("failed to launch kpsewhich for {candidate}"))?;
    if !output.status.success() {
        return Ok(None);
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        Ok(None)
    } else {
        Ok(Some(PathBuf::from(path)))
    }
}

fn tex_rerun_reasons(log_path: &Path) -> Result<Vec<String>> {
    let bytes = fs::read(log_path)
        .with_context(|| format!("failed to read TeX log {}", log_path.display()))?;
    let log = String::from_utf8_lossy(&bytes);
    let mut reasons = Vec::new();
    for line in log.lines() {
        let Some(reason) = line_tex_rerun_reason(line) else {
            continue;
        };
        if !reasons.iter().any(|known| known == reason) {
            reasons.push(reason.to_string());
        }
    }
    Ok(reasons)
}

#[cfg(test)]
fn line_requests_tex_rerun(line: &str) -> bool {
    line_tex_rerun_reason(line).is_some()
}

fn line_tex_rerun_reason(line: &str) -> Option<&'static str> {
    let lower = line.to_ascii_lowercase();
    if lower.contains("rerun to get") {
        Some("rerun-to-get-cross-references")
    } else if lower.contains("rerun latex") {
        Some("rerun-latex")
    } else if lower.contains("rerun lualatex") {
        Some("rerun-lualatex")
    } else if lower.contains("rerun xelatex") {
        Some("rerun-xelatex")
    } else if lower.contains("label(s) may have changed") {
        Some("labels-changed")
    } else if lower.contains("reference(s) may have changed") {
        Some("references-changed")
    } else if lower.contains("citation(s) may have changed") {
        Some("citations-changed")
    } else if (lower.contains("file `") && lower.contains("' has changed"))
        || (lower.contains("file \"") && lower.contains("\" has changed"))
    {
        Some("file-changed")
    } else {
        None
    }
}

fn configure_output(command: &mut Command, options: &BuildOptions) {
    if options.quiet && !options.print_command {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    }
}

fn direct_mode_key(options: &BuildOptions, main: &Path) -> String {
    format!(
        "v{};main={};job={};engine={:?};bib={:?};fast={};once={};precompile_preamble={};synctex={};shell_escape={};env={}",
        BUILD_STATE_VERSION,
        main.display(),
        options.job_name.as_deref().unwrap_or("<default>"),
        options.engine,
        options.bib_mode,
        options.fast,
        options.once,
        options.precompile_preamble,
        options.synctex,
        options.shell_escape,
        environment_signature(BUILD_ENV_VARS)
    )
}

fn environment_signature(vars: &[&str]) -> String {
    let mut signature = String::new();
    for var in vars {
        let value = std::env::var_os(var)
            .map(|value| format!("{:016x}", content_hash(value.to_string_lossy().as_bytes())))
            .unwrap_or_else(|| "unset".to_string());
        let _ = write!(&mut signature, "{var}={value};");
    }
    signature
}

fn read_build_state_if_exists(state_path: &Path) -> Result<Option<BuildState>> {
    let Some(source) = read_optional_text_file(state_path, "build state")? else {
        return Ok(None);
    };
    let state: BuildState = toml::from_str(&source)
        .with_context(|| format!("failed to parse build state {}", state_path.display()))?;
    Ok(Some(state))
}

fn read_optional_text_file(path: &Path, description: &str) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(source) => Ok(Some(source)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => {
            Err(error).with_context(|| format!("failed to read {description} {}", path.display()))
        }
    }
}

fn build_state_is_compatible(state: &BuildState, mode_key: &str, pdf_path: &Path) -> bool {
    state.version == BUILD_STATE_VERSION
        && state.mode_key == mode_key
        && state.pdf_path == pdf_path.display().to_string()
}

fn build_state_inputs_are_fresh(
    state: &BuildState,
    build_state_input_freshness: &mut HashMap<FileFingerprint, bool>,
) -> Result<bool> {
    for input in &state.inputs {
        if !input_fingerprint_is_fresh_cached(input, build_state_input_freshness)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn can_preflight_aux_tools(
    previous_build_state: Option<&BuildState>,
    direct: DirectContext<'_>,
    build_state_input_freshness: &mut HashMap<FileFingerprint, bool>,
) -> Result<bool> {
    let Some(state) = previous_build_state else {
        return Ok(false);
    };

    let aux_tool_inputs = aux_tool_input_paths(
        direct.out_dir,
        direct.doc_dir,
        direct.job_name,
        direct.main,
        direct.options,
        Some(direct.aux_session_cache),
    )?;
    if aux_tool_inputs.is_empty() {
        return Ok(false);
    }

    let mut saw_stale_aux_tool_input = false;
    for input in &state.inputs {
        let fresh = input_fingerprint_is_fresh_cached(input, build_state_input_freshness)?;
        if aux_tool_inputs.contains(&input.path) {
            if !fresh {
                saw_stale_aux_tool_input = true;
            }
            continue;
        }
        if !fresh {
            return Ok(false);
        }
    }
    Ok(saw_stale_aux_tool_input)
}

fn aux_tool_input_paths(
    out_dir: &Path,
    doc_dir: &Path,
    job_name: &str,
    main: &Path,
    options: &BuildOptions,
    aux_session_cache: Option<&AuxToolSessionCache>,
) -> Result<HashSet<String>> {
    let mut inputs = Vec::new();
    append_bibtex_inputs(&mut inputs, doc_dir, out_dir, job_name)?;
    append_biber_inputs(&mut inputs, doc_dir, out_dir, job_name)?;
    append_index_inputs(&mut inputs, doc_dir, out_dir, job_name)?;
    append_external_tool_inputs(
        &mut inputs,
        doc_dir,
        out_dir,
        job_name,
        main,
        options,
        aux_session_cache,
    )?;
    Ok(inputs.into_iter().map(|input| input.path).collect())
}

fn previous_build_input_map(
    previous_build_state: Option<&BuildState>,
) -> HashMap<String, FileFingerprint> {
    if let Some(state) = previous_build_state.filter(|state| state.version == BUILD_STATE_VERSION) {
        fingerprint_map(state.inputs.clone())
    } else {
        HashMap::new()
    }
}

fn write_build_state(
    state_path: &Path,
    mode_key: &str,
    pdf_path: &Path,
    direct: DirectContext<'_>,
    previous_build_state: Option<&BuildState>,
    extra_inputs: &[FileFingerprint],
) -> Result<BuildState> {
    let fls_path = direct.out_dir.join(format!("{}.fls", direct.job_name));
    let previous = previous_build_input_map(previous_build_state);
    let mut inputs = recorded_inputs(
        &fls_path,
        direct.doc_dir,
        direct.out_dir,
        direct.main,
        &previous,
    )?;
    append_bibtex_inputs(&mut inputs, direct.doc_dir, direct.out_dir, direct.job_name)?;
    append_biber_inputs(&mut inputs, direct.doc_dir, direct.out_dir, direct.job_name)?;
    append_index_inputs(&mut inputs, direct.doc_dir, direct.out_dir, direct.job_name)?;
    append_external_tool_inputs(
        &mut inputs,
        direct.doc_dir,
        direct.out_dir,
        direct.job_name,
        direct.main,
        direct.options,
        Some(direct.aux_session_cache),
    )?;
    inputs.extend(extra_inputs.iter().cloned());
    write_build_state_from_fingerprints(state_path, mode_key, pdf_path, inputs)
}

fn write_build_state_from_fingerprints(
    state_path: &Path,
    mode_key: &str,
    pdf_path: &Path,
    mut inputs: Vec<FileFingerprint>,
) -> Result<BuildState> {
    inputs.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then_with(|| left.hash.cmp(&right.hash))
    });
    inputs.dedup_by(|left, right| left.path == right.path && left.hash == right.hash);
    let state = BuildState {
        version: BUILD_STATE_VERSION,
        mode_key: mode_key.to_string(),
        pdf_path: pdf_path.display().to_string(),
        inputs,
    };
    let source = toml::to_string(&state).context("failed to serialize build state")?;
    fs::write(state_path, source)
        .with_context(|| format!("failed to write build state {}", state_path.display()))?;
    Ok(state)
}

fn previous_bib_input_map(state_path: &Path) -> Result<HashMap<String, FileFingerprint>> {
    let Some(source) = read_optional_text_file(state_path, "bibliography state")? else {
        return Ok(HashMap::new());
    };
    let state: BibState = toml::from_str(&source).with_context(|| {
        format!(
            "failed to parse bibliography state {}",
            state_path.display()
        )
    })?;
    if state.version == BIB_STATE_VERSION {
        Ok(fingerprint_map(state.inputs))
    } else {
        Ok(HashMap::new())
    }
}

fn previous_index_input_map(state_path: &Path) -> Result<HashMap<String, FileFingerprint>> {
    let Some(source) = read_optional_text_file(state_path, "index state")? else {
        return Ok(HashMap::new());
    };
    let state: IndexState = toml::from_str(&source)
        .with_context(|| format!("failed to parse index state {}", state_path.display()))?;
    if state.version == INDEX_STATE_VERSION {
        Ok(fingerprint_map(state.inputs))
    } else {
        Ok(HashMap::new())
    }
}

fn previous_external_tool_input_map(state_path: &Path) -> Result<HashMap<String, FileFingerprint>> {
    let Some(source) = read_optional_text_file(state_path, "external tool state")? else {
        return Ok(HashMap::new());
    };
    let state: ExternalToolState = toml::from_str(&source).with_context(|| {
        format!(
            "failed to parse external tool state {}",
            state_path.display()
        )
    })?;
    if state.version == EXTERNAL_TOOL_STATE_VERSION {
        Ok(fingerprint_map(state.inputs))
    } else {
        Ok(HashMap::new())
    }
}

fn fingerprint_map(inputs: Vec<FileFingerprint>) -> HashMap<String, FileFingerprint> {
    inputs
        .into_iter()
        .map(|input| (input.path.clone(), input))
        .collect()
}

fn write_native_build_state(
    state_path: &Path,
    mode_key: &str,
    pdf_path: &Path,
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    input_paths: &[PathBuf],
    previous_build_state: Option<&BuildState>,
) -> Result<BuildState> {
    let previous = previous_build_input_map(previous_build_state);
    let inputs = native_input_fingerprints(input_paths, doc_dir, out_dir, main, &previous)?;
    write_build_state_from_fingerprints(state_path, mode_key, pdf_path, inputs)
}

fn native_input_fingerprints(
    input_paths: &[PathBuf],
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    previous: &HashMap<String, FileFingerprint>,
) -> Result<Vec<FileFingerprint>> {
    let main = main.canonicalize().ok();
    let mut inputs = Vec::new();
    for path in input_paths {
        let path = if path.is_absolute() {
            path.clone()
        } else {
            doc_dir.join(path)
        };
        if path_is_under(&path, out_dir) {
            continue;
        }
        let fingerprint = if main.as_ref().is_some_and(|main| path_matches(&path, main)) {
            fingerprint_effective_tex_path_reusing(&path, Some(previous), EffectiveTexMode::Root)?
        } else if is_tex_like_source_input(&path) {
            fingerprint_effective_tex_path_reusing(&path, Some(previous), EffectiveTexMode::Input)?
        } else {
            fingerprint_path_reusing(&path, Some(previous))?
        };
        if let Some(fingerprint) = fingerprint {
            inputs.push(fingerprint);
        }
    }
    inputs.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then_with(|| left.hash.cmp(&right.hash))
    });
    inputs.dedup_by(|left, right| left.path == right.path && left.hash == right.hash);
    Ok(inputs)
}

fn append_bibtex_inputs(
    inputs: &mut Vec<FileFingerprint>,
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<()> {
    for aux_path in bibliography_aux_files(doc_dir, out_dir, job_name)? {
        let state_path = bibtex_job(out_dir, &aux_path, None).state_path;
        if !state_path.exists() {
            continue;
        }
        let source = fs::read_to_string(&state_path).with_context(|| {
            format!("failed to read bibliography state {}", state_path.display())
        })?;
        let state: BibState = toml::from_str(&source).with_context(|| {
            format!(
                "failed to parse bibliography state {}",
                state_path.display()
            )
        })?;
        if state.version == BIB_STATE_VERSION {
            inputs.extend(state.inputs);
        }
    }
    Ok(())
}

fn append_biber_inputs(
    inputs: &mut Vec<FileFingerprint>,
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<()> {
    if biber_control_file_from_latest_run(doc_dir, out_dir, job_name)?.is_none() {
        return Ok(());
    }
    let state_path = out_dir.join(format!(".texpilot-{job_name}.biberstate.toml"));
    if !state_path.exists() {
        return Ok(());
    }
    let source = fs::read_to_string(&state_path)
        .with_context(|| format!("failed to read biber state {}", state_path.display()))?;
    let state: BibState = toml::from_str(&source)
        .with_context(|| format!("failed to parse biber state {}", state_path.display()))?;
    if state.version == BIB_STATE_VERSION {
        inputs.extend(state.inputs);
    }
    Ok(())
}

fn append_index_inputs(
    inputs: &mut Vec<FileFingerprint>,
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<()> {
    for job in makeindex_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        if !job.state_path.exists() {
            continue;
        }
        let source = fs::read_to_string(&job.state_path)
            .with_context(|| format!("failed to read index state {}", job.state_path.display()))?;
        let state: IndexState = toml::from_str(&source)
            .with_context(|| format!("failed to parse index state {}", job.state_path.display()))?;
        if state.version == INDEX_STATE_VERSION {
            inputs.extend(state.inputs);
        }
    }
    Ok(())
}

fn append_external_tool_inputs(
    inputs: &mut Vec<FileFingerprint>,
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    main: &Path,
    options: &BuildOptions,
    aux_session_cache: Option<&AuxToolSessionCache>,
) -> Result<()> {
    if let Some(aux_session_cache) = aux_session_cache {
        let jobs = source_external_tool_jobs(doc_dir, out_dir, main, options, aux_session_cache)?;
        for job in jobs.eps {
            append_external_tool_state_inputs(inputs, &job.state_path)?;
        }
        for job in jobs.svg {
            append_external_tool_state_inputs(inputs, &job.state_path)?;
        }
    } else {
        if !options.fast && uses_pdftex_graphics_pipeline(options.engine) {
            for job in eps_conversion_jobs_from_source(doc_dir, out_dir, main)? {
                append_external_tool_state_inputs(inputs, &job.state_path)?;
            }
        }
        if !options.fast {
            for job in svg_conversion_jobs_from_source(doc_dir, out_dir, main)? {
                append_external_tool_state_inputs(inputs, &job.state_path)?;
            }
        }
    }
    for job in asymptote_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        append_external_tool_state_inputs(inputs, &job.state_path)?;
    }
    for job in pythontex_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        append_external_tool_state_inputs(inputs, &job.state_path)?;
    }
    for job in metapost_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        append_external_tool_state_inputs(inputs, &job.state_path)?;
    }
    for job in gnuplottex_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        append_external_tool_state_inputs(inputs, &job.state_path)?;
    }
    if let Some(job) = bib2gls_job_from_latest_run(doc_dir, out_dir, job_name)? {
        append_external_tool_state_inputs(inputs, &job.state_path)?;
    }
    Ok(())
}

fn append_external_tool_state_inputs(
    inputs: &mut Vec<FileFingerprint>,
    state_path: &Path,
) -> Result<()> {
    if !state_path.exists() {
        return Ok(());
    }
    let source = fs::read_to_string(state_path).with_context(|| {
        format!(
            "failed to read external tool state {}",
            state_path.display()
        )
    })?;
    let state: ExternalToolState = toml::from_str(&source).with_context(|| {
        format!(
            "failed to parse external tool state {}",
            state_path.display()
        )
    })?;
    if state.version == EXTERNAL_TOOL_STATE_VERSION {
        inputs.extend(state.inputs);
    }
    Ok(())
}

fn recorded_inputs(
    fls_path: &Path,
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    previous: &HashMap<String, FileFingerprint>,
) -> Result<Vec<FileFingerprint>> {
    recorded_inputs_with_root_mode(
        fls_path,
        doc_dir,
        out_dir,
        main,
        previous,
        EffectiveTexMode::Root,
    )
}

fn recorded_preamble_format_inputs(
    fls_path: &Path,
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    previous: &HashMap<String, FileFingerprint>,
) -> Result<Vec<FileFingerprint>> {
    recorded_inputs_with_root_mode(
        fls_path,
        doc_dir,
        out_dir,
        main,
        previous,
        EffectiveTexMode::Preamble,
    )
}

fn recorded_inputs_with_root_mode(
    fls_path: &Path,
    doc_dir: &Path,
    out_dir: &Path,
    main: &Path,
    previous: &HashMap<String, FileFingerprint>,
    root_mode: EffectiveTexMode,
) -> Result<Vec<FileFingerprint>> {
    let source = fs::read_to_string(fls_path)
        .with_context(|| format!("failed to read recorder file {}", fls_path.display()))?;
    let mut inputs = Vec::new();
    let main = main.canonicalize().ok();
    for line in source.lines() {
        let Some(raw_path) = line.strip_prefix("INPUT ") else {
            continue;
        };
        let path = resolve_recorded_path(raw_path, doc_dir);
        if path_is_under(&path, out_dir) {
            continue;
        }
        let fingerprint = if main.as_ref().is_some_and(|main| path_matches(&path, main)) {
            fingerprint_effective_tex_path_reusing(&path, Some(previous), root_mode)?
        } else if is_tex_like_source_input(&path) {
            fingerprint_effective_tex_path_reusing(&path, Some(previous), EffectiveTexMode::Input)?
        } else {
            fingerprint_path_reusing(&path, Some(previous))?
        };
        if let Some(fingerprint) = fingerprint {
            inputs.push(fingerprint);
        }
    }
    inputs.sort_by(|left, right| left.path.cmp(&right.path));
    inputs.dedup_by(|left, right| left.path == right.path);
    Ok(inputs)
}

fn recorded_outputs(fls_path: &Path, doc_dir: &Path) -> Result<Vec<PathBuf>> {
    let source = fs::read_to_string(fls_path)
        .with_context(|| format!("failed to read recorder file {}", fls_path.display()))?;
    let mut outputs = Vec::new();
    for line in source.lines() {
        let Some(raw_path) = line.strip_prefix("OUTPUT ") else {
            continue;
        };
        outputs.push(resolve_recorded_path(raw_path, doc_dir));
    }
    outputs.sort();
    outputs.dedup();
    Ok(outputs)
}

fn recorded_input_paths(fls_path: &Path, doc_dir: &Path) -> Result<Vec<PathBuf>> {
    let source = fs::read_to_string(fls_path)
        .with_context(|| format!("failed to read recorder file {}", fls_path.display()))?;
    let mut inputs = Vec::new();
    for line in source.lines() {
        let Some(raw_path) = line.strip_prefix("INPUT ") else {
            continue;
        };
        inputs.push(resolve_recorded_path(raw_path, doc_dir));
    }
    inputs.sort();
    inputs.dedup();
    Ok(inputs)
}

fn aux_output_snapshot(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
    main: &Path,
    options: &BuildOptions,
    aux_session_cache: &AuxToolSessionCache,
) -> Result<Vec<GeneratedOutputFingerprint>> {
    let mut paths = Vec::new();
    let source_external_jobs =
        source_external_tool_jobs(doc_dir, out_dir, main, options, aux_session_cache)?;
    for job in source_external_jobs.eps {
        paths.push(job.output_path);
    }
    for job in source_external_jobs.svg {
        paths.extend(job.output_paths());
    }
    for aux_path in bibliography_aux_files(doc_dir, out_dir, job_name)? {
        paths.push(bibtex_job(out_dir, &aux_path, None).bbl_path);
    }
    if biber_control_file_from_latest_run(doc_dir, out_dir, job_name)?.is_some() {
        let biber_bbl = out_dir.join(format!("{job_name}.bbl"));
        paths.push(biber_bbl);
    }
    for job in makeindex_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        paths.push(job.output_path);
    }
    for job in asymptote_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        paths.push(job.output_path);
    }
    for job in pythontex_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        paths.extend(job.output_paths);
    }
    for job in metapost_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        paths.extend(job.output_paths);
    }
    for job in gnuplottex_jobs_from_latest_run(doc_dir, out_dir, job_name)? {
        paths.push(job.output_path);
    }
    if let Some(job) = pgf_external_job_from_latest_run(out_dir, job_name)? {
        paths.extend(job.output_paths);
    }
    if let Some(job) = bib2gls_job_from_latest_run(doc_dir, out_dir, job_name)? {
        paths.extend(job.output_paths);
    }
    output_file_snapshot(paths)
}

fn standard_rerun_output_snapshot(out_dir: &Path) -> Result<Vec<GeneratedOutputFingerprint>> {
    let mut paths = Vec::new();
    collect_standard_rerun_output_paths(out_dir, &mut paths)?;
    output_file_snapshot(paths)
}

fn collect_standard_rerun_output_paths(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<()> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries {
        let entry = entry.with_context(|| format!("failed to read {}", dir.display()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to inspect {}", path.display()))?;
        if file_type.is_dir() {
            collect_standard_rerun_output_paths(&path, paths)?;
        } else if file_type.is_file() && is_standard_rerun_output(&path) {
            paths.push(path);
        }
    }
    Ok(())
}

fn is_standard_rerun_output(path: &Path) -> bool {
    path_extension_is_any(
        path,
        &[
            "aux", "out", "toc", "lof", "lot", "brf", "nav", "snm", "vrb", "thm",
        ],
    )
}

fn output_file_snapshot<I>(paths: I) -> Result<Vec<GeneratedOutputFingerprint>>
where
    I: IntoIterator<Item = PathBuf>,
{
    let mut paths = paths.into_iter().collect::<Vec<_>>();
    paths.sort();
    paths.dedup();

    let mut fingerprints = if paths.len() >= 16 {
        parallel_output_file_snapshot(&paths)?
    } else {
        let mut fingerprints = Vec::new();
        for path in paths {
            if let Some(fingerprint) = generated_output_fingerprint(&path)? {
                fingerprints.push(fingerprint);
            }
        }
        fingerprints
    };
    fingerprints.sort_by(|left, right| left.path.cmp(&right.path));
    fingerprints.dedup_by(|left, right| left.path == right.path);
    Ok(fingerprints)
}

fn parallel_output_file_snapshot(paths: &[PathBuf]) -> Result<Vec<GeneratedOutputFingerprint>> {
    let worker_count = thread::available_parallelism()
        .map(|parallelism| parallelism.get())
        .unwrap_or(1)
        .min(paths.len());
    if worker_count <= 1 {
        let mut fingerprints = Vec::new();
        for path in paths {
            if let Some(fingerprint) = generated_output_fingerprint(path)? {
                fingerprints.push(fingerprint);
            }
        }
        return Ok(fingerprints);
    }

    let chunk_size = paths.len().div_ceil(worker_count);
    thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in paths.chunks(chunk_size) {
            handles.push(scope.spawn(move || {
                let mut fingerprints = Vec::new();
                for path in chunk {
                    if let Some(fingerprint) = generated_output_fingerprint(path)? {
                        fingerprints.push(fingerprint);
                    }
                }
                Ok::<_, anyhow::Error>(fingerprints)
            }));
        }

        let mut fingerprints = Vec::new();
        for handle in handles {
            fingerprints.extend(
                handle
                    .join()
                    .map_err(|_| anyhow!("generated-output snapshot worker panicked"))??,
            );
        }
        Ok(fingerprints)
    })
}

fn generated_output_fingerprint(path: &Path) -> Result<Option<GeneratedOutputFingerprint>> {
    if !path.is_file() {
        return Ok(None);
    }
    let bytes = fs::read(path)
        .with_context(|| format!("failed to read generated output {}", path.display()))?;
    Ok(Some(GeneratedOutputFingerprint {
        path: canonical_or_original(path).display().to_string(),
        len: bytes.len() as u64,
        hash: content_hash(&bytes),
    }))
}

fn generated_output_snapshot(
    out_dir: &Path,
    doc_dir: &Path,
    job_name: &str,
) -> Result<Vec<GeneratedOutputFingerprint>> {
    let fls_path = out_dir.join(format!("{job_name}.fls"));
    let mut paths = if fls_path.exists() {
        recorded_outputs(&fls_path, doc_dir)?
    } else {
        Vec::new()
    };
    paths.extend(minted_cache_paths_from_latest_run(
        doc_dir, out_dir, job_name,
    )?);

    paths.sort();
    paths.dedup();

    output_file_snapshot(paths.into_iter().filter(|path| is_convergence_output(path)))
}

fn generated_inputs_unread_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<bool> {
    let fls_path = out_dir.join(format!("{job_name}.fls"));
    if !fls_path.exists() {
        return Ok(false);
    }
    let minted_paths = minted_cache_paths_from_latest_run(doc_dir, out_dir, job_name)?;
    if minted_paths.is_empty() {
        return Ok(false);
    }
    let recorded_inputs = recorded_input_paths(&fls_path, doc_dir)?
        .into_iter()
        .map(|path| canonical_or_original(&path).display().to_string())
        .collect::<HashSet<_>>();
    Ok(minted_paths
        .into_iter()
        .filter(|path| path.is_file())
        .any(|path| !recorded_inputs.contains(&canonical_or_original(&path).display().to_string())))
}

fn minted_cache_paths_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Vec<PathBuf>> {
    let hashes = minted_hashes_from_latest_run(doc_dir, out_dir, job_name)?;
    if hashes.is_empty() {
        return Ok(Vec::new());
    }
    let cache_dir = out_dir.join("_minted");
    let mut paths = Vec::new();
    for hash in hashes {
        let index_path = cache_dir.join(format!("_{hash}.index.minted"));
        if !index_path.is_file() {
            continue;
        }
        paths.push(index_path.clone());
        let source = fs::read_to_string(&index_path).with_context(|| {
            format!("failed to read minted cache index {}", index_path.display())
        })?;
        let index: MintedCacheIndex = serde_json::from_str(&source).with_context(|| {
            format!(
                "failed to parse minted cache index {}",
                index_path.display()
            )
        })?;
        for cachefile in index.cachefiles {
            let cachefile_path = Path::new(&cachefile);
            if safe_relative_path(cachefile_path) {
                paths.push(cache_dir.join(cachefile_path));
            }
        }
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn minted_hashes_from_latest_run(
    doc_dir: &Path,
    out_dir: &Path,
    job_name: &str,
) -> Result<Vec<String>> {
    let fls_path = out_dir.join(format!("{job_name}.fls"));
    if !fls_path.exists() {
        return Ok(Vec::new());
    }
    let source = fs::read_to_string(&fls_path)
        .with_context(|| format!("failed to read recorder file {}", fls_path.display()))?;
    let mut hashes = Vec::new();
    for line in source.lines() {
        let Some(raw_path) = line
            .strip_prefix("INPUT ")
            .or_else(|| line.strip_prefix("OUTPUT "))
        else {
            continue;
        };
        let path = resolve_recorded_path(raw_path, doc_dir);
        if let Some(hash) = minted_hash_from_path(&path) {
            hashes.push(hash);
        }
    }
    hashes.sort();
    hashes.dedup();
    Ok(hashes)
}

fn minted_hash_from_path(path: &Path) -> Option<String> {
    let file_name = path.file_name()?.to_str()?;
    let hash = file_name
        .strip_prefix('_')?
        .strip_suffix(".data.minted")
        .or_else(|| file_name.strip_prefix('_')?.strip_suffix(".config.minted"))
        .or_else(|| file_name.strip_prefix('_')?.strip_suffix(".index.minted"))?;
    if hash.len() == 32 && hash.chars().all(|ch| ch.is_ascii_hexdigit()) {
        Some(hash.to_ascii_uppercase())
    } else {
        None
    }
}

fn is_convergence_output(path: &Path) -> bool {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if file_name.ends_with(".synctex.gz") || file_name.ends_with(".fdb_latexmk") {
        return false;
    }
    if file_name.ends_with(".run.xml") {
        return false;
    }
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    !matches!(
        extension.as_str(),
        "aux"
            | "out"
            | "toc"
            | "lof"
            | "lot"
            | "lol"
            | "brf"
            | "bcf"
            | "maf"
            | "mtc"
            | "mtc0"
            | "idx"
            | "glo"
            | "acn"
            | "nlo"
            | "ind"
            | "gls"
            | "acr"
            | "nls"
            | "bbl"
            | "xml"
            | "toml"
            | "md5"
            | "stderr"
            | "stdout"
            | "tmp"
            | "log"
            | "fls"
            | "pdf"
            | "dvi"
            | "xdv"
            | "ps"
            | "gz"
            | "blg"
            | "ilg"
            | "glg"
            | "alg"
            | "nlg"
    )
}

fn canonical_or_original(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn path_matches(path: &Path, other: &Path) -> bool {
    let Ok(path) = path.canonicalize() else {
        return false;
    };
    let Ok(other) = other.canonicalize() else {
        return false;
    };
    path == other
}

fn content_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn resolve_recorded_path(raw_path: &str, doc_dir: &Path) -> PathBuf {
    let path = Path::new(raw_path);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        doc_dir.join(path)
    }
}

fn path_is_under(path: &Path, directory: &Path) -> bool {
    let Ok(path) = path.canonicalize() else {
        return path.starts_with(directory);
    };
    let Ok(directory) = directory.canonicalize() else {
        return path.starts_with(directory);
    };
    path.starts_with(directory)
}

fn same_existing_path(left: &Path, right: &Path) -> bool {
    let Ok(left) = left.canonicalize() else {
        return false;
    };
    let Ok(right) = right.canonicalize() else {
        return false;
    };
    left == right
}

fn fingerprint_path_reusing(
    path: &Path,
    previous: Option<&HashMap<String, FileFingerprint>>,
) -> Result<Option<FileFingerprint>> {
    let Ok(canonical) = path.canonicalize() else {
        return Ok(None);
    };
    let Some((len, modified_ns)) = file_metadata_fingerprint(&canonical)? else {
        return Ok(None);
    };
    let path = canonical.display().to_string();
    let hash = previous
        .and_then(|previous| previous.get(&path))
        .filter(|fingerprint| {
            fingerprint.len == len
                && fingerprint.modified_ns == modified_ns
                && !fingerprint.hash.is_empty()
        })
        .map(|fingerprint| fingerprint.hash.clone())
        .map(Ok)
        .unwrap_or_else(|| file_content_hash_hex(&canonical))?;
    Ok(Some(FileFingerprint {
        path,
        len,
        modified_ns,
        hash,
    }))
}

#[derive(Debug, Clone, Copy)]
enum EffectiveTexMode {
    Root,
    Preamble,
    Input,
}

impl EffectiveTexMode {
    fn hash_prefix(self) -> &'static str {
        match self {
            EffectiveTexMode::Root => TEX_ROOT_EFFECTIVE_HASH_PREFIX,
            EffectiveTexMode::Preamble => TEX_PREAMBLE_EFFECTIVE_HASH_PREFIX,
            EffectiveTexMode::Input => TEX_INPUT_EFFECTIVE_HASH_PREFIX,
        }
    }
}

fn fingerprint_effective_tex_path_reusing(
    path: &Path,
    previous: Option<&HashMap<String, FileFingerprint>>,
    mode: EffectiveTexMode,
) -> Result<Option<FileFingerprint>> {
    let Ok(canonical) = path.canonicalize() else {
        return Ok(None);
    };
    let Some((_, modified_ns)) = file_metadata_fingerprint(&canonical)? else {
        return Ok(None);
    };
    let path = canonical.display().to_string();
    let hash_prefix = mode.hash_prefix();
    if let Some(fingerprint) =
        previous
            .and_then(|previous| previous.get(&path))
            .filter(|fingerprint| {
                fingerprint.modified_ns == modified_ns && fingerprint.hash.starts_with(hash_prefix)
            })
    {
        return Ok(Some(fingerprint.clone()));
    }
    let bytes = fs::read(&canonical)
        .with_context(|| format!("failed to read TeX source {}", canonical.display()))?;
    let effective = effective_tex_bytes(&bytes, mode);
    let len = effective.len() as u64;
    let hash = previous
        .and_then(|previous| previous.get(&path))
        .filter(|fingerprint| {
            fingerprint.len == len
                && fingerprint.modified_ns == modified_ns
                && fingerprint.hash.starts_with(hash_prefix)
        })
        .map(|fingerprint| fingerprint.hash.clone())
        .unwrap_or_else(|| format!("{hash_prefix}{:016x}", content_hash(&effective)));
    Ok(Some(FileFingerprint {
        path,
        len,
        modified_ns,
        hash,
    }))
}

fn input_fingerprint_is_fresh(input: &FileFingerprint) -> Result<bool> {
    if let Some(expected_hash) = input.hash.strip_prefix(BIBER_GLOB_MATCHES_HASH_PREFIX) {
        let Some((doc_dir, pattern)) = decode_biber_glob_fingerprint_path(&input.path) else {
            return Ok(false);
        };
        let paths = resolve_biber_glob_datasource(&doc_dir, &pattern)?;
        let current = fingerprint_biber_glob_matches(&doc_dir, &pattern, &paths)?;
        let Some(current_hash) = current.hash.strip_prefix(BIBER_GLOB_MATCHES_HASH_PREFIX) else {
            return Ok(false);
        };
        return Ok(current.len == input.len && current_hash == expected_hash);
    }
    if let Some(expected_hash) = input.hash.strip_prefix(BIBER_CONFIG_CHOICE_HASH_PREFIX) {
        let Some(doc_dir) = decode_biber_config_fingerprint_path(&input.path) else {
            return Ok(false);
        };
        let config_path = resolve_biber_config_path(&doc_dir)?;
        let current = fingerprint_biber_config_choice(&doc_dir, config_path.as_deref());
        let Some(current_hash) = current.hash.strip_prefix(BIBER_CONFIG_CHOICE_HASH_PREFIX) else {
            return Ok(false);
        };
        return Ok(current.len == input.len && current_hash == expected_hash);
    }

    let path = Path::new(&input.path);
    if let Some((expected_hash, mode)) = input
        .hash
        .strip_prefix(TEX_ROOT_EFFECTIVE_HASH_PREFIX)
        .map(|hash| (hash, EffectiveTexMode::Root))
        .or_else(|| {
            input
                .hash
                .strip_prefix(TEX_PREAMBLE_EFFECTIVE_HASH_PREFIX)
                .map(|hash| (hash, EffectiveTexMode::Preamble))
        })
        .or_else(|| {
            input
                .hash
                .strip_prefix(TEX_INPUT_EFFECTIVE_HASH_PREFIX)
                .map(|hash| (hash, EffectiveTexMode::Input))
        })
    {
        let Some((_, modified_ns)) = file_metadata_fingerprint(path)? else {
            return Ok(false);
        };
        if modified_ns == input.modified_ns {
            return Ok(true);
        }
        let bytes =
            fs::read(path).with_context(|| format!("failed to read input {}", path.display()))?;
        let effective = effective_tex_bytes(&bytes, mode);
        if effective.len() as u64 != input.len {
            return Ok(false);
        }
        return Ok(format!("{:016x}", content_hash(&effective)) == expected_hash);
    }
    if let Some(encoded) = input.hash.strip_prefix(BIB_CITED_EFFECTIVE_HASH_PREFIX) {
        let Some((encoded_keys, expected_hash)) = encoded.rsplit_once(':') else {
            return Ok(false);
        };
        let Some(citation_keys) = decode_bibtex_citation_keys(encoded_keys) else {
            return Ok(false);
        };
        let Some((_, modified_ns)) = file_metadata_fingerprint(path)? else {
            return Ok(false);
        };
        if modified_ns == input.modified_ns {
            return Ok(true);
        }
        let bytes =
            fs::read(path).with_context(|| format!("failed to read input {}", path.display()))?;
        let Some(effective) = cited_bibtex_bytes(&bytes, &citation_keys) else {
            return Ok(false);
        };
        if effective.len() as u64 != input.len {
            return Ok(false);
        }
        return Ok(format!("{:016x}", content_hash(&effective)) == expected_hash);
    }

    let Some((len, modified_ns)) = file_metadata_fingerprint(path)? else {
        return Ok(false);
    };
    if len != input.len {
        return Ok(false);
    }
    if modified_ns == input.modified_ns {
        return Ok(true);
    }
    Ok(file_content_hash_hex(path)? == input.hash)
}

fn is_virtual_fingerprint_path(path: &str) -> bool {
    path.starts_with(BIBER_GLOB_FINGERPRINT_PATH_PREFIX)
        || path.starts_with(BIBER_CONFIG_FINGERPRINT_PATH_PREFIX)
}

fn input_fingerprint_is_fresh_cached(
    input: &FileFingerprint,
    build_state_input_freshness: &mut HashMap<FileFingerprint, bool>,
) -> Result<bool> {
    if let Some(fresh) = build_state_input_freshness.get(input) {
        return Ok(*fresh);
    }
    let fresh = input_fingerprint_is_fresh(input)?;
    build_state_input_freshness.insert(input.clone(), fresh);
    Ok(fresh)
}

fn effective_tex_bytes(bytes: &[u8], mode: EffectiveTexMode) -> Vec<u8> {
    let mut offset = 0;
    let mut effective = Vec::with_capacity(bytes.len());
    let policy = effective_tex_policy(bytes);
    for line in bytes.split_inclusive(|byte| *byte == b'\n') {
        let visible = strip_tex_comment_bytes(line);
        if policy.stop_at_boundaries
            && matches!(mode, EffectiveTexMode::Preamble)
            && let Some(begin_offset) = begin_document_end_offset_bytes(visible)
        {
            append_canonical_tex_line(
                &mut effective,
                &bytes[offset..offset + begin_offset],
                policy,
            );
            return effective;
        }
        if policy.stop_at_boundaries
            && matches!(mode, EffectiveTexMode::Root)
            && let Some(end_offset) = end_document_end_offset_bytes(visible)
        {
            append_canonical_tex_line(&mut effective, &bytes[offset..offset + end_offset], policy);
            return effective;
        }
        if policy.stop_at_boundaries && line_contains_endinput_bytes(visible) {
            append_canonical_tex_line(&mut effective, line, policy);
            return effective;
        }
        append_canonical_tex_line(&mut effective, line, policy);
        offset += line.len();
    }
    effective
}

#[derive(Debug, Clone, Copy)]
struct EffectiveTexPolicy {
    stop_at_boundaries: bool,
    ignore_comment_text: bool,
}

fn effective_tex_policy(bytes: &[u8]) -> EffectiveTexPolicy {
    let has_explicit_catcode = bytes
        .windows(b"\\catcode".len())
        .any(|window| window == b"\\catcode");
    let has_catcode_sensitive_construct = [
        b"\\begin{verbatim}".as_slice(),
        b"\\begin {verbatim}".as_slice(),
        b"\\begin{lstlisting}".as_slice(),
        b"\\begin {lstlisting}".as_slice(),
        b"\\begin{minted}".as_slice(),
        b"\\begin {minted}".as_slice(),
        b"\\begin{filecontents}".as_slice(),
        b"\\begin {filecontents}".as_slice(),
        b"\\begin{filecontents*}".as_slice(),
        b"\\begin {filecontents*}".as_slice(),
        b"\\begin{comment}".as_slice(),
        b"\\begin {comment}".as_slice(),
    ]
    .into_iter()
    .any(|needle| bytes.windows(needle.len()).any(|window| window == needle));
    let conservative = has_explicit_catcode
        || has_catcode_sensitive_construct
        || has_unparseable_inline_literal(bytes);
    EffectiveTexPolicy {
        stop_at_boundaries: !conservative,
        ignore_comment_text: !conservative,
    }
}

fn append_canonical_tex_line(output: &mut Vec<u8>, line: &[u8], policy: EffectiveTexPolicy) {
    let (body, ending) = if let Some(body) = line.strip_suffix(b"\r\n") {
        (body, &b"\r\n"[..])
    } else if let Some(body) = line.strip_suffix(b"\n") {
        (body, &b"\n"[..])
    } else if let Some(body) = line.strip_suffix(b"\r") {
        (body, &b"\r"[..])
    } else {
        (line, &b""[..])
    };
    if policy.ignore_comment_text
        && let Some(comment_start) = tex_comment_start_bytes(body)
    {
        if comment_start > 0 {
            append_canonical_inline_comment_prefix(output, &body[..comment_start]);
            output.push(b'%');
        }
        return;
    }
    let mut body_end = body.len();
    while body_end > 0 && body[body_end - 1] == b' ' {
        body_end -= 1;
    }
    let body = &body[..body_end];
    output.extend_from_slice(body);
    output.extend_from_slice(ending);
}

fn append_canonical_inline_comment_prefix(output: &mut Vec<u8>, prefix: &[u8]) {
    let mut trimmed_end = prefix.len();
    while trimmed_end > 0 && prefix[trimmed_end - 1] == b' ' {
        trimmed_end -= 1;
    }
    if trimmed_end == 0 {
        output.extend_from_slice(prefix);
        return;
    }
    output.extend_from_slice(&prefix[..trimmed_end]);
    if trimmed_end < prefix.len() {
        output.push(b' ');
    }
}

fn strip_tex_comment_bytes(line: &[u8]) -> &[u8] {
    if let Some(comment_start) = tex_comment_start_bytes(line) {
        return &line[..comment_start];
    }
    line
}

fn tex_comment_start_bytes(line: &[u8]) -> Option<usize> {
    let mut escaped = false;
    let mut index = 0;
    while index < line.len() {
        if line[index] == b'\\'
            && !escaped
            && let Some(end) = inline_literal_span_end_bytes(line, index)
        {
            index = end;
            escaped = false;
            continue;
        }
        let byte = line[index];
        if byte == b'%' && !escaped {
            return Some(index);
        }
        if byte == b'\\' {
            escaped = !escaped;
        } else {
            escaped = false;
        }
        index += 1;
    }
    None
}

fn begin_document_end_offset_bytes(line: &[u8]) -> Option<usize> {
    document_environment_command_end_offset_bytes(line, b"\\begin")
}

fn end_document_end_offset_bytes(line: &[u8]) -> Option<usize> {
    document_environment_command_end_offset_bytes(line, b"\\end")
}

fn document_environment_command_end_offset_bytes(line: &[u8], command: &[u8]) -> Option<usize> {
    let mut cursor = 0;
    while cursor < line.len() {
        if let Some(end) = inline_literal_span_end_bytes(line, cursor) {
            cursor = end;
            continue;
        }
        let command_start = cursor;
        if !line[command_start..].starts_with(command) {
            cursor += 1;
            continue;
        }
        let after_command = command_start + command.len();
        if line.get(after_command).is_some_and(u8::is_ascii_alphabetic) {
            cursor = after_command;
            continue;
        }
        let open = skip_tex_whitespace_bytes(line, after_command);
        let Some((payload, end)) = braced_tex_payload_bytes(line, open) else {
            cursor = after_command;
            continue;
        };
        if payload == b"document" {
            return Some(end);
        }
        cursor = end;
    }
    None
}

fn line_contains_endinput_bytes(line: &[u8]) -> bool {
    let command = b"\\endinput";
    let mut cursor = 0;
    while cursor < line.len() {
        if let Some(end) = inline_literal_span_end_bytes(line, cursor) {
            cursor = end;
            continue;
        }
        let command_start = cursor;
        if !line[command_start..].starts_with(command) {
            cursor += 1;
            continue;
        }
        let after_command = command_start + command.len();
        if line.get(after_command).is_some_and(u8::is_ascii_alphabetic) {
            cursor = after_command;
            continue;
        }
        return true;
    }
    false
}

fn has_unparseable_inline_literal(bytes: &[u8]) -> bool {
    for line in bytes.split_inclusive(|byte| *byte == b'\n') {
        let mut cursor = 0;
        while cursor < line.len() {
            if starts_inline_literal_command_bytes(line, cursor) {
                let Some(end) = inline_literal_span_end_bytes(line, cursor) else {
                    return true;
                };
                cursor = end;
                continue;
            }
            cursor += 1;
        }
    }
    false
}

fn inline_literal_span_end_bytes(line: &[u8], command_start: usize) -> Option<usize> {
    if let Some(mut delimiter) = inline_command_end_bytes(line, command_start, b"\\verb") {
        if line.get(delimiter) == Some(&b'*') {
            delimiter += 1;
        }
        return delimited_inline_span_end_bytes(line, delimiter);
    }

    if let Some(mut delimiter) = inline_command_end_bytes(line, command_start, b"\\lstinline") {
        delimiter = skip_tex_whitespace_bytes(line, delimiter);
        if line.get(delimiter) == Some(&b'[') {
            delimiter = bracketed_tex_argument_end_bytes(line, delimiter)?;
            delimiter = skip_tex_whitespace_bytes(line, delimiter);
        }
        return delimited_inline_span_end_bytes(line, delimiter);
    }

    for command in [b"\\mintinline".as_slice(), b"\\mint".as_slice()] {
        if let Some(mut delimiter) = inline_command_end_bytes(line, command_start, command) {
            delimiter = skip_tex_whitespace_bytes(line, delimiter);
            if line.get(delimiter) == Some(&b'[') {
                delimiter = bracketed_tex_argument_end_bytes(line, delimiter)?;
                delimiter = skip_tex_whitespace_bytes(line, delimiter);
            }
            delimiter = braced_tex_argument_end_bytes(line, delimiter)?;
            delimiter = skip_tex_whitespace_bytes(line, delimiter);
            return delimited_inline_span_end_bytes(line, delimiter);
        }
    }

    None
}

fn delimited_inline_span_end_bytes(line: &[u8], delimiter: usize) -> Option<usize> {
    let delimiter_byte = *line.get(delimiter)?;
    if delimiter_byte == b'\\'
        || delimiter_byte == b'\r'
        || delimiter_byte == b'\n'
        || delimiter_byte.is_ascii_alphanumeric()
    {
        return None;
    }
    line[delimiter + 1..]
        .iter()
        .position(|byte| *byte == delimiter_byte)
        .map(|offset| delimiter + 1 + offset + 1)
}

fn starts_inline_literal_command_bytes(line: &[u8], command_start: usize) -> bool {
    inline_command_end_bytes(line, command_start, b"\\verb").is_some()
        || inline_command_end_bytes(line, command_start, b"\\lstinline").is_some()
        || inline_command_end_bytes(line, command_start, b"\\mintinline").is_some()
        || inline_command_end_bytes(line, command_start, b"\\mint").is_some()
}

fn inline_command_end_bytes(line: &[u8], command_start: usize, command: &[u8]) -> Option<usize> {
    if !line.get(command_start..)?.starts_with(command) {
        return None;
    }
    let after_command = command_start + command.len();
    if line.get(after_command).is_some_and(u8::is_ascii_alphabetic) {
        return None;
    }
    Some(after_command)
}

fn bracketed_tex_argument_end_bytes(line: &[u8], open: usize) -> Option<usize> {
    balanced_tex_argument_end_bytes(line, open, b'[', b']')
}

fn braced_tex_argument_end_bytes(line: &[u8], open: usize) -> Option<usize> {
    balanced_tex_argument_end_bytes(line, open, b'{', b'}')
}

fn balanced_tex_argument_end_bytes(line: &[u8], open: usize, left: u8, right: u8) -> Option<usize> {
    if line.get(open) != Some(&left) {
        return None;
    }
    let mut depth = 0_u32;
    let mut index = open;
    let mut escaped = false;
    while index < line.len() {
        let byte = line[index];
        if byte == left && !escaped {
            depth += 1;
        } else if byte == right && !escaped {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(index + 1);
            }
        }
        if byte == b'\\' {
            escaped = !escaped;
        } else {
            escaped = false;
        }
        index += 1;
    }
    None
}

fn skip_tex_whitespace_bytes(line: &[u8], start: usize) -> usize {
    let mut cursor = start;
    while line.get(cursor).is_some_and(u8::is_ascii_whitespace) {
        cursor += 1;
    }
    cursor
}

fn braced_tex_payload_bytes(line: &[u8], open: usize) -> Option<(&[u8], usize)> {
    if line.get(open).is_none_or(|byte| *byte != b'{') {
        return None;
    }
    let payload_start = open + 1;
    let close_offset = line[payload_start..]
        .iter()
        .position(|byte| *byte == b'}')?;
    let payload_end = payload_start + close_offset;
    let payload = trim_ascii_bytes(&line[payload_start..payload_end]);
    Some((payload, payload_end + 1))
}

fn trim_ascii_bytes(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let end = bytes
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace())
        .map(|index| index + 1)
        .unwrap_or(start);
    &bytes[start..end]
}

fn is_tex_like_source_input(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("tex" | "ltx" | "sty" | "cls" | "def" | "cfg" | "clo")
    )
}

fn file_metadata_fingerprint(path: &Path) -> Result<Option<(u64, u64)>> {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(error).with_context(|| format!("failed to stat input {}", path.display()));
        }
    };
    if !metadata.is_file() {
        return Ok(None);
    }
    let modified_ns = metadata
        .modified()
        .with_context(|| format!("failed to read mtime for {}", path.display()))?
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    Ok(Some((metadata.len(), modified_ns)))
}

fn file_content_hash_hex(path: &Path) -> Result<String> {
    let bytes =
        fs::read(path).with_context(|| format!("failed to read input {}", path.display()))?;
    Ok(format!("{:016x}", content_hash(&bytes)))
}

fn kpathsea_env(var: &str, doc_dir: &Path) -> OsString {
    kpathsea_env_with_existing(doc_dir, std::env::var_os(var).as_deref())
}

fn texinputs_env(doc_dir: &Path, out_dir: &Path) -> OsString {
    let existing = std::env::var_os("TEXINPUTS");
    let mut value = OsString::from(format!(
        "{}//{}{}//{}",
        doc_dir.display(),
        KPATHSEA_PATH_SEPARATOR,
        out_dir.display(),
        KPATHSEA_PATH_SEPARATOR
    ));
    if let Some(existing) = existing.filter(|value| !value.is_empty()) {
        value.push(existing);
    }
    value
}

fn texinputs_env_with_format_dir(doc_dir: &Path, out_dir: &Path, format_dir: &Path) -> OsString {
    let existing = std::env::var_os("TEXINPUTS");
    let mut value = OsString::from(format!(
        "{}//{}{}//{}{}//{}",
        format_dir.display(),
        KPATHSEA_PATH_SEPARATOR,
        doc_dir.display(),
        KPATHSEA_PATH_SEPARATOR,
        out_dir.display(),
        KPATHSEA_PATH_SEPARATOR
    ));
    if let Some(existing) = existing.filter(|value| !value.is_empty()) {
        value.push(existing);
    }
    value
}

fn texformats_env(format_dir: &Path) -> OsString {
    let existing = std::env::var_os("TEXFORMATS");
    let mut value = OsString::from(format!(
        "{}{}",
        format_dir.display(),
        KPATHSEA_PATH_SEPARATOR
    ));
    if let Some(existing) = existing.filter(|value| !value.is_empty()) {
        value.push(existing);
    }
    value
}

fn kpathsea_env_with_fallback_vars(primary: &str, doc_dir: &Path, fallbacks: &[&str]) -> OsString {
    let existing = std::env::var_os(primary).filter(|value| !value.is_empty());
    let existing = existing.or_else(|| {
        fallbacks
            .iter()
            .find_map(|fallback| std::env::var_os(fallback).filter(|value| !value.is_empty()))
    });
    kpathsea_env_with_existing(doc_dir, existing.as_deref())
}

fn kpathsea_env_with_existing(doc_dir: &Path, existing: Option<&OsStr>) -> OsString {
    let mut value = OsString::from(format!(
        "{}//{}",
        doc_dir.display(),
        KPATHSEA_PATH_SEPARATOR
    ));
    if let Some(existing) = existing.filter(|value| !value.is_empty()) {
        value.push(existing);
    }
    value
}

fn kpathsea_env_overrides_for_extension(
    extension: &str,
    doc_dir: &Path,
) -> Vec<(&'static str, OsString)> {
    match extension {
        "bib" => vec![("BIBINPUTS", kpathsea_env("BIBINPUTS", doc_dir))],
        "bst" => vec![("BSTINPUTS", kpathsea_env("BSTINPUTS", doc_dir))],
        "ist" => vec![
            (
                "TEXINDEXSTYLE",
                kpathsea_env_with_fallback_vars("TEXINDEXSTYLE", doc_dir, &["INDEXSTYLE"]),
            ),
            ("INDEXSTYLE", kpathsea_env("INDEXSTYLE", doc_dir)),
        ],
        _ => vec![("TEXINPUTS", kpathsea_env("TEXINPUTS", doc_dir))],
    }
}

fn engine_program(engine: Engine) -> &'static str {
    match engine {
        Engine::PdfLatex => "pdflatex",
        Engine::XeLatex => "xelatex",
        Engine::LuaLatex => "lualatex",
        Engine::Tectonic => "tectonic",
        Engine::TexpilotPdftex => "pdftex-rust",
        Engine::TexpilotPdftexCertified => "pdflatex",
    }
}

fn tex_engine_command(engine: Engine) -> Command {
    Command::new(engine_program_path(engine))
}

fn engine_program_path(engine: Engine) -> PathBuf {
    if engine != Engine::TexpilotPdftex {
        return PathBuf::from(engine_program(engine));
    }

    if let Ok(current_exe) = std::env::current_exe()
        && let Some(bin_dir) = current_exe.parent()
    {
        let binary_name = if cfg!(windows) {
            "pdftex-rust.exe"
        } else {
            "pdftex-rust"
        };

        for dir in bin_dir.ancestors() {
            for candidate in [
                dir.join(binary_name),
                dir.join("debug").join(binary_name),
                dir.join("release").join(binary_name),
            ] {
                if candidate.exists() {
                    return candidate;
                }
            }
        }
    }

    PathBuf::from(engine_program(engine))
}

fn uses_pdftex_graphics_pipeline(engine: Engine) -> bool {
    matches!(
        engine,
        Engine::PdfLatex | Engine::TexpilotPdftex | Engine::TexpilotPdftexCertified
    )
}

fn latexmk_command(
    doc_dir: &Path,
    file_name: &OsString,
    job_name: &str,
    out_dir: &Path,
    options: &BuildOptions,
) -> Command {
    let mut command = Command::new("latexmk");
    command.current_dir(doc_dir);
    match options.engine {
        Engine::PdfLatex => {
            command.arg("-pdf");
        }
        Engine::XeLatex => {
            command.arg("-xelatex");
        }
        Engine::LuaLatex => {
            command.arg("-lualatex");
        }
        Engine::Tectonic => unreachable!("tectonic uses its own command path"),
        Engine::TexpilotPdftex | Engine::TexpilotPdftexCertified => {
            unreachable!("texpilot-pdftex does not use latexmk")
        }
    }
    command
        .arg("-interaction=nonstopmode")
        .arg("-halt-on-error")
        .arg("-file-line-error")
        .arg("-recorder")
        .arg(format!("-jobname={job_name}"))
        .arg(format!("-outdir={}", out_dir.display()));

    if options.quiet {
        command.arg("-quiet");
    }
    if options.synctex {
        command.arg("-synctex=1");
    }
    if options.shell_escape {
        command.arg("-shell-escape");
    }
    if options.fast {
        command.arg(format!("-usepretex={}", fast_preview_pretex(doc_dir)));
    }
    match options.bib_mode {
        BibMode::Auto => {}
        BibMode::BibTex => {
            command.arg("-bibtex");
        }
        BibMode::Biber => {
            command
                .arg("-e")
                .arg(r#"$bibtex = 'biber %O %B'; $bibtex_use = 2;"#);
        }
        BibMode::None => {
            command.arg("-bibtex-");
        }
    }
    command.arg(file_name);
    command
}

fn tectonic_command(
    doc_dir: &Path,
    file_name: &OsString,
    out_dir: &Path,
    options: &BuildOptions,
) -> Command {
    let mut command = Command::new("tectonic");
    command.current_dir(doc_dir);
    command
        .arg("--outdir")
        .arg(out_dir)
        .arg("--keep-intermediates")
        .arg("--keep-logs");
    if options.synctex {
        command.arg("--synctex");
    }
    if options.shell_escape {
        command.arg("--shell-escape");
    }
    command.arg(file_name);
    command
}

fn build_job_name(options: &BuildOptions, main: &Path) -> Result<String> {
    if let Some(job_name) = &options.job_name {
        validate_job_name(job_name)?;
        return Ok(job_name.clone());
    }
    main.file_stem()
        .context("root TeX file has no stem")
        .map(|stem| stem.to_string_lossy().to_string())
}

fn validate_job_name(job_name: &str) -> Result<()> {
    if job_name.is_empty() {
        bail!("--job-name must not be empty");
    }
    let mut components = Path::new(job_name).components();
    let valid = matches!(components.next(), Some(std::path::Component::Normal(_)))
        && components.next().is_none();
    if !valid {
        bail!("--job-name must be a single filename component, not a path");
    }
    Ok(())
}

fn job_output_path(out_dir: &Path, job_name: &str, extension: &str) -> PathBuf {
    out_dir.join(format!("{job_name}.{extension}"))
}

fn absolute_from_cwd(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn display_command(command: &Command) -> String {
    let mut out = String::new();
    let _ = write!(&mut out, "{}", command.get_program().to_string_lossy());
    for arg in command.get_args() {
        let value = arg.to_string_lossy();
        if value.contains(' ')
            || value.contains('\\')
            || value.contains('"')
            || value.contains('\'')
        {
            let escaped = value.replace('\'', r"'\''");
            let _ = write!(&mut out, " '{escaped}'");
        } else {
            let _ = write!(&mut out, " {value}");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEXINPUTS_TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn source_features_detect_graphics_and_multipass_signals_recursively() {
        let root = unique_temp_dir("texpilot-source-features");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        let section = root.join("section.tex");
        fs::write(&main, "\\documentclass{article}\n\\input{section}\n")
            .expect("failed to write main source");
        fs::write(
            &section,
            "\\begin{document}\n\\includegraphics{plot}\nSee~\\citep{knuth}.\n\\end{document}\n",
        )
        .expect("failed to write section source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(features.has_graphics, "{features:#?}");
        assert!(features.has_multipass_signal, "{features:#?}");
        assert_eq!(features.graphic_command_count, 1, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_features_detect_unbraced_input_recursively() {
        let root = unique_temp_dir("texpilot-source-unbraced-input");
        let section_dir = root.join("sections");
        fs::create_dir_all(&section_dir).expect("failed to create temp root");
        let main = root.join("main.tex");
        let section = section_dir.join("intro.tex");
        fs::write(&main, "\\documentclass{article}\n\\input sections/intro\n")
            .expect("failed to write main source");
        fs::write(
            &section,
            "\\begin{document}\n\\includegraphics{plot}\nSee~\\ref{fig:plot}.\n\\end{document}\n",
        )
        .expect("failed to write section source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(features.has_graphics, "{features:#?}");
        assert!(features.has_multipass_signal, "{features:#?}");
        assert_eq!(features.graphic_command_count, 1, "{features:#?}");
        assert!(!features.has_backref_signal, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_features_detect_subfile_recursively() {
        let root = unique_temp_dir("texpilot-source-subfile");
        let section_dir = root.join("sections");
        fs::create_dir_all(&section_dir).expect("failed to create temp root");
        let main = root.join("main.tex");
        let section = section_dir.join("intro.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\subfile{sections/intro}\n",
        )
        .expect("failed to write main source");
        fs::write(
            &section,
            "\\begin{document}\n\\includegraphics{plot}\nSee~\\cite{x}.\n\\end{document}\n",
        )
        .expect("failed to write section source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(features.has_graphics, "{features:#?}");
        assert!(features.has_multipass_signal, "{features:#?}");
        assert_eq!(features.graphic_command_count, 1, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_features_respect_includeonly() {
        let root = unique_temp_dir("texpilot-source-includeonly");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        let active = root.join("active.tex");
        let excluded = root.join("excluded.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\
             \\includeonly{active}\n\
             \\include{active}\n\
             \\include{excluded}\n",
        )
        .expect("failed to write main source");
        fs::write(&active, "\\begin{document}Active chapter.\\end{document}\n")
            .expect("failed to write active source");
        fs::write(
            &excluded,
            "\\begin{document}\n\\includegraphics{plot}\n\\cite{x}\n\\end{document}\n",
        )
        .expect("failed to write excluded source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(
            !features.has_graphics,
            "excluded include should not trigger graphics: {features:#?}"
        );
        assert_eq!(features.graphic_command_count, 0, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn includeonly_filter_strips_tex_extension_case_insensitively() {
        let source = "\\includeonly{chapter-a.TEX, sections/chapter-b.TeX}\n\
             \\include{chapter-a}\n\
             \\include{sections/chapter-b}\n\
             \\include{chapter-c}\n";
        let includeonly = includeonly_filter(source).expect("expected includeonly filter");
        let dependencies = active_include_source_dependencies(source, Some(&includeonly));
        let payloads = dependencies
            .into_iter()
            .map(|dependency| dependency.payload)
            .collect::<Vec<_>>();

        assert_eq!(payloads, ["chapter-a", "sections/chapter-b"]);
    }

    #[test]
    fn source_features_follow_tex_file_exists_probes() {
        let root = unique_temp_dir("texpilot-source-if-file-exists-tex");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        let conditional = root.join("conditional.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\IfFileExists{conditional.tex}{Found}{Missing}\n",
        )
        .expect("failed to write main source");
        fs::write(
            &conditional,
            "\\begin{document}\n\\includegraphics{plot}\nSee~\\citep{knuth}.\n\\end{document}\n",
        )
        .expect("failed to write conditional source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(features.has_graphics, "{features:#?}");
        assert!(features.has_multipass_signal, "{features:#?}");
        assert_eq!(features.graphic_command_count, 1, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_features_follow_uppercase_tex_file_exists_probes() {
        let root = unique_temp_dir("texpilot-source-if-file-exists-uppercase-tex");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        let conditional = root.join("conditional.TEX");
        fs::write(
            &main,
            "\\documentclass{article}\n\\IfFileExists{conditional.TEX}{Found}{Missing}\n",
        )
        .expect("failed to write main source");
        fs::write(
            &conditional,
            "\\begin{document}\n\\includegraphics{plot}\nSee~\\citep{knuth}.\n\\end{document}\n",
        )
        .expect("failed to write conditional source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(features.has_graphics, "{features:#?}");
        assert!(features.has_multipass_signal, "{features:#?}");
        assert_eq!(features.graphic_command_count, 1, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_features_ignore_non_tex_file_exists_probes() {
        let root = unique_temp_dir("texpilot-source-if-file-exists-binary");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        let asset = root.join("asset.pdf");
        fs::write(
            &main,
            "\\documentclass{article}\n\\IfFileExists{asset.pdf}{Asset exists}{Missing}\n",
        )
        .expect("failed to write main source");
        fs::write(&asset, [0xff, 0xfe, 0x00, 0x25]).expect("failed to write binary asset");

        let features = source_features(&root, &main).expect("feature scan should ignore asset");
        assert!(!features.has_graphics, "{features:#?}");
        assert!(!features.has_multipass_signal, "{features:#?}");
        assert_eq!(features.graphic_command_count, 0, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_features_detect_svg_graphics() {
        let root = unique_temp_dir("texpilot-source-svg");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}\n\\includesvg{plot}\nSee~\\ref{fig:plot}.\n\\end{document}\n",
        )
        .expect("failed to write main source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(features.has_graphics, "{features:#?}");
        assert!(features.has_multipass_signal, "{features:#?}");
        assert_eq!(features.graphic_command_count, 1, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_features_detect_import_recursively() {
        let root = unique_temp_dir("texpilot-source-import");
        let section_dir = root.join("sections");
        fs::create_dir_all(&section_dir).expect("failed to create temp root");
        let main = root.join("main.tex");
        let section = section_dir.join("intro.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\import{sections/}{intro}\n",
        )
        .expect("failed to write main source");
        fs::write(
            &section,
            "\\begin{document}\n\\includegraphics{plot}\nSee~\\cite{x}.\n\\end{document}\n",
        )
        .expect("failed to write section source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(features.has_graphics, "{features:#?}");
        assert!(features.has_multipass_signal, "{features:#?}");
        assert_eq!(features.graphic_command_count, 1, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_features_detect_backref_draft_convergence_signal() {
        let root = unique_temp_dir("texpilot-source-backref");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\usepackage[pagebackref=true]{hyperref}\n\\begin{document}\n\\includegraphics{plot}\n\\cite{x}\n\\end{document}\n",
        )
        .expect("failed to write main source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(features.has_graphics, "{features:#?}");
        assert!(features.has_multipass_signal, "{features:#?}");
        assert!(features.has_backref_signal, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn auto_draft_prepass_stays_draft_for_graphic_multipass_documents() {
        let root = unique_temp_dir("texpilot-auto-small-graphics");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}\n\\includegraphics{plot}\nSee~\\cite{x}.\n\\end{document}\n",
        )
        .expect("failed to write main source");
        let options = test_build_options(&main, &out_dir, DraftPrepass::Auto);

        let strategy = draft_graphics_strategy(&options, &root, &main, false, None)
            .expect("draft strategy failed");

        assert_eq!(strategy, DraftGraphicsStrategy::UntilSettled);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn auto_draft_prepass_stays_draft_until_settled_for_image_heavy_documents() {
        let root = unique_temp_dir("texpilot-auto-heavy-graphics");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}\n\
             \\includegraphics{a}\n\\includegraphics{b}\n\\includegraphics{c}\n\
             See~\\cite{x}.\n\\end{document}\n",
        )
        .expect("failed to write main source");
        let options = test_build_options(&main, &out_dir, DraftPrepass::Auto);

        let features = source_features(&root, &main).expect("feature scan failed");
        assert_eq!(features.graphic_command_count, 3, "{features:#?}");
        let strategy = draft_graphics_strategy(&options, &root, &main, false, Some(&features))
            .expect("draft strategy failed");

        assert_eq!(strategy, DraftGraphicsStrategy::UntilSettled);
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn draft_prepass_uses_engine_no_pdf_output_modes() {
        assert_eq!(
            nonfinal_output_mode_arg(Engine::PdfLatex),
            Some("-draftmode")
        );
        assert_eq!(
            nonfinal_output_mode_arg(Engine::LuaLatex),
            Some("-draftmode")
        );
        assert_eq!(nonfinal_output_mode_arg(Engine::XeLatex), Some("-no-pdf"));
        assert_eq!(nonfinal_output_mode_arg(Engine::Tectonic), None);
    }

    #[test]
    fn draft_graphics_prepass_skips_recorder_when_no_generated_tools_need_it() {
        let options =
            test_build_options(Path::new("main.tex"), Path::new("out"), DraftPrepass::Auto);
        let draft_mode = TexRunMode {
            draft_graphics: true,
            suppress_pdf_output: true,
            force_pgf_list_and_make: false,
        };
        let final_mode = TexRunMode {
            draft_graphics: false,
            suppress_pdf_output: false,
            force_pgf_list_and_make: false,
        };
        let pgf_mode = TexRunMode {
            force_pgf_list_and_make: true,
            ..draft_mode
        };

        assert!(!tex_run_records_files(&options, draft_mode));
        assert!(tex_run_records_files(&options, final_mode));
        assert!(tex_run_records_files(&options, pgf_mode));

        let draft_command = tex_direct_base_command(
            Path::new("."),
            "main",
            Path::new("out"),
            &options,
            draft_mode,
        );
        let draft_args = draft_command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        assert!(!draft_args.iter().any(|arg| arg == "-recorder"));

        let final_command = tex_direct_base_command(
            Path::new("."),
            "main",
            Path::new("out"),
            &options,
            final_mode,
        );
        let final_args = final_command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        assert!(final_args.iter().any(|arg| arg == "-recorder"));
    }

    #[test]
    fn opportunistic_preamble_format_is_limited_to_texpilot_pdftex_final_runs() {
        let final_mode = TexRunMode {
            draft_graphics: false,
            suppress_pdf_output: false,
            force_pgf_list_and_make: false,
        };
        let draft_mode = TexRunMode {
            draft_graphics: true,
            suppress_pdf_output: true,
            force_pgf_list_and_make: false,
        };
        let mut options =
            test_build_options(Path::new("main.tex"), Path::new("out"), DraftPrepass::Auto);
        options.engine = Engine::TexpilotPdftex;

        assert_eq!(
            opportunistic_preamble_format_kind_for_run(&options, final_mode),
            Some(PreambleFormatKind::Final)
        );
        assert_eq!(
            opportunistic_preamble_format_kind_for_run(&options, draft_mode),
            None
        );

        options.engine = Engine::PdfLatex;
        assert_eq!(
            opportunistic_preamble_format_kind_for_run(&options, final_mode),
            None
        );

        options.engine = Engine::TexpilotPdftex;
        options.shell_escape = true;
        assert_eq!(
            opportunistic_preamble_format_kind_for_run(&options, final_mode),
            None
        );
    }

    #[test]
    fn automatic_preamble_format_allows_plain_metadata_preamble() {
        let source = "\\documentclass{article}\n\
             \\title{A title}\n\
             \\author{An author}\n\
             \\begin{document}\n\
             \\maketitle\n\
             Text.\n\
             \\end{document}\n";

        assert!(!preamble_contains_input_like_dependency(source));
    }

    #[test]
    fn automatic_preamble_format_rejects_predocument_inputs() {
        let source = "\\documentclass{article}\n\
             \\newcommand{\\paperabstract}{\\input{abstract}}\n\
             \\input{macros}\n\
             \\begin{document}\n\
             Text.\n\
             \\end{document}\n";

        assert!(preamble_contains_input_like_dependency(source));
    }

    #[test]
    fn preamble_format_cache_dir_is_stable_for_document() {
        let root = unique_temp_dir("texpilot-preamble-cache-dir");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}x\\end{document}\n",
        )
        .expect("failed to write source");
        let cache_root = root.join("cache");
        let _guard = EnvVarGuard::set("TEXPILOT_FORMAT_CACHE", cache_root.display().to_string());

        let first = preamble_format_cache_dir(&root, &main);
        let second = preamble_format_cache_dir(&root, &main);

        assert_eq!(first, second);
        assert!(first.starts_with(&cache_root), "{first:?}");
        assert_ne!(first, cache_root);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn relative_format_cache_env_is_anchored_to_cwd() {
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let cwd = std::env::current_dir().expect("failed to read cwd");
        let root = unique_temp_dir("texpilot-relative-format-cache");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}x\\end{document}\n",
        )
        .expect("failed to write source");
        let relative_cache = PathBuf::from("target").join("texpilot-relative-format-cache-test");
        let _guard = EnvVarGuard::set(
            "TEXPILOT_FORMAT_CACHE",
            relative_cache.display().to_string(),
        );

        let dir = preamble_format_cache_dir(&root, &main);

        assert!(dir.starts_with(cwd.join(&relative_cache)), "{dir:?}");
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn relative_aux_cache_env_is_anchored_to_cwd() {
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let cwd = std::env::current_dir().expect("failed to read cwd");
        let root = unique_temp_dir("texpilot-relative-aux-cache");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}x\\end{document}\n",
        )
        .expect("failed to write source");
        let relative_cache = PathBuf::from("target").join("texpilot-relative-aux-cache-test");
        let _guard = EnvVarGuard::set("TEXPILOT_AUX_CACHE", relative_cache.display().to_string());

        let dir = settled_aux_cache_dir(&root, &main, "mode");

        assert!(dir.starts_with(cwd.join(&relative_cache)), "{dir:?}");
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn global_bibtex_cache_restores_fresh_bbl() {
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let root = unique_temp_dir("texpilot-global-bibtex-cache");
        fs::create_dir_all(&root).expect("failed to create temp root");
        fs::write(
            root.join("refs.bib"),
            "@book{x, author={A. Author}, title={Cached}, year={2026}}\n",
        )
        .expect("failed to write bibliography");
        let cache_root = root.join("cache");
        let _guard = EnvVarGuard::set("TEXPILOT_BIBTEX_CACHE", cache_root.display().to_string());
        let aux_source = "\\relax\n\\citation{x}\n\\bibstyle{plain}\n\\bibdata{refs}\n";

        let source_out = root.join("source-out");
        fs::create_dir_all(&source_out).expect("failed to create source output");
        let source_aux = source_out.join("main.aux");
        fs::write(&source_aux, aux_source).expect("failed to write source aux");
        let source_job = bibtex_job(&source_out, &source_aux, None);
        fs::write(
            &source_job.bbl_path,
            "\\begin{thebibliography}{1}\n\\bibitem{x} X.\n",
        )
        .expect("failed to write source bbl");
        let signature = bibtex_aux_signature_from_source(aux_source, &source_job);
        save_global_bibtex_cache(&root, &source_out, &source_job, &signature, aux_source)
            .expect("failed to save global bibliography cache");

        let restored_out = root.join("restored-out");
        fs::create_dir_all(&restored_out).expect("failed to create restored output");
        let restored_aux = restored_out.join("main.aux");
        fs::write(&restored_aux, aux_source).expect("failed to write restored aux");
        let restored_job = bibtex_job(&restored_out, &restored_aux, None);

        assert!(
            restore_global_bibtex_cache_if_fresh(
                &root,
                &restored_out,
                &restored_job,
                &signature,
                aux_source,
            )
            .expect("failed to restore global bibliography cache")
        );
        let restored = fs::read_to_string(&restored_job.bbl_path)
            .expect("failed to read restored bibliography");
        assert!(restored.contains("\\bibitem{x} X."), "{restored}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_features_do_not_treat_comments_as_commands() {
        let root = unique_temp_dir("texpilot-source-comments");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n% \\includegraphics{plot} \\ref{x}\nText.\n",
        )
        .expect("failed to write main source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(!features.has_graphics, "{features:#?}");
        assert!(!features.has_multipass_signal, "{features:#?}");
        assert_eq!(features.graphic_command_count, 0, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_features_do_not_treat_inline_literals_as_commands() {
        let root = unique_temp_dir("texpilot-source-inline-literals");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\
             \\begin{document}\n\
             \\verb|\\includegraphics{plot} \\cite{x} \\ref{y}|\n\
             \\lstinline!\\includesvg{plot} \\pagebackref!\n\
             \\mintinline{tex}|\\label{z}|\n\
             \\end{document}\n",
        )
        .expect("failed to write main source");

        let features = source_features(&root, &main).expect("feature scan failed");
        assert!(!features.has_graphics, "{features:#?}");
        assert!(!features.has_multipass_signal, "{features:#?}");
        assert!(!features.has_backref_signal, "{features:#?}");
        assert_eq!(features.graphic_command_count, 0, "{features:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn effective_root_bytes_stop_at_uncommented_end_document() {
        let source =
            b"before\n% \\end{document}\nstill effective\n\\end {document} ignored\n% trailing\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Root))
            .expect("effective root bytes should stay valid UTF-8");

        assert_eq!(effective, "before\nstill effective\n\\end {document}");
    }

    #[test]
    fn effective_input_bytes_stop_after_endinput_line() {
        let source = b"before\n% \\endinput\nstill effective\n\\endinput same-line kept\nignored\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Input))
            .expect("effective input bytes should stay valid UTF-8");

        assert_eq!(
            effective,
            "before\nstill effective\n\\endinput same-line kept\n"
        );
    }

    #[test]
    fn effective_tex_bytes_discard_physical_trailing_spaces() {
        let source = b"before   \ninside  text   \n\\end{document}   ignored\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Root))
            .expect("effective root bytes should stay valid UTF-8");

        assert_eq!(effective, "before\ninside  text\n\\end{document}");
    }

    #[test]
    fn effective_tex_bytes_ignore_column_zero_comment_text() {
        let source = b"before\n% comment text changes here   \n  % indented comment remains visible spacing\n\\end{document}\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Root))
            .expect("effective root bytes should stay valid UTF-8");

        assert_eq!(effective, "before\n  %\\end{document}");
    }

    #[test]
    fn effective_tex_bytes_ignore_inline_comment_text() {
        let source = b"before % comment text\nnext line\nliteral \\% percent\n\\end{document}\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Root))
            .expect("effective root bytes should stay valid UTF-8");

        assert_eq!(
            effective,
            "before %next line\nliteral \\% percent\n\\end{document}"
        );
    }

    #[test]
    fn effective_tex_bytes_canonicalize_spaces_before_inline_comments() {
        let compact = effective_tex_bytes(
            b"before % comment text\n\\end{document}\n",
            EffectiveTexMode::Root,
        );
        let padded = effective_tex_bytes(
            b"before    % changed comment text\n\\end{document}\n",
            EffectiveTexMode::Root,
        );
        let tight = effective_tex_bytes(
            b"before% changed comment text\n\\end{document}\n",
            EffectiveTexMode::Root,
        );

        assert_eq!(compact, padded);
        assert_ne!(compact, tight);
    }

    #[test]
    fn effective_tex_bytes_ignore_comments_and_boundaries_inside_inline_verb() {
        let source =
            b"before \\verb|% literal \\end{document}| text % comment\n\\end{document}\ntrailing\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Root))
            .expect("effective root bytes should stay valid UTF-8");

        assert_eq!(
            effective,
            "before \\verb|% literal \\end{document}| text %\\end{document}"
        );
    }

    #[test]
    fn effective_tex_bytes_ignore_comments_and_boundaries_inside_inline_literal_commands() {
        let source = b"before \\lstinline[language=TeX]|% literal \\end{document}| text % comment\nnext \\mintinline{tex}|% literal \\endinput| text % comment\n\\end{document}\ntrailing\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Root))
            .expect("effective root bytes should stay valid UTF-8");

        assert_eq!(
            effective,
            "before \\lstinline[language=TeX]|% literal \\end{document}| text %next \\mintinline{tex}|% literal \\endinput| text %\\end{document}"
        );
    }

    #[test]
    fn effective_tex_bytes_preserve_unparseable_inline_verb_conservatively() {
        let source =
            b"before \\verb|unterminated % visible percent text\n\\end{document}\ntrailing\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Root))
            .expect("effective root bytes should stay valid UTF-8");

        assert_eq!(
            effective,
            "before \\verb|unterminated % visible percent text\n\\end{document}\ntrailing\n"
        );
    }

    #[test]
    fn effective_tex_bytes_preserve_unparseable_inline_literal_conservatively() {
        let source =
            b"before \\lstinline|unterminated % visible percent text\n\\end{document}\ntrailing\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Root))
            .expect("effective root bytes should stay valid UTF-8");

        assert_eq!(
            effective,
            "before \\lstinline|unterminated % visible percent text\n\\end{document}\ntrailing\n"
        );
    }

    #[test]
    fn effective_tex_bytes_preserve_percent_lines_when_catcode_is_explicit() {
        let source = b"before\n\\catcode37=12\n% visible percent text\n\\end{document}\ntrailing\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Root))
            .expect("effective root bytes should stay valid UTF-8");

        assert_eq!(
            effective,
            "before\n\\catcode37=12\n% visible percent text\n\\end{document}\ntrailing\n"
        );
    }

    #[test]
    fn effective_tex_bytes_preserve_percent_text_in_verbatim() {
        let source = b"before\n\\begin{verbatim}\n% visible percent text\n\\end{verbatim}\n\\end{document}\ntrailing\n";
        let effective = String::from_utf8(effective_tex_bytes(source, EffectiveTexMode::Root))
            .expect("effective root bytes should stay valid UTF-8");

        assert_eq!(
            effective,
            "before\n\\begin{verbatim}\n% visible percent text\n\\end{verbatim}\n\\end{document}\ntrailing\n"
        );
    }

    #[test]
    fn tex_input_payloads_support_braced_and_bare_local_files() {
        let payloads = tex_input_payloads(
            "\\input{braced}\n\
             \\input bare-file.tex\n\
             \\input { spaced }\n\
             \\input\n\
             {multi/line}\n\
             \\input\\macro\n\
             \\inputfoo{not-this-command}\n\
             % \\input commented\n",
        );

        assert_eq!(
            payloads,
            ["braced", "bare-file.tex", "spaced", "multi/line"]
        );
    }

    #[test]
    fn tex_source_payloads_ignore_inline_literal_commands() {
        let source = "\\verb|\\input{fake}|\n\
             \\lstinline!\\include{fake}!\n\
             \\mintinline{tex}|\\subfile{fake}|\n\
             \\input{real}\n\
             \\include{chapter}\n\
             \\subfile{sections/intro}\n";

        assert_eq!(tex_input_payloads(source), ["real"]);
        assert_eq!(tex_include_payloads(source), ["chapter"]);
        assert_eq!(tex_input_like_payloads(source), ["real", "sections/intro"]);
    }

    #[test]
    fn tex_command_payloads_support_multiline_payloads() {
        let source = "\\include\n  {sections/ch1}\n% \\include{ignored}\n";

        assert_eq!(tex_command_payloads(source, "include"), ["sections/ch1"]);
    }

    #[test]
    fn tex_source_payloads_support_subfiles_and_file_exists_conditionals() {
        let source = "\\include{chapters/a}\n\
             \\subfileinclude{chapters/b}\n\
             \\includefrom{parts/}{ch1}\n\
             \\subincludefrom{parts}{ch2}\n\
             \\subfile\n\
             {sections/intro}\n\
             \\InputIfFileExists{fallback}{\\input{fallback}}{}\n\
             \\IfFileExists{conditional}{Found}{Missing}\n\
             \\subfile{sections/intro}\n\
             \\includestandalone[mode=buildnew]{figures/standalone}\n\
             \\import{chapters/}{intro}\n\
             \\subimport{appendix}{extra.tex}\n\
             \\inputfrom{shared/}{defs}\n\
             \\subinputfrom{shared}{defs}\n";

        assert_eq!(
            tex_include_payloads(source),
            ["chapters/a", "chapters/b", "parts/ch1", "parts/ch2"]
        );
        assert_eq!(
            tex_input_like_payloads(source),
            [
                "fallback",
                "sections/intro",
                "figures/standalone",
                "conditional",
                "chapters/intro",
                "appendix/extra.tex",
                "shared/defs"
            ]
        );
    }

    #[test]
    fn tex_source_dependencies_track_import_graphic_paths() {
        let source = "\\input{plain}\n\
             \\import{sections/}{intro}\n\
             \\subimport{appendix}{extra.tex}\n\
             \\inputfrom{/absolute/}{ignored}\n";

        let dependencies = tex_input_like_source_dependencies(source);

        assert_eq!(dependencies[0].payload, "plain");
        assert_eq!(dependencies[0].local_graphic_path, None);
        assert_eq!(dependencies[1].payload, "sections/intro");
        assert_eq!(
            dependencies[1].local_graphic_path.as_deref(),
            Some(Path::new("sections/"))
        );
        assert_eq!(dependencies[2].payload, "appendix/extra.tex");
        assert_eq!(
            dependencies[2].local_graphic_path.as_deref(),
            Some(Path::new("appendix"))
        );
        assert_eq!(dependencies[3].payload, "/absolute/ignored");
        assert_eq!(dependencies[3].local_graphic_path, None);
    }

    #[test]
    fn source_bibliography_payloads_parse_bibtex_and_biblatex_commands() {
        let source = "\\bibliography{refs, more}\n\
             \\bibliographystyle{custom}\n\
             \\addbibresource[datatype=bibtex]{bib/main.bib}\n\
             \\addglobalbib{global}\n\
             \\addsectionbib[section=1]{section.bib}\n\
             % \\bibliography{commented}\n\
             \\bibliographyExtra{ignored}\n";

        assert_eq!(
            source_bibliography_payloads(source),
            ["bib/main.bib", "global", "more", "refs", "section.bib"]
        );
        assert_eq!(source_bibliography_style_payloads(source), ["custom"]);
    }

    #[test]
    fn ordered_source_bibtex_events_preserve_common_bibtex_order() {
        let source = "\\input{intro}\n\
             \\citep[JEPAs,][]{lecun2022, bardes2022}\n\
             \\bibliographystyle{plainnat}\n\
             \\bibliography{refs, more}\n\
             \\addbibresource{biblatex.bib}\n";

        let events = ordered_source_bibtex_events(source);

        assert_eq!(
            events,
            [
                OrderedSourceBibtexEvent::Input(tex_source_dependency("intro".to_string())),
                OrderedSourceBibtexEvent::Citation("lecun2022,bardes2022".to_string()),
                OrderedSourceBibtexEvent::BibStyle("plainnat".to_string()),
                OrderedSourceBibtexEvent::BibData("refs,more".to_string()),
                OrderedSourceBibtexEvent::Unsupported
            ]
        );
    }

    #[test]
    fn natbib_bibcite_seeds_parse_optional_bbl_labels() {
        let bbl = "\\begin{thebibliography}{2}\n\
            \\bibitem[LeCun(2022)]{lecun2022}\n\
            Yann LeCun.\n\
            \\bibitem[Bardes et~al.(2021)Bardes, Ponce, and LeCun]{bardes2022}\n\
            Adrien Bardes, Jean Ponce, and Yann LeCun.\n\
            \\end{thebibliography}\n";

        let seeds = natbib_bibcite_seeds_from_bbl(bbl);

        assert_eq!(
            seeds,
            [
                NatbibBibciteSeed {
                    key: "lecun2022".to_string(),
                    line: "\\bibcite{lecun2022}{{1}{2022}{{LeCun}}{{}}}".to_string()
                },
                NatbibBibciteSeed {
                    key: "bardes2022".to_string(),
                    line: "\\bibcite{bardes2022}{{2}{2021}{{Bardes et~al.}}{{Bardes, Ponce, and LeCun}}}".to_string()
                }
            ]
        );
    }

    #[test]
    fn source_package_and_class_payloads_parse_common_declarations() {
        let source = "\\documentclass[11pt]{customclass}\n\
             \\usepackage{alpha, beta}\n\
             \\RequirePackage[options]{gamma}\n\
             \\RequirePackageWithOptions{delta}\n\
             \\LoadClass[twocolumn]{baseclass}\n\
             \\LoadClassWithOptions{fallbackclass}\n\
             % \\usepackage{commented}\n\
             \\usepackageExtra{ignored}\n";

        assert_eq!(
            source_package_payloads(source),
            ["alpha", "beta", "delta", "gamma"]
        );
        assert_eq!(
            source_class_payloads(source),
            ["baseclass", "customclass", "fallbackclass"]
        );
    }

    #[test]
    fn source_pdfpages_payloads_parse_includepdf_and_merge_inputs() {
        let source = "\\includepdf[pages=-]{supplement}\n\
             \\includepdfmerge[nup=2x1]{first.pdf,1-2, second.pdf,3, third,4}\n\
             % \\includepdf{commented}\n\
             \\includepdfExtra{ignored}\n";

        assert_eq!(
            source_pdfpages_payloads(source),
            ["first.pdf", "second.pdf", "supplement", "third"]
        );
    }

    #[test]
    fn source_file_payloads_parse_listing_and_minted_inputs() {
        let source = "\\inputminted[linenos]{python}{snippets/example.py}\n\
             \\lstinputlisting[language=json]{snippets/data.json}\n\
             \\verbatiminput{logs/output.txt}\n\
             \\VerbatimInput[fontsize=\\small]{logs/raw.log}\n\
             \\includemedia[width=2cm]{Poster}{media/demo.mp4}\n\
             \\attachfile[description={Data}]{artifacts/data.csv}\n\
             \\textattachfile[color=0 0 1]{artifacts/report.json}{report}\n\
             \\notextattachfile{artifacts/raw.bin}\n\
             \\DTLloaddb[noheader]{measurements}{tables/measurements.csv}\n\
             \\DTLloadrawdb{rawdata}{tables/raw.tsv}\n\
             \\csvreader[head to column names]{tables/rows.csv}{}{\\name}\n\
             \\csvautobooktabular[separator=semicolon]{tables/book.csv}\n\
             \\externaldocument[prefix-][nocite]{refs/supplement}\n\
             \\externalcitedocument{refs/cited}\n\
             \\zexternaldocument[z-]{refs/zref}\n\
             \\zexternaldocument*[z-]{refs/zref-star}\n\
             \\pgfplotstableread[col sep=comma]{tables/curve.csv}\\curve\n\
             \\addplot+[blue] table[x=x,y=y] {tables/points.dat};\n\
             \\addplot3 table {tables/surface.tsv};\n\
             % \\inputminted{python}{commented.py}\n\
             % \\DTLloaddb{ignored}{tables/commented.csv}\n\
             % \\externaldocument{refs/commented}\n\
             \\csvreaderExtra{tables/ignored.csv}{}{}\n\
             \\zexternaldocumentExtra{refs/ignored}\n\
             % \\addplot table {tables/commented.dat};\n\
             \\inputmintedExtra{python}{ignored.py}\n";

        assert_eq!(
            source_file_payloads(source),
            [
                "artifacts/data.csv",
                "artifacts/raw.bin",
                "artifacts/report.json",
                "logs/output.txt",
                "logs/raw.log",
                "media/demo.mp4",
                "refs/cited.aux",
                "refs/supplement.aux",
                "refs/zref-star.aux",
                "refs/zref.aux",
                "snippets/data.json",
                "snippets/example.py",
                "tables/book.csv",
                "tables/curve.csv",
                "tables/measurements.csv",
                "tables/points.dat",
                "tables/raw.tsv",
                "tables/rows.csv",
                "tables/surface.tsv"
            ]
        );
    }

    #[test]
    fn source_aux_input_payloads_parse_nested_aux_inputs() {
        let source = "\\relax\n\
             \\@input{sections/chapter.aux}\n\
               \\@input{appendix/a.aux}\n\
             \\citation{x}\n";

        assert_eq!(
            source_aux_input_payloads(source),
            ["sections/chapter.aux", "appendix/a.aux"]
        );
    }

    #[test]
    fn graphics_payloads_support_multiline_options_and_braces() {
        let source = "\\includegraphics*[\n  width=1cm,\n  trim={0 0 1 1}\n]{figures/plot}\n";

        assert_eq!(includegraphics_payloads(source), ["figures/plot"]);
    }

    #[test]
    fn animategraphics_refs_parse_frame_sequences() {
        let source = "\\animategraphics[controls]{12}{frames/frame-}{0}{2}\n\
             \\animategraphics{24}{frames/padded-}{001}{003}\n\
             % \\animategraphics{12}{frames/commented-}{0}{2}\n\
             \\animategraphicsExtra{12}{frames/ignored-}{0}{2}\n";

        let refs = animategraphics_refs(source);

        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0].prefix, "frames/frame-");
        assert_eq!(refs[0].first, "0");
        assert_eq!(refs[0].last, "2");
        assert_eq!(refs[1].prefix, "frames/padded-");
        assert_eq!(refs[1].first, "001");
        assert_eq!(refs[1].last, "003");
        assert_eq!(
            animategraphics_frame_payloads(&refs[0]),
            ["frames/frame-0", "frames/frame-1", "frames/frame-2"]
        );
        assert_eq!(
            animategraphics_frame_payloads(&refs[1]),
            [
                "frames/padded-001",
                "frames/padded-002",
                "frames/padded-003"
            ]
        );
    }

    #[test]
    fn graphics_payloads_ignore_inline_literal_commands() {
        let source = "\\verb|\\includegraphics{fake}|\n\
             \\lstinline!\\includesvg{fake}!\n\
             \\mintinline{tex}|\\graphicspath{{fake/}}|\n\
             \\includegraphics{figures/plot}\n\
             \\includesvg{figures/icon}\n\
             \\graphicspath{{figures/}}\n";

        assert_eq!(includegraphics_payloads(source), ["figures/plot"]);
        assert_eq!(
            includesvg_refs(source)
                .into_iter()
                .map(|svg| svg.payload)
                .collect::<Vec<_>>(),
            ["figures/icon"]
        );
        assert_eq!(graphicspath_entries(source), ["figures/"]);
    }

    #[test]
    fn source_eps_conversion_jobs_respect_includeonly() {
        let root = unique_temp_dir("texpilot-eps-includeonly");
        let out_dir = root.join("out");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        let active = root.join("active.tex");
        let excluded = root.join("excluded.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\
             \\includeonly{active}\n\
             \\include{active}\n\
             \\include{excluded}\n",
        )
        .expect("failed to write main source");
        fs::write(&active, "\\begin{document}Active chapter.\\end{document}\n")
            .expect("failed to write active source");
        fs::write(
            &excluded,
            "\\begin{document}\n\\includegraphics{excluded.eps}\n\\end{document}\n",
        )
        .expect("failed to write excluded source");
        fs::write(root.join("excluded.eps"), "%!PS-Adobe-3.0 EPSF-3.0\n")
            .expect("failed to write excluded EPS");

        let jobs =
            eps_conversion_jobs_from_source(&root, &out_dir, &main).expect("EPS scan failed");
        assert!(jobs.is_empty(), "{jobs:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn includesvg_refs_support_multiline_options() {
        let source = "\\includesvg[\n  width=1cm,\n  inkscapearea=page\n]{figures/plot}\n\
             \\includesvgfoo{ignored}\n\
             % \\includesvg{commented}\n";

        assert_eq!(
            includesvg_refs(source),
            [IncludeSvgRef {
                command_start: 0,
                options: "width=1cm,\n  inkscapearea=page".to_string(),
                payload: "figures/plot".to_string(),
            }]
        );
    }

    #[test]
    fn svg_setup_refs_track_ordered_package_settings() {
        let source = "\\setsvg{inkscapearea=page}\n\
             \\includesvg{before}\n\
             \\svgsetup{inkscapelatex=false}\n\
             \\includesvg{after}\n";

        let setup_refs = svg_setup_refs(source);
        let includes = includesvg_refs(source);

        assert_eq!(setup_refs.len(), 2);
        assert_eq!(setup_refs[0].options, "inkscapearea=page");
        assert_eq!(setup_refs[1].options, "inkscapelatex=false");

        let first =
            svg_settings_for_include(&default_svg_include_settings(), &setup_refs, &includes[0])
                .expect("first include should be supported");
        let second =
            svg_settings_for_include(&default_svg_include_settings(), &setup_refs, &includes[1])
                .expect("second include should be supported");

        assert_eq!(first.area, SvgExportArea::Page);
        assert!(first.export_latex);
        assert_eq!(second.area, SvgExportArea::Page);
        assert!(!second.export_latex);
    }

    #[test]
    fn graphicspath_entries_support_multiline_payloads() {
        let source = "\\graphicspath{\n  {figures/}\n  {plots/}\n}\n";

        assert_eq!(graphicspath_entries(source), ["figures/", "plots/"]);
    }

    #[test]
    fn declared_graphics_extensions_parse_latest_safe_order() {
        let source = "\\DeclareGraphicsExtensions{.pdf,.png}\n\
             \\verb|\\DeclareGraphicsExtensions{.fake}|\n\
             \\DeclareGraphicsExtensions{.eps,.PDF, .jpg, ../bad}\n";

        assert_eq!(
            declared_graphics_extensions(source),
            Some(vec![
                "eps".to_string(),
                "pdf".to_string(),
                "jpg".to_string()
            ])
        );
    }

    #[test]
    fn declared_graphics_extensions_can_make_eps_precede_existing_raster() {
        let root = unique_temp_dir("texpilot-declared-graphics-extensions");
        fs::create_dir_all(&root).expect("failed to create temp root");
        fs::write(root.join("fig.eps"), "%!PS\n").expect("failed to write EPS");
        fs::write(root.join("fig.png"), "not a real PNG\n").expect("failed to write PNG");

        let eps_first = vec!["eps".to_string(), "png".to_string()];
        let raster_first = vec!["png".to_string(), "eps".to_string()];

        let resolved = resolve_eps_graphic_input(&root, &[], "fig", Some(&eps_first))
            .expect("EPS-first resolution failed")
            .expect("EPS-first declaration should select EPS");
        assert_eq!(resolved, (root.join("fig.eps"), false));
        assert_eq!(
            resolve_eps_graphic_input(&root, &[], "fig", Some(&raster_first))
                .expect("raster-first resolution failed"),
            None
        );
        assert_eq!(
            resolve_eps_graphic_input(&root, &[], "fig", None).expect("default resolution failed"),
            None
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn svgpath_entries_support_nested_and_single_paths() {
        let source = "\\svgpath{\n  {figures/}\n  {plots/}\n}\n\\svgpath{icons/}\n";

        assert_eq!(svgpath_entries(source), ["figures/", "plots/", "icons/"]);
    }

    #[test]
    fn svg_include_settings_parse_common_inkscape_options() {
        let settings = svg_include_settings("width=1cm, inkscapelatex=false, inkscapearea=page")
            .expect("settings should be supported");

        assert_eq!(
            settings,
            SvgIncludeSettings {
                inkscape_enabled: true,
                inkscape_executable: "inkscape".to_string(),
                export_latex: false,
                format: "pdf".to_string(),
                source_extension: "svg".to_string(),
                area: SvgExportArea::Page,
                inkscape_name: None,
                dpi: None,
            }
        );
        assert_eq!(
            svg_include_settings("latex=false,inkscapearea=nocrop")
                .expect("package aliases should be supported"),
            settings
        );
        assert_eq!(
            svg_include_settings("tex=false,inkscapearea=crop")
                .expect("package aliases should be supported")
                .area,
            SvgExportArea::Drawing
        );
        assert!(svg_include_settings("inkscapeformat=png").is_some());
        assert_eq!(
            svg_include_settings("ext=svgz")
                .expect("safe custom SVG extension should be supported")
                .source_extension,
            "svgz"
        );
        assert!(svg_include_settings("svgextension=../svgz").is_none());
        assert_eq!(
            svg_include_settings("inkscapedpi=300")
                .expect("numeric DPI should be supported")
                .dpi
                .as_deref(),
            Some("300")
        );
        assert_eq!(
            svg_include_settings("inkscapedensity=144dpi")
                .expect("DPI suffix should be supported")
                .dpi
                .as_deref(),
            Some("144")
        );
        assert!(svg_include_settings("inkscapedpi=fast").is_none());
        assert!(svg_include_settings("inkscapepath=svgpath").is_none());
        assert_eq!(
            svg_include_settings("inkscapename=custom")
                .expect("safe custom name should be supported")
                .inkscape_name
                .as_deref(),
            Some("custom")
        );
        assert!(svg_include_settings("inkscapename=../escape").is_none());
        assert_eq!(
            svg_include_settings("inkscapeexe=inkscape-custom.1")
                .expect("safe custom executable should be supported")
                .inkscape_executable,
            "inkscape-custom.1"
        );
        assert!(svg_include_settings("inkscapeexe=../inkscape").is_none());
        assert!(svg_include_settings("inkscapeexe=inkscape --shell").is_none());
    }

    #[test]
    fn gnuplottex_output_path_parses_generated_script() {
        let script = Path::new("/tmp/texpilot/out/main-gnuplottex-fig1.gnuplot");
        let source = "set terminal latex\nset output './main-gnuplottex-fig1.tex'\nplot sin(x)\n";

        assert_eq!(
            gnuplottex_output_path(script, source).as_deref(),
            Some(Path::new("/tmp/texpilot/out/./main-gnuplottex-fig1.tex"))
        );
        assert_eq!(
            quoted_or_bare_shell_value("\"figure.pdf\" trailing"),
            Some("figure.pdf")
        );
    }

    #[test]
    fn gnuplottex_input_paths_track_existing_local_plot_data() {
        let root = unique_temp_dir("texpilot-gnuplottex-inputs");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        let data = root.join("points.dat");
        let macros = out_dir.join("macros.gp");
        fs::write(&data, "0 0\n1 1\n").expect("failed to write data");
        fs::write(&macros, "set grid\n").expect("failed to write macro script");
        let script = out_dir.join("main-gnuplottex-fig1.gnuplot");
        let source = "set output './main-gnuplottex-fig1.tex'\n\
                      load 'macros.gp'\n\
                      plot '../points.dat' using 1:2 title 'not-a-file'\n\
                      plot '< sort points.dat'\n\
                      # plot '../ignored.dat'\n";

        let paths = gnuplottex_input_paths(&script, source);

        assert_eq!(paths.len(), 2, "{paths:#?}");
        assert!(
            paths.contains(&root.join("out").join("macros.gp")),
            "{paths:#?}"
        );
        assert!(
            paths.contains(&root.join("out").join("../points.dat")),
            "{paths:#?}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn pythontex_pygments_setting_controls_required_outputs() {
        assert!(pythontex_pygments_enabled(
            "outputdir=pythontex-files-main\n"
        ));
        assert!(pythontex_pygments_enabled("pygments=true\n"));
        assert!(!pythontex_pygments_enabled("pygments=false\n"));
    }

    #[test]
    fn pythontex_dependency_paths_resolve_against_workingdir() {
        let root = unique_temp_dir("texpilot-pythontex-dependencies");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        let data = root.join("data.txt");
        fs::write(&data, "sample\n").expect("failed to write dependency");

        let working_dir = pythontex_working_dir(&out_dir, "..");
        let resolved = pythontex_local_dependency_path(&working_dir, "data.txt")
            .expect("dependency should resolve");

        assert_eq!(
            resolved
                .canonicalize()
                .expect("failed to canonicalize resolved dependency"),
            data.canonicalize()
                .expect("failed to canonicalize expected dependency")
        );
        assert!(pythontex_local_dependency_path(&working_dir, "missing.txt").is_none());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn pgf_external_make_parallel_args_scale_with_targets() {
        assert!(pgf_external_make_parallel_args(0).is_empty());
        assert!(pgf_external_make_parallel_args(1).is_empty());

        let args = pgf_external_make_parallel_args(4);
        if thread::available_parallelism()
            .map(usize::from)
            .unwrap_or(1)
            > 1
        {
            assert_eq!(args.first().and_then(|arg| arg.to_str()), Some("-j"));
            let jobs = args
                .get(1)
                .and_then(|arg| arg.to_str())
                .and_then(|value| value.parse::<usize>().ok())
                .expect("parallel job count should be present");
            assert!((2..=4).contains(&jobs), "{args:#?}");
        } else {
            assert!(args.is_empty(), "{args:#?}");
        }
    }

    #[test]
    fn asymptote_input_paths_track_existing_local_imports() {
        let root = unique_temp_dir("texpilot-asymptote-inputs");
        fs::create_dir_all(&root).expect("failed to create temp root");
        fs::write(root.join("style.asy"), "pen accent = red;\n")
            .expect("failed to write style input");
        fs::write(root.join("shared.asy"), "real scale = 1;\n")
            .expect("failed to write shared input");
        fs::write(root.join("quoted.asy"), "pair p = (0,0);\n")
            .expect("failed to write quoted input");
        let input = root.join("main-1.asy");
        let source = "import graph;\n\
                      import style;\n\
                      include \"quoted.asy\";\n\
                      access shared;\n\
                      from style import accent;\n\
                      // import ignored;\n\
                      /* include blocked; */\n";

        let paths = asymptote_input_paths(&input, source);

        assert_eq!(paths.len(), 3, "{paths:#?}");
        assert!(paths.contains(&root.join("style.asy")), "{paths:#?}");
        assert!(paths.contains(&root.join("shared.asy")), "{paths:#?}");
        assert!(paths.contains(&root.join("quoted.asy")), "{paths:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn asymptote_state_tracks_sidecar_input_freshness() {
        let root = unique_temp_dir("texpilot-asymptote-state-inputs");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let input_path = root.join("main-1.asy");
        let macro_path = root.join("style.asy");
        let output_path = root.join("main-1.pdf");
        let state_path = root.join("main-1.asystate.toml");
        fs::write(&input_path, "import style;\ndraw((0,0)--(1,1));\n")
            .expect("failed to write Asymptote input");
        fs::write(&macro_path, "pen accent = red;\n").expect("failed to write macro input");
        fs::write(&output_path, "fake asymptote output\n")
            .expect("failed to write Asymptote output");
        let job = AsymptoteJob {
            input_path,
            output_path: output_path.clone(),
            input_paths: vec![macro_path.clone()],
            state_path: state_path.clone(),
        };

        write_asymptote_state(&job, "signature").expect("failed to write Asymptote state");
        let source = fs::read_to_string(&state_path).expect("failed to read external tool state");
        let state: ExternalToolState =
            toml::from_str(&source).expect("failed to parse external tool state");
        assert!(
            state
                .inputs
                .iter()
                .any(|input| input.path.ends_with("style.asy")),
            "{state:#?}"
        );
        assert!(
            external_tool_cache_is_fresh(&state_path, "signature", &output_path)
                .expect("freshness check failed")
        );

        fs::write(&macro_path, "pen accent = blue;\n").expect("failed to update macro input");
        assert!(
            !external_tool_cache_is_fresh(&state_path, "signature", &output_path)
                .expect("freshness check failed")
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn metapost_input_paths_track_existing_local_inputs() {
        let root = unique_temp_dir("texpilot-metapost-inputs");
        fs::create_dir_all(&root).expect("failed to create temp root");
        fs::write(root.join("macros.mp"), "vardef marker = enddef;\n")
            .expect("failed to write macro input");
        fs::write(root.join("style.dat"), "style\n").expect("failed to write style input");
        let input = root.join("diagram.mp");
        let source = "input macros;\n\
                      input \"style.dat\";\n\
                      input missing;\n\
                      % input ignored;\n";

        let paths = metapost_input_paths(&input, source);

        assert_eq!(paths.len(), 2, "{paths:#?}");
        assert!(paths.contains(&root.join("macros.mp")), "{paths:#?}");
        assert!(paths.contains(&root.join("style.dat")), "{paths:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn metapost_state_tracks_sidecar_input_freshness() {
        let root = unique_temp_dir("texpilot-metapost-state-inputs");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let input_path = root.join("diagram.mp");
        let macro_path = root.join("macros.mp");
        let state_path = root.join("diagram.mpoststate.toml");
        fs::write(&input_path, "input macros;\nbeginfig(1); endfig;\n")
            .expect("failed to write MetaPost input");
        fs::write(&macro_path, "vardef marker = enddef;\n").expect("failed to write macro input");
        fs::write(root.join("diagram.1"), "fake mpost output\n")
            .expect("failed to write MetaPost output");
        let job = MetapostJob {
            input_path: input_path.clone(),
            output_paths: vec![root.join("diagram.1")],
            input_paths: vec![macro_path.clone()],
            state_path: state_path.clone(),
        };

        write_metapost_state(&job, "signature").expect("failed to write MetaPost state");
        let source = fs::read_to_string(&state_path).expect("failed to read external tool state");
        let state: ExternalToolState =
            toml::from_str(&source).expect("failed to parse external tool state");
        assert!(
            state
                .inputs
                .iter()
                .any(|input| input.path.ends_with("macros.mp")),
            "{state:#?}"
        );
        assert!(
            external_tool_cache_is_fresh_for_outputs(&state_path, "signature", &job.output_paths)
                .expect("freshness check failed")
        );

        fs::write(&macro_path, "vardef marker = draw origin; enddef;\n")
            .expect("failed to update macro input");
        assert!(
            !external_tool_cache_is_fresh_for_outputs(&state_path, "signature", &job.output_paths)
                .expect("freshness check failed")
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn kpathsea_env_preserves_existing_search_path() {
        let doc_dir = PathBuf::from("/tmp/texpilot-paper");
        let existing = OsStr::new("/tmp/texpilot-bib//:");
        let value = kpathsea_env_with_existing(&doc_dir, Some(existing));

        assert_eq!(
            value.to_string_lossy(),
            format!(
                "{}//{}{existing}",
                doc_dir.display(),
                KPATHSEA_PATH_SEPARATOR,
                existing = existing.to_string_lossy()
            )
        );

        let defaulted = kpathsea_env_with_existing(&doc_dir, None);
        assert_eq!(
            defaulted.to_string_lossy(),
            format!("{}//{}", doc_dir.display(), KPATHSEA_PATH_SEPARATOR)
        );
    }

    #[test]
    fn makeindex_log_command_parser_extracts_quoted_style_options() {
        let log = "runsystem(makeindex -s \"custom style.ist\" people.idx)...executed safely\n\
                   Package imakeidx Warning: `makeindex -s other.ist subjects.idx'.\n\
                   Package imakeidx Warning: `texindy -L english people.idx'.\n";
        let commands = makeindex_commands_from_log(log);

        assert_eq!(
            commands,
            [
                "makeindex -s \"custom style.ist\" people.idx",
                "makeindex -s other.ist subjects.idx",
                "texindy -L english people.idx"
            ]
        );
        assert_eq!(
            parse_makeindex_command(&commands[0]),
            [(
                "people.idx".to_string(),
                ParsedMakeIndexCommand {
                    program: IndexCommandProgram::MakeIndex,
                    style: Some("custom style.ist".to_string()),
                    options: Vec::new()
                }
            )]
        );
        assert_eq!(
            parse_makeindex_command("makeindex -q -l -sother.ist -o people.ind people.idx"),
            [(
                "people.idx".to_string(),
                ParsedMakeIndexCommand {
                    program: IndexCommandProgram::MakeIndex,
                    style: Some("other.ist".to_string()),
                    options: vec!["-q".to_string(), "-l".to_string()]
                }
            )]
        );
        assert_eq!(
            parse_makeindex_command("makeindex -p any -r people.idx"),
            [(
                "people.idx".to_string(),
                ParsedMakeIndexCommand {
                    program: IndexCommandProgram::MakeIndex,
                    style: None,
                    options: vec!["-p".to_string(), "any".to_string(), "-r".to_string()]
                }
            )]
        );
        assert_eq!(
            parse_makeindex_command("texindy -L english -M custom people.idx"),
            [(
                "people.idx".to_string(),
                ParsedMakeIndexCommand {
                    program: IndexCommandProgram::Texindy,
                    style: None,
                    options: vec![
                        "-L".to_string(),
                        "english".to_string(),
                        "-M".to_string(),
                        "custom".to_string()
                    ]
                }
            )]
        );
    }

    #[test]
    fn logreq_parser_extracts_bibtex8_command() {
        let root = unique_temp_dir("texpilot-logreq-bibtex8");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create temp root");
        fs::write(out_dir.join("main.aux"), "\\relax\n").expect("failed to write aux");
        fs::write(out_dir.join("main-blx.bib"), "% generated\n")
            .expect("failed to write generated bib");
        fs::write(root.join("refs.bib"), "@book{knuth,title={The TeXbook}}\n")
            .expect("failed to write editable bib");
        fs::write(
            out_dir.join("main.run.xml"),
            r#"<requests version="1.0">
  <external package="biblatex" priority="5" active="0">
    <generic>bibtex</generic>
    <cmdline>
      <binary>bibtex8</binary>
      <option>--wolfgang</option>
      <option>--min_crossrefs 2</option>
      <infile>main</infile>
    </cmdline>
    <input>
      <file>main.aux</file>
    </input>
    <requires type="dynamic">
      <file>main-blx.bib</file>
    </requires>
    <requires type="editable">
      <file>refs.bib</file>
    </requires>
  </external>
</requests>"#,
        )
        .expect("failed to write run.xml");

        let specs = bibtex_command_specs_from_logreq(&root, &out_dir, "main")
            .expect("failed to parse logreq");
        let key = canonical_or_original(&out_dir.join("main.aux"))
            .display()
            .to_string();
        let spec = specs.get(&key).expect("missing bibtex8 request");

        assert_eq!(spec.program, BibtexProgram::Bibtex8);
        assert_eq!(
            spec.options,
            ["--wolfgang", "--min_crossrefs", "2"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
        assert!(
            spec.request_inputs
                .iter()
                .any(|path| path == &out_dir.join("main-blx.bib")),
            "{spec:#?}"
        );
        assert!(
            spec.request_inputs
                .iter()
                .any(|path| path == &root.join("refs.bib")),
            "{spec:#?}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn cited_bibtex_bytes_ignore_uncited_entries() {
        let source = br#"@string{press = {Addison-Wesley}}

@book{cited,
  author = {Donald E. Knuth},
  title = {The TeXbook},
  publisher = press
}

@book{unused,
  author = {Unused Author},
  title = {Unused Book}
}
"#;
        let selected = cited_bibtex_bytes(source, &["cited".to_string()])
            .expect("ordinary cited entry should be selectable");
        let selected = String::from_utf8(selected).expect("selected bytes should be UTF-8");

        assert!(selected.contains("@string{press"));
        assert!(selected.contains("@book{cited"));
        assert!(!selected.contains("@book{unused"));
    }

    #[test]
    fn cited_bibtex_bytes_fall_back_for_crossref_entries() {
        let source = br#"@inproceedings{child,
  title = {Child},
  crossref = {parent}
}

@proceedings{parent,
  title = {Parent}
}
"#;

        assert!(cited_bibtex_bytes(source, &["child".to_string()]).is_none());
    }

    #[test]
    fn bibtex_state_writer_uses_supplied_aux_source() {
        let root = unique_temp_dir("texpilot-bibtex-source-state");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        fs::write(
            root.join("refs.bib"),
            "@book{knuth1984,title={The TeXbook}}\n",
        )
        .expect("failed to write bibliography");
        let aux_path = out_dir.join("main.aux");
        let job = bibtex_job(&out_dir, &aux_path, None);
        let aux_source = "\\relax\n\\citation{knuth1984}\n\\bibdata{refs}\n";
        let signature = bibtex_aux_signature_from_source(aux_source, &job);

        write_bibtex_state_from_source(&job, &signature, aux_source, &root, &out_dir)
            .expect("state writer should use supplied aux source");

        let source = fs::read_to_string(&job.state_path).expect("failed to read BibTeX state");
        let state: BibState = toml::from_str(&source).expect("failed to parse BibTeX state");
        let refs = root
            .join("refs.bib")
            .canonicalize()
            .expect("failed to canonicalize bibliography")
            .display()
            .to_string();
        assert_eq!(state.signature, signature);
        assert!(
            state.inputs.iter().any(|input| input.path == refs),
            "{state:#?}"
        );
        assert!(!aux_path.exists(), "test should not create an aux file");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn bibtex_signature_canonicalizes_duplicate_citation_noise() {
        let root = unique_temp_dir("texpilot-bibtex-canonical-signature");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        let aux_path = out_dir.join("main.aux");
        let job = bibtex_job(&out_dir, &aux_path, None);
        let synthetic = "\\relax\n\
             \\citation{alpha,beta}\n\
             \\bibstyle{plainnat}\n\
             \\bibdata{refs}\n";
        let actual = "\\relax\n\
             \\citation{alpha, beta}\n\
             \\citation{alpha}\n\
             \\bibstyle{plainnat}\n\
             \\bibdata{refs}\n";

        assert_eq!(
            bibtex_aux_signature_from_source(synthetic, &job),
            bibtex_aux_signature_from_source(actual, &job)
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn biber_citation_keys_parse_finite_bcf_keys() {
        let source = r#"<bcf:controlfile>
  <bcf:bibdata section="0">
    <bcf:datasource type="file" datatype="bibtex" glob="false">refs.bib</bcf:datasource>
  </bcf:bibdata>
  <bcf:section number="0">
    <bcf:citekey order="1">knuth1984</bcf:citekey>
    <bcf:citekey order="2">lamport1994</bcf:citekey>
  </bcf:section>
</bcf:controlfile>"#;

        assert_eq!(
            biber_citation_keys(source),
            Some(vec!["knuth1984".to_string(), "lamport1994".to_string()])
        );
        let datasources = biber_datasource_specs(source);
        assert_eq!(datasources.len(), 1);
        assert_eq!(datasources[0].name, "refs.bib");
        assert!(!datasources[0].glob);
        assert!(datasources[0].supports_cited_entry_fingerprint);
    }

    #[test]
    fn biber_datasource_specs_parse_single_quoted_glob_attributes() {
        let source = r#"<bcf:controlfile>
  <bcf:bibdata section="0">
    <bcf:datasource glob='true' type='file' datatype='bibtex'>refs/*.bib</bcf:datasource>
  </bcf:bibdata>
</bcf:controlfile>"#;

        let datasources = biber_datasource_specs(source);
        assert_eq!(datasources.len(), 1);
        assert_eq!(datasources[0].name, "refs/*.bib");
        assert!(datasources[0].glob);
        assert!(datasources[0].supports_cited_entry_fingerprint);
    }

    #[test]
    fn biber_glob_fingerprint_tracks_matching_file_set() {
        let root = unique_temp_dir("texpilot-biber-glob-fingerprint");
        let refs_dir = root.join("refs");
        fs::create_dir_all(&refs_dir).expect("failed to create refs dir");
        fs::write(refs_dir.join("a.bib"), "@book{a,title={A}}\n")
            .expect("failed to write first bibliography");

        let datasource = BiberDatasource {
            name: "refs/*.bib".to_string(),
            glob: true,
            supports_cited_entry_fingerprint: true,
        };
        let paths = resolve_biber_datasource_paths(&root, &datasource)
            .expect("failed to resolve Biber glob");
        assert_eq!(paths.len(), 1);
        let fingerprint = fingerprint_biber_glob_matches(&root, &datasource.name, &paths)
            .expect("failed to fingerprint Biber glob");
        assert!(
            input_fingerprint_is_fresh(&fingerprint).expect("freshness check failed"),
            "{fingerprint:#?}"
        );

        fs::write(refs_dir.join("b.bib"), "@book{b,title={B}}\n")
            .expect("failed to write second bibliography");
        assert!(
            !input_fingerprint_is_fresh(&fingerprint).expect("freshness check failed"),
            "{fingerprint:#?}"
        );

        let paths = resolve_biber_datasource_paths(&root, &datasource)
            .expect("failed to resolve updated Biber glob");
        assert_eq!(paths.len(), 2);
        let updated = fingerprint_biber_glob_matches(&root, &datasource.name, &paths)
            .expect("failed to fingerprint updated Biber glob");
        assert!(
            input_fingerprint_is_fresh(&updated).expect("freshness check failed"),
            "{updated:#?}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn biber_config_choice_fingerprint_tracks_project_config_appearance() {
        let root = unique_temp_dir("texpilot-biber-config-choice");
        fs::create_dir_all(&root).expect("failed to create temp root");

        let initial_config =
            resolve_biber_config_path(&root).expect("failed to resolve initial Biber config");
        let fingerprint = fingerprint_biber_config_choice(&root, initial_config.as_deref());
        assert!(
            input_fingerprint_is_fresh(&fingerprint).expect("freshness check failed"),
            "{fingerprint:#?}"
        );

        fs::write(root.join("biber.conf"), "[sourcemap]\n")
            .expect("failed to write project Biber config");
        assert!(
            !input_fingerprint_is_fresh(&fingerprint).expect("freshness check failed"),
            "{fingerprint:#?}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn biber_config_inputs_track_project_config_content() {
        let root = unique_temp_dir("texpilot-biber-config-content");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let config_path = root.join("biber.conf");
        fs::write(&config_path, "[sourcemap]\n").expect("failed to write project Biber config");

        let inputs = biber_config_inputs(&root, &HashMap::new())
            .expect("failed to fingerprint Biber config inputs");
        assert_eq!(inputs.len(), 2, "{inputs:#?}");
        assert!(
            inputs
                .iter()
                .all(|input| input_fingerprint_is_fresh(input).expect("freshness check failed")),
            "{inputs:#?}"
        );

        fs::write(&config_path, "[sourcemap]\n  changed = true\n")
            .expect("failed to update project Biber config");
        assert!(
            inputs
                .iter()
                .any(|input| !input_fingerprint_is_fresh(input).expect("freshness check failed")),
            "{inputs:#?}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn biber_citation_keys_fall_back_for_nocite_all() {
        let source = r#"<bcf:controlfile>
  <bcf:bibdata section="0">
    <bcf:datasource type="file" datatype="bibtex" glob="false">refs.bib</bcf:datasource>
  </bcf:bibdata>
  <bcf:section number="0">
    <bcf:citekey order="1" nocite="1">*</bcf:citekey>
  </bcf:section>
</bcf:controlfile>"#;

        assert_eq!(biber_citation_keys(source), None);
    }

    #[test]
    fn fingerprint_reuses_previous_hash_when_metadata_matches() {
        let root = unique_temp_dir("texpilot-fingerprint-reuse");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let input = root.join("input.txt");
        fs::write(&input, "unchanged bytes\n").expect("failed to write input");

        let first = fingerprint_path_reusing(&input, None)
            .expect("fingerprint failed")
            .expect("missing fingerprint");
        let reused = FileFingerprint {
            hash: "reused-without-reading".to_string(),
            ..first
        };
        let mut previous = HashMap::new();
        previous.insert(reused.path.clone(), reused.clone());

        let second = fingerprint_path_reusing(&input, Some(&previous))
            .expect("fingerprint failed")
            .expect("missing fingerprint");
        assert_eq!(second, reused);

        let _ = fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn effective_tex_fingerprint_reuses_previous_metadata_without_reading() {
        use std::os::unix::fs::PermissionsExt;

        let root = unique_temp_dir("texpilot-effective-fingerprint-reuse");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let input = root.join("main.tex");
        fs::write(
            &input,
            "\\documentclass{article}\n\\begin{document}A\\end{document}\n",
        )
        .expect("failed to write input");

        let initial = fingerprint_effective_tex_path_reusing(&input, None, EffectiveTexMode::Root)
            .expect("failed to fingerprint TeX input")
            .expect("fingerprint should exist");
        let mut previous = HashMap::new();
        previous.insert(initial.path.clone(), initial.clone());

        let original_permissions = fs::metadata(&input)
            .expect("failed to inspect input")
            .permissions();
        fs::set_permissions(&input, fs::Permissions::from_mode(0o000))
            .expect("failed to make input unreadable");

        let reused =
            fingerprint_effective_tex_path_reusing(&input, Some(&previous), EffectiveTexMode::Root)
                .expect("unchanged effective fingerprint should not read file")
                .expect("fingerprint should exist");
        assert_eq!(reused, initial);
        assert!(
            input_fingerprint_is_fresh(&initial)
                .expect("unchanged effective freshness should not read file"),
            "{initial:#?}"
        );

        fs::set_permissions(&input, original_permissions).expect("failed to restore permissions");
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn input_freshness_cache_reuses_previous_check_result() {
        let root = unique_temp_dir("texpilot-input-freshness-cache");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let input = root.join("input.txt");
        fs::write(&input, "cached bytes\n").expect("failed to write input");

        let fingerprint = fingerprint_path_reusing(&input, None)
            .expect("fingerprint failed")
            .expect("missing fingerprint");
        let mut freshness = HashMap::new();
        assert!(
            input_fingerprint_is_fresh_cached(&fingerprint, &mut freshness)
                .expect("cached freshness check failed")
        );
        fs::remove_file(&input).expect("failed to remove input");

        assert!(
            input_fingerprint_is_fresh_cached(&fingerprint, &mut freshness)
                .expect("cached freshness check should not touch disk")
        );
        assert!(
            !input_fingerprint_is_fresh(&fingerprint).expect("uncached freshness check failed"),
            "uncached freshness should observe the removed input"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn rerun_detector_handles_common_package_spellings() {
        for (line, reason) in [
            (
                "LaTeX Warning: Label(s) may have changed. Rerun to get cross-references right.",
                "rerun-to-get-cross-references",
            ),
            (
                "Package natbib Warning: Citation(s) may have changed.",
                "citations-changed",
            ),
            (
                "Package biblatex Warning: Please rerun LaTeX.",
                "rerun-latex",
            ),
            (
                "Package rerunfilecheck Warning: File `main.out' has changed.",
                "file-changed",
            ),
            (
                "Package hyperref Warning: Rerun to get /PageLabels entry.",
                "rerun-to-get-cross-references",
            ),
            (
                "Package glossaries Warning: File \"main.gls\" has changed.",
                "file-changed",
            ),
        ] {
            assert!(line_requests_tex_rerun(line), "{line}");
            assert_eq!(line_tex_rerun_reason(line), Some(reason), "{line}");
        }

        for line in [
            "LaTeX Warning: There were undefined references.",
            "LaTeX Warning: Citation `missing' on page 1 undefined.",
            "Package biblatex Warning: Please (re)run Biber on the file: main",
            "No file main.bbl.",
        ] {
            assert!(!line_requests_tex_rerun(line), "{line}");
        }
    }

    #[test]
    fn bibtex_session_cache_is_scoped_to_exact_signature_and_outputs() {
        let cache = AuxToolSessionCache::default();
        let key = BibtexSessionKey {
            state_path: PathBuf::from("main.state.toml"),
            bbl_path: PathBuf::from("main.bbl"),
            signature: "citation:a".to_string(),
        };
        let changed_signature = BibtexSessionKey {
            signature: "citation:b".to_string(),
            ..key.clone()
        };
        let changed_output = BibtexSessionKey {
            bbl_path: PathBuf::from("chapter.bbl"),
            ..key.clone()
        };

        assert!(!cache.bibtex_job_is_fresh(&key));
        cache.mark_bibtex_job_fresh(key.clone());

        assert!(cache.bibtex_job_is_fresh(&key));
        assert!(!cache.bibtex_job_is_fresh(&changed_signature));
        assert!(!cache.bibtex_job_is_fresh(&changed_output));
    }

    #[test]
    fn auto_bibliography_rejects_mixed_backends_with_same_bbl_output() {
        let root = unique_temp_dir("texpilot-mixed-bibliography-conflict");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create temp output dir");
        let main = root.join("main.tex");
        fs::write(&main, "\\documentclass{article}\n").expect("failed to write main source");
        let aux_path = out_dir.join("main.aux");
        let bcf_path = out_dir.join("main.bcf");
        fs::write(
            &aux_path,
            "\\relax\n\\citation{knuth1984}\n\\bibstyle{plain}\n\\bibdata{refs}\n",
        )
        .expect("failed to write root aux file");
        fs::write(&bcf_path, "<bcf:controlfile/>\n").expect("failed to write bcf file");
        fs::write(
            out_dir.join("main.fls"),
            format!(
                "OUTPUT {}\nOUTPUT {}\n",
                aux_path.display(),
                bcf_path.display()
            ),
        )
        .expect("failed to write recorder file");
        let options = test_build_options(&main, &out_dir, DraftPrepass::Never);
        let cache = AuxToolSessionCache::default();

        let error = run_bibliographies_if_needed(&root, &out_dir, "main", &options, &cache)
            .expect_err("conflicting bibliography outputs should fail before launching tools");

        assert!(
            error
                .to_string()
                .contains("both BibTeX and Biber jobs writing"),
            "{error:#}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn root_includeonly_filter_is_cached_per_session() {
        let root = unique_temp_dir("texpilot-includeonly-session-cache");
        let main = root.join("main.tex");
        fs::create_dir_all(&root).expect("failed to create temp dir");
        fs::write(
            &main,
            "\\documentclass{article}\n\\includeonly{chapters/a}\n\\begin{document}\\end{document}\n",
        )
        .expect("failed to write main source");
        let cache = AuxToolSessionCache::default();

        let first = cache
            .root_includeonly_filter(&main)
            .expect("first includeonly parse failed");
        fs::remove_file(&main).expect("failed to remove main source");
        let second = cache
            .root_includeonly_filter(&main)
            .expect("cached includeonly parse should not read source");

        assert_eq!(first, second);
        assert_eq!(
            second.expect("includeonly filter missing"),
            HashSet::from(["chapters/a".to_string()])
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_external_tools_are_checked_once_per_session() {
        let root = unique_temp_dir("texpilot-source-external-session");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}Hi\\end{document}\n",
        )
        .expect("failed to write main source");
        let options = test_build_options(&main, &out_dir, DraftPrepass::Never);
        let cache = AuxToolSessionCache::default();

        assert_eq!(
            run_source_external_tools_if_needed_once(&root, &out_dir, &main, &options, &cache)
                .expect("first source external-tool scan failed"),
            0
        );
        fs::remove_file(&main).expect("failed to remove main source");
        assert_eq!(
            run_source_external_tools_if_needed_once(&root, &out_dir, &main, &options, &cache)
                .expect("second source external-tool scan should be cached"),
            0
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_external_tool_jobs_are_cached_per_session() {
        let root = unique_temp_dir("texpilot-source-external-job-cache");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        let main = root.join("main.tex");
        let eps = root.join("figure.eps");
        fs::write(
            &main,
            "\\documentclass{article}\n\
             \\begin{document}\n\
             \\includegraphics{figure.eps}\n\
             \\end{document}\n",
        )
        .expect("failed to write main source");
        fs::write(&eps, "%!PS-Adobe-3.0 EPSF-3.0\n").expect("failed to write EPS placeholder");
        let options = test_build_options(&main, &out_dir, DraftPrepass::Never);
        let cache = AuxToolSessionCache::default();

        let first = source_external_tool_jobs(&root, &out_dir, &main, &options, &cache)
            .expect("first source external-tool job scan failed");
        assert_eq!(first.eps.len(), 1, "{first:#?}");
        assert!(first.svg.is_empty(), "{first:#?}");

        fs::remove_file(&main).expect("failed to remove main source");
        let second = source_external_tool_jobs(&root, &out_dir, &main, &options, &cache)
            .expect("second source external-tool job scan should be cached");
        assert_eq!(second.eps, first.eps);
        assert_eq!(second.svg, first.svg);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn tex_source_read_cache_reuses_source_text() {
        let root = unique_temp_dir("texpilot-source-read-cache");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        let main = root.join("main.tex");
        fs::write(&main, "\\documentclass{article}\n").expect("failed to write main source");
        let source_cache = TexSourceReadCache::default();
        let context = SourceConversionContext {
            doc_dir: &root,
            out_dir: &out_dir,
            includeonly: None,
            source_cache: &source_cache,
        };
        let canonical = canonical_tex_source_path(&main).expect("canonical source failed");

        let first = read_cached_tex_source(context.source_cache, &canonical)
            .expect("first source read failed");
        fs::remove_file(&main).expect("failed to remove source after first read");
        let second = read_cached_tex_source(context.source_cache, &canonical)
            .expect("cached source read failed");

        assert_eq!(first, second);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn tex_source_analysis_cache_reuses_parsed_source_facts() {
        let root = unique_temp_dir("texpilot-source-analysis-cache");
        fs::create_dir_all(&root).expect("failed to create temp dir");
        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\
             \\graphicspath{{figures/}}\n\
             \\tikzexternalize[prefix=tikz/]\n\
             \\includegraphics{plot}\n\
             \\includesvg[inkscape=true]{diagram}\n",
        )
        .expect("failed to write main source");
        let source_cache = TexSourceReadCache::default();
        let canonical = canonical_tex_source_path(&main).expect("canonical source failed");

        let first = read_cached_tex_source_analysis(&source_cache, &canonical)
            .expect("first source analysis failed");
        fs::remove_file(&main).expect("failed to remove source after first analysis");
        let second = read_cached_tex_source_analysis(&source_cache, &canonical)
            .expect("cached source analysis failed");

        assert_eq!(
            first.includegraphics_payloads,
            second.includegraphics_payloads
        );
        assert_eq!(first.includesvg_refs, second.includesvg_refs);
        assert_eq!(first.graphicspath_entries, ["figures/"]);
        assert!(second.pgf_externalize.uses_externalize);
        assert!(second.features.has_graphics);
        assert_eq!(second.features.graphic_command_count, 2);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_preflight_scan_combines_output_features_and_pgf() {
        let root = unique_temp_dir("texpilot-source-preflight-scan");
        let out_dir = root.join("out");
        let sections = root.join("sections");
        fs::create_dir_all(&sections).expect("failed to create section dir");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        let main = root.join("main.tex");
        let section = sections.join("a.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}\n\\include{sections/a}\n\\end{document}\n",
        )
        .expect("failed to write main source");
        fs::write(
            &section,
            "\\tikzexternalize[prefix=figures/]\n\\includegraphics{figure}\n",
        )
        .expect("failed to write section source");
        let source_cache = TexSourceReadCache::default();

        let scan = source_preflight_scan(
            &root,
            &out_dir,
            &main,
            None,
            &source_cache,
            SourcePreflightOptions {
                collect_features: true,
                collect_pgf_externalize: true,
                prepare_output_subdirs: true,
            },
        )
        .expect("preflight scan failed");

        assert!(out_dir.join("sections").is_dir());
        assert!(scan.features.has_graphics);
        assert_eq!(scan.features.graphic_command_count, 1);
        assert!(scan.pgf_externalize.uses_externalize);
        assert!(!scan.pgf_externalize.has_explicit_mode);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_seed_dependencies_follow_kpathsea_texinputs_tree() {
        if !command_available("kpsewhich") {
            eprintln!("skipping Kpathsea source seed test; kpsewhich is not available");
            return;
        }

        let root = unique_temp_dir("texpilot-source-kpathsea-seed");
        let paper = root.join("paper");
        let shared = root.join("shared").join("tex");
        fs::create_dir_all(&paper).expect("failed to create paper tree");
        fs::create_dir_all(&shared).expect("failed to create shared tree");
        let _env_guard = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let _texinputs = EnvVarGuard::set(
            "TEXINPUTS",
            format!("{}//{}", shared.display(), KPATHSEA_PATH_SEPARATOR),
        );
        let main = paper.join("main.tex");
        let package = shared.join("sharedpkg.sty");
        let package_extra = shared.join("sharedpkg-extra.def");
        let section = shared.join("sharedsection.tex");
        let section_data = shared.join("sharedsection-extra.dat");
        let data = shared.join("shareddata.dat");
        fs::write(
            &main,
            "\\documentclass{article}\n\
             \\usepackage{sharedpkg}\n\
             \\begin{document}\n\
             \\input{sharedsection}\n\
             \\input{shareddata.dat}\n\
             \\end{document}\n",
        )
        .expect("failed to write main source");
        fs::write(
            &package,
            "\\ProvidesPackage{sharedpkg}\n\\input{sharedpkg-extra.def}\n",
        )
        .expect("failed to write shared package");
        fs::write(&package_extra, "\\def\\sharedpkgextra{}\n")
            .expect("failed to write shared package dependency");
        fs::write(&section, "\\input{sharedsection-extra.dat}\n")
            .expect("failed to write shared section");
        fs::write(&section_data, "Shared section data.\n")
            .expect("failed to write shared section data");
        fs::write(&data, "Shared data.\n").expect("failed to write shared data");

        let paths = source_seed_dependency_paths(&main).expect("source seed scan failed");

        assert!(
            paths
                .iter()
                .any(|path| path == &package.canonicalize().unwrap()),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path == &package_extra.canonicalize().unwrap()),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path == &section.canonicalize().unwrap()),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path == &section_data.canonicalize().unwrap()),
            "{paths:#?}"
        );
        assert!(
            paths
                .iter()
                .any(|path| path == &data.canonicalize().unwrap()),
            "{paths:#?}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn external_tool_input_collection_reuses_cached_source_jobs() {
        let root = unique_temp_dir("texpilot-external-input-cache");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        let main = root.join("main.tex");
        let eps = root.join("figure.eps");
        fs::write(
            &main,
            "\\documentclass{article}\n\
             \\begin{document}\n\
             \\includegraphics{figure.eps}\n\
             \\end{document}\n",
        )
        .expect("failed to write main source");
        fs::write(&eps, "%!PS-Adobe-3.0 EPSF-3.0\n").expect("failed to write EPS placeholder");
        let options = test_build_options(&main, &out_dir, DraftPrepass::Never);
        let cache = AuxToolSessionCache::default();
        let mut inputs = Vec::new();

        source_external_tool_jobs(&root, &out_dir, &main, &options, &cache)
            .expect("source external-tool job scan failed");
        fs::remove_file(&main).expect("failed to remove main source");

        append_external_tool_inputs(
            &mut inputs,
            &root,
            &out_dir,
            "main",
            &main,
            &options,
            Some(&cache),
        )
        .expect("cached external-tool input collection should not rescan source");
        assert!(
            append_external_tool_inputs(
                &mut inputs,
                &root,
                &out_dir,
                "main",
                &main,
                &options,
                None
            )
            .is_err(),
            "uncached external-tool input collection should need the source file"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn aux_tool_input_paths_reuses_cached_source_jobs() {
        let root = unique_temp_dir("texpilot-aux-input-cache");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create output dir");
        let main = root.join("main.tex");
        let eps = root.join("figure.eps");
        fs::write(
            &main,
            "\\documentclass{article}\n\
             \\usepackage{graphicx}\n\
             \\begin{document}\n\
             \\includegraphics{figure.eps}\n\
             \\end{document}\n",
        )
        .expect("failed to write main source");
        fs::write(&eps, "%!PS-Adobe-3.0 EPSF-3.0\n").expect("failed to write EPS placeholder");
        let options = test_build_options(&main, &out_dir, DraftPrepass::Never);
        let cache = AuxToolSessionCache::default();

        source_external_tool_jobs(&root, &out_dir, &main, &options, &cache)
            .expect("source external-tool job scan failed");
        fs::remove_file(&main).expect("failed to remove main source");

        aux_tool_input_paths(&out_dir, &root, "main", &main, &options, Some(&cache))
            .expect("cached aux-tool input discovery should not rescan source");
        assert!(
            aux_tool_input_paths(&out_dir, &root, "main", &main, &options, None).is_err(),
            "uncached aux-tool input discovery should need the source file"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn standard_rerun_output_snapshot_tracks_small_latex_state_files() {
        let root = unique_temp_dir("texpilot-standard-rerun-snapshot");
        let nested = root.join("sections");
        fs::create_dir_all(&nested).expect("failed to create temp output dirs");
        fs::write(root.join("main.aux"), "a").expect("failed to write aux");
        fs::write(root.join("main.brf"), "b").expect("failed to write brf");
        fs::write(nested.join("chapter.aux"), "c").expect("failed to write nested aux");
        fs::write(root.join("main.log"), "ignored").expect("failed to write log");
        fs::write(root.join("main.pdf"), "ignored").expect("failed to write pdf");

        let snapshot =
            standard_rerun_output_snapshot(&root).expect("standard output snapshot failed");
        let paths = snapshot
            .into_iter()
            .map(|fingerprint| fingerprint.path)
            .collect::<Vec<_>>();

        assert_eq!(paths.len(), 3, "{paths:#?}");
        assert!(paths.iter().any(|path| path.ends_with("main.aux")));
        assert!(paths.iter().any(|path| path.ends_with("main.brf")));
        assert!(paths.iter().any(|path| path.ends_with("chapter.aux")));
        assert!(!paths.iter().any(|path| path.ends_with("main.log")));
        assert!(!paths.iter().any(|path| path.ends_with("main.pdf")));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn output_file_snapshot_batches_large_path_sets() {
        let root = unique_temp_dir("texpilot-output-snapshot-batch");
        fs::create_dir_all(&root).expect("failed to create temp output dir");
        let mut paths = Vec::new();
        for index in 0..24 {
            let path = root.join(format!("generated-{index}.aux"));
            fs::write(&path, format!("generated {index}\n"))
                .expect("failed to write generated output");
            paths.push(path);
        }
        paths.push(root.join("missing.aux"));
        paths.push(paths[3].clone());

        let snapshot = output_file_snapshot(paths).expect("output snapshot failed");
        assert_eq!(snapshot.len(), 24, "{snapshot:#?}");
        let mut seen = HashSet::new();
        for fingerprint in snapshot {
            assert!(fingerprint.path.ends_with(".aux"), "{fingerprint:#?}");
            assert!(fingerprint.len > 0, "{fingerprint:#?}");
            assert!(seen.insert(fingerprint.path), "duplicate snapshot entry");
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn settled_aux_cache_restores_fresh_small_sidecars() {
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let root = unique_temp_dir("texpilot-settled-aux-cache");
        let out_dir = root.join("out");
        let fresh_out_dir = root.join("fresh-out");
        let cache_root = root.join("cache");
        fs::create_dir_all(out_dir.join("sections")).expect("failed to create output dirs");
        fs::create_dir_all(&fresh_out_dir).expect("failed to create fresh output dir");
        let _cache_guard = EnvVarGuard::set("TEXPILOT_AUX_CACHE", cache_root.display().to_string());

        let main = root.join("main.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}Hi\\end{document}\n",
        )
        .expect("failed to write main source");
        fs::write(out_dir.join("main.aux"), "\\relax\n").expect("failed to write aux");
        fs::write(out_dir.join("main.bbl"), "\\begin{thebibliography}{1}\n")
            .expect("failed to write bbl");
        fs::write(out_dir.join("sections").join("intro.aux"), "\\relax\n")
            .expect("failed to write nested aux");
        fs::write(
            out_dir.join(".texpilot-main.bibstate.toml"),
            format!(
                "version = 10\nsignature = \"test\"\nbbl_path = \"{}\"\n",
                out_dir.join("main.bbl").display()
            ),
        )
        .expect("failed to write bib state");
        fs::write(out_dir.join(".texpilot-main.state.toml"), "not cached\n")
            .expect("failed to write build state");
        fs::write(out_dir.join("main.log"), "not cached\n").expect("failed to write log");
        fs::write(out_dir.join("main.pdf"), "%PDF cached artifact\n").expect("failed to write pdf");

        let main = main.canonicalize().expect("failed to canonicalize main");
        let options = test_build_options(&main, &out_dir, DraftPrepass::Never);
        let mode_key = direct_mode_key(&options, &main);
        let fingerprint = fingerprint_path_reusing(&main, None)
            .expect("failed to fingerprint main")
            .expect("missing main fingerprint");
        let state = BuildState {
            version: BUILD_STATE_VERSION,
            mode_key: mode_key.clone(),
            pdf_path: out_dir.join("main.pdf").display().to_string(),
            inputs: vec![fingerprint],
        };

        save_settled_aux_cache(
            &root,
            &out_dir,
            &main,
            &mode_key,
            &state,
            Some(&out_dir.join("main.pdf")),
            None,
            false,
        )
        .expect("failed to save settled aux cache");
        let restored = restore_settled_aux_cache_if_fresh(
            &root,
            &fresh_out_dir,
            &main,
            &mode_key,
            &fresh_out_dir.join("main.pdf"),
        )
        .expect("failed to restore settled aux cache")
        .expect("settled aux cache should restore");

        assert_eq!(
            restored.state.pdf_path,
            fresh_out_dir.join("main.pdf").display().to_string()
        );
        assert!(!restored.accept_stale_final_pdf);
        assert!(restored.restored_pdf);
        assert_eq!(
            fs::read_to_string(fresh_out_dir.join("main.aux")).expect("missing restored aux"),
            "\\relax\n"
        );
        assert_eq!(
            fs::read_to_string(fresh_out_dir.join("main.pdf")).expect("missing restored pdf"),
            "%PDF cached artifact\n"
        );
        assert_eq!(
            fs::read_to_string(fresh_out_dir.join("main.bbl")).expect("missing restored bbl"),
            "\\begin{thebibliography}{1}\n"
        );
        assert_eq!(
            fs::read_to_string(fresh_out_dir.join("sections").join("intro.aux"))
                .expect("missing restored nested aux"),
            "\\relax\n"
        );
        assert_eq!(
            fs::read_to_string(fresh_out_dir.join(".texpilot-main.bibstate.toml"))
                .expect("missing restored bib state"),
            format!(
                "version = 10\nsignature = \"test\"\nbbl_path = \"{}\"\n",
                fresh_out_dir.join("main.bbl").display()
            )
        );
        assert!(!fresh_out_dir.join("main.log").exists());
        assert!(!fresh_out_dir.join(".texpilot-main.state.toml").exists());

        fs::write(
            &main,
            "\\documentclass{article}\n\\begin{document}Bye\\end{document}\n",
        )
        .expect("failed to edit main source");
        let stale_out_dir = root.join("stale-out");
        fs::create_dir_all(&stale_out_dir).expect("failed to create stale output dir");
        assert!(
            restore_settled_aux_cache_if_fresh(
                &root,
                &stale_out_dir,
                &main,
                &mode_key,
                &stale_out_dir.join("main.pdf"),
            )
            .expect("failed to test stale settled aux cache")
            .is_none()
        );
        assert!(!stale_out_dir.join("main.pdf").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn generated_file_extension_classifiers_are_case_insensitive() {
        assert!(is_tex_source_extension(Path::new("chapter.TEX")));
        assert!(path_extension_is_any(Path::new("main.AUX"), &["aux"]));
        assert!(is_standard_rerun_output(Path::new("main.BRF")));
        assert!(is_settled_aux_cache_sidecar(Path::new("main.BBL")));
        assert!(is_settled_aux_cache_sidecar(Path::new(
            ".texpilot-main.bibstate.toml"
        )));
        assert!(!is_settled_aux_cache_sidecar(Path::new(
            ".texpilot-main.state.toml"
        )));
        assert_eq!(
            makeindex_input_kind(Path::new("people.IDX")),
            Some(MakeIndexKind::Index)
        );
        assert_eq!(
            makeindex_input_kind(Path::new("main.GLO")),
            Some(MakeIndexKind::Glossary)
        );
        assert!(is_gnuplottex_script(Path::new(
            "main-gnuplottex-fig1.GNUPLOT"
        )));
        assert!(ends_with_ignore_ascii_case("people.IDX", ".idx"));
    }

    #[test]
    fn makeindex_command_parser_accepts_uppercase_idx_arguments() {
        let parsed = parse_makeindex_command("makeindex -s custom.ist PEOPLE.IDX");

        assert_eq!(parsed.len(), 1, "{parsed:#?}");
        assert_eq!(parsed[0].0, "PEOPLE.IDX");
        assert_eq!(parsed[0].1.style.as_deref(), Some("custom.ist"));
    }

    #[test]
    fn stale_final_pdf_after_draft_is_reserved_for_file_churn() {
        let plain_features = SourceFeatures {
            has_graphics: true,
            has_multipass_signal: true,
            has_backref_signal: false,
            graphic_command_count: 1,
        };
        let citation_only_reasons = vec![
            "citations-changed".to_string(),
            "rerun-to-get-cross-references".to_string(),
        ];
        assert!(!should_accept_stale_final_pdf_after_stable_draft(
            Some(&plain_features),
            &citation_only_reasons,
            false
        ));

        let file_changed_reasons = vec!["file-changed".to_string()];
        assert!(should_accept_stale_final_pdf_after_stable_draft(
            Some(&plain_features),
            &file_changed_reasons,
            false
        ));
        assert!(!should_accept_stale_final_pdf_after_stable_draft(
            Some(&plain_features),
            &file_changed_reasons,
            true
        ));

        let backref_features = SourceFeatures {
            has_backref_signal: true,
            ..plain_features
        };
        assert!(should_accept_stale_final_pdf_after_stable_draft(
            Some(&backref_features),
            &citation_only_reasons,
            true
        ));
    }

    #[test]
    fn full_layout_pdf_promotion_uses_stable_standard_file_churn_signal() {
        assert_eq!(full_layout_pdf_promotion_threshold(false, 0, false), 1);
        assert_eq!(full_layout_pdf_promotion_threshold(true, 0, false), 2);
        assert_eq!(full_layout_pdf_promotion_threshold(true, 0, true), 1);
        assert_eq!(full_layout_pdf_promotion_threshold(true, 1, false), 1);
    }

    #[test]
    fn final_pdf_accepts_only_stale_standard_rerun_warnings() {
        let standard_reasons = vec![
            "file-changed".to_string(),
            "rerun-to-get-cross-references".to_string(),
        ];
        assert!(can_accept_final_pdf_with_stale_rerun_warnings(
            false,
            false,
            false,
            false,
            false,
            &standard_reasons,
            false,
        ));
        assert!(!can_accept_final_pdf_with_stale_rerun_warnings(
            false,
            false,
            true,
            false,
            false,
            &standard_reasons,
            false,
        ));
        assert!(!can_accept_final_pdf_with_stale_rerun_warnings(
            false,
            false,
            false,
            false,
            true,
            &standard_reasons,
            false,
        ));
        assert!(can_accept_final_pdf_with_stale_rerun_warnings(
            false,
            false,
            false,
            false,
            true,
            &standard_reasons,
            true,
        ));

        let citation_reasons = vec!["citations-changed".to_string()];
        assert!(!can_accept_final_pdf_with_stale_rerun_warnings(
            false,
            false,
            false,
            false,
            false,
            &citation_reasons,
            true,
        ));
    }

    #[test]
    fn direct_mode_key_tracks_build_state_version() {
        let root = unique_temp_dir("texpilot-mode-key-version");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        fs::write(&main, "\\documentclass{article}\n").expect("failed to write main source");
        let options = test_build_options(&main, &root.join("out"), DraftPrepass::Never);
        let key = direct_mode_key(&options, &main);

        assert!(
            key.starts_with(&format!("v{BUILD_STATE_VERSION};")),
            "{key}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn build_state_compatibility_rejects_previous_schema_version() {
        let root = unique_temp_dir("texpilot-state-version");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        let pdf = root.join("main.pdf");
        fs::write(&main, "\\documentclass{article}\n").expect("failed to write main source");
        fs::write(&pdf, "%PDF placeholder\n").expect("failed to write placeholder pdf");
        let options = test_build_options(&main, &root.join("out"), DraftPrepass::Never);
        let mode_key = direct_mode_key(&options, &main);
        let old_state = BuildState {
            version: BUILD_STATE_VERSION - 1,
            mode_key: mode_key.clone(),
            pdf_path: pdf.display().to_string(),
            inputs: Vec::new(),
        };
        let current_state = BuildState {
            version: BUILD_STATE_VERSION,
            mode_key,
            pdf_path: pdf.display().to_string(),
            inputs: Vec::new(),
        };

        assert!(!build_state_is_compatible(
            &old_state,
            &current_state.mode_key,
            &pdf
        ));
        assert!(build_state_is_compatible(
            &current_state,
            &current_state.mode_key,
            &pdf
        ));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn build_dependency_paths_seed_source_graph_without_state() {
        let root = unique_temp_dir("texpilot-dependency-seed");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create temp output dir");
        let main = root.join("main.tex");
        let section = root.join("section.tex");
        let data = root.join("values.dat");
        let nested_data = root.join("nested.dat");
        let refs = root.join("refs.bib");
        let biber_refs = root.join("bib").join("more.bib");
        let style = root.join("custom.bst");
        let class = root.join("customclass.cls");
        let package = root.join("localpkg.sty");
        let package_dependency = root.join("package-extra.def");
        let figure = root.join("figures").join("plot.pdf");
        let kpathsea_figure = root
            .join("shared")
            .join("figures")
            .join("kpathsea-plot.pdf");
        let animation_frame_0 = root.join("figures").join("frames").join("frame-0.png");
        let animation_frame_1 = root.join("figures").join("frames").join("frame-1.png");
        let animation_frame_2 = root.join("figures").join("frames").join("frame-2.png");
        let svg = root.join("figures").join("icon.svg");
        let kpathsea_svg = root
            .join("shared")
            .join("figures")
            .join("kpathsea-icon.svg");
        let included_pdf = root.join("figures").join("supplement.pdf");
        let kpathsea_pdf = root
            .join("shared")
            .join("figures")
            .join("kpathsea-supplement.pdf");
        let snippet = root.join("snippets").join("example.py");
        let listing = root.join("snippets").join("data.json");
        let kpathsea_listing = root
            .join("shared")
            .join("data")
            .join("kpathsea-listing.json");
        let plot_points = root.join("tables").join("points.dat");
        let plot_curve = root.join("tables").join("curve.csv");
        let kpathsea_curve = root.join("shared").join("data").join("kpathsea-curve.csv");
        let datatool_measurements = root.join("tables").join("measurements.csv");
        let csvsimple_rows = root.join("tables").join("rows.csv");
        let kpathsea_csv_rows = root.join("shared").join("data").join("kpathsea-rows.csv");
        let external_aux = root.join("refs").join("supplement.aux");
        let external_nested_aux = root.join("refs").join("sections").join("chapter.aux");
        let zref_aux = root.join("refs").join("zref.aux");
        let zref_nested_aux = root.join("refs").join("sections").join("zchapter.aux");
        let standalone = root.join("figures").join("standalone.tex");
        let standalone_data = root.join("figures").join("standalone-data.txt");
        let media = root.join("media").join("demo.mp4");
        let attachment = root.join("artifacts").join("data.csv");
        fs::create_dir_all(root.join("bib")).expect("failed to create bibliography directory");
        fs::create_dir_all(root.join("figures")).expect("failed to create figure directory");
        fs::create_dir_all(root.join("shared").join("figures"))
            .expect("failed to create shared figure directory");
        fs::create_dir_all(root.join("shared").join("data"))
            .expect("failed to create shared data directory");
        fs::create_dir_all(root.join("figures").join("frames"))
            .expect("failed to create animation frame directory");
        fs::create_dir_all(root.join("snippets")).expect("failed to create snippets directory");
        fs::create_dir_all(root.join("tables")).expect("failed to create tables directory");
        fs::create_dir_all(root.join("refs")).expect("failed to create external refs directory");
        fs::create_dir_all(root.join("refs").join("sections"))
            .expect("failed to create nested external refs directory");
        fs::create_dir_all(root.join("media")).expect("failed to create media directory");
        fs::create_dir_all(root.join("artifacts")).expect("failed to create artifacts directory");
        fs::write(
            &main,
            "\\documentclass{customclass}\n\
             \\usepackage{localpkg}\n\
             \\graphicspath{{figures/}}\n\
             \\includegraphics{plot}\n\
             \\includegraphics{kpathsea-plot}\n\
             \\animategraphics[controls]{12}{frames/frame-}{0}{2}\n\
             \\includesvg{icon}\n\
             \\includesvg{kpathsea-icon}\n\
             \\includepdf{supplement}\n\
             \\includepdf{kpathsea-supplement}\n\
             \\inputminted{python}{snippets/example.py}\n\
             \\lstinputlisting{snippets/data.json}\n\
             \\lstinputlisting{data/kpathsea-listing.json}\n\
             \\DTLloaddb[noheader]{measurements}{tables/measurements.csv}\n\
             \\csvreader[head to column names]{tables/rows.csv}{}{\\name}\n\
             \\csvreader{data/kpathsea-rows.csv}{}{\\name}\n\
             \\externaldocument[prefix-]{refs/supplement}\n\
             \\zexternaldocument[z-]{refs/zref}\n\
             \\pgfplotstableread[col sep=comma]{tables/curve.csv}\\curve\n\
             \\pgfplotstableread[col sep=comma]{data/kpathsea-curve.csv}\\sharedcurve\n\
             \\addplot+[blue] table[x=x,y=y] {tables/points.dat};\n\
             \\includestandalone{figures/standalone}\n\
             \\includemedia{Poster}{media/demo.mp4}\n\
             \\attachfile{artifacts/data.csv}\n\
             \\bibliography{refs}\n\
             \\bibliographystyle{custom}\n\
             \\addbibresource[datatype=bibtex]{bib/more.bib}\n\
             \\input{section}\n\
             \\input{values.dat}\n",
        )
        .expect("failed to write main source");
        fs::write(&section, "\\input{nested.dat}\n").expect("failed to write section source");
        fs::write(&data, [0xff]).expect("failed to write binary data dependency");
        fs::write(&nested_data, [0xfe]).expect("failed to write nested binary data dependency");
        fs::write(&refs, "@book{x,title={X}}\n").expect("failed to write bibliography");
        fs::write(&biber_refs, "@book{y,title={Y}}\n").expect("failed to write Biber bibliography");
        fs::write(&style, "ENTRY{}{}{}\n").expect("failed to write bibliography style");
        fs::write(&class, "\\NeedsTeXFormat{LaTeX2e}\n").expect("failed to write class");
        fs::write(&package, "\\input{package-extra.def}\n").expect("failed to write package");
        fs::write(&package_dependency, "\\def\\extra{}\n").expect("failed to write package dep");
        fs::write(&figure, "%PDF placeholder\n").expect("failed to write figure");
        fs::write(&kpathsea_figure, "%PDF shared placeholder\n")
            .expect("failed to write Kpathsea figure");
        fs::write(&animation_frame_0, "not a real PNG\n")
            .expect("failed to write animation frame 0");
        fs::write(&animation_frame_1, "not a real PNG\n")
            .expect("failed to write animation frame 1");
        fs::write(&animation_frame_2, "not a real PNG\n")
            .expect("failed to write animation frame 2");
        fs::write(&svg, "<svg/>").expect("failed to write SVG");
        fs::write(&kpathsea_svg, "<svg/>").expect("failed to write Kpathsea SVG");
        fs::write(&included_pdf, "%PDF supplement\n").expect("failed to write included PDF");
        fs::write(&kpathsea_pdf, "%PDF shared supplement\n")
            .expect("failed to write Kpathsea included PDF");
        fs::write(&snippet, "print('hello')\n").expect("failed to write snippet");
        fs::write(&listing, "{\"ok\": true}\n").expect("failed to write listing");
        fs::write(&kpathsea_listing, "{\"shared\": true}\n")
            .expect("failed to write Kpathsea listing");
        fs::write(&plot_points, "x y\n0 0\n1 1\n").expect("failed to write plot points");
        fs::write(&plot_curve, "x,y\n0,0\n1,1\n").expect("failed to write plot curve");
        fs::write(&kpathsea_curve, "x,y\n2,3\n").expect("failed to write Kpathsea plot curve");
        fs::write(&datatool_measurements, "name,value\nalpha,1\n")
            .expect("failed to write datatool measurements");
        fs::write(&csvsimple_rows, "name,value\nbeta,2\n").expect("failed to write csv rows");
        fs::write(&kpathsea_csv_rows, "name,value\ngamma,3\n")
            .expect("failed to write Kpathsea csv rows");
        fs::write(
            &external_aux,
            "\\relax\n\\@input{sections/chapter.aux}\n\\newlabel{x}{{1}{1}}\n",
        )
        .expect("failed to write external aux");
        fs::write(&external_nested_aux, "\\newlabel{nested}{{2}{2}}\n")
            .expect("failed to write nested external aux");
        fs::write(
            &zref_aux,
            "\\relax\n\\@input{sections/zchapter.aux}\n\\zref@newlabel{z}{}\n",
        )
        .expect("failed to write zref aux");
        fs::write(&zref_nested_aux, "\\zref@newlabel{znested}{}\n")
            .expect("failed to write nested zref aux");
        fs::write(&standalone, "\\input{figures/standalone-data.txt}\n")
            .expect("failed to write standalone source");
        fs::write(&standalone_data, "standalone data\n").expect("failed to write standalone data");
        fs::write(&media, "not a real mp4\n").expect("failed to write media");
        fs::write(&attachment, "value\n").expect("failed to write attachment");
        let options = test_build_options(&main, &out_dir, DraftPrepass::Never);
        let mut expected = vec![
            main.canonicalize().expect("failed to canonicalize main"),
            class.canonicalize().expect("failed to canonicalize class"),
            package
                .canonicalize()
                .expect("failed to canonicalize package"),
            package_dependency
                .canonicalize()
                .expect("failed to canonicalize package dependency"),
            figure
                .canonicalize()
                .expect("failed to canonicalize figure"),
            kpathsea_figure
                .canonicalize()
                .expect("failed to canonicalize Kpathsea figure"),
            animation_frame_0
                .canonicalize()
                .expect("failed to canonicalize animation frame 0"),
            animation_frame_1
                .canonicalize()
                .expect("failed to canonicalize animation frame 1"),
            animation_frame_2
                .canonicalize()
                .expect("failed to canonicalize animation frame 2"),
            svg.canonicalize().expect("failed to canonicalize SVG"),
            kpathsea_svg
                .canonicalize()
                .expect("failed to canonicalize Kpathsea SVG"),
            included_pdf
                .canonicalize()
                .expect("failed to canonicalize included PDF"),
            kpathsea_pdf
                .canonicalize()
                .expect("failed to canonicalize Kpathsea included PDF"),
            snippet
                .canonicalize()
                .expect("failed to canonicalize snippet"),
            listing
                .canonicalize()
                .expect("failed to canonicalize listing"),
            kpathsea_listing
                .canonicalize()
                .expect("failed to canonicalize Kpathsea listing"),
            plot_points
                .canonicalize()
                .expect("failed to canonicalize plot points"),
            plot_curve
                .canonicalize()
                .expect("failed to canonicalize plot curve"),
            kpathsea_curve
                .canonicalize()
                .expect("failed to canonicalize Kpathsea plot curve"),
            datatool_measurements
                .canonicalize()
                .expect("failed to canonicalize datatool measurements"),
            csvsimple_rows
                .canonicalize()
                .expect("failed to canonicalize csv rows"),
            kpathsea_csv_rows
                .canonicalize()
                .expect("failed to canonicalize Kpathsea csv rows"),
            external_aux
                .canonicalize()
                .expect("failed to canonicalize external aux"),
            external_nested_aux
                .canonicalize()
                .expect("failed to canonicalize nested external aux"),
            zref_aux
                .canonicalize()
                .expect("failed to canonicalize zref aux"),
            zref_nested_aux
                .canonicalize()
                .expect("failed to canonicalize nested zref aux"),
            standalone
                .canonicalize()
                .expect("failed to canonicalize standalone"),
            standalone_data
                .canonicalize()
                .expect("failed to canonicalize standalone data"),
            media.canonicalize().expect("failed to canonicalize media"),
            attachment
                .canonicalize()
                .expect("failed to canonicalize attachment"),
            section
                .canonicalize()
                .expect("failed to canonicalize section"),
            data.canonicalize().expect("failed to canonicalize data"),
            nested_data
                .canonicalize()
                .expect("failed to canonicalize nested data"),
            refs.canonicalize()
                .expect("failed to canonicalize bibliography"),
            biber_refs
                .canonicalize()
                .expect("failed to canonicalize Biber bibliography"),
            style
                .canonicalize()
                .expect("failed to canonicalize bibliography style"),
        ];
        expected.sort();

        let paths = build_dependency_paths(&options).expect("failed to read dependencies");

        assert_eq!(paths, expected, "{paths:#?}");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn build_dependency_paths_filter_virtual_fingerprints() {
        let _env_guard = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let root = unique_temp_dir("texpilot-dependency-virtual-inputs");
        let out_dir = root.join("out");
        fs::create_dir_all(&out_dir).expect("failed to create temp output dir");
        let main = root.join("main.tex");
        let refs = root.join("refs.bib");
        fs::write(&main, "\\documentclass{article}\n").expect("failed to write main source");
        fs::write(&refs, "@book{x,title={X}}\n").expect("failed to write bibliography");
        let options = test_build_options(&main, &out_dir, DraftPrepass::Never);
        let main = main.canonicalize().expect("failed to canonicalize main");
        let refs = refs
            .canonicalize()
            .expect("failed to canonicalize bibliography");
        let mode_key = direct_mode_key(&options, &main);
        let state = BuildState {
            version: BUILD_STATE_VERSION,
            mode_key,
            pdf_path: out_dir.join("main.pdf").display().to_string(),
            inputs: vec![
                FileFingerprint {
                    path: refs.display().to_string(),
                    len: 0,
                    modified_ns: 0,
                    hash: "real".to_string(),
                },
                FileFingerprint {
                    path: biber_glob_fingerprint_path(&root, "refs/*.bib"),
                    len: 0,
                    modified_ns: 0,
                    hash: format!("{BIBER_GLOB_MATCHES_HASH_PREFIX}0000000000000000"),
                },
                FileFingerprint {
                    path: biber_config_fingerprint_path(&root),
                    len: 0,
                    modified_ns: 0,
                    hash: format!("{BIBER_CONFIG_CHOICE_HASH_PREFIX}0000000000000000"),
                },
            ],
        };
        fs::write(
            out_dir.join(".texpilot-main.state.toml"),
            toml::to_string(&state).expect("failed to serialize build state"),
        )
        .expect("failed to write build state");

        let paths = build_dependency_paths(&options).expect("failed to read dependencies");

        assert!(paths.contains(&main), "{paths:#?}");
        assert!(paths.contains(&refs), "{paths:#?}");
        assert!(
            !paths.iter().any(|path| {
                path.to_string_lossy()
                    .starts_with(BIBER_GLOB_FINGERPRINT_PATH_PREFIX)
                    || path
                        .to_string_lossy()
                        .starts_with(BIBER_CONFIG_FINGERPRINT_PATH_PREFIX)
            }),
            "{paths:#?}"
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn pgf_externalize_scan_detects_implicit_and_explicit_modes() {
        let implicit = pgf_externalize_scan(r"\tikzexternalize[prefix=figures/]");
        assert!(implicit.uses_externalize);
        assert!(!implicit.has_explicit_mode);

        let explicit_command = pgf_externalize_scan(r"\tikzexternalize[mode=list and make]");
        assert!(explicit_command.uses_externalize);
        assert!(explicit_command.has_explicit_mode);

        let explicit_tikzset = pgf_externalize_scan(
            r"\tikzset{external/mode=convert with system call}\tikzexternalize",
        );
        assert!(explicit_tikzset.uses_externalize);
        assert!(explicit_tikzset.has_explicit_mode);

        let commented = pgf_externalize_scan(r"% \tikzexternalize");
        assert!(!commented.uses_externalize);
        assert!(!commented.has_explicit_mode);
    }

    #[test]
    fn pgf_externalize_source_scan_respects_includeonly() {
        let root = unique_temp_dir("texpilot-pgf-includeonly");
        fs::create_dir_all(&root).expect("failed to create temp root");
        let main = root.join("main.tex");
        let active = root.join("active.tex");
        let excluded = root.join("excluded.tex");
        fs::write(
            &main,
            "\\documentclass{article}\n\
             \\includeonly{active}\n\
             \\include{active}\n\
             \\include{excluded}\n",
        )
        .expect("failed to write main source");
        fs::write(&active, "\\begin{document}Active chapter.\\end{document}\n")
            .expect("failed to write active source");
        fs::write(&excluded, "\\usetikzlibrary{external}\n\\tikzexternalize\n")
            .expect("failed to write excluded source");

        let includeonly = includeonly_filter_for_root(&main).expect("includeonly parse failed");
        let mut visited = HashSet::new();
        let scan =
            pgf_externalize_scan_from_source(&root, &main, &mut visited, includeonly.as_ref())
                .expect("PGF scan failed");

        assert!(!scan.uses_externalize, "{scan:#?}");
        assert!(!scan.has_explicit_mode, "{scan:#?}");

        let _ = fs::remove_dir_all(root);
    }

    fn test_build_options(
        main: &Path,
        out_dir: &Path,
        draft_prepass: DraftPrepass,
    ) -> BuildOptions {
        BuildOptions {
            main: main.to_path_buf(),
            job_name: None,
            engine: Engine::PdfLatex,
            runner: Runner::Direct,
            bib_mode: BibMode::Auto,
            out_dir: out_dir.to_path_buf(),
            fast: false,
            draft_prepass,
            once: false,
            max_runs: 8,
            force: false,
            precompile_preamble: false,
            synctex: false,
            shell_escape: false,
            quiet: true,
            print_command: false,
        }
    }

    fn command_available(program: &str) -> bool {
        std::env::var_os("PATH")
            .map(|paths| {
                std::env::split_paths(&paths).any(|directory| directory.join(program).is_file())
            })
            .unwrap_or(false)
    }

    struct EnvVarGuard {
        name: &'static str,
        previous: Option<OsString>,
    }

    impl EnvVarGuard {
        fn set(name: &'static str, value: String) -> Self {
            let previous = std::env::var_os(name);
            unsafe {
                std::env::set_var(name, value);
            }
            Self { name, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            unsafe {
                if let Some(previous) = &self.previous {
                    std::env::set_var(self.name, previous);
                } else {
                    std::env::remove_var(self.name);
                }
            }
        }
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let unique = format!(
            "{}-{}-{}",
            prefix,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock before UNIX epoch")
                .as_nanos()
        );
        std::env::temp_dir().join(unique)
    }
}

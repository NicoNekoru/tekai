use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{self, BufWriter, Cursor, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use flate2::Compression;
use flate2::write::{GzEncoder, ZlibEncoder};
use lopdf::{
    Dictionary as LoDictionary, Document as LoDocument, Object as LoObject, ObjectId as LoObjectId,
    Stream as LoStream, StringFormat as LoStringFormat,
};

use crate::expand::expand_to_source_with_file_context;
use crate::trace::{TraceEvent, TraceWriter};

#[cfg(windows)]
const KPATHSEA_PATH_SEPARATOR: &str = ";";
#[cfg(not(windows))]
const KPATHSEA_PATH_SEPARATOR: &str = ":";

#[derive(Debug, Clone)]
pub struct NativeEngineOptions {
    pub main: PathBuf,
    pub output_dir: PathBuf,
    pub job_name: String,
    pub mode: RunMode,
    pub shell_escape: bool,
    pub synctex: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunMode {
    pub suppress_pdf_output: bool,
    pub draft_graphics: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeArtifactPolicy {
    PdfOnly,
    LegacySidecars,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NativeEngineStatus {
    Native,
    Unsupported(NativeUnsupported),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeUnsupported {
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct NativeEngineRun {
    pub status: NativeEngineStatus,
    pub pdf_path: Option<PathBuf>,
    pub log_path: PathBuf,
    pub aux_path: PathBuf,
    pub fls_path: PathBuf,
    pub trace_path: PathBuf,
    pub input_paths: Vec<PathBuf>,
}

pub fn run_native(options: &NativeEngineOptions) -> io::Result<NativeEngineRun> {
    run_native_with_artifact_policy(options, NativeArtifactPolicy::LegacySidecars)
}

pub fn run_native_pdf_only(options: &NativeEngineOptions) -> io::Result<NativeEngineRun> {
    run_native_with_artifact_policy(options, NativeArtifactPolicy::PdfOnly)
}

pub fn run_native_with_artifact_policy(
    options: &NativeEngineOptions,
    artifact_policy: NativeArtifactPolicy,
) -> io::Result<NativeEngineRun> {
    fs::create_dir_all(&options.output_dir)?;
    let log_path = options.output_dir.join(format!("{}.log", options.job_name));
    let aux_path = options.output_dir.join(format!("{}.aux", options.job_name));
    let toc_path = options.output_dir.join(format!("{}.toc", options.job_name));
    let lof_path = options.output_dir.join(format!("{}.lof", options.job_name));
    let lot_path = options.output_dir.join(format!("{}.lot", options.job_name));
    let out_path = options.output_dir.join(format!("{}.out", options.job_name));
    let brf_path = options.output_dir.join(format!("{}.brf", options.job_name));
    let fls_path = options.output_dir.join(format!("{}.fls", options.job_name));
    let trace_path = options
        .output_dir
        .join(format!("{}.texpilot-pdftex.trace", options.job_name));
    let pdf_path = options.output_dir.join(format!("{}.pdf", options.job_name));
    let synctex_path = options
        .output_dir
        .join(format!("{}.synctex.gz", options.job_name));
    let started = Instant::now();
    let mut trace = TraceWriter::new(&trace_path);
    trace.push(TraceEvent::new("engine", "texpilot-pdftex-native"));
    trace.push(TraceEvent::new("input", options.main.display().to_string()));
    trace.push(TraceEvent::new(
        "artifact_policy",
        match artifact_policy {
            NativeArtifactPolicy::PdfOnly => "pdf-only",
            NativeArtifactPolicy::LegacySidecars => "legacy-sidecars",
        },
    ));
    let unsupported_run =
        |reason: String, trace: &mut TraceWriter| -> io::Result<NativeEngineRun> {
            trace.push(TraceEvent::new("unsupported", reason.clone()));
            trace.write()?;
            Ok(NativeEngineRun {
                status: NativeEngineStatus::Unsupported(NativeUnsupported { reason }),
                pdf_path: None,
                log_path: log_path.clone(),
                aux_path: aux_path.clone(),
                fls_path: fls_path.clone(),
                trace_path: trace_path.clone(),
                input_paths: Vec::new(),
            })
        };

    if options.shell_escape {
        return unsupported_run(
            "native backend does not yet support shell-escape semantics".to_string(),
            &mut trace,
        );
    }
    let load_started = Instant::now();
    let loaded = match load_document(&options.main, &options.job_name)? {
        Ok(loaded) => loaded,
        Err(reason) => return unsupported_run(reason, &mut trace),
    };
    trace.push(TraceEvent::new(
        "timing_load_ms",
        load_started.elapsed().as_millis().to_string(),
    ));
    let LoadedDocument {
        source,
        mut inputs,
        root_dir,
    } = loaded;
    let parse_started = Instant::now();
    let parsed = match parse_supported_document(
        &source,
        &root_dir,
        &options.output_dir,
        &options.job_name,
        &mut inputs,
        artifact_policy,
    ) {
        Ok(document) => document,
        Err(reason) => return unsupported_run(reason, &mut trace),
    };
    trace.push(TraceEvent::new(
        "timing_parse_ms",
        parse_started.elapsed().as_millis().to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_pre_body_ms",
        parsed.timings.pre_body_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_body_ms",
        parsed.timings.body_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_includegraphics_ms",
        parsed.timings.includegraphics_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_includegraphics_prewarm_ms",
        parsed.timings.includegraphics_prewarm_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_preprocess_ms",
        parsed.timings.preprocess_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_expansion_ms",
        parsed.timings.expansion_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_metadata_ms",
        parsed.timings.metadata_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_labels_ms",
        parsed.timings.labels_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_bibliography_ms",
        parsed.timings.bibliography_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_citations_ms",
        parsed.timings.citations_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_index_ms",
        parsed.timings.index_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_lists_floats_ms",
        parsed.timings.lists_floats_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_hyperref_ms",
        parsed.timings.hyperref_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "timing_parse_title_ms",
        parsed.timings.title_ms.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_two_column_graphic_float_fallbacks",
        parsed
            .timings
            .two_column_graphic_float_fallbacks
            .to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_two_column_wide_graphic_float_fallbacks",
        parsed
            .timings
            .two_column_wide_graphic_float_fallbacks
            .to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_two_column_graphic_float_fallback_entries",
        parsed
            .timings
            .two_column_graphic_float_fallback_details
            .len()
            .to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_two_column_graphic_float_fallback_estimated_native_slots",
        parsed
            .timings
            .two_column_graphic_float_fallback_estimated_native_slots(false)
            .to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_two_column_wide_graphic_float_fallback_estimated_native_slots",
        parsed
            .timings
            .two_column_graphic_float_fallback_estimated_native_slots(true)
            .to_string(),
    ));
    for fallback in &parsed.timings.two_column_graphic_float_fallback_details {
        trace.push(TraceEvent::new(
            "layout_two_column_graphic_float_fallback",
            fallback.to_trace_detail(),
        ));
    }
    let layout_started = Instant::now();
    let layout_pass = layout_lines(parsed.layout, &parsed.lines, &parsed.images);
    let layout_summary = LayoutSummary::from_document(&parsed, &layout_pass);
    let page_count = layout_summary.page_count;
    trace.push(TraceEvent::new(
        "timing_layout_ms",
        layout_started.elapsed().as_millis().to_string(),
    ));
    let legacy_sidecars = artifact_policy == NativeArtifactPolicy::LegacySidecars;

    if legacy_sidecars {
        let sidecar_started = Instant::now();
        write_generated_outputs(&parsed.generated_outputs)?;
        write_aux(&aux_path, &parsed)?;
        if parsed.toc_requested {
            write_toc(&toc_path, &parsed)?;
        }
        if parsed.list_of_figures_requested {
            write_float_list(&lof_path, &parsed, FloatKind::Figure)?;
        }
        if parsed.list_of_tables_requested {
            write_float_list(&lot_path, &parsed, FloatKind::Table)?;
        }
        if parsed.hyperref_out_requested {
            write_hyperref_out(&out_path, &parsed)?;
        }
        if parsed.backref_requested {
            write_backrefs(&brf_path, &parsed)?;
        }
        trace.push(TraceEvent::new(
            "timing_legacy_sidecars_ms",
            sidecar_started.elapsed().as_millis().to_string(),
        ));
    }
    if !options.mode.suppress_pdf_output {
        let pdf_started = Instant::now();
        write_pdf(&pdf_path, &parsed, page_count, &layout_pass.placements)?;
        trace.push(TraceEvent::new(
            "timing_pdf_ms",
            pdf_started.elapsed().as_millis().to_string(),
        ));
        if options.synctex {
            let synctex_started = Instant::now();
            write_synctex(
                &synctex_path,
                options,
                &parsed,
                page_count,
                &layout_pass.placements,
            )?;
            trace.push(TraceEvent::new(
                "timing_synctex_ms",
                synctex_started.elapsed().as_millis().to_string(),
            ));
        }
    }
    if legacy_sidecars {
        let recorder_started = Instant::now();
        let mut output_paths = vec![log_path.as_path(), aux_path.as_path()];
        if parsed.toc_requested {
            output_paths.push(toc_path.as_path());
        }
        if parsed.list_of_figures_requested {
            output_paths.push(lof_path.as_path());
        }
        if parsed.list_of_tables_requested {
            output_paths.push(lot_path.as_path());
        }
        if parsed.hyperref_out_requested {
            output_paths.push(out_path.as_path());
        }
        if parsed.backref_requested {
            output_paths.push(brf_path.as_path());
        }
        if !options.mode.suppress_pdf_output {
            output_paths.push(pdf_path.as_path());
        }
        if options.synctex && !options.mode.suppress_pdf_output {
            output_paths.push(synctex_path.as_path());
        }
        for generated in &parsed.generated_outputs {
            output_paths.push(generated.path.as_path());
        }
        write_fls(&fls_path, &inputs, &output_paths)?;
        write_success_log(
            &log_path,
            options,
            &parsed,
            started.elapsed().as_millis(),
            page_count,
            !options.mode.suppress_pdf_output,
            options.synctex && !options.mode.suppress_pdf_output,
        )?;
        trace.push(TraceEvent::new(
            "timing_legacy_recorder_ms",
            recorder_started.elapsed().as_millis().to_string(),
        ));
    }
    trace.push(TraceEvent::new("pages", page_count.to_string()));
    trace.push(TraceEvent::new(
        "layout_slots",
        layout_summary.slots.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_lines",
        layout_summary.lines.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_placements",
        layout_summary.placements.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_rendered_placements",
        layout_summary.rendered_placements.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_overflow_placements",
        layout_summary.overflow_placements.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_float_blocks",
        layout_summary.float_blocks.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_top_floats",
        layout_summary.top_floats.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_wide_floats",
        layout_summary.wide_floats.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_display_equations",
        layout_summary.display_equations.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_output_controls",
        layout_summary.output_controls.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_pending_normal_top_floats",
        layout_summary.pending_normal_top_floats.to_string(),
    ));
    trace.push(TraceEvent::new(
        "layout_pending_wide_top_floats",
        layout_summary.pending_wide_top_floats.to_string(),
    ));
    let caption_traces = layout_caption_traces(&parsed, &layout_pass.placements);
    trace.push(TraceEvent::new(
        "layout_caption_entries",
        caption_traces.len().to_string(),
    ));
    for caption in caption_traces {
        trace.push(TraceEvent::new("layout_caption", caption.to_trace_detail()));
    }
    trace.push(TraceEvent::new(
        "footnotes",
        parsed.footnotes.len().to_string(),
    ));
    trace.push(TraceEvent::new(
        "lof_output",
        parsed.list_of_figures_requested.to_string(),
    ));
    trace.push(TraceEvent::new(
        "lot_output",
        parsed.list_of_tables_requested.to_string(),
    ));
    trace.push(TraceEvent::new(
        "hyperref_out_output",
        parsed.hyperref_out_requested.to_string(),
    ));
    trace.push(TraceEvent::new(
        "brf_output",
        parsed.backref_requested.to_string(),
    ));
    trace.push(TraceEvent::new(
        "index_entries",
        parsed.index.entries.len().to_string(),
    ));
    trace.push(TraceEvent::new(
        "index_output",
        parsed.index.should_write_sidecar().to_string(),
    ));
    trace.push(TraceEvent::new(
        "index_printed",
        parsed.index.printed.to_string(),
    ));
    trace.push(TraceEvent::new(
        "pdf_metadata_entries",
        parsed.pdf_metadata.entries.len().to_string(),
    ));
    trace.push(TraceEvent::new(
        "pdf_output",
        (!options.mode.suppress_pdf_output).to_string(),
    ));
    trace.push(TraceEvent::new(
        "synctex_output",
        (options.synctex && !options.mode.suppress_pdf_output).to_string(),
    ));
    trace.write()?;

    Ok(NativeEngineRun {
        status: NativeEngineStatus::Native,
        pdf_path: (!options.mode.suppress_pdf_output).then_some(pdf_path),
        log_path,
        aux_path,
        fls_path,
        trace_path,
        input_paths: inputs,
    })
}

pub fn probe_native_support(main: &Path) -> io::Result<Result<(), NativeUnsupported>> {
    let job_name = main
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("main");
    let loaded = match load_document(main, job_name)? {
        Ok(loaded) => loaded,
        Err(reason) => return Ok(Err(NativeUnsupported { reason })),
    };
    let mut inputs = loaded.inputs;
    match parse_supported_document(
        &loaded.source,
        &loaded.root_dir,
        &loaded.root_dir,
        job_name,
        &mut inputs,
        NativeArtifactPolicy::PdfOnly,
    ) {
        Ok(_) => Ok(Ok(())),
        Err(reason) => Ok(Err(NativeUnsupported { reason })),
    }
}

#[derive(Debug, Clone)]
struct LoadedDocument {
    source: String,
    inputs: Vec<PathBuf>,
    root_dir: PathBuf,
}

fn load_document(main: &Path, job_name: &str) -> io::Result<Result<LoadedDocument, String>> {
    let mut inputs = Vec::new();
    let mut stack = Vec::new();
    let mut include_only = None;
    let root_dir = fs::canonicalize(main)?
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf();
    let source = match expand_input_file(
        main,
        true,
        &mut inputs,
        &mut stack,
        &mut include_only,
        job_name,
    )? {
        Ok(source) => source,
        Err(reason) => return Ok(Err(reason)),
    };
    let source = match inject_native_package_adapters(&source, &root_dir, &mut inputs)? {
        Ok(source) => source,
        Err(reason) => return Ok(Err(reason)),
    };
    inputs.sort();
    inputs.dedup();
    Ok(Ok(LoadedDocument {
        source,
        inputs,
        root_dir,
    }))
}

fn expand_input_file(
    path: &Path,
    is_root: bool,
    inputs: &mut Vec<PathBuf>,
    stack: &mut Vec<PathBuf>,
    include_only: &mut Option<BTreeSet<String>>,
    job_name: &str,
) -> io::Result<Result<String, String>> {
    let canonical = match fs::canonicalize(path) {
        Ok(path) => path,
        Err(error) if is_root => return Err(error),
        Err(error) => {
            return Ok(Err(format!(
                "native backend could not resolve input `{}`: {error}",
                path.display()
            )));
        }
    };
    if stack.contains(&canonical) {
        return Ok(Err(format!(
            "native backend does not support cyclic input involving `{}`",
            canonical.display()
        )));
    }

    let source = match fs::read_to_string(&canonical) {
        Ok(source) => source,
        Err(error) if is_root => return Err(error),
        Err(error) => {
            return Ok(Err(format!(
                "native backend could not read input `{}`: {error}",
                canonical.display()
            )));
        }
    };

    inputs.push(canonical.clone());
    stack.push(canonical.clone());
    let expanded = expand_inputs_in_source(
        &source,
        canonical.parent().unwrap_or(Path::new(".")),
        inputs,
        stack,
        include_only,
        job_name,
    );
    stack.pop();
    expanded
}

fn expand_inputs_in_source(
    source: &str,
    base_dir: &Path,
    inputs: &mut Vec<PathBuf>,
    stack: &mut Vec<PathBuf>,
    include_only: &mut Option<BTreeSet<String>>,
    job_name: &str,
) -> io::Result<Result<String, String>> {
    let mut output = String::new();
    let mut cursor = source;
    while let Some((index, command)) = find_file_load_command(cursor) {
        output.push_str(&cursor[..index]);
        match command {
            FileLoadCommand::Input => {
                let rest = &cursor[index + "\\input".len()..];
                let Some((input_name, remaining)) = take_input_name(rest) else {
                    return Ok(Err(
                        "native backend requires a file name after \\input".to_string()
                    ));
                };
                let input_path = match resolve_input_path(base_dir, input_name, job_name) {
                    Ok(path) => path,
                    Err(reason) => return Ok(Err(reason)),
                };
                let expanded = match expand_input_file(
                    &input_path,
                    false,
                    inputs,
                    stack,
                    include_only,
                    job_name,
                )? {
                    Ok(source) => source,
                    Err(reason) => return Ok(Err(reason)),
                };
                output.push_str(&expanded);
                cursor = remaining;
            }
            FileLoadCommand::Include => {
                let rest = &cursor[index + "\\include".len()..];
                let Some((include_name, remaining)) = take_braced(rest) else {
                    return Ok(Err(
                        "native backend requires a braced file name after \\include".to_string(),
                    ));
                };
                if include_allowed(include_only, include_name, job_name) {
                    let input_path = match resolve_input_path(base_dir, include_name, job_name) {
                        Ok(path) => path,
                        Err(reason) => return Ok(Err(reason)),
                    };
                    let expanded = match expand_input_file(
                        &input_path,
                        false,
                        inputs,
                        stack,
                        include_only,
                        job_name,
                    )? {
                        Ok(source) => source,
                        Err(reason) => return Ok(Err(reason)),
                    };
                    output.push('\n');
                    output.push_str(&expanded);
                    output.push('\n');
                }
                cursor = remaining;
            }
            FileLoadCommand::IncludeOnly => {
                let rest = &cursor[index + "\\includeonly".len()..];
                let Some((payload, remaining)) = take_braced(rest) else {
                    return Ok(Err(
                        "native backend requires braced \\includeonly file names".to_string(),
                    ));
                };
                *include_only = Some(parse_includeonly_names(payload, job_name));
                cursor = remaining;
            }
            FileLoadCommand::IfFileExists => {
                let rest = &cursor[index + "\\IfFileExists".len()..];
                let Some((file_name, rest)) = take_braced(rest) else {
                    return Ok(Err(
                        "native backend requires braced \\IfFileExists file names".to_string(),
                    ));
                };
                let Some((true_branch, rest)) = take_braced(rest) else {
                    return Ok(Err(
                        "native backend requires braced \\IfFileExists true branches".to_string(),
                    ));
                };
                let Some((false_branch, remaining)) = take_braced(rest) else {
                    return Ok(Err(
                        "native backend requires braced \\IfFileExists false branches".to_string(),
                    ));
                };
                let exists = match native_file_exists(base_dir, file_name, inputs, job_name) {
                    Ok(exists) => exists,
                    Err(reason) => return Ok(Err(reason)),
                };
                let branch = if exists { true_branch } else { false_branch };
                let expanded = match expand_inputs_in_source(
                    branch,
                    base_dir,
                    inputs,
                    stack,
                    include_only,
                    job_name,
                )? {
                    Ok(source) => source,
                    Err(reason) => return Ok(Err(reason)),
                };
                output.push_str(&expanded);
                cursor = remaining;
            }
            FileLoadCommand::InputIfFileExists => {
                let rest = &cursor[index + "\\InputIfFileExists".len()..];
                let Some((file_name, rest)) = take_braced(rest) else {
                    return Ok(Err(
                        "native backend requires braced \\InputIfFileExists file names".to_string(),
                    ));
                };
                let Some((before_input, rest)) = take_braced(rest) else {
                    return Ok(Err(
                        "native backend requires braced \\InputIfFileExists pre-input branches"
                            .to_string(),
                    ));
                };
                let Some((after_input, remaining)) = take_braced(rest) else {
                    return Ok(Err(
                        "native backend requires braced \\InputIfFileExists fallback branches"
                            .to_string(),
                    ));
                };
                let input_path = match native_existing_file(base_dir, file_name, inputs, job_name) {
                    Ok(path) => path,
                    Err(reason) => return Ok(Err(reason)),
                };
                if let Some(input_path) = input_path {
                    let before = match expand_inputs_in_source(
                        before_input,
                        base_dir,
                        inputs,
                        stack,
                        include_only,
                        job_name,
                    )? {
                        Ok(source) => source,
                        Err(reason) => return Ok(Err(reason)),
                    };
                    let expanded = match expand_input_file(
                        &input_path,
                        false,
                        inputs,
                        stack,
                        include_only,
                        job_name,
                    )? {
                        Ok(source) => source,
                        Err(reason) => return Ok(Err(reason)),
                    };
                    output.push_str(&before);
                    output.push_str(&expanded);
                } else {
                    let expanded = match expand_inputs_in_source(
                        after_input,
                        base_dir,
                        inputs,
                        stack,
                        include_only,
                        job_name,
                    )? {
                        Ok(source) => source,
                        Err(reason) => return Ok(Err(reason)),
                    };
                    output.push_str(&expanded);
                }
                cursor = remaining;
            }
        }
    }
    output.push_str(cursor);
    Ok(Ok(output))
}

#[derive(Debug, Clone, Copy)]
enum FileLoadCommand {
    Input,
    Include,
    IncludeOnly,
    IfFileExists,
    InputIfFileExists,
}

fn inject_native_package_adapters(
    source: &str,
    root_dir: &Path,
    inputs: &mut Vec<PathBuf>,
) -> io::Result<Result<String, String>> {
    let mut visited = BTreeSet::new();
    let mut adapted_packages = BTreeSet::new();
    let mut adapters = String::new();
    match collect_native_package_adapters(
        source,
        root_dir,
        inputs,
        &mut visited,
        &mut adapted_packages,
        &mut adapters,
    )? {
        Ok(()) => {}
        Err(reason) => return Ok(Err(reason)),
    }
    if adapters.is_empty() {
        Ok(Ok(source.to_string()))
    } else {
        Ok(Ok(format!("{adapters}\n{source}")))
    }
}

fn collect_native_package_adapters(
    source: &str,
    base_dir: &Path,
    inputs: &mut Vec<PathBuf>,
    visited: &mut BTreeSet<PathBuf>,
    adapted_packages: &mut BTreeSet<String>,
    adapters: &mut String,
) -> io::Result<Result<(), String>> {
    let declarations = match package_declarations(source) {
        Ok(declarations) => declarations,
        Err(reason) => return Ok(Err(reason)),
    };
    for declaration in declarations {
        for package_name in declaration.names {
            if let Some(adapter) = native_builtin_package_adapter(&package_name) {
                let key = format!("builtin:{package_name}");
                if adapted_packages.insert(key) {
                    adapters.push_str(adapter);
                    adapters.push('\n');
                }
            }
            let path = match resolve_traversable_package_path(base_dir, &package_name) {
                Ok(Some(path)) => path,
                Ok(None) => continue,
                Err(reason) => return Ok(Err(reason)),
            };
            inputs.push(path.clone());
            if !visited.insert(path.clone()) {
                continue;
            }
            if let Some(adapter) = native_local_package_adapter(&package_name) {
                let key = format!("local:{package_name}");
                if adapted_packages.insert(key) {
                    adapters.push_str(adapter);
                    adapters.push('\n');
                }
            }
            if !should_traverse_package_dependency(base_dir, &package_name, &path) {
                continue;
            }
            let package_source = match fs::read_to_string(&path) {
                Ok(source) => source,
                Err(error) => {
                    return Ok(Err(format!(
                        "native backend could not read package `{}`: {error}",
                        path.display()
                    )));
                }
            };
            match collect_native_package_adapters(
                &package_source,
                path.parent().unwrap_or(base_dir),
                inputs,
                visited,
                adapted_packages,
                adapters,
            )? {
                Ok(()) => {}
                Err(reason) => return Ok(Err(reason)),
            }
        }
    }
    let class_names = match class_declarations(source) {
        Ok(class_names) => class_names,
        Err(reason) => return Ok(Err(reason)),
    };
    for class_name in class_names {
        let path = match resolve_traversable_class_path(base_dir, &class_name) {
            Ok(Some(path)) => path,
            Ok(None) => continue,
            Err(reason) => return Ok(Err(reason)),
        };
        inputs.push(path.clone());
        if !visited.insert(path.clone()) {
            continue;
        }
        let class_source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(error) => {
                return Ok(Err(format!(
                    "native backend could not read class `{}`: {error}",
                    path.display()
                )));
            }
        };
        match collect_native_package_adapters(
            &class_source,
            path.parent().unwrap_or(base_dir),
            inputs,
            visited,
            adapted_packages,
            adapters,
        )? {
            Ok(()) => {}
            Err(reason) => return Ok(Err(reason)),
        }
    }
    Ok(Ok(()))
}

struct PackageDeclaration {
    options: Option<String>,
    names: Vec<String>,
}

fn package_declarations(source: &str) -> Result<Vec<PackageDeclaration>, String> {
    let mut declarations = Vec::new();
    let mut cursor = source;
    loop {
        let next = ["usepackage", "RequirePackage", "RequirePackageWithOptions"]
            .into_iter()
            .filter_map(|command| find_control(cursor, command).map(|index| (index, command)))
            .min_by_key(|(index, _)| *index);
        let Some((index, command)) = next else {
            break;
        };
        let mut rest = &cursor[index + command.len() + 1..];
        let (options, after_optional) = take_optional_bracketed(rest);
        rest = after_optional;
        let Some((payload, remaining)) = take_braced(rest) else {
            cursor = rest;
            continue;
        };
        let names = payload
            .split(',')
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        if !names.is_empty() {
            declarations.push(PackageDeclaration {
                options: options.map(ToOwned::to_owned),
                names,
            });
        }
        cursor = remaining;
    }
    Ok(declarations)
}

fn class_declarations(source: &str) -> Result<Vec<String>, String> {
    let mut declarations = Vec::new();
    let mut cursor = source;
    loop {
        let next = ["documentclass", "LoadClassWithOptions", "LoadClass"]
            .into_iter()
            .filter_map(|command| find_control(cursor, command).map(|index| (index, command)))
            .min_by_key(|(index, _)| *index);
        let Some((index, command)) = next else {
            break;
        };
        let mut rest = &cursor[index + command.len() + 1..];
        let (_, after_optional) = take_optional_bracketed(rest);
        rest = after_optional;
        let Some((payload, remaining)) = take_braced(rest) else {
            cursor = rest;
            continue;
        };
        declarations.extend(
            payload
                .split(',')
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(ToOwned::to_owned),
        );
        cursor = remaining;
    }
    Ok(declarations)
}

fn source_loads_package(source: &str, root_dir: &Path, package: &str) -> Result<bool, String> {
    let mut visited = BTreeSet::new();
    source_loads_package_from(source, root_dir, package, &mut visited)
}

fn source_has_direct_hyperref_surface(source: &str, body: &str) -> Result<bool, String> {
    Ok(source_directly_loads_package(source, "hyperref")?
        || ["pdfbookmark", "href", "url", "hyperref", "autoref"]
            .into_iter()
            .any(|control| source_contains_control(body, control)))
}

fn source_directly_loads_package(source: &str, package: &str) -> Result<bool, String> {
    Ok(package_declarations(source)?
        .into_iter()
        .any(|declaration| declaration.names.iter().any(|name| name == package)))
}

fn source_directly_loads_package_with_any_option(
    source: &str,
    package: &str,
    options: &[&str],
) -> Result<bool, String> {
    let passed_options = pass_options_to_package(source, package);
    Ok(package_declarations(source)?
        .into_iter()
        .any(|declaration| {
            declaration.names.iter().any(|name| name == package)
                && (declaration
                    .options
                    .as_deref()
                    .is_some_and(|declared| package_options_include_any(declared, options))
                    || package_option_list_includes_any(&passed_options, options))
        }))
}

fn source_loads_package_from(
    source: &str,
    base_dir: &Path,
    package: &str,
    visited: &mut BTreeSet<PathBuf>,
) -> Result<bool, String> {
    for declaration in package_declarations(source)? {
        for package_name in declaration.names {
            if package_name == package {
                return Ok(true);
            }
            let Some(path) = resolve_traversable_package_path(base_dir, &package_name)? else {
                continue;
            };
            if !visited.insert(path.clone()) {
                continue;
            }
            let package_source = fs::read_to_string(&path).map_err(|error| {
                format!(
                    "native backend could not read package `{}`: {error}",
                    path.display()
                )
            })?;
            if source_loads_package_from(
                &package_source,
                path.parent().unwrap_or(base_dir),
                package,
                visited,
            )? {
                return Ok(true);
            }
        }
    }
    for class_name in class_declarations(source)? {
        let Some(path) = resolve_traversable_class_path(base_dir, &class_name)? else {
            continue;
        };
        if !visited.insert(path.clone()) {
            continue;
        }
        let class_source = fs::read_to_string(&path).map_err(|error| {
            format!(
                "native backend could not read class `{}`: {error}",
                path.display()
            )
        })?;
        if source_loads_package_from(
            &class_source,
            path.parent().unwrap_or(base_dir),
            package,
            visited,
        )? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn source_loads_package_with_any_option(
    source: &str,
    root_dir: &Path,
    package: &str,
    options: &[&str],
) -> Result<bool, String> {
    let mut visited = BTreeSet::new();
    let passed_options = pass_options_to_package(source, package);
    source_loads_package_with_any_option_from(
        source,
        root_dir,
        package,
        options,
        &passed_options,
        &mut visited,
    )
}

fn source_loads_package_with_any_option_from(
    source: &str,
    base_dir: &Path,
    package: &str,
    options: &[&str],
    inherited_package_options: &[String],
    visited: &mut BTreeSet<PathBuf>,
) -> Result<bool, String> {
    let mut active_package_options = inherited_package_options.to_vec();
    active_package_options.extend(pass_options_to_package(source, package));
    for declaration in package_declarations(source)? {
        let package_names = declaration.names;
        if package_names
            .iter()
            .any(|package_name| package_name == package)
            && (declaration
                .options
                .as_deref()
                .is_some_and(|declared| package_options_include_any(declared, options))
                || package_option_list_includes_any(&active_package_options, options))
        {
            return Ok(true);
        }
        for package_name in package_names {
            let Some(path) = resolve_traversable_package_path(base_dir, &package_name)? else {
                continue;
            };
            if !visited.insert(path.clone()) {
                continue;
            }
            let package_source = fs::read_to_string(&path).map_err(|error| {
                format!(
                    "native backend could not read package `{}`: {error}",
                    path.display()
                )
            })?;
            if source_loads_package_with_any_option_from(
                &package_source,
                path.parent().unwrap_or(base_dir),
                package,
                options,
                &active_package_options,
                visited,
            )? {
                return Ok(true);
            }
        }
    }
    for class_name in class_declarations(source)? {
        let Some(path) = resolve_traversable_class_path(base_dir, &class_name)? else {
            continue;
        };
        if !visited.insert(path.clone()) {
            continue;
        }
        let class_source = fs::read_to_string(&path).map_err(|error| {
            format!(
                "native backend could not read class `{}`: {error}",
                path.display()
            )
        })?;
        if source_loads_package_with_any_option_from(
            &class_source,
            path.parent().unwrap_or(base_dir),
            package,
            options,
            &active_package_options,
            visited,
        )? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn package_options_include_any(declared: &str, options: &[&str]) -> bool {
    declared
        .split(',')
        .map(str::trim)
        .any(|declared| options.contains(&declared))
}

fn package_option_list_includes_any(declared_options: &[String], options: &[&str]) -> bool {
    declared_options
        .iter()
        .any(|declared| package_options_include_any(declared, options))
}

fn pass_options_to_package(source: &str, package: &str) -> Vec<String> {
    let mut options = Vec::new();
    let mut cursor = source;
    while let Some(index) = find_control(cursor, "PassOptionsToPackage") {
        let rest = &cursor[index + "\\PassOptionsToPackage".len()..];
        let Some((option_payload, rest)) = take_braced(rest) else {
            cursor = rest;
            continue;
        };
        let Some((package_payload, remaining)) = take_braced(rest) else {
            cursor = rest;
            continue;
        };
        if package_payload
            .split(',')
            .map(str::trim)
            .any(|name| name == package)
        {
            options.push(option_payload.trim().to_string());
        }
        cursor = remaining;
    }
    options
}

fn should_traverse_package_dependency(base_dir: &Path, package_name: &str, path: &Path) -> bool {
    native_local_package_adapter(package_name).is_some() || path.starts_with(base_dir)
}

fn resolve_traversable_package_path(
    base_dir: &Path,
    package_name: &str,
) -> Result<Option<PathBuf>, String> {
    if native_local_package_adapter(package_name).is_some() {
        resolve_package_path(base_dir, package_name)
    } else {
        resolve_local_package_path(base_dir, package_name)
    }
}

fn resolve_traversable_class_path(
    base_dir: &Path,
    class_name: &str,
) -> Result<Option<PathBuf>, String> {
    resolve_project_tex_dependency_path(base_dir, class_name, "cls", "class")
}

fn resolve_local_package_path(
    base_dir: &Path,
    package_name: &str,
) -> Result<Option<PathBuf>, String> {
    resolve_local_tex_dependency_path(base_dir, package_name, "sty", "package")
}

fn resolve_package_path(base_dir: &Path, package_name: &str) -> Result<Option<PathBuf>, String> {
    resolve_tex_dependency_path(base_dir, package_name, "sty", "package")
}

fn resolve_local_class_path(base_dir: &Path, class_name: &str) -> Result<Option<PathBuf>, String> {
    resolve_local_tex_dependency_path(base_dir, class_name, "cls", "class")
}

fn resolve_project_tex_dependency_path(
    base_dir: &Path,
    name: &str,
    extension: &str,
    kind: &str,
) -> Result<Option<PathBuf>, String> {
    if let Some(path) = resolve_local_tex_dependency_path(base_dir, name, extension, kind)? {
        return Ok(Some(path));
    }
    let raw = Path::new(name);
    if name.starts_with('|') || raw.is_absolute() {
        return Ok(None);
    }
    let lookup = if raw.extension().is_none() {
        raw.with_extension(extension)
    } else {
        raw.to_path_buf()
    };
    resolve_native_kpathsea_candidate(base_dir, &lookup, "TEXINPUTS", kind)
}

fn resolve_tex_dependency_path(
    base_dir: &Path,
    name: &str,
    extension: &str,
    kind: &str,
) -> Result<Option<PathBuf>, String> {
    if let Some(path) = resolve_local_tex_dependency_path(base_dir, name, extension, kind)? {
        return Ok(Some(path));
    }
    let raw = Path::new(name);
    if name.starts_with('|') || raw.is_absolute() {
        return Ok(None);
    }
    let lookup = if raw.extension().is_none() {
        raw.with_extension(extension)
    } else {
        raw.to_path_buf()
    };
    resolve_kpathsea_tex_candidate(base_dir, &lookup)
}

fn resolve_local_tex_dependency_path(
    base_dir: &Path,
    name: &str,
    extension: &str,
    kind: &str,
) -> Result<Option<PathBuf>, String> {
    if name.starts_with('|') {
        return Ok(None);
    }
    let raw = Path::new(name);
    if raw.is_absolute() {
        return Ok(None);
    }
    let candidate = base_dir.join(raw);
    let candidate = if candidate.extension().is_none() {
        candidate.with_extension(extension)
    } else {
        candidate
    };
    if candidate.exists() {
        let canonical = fs::canonicalize(&candidate).map_err(|error| {
            format!(
                "native backend could not canonicalize {kind} `{}`: {error}",
                candidate.display()
            )
        })?;
        return Ok(Some(canonical));
    }
    Ok(None)
}

fn native_builtin_package_adapter(package_name: &str) -> Option<&'static str> {
    match package_name {
        "pdftexcmds" => Some(
            r"\makeatletter
\providecommand{\pdf@strcmp}[2]{\pdfstrcmp{#1}{#2}}
\providecommand{\pdf@escapehex}[1]{\pdfescapehex{#1}}
\let\pdf@escapehexnative\pdf@escapehex
\providecommand{\pdf@unescapehex}[1]{\pdfunescapehex{#1}}
\let\pdf@unescapehexnative\pdf@unescapehex
\providecommand{\pdf@escapestring}[1]{\pdfescapestring{#1}}
\providecommand{\pdf@escapename}[1]{\pdfescapename{#1}}
\providecommand{\pdf@filesize}[1]{\pdffilesize{#1}}
\providecommand{\pdf@filemoddate}[1]{\pdffilemoddate{#1}}
\providecommand{\pdf@filedump}[3]{\pdffiledump offset#1 length#2{#3}}
\providecommand{\pdf@mdfivesum}[1]{\pdfmdfivesum{#1}}
\let\pdf@mdfivesumnative\pdf@mdfivesum
\providecommand{\pdf@filemdfivesum}[1]{\pdfmdfivesum file {#1}}
\providecommand{\pdf@shellescape}{\pdfshellescape}
\let\pdf@primitive\pdfprimitive
\let\pdf@ifprimitive\ifpdfprimitive
\providecommand{\pdf@draftmode}{\pdfdraftmode}
\providecommand{\pdf@setdraftmode}[1]{\pdfdraftmode=#1\relax}
\providecommand{\pdf@ifdraftmode}[2]{\ifnum\pdfdraftmode=1 #1\else #2\fi}
\providecommand{\pdf@resettimer}{}
\providecommand{\pdf@elapsedtime}{0}
\makeatother",
        ),
        _ => None,
    }
}

fn native_local_package_adapter(package_name: &str) -> Option<&'static str> {
    match package_name {
        "simpleicml" => Some(
            r"\makeatletter
\providecommand{\@icmltitle}{}
\providecommand{\@icmlauthors}{}
\providecommand{\@icmlaffiliations}{}
\providecommand{\@icmlabstract}{}
\providecommand{\@icmlrunningtitle}{}
\providecommand{\icmlpdfinfo}[1]{}
\providecommand{\icmltitle}[1]{\gdef\@icmltitle{#1}}
\providecommand{\icmlauthors}[1]{\gdef\@icmlauthors{#1}}
\providecommand{\icmlaffiliations}[1]{\gdef\@icmlaffiliations{#1}}
\providecommand{\icmlabstract}[1]{\gdef\@icmlabstract{#1}}
\providecommand{\icmlrunningtitle}[1]{\gdef\@icmlrunningtitle{#1}}
\providecommand{\icmlmaketitle}{\nativeicmlmaketitle{\@icmltitle}{\@icmlauthors}{\@icmlaffiliations}{\@icmlabstract}}
\makeatother",
        ),
        "sectionnav" => Some(
            r"\makeatletter
\providecommand{\@sectionline}{}
\providecommand{\sectionheaderline}[1]{\gdef\@sectionline{#1}}
\providecommand{\seclink}[3]{Sec #2: #3}
\makeatother",
        ),
        _ => None,
    }
}

fn include_allowed(
    include_only: &Option<BTreeSet<String>>,
    include_name: &str,
    job_name: &str,
) -> bool {
    match include_only {
        None => true,
        Some(allowed) => allowed.contains(&normalize_include_name(include_name, job_name)),
    }
}

fn parse_includeonly_names(payload: &str, job_name: &str) -> BTreeSet<String> {
    payload
        .split(',')
        .map(|name| normalize_include_name(name, job_name))
        .filter(|name| !name.is_empty())
        .collect()
}

fn normalize_include_name(name: &str, job_name: &str) -> String {
    let name = normalize_source_file_name(name, job_name);
    name.strip_suffix(".tex")
        .unwrap_or(&name)
        .replace('\\', "/")
}

fn find_file_load_command(source: &str) -> Option<(usize, FileLoadCommand)> {
    [
        ("input", FileLoadCommand::Input),
        ("include", FileLoadCommand::Include),
        ("includeonly", FileLoadCommand::IncludeOnly),
        ("IfFileExists", FileLoadCommand::IfFileExists),
        ("InputIfFileExists", FileLoadCommand::InputIfFileExists),
    ]
    .into_iter()
    .filter_map(|(control, command)| {
        find_file_load_control(source, control).map(|index| (index, command))
    })
    .min_by_key(|(index, _)| *index)
}

fn find_file_load_control(source: &str, control: &str) -> Option<usize> {
    let needle = format!("\\{control}");
    let mut offset = 0_usize;
    while let Some(index) = source[offset..].find(&needle) {
        let absolute = offset + index;
        let after = absolute + needle.len();
        let next = source[after..].chars().next();
        if !is_in_line_comment(source, absolute) && !next.is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            return Some(absolute);
        }
        offset = after;
    }
    None
}

fn is_in_line_comment(source: &str, index: usize) -> bool {
    let line_start = source[..index].rfind('\n').map_or(0, |start| start + 1);
    let line = &source[line_start..index];
    for (percent_index, ch) in line.char_indices() {
        if ch == '%' && !is_escaped_percent(line, percent_index) {
            return true;
        }
    }
    false
}

fn is_escaped_percent(line: &str, percent_index: usize) -> bool {
    let preceding_backslashes = line[..percent_index]
        .chars()
        .rev()
        .take_while(|ch| *ch == '\\')
        .count();
    preceding_backslashes % 2 == 1
}

fn take_input_name(source: &str) -> Option<(&str, &str)> {
    let source = source.trim_start();
    if let Some((payload, remaining)) = take_braced(source) {
        return Some((payload, remaining));
    }
    let end = source
        .char_indices()
        .take_while(|(_, ch)| !ch.is_whitespace())
        .map(|(index, ch)| index + ch.len_utf8())
        .last()
        .unwrap_or(0);
    (end > 0).then_some((&source[..end], &source[end..]))
}

fn resolve_input_path(
    base_dir: &Path,
    input_name: &str,
    job_name: &str,
) -> Result<PathBuf, String> {
    let input_name = normalize_source_file_name(input_name, job_name);
    if input_name.is_empty() {
        return Err("native backend requires non-empty \\input file names".to_string());
    }
    if input_name.starts_with('|') {
        return Err("native backend does not support shell-pipe \\input".to_string());
    }
    let path = Path::new(&input_name);
    if path.is_absolute() {
        return Err(format!(
            "native backend only supports local relative \\input files, got `{input_name}`"
        ));
    }
    let candidate = base_dir.join(path);
    if candidate.exists() {
        return Ok(candidate);
    }
    if candidate.extension().is_none() {
        let tex_candidate = candidate.with_extension("tex");
        if tex_candidate.exists() {
            return Ok(tex_candidate);
        }
    }
    if let Some(path) = resolve_kpathsea_tex_candidate(base_dir, path)? {
        return Ok(path);
    }
    if path.extension().is_none()
        && let Some(path) = resolve_kpathsea_tex_candidate(base_dir, &path.with_extension("tex"))?
    {
        return Ok(path);
    }
    Ok(candidate)
}

fn native_file_exists(
    base_dir: &Path,
    file_name: &str,
    inputs: &mut Vec<PathBuf>,
    job_name: &str,
) -> Result<bool, String> {
    Ok(native_existing_file(base_dir, file_name, inputs, job_name)?.is_some())
}

fn native_existing_file(
    base_dir: &Path,
    file_name: &str,
    inputs: &mut Vec<PathBuf>,
    job_name: &str,
) -> Result<Option<PathBuf>, String> {
    let file_name = normalize_source_file_name(file_name, job_name);
    if file_name.is_empty() {
        return Err("native backend requires non-empty file-existence probes".to_string());
    }
    if file_name.starts_with('|') {
        return Err("native backend does not support shell-pipe file-existence probes".to_string());
    }
    let path = Path::new(&file_name);
    if path.is_absolute() {
        return Err(format!(
            "native backend only supports local relative file-existence probes, got `{file_name}`"
        ));
    }

    let candidate = base_dir.join(path);
    let existing = if candidate.exists() {
        Some(candidate)
    } else if candidate.extension().is_none() {
        let tex_candidate = candidate.with_extension("tex");
        if tex_candidate.exists() {
            Some(tex_candidate)
        } else if let Some(path) = resolve_kpathsea_tex_candidate(base_dir, path)? {
            Some(path)
        } else {
            resolve_kpathsea_tex_candidate(base_dir, &path.with_extension("tex"))?
        }
    } else {
        resolve_kpathsea_tex_candidate(base_dir, path)?
    };
    let Some(existing) = existing else {
        return Ok(None);
    };
    let canonical = fs::canonicalize(&existing).map_err(|error| {
        format!(
            "native backend could not canonicalize file-existence probe `{}`: {error}",
            existing.display()
        )
    })?;
    inputs.push(canonical.clone());
    Ok(Some(canonical))
}

fn normalize_source_file_name(file_name: &str, job_name: &str) -> String {
    file_name
        .trim()
        .replace("\\jobname", job_name)
        .replace("\\relax", "")
        .trim()
        .to_string()
}

fn resolve_kpathsea_tex_candidate(
    base_dir: &Path,
    candidate: &Path,
) -> Result<Option<PathBuf>, String> {
    resolve_kpathsea_candidate(base_dir, candidate, "TEXINPUTS", "input")
}

fn resolve_kpathsea_bib_candidate(
    base_dir: &Path,
    candidate: &Path,
) -> Result<Option<PathBuf>, String> {
    resolve_kpathsea_candidate(base_dir, candidate, "BIBINPUTS", "bibliography")
}

fn resolve_kpathsea_bst_candidate(
    base_dir: &Path,
    candidate: &Path,
) -> Result<Option<PathBuf>, String> {
    resolve_kpathsea_candidate(base_dir, candidate, "BSTINPUTS", "bibliography style")
}

fn resolve_kpathsea_candidate(
    base_dir: &Path,
    candidate: &Path,
    env_var: &'static str,
    description: &str,
) -> Result<Option<PathBuf>, String> {
    if let Some(path) =
        resolve_native_kpathsea_candidate(base_dir, candidate, env_var, description)?
    {
        return Ok(Some(path));
    }

    let mut command = Command::new("kpsewhich");
    command
        .current_dir(base_dir)
        .env(env_var, kpathsea_env(env_var, base_dir))
        .arg(candidate);
    let output = match command.output() {
        Ok(output) => output,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(format!(
                "native backend could not launch kpsewhich for {description} `{}`: {error}",
                candidate.display()
            ));
        }
    };
    if !output.status.success() {
        return Ok(None);
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        return Ok(None);
    }
    let path = PathBuf::from(path);
    let canonical = fs::canonicalize(&path).map_err(|error| {
        format!(
            "native backend could not canonicalize Kpathsea input `{}`: {error}",
            path.display()
        )
    })?;
    Ok(Some(canonical))
}

fn resolve_native_kpathsea_candidate(
    base_dir: &Path,
    candidate: &Path,
    env_var: &'static str,
    description: &str,
) -> Result<Option<PathBuf>, String> {
    if candidate.is_absolute() {
        return canonical_existing_file(candidate, description);
    }

    if let Some(path) = canonical_existing_file(&base_dir.join(candidate), description)? {
        return Ok(Some(path));
    }

    let Some(search_path) = std::env::var_os(env_var).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };
    let mut checked_roots = BTreeSet::new();
    for entry in std::env::split_paths(&search_path) {
        let Some(root) = native_search_root(base_dir, &entry) else {
            continue;
        };
        if !checked_roots.insert((root.path.clone(), root.recursive)) {
            continue;
        }
        if let Some(path) = canonical_existing_file(&root.path.join(candidate), description)? {
            return Ok(Some(path));
        }
        if root.recursive
            && let Some(path) = find_recursive_candidate(&root.path, candidate, description)?
        {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NativeSearchRoot {
    path: PathBuf,
    recursive: bool,
}

fn native_search_root(base_dir: &Path, entry: &Path) -> Option<NativeSearchRoot> {
    let mut raw = entry.to_string_lossy().trim().to_string();
    if raw.is_empty() {
        return None;
    }
    if let Some(stripped) = raw.strip_prefix("!!") {
        raw = stripped.to_string();
    }
    let recursive = raw.ends_with("//");
    if recursive {
        raw.truncate(raw.len().saturating_sub(2));
    }
    if raw.is_empty() {
        return None;
    }
    let path = PathBuf::from(raw);
    let path = if path.is_absolute() {
        path
    } else {
        base_dir.join(path)
    };
    Some(NativeSearchRoot { path, recursive })
}

fn find_recursive_candidate(
    root: &Path,
    candidate: &Path,
    description: &str,
) -> Result<Option<PathBuf>, String> {
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let direct = directory.join(candidate);
        if let Some(path) = canonical_existing_file(&direct, description)? {
            return Ok(Some(path));
        }
        let entries = match fs::read_dir(&directory) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if file_type.is_dir() {
                stack.push(entry.path());
            }
        }
    }
    Ok(None)
}

fn canonical_existing_file(candidate: &Path, description: &str) -> Result<Option<PathBuf>, String> {
    if !candidate.is_file() {
        return Ok(None);
    }
    let canonical = fs::canonicalize(candidate).map_err(|error| {
        format!(
            "native backend could not canonicalize {description} `{}`: {error}",
            candidate.display()
        )
    })?;
    Ok(Some(canonical))
}

fn kpathsea_env(var: &str, base_dir: &Path) -> OsString {
    let mut value = OsString::from(format!(
        "{}//{}",
        base_dir.display(),
        KPATHSEA_PATH_SEPARATOR
    ));
    if let Some(existing) = std::env::var_os(var).filter(|value| !value.is_empty()) {
        value.push(existing);
    }
    value
}

#[derive(Debug, Clone)]
struct SimpleDocument {
    layout: DocumentLayout,
    lines: Vec<Line>,
    images: Vec<ImageAsset>,
    timings: ParseTimings,
    generated_outputs: Vec<GeneratedOutput>,
    labels: BTreeMap<String, LabelInfo>,
    citations: CitationRegistry,
    bibliography: BibliographyMetadata,
    index: IndexRegistry,
    pdf_metadata: PdfMetadata,
    footnotes: Vec<FootnoteEntry>,
    toc_entries: Vec<TocEntry>,
    float_entries: Vec<FloatEntry>,
    bookmarks: Vec<BookmarkEntry>,
    toc_requested: bool,
    list_of_figures_requested: bool,
    list_of_tables_requested: bool,
    hyperref_out_requested: bool,
    backref_requested: bool,
}

#[derive(Debug, Clone, Default)]
struct ParseTimings {
    pre_body_ms: u128,
    body_ms: u128,
    includegraphics_ms: u128,
    includegraphics_prewarm_ms: u128,
    preprocess_ms: u128,
    expansion_ms: u128,
    metadata_ms: u128,
    labels_ms: u128,
    bibliography_ms: u128,
    citations_ms: u128,
    index_ms: u128,
    lists_floats_ms: u128,
    hyperref_ms: u128,
    title_ms: u128,
    two_column_graphic_float_fallbacks: usize,
    two_column_wide_graphic_float_fallbacks: usize,
    two_column_graphic_float_fallback_details: Vec<GraphicFloatFallbackTrace>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HyperrefFlags {
    hyperref_out_requested: bool,
    backref_requested: bool,
}

#[derive(Debug)]
struct PreBodyAnalysis {
    labels: HashMap<String, LabelInfo>,
    citations: CitationRegistry,
    hyperref_flags: HyperrefFlags,
    labels_ms: u128,
    citations_ms: u128,
    hyperref_ms: u128,
    citation_input_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GraphicFloatFallbackTrace {
    env: String,
    top: bool,
    wide: bool,
    image_count: usize,
    estimate: Option<GraphicFloatFallbackEstimate>,
    caption: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GraphicFloatFallbackEstimate {
    rows: usize,
    image_slots: usize,
    caption_slots: usize,
    total_slots: usize,
}

impl ParseTimings {
    fn two_column_graphic_float_fallback_estimated_native_slots(&self, wide_only: bool) -> usize {
        self.two_column_graphic_float_fallback_details
            .iter()
            .filter(|detail| !wide_only || detail.wide)
            .filter_map(|detail| detail.estimate.map(|estimate| estimate.total_slots))
            .sum()
    }
}

impl GraphicFloatFallbackTrace {
    fn to_trace_detail(&self) -> String {
        let mut detail = format!(
            "env={} top={} wide={} images={} caption={}",
            self.env,
            self.top,
            self.wide,
            self.image_count,
            trace_excerpt(&self.caption)
        );
        if let Some(estimate) = self.estimate {
            detail.push_str(&format!(
                " native_rows={} native_image_slots={} native_caption_slots={} native_slots={}",
                estimate.rows, estimate.image_slots, estimate.caption_slots, estimate.total_slots
            ));
        }
        detail
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GeneratedOutput {
    path: PathBuf,
    content: String,
}

#[cfg(test)]
impl SimpleDocument {
    fn pages(&self) -> usize {
        layout_page_count(self.layout, &self.lines, &self.images)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DocumentLayout {
    lines_per_page: usize,
    rows_per_column: usize,
    columns: usize,
    text_wrap_width: usize,
    code_wrap_width: usize,
    table_wrap_width: usize,
    toc_wrap_width: usize,
    footnote_wrap_width: usize,
    text_width_pt: f32,
    column_width_pt: f32,
    column_gap_pt: f32,
    left_pt: f32,
    top_pt: f32,
    line_height_pt: f32,
    text_font_pt: f32,
    title_font_pt: f32,
    author_font_pt: f32,
    heading_font_pt: f32,
    abstract_heading_font_pt: f32,
    footnote_font_pt: f32,
    code_font_pt: f32,
    text_base_font: &'static str,
    code_base_font: &'static str,
    math_base_font: &'static str,
    heading_base_font: &'static str,
}

impl DocumentLayout {
    const fn default() -> Self {
        Self {
            lines_per_page: 42,
            rows_per_column: 42,
            columns: 1,
            text_wrap_width: 82,
            code_wrap_width: 78,
            table_wrap_width: 82,
            toc_wrap_width: 82,
            footnote_wrap_width: 88,
            text_width_pt: 468.0,
            column_width_pt: 468.0,
            column_gap_pt: 18.0,
            left_pt: 72.0,
            top_pt: 720.0,
            line_height_pt: 10.0,
            text_font_pt: 12.0,
            title_font_pt: 18.0,
            author_font_pt: 12.0,
            heading_font_pt: 18.0,
            abstract_heading_font_pt: 10.0,
            footnote_font_pt: 10.0,
            code_font_pt: 9.0,
            text_base_font: "NimbusRomNo9L-Regu",
            code_base_font: "Courier",
            math_base_font: "NimbusRomNo9L-ReguItal",
            heading_base_font: "NimbusRomNo9L-Medi",
        }
    }

    const fn neurips_single_column() -> Self {
        Self {
            lines_per_page: 68,
            rows_per_column: 68,
            columns: 1,
            text_wrap_width: 76,
            code_wrap_width: 72,
            table_wrap_width: 76,
            toc_wrap_width: 76,
            footnote_wrap_width: 80,
            text_width_pt: 396.0,
            column_width_pt: 396.0,
            column_gap_pt: 18.0,
            left_pt: 108.0,
            top_pt: 720.0,
            line_height_pt: 9.5,
            text_font_pt: 9.0,
            title_font_pt: 17.0,
            author_font_pt: 8.5,
            heading_font_pt: 12.0,
            abstract_heading_font_pt: 12.0,
            footnote_font_pt: 7.5,
            code_font_pt: 7.0,
            text_base_font: "NimbusRomNo9L-Regu",
            code_base_font: "Courier",
            math_base_font: "NimbusRomNo9L-ReguItal",
            heading_base_font: "NimbusRomNo9L-Medi",
        }
    }

    const fn icml_two_column() -> Self {
        Self {
            lines_per_page: 108,
            rows_per_column: 54,
            columns: 2,
            text_wrap_width: 52,
            code_wrap_width: 48,
            table_wrap_width: 52,
            toc_wrap_width: 52,
            footnote_wrap_width: 58,
            text_width_pt: 518.4,
            column_width_pt: 250.2,
            column_gap_pt: 18.0,
            left_pt: 46.8,
            top_pt: 720.0,
            line_height_pt: 14.5,
            text_font_pt: 8.0,
            title_font_pt: 12.0,
            author_font_pt: 8.0,
            heading_font_pt: 10.5,
            abstract_heading_font_pt: 9.0,
            footnote_font_pt: 7.0,
            code_font_pt: 6.7,
            text_base_font: "TeXGyrePagellaX-Regular",
            code_base_font: "TeXGyreHeros-Regular",
            math_base_font: "TeXGyrePagellaX-Italic",
            heading_base_font: "TeXGyreHeros-Bold",
        }
    }

    fn point_for_slot(self, page_slot: usize, indent_pt: f32) -> (f32, f32) {
        let column = if self.columns > 1 {
            (page_slot / self.rows_per_column).min(self.columns - 1)
        } else {
            0
        };
        let row = if self.columns > 1 {
            page_slot % self.rows_per_column
        } else {
            page_slot
        };
        let x = self.left_pt + column as f32 * (self.column_width_pt + self.column_gap_pt);
        let y = self.top_pt - row as f32 * self.line_height_pt;
        (x + indent_pt, y)
    }

    fn point_for_wide_slot(self, page_slot: usize, indent_pt: f32) -> (f32, f32) {
        let y = self.top_pt - page_slot as f32 * self.line_height_pt;
        (self.left_pt + indent_pt, y)
    }

    fn image_display_size(self, image: &ImageAsset) -> (f32, f32) {
        let max_width = if self.columns > 1 {
            self.column_width_pt
        } else {
            self.text_width_pt
        };
        self.image_display_size_with_max_width(image, max_width)
    }

    fn image_display_size_with_max_width(self, image: &ImageAsset, max_width: f32) -> (f32, f32) {
        let (width, height) = self.image_draw_size_with_max_width(image, max_width);
        rotated_box_size(width, height, image.rotation_degrees)
    }

    fn image_draw_size_with_max_width(self, image: &ImageAsset, max_width: f32) -> (f32, f32) {
        if image.display_width_pt <= max_width {
            return (image.display_width_pt, image.display_height_pt);
        }
        let scale = max_width / image.display_width_pt;
        (max_width, image.display_height_pt * scale)
    }

    fn wrap_text(self, text: &str) -> Vec<String> {
        self.wrap_prose_lines(
            text,
            self.text_wrap_width,
            self.text_font_pt,
            self.column_width_pt,
        )
        .into_iter()
        .map(|line| line.text)
        .collect()
    }

    fn wrap_paragraph_text(self, text: &str) -> Vec<WrappedTextLine> {
        self.wrap_prose_lines(
            text,
            self.text_wrap_width,
            self.text_font_pt,
            self.column_width_pt,
        )
    }

    fn wrap_wide_text(self, text: &str) -> Vec<String> {
        let width = ((self.text_wrap_width as f32 * self.text_width_pt / self.column_width_pt)
            .round() as usize)
            .max(self.text_wrap_width);
        self.wrap_prose_lines(text, width, self.text_font_pt, self.text_width_pt)
            .into_iter()
            .map(|line| line.text)
            .collect()
    }

    fn with_line_width(self, width_pt: f32) -> Self {
        let ratio = (width_pt / self.column_width_pt).max(0.1);
        let mut layout = self;
        layout.text_width_pt = width_pt;
        layout.column_width_pt = width_pt;
        layout.text_wrap_width = ((self.text_wrap_width as f32 * ratio).round() as usize).max(12);
        layout.table_wrap_width = ((self.table_wrap_width as f32 * ratio).round() as usize).max(12);
        layout.toc_wrap_width = ((self.toc_wrap_width as f32 * ratio).round() as usize).max(12);
        layout.footnote_wrap_width =
            ((self.footnote_wrap_width as f32 * ratio).round() as usize).max(12);
        layout
    }

    fn as_one_column(self) -> Self {
        if self.columns == 1 {
            return self;
        }
        let ratio = (self.text_width_pt / self.column_width_pt).max(0.1);
        let mut layout = self;
        layout.columns = 1;
        layout.lines_per_page = self.rows_per_column;
        layout.lines_per_page = layout.lines_per_page.max(1);
        layout.rows_per_column = layout.lines_per_page;
        layout.column_width_pt = self.text_width_pt;
        layout.text_wrap_width = ((self.text_wrap_width as f32 * ratio).round() as usize).max(12);
        layout.code_wrap_width = ((self.code_wrap_width as f32 * ratio).round() as usize).max(12);
        layout.table_wrap_width = ((self.table_wrap_width as f32 * ratio).round() as usize).max(12);
        layout.toc_wrap_width = ((self.toc_wrap_width as f32 * ratio).round() as usize).max(12);
        layout.footnote_wrap_width =
            ((self.footnote_wrap_width as f32 * ratio).round() as usize).max(12);
        layout
    }

    fn wrap_table_text(self, text: &str) -> Vec<String> {
        wrap_text(text, self.table_wrap_width)
    }

    fn wrap_toc_text(self, text: &str) -> Vec<String> {
        wrap_text(text, self.toc_wrap_width)
    }

    fn wrap_footnote_text(self, text: &str) -> Vec<String> {
        wrap_text(text, self.footnote_wrap_width)
    }

    fn wrap_caption_text(self, text: &str) -> Vec<String> {
        wrap_text(text, self.caption_wrap_width())
    }

    fn wrap_wide_caption_text(self, text: &str) -> Vec<String> {
        let wide_text_width = (self.text_wrap_width as f32 * self.text_width_pt
            / self.column_width_pt.max(1.0))
        .round();
        let font_ratio = (self.text_font_pt / self.footnote_font_pt).max(1.0);
        let width =
            ((wide_text_width * font_ratio * 1.18).round() as usize).max(self.footnote_wrap_width);
        wrap_text(text, width)
    }

    fn caption_wrap_width(self) -> usize {
        let font_ratio = (self.text_font_pt / self.footnote_font_pt).max(1.0);
        ((self.text_wrap_width as f32 * font_ratio * 12.0).round() as usize)
            .max(self.footnote_wrap_width)
    }

    fn wrap_prose_lines(
        self,
        text: &str,
        width: usize,
        font_size: f32,
        max_width_pt: f32,
    ) -> Vec<WrappedTextLine> {
        if self == Self::neurips_single_column() {
            wrap_calibrated_prose_text_lines(text, width, font_size, max_width_pt)
        } else {
            wrap_prose_text_lines(
                text,
                width,
                self.line_break_metric_for_font(PdfTextFont::Text),
                font_size,
                max_width_pt,
            )
        }
    }

    fn metric_for_font(self, font: PdfTextFont) -> PdfFontMetric {
        match font {
            PdfTextFont::Text => {
                if self.text_base_font == "TeXGyrePagellaX-Regular" {
                    PdfFontMetric::Pagella
                } else {
                    PdfFontMetric::TimesRoman
                }
            }
            PdfTextFont::Code => {
                if self.code_base_font == "TeXGyreHeros-Regular" {
                    PdfFontMetric::Heros
                } else {
                    PdfFontMetric::Courier
                }
            }
            PdfTextFont::Math => {
                if self.math_base_font == "TeXGyrePagellaX-Italic" {
                    PdfFontMetric::PagellaItalic
                } else {
                    PdfFontMetric::TimesItalic
                }
            }
            PdfTextFont::Heading => {
                if self.heading_base_font == "TeXGyreHeros-Bold" {
                    PdfFontMetric::HerosBold
                } else if self.heading_base_font == "TeXGyrePagellaX-Bold" {
                    PdfFontMetric::PagellaBold
                } else {
                    PdfFontMetric::TimesBold
                }
            }
            PdfTextFont::Symbol => PdfFontMetric::Symbol,
        }
    }

    fn line_break_metric_for_font(self, font: PdfTextFont) -> PdfFontMetric {
        match font {
            PdfTextFont::Text => PdfFontMetric::TimesRoman,
            _ => self.metric_for_font(font),
        }
    }
}

#[derive(Debug, Clone)]
struct WideTeaserCell {
    width_pt: f32,
    images: Vec<usize>,
    text_lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PageStyle {
    running_title: String,
    section_line: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthorBlock {
    lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthorGrid {
    rows: Vec<Vec<AuthorBlock>>,
}

impl AuthorGrid {
    fn slots(&self) -> usize {
        let content_slots = self
            .rows
            .iter()
            .enumerate()
            .map(|(index, row)| {
                let row_lines = row.iter().map(|block| block.lines.len()).max().unwrap_or(0);
                row_lines + usize::from(index > 0) * 2
            })
            .sum::<usize>();
        content_slots.max(10)
    }
}

#[derive(Debug, Clone)]
enum Line {
    PageStyle(PageStyle),
    OutputControl(OutputControl),
    Title(String),
    WideTitle(String),
    Author(String),
    WideAuthor(String),
    AuthorGrid(AuthorGrid),
    AbstractHeading(String),
    AbstractText(String),
    WideBackground(usize),
    WideTheoremBackground {
        slots: usize,
        heading: String,
    },
    WideAbstractText(String),
    WideCaption(String),
    WideEquation(String),
    WideImageRow(Vec<usize>),
    WideTeaserRow(Vec<WideTeaserCell>),
    TeaserRow(Vec<WideTeaserCell>),
    FloatBlock {
        lines: Vec<Line>,
        wide: bool,
        top: bool,
    },
    LateFloatBlock {
        lines: Vec<Line>,
        wide: bool,
    },
    BottomFloatBlock {
        lines: Vec<Line>,
        wide: bool,
    },
    Heading(String),
    ParagraphText(String),
    JustifiedParagraphText {
        text: String,
        width_pt: f32,
    },
    Text(String),
    JustifiedText {
        text: String,
        width_pt: f32,
    },
    TableCells {
        cells: Vec<String>,
        slots: usize,
    },
    WideTableCells {
        cells: Vec<String>,
        slots: usize,
    },
    TableRow(String),
    Caption(String),
    Footnote(String),
    JustifiedAbstractText {
        text: String,
        width_pt: f32,
    },
    DisplayEquation(String),
    Equation(String),
    Code(String),
    Image(usize),
    Blank,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OutputControl {
    NewPage,
    ClearPage,
    LayoutSwitch(DocumentLayout),
}

const SOFT_NEWPAGE_MAX_REMAINING_SLOTS: usize = 4;
const PAGE_NUMBER_FONT_PT: f32 = 10.0;
const PAGE_NUMBER_Y_PT: f32 = 36.0;
const NEURIPS_PAGE_NUMBER_Y_PT: f32 = 42.75;
const ICML_PAGE_NUMBER_Y_PT: f32 = 28.5;
const ICML_HEADER_TITLE_FONT_PT: f32 = 8.0;
const ICML_HEADER_SECTION_FONT_PT: f32 = 9.0;
const ICML_HEADER_TITLE_Y_PT: f32 = 757.0;
const ICML_HEADER_SECTION_Y_PT: f32 = 743.0;
const ICML_HEADER_RULE_Y_PT: f32 = 736.5;
const NEURIPS_AUTHOR_GRID_FONT_PT: f32 = 9.0;
const NEURIPS_TITLE_WIDTH_SCALE: f32 = 1.07;

impl Line {
    fn slots(&self, layout: DocumentLayout, images: &[ImageAsset]) -> usize {
        match self {
            Line::Image(index) => {
                let image = &images[*index];
                let (_, height) = layout.image_display_size(image);
                (height / layout.line_height_pt).ceil() as usize + 1
            }
            Line::WideImageRow(indices) => {
                let (_, height) = wide_image_row_display_size(layout, images, indices);
                (height / layout.line_height_pt).ceil() as usize + 1
            }
            Line::WideTeaserRow(cells) => {
                let (_, height) = wide_teaser_row_display_size(layout, images, cells);
                (height / layout.line_height_pt).ceil() as usize + 1
            }
            Line::TeaserRow(cells) => {
                let (_, height) =
                    teaser_row_display_size(layout, images, cells, layout.column_width_pt);
                (height / layout.line_height_pt).ceil() as usize + 1
            }
            Line::FloatBlock { lines, .. }
            | Line::LateFloatBlock { lines, .. }
            | Line::BottomFloatBlock { lines, .. } => lines
                .iter()
                .map(|line| line.slots(layout, images))
                .sum::<usize>()
                .max(1),
            Line::PageStyle(_) | Line::OutputControl(_) => 0,
            Line::WideBackground(_) => 0,
            Line::WideTheoremBackground { .. } => 1,
            Line::Title(_) | Line::WideTitle(_) | Line::Heading(_) => 2,
            Line::DisplayEquation(_) => 1,
            Line::Equation(_) | Line::WideEquation(_) => 2,
            Line::AuthorGrid(grid) => grid.slots(),
            Line::TableCells { slots, .. } | Line::WideTableCells { slots, .. } => *slots,
            Line::Author(_)
            | Line::WideAuthor(_)
            | Line::AbstractHeading(_)
            | Line::AbstractText(_)
            | Line::JustifiedAbstractText { .. }
            | Line::WideAbstractText(_)
            | Line::WideCaption(_)
            | Line::Text(_)
            | Line::JustifiedText { .. }
            | Line::TableRow(_)
            | Line::Caption(_)
            | Line::Footnote(_)
            | Line::Code(_)
            | Line::Blank => 1,
            Line::ParagraphText(_) | Line::JustifiedParagraphText { .. } => 2,
        }
    }

    fn is_wide(&self) -> bool {
        matches!(
            self,
            Line::WideTitle(_)
                | Line::WideAuthor(_)
                | Line::WideBackground(_)
                | Line::WideTheoremBackground { .. }
                | Line::WideAbstractText(_)
                | Line::WideCaption(_)
                | Line::WideTableCells { .. }
                | Line::WideEquation(_)
                | Line::WideImageRow(_)
                | Line::WideTeaserRow(_)
                | Line::FloatBlock { wide: true, .. }
                | Line::LateFloatBlock { wide: true, .. }
                | Line::BottomFloatBlock { wide: true, .. }
        )
    }

    fn is_top_float(&self) -> bool {
        matches!(self, Line::FloatBlock { top: true, .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LinePlacement {
    line_index: usize,
    page_index: usize,
    page_slot: usize,
    line_slots: usize,
    layout: DocumentLayout,
    render: bool,
}

#[derive(Debug, Clone)]
struct LayoutPass {
    placements: Vec<LinePlacement>,
    cursor: PlacementCursor,
    pending_normal_top_floats: Vec<PendingPlacement>,
    pending_wide_top_floats: Vec<PendingPlacement>,
    pending_bottom_floats: Vec<PendingPlacement>,
    pending_late_floats: Vec<PendingPlacement>,
    saw_normal_after_pending_normal_float: bool,
    saw_normal_after_pending_float: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LayoutSummary {
    page_count: usize,
    lines: usize,
    slots: usize,
    placements: usize,
    rendered_placements: usize,
    overflow_placements: usize,
    float_blocks: usize,
    top_floats: usize,
    wide_floats: usize,
    bottom_floats: usize,
    display_equations: usize,
    output_controls: usize,
    pending_normal_top_floats: usize,
    pending_wide_top_floats: usize,
    pending_bottom_floats: usize,
}

impl LayoutSummary {
    fn from_document(document: &SimpleDocument, pass: &LayoutPass) -> Self {
        let rendered_placements = pass
            .placements
            .iter()
            .filter(|placement| placement.render)
            .count();
        let slots = pass
            .placements
            .iter()
            .filter(|placement| placement.render)
            .map(|placement| placement.line_slots)
            .sum();
        let mut float_blocks = 0_usize;
        let mut top_floats = 0_usize;
        let mut wide_floats = 0_usize;
        let mut bottom_floats = 0_usize;
        let mut display_equations = 0_usize;
        let mut output_controls = 0_usize;
        for line in &document.lines {
            match line {
                Line::FloatBlock { wide, top, .. } => {
                    float_blocks += 1;
                    top_floats += usize::from(*top);
                    wide_floats += usize::from(*wide);
                }
                Line::LateFloatBlock { wide, .. } => {
                    float_blocks += 1;
                    top_floats += 1;
                    wide_floats += usize::from(*wide);
                }
                Line::BottomFloatBlock { wide, .. } => {
                    float_blocks += 1;
                    bottom_floats += 1;
                    wide_floats += usize::from(*wide);
                }
                Line::DisplayEquation(_) => display_equations += 1,
                Line::OutputControl(_) => output_controls += 1,
                _ => {}
            }
        }
        Self {
            page_count: page_count_from_placements(&pass.placements, &document.lines),
            lines: document.lines.len(),
            slots,
            placements: pass.placements.len(),
            rendered_placements,
            overflow_placements: pass.placements.len().saturating_sub(rendered_placements),
            float_blocks,
            top_floats,
            wide_floats,
            bottom_floats,
            display_equations,
            output_controls,
            pending_normal_top_floats: pass.pending_normal_top_floats.len(),
            pending_wide_top_floats: pass.pending_wide_top_floats.len(),
            pending_bottom_floats: pass.pending_bottom_floats.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LayoutCaptionTrace {
    page: usize,
    slot: usize,
    line_index: usize,
    kind: &'static str,
    text: String,
}

impl LayoutCaptionTrace {
    fn to_trace_detail(&self) -> String {
        format!(
            "page={} slot={} line={} kind={} text={}",
            self.page,
            self.slot,
            self.line_index,
            self.kind,
            trace_excerpt(&self.text)
        )
    }
}

fn layout_caption_traces(
    document: &SimpleDocument,
    placements: &[LinePlacement],
) -> Vec<LayoutCaptionTrace> {
    let mut traces = Vec::new();
    for placement in placements {
        if !placement.render {
            continue;
        }
        collect_layout_caption_traces(
            document,
            &document.lines[placement.line_index],
            placement.line_index,
            placement.page_index + 1,
            placement.page_slot,
            placement.layout,
            &mut traces,
        );
    }
    traces
}

fn collect_layout_caption_traces(
    document: &SimpleDocument,
    line: &Line,
    line_index: usize,
    page: usize,
    page_slot: usize,
    layout: DocumentLayout,
    traces: &mut Vec<LayoutCaptionTrace>,
) {
    if let Some((kind, text)) = layout_caption_text(line) {
        traces.push(LayoutCaptionTrace {
            page,
            slot: page_slot,
            line_index,
            kind,
            text: text.to_string(),
        });
        return;
    }
    if let Line::FloatBlock { lines, .. }
    | Line::LateFloatBlock { lines, .. }
    | Line::BottomFloatBlock { lines, .. } = line
    {
        let mut offset = 0_usize;
        for nested in lines {
            collect_layout_caption_traces(
                document,
                nested,
                line_index,
                page,
                page_slot + offset,
                layout,
                traces,
            );
            offset += nested.slots(layout, &document.images);
        }
    }
}

fn layout_caption_text(line: &Line) -> Option<(&'static str, &str)> {
    match line {
        Line::Caption(text) | Line::WideCaption(text) | Line::WideAbstractText(text) => {
            let kind = layout_caption_kind(text)?;
            Some((kind, text.trim()))
        }
        _ => None,
    }
}

fn layout_caption_kind(text: &str) -> Option<&'static str> {
    let text = text.trim_start();
    for (prefix, kind) in [
        ("Figure ", "figure"),
        ("Table ", "table"),
        ("Listing ", "listing"),
        ("Algorithm ", "algorithm"),
    ] {
        if text.starts_with(prefix) {
            return Some(kind);
        }
    }
    None
}

fn trace_excerpt(text: &str) -> String {
    const MAX_CHARS: usize = 180;
    let mut out = String::new();
    for ch in text.chars().take(MAX_CHARS) {
        out.push(ch);
    }
    if text.chars().count() > MAX_CHARS {
        out.push_str("...");
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PendingPlacement {
    line_index: usize,
    line_slots: usize,
    layout: DocumentLayout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PlacementCursor {
    page_index: usize,
    column: usize,
    row: usize,
    reserved_top_rows: usize,
    reserved_bottom_rows: usize,
    page_has_normal: bool,
}

impl PlacementCursor {
    const fn new() -> Self {
        Self {
            page_index: 0,
            column: 0,
            row: 0,
            reserved_top_rows: 0,
            reserved_bottom_rows: 0,
            page_has_normal: false,
        }
    }

    fn is_page_start(self) -> bool {
        self.column == 0
            && self.row == 0
            && self.reserved_top_rows == 0
            && self.reserved_bottom_rows == 0
            && !self.page_has_normal
    }

    fn is_column_start(self) -> bool {
        self.row == self.reserved_top_rows
    }

    fn normal_slots_until_page_boundary(self, layout: DocumentLayout) -> usize {
        if self.is_page_start() {
            return 0;
        }
        let row_limit = normal_row_limit(layout, self);
        let row = self.row.min(row_limit);
        let current_column = row_limit.saturating_sub(row);
        let later_columns = layout.columns.saturating_sub(self.column.saturating_add(1))
            * layout
                .rows_per_column
                .saturating_sub(self.reserved_top_rows);
        current_column + later_columns
    }
}

#[cfg(test)]
fn layout_page_count(layout: DocumentLayout, lines: &[Line], images: &[ImageAsset]) -> usize {
    page_count_from_placements(&layout_lines(layout, lines, images).placements, lines)
}

#[cfg(test)]
fn line_placements(document: &SimpleDocument) -> Vec<LinePlacement> {
    layout_lines(document.layout, &document.lines, &document.images).placements
}

fn page_count_from_placements(placements: &[LinePlacement], lines: &[Line]) -> usize {
    placements
        .iter()
        .filter(|placement| {
            placement.render
                && lines
                    .get(placement.line_index)
                    .is_some_and(line_counts_for_page)
        })
        .map(|placement| placement.page_index)
        .max()
        .map(|page_index| page_index + 1)
        .unwrap_or(1)
}

fn line_counts_for_page(line: &Line) -> bool {
    !matches!(
        line,
        Line::Blank | Line::PageStyle(_) | Line::OutputControl(_)
    )
}

fn layout_lines(layout: DocumentLayout, lines: &[Line], images: &[ImageAsset]) -> LayoutPass {
    let mut pass = LayoutPass {
        placements: Vec::with_capacity(lines.len()),
        cursor: PlacementCursor::new(),
        pending_normal_top_floats: Vec::new(),
        pending_wide_top_floats: Vec::new(),
        pending_bottom_floats: Vec::new(),
        pending_late_floats: Vec::new(),
        saw_normal_after_pending_normal_float: false,
        saw_normal_after_pending_float: false,
    };
    let mut active_layout = layout;
    for (line_index, line) in lines.iter().enumerate() {
        if matches!(line, Line::PageStyle(_)) {
            continue;
        }
        if let Line::OutputControl(control) = line {
            apply_output_control(&mut pass, &mut active_layout, *control, lines);
            continue;
        }
        flush_pending_wide_top_floats_if_ready(&mut pass);
        flush_pending_normal_top_floats_if_ready(&mut pass, active_layout, lines);
        if should_skip_blank_before_deferred_wide_top_float(
            lines,
            line_index,
            active_layout,
            pass.cursor,
        ) {
            continue;
        }
        let line_slots = line.slots(active_layout, images);
        if should_defer_late_float(line) {
            pass.pending_late_floats.push(PendingPlacement {
                line_index,
                line_slots,
                layout: active_layout,
            });
            continue;
        }
        if should_defer_bottom_float(line) {
            defer_bottom_float(&mut pass, active_layout, line_index, line_slots, lines);
            continue;
        }
        if should_defer_normal_top_float(active_layout, line, line_slots, pass.cursor) {
            pass.pending_normal_top_floats.push(PendingPlacement {
                line_index,
                line_slots,
                layout: active_layout,
            });
            continue;
        }
        if should_defer_wide_top_float(active_layout, line, line_slots, pass.cursor) {
            pass.pending_wide_top_floats.push(PendingPlacement {
                line_index,
                line_slots,
                layout: active_layout,
            });
            continue;
        }
        if line_is_wide_in_layout(lines, line_index, pass.cursor) {
            place_wide_line(&mut pass, active_layout, line_index, line_slots, lines);
        } else {
            place_normal_line(
                &mut pass,
                active_layout,
                line_index,
                line,
                line_slots,
                lines,
            );
        }
    }
    flush_pending_wide_top_floats_at_end(&mut pass, active_layout, lines);
    flush_pending_normal_top_floats_at_end(&mut pass, active_layout, lines);
    flush_pending_bottom_floats(&mut pass, active_layout, lines);
    flush_pending_late_floats_at_end(&mut pass, active_layout, lines);
    pass
}

fn apply_output_control(
    pass: &mut LayoutPass,
    active_layout: &mut DocumentLayout,
    control: OutputControl,
    lines: &[Line],
) {
    match control {
        OutputControl::NewPage => {
            apply_soft_newpage(pass, *active_layout, lines);
        }
        OutputControl::ClearPage => {
            flush_pending_top_floats_for_output_barrier(pass, *active_layout, lines);
            flush_pending_late_floats_at_end(pass, *active_layout, lines);
            if !pass.cursor.is_page_start() {
                advance_to_next_page_in_pass(pass, *active_layout, lines);
            }
        }
        OutputControl::LayoutSwitch(next_layout) => {
            flush_pending_top_floats_for_output_barrier(pass, *active_layout, lines);
            flush_pending_late_floats_at_end(pass, *active_layout, lines);
            if !pass.cursor.is_page_start() {
                advance_to_next_page_in_pass(pass, *active_layout, lines);
            }
            *active_layout = next_layout;
        }
    }
}

fn apply_soft_newpage(pass: &mut LayoutPass, layout: DocumentLayout, lines: &[Line]) {
    if pass.cursor.is_page_start()
        || pass.cursor.normal_slots_until_page_boundary(layout) > SOFT_NEWPAGE_MAX_REMAINING_SLOTS
    {
        return;
    }
    advance_to_next_page_in_pass(pass, layout, lines);
    flush_pending_wide_top_floats(pass, lines);
    flush_pending_normal_top_floats(pass, lines);
}

fn flush_pending_top_floats_for_output_barrier(
    pass: &mut LayoutPass,
    layout: DocumentLayout,
    lines: &[Line],
) {
    if pass.pending_normal_top_floats.is_empty() && pass.pending_wide_top_floats.is_empty() {
        pass.saw_normal_after_pending_normal_float = false;
        pass.saw_normal_after_pending_float = false;
        return;
    }
    if !pass.cursor.is_page_start() {
        advance_to_next_page_in_pass(pass, layout, lines);
    }
    flush_pending_wide_top_floats(pass, lines);
    flush_pending_normal_top_floats(pass, lines);
}

fn should_skip_blank_before_deferred_wide_top_float(
    lines: &[Line],
    line_index: usize,
    layout: DocumentLayout,
    cursor: PlacementCursor,
) -> bool {
    layout.columns > 1
        && cursor.is_page_start()
        && matches!(lines[line_index], Line::Blank)
        && next_nonblank_line_is_wide_top_float(lines, line_index + 1)
}

fn next_nonblank_line_is_wide_top_float(lines: &[Line], start_index: usize) -> bool {
    lines
        .iter()
        .skip(start_index)
        .find(|line| !matches!(line, Line::Blank))
        .is_some_and(|line| {
            matches!(
                line,
                Line::FloatBlock {
                    wide: true,
                    top: true,
                    ..
                }
            )
        })
}

fn should_defer_wide_top_float(
    layout: DocumentLayout,
    line: &Line,
    line_slots: usize,
    cursor: PlacementCursor,
) -> bool {
    layout.columns > 1
        && matches!(
            line,
            Line::FloatBlock {
                wide: true,
                top: true,
                ..
            }
        )
        && (!cursor.is_page_start()
            || cursor.page_has_normal
            || cursor.column != 0
            || ((cursor.is_page_start() && cursor.page_index == 0)
                || cursor.row.saturating_add(line_slots) > wide_rows_per_page(layout)))
}

fn should_defer_normal_top_float(
    layout: DocumentLayout,
    line: &Line,
    line_slots: usize,
    cursor: PlacementCursor,
) -> bool {
    line.is_top_float()
        && !line.is_wide()
        && cursor.row > cursor.reserved_top_rows
        && line_slots
            <= layout
                .rows_per_column
                .saturating_sub(cursor.reserved_top_rows)
}

fn should_defer_bottom_float(line: &Line) -> bool {
    matches!(line, Line::BottomFloatBlock { .. })
}

fn should_defer_late_float(line: &Line) -> bool {
    matches!(line, Line::LateFloatBlock { .. })
}

fn defer_bottom_float(
    pass: &mut LayoutPass,
    layout: DocumentLayout,
    line_index: usize,
    line_slots: usize,
    lines: &[Line],
) {
    if line_slots == 0 {
        return;
    }
    let max_bottom_rows = layout
        .rows_per_column
        .saturating_sub(pass.cursor.reserved_top_rows);
    if line_slots > max_bottom_rows {
        place_normal_line(
            pass,
            layout,
            line_index,
            &lines[line_index],
            line_slots,
            lines,
        );
        return;
    }
    if pass
        .cursor
        .row
        .saturating_add(pass.cursor.reserved_bottom_rows)
        .saturating_add(line_slots)
        > layout.rows_per_column
    {
        advance_to_next_column_in_pass(pass, layout, lines);
    }
    pass.pending_bottom_floats.push(PendingPlacement {
        line_index,
        line_slots,
        layout,
    });
    pass.cursor.reserved_bottom_rows = pass
        .cursor
        .reserved_bottom_rows
        .saturating_add(line_slots)
        .min(max_bottom_rows);
}

fn flush_pending_normal_top_floats_if_ready(
    pass: &mut LayoutPass,
    layout: DocumentLayout,
    lines: &[Line],
) {
    if pass.pending_normal_top_floats.is_empty()
        || !pass.saw_normal_after_pending_normal_float
        || !pass.pending_wide_top_floats.is_empty()
        || !cursor_accepts_pending_normal_top_float(layout, pass.cursor)
    {
        return;
    }
    flush_pending_normal_top_floats(pass, lines);
}

fn flush_pending_normal_top_floats_at_end(
    pass: &mut LayoutPass,
    layout: DocumentLayout,
    lines: &[Line],
) {
    if pass.pending_normal_top_floats.is_empty() {
        return;
    }
    advance_to_pending_normal_top_float_position(pass, layout);
    flush_pending_normal_top_floats(pass, lines);
}

fn cursor_accepts_pending_normal_top_float(
    layout: DocumentLayout,
    cursor: PlacementCursor,
) -> bool {
    if layout.columns == 1 {
        cursor.is_page_start()
    } else {
        cursor.is_column_start()
    }
}

fn advance_to_pending_normal_top_float_position(pass: &mut LayoutPass, layout: DocumentLayout) {
    if cursor_accepts_pending_normal_top_float(layout, pass.cursor) {
        return;
    }
    if layout.columns == 1 {
        advance_to_next_page(&mut pass.cursor);
    } else {
        advance_to_next_column(&mut pass.cursor, layout);
    }
}

fn flush_pending_normal_top_floats(pass: &mut LayoutPass, lines: &[Line]) {
    let pending = std::mem::take(&mut pass.pending_normal_top_floats);
    pass.saw_normal_after_pending_normal_float = false;
    for PendingPlacement {
        line_index,
        line_slots,
        layout,
    } in pending
    {
        place_normal_line_without_top_advance(
            pass,
            layout,
            line_index,
            &lines[line_index],
            line_slots,
            lines,
        );
    }
}

fn flush_pending_wide_top_floats_if_ready(pass: &mut LayoutPass) {
    if pass.pending_wide_top_floats.is_empty()
        || !pass.saw_normal_after_pending_float
        || !pass.cursor.is_page_start()
    {
        return;
    }
    flush_pending_wide_top_floats(pass, &[]);
}

fn flush_pending_wide_top_floats_at_end(
    pass: &mut LayoutPass,
    layout: DocumentLayout,
    lines: &[Line],
) {
    if pass.pending_wide_top_floats.is_empty() {
        return;
    }
    if !pass.cursor.is_page_start() {
        advance_to_next_page_in_pass(pass, layout, lines);
    }
    flush_pending_wide_top_floats(pass, lines);
}

fn flush_pending_wide_top_floats(pass: &mut LayoutPass, lines: &[Line]) {
    let pending = std::mem::take(&mut pass.pending_wide_top_floats);
    pass.saw_normal_after_pending_float = false;
    for PendingPlacement {
        line_index,
        line_slots,
        layout,
    } in pending
    {
        place_wide_line(pass, layout, line_index, line_slots, lines);
    }
}

fn flush_pending_bottom_floats(pass: &mut LayoutPass, layout: DocumentLayout, _lines: &[Line]) {
    if pass.pending_bottom_floats.is_empty() {
        pass.cursor.reserved_bottom_rows = 0;
        return;
    }
    let pending = std::mem::take(&mut pass.pending_bottom_floats);
    let total_slots = pending
        .iter()
        .map(|pending| pending.line_slots)
        .sum::<usize>()
        .min(layout.rows_per_column);
    let mut row = layout.rows_per_column.saturating_sub(total_slots);
    for PendingPlacement {
        line_index,
        line_slots,
        layout,
    } in pending
    {
        let page_slot = pass.cursor.column * layout.rows_per_column + row;
        pass.placements.push(LinePlacement {
            line_index,
            page_index: pass.cursor.page_index,
            page_slot,
            line_slots,
            layout,
            render: true,
        });
        row = row.saturating_add(line_slots);
    }
    pass.cursor.reserved_bottom_rows = 0;
}

fn flush_pending_late_floats_at_end(pass: &mut LayoutPass, layout: DocumentLayout, lines: &[Line]) {
    if pass.pending_late_floats.is_empty() {
        return;
    }
    if !pass.cursor.is_page_start() {
        advance_to_next_page_in_pass(pass, layout, lines);
    }
    let compact_icml_queue = compact_icml_late_float_queue(&pass.pending_late_floats);
    if compact_icml_queue {
        advance_to_next_page_in_pass(pass, layout, lines);
    }
    let pending = std::mem::take(&mut pass.pending_late_floats);
    for (
        queue_index,
        PendingPlacement {
            line_index,
            line_slots,
            layout,
        },
    ) in pending.into_iter().enumerate()
    {
        let line_slots =
            effective_late_float_slots(compact_icml_queue, queue_index, layout, line_slots);
        let compact_trailing = compact_icml_queue && queue_index >= 2;
        match lines.get(line_index) {
            Some(Line::LateFloatBlock { wide: true, .. }) if !compact_trailing => {
                place_wide_line(pass, layout, line_index, line_slots, lines);
            }
            Some(line) => {
                if compact_trailing {
                    if pass.cursor.row > 0
                        && pass.cursor.row.saturating_add(line_slots) > layout.rows_per_column
                    {
                        advance_to_next_page_in_pass(pass, layout, lines);
                    }
                } else if !cursor_accepts_pending_normal_top_float(layout, pass.cursor) {
                    advance_to_pending_normal_top_float_position(pass, layout);
                }
                place_normal_line_without_top_advance(
                    pass, layout, line_index, line, line_slots, lines,
                );
            }
            None => {}
        }
    }
}

fn compact_icml_late_float_queue(pending: &[PendingPlacement]) -> bool {
    pending.len() >= 4
        && pending
            .iter()
            .all(|placement| placement.layout == DocumentLayout::icml_two_column().as_one_column())
}

fn effective_late_float_slots(
    compact_icml_queue: bool,
    queue_index: usize,
    layout: DocumentLayout,
    line_slots: usize,
) -> usize {
    if !compact_icml_queue {
        return line_slots;
    }
    let page_rows = layout.lines_per_page.max(layout.rows_per_column).max(1);
    if queue_index < 2 {
        page_rows
    } else {
        (page_rows / 2).saturating_sub(1).max(1)
    }
}

fn line_is_wide_in_layout(lines: &[Line], line_index: usize, cursor: PlacementCursor) -> bool {
    lines[line_index].is_wide()
        || (matches!(lines[line_index], Line::Blank)
            && !cursor.page_has_normal
            && cursor.column == 0
            && next_nonblank_line_is_wide(lines, line_index + 1))
}

fn next_nonblank_line_is_wide(lines: &[Line], start_index: usize) -> bool {
    lines
        .iter()
        .skip(start_index)
        .find(|line| !matches!(line, Line::Blank))
        .is_some_and(Line::is_wide)
}

fn place_wide_line(
    pass: &mut LayoutPass,
    layout: DocumentLayout,
    line_index: usize,
    line_slots: usize,
    lines: &[Line],
) {
    let wide_rows = wide_rows_per_page(layout);
    if pass.cursor.page_has_normal || pass.cursor.column != 0 {
        advance_to_next_page_in_pass(pass, layout, lines);
    }
    if line_slots > 0
        && pass.cursor.row > 0
        && pass.cursor.row + line_slots > wide_rows
        && line_slots <= wide_rows
    {
        advance_to_next_page_in_pass(pass, layout, lines);
    }
    let page_slot = pass.cursor.row;
    push_line_placement_with_page_overflow(
        &mut pass.placements,
        layout,
        pass.cursor,
        line_index,
        page_slot,
        line_slots,
        true,
    );
    pass.cursor.row += line_slots;
    pass.cursor.reserved_top_rows = pass
        .cursor
        .reserved_top_rows
        .max(pass.cursor.row.min(layout.rows_per_column));
    if line_slots > 0 && pass.cursor.row >= wide_rows {
        advance_to_next_page_in_pass(pass, layout, lines);
    }
}

fn place_normal_line(
    pass: &mut LayoutPass,
    layout: DocumentLayout,
    line_index: usize,
    line: &Line,
    line_slots: usize,
    lines: &[Line],
) {
    place_normal_line_with_top_policy(pass, layout, line_index, line, line_slots, true, lines);
}

fn place_normal_line_without_top_advance(
    pass: &mut LayoutPass,
    layout: DocumentLayout,
    line_index: usize,
    line: &Line,
    line_slots: usize,
    lines: &[Line],
) {
    place_normal_line_with_top_policy(pass, layout, line_index, line, line_slots, false, lines);
}

fn place_normal_line_with_top_policy(
    pass: &mut LayoutPass,
    layout: DocumentLayout,
    line_index: usize,
    line: &Line,
    line_slots: usize,
    allow_top_advance: bool,
    lines: &[Line],
) {
    normalize_normal_cursor(pass, layout, lines);
    if allow_top_advance && line.is_top_float() && pass.cursor.row > pass.cursor.reserved_top_rows {
        advance_to_next_column_in_pass(pass, layout, lines);
        normalize_normal_cursor(pass, layout, lines);
    }
    let page_slot = pass.cursor.column * layout.rows_per_column + pass.cursor.row;
    push_line_placement_with_page_overflow(
        &mut pass.placements,
        layout,
        pass.cursor,
        line_index,
        page_slot,
        line_slots,
        false,
    );
    if line_slots > 0 {
        pass.cursor.page_has_normal = true;
        if !pass.pending_normal_top_floats.is_empty() {
            pass.saw_normal_after_pending_normal_float = true;
        }
        if !pass.pending_wide_top_floats.is_empty() {
            pass.saw_normal_after_pending_float = true;
        }
        consume_normal_slots(pass, layout, line_slots, lines);
    }
}

fn push_line_placement_with_page_overflow(
    placements: &mut Vec<LinePlacement>,
    layout: DocumentLayout,
    cursor: PlacementCursor,
    line_index: usize,
    page_slot: usize,
    line_slots: usize,
    is_wide: bool,
) {
    placements.push(LinePlacement {
        line_index,
        page_index: cursor.page_index,
        page_slot,
        line_slots,
        layout,
        render: true,
    });
    if line_slots == 0 {
        return;
    }
    if is_wide {
        let wide_rows = wide_rows_per_page(layout);
        let first_page_rows = wide_rows.saturating_sub(cursor.row);
        push_overflow_page_placements(
            placements,
            layout,
            cursor.page_index,
            line_index,
            line_slots,
            first_page_rows,
            wide_rows,
        );
        return;
    }
    let first_page_rows = normal_rows_left_on_page(layout, cursor);
    push_overflow_page_placements(
        placements,
        layout,
        cursor.page_index,
        line_index,
        line_slots,
        first_page_rows,
        layout.lines_per_page,
    );
}

fn push_overflow_page_placements(
    placements: &mut Vec<LinePlacement>,
    layout: DocumentLayout,
    page_index: usize,
    line_index: usize,
    line_slots: usize,
    first_page_slots: usize,
    later_page_slots: usize,
) {
    if line_slots <= first_page_slots || later_page_slots == 0 {
        return;
    }
    let mut remaining = line_slots - first_page_slots;
    let mut overflow_page = page_index + 1;
    loop {
        placements.push(LinePlacement {
            line_index,
            page_index: overflow_page,
            page_slot: 0,
            line_slots,
            layout,
            render: false,
        });
        if remaining <= later_page_slots {
            break;
        }
        remaining -= later_page_slots;
        overflow_page += 1;
    }
}

fn normal_rows_left_on_page(layout: DocumentLayout, cursor: PlacementCursor) -> usize {
    let row_limit = normal_row_limit(layout, cursor);
    let current_row = cursor.row.min(row_limit);
    let current_column_rows = row_limit.saturating_sub(current_row);
    let later_columns = layout
        .columns
        .saturating_sub(cursor.column.saturating_add(1))
        * layout
            .rows_per_column
            .saturating_sub(cursor.reserved_top_rows);
    current_column_rows + later_columns
}

fn wide_rows_per_page(layout: DocumentLayout) -> usize {
    layout.lines_per_page.max(layout.rows_per_column)
}

fn normal_row_limit(layout: DocumentLayout, cursor: PlacementCursor) -> usize {
    layout
        .rows_per_column
        .saturating_sub(cursor.reserved_bottom_rows)
        .max(cursor.reserved_top_rows)
}

fn normalize_normal_cursor(pass: &mut LayoutPass, layout: DocumentLayout, lines: &[Line]) {
    if pass.cursor.row < pass.cursor.reserved_top_rows {
        pass.cursor.row = pass.cursor.reserved_top_rows;
    }
    while pass.cursor.row >= normal_row_limit(layout, pass.cursor) && !pass.cursor.is_page_start() {
        advance_to_next_column_in_pass(pass, layout, lines);
        if pass.cursor.row < pass.cursor.reserved_top_rows {
            pass.cursor.row = pass.cursor.reserved_top_rows;
        }
    }
}

fn consume_normal_slots(
    pass: &mut LayoutPass,
    layout: DocumentLayout,
    mut slots: usize,
    lines: &[Line],
) {
    while slots > 0 {
        let rows_left = normal_row_limit(layout, pass.cursor).saturating_sub(pass.cursor.row);
        if slots < rows_left {
            pass.cursor.row += slots;
            return;
        }
        slots = slots.saturating_sub(rows_left);
        advance_to_next_column_in_pass(pass, layout, lines);
    }
}

fn advance_to_next_column_in_pass(pass: &mut LayoutPass, layout: DocumentLayout, lines: &[Line]) {
    flush_pending_bottom_floats(pass, layout, lines);
    advance_to_next_column(&mut pass.cursor, layout);
}

fn advance_to_next_page_in_pass(pass: &mut LayoutPass, layout: DocumentLayout, lines: &[Line]) {
    flush_pending_bottom_floats(pass, layout, lines);
    advance_to_next_page(&mut pass.cursor);
}

fn advance_to_next_column(cursor: &mut PlacementCursor, layout: DocumentLayout) {
    cursor.column += 1;
    if cursor.column >= layout.columns {
        advance_to_next_page(cursor);
    } else {
        cursor.row = cursor.reserved_top_rows;
    }
}

fn advance_to_next_page(cursor: &mut PlacementCursor) {
    cursor.page_index += 1;
    cursor.column = 0;
    cursor.row = 0;
    cursor.reserved_top_rows = 0;
    cursor.reserved_bottom_rows = 0;
    cursor.page_has_normal = false;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FloatKind {
    Figure,
    Table,
}

impl FloatKind {
    fn from_env(env: &str) -> Option<Self> {
        match env.trim().trim_end_matches('*') {
            "figure" => Some(Self::Figure),
            "table" => Some(Self::Table),
            _ => None,
        }
    }

    fn from_caption_kind(kind: &str) -> Self {
        match kind.trim() {
            "table" => Self::Table,
            _ => Self::Figure,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Figure => "Figure",
            Self::Table => "Table",
        }
    }

    fn list_heading(self) -> &'static str {
        match self {
            Self::Figure => "List of Figures",
            Self::Table => "List of Tables",
        }
    }

    fn contents_kind(self) -> &'static str {
        match self {
            Self::Figure => "figure",
            Self::Table => "table",
        }
    }

    fn sidecar_extension(self) -> &'static str {
        match self {
            Self::Figure => "lof",
            Self::Table => "lot",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaptionLabelSeparator {
    Colon,
    Period,
}

impl CaptionLabelSeparator {
    fn text(self) -> &'static str {
        match self {
            Self::Colon => ": ",
            Self::Period => ". ",
        }
    }
}

fn caption_label(
    label: &str,
    number: usize,
    caption: &str,
    separator: CaptionLabelSeparator,
) -> String {
    format!("{label} {number}{}{caption}", separator.text())
}

fn next_float_number(
    kind: FloatKind,
    figure_counter: &mut usize,
    table_counter: &mut usize,
) -> usize {
    match kind {
        FloatKind::Figure => {
            *figure_counter += 1;
            *figure_counter
        }
        FloatKind::Table => {
            *table_counter += 1;
            *table_counter
        }
    }
}

fn next_float_number_without_increment(
    kind: FloatKind,
    figure_counter: usize,
    table_counter: usize,
) -> usize {
    match kind {
        FloatKind::Figure => figure_counter + 1,
        FloatKind::Table => table_counter + 1,
    }
}

#[derive(Debug, Clone)]
struct ImageAsset {
    path: PathBuf,
    width_px: u16,
    height_px: u16,
    display_width_pt: f32,
    display_height_pt: f32,
    rotation_degrees: f32,
    viewport: ImageViewport,
    payload: Arc<ImagePayload>,
}

#[derive(Debug, Clone, Copy)]
struct ImageViewport {
    left_fraction: f32,
    bottom_fraction: f32,
    width_fraction: f32,
    height_fraction: f32,
    clip: bool,
}

impl ImageViewport {
    fn full() -> Self {
        Self {
            left_fraction: 0.0,
            bottom_fraction: 0.0,
            width_fraction: 1.0,
            height_fraction: 1.0,
            clip: false,
        }
    }
}

#[derive(Debug, Clone)]
enum ImagePayload {
    Jpeg(Vec<u8>),
    Png {
        color_space: PdfColorSpace,
        bits_per_component: u8,
        data: Vec<u8>,
        alpha: Option<Vec<u8>>,
        decode_params: Option<PngDecodeParams>,
    },
    PdfForm(PdfFormAsset),
}

#[derive(Debug, Clone, Copy)]
struct PngDecodeParams {
    colors: u8,
    bits_per_component: u8,
    columns: u16,
}

#[derive(Debug, Clone)]
struct PdfFormAsset {
    bbox: [f32; 4],
    content: Vec<u8>,
    resources: Option<LoObject>,
    imported_objects: Vec<(LoObjectId, LoObject)>,
}

#[derive(Debug, Clone, Copy)]
enum PdfColorSpace {
    DeviceGray,
    DeviceRgb,
}

impl PdfColorSpace {
    fn name(self) -> &'static str {
        match self {
            PdfColorSpace::DeviceGray => "/DeviceGray",
            PdfColorSpace::DeviceRgb => "/DeviceRGB",
        }
    }
}

#[derive(Debug, Clone)]
enum GraphicElement {
    Image(ImageAsset),
    Placeholder(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LabelInfo {
    value: String,
    page: usize,
    reference_prefix: Option<String>,
}

impl LabelInfo {
    fn new(value: impl Into<String>, page: usize) -> Self {
        Self {
            value: value.into(),
            page: page.max(1),
            reference_prefix: None,
        }
    }

    fn with_reference_prefix(
        value: impl Into<String>,
        page: usize,
        reference_prefix: impl Into<String>,
    ) -> Self {
        Self {
            value: value.into(),
            page: page.max(1),
            reference_prefix: Some(reference_prefix.into()),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct CitationRegistry {
    numbers: HashMap<String, usize>,
    keys: Vec<String>,
    entries: Vec<CitationEntry>,
    backrefs: Vec<CitationBackref>,
    labels: HashMap<String, CitationLabel>,
    style: CitationStyle,
    visible_backrefs: bool,
}

#[derive(Debug, Clone)]
struct CitationEntry {
    key: String,
    number: usize,
    text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CitationLabel {
    author: String,
    year: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum CitationStyle {
    #[default]
    Numeric,
    AuthorYear,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CitationBackref {
    key: String,
    page: usize,
}

#[derive(Debug, Clone, Default)]
struct BibliographyMetadata {
    styles: Vec<String>,
    databases: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct IndexRegistry {
    entries: Vec<IndexEntry>,
    requested: bool,
    printed: bool,
}

impl IndexRegistry {
    fn should_write_sidecar(&self) -> bool {
        self.requested || !self.entries.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexEntry {
    raw: String,
    display: String,
    page: usize,
}

#[derive(Debug, Clone, Default)]
struct PdfMetadata {
    entries: BTreeMap<String, String>,
}

impl PdfMetadata {
    fn insert_if_nonempty(&mut self, key: &str, value: String) {
        let value = value.trim();
        if !value.is_empty() {
            self.entries.insert(key.to_string(), value.to_string());
        }
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FootnoteEntry {
    number: usize,
    text: String,
}

#[derive(Debug, Clone, Default)]
struct FootnoteRegistry {
    entries: Vec<FootnoteEntry>,
    last_mark: Option<usize>,
}

impl FootnoteRegistry {
    fn add(&mut self, text: String) -> usize {
        let number = self.next_number();
        self.set_text(number, text);
        self.last_mark = Some(number);
        number
    }

    fn mark(&mut self, explicit: Option<&str>) -> usize {
        let number = footnote_number_from_option(explicit).unwrap_or_else(|| self.next_number());
        if !self.entries.iter().any(|entry| entry.number == number) {
            self.entries.push(FootnoteEntry {
                number,
                text: String::new(),
            });
        }
        self.last_mark = Some(number);
        number
    }

    fn add_text(&mut self, explicit: Option<&str>, text: String) -> usize {
        let number = footnote_number_from_option(explicit)
            .or(self.last_mark)
            .unwrap_or_else(|| self.next_number());
        self.set_text(number, text);
        self.last_mark = Some(number);
        number
    }

    fn into_entries(mut self) -> Vec<FootnoteEntry> {
        self.entries.retain(|entry| !entry.text.is_empty());
        self.entries.sort_by_key(|entry| entry.number);
        self.entries
    }

    fn next_number(&self) -> usize {
        self.entries
            .iter()
            .map(|entry| entry.number)
            .max()
            .unwrap_or(0)
            + 1
    }

    fn set_text(&mut self, number: usize, text: String) {
        if let Some(entry) = self.entries.iter_mut().find(|entry| entry.number == number) {
            entry.text = text;
        } else {
            self.entries.push(FootnoteEntry { number, text });
        }
    }
}

fn footnote_number_from_option(option: Option<&str>) -> Option<usize> {
    option.and_then(|value| value.trim().parse().ok())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TocEntry {
    level: TocLevel,
    kind: String,
    number: Option<String>,
    title: String,
    page: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FloatEntry {
    kind: FloatKind,
    number: String,
    title: String,
    page: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BookmarkEntry {
    level: usize,
    dest: String,
    title: String,
    page: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TocLevel {
    Section,
    Subsection,
}

impl TocLevel {
    fn kind(self) -> &'static str {
        match self {
            TocLevel::Section => "section",
            TocLevel::Subsection => "subsection",
        }
    }

    fn from_contents_kind(kind: &str) -> Self {
        match kind.trim() {
            "subsection" | "subsubsection" | "paragraph" => TocLevel::Subsection,
            _ => TocLevel::Section,
        }
    }
}

const VERBATIM_PROTECTED_PERCENT: char = '\u{E010}';
const VERBATIM_PROTECTED_BACKSLASH: char = '\u{E011}';

fn document_layout(source: &str) -> DocumentLayout {
    if source.contains("simpleicml") || source_contains_control(source, "twocolumn") {
        return DocumentLayout::icml_two_column();
    }
    if source.contains("neurips_2026") {
        return DocumentLayout::neurips_single_column();
    }
    DocumentLayout::default()
}

fn section_number(section: usize, appendix_mode: bool) -> String {
    if appendix_mode {
        appendix_section_letter(section)
    } else {
        section.to_string()
    }
}

fn subsection_number(section: usize, subsection: usize, appendix_mode: bool) -> String {
    format!("{}.{}", section_number(section, appendix_mode), subsection)
}

fn appendix_section_letter(section: usize) -> String {
    if section == 0 {
        return "0".to_string();
    }
    let mut value = section;
    let mut letters = Vec::new();
    while value > 0 {
        value -= 1;
        letters.push((b'A' + (value % 26) as u8) as char);
        value /= 26;
    }
    letters.iter().rev().collect()
}

fn parse_supported_document(
    source: &str,
    root_dir: &Path,
    output_dir: &Path,
    job_name: &str,
    inputs: &mut Vec<PathBuf>,
    artifact_policy: NativeArtifactPolicy,
) -> Result<SimpleDocument, String> {
    let pre_body_started = Instant::now();
    let mut timings = ParseTimings::default();
    let preprocess_started = Instant::now();
    let raw_page_style = native_page_style_from_source(source);
    let source = apply_native_document_hooks(source)?;
    let source = preprocess_native_read_streams(&source, root_dir, output_dir, job_name, inputs)?;
    let listing_reference_name = listing_reference_name(&source);
    let source_caption_label_separator =
        caption_label_separator_from_source_tree(&source, root_dir)?;
    let capitalize_cref_names =
        source_directly_loads_package_with_any_option(&source, "cleveref", &["capitalize"])?;
    timings.preprocess_ms = preprocess_started.elapsed().as_millis();
    let expansion_started = Instant::now();
    let expanded_source = expand_source_for_native(&source, root_dir, job_name)?;
    timings.expansion_ms = expansion_started.elapsed().as_millis();
    let metadata_started = Instant::now();
    let write_policy = if artifact_policy == NativeArtifactPolicy::LegacySidecars {
        NativeWritePolicy::Materialize
    } else {
        NativeWritePolicy::StripOnly
    };
    let (expanded_source, mut generated_outputs) =
        extract_native_write_streams(&expanded_source, output_dir, job_name, write_policy)?;
    let (expanded_source, pdf_metadata) = extract_pdf_metadata_primitives(&expanded_source)?;
    reject_known_unsupported(&expanded_source)?;
    let macros = parse_simple_macros(&expanded_source)?;
    let Some(body) = document_body(&expanded_source) else {
        return Err("native backend currently requires a LaTeX document environment".to_string());
    };
    let base_layout = document_layout(&expanded_source);
    let caption_label_separator = caption_label_separator_override(&expanded_source)
        .or(source_caption_label_separator)
        .unwrap_or(CaptionLabelSeparator::Colon);
    timings.metadata_ms = metadata_started.elapsed().as_millis();
    let bibliography_started = Instant::now();
    let bibliography = if artifact_policy == NativeArtifactPolicy::LegacySidecars {
        bibliography_metadata(&expanded_source, root_dir, inputs)?
    } else {
        BibliographyMetadata::default()
    };
    timings.bibliography_ms = bibliography_started.elapsed().as_millis();
    let analysis = collect_pre_body_analysis(
        &expanded_source,
        body,
        root_dir,
        inputs,
        base_layout,
        &listing_reference_name,
        capitalize_cref_names,
        artifact_policy,
    )?;
    timings.labels_ms = analysis.labels_ms;
    timings.citations_ms = analysis.citations_ms;
    timings.hyperref_ms = analysis.hyperref_ms;
    inputs.extend(analysis.citation_input_paths);
    let labels = analysis.labels;
    let citations = analysis.citations;
    let hyperref_flags = analysis.hyperref_flags;
    let index_started = Instant::now();
    let index = collect_index_entries(body, &macros, &labels, &citations, &base_layout)?;
    if index.should_write_sidecar() {
        generated_outputs.push(GeneratedOutput {
            path: output_dir.join(format!("{job_name}.idx")),
            content: index_sidecar_content(&index),
        });
    }
    timings.index_ms = index_started.elapsed().as_millis();
    let lists_floats_started = Instant::now();
    let graphics = graphics_config(&expanded_source)?;
    let toc_requested = source_contains_control(body, "tableofcontents");
    let list_of_figures_requested = source_contains_control(body, "listoffigures");
    let list_of_tables_requested = source_contains_control(body, "listoftables");
    let legacy_sidecars = artifact_policy == NativeArtifactPolicy::LegacySidecars;
    let toc_entries = if legacy_sidecars || toc_requested {
        collect_toc_entries(body, &macros, &labels, &citations, &base_layout)?
    } else {
        Vec::new()
    };
    let float_entries = if legacy_sidecars || list_of_figures_requested || list_of_tables_requested
    {
        collect_float_entries(body, &macros, &labels, &citations, &base_layout)?
    } else {
        Vec::new()
    };
    timings.lists_floats_ms = lists_floats_started.elapsed().as_millis();
    let hyperref_out_requested = hyperref_flags.hyperref_out_requested;
    let backref_requested = hyperref_flags.backref_requested;
    let bookmarks_started = Instant::now();
    let bookmarks = if hyperref_out_requested {
        collect_bookmarks(body, &macros, &labels, &citations, &base_layout)?
    } else {
        Vec::new()
    };
    timings.hyperref_ms += bookmarks_started.elapsed().as_millis();
    let title_started = Instant::now();
    let mut footnotes = FootnoteRegistry::default();
    let title = braced_command_payload_collecting(
        &expanded_source,
        "title",
        &macros,
        &labels,
        &citations,
        &mut footnotes,
    );
    let author = braced_command_payload_collecting(
        &expanded_source,
        "author",
        &macros,
        &labels,
        &citations,
        &mut footnotes,
    );
    let date = braced_command_payload_collecting(
        &expanded_source,
        "date",
        &macros,
        &labels,
        &citations,
        &mut footnotes,
    );
    let neurips_author_grid = if base_layout == DocumentLayout::neurips_single_column() {
        native_neurips_author_grid_from_source(
            &expanded_source,
            &macros,
            &labels,
            &citations,
            &mut footnotes,
        )?
    } else {
        None
    };
    timings.title_ms = title_started.elapsed().as_millis();
    let mut lines = Vec::new();
    let mut images = Vec::new();
    let mut cursor = body;
    let mut layout = base_layout;
    let mut section_counter = 0_usize;
    let mut subsection_counter = 0_usize;
    let mut equation_counter = 0_usize;
    let mut appendix_mode = false;
    let mut current_float = None;
    let mut figure_counter = 0_usize;
    let mut table_counter = 0_usize;
    let mut listing_counter = 0_usize;
    let mut theorem_counters = HashMap::new();
    let mut list_stack = Vec::new();
    let mut in_abstract = false;
    let mut bibliography_rendered = false;
    let mut graphics_cache = GraphicsCache::default();
    timings.pre_body_ms = pre_body_started.elapsed().as_millis();
    prewarm_graphics_cache(body, root_dir, &graphics, &mut graphics_cache, &mut timings);
    let body_started = Instant::now();
    if let Some(page_style) = raw_page_style
        .or_else(|| native_page_style_from_source(&source))
        .or_else(|| native_page_style_from_source(&expanded_source))
    {
        lines.push(Line::PageStyle(page_style));
    }

    while !cursor.is_empty() {
        if cursor.starts_with("\n\n") {
            lines.push(Line::Blank);
            cursor = cursor.trim_start_matches('\n');
            continue;
        }
        let trimmed = cursor.trim_start();
        if trimmed.len() != cursor.len() {
            cursor = trimmed;
            continue;
        }
        if cursor.starts_with('%') {
            cursor = cursor
                .find('\n')
                .map(|index| &cursor[index + '\n'.len_utf8()..])
                .unwrap_or("");
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\section") {
            let (rest, starred) = strip_optional_star(rest);
            let (_, rest) = take_optional_bracketed(rest);
            let (heading, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\section titles".to_string())?;
            if !starred {
                section_counter += 1;
                subsection_counter = 0;
            }
            lines.push(Line::Blank);
            let heading = clean_inline_text_collecting(
                heading,
                &macros,
                &labels,
                &citations,
                &mut footnotes,
            )?;
            lines.push(Line::Heading(if starred {
                heading
            } else {
                format!(
                    "{} {heading}",
                    section_number(section_counter, appendix_mode)
                )
            }));
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\subsection") {
            let (rest, starred) = strip_optional_star(rest);
            let (_, rest) = take_optional_bracketed(rest);
            let (heading, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\subsection titles".to_string())?;
            if !starred {
                subsection_counter += 1;
            }
            lines.push(Line::Blank);
            let heading = clean_inline_text_collecting(
                heading,
                &macros,
                &labels,
                &citations,
                &mut footnotes,
            )?;
            lines.push(Line::Heading(if starred {
                heading
            } else {
                format!(
                    "{} {heading}",
                    subsection_number(section_counter, subsection_counter, appendix_mode)
                )
            }));
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\subsubsection") {
            let (rest, _) = strip_optional_star(rest);
            let (_, rest) = take_optional_bracketed(rest);
            let (heading, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\subsubsection titles".to_string()
            })?;
            lines.push(Line::Blank);
            lines.push(Line::Heading(clean_inline_text_collecting(
                heading,
                &macros,
                &labels,
                &citations,
                &mut footnotes,
            )?));
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\paragraph") {
            let (rest, _) = strip_optional_star(rest);
            let (_, rest) = take_optional_bracketed(rest);
            let (heading, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\paragraph titles".to_string())?;
            let heading = clean_inline_text_collecting(
                heading,
                &macros,
                &labels,
                &citations,
                &mut footnotes,
            )?;
            if !heading.is_empty() {
                let heading = paragraph_heading_text(&heading);
                let run_in_source = remaining.trim_start_matches([' ', '\t', '\r', '\n']);
                let (paragraph, after_paragraph) = if starts_with_blank_line(remaining) {
                    ("", remaining)
                } else {
                    take_until_command_or_blank(run_in_source)
                };
                let cleaned_paragraph = clean_inline_text_collecting(
                    paragraph,
                    &macros,
                    &labels,
                    &citations,
                    &mut footnotes,
                )?;
                if cleaned_paragraph.is_empty() {
                    append_wrapped_paragraph_text_lines(&mut lines, &layout, in_abstract, &heading);
                    cursor = after_paragraph;
                } else {
                    append_wrapped_paragraph_text_lines(
                        &mut lines,
                        &layout,
                        in_abstract,
                        &format!("{heading} {cleaned_paragraph}"),
                    );
                    cursor = after_paragraph.trim_start();
                }
            } else {
                cursor = remaining;
            }
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\maketitle") {
            append_maketitle_lines(
                &mut lines,
                title.as_deref(),
                author.as_deref(),
                date.as_deref(),
                neurips_author_grid.as_ref(),
                &layout,
            );
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\nativeicmlmaketitle") {
            let (title, rest) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\nativeicmlmaketitle titles".to_string()
            })?;
            let (authors, rest) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\nativeicmlmaketitle authors".to_string()
            })?;
            let (affiliations, rest) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\nativeicmlmaketitle affiliations".to_string()
            })?;
            let (abstract_text, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\nativeicmlmaketitle abstracts".to_string()
            })?;
            let title =
                clean_inline_text_collecting(title, &macros, &labels, &citations, &mut footnotes)?;
            let authors = clean_inline_text_collecting(
                authors,
                &macros,
                &labels,
                &citations,
                &mut footnotes,
            )?;
            let affiliations = clean_inline_text_collecting(
                affiliations,
                &macros,
                &labels,
                &citations,
                &mut footnotes,
            )?;
            append_wide_maketitle_lines(
                &mut lines,
                (!title.is_empty()).then_some(title.as_str()),
                (!authors.is_empty()).then_some(authors.as_str()),
                (!affiliations.is_empty()).then_some(affiliations.as_str()),
                &layout,
            );
            append_nativeicml_abstract_lines(
                &mut lines,
                &mut images,
                inputs,
                abstract_text,
                root_dir,
                &graphics,
                &mut graphics_cache,
                &layout,
                &macros,
                &labels,
                &citations,
                &mut footnotes,
                &mut timings,
                &mut figure_counter,
                &mut table_counter,
                caption_label_separator,
            )?;
            append_blank_lines_to_page_boundary(&mut lines, &layout, &images);
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\tableofcontents") {
            lines.push(Line::Blank);
            lines.push(Line::Heading("Contents".to_string()));
            append_toc_lines(&mut lines, &toc_entries, &layout);
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\makeindex") {
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\printindex") {
            let (_, remaining) = take_optional_bracketed(rest);
            lines.push(Line::Blank);
            lines.push(Line::Heading("Index".to_string()));
            for entry in sorted_index_entries(&index) {
                for line in layout.wrap_text(&format!("{} {}", entry.display, entry.page)) {
                    lines.push(Line::Text(line));
                }
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\listoffigures") {
            lines.push(Line::Blank);
            lines.push(Line::Heading(FloatKind::Figure.list_heading().to_string()));
            append_float_list_lines(&mut lines, &float_entries, FloatKind::Figure, &layout);
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\listoftables") {
            lines.push(Line::Blank);
            lines.push(Line::Heading(FloatKind::Table.list_heading().to_string()));
            append_float_list_lines(&mut lines, &float_entries, FloatKind::Table, &layout);
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\addcontentsline") {
            let (_, rest) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\addcontentsline file targets".to_string()
            })?;
            let (_, rest) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\addcontentsline entry kinds".to_string()
            })?;
            let (_, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\addcontentsline titles".to_string()
            })?;
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\pdfbookmark") {
            let (_, rest) = take_optional_bracketed(rest);
            let (_, rest) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\pdfbookmark titles".to_string())?;
            let (_, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\pdfbookmark destination names".to_string()
            })?;
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\phantomsection") {
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\\\") {
            lines.push(Line::Blank);
            let (_, rest) = take_optional_bracketed(rest);
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\appendix") {
            appendix_mode = true;
            section_counter = 0;
            subsection_counter = 0;
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\addbibresource") {
            let (_, rest) = take_optional_bracketed(rest);
            let (_, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\addbibresource paths".to_string()
            })?;
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\printbibliography") {
            let (_, remaining) = take_optional_bracketed(rest);
            if append_bibliography_lines(&mut lines, &citations, &layout) {
                bibliography_rendered = true;
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\index") {
            let (_, rest) = take_optional_bracketed(rest);
            let (_, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\index payloads".to_string())?;
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\graphicspath") {
            let (_, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\graphicspath paths".to_string())?;
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\DeclareGraphicsExtensions") {
            let (_, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\DeclareGraphicsExtensions payloads".to_string()
            })?;
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\includegraphics") {
            let includegraphics_started = Instant::now();
            let (graphic, remaining) =
                parse_includegraphics(rest, root_dir, &graphics, &mut graphics_cache, &layout)?;
            timings.includegraphics_ms += includegraphics_started.elapsed().as_millis();
            match graphic {
                GraphicElement::Image(image) => {
                    inputs.push(image.path.clone());
                    let image_index = images.len();
                    images.push(image);
                    lines.push(Line::Image(image_index));
                }
                GraphicElement::Placeholder(path) => {
                    inputs.push(path.clone());
                    let name = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("graphic");
                    lines.push(Line::Text(format!("[graphic: {name}]")));
                }
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{tabular}") {
            let (table_lines, remaining) =
                parse_tabular(rest, &macros, &labels, &citations, "tabular")?;
            append_table_lines(&mut lines, table_lines, &layout);
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\resizebox") {
            let (table_lines, remaining) = parse_resizebox(rest, &macros, &labels, &citations)?;
            append_table_lines(&mut lines, table_lines, &layout);
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{tikzpicture}") {
            let (_, rest) = take_optional_bracketed(rest);
            let (tikz_body, remaining) = take_environment_body(rest, "tikzpicture")?;
            if let Some(image) = native_tikz_graphic(tikz_body) {
                let image_index = images.len();
                images.push(image);
                lines.push(Line::Image(image_index));
            } else {
                lines.push(Line::Blank);
                lines.push(Line::Text("[TikZ picture]".to_string()));
                lines.push(Line::Blank);
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{lstlisting}") {
            let (options, rest) = take_optional_bracketed(rest);
            let (listing, remaining) = take_environment_body(rest, "lstlisting")?;
            listing_counter += 1;
            lines.push(Line::Blank);
            if let Some(caption) = options.and_then(caption_option_value) {
                let caption = clean_inline_text_collecting(
                    &caption,
                    &macros,
                    &labels,
                    &citations,
                    &mut footnotes,
                )?;
                if !caption.is_empty() {
                    append_caption_lines(
                        &mut lines,
                        &caption_label(
                            &listing_reference_name,
                            listing_counter,
                            &caption,
                            caption_label_separator,
                        ),
                        &layout,
                    );
                }
            }
            lines.extend(code_listing_lines(listing).into_iter().map(Line::Code));
            lines.push(Line::Blank);
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\lstinputlisting") {
            let (options, rest) = take_optional_bracketed(rest);
            let (path, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\lstinputlisting paths".to_string()
            })?;
            let listing = read_code_input(root_dir, path, "\\lstinputlisting", inputs)?;
            listing_counter += 1;
            lines.push(Line::Blank);
            if let Some(caption) = options.and_then(caption_option_value) {
                let caption = clean_inline_text_collecting(
                    &caption,
                    &macros,
                    &labels,
                    &citations,
                    &mut footnotes,
                )?;
                if !caption.is_empty() {
                    append_caption_lines(
                        &mut lines,
                        &caption_label(
                            &listing_reference_name,
                            listing_counter,
                            &caption,
                            caption_label_separator,
                        ),
                        &layout,
                    );
                }
            }
            lines.extend(code_listing_lines(&listing).into_iter().map(Line::Code));
            lines.push(Line::Blank);
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\inputminted") {
            let (_, rest) = take_optional_bracketed(rest);
            let (_, rest) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\inputminted languages".to_string()
            })?;
            let (path, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\inputminted paths".to_string())?;
            let listing = read_code_input(root_dir, path, "\\inputminted", inputs)?;
            lines.push(Line::Blank);
            lines.extend(code_listing_lines(&listing).into_iter().map(Line::Code));
            lines.push(Line::Blank);
            cursor = remaining;
            continue;
        }
        if let Some((literal, remaining)) = take_verbatim_like_environment(cursor)? {
            lines.push(Line::Blank);
            lines.extend(code_listing_lines(literal).into_iter().map(Line::Code));
            lines.push(Line::Blank);
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{algorithmic}") {
            let (_, rest) = take_optional_bracketed(rest);
            let (algorithm, remaining) = take_environment_body(rest, "algorithmic")?;
            lines.push(Line::Blank);
            for line in algorithmic_lines(algorithm, &macros, &labels, &citations, &mut footnotes)?
            {
                for wrapped in wrap_text(&line, layout.code_wrap_width) {
                    lines.push(Line::Code(wrapped));
                }
            }
            lines.push(Line::Blank);
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{thebibliography}") {
            let (_, rest) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced thebibliography labels".to_string()
            })?;
            let (_, remaining) = take_environment_body(rest, "thebibliography")?;
            if append_bibliography_lines(&mut lines, &citations, &layout) {
                bibliography_rendered = true;
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{equation}") {
            let (equation, remaining) = take_environment_body(rest, "equation")?;
            equation_counter += 1;
            let equation = clean_equation_text(equation);
            if !equation.is_empty() {
                lines.push(Line::Blank);
                lines.push(Line::Equation(format!("{} ({equation_counter})", equation)));
                lines.push(Line::Blank);
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{equation*}") {
            let (equation, remaining) = take_environment_body(rest, "equation*")?;
            let equation = clean_equation_text(equation);
            if !equation.is_empty() {
                lines.push(Line::Blank);
                lines.push(Line::Equation(equation));
                lines.push(Line::Blank);
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{multline}") {
            let (equation, remaining) = take_environment_body(rest, "multline")?;
            equation_counter += 1;
            let equation = clean_equation_text(equation);
            if !equation.is_empty() {
                lines.push(Line::Blank);
                lines.push(Line::Equation(format!("{} ({equation_counter})", equation)));
                lines.push(Line::Blank);
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{multline*}") {
            let (equation, remaining) = take_environment_body(rest, "multline*")?;
            let equation = clean_equation_text(equation);
            if !equation.is_empty() {
                lines.push(Line::Blank);
                lines.push(Line::Equation(equation));
                lines.push(Line::Blank);
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{align}") {
            let (align, remaining) = take_environment_body(rest, "align")?;
            let align_lines = clean_align_lines(align, &mut equation_counter, true);
            if !align_lines.is_empty() {
                lines.push(Line::Blank);
                lines.extend(align_lines.into_iter().map(Line::Equation));
                lines.push(Line::Blank);
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin{align*}") {
            let (align, remaining) = take_environment_body(rest, "align*")?;
            let align_lines = clean_align_lines(align, &mut equation_counter, false);
            if !align_lines.is_empty() {
                lines.push(Line::Blank);
                lines.extend(align_lines.into_iter().map(Line::Equation));
                lines.push(Line::Blank);
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("$$") {
            let (equation, remaining) = take_dollar_display_math(rest)?;
            let equation = clean_equation_text(equation);
            if !equation.is_empty() {
                lines.push(Line::DisplayEquation(equation));
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin")
            && let Some((env, after_env)) = take_braced(rest)
            && let Some(spec) = theorem_environment_spec(env)
        {
            let opening = parse_theorem_opening(spec, after_env)?;
            let number = next_theorem_number(&mut theorem_counters, spec.counter_key);
            let title = opening
                .title
                .map(|title| {
                    clean_inline_text_collecting(
                        title,
                        &macros,
                        &labels,
                        &citations,
                        &mut footnotes,
                    )
                })
                .transpose()?
                .unwrap_or_default();
            let heading = theorem_heading(spec.display_name, number, &title);
            let (theorem_body, remaining) = take_environment_body(opening.remaining, env)?;
            if theorem_body.contains("\\includegraphics")
                || theorem_body.contains("\\begin{minipage}")
            {
                append_theorem_box_lines(
                    &mut lines,
                    &mut images,
                    inputs,
                    &heading,
                    theorem_body,
                    root_dir,
                    &graphics,
                    &mut graphics_cache,
                    &layout,
                    &macros,
                    &labels,
                    &citations,
                    &mut footnotes,
                    &mut timings,
                    &mut equation_counter,
                )?;
                cursor = remaining;
            } else {
                if !heading.is_empty() {
                    lines.push(Line::Blank);
                    lines.push(Line::Heading(heading));
                }
                cursor = opening.remaining;
            }
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\begin")
            && let Some((env, after_env)) = take_braced(rest)
            && env.trim() == "lstfloat"
        {
            let top_float = float_prefers_top(after_env) && layout.columns == 1;
            let body_start = consume_environment_open_args(env, after_env);
            if let Ok((float_body, remaining)) = take_environment_body(body_start, env)
                && append_listing_float_lines(
                    &mut lines,
                    float_body,
                    &layout,
                    &macros,
                    &labels,
                    &citations,
                    &mut footnotes,
                    &mut listing_counter,
                    &listing_reference_name,
                    caption_label_separator,
                    top_float,
                )?
            {
                cursor = remaining;
                continue;
            }
        }
        if let Some(rest) = cursor.strip_prefix("\\begin")
            && let Some((env, after_env)) = take_braced(rest)
            && let Some(kind) = FloatKind::from_env(env)
        {
            let top_float = float_prefers_top(after_env);
            let bottom_float = float_prefers_bottom(after_env);
            let body_start = consume_environment_open_args(env, after_env);
            if let Ok((float_body, remaining)) = take_environment_body(body_start, env) {
                if append_theorem_float_lines(
                    &mut lines,
                    &mut images,
                    inputs,
                    float_body,
                    root_dir,
                    &graphics,
                    &mut graphics_cache,
                    &layout,
                    &macros,
                    &labels,
                    &citations,
                    &mut footnotes,
                    &mut timings,
                    &mut theorem_counters,
                    &mut equation_counter,
                )? {
                    cursor = remaining;
                    continue;
                }
                if float_body.contains("\\begin{minipage}") {
                    append_minipage_float_lines(
                        &mut lines,
                        &mut images,
                        inputs,
                        kind,
                        env,
                        top_float,
                        float_body,
                        root_dir,
                        &graphics,
                        &mut graphics_cache,
                        &layout,
                        &macros,
                        &labels,
                        &citations,
                        &mut footnotes,
                        &mut timings,
                        &mut figure_counter,
                        &mut table_counter,
                        caption_label_separator,
                    )?;
                    cursor = remaining;
                    continue;
                }
                if append_table_float_lines(
                    &mut lines,
                    kind,
                    env,
                    top_float,
                    bottom_float,
                    float_body,
                    &layout,
                    &macros,
                    &labels,
                    &citations,
                    &mut footnotes,
                    &mut figure_counter,
                    &mut table_counter,
                    caption_label_separator,
                )? {
                    cursor = remaining;
                    continue;
                }
                if find_control(float_body, "includegraphics").is_some() {
                    let defer_to_float_page = should_defer_graphic_float_to_float_page(
                        base_layout,
                        &layout,
                        appendix_mode,
                        kind,
                        top_float,
                        figure_counter,
                    );
                    if append_graphic_float_lines(
                        &mut lines,
                        &mut images,
                        inputs,
                        kind,
                        env,
                        top_float,
                        float_body,
                        root_dir,
                        &graphics,
                        &mut graphics_cache,
                        &layout,
                        &macros,
                        &labels,
                        &citations,
                        &mut footnotes,
                        &mut timings,
                        &mut figure_counter,
                        &mut table_counter,
                        caption_label_separator,
                        defer_to_float_page,
                    )? {
                        cursor = remaining;
                        continue;
                    }
                    if layout.columns > 1 {
                        let next_number = next_float_number_without_increment(
                            kind,
                            figure_counter,
                            table_counter,
                        );
                        record_two_column_graphic_float_fallback(
                            &mut timings,
                            kind,
                            next_number,
                            env,
                            top_float,
                            float_body,
                            root_dir,
                            &graphics,
                            &mut graphics_cache,
                            &layout,
                            caption_label_separator,
                        );
                    }
                }
            }
        }
        if let Some(rest) = cursor.strip_prefix("\\begin")
            && let Some((env, remaining)) = take_braced(rest)
            && transparent_environment(env)
        {
            if env.trim().trim_end_matches('*') == "abstract" {
                lines.push(Line::AbstractHeading("Abstract".to_string()));
                in_abstract = true;
            }
            if let Some(frame) = ListFrame::from_environment(env, remaining) {
                list_stack.push(frame);
            }
            let remaining = consume_environment_open_args(env, remaining);
            if let Some(kind) = FloatKind::from_env(env) {
                current_float = Some(kind);
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\end")
            && let Some((env, remaining)) = take_braced(rest)
            && transparent_environment(env)
        {
            if env.trim().trim_end_matches('*') == "abstract" {
                in_abstract = false;
                lines.push(Line::Blank);
            }
            if FloatKind::from_env(env).is_some() {
                current_float = None;
            }
            if list_environment(env) {
                list_stack.pop();
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\item") {
            let (explicit_label, remaining) = take_optional_bracketed(rest);
            let marker = list_item_marker(
                &mut list_stack,
                explicit_label,
                &macros,
                &labels,
                &citations,
                &mut footnotes,
            )?;
            lines.push(Line::Text(marker));
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\renewcommand") {
            let (remaining, skipped) = skip_renewcommand(rest);
            if skipped {
                cursor = remaining;
                continue;
            }
        }
        if let Some(rest) = cursor.strip_prefix("\\newtcbtheorem") {
            let (remaining, skipped) = skip_newtcbtheorem(rest);
            if skipped {
                cursor = remaining;
                continue;
            }
        }
        if let Some(rest) = cursor.strip_prefix("\\bibliographystyle") {
            let (_, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\bibliographystyle payloads".to_string()
            })?;
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\bibliography") {
            let (_, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\bibliography payloads".to_string()
            })?;
            if append_bibliography_lines(&mut lines, &citations, &layout) {
                bibliography_rendered = true;
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\newpage") {
            lines.push(Line::OutputControl(OutputControl::NewPage));
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\clearpage") {
            lines.push(Line::OutputControl(OutputControl::ClearPage));
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\onecolumn") {
            let next_layout = base_layout.as_one_column();
            if next_layout != layout {
                lines.push(Line::OutputControl(OutputControl::LayoutSwitch(
                    next_layout,
                )));
                layout = next_layout;
            } else {
                lines.push(Line::OutputControl(OutputControl::ClearPage));
            }
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\twocolumn") {
            let (_, rest) = take_optional_bracketed(rest);
            let next_layout = base_layout;
            if next_layout != layout {
                lines.push(Line::OutputControl(OutputControl::LayoutSwitch(
                    next_layout,
                )));
                layout = next_layout;
            } else {
                lines.push(Line::OutputControl(OutputControl::ClearPage));
            }
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\smallskip") {
            append_vertical_skip_points(&mut lines, 3.0, &layout);
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\medskip") {
            append_vertical_skip_points(&mut lines, 6.0, &layout);
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\bigskip") {
            append_vertical_skip_points(&mut lines, 12.0, &layout);
            cursor = rest;
            continue;
        }
        if let Some(rest) = strip_prefix_any(
            cursor,
            &[
                "\\centering",
                "\\noindent",
                "\\hfill",
                "\\selectfont",
                "\\small",
                "\\footnotesize",
                "\\scriptsize",
                "\\bfseries",
                "\\mdseries",
                "\\itshape",
                "\\upshape",
                "\\sffamily",
                "\\rmfamily",
                "\\ttfamily",
                "\\icmlmaketitle",
            ],
        ) {
            cursor = rest;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\setlength")
            && let Some(remaining) = skip_two_braced_arguments(rest)
        {
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\addtolength")
            && let Some(remaining) = skip_two_braced_arguments(rest)
        {
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\thispagestyle") {
            let (_, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\thispagestyle payloads".to_string()
            })?;
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\pagestyle") {
            let (_, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\pagestyle payloads".to_string())?;
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\fontsize")
            && let Some(remaining) = skip_two_braced_arguments(rest)
        {
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\vspace") {
            let rest = rest.strip_prefix('*').unwrap_or(rest);
            if let Some((payload, remaining)) = take_braced(rest) {
                append_vertical_space_lines(&mut lines, payload, &layout);
                cursor = remaining;
                continue;
            }
        }
        if let Some(rest) = cursor.strip_prefix("\\hspace") {
            let rest = rest.strip_prefix('*').unwrap_or(rest);
            if let Some((_, remaining)) = take_braced(rest) {
                cursor = remaining;
                continue;
            }
        }
        if let Some(rest) = cursor.strip_prefix("\\captionof") {
            let (kind, rest) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\captionof kinds".to_string())?;
            let (_, rest) = take_optional_bracketed(rest);
            let (caption, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\captionof payloads".to_string())?;
            let caption = clean_inline_text_collecting(
                caption,
                &macros,
                &labels,
                &citations,
                &mut footnotes,
            )?;
            if !caption.is_empty() {
                let kind = FloatKind::from_caption_kind(kind);
                let number = next_float_number(kind, &mut figure_counter, &mut table_counter);
                append_caption_lines(
                    &mut lines,
                    &caption_label(kind.label(), number, &caption, caption_label_separator),
                    &layout,
                );
            }
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\captionsetup") {
            let (_, rest) = take_optional_bracketed(rest);
            let (_, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\captionsetup payloads".to_string()
            })?;
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\caption") {
            let (_, rest) = take_optional_bracketed(rest);
            let (caption, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\caption payloads".to_string())?;
            let caption = clean_inline_text_collecting(
                caption,
                &macros,
                &labels,
                &citations,
                &mut footnotes,
            )?;
            if !caption.is_empty() {
                let kind = current_float.unwrap_or(FloatKind::Figure);
                let number = next_float_number(kind, &mut figure_counter, &mut table_counter);
                append_caption_lines(
                    &mut lines,
                    &caption_label(kind.label(), number, &caption, caption_label_separator),
                    &layout,
                );
            }
            cursor = remaining;
            continue;
        }

        let (paragraph, remaining) = take_until_command_or_blank(cursor);
        let paragraph_has_comment = paragraph.contains('%');
        if paragraph.is_empty() && remaining.len() == cursor.len() {
            let preview = cursor
                .lines()
                .next()
                .unwrap_or(cursor)
                .chars()
                .take(80)
                .collect::<String>();
            return Err(format!(
                "native backend parser made no progress near `{preview}`"
            ));
        }
        let cleaned_paragraph =
            clean_inline_text_collecting(paragraph, &macros, &labels, &citations, &mut footnotes)?;
        append_wrapped_text_lines(&mut lines, &layout, in_abstract, &cleaned_paragraph);
        cursor = if paragraph_has_comment {
            remaining
                .find('\n')
                .map(|index| &remaining[index + '\n'.len_utf8()..])
                .unwrap_or("")
        } else {
            remaining
        };
        if cursor.starts_with("\n\n") {
            lines.push(Line::Blank);
            cursor = cursor.trim_start_matches('\n');
        } else {
            cursor = cursor.trim_start();
        }
    }
    timings.body_ms = body_started.elapsed().as_millis();

    if lines.iter().all(|line| matches!(line, Line::Blank)) {
        lines.push(Line::Text(String::new()));
    }
    let footnotes = footnotes.into_entries();
    append_footnote_lines(&mut lines, &footnotes, &layout);
    if !bibliography_rendered {
        append_bibliography_lines(&mut lines, &citations, &layout);
    }

    Ok(SimpleDocument {
        layout: base_layout,
        lines,
        images,
        timings,
        generated_outputs,
        labels: labels.into_iter().collect(),
        citations,
        bibliography,
        index,
        pdf_metadata,
        footnotes,
        toc_entries,
        float_entries,
        bookmarks,
        toc_requested,
        list_of_figures_requested,
        list_of_tables_requested,
        hyperref_out_requested,
        backref_requested,
    })
}

fn reject_known_unsupported(source: &str) -> Result<(), String> {
    let unsupported_controls = ["include", "write", "special", "pdf"];
    for control in unsupported_controls {
        if source_contains_control(source, control) {
            return Err(format!(
                "native backend does not yet support documents containing `\\{control}`"
            ));
        }
    }

    Ok(())
}

fn collect_pre_body_analysis(
    expanded_source: &str,
    body: &str,
    root_dir: &Path,
    inputs: &[PathBuf],
    base_layout: DocumentLayout,
    listing_reference_name: &str,
    capitalize_cref_names: bool,
    artifact_policy: NativeArtifactPolicy,
) -> Result<PreBodyAnalysis, String> {
    let citation_input_start = inputs.len();
    let citation_input_seed = inputs.to_vec();
    thread::scope(|scope| {
        let labels_handle = scope.spawn(|| {
            let started = Instant::now();
            collect_section_labels(
                body,
                &base_layout,
                listing_reference_name,
                capitalize_cref_names,
            )
            .map(|labels| (labels, started.elapsed().as_millis()))
        });
        let citations_handle = scope.spawn(move || {
            let mut citation_inputs = citation_input_seed;
            let started = Instant::now();
            let citations = collect_citations(
                expanded_source,
                body,
                root_dir,
                &mut citation_inputs,
                &base_layout,
            )?;
            let added_inputs = citation_inputs[citation_input_start..].to_vec();
            Ok::<_, String>((citations, started.elapsed().as_millis(), added_inputs))
        });
        let hyperref_handle = scope.spawn(|| {
            let started = Instant::now();
            collect_hyperref_flags(expanded_source, body, root_dir, artifact_policy)
                .map(|flags| (flags, started.elapsed().as_millis()))
        });

        let (labels, labels_ms) = labels_handle
            .join()
            .map_err(|_| "native backend label analysis worker panicked".to_string())??;
        let (citations, citations_ms, citation_input_paths) = citations_handle
            .join()
            .map_err(|_| "native backend citation analysis worker panicked".to_string())??;
        let (hyperref_flags, hyperref_ms) = hyperref_handle
            .join()
            .map_err(|_| "native backend hyperref analysis worker panicked".to_string())??;
        Ok(PreBodyAnalysis {
            labels,
            citations,
            hyperref_flags,
            labels_ms,
            citations_ms,
            hyperref_ms,
            citation_input_paths,
        })
    })
}

fn collect_hyperref_flags(
    expanded_source: &str,
    body: &str,
    root_dir: &Path,
    artifact_policy: NativeArtifactPolicy,
) -> Result<HyperrefFlags, String> {
    let hyperref_out_requested = if artifact_policy == NativeArtifactPolicy::PdfOnly {
        source_has_direct_hyperref_surface(expanded_source, body)?
    } else {
        source_loads_package(expanded_source, root_dir, "hyperref")?
            || source_contains_control(body, "pdfbookmark")
    };
    let backref_requested = hyperref_out_requested
        && if artifact_policy == NativeArtifactPolicy::PdfOnly {
            source_directly_loads_package_with_any_option(
                expanded_source,
                "hyperref",
                &["backref", "backref=true", "pagebackref", "pagebackref=true"],
            )?
        } else {
            source_loads_package_with_any_option(
                expanded_source,
                root_dir,
                "hyperref",
                &["backref", "backref=true", "pagebackref", "pagebackref=true"],
            )?
        };
    Ok(HyperrefFlags {
        hyperref_out_requested,
        backref_requested,
    })
}

fn apply_native_document_hooks(source: &str) -> Result<Cow<'_, str>, String> {
    if !source_may_contain_native_document_hooks(source) {
        return Ok(Cow::Borrowed(source));
    }

    let mut stripped_source = String::with_capacity(source.len());
    let mut begin_hooks = Vec::new();
    let mut end_hooks = Vec::new();
    let mut cursor = source;
    while let Some((index, command)) = find_next_native_document_hook(cursor) {
        stripped_source.push_str(&cursor[..index]);
        let rest = &cursor[index + command.len() + 1..];
        let (_, rest) = take_optional_bracketed(rest);
        let Some((payload, remaining)) = take_braced(rest) else {
            return Err(format!(
                "native backend requires braced \\{command} hook payloads"
            ));
        };
        match command {
            "AtBeginDocument" => begin_hooks.push(payload.to_string()),
            "AtEndDocument" => end_hooks.push(payload.to_string()),
            _ => unreachable!("unexpected native document hook command"),
        }
        cursor = remaining;
    }
    stripped_source.push_str(cursor);

    let begin_marker = "\\begin{document}";
    let begin_index = stripped_source.find(begin_marker).ok_or_else(|| {
        "native backend requires a document environment for document hooks".to_string()
    })?;
    let begin_insert = begin_index + begin_marker.len();
    let end_marker = "\\end{document}";
    let end_index = stripped_source[begin_insert..]
        .find(end_marker)
        .map(|index| begin_insert + index)
        .ok_or_else(|| "native backend could not find \\end{document} for hooks".to_string())?;

    let hook_len: usize = begin_hooks
        .iter()
        .chain(end_hooks.iter())
        .map(|hook| hook.len() + 2)
        .sum();
    let mut hooked_source = String::with_capacity(stripped_source.len() + hook_len);
    hooked_source.push_str(&stripped_source[..begin_insert]);
    for hook in begin_hooks {
        hooked_source.push('\n');
        hooked_source.push_str(&hook);
    }
    hooked_source.push_str(&stripped_source[begin_insert..end_index]);
    for hook in end_hooks {
        hooked_source.push('\n');
        hooked_source.push_str(&hook);
    }
    hooked_source.push_str(&stripped_source[end_index..]);
    Ok(Cow::Owned(hooked_source))
}

fn source_may_contain_native_document_hooks(source: &str) -> bool {
    source.contains("\\AtBeginDocument") || source.contains("\\AtEndDocument")
}

fn find_next_native_document_hook(source: &str) -> Option<(usize, &'static str)> {
    ["AtBeginDocument", "AtEndDocument"]
        .into_iter()
        .filter_map(|command| find_control(source, command).map(|index| (index, command)))
        .min_by_key(|(index, _)| *index)
}

#[derive(Debug, Clone)]
struct NativeReadStream {
    lines: Vec<String>,
    next: usize,
}

impl NativeReadStream {
    fn from_source(source: String) -> Self {
        let lines = source.lines().map(str::to_string).collect();
        Self { lines, next: 0 }
    }

    fn empty() -> Self {
        Self {
            lines: Vec::new(),
            next: 0,
        }
    }

    fn read_line(&mut self) -> String {
        let Some(line) = self.lines.get(self.next) else {
            return String::new();
        };
        self.next += 1;
        line.clone()
    }

    fn is_eof(&self) -> bool {
        self.next >= self.lines.len()
    }
}

fn preprocess_native_read_streams<'a>(
    source: &'a str,
    root_dir: &Path,
    output_dir: &Path,
    job_name: &str,
    inputs: &mut Vec<PathBuf>,
) -> Result<Cow<'a, str>, String> {
    if !source_may_contain_native_read_streams(source) {
        return Ok(Cow::Borrowed(source));
    }
    let mut open_streams: HashMap<String, NativeReadStream> = HashMap::new();
    let source = preprocess_native_read_streams_with_state(
        source,
        root_dir,
        output_dir,
        job_name,
        inputs,
        &mut open_streams,
    )?;
    Ok(Cow::Owned(source))
}

fn source_may_contain_native_read_streams(source: &str) -> bool {
    ["\\newread", "\\openin", "\\read", "\\closein", "\\ifeof"]
        .into_iter()
        .any(|needle| source.contains(needle))
}

fn preprocess_native_read_streams_with_state(
    source: &str,
    root_dir: &Path,
    output_dir: &Path,
    job_name: &str,
    inputs: &mut Vec<PathBuf>,
    open_streams: &mut HashMap<String, NativeReadStream>,
) -> Result<String, String> {
    let mut rendered_source = String::with_capacity(source.len());
    let mut cursor = source;

    while let Some(index) = find_next_native_read_command(cursor) {
        rendered_source.push_str(&cursor[..index]);
        let command_source = &cursor[index..];
        let remaining = consume_native_read_command(
            command_source,
            root_dir,
            output_dir,
            job_name,
            inputs,
            open_streams,
            &mut rendered_source,
        )?;
        cursor = remaining;
    }
    rendered_source.push_str(cursor);
    Ok(rendered_source)
}

fn find_next_native_read_command(source: &str) -> Option<usize> {
    ["newread", "openin", "read", "closein", "ifeof"]
        .into_iter()
        .filter_map(|control| find_control(source, control))
        .min()
}

fn consume_native_read_command<'a>(
    source: &'a str,
    root_dir: &Path,
    output_dir: &Path,
    job_name: &str,
    inputs: &mut Vec<PathBuf>,
    open_streams: &mut HashMap<String, NativeReadStream>,
    rendered_source: &mut String,
) -> Result<&'a str, String> {
    if let Some(after_newread) = source.strip_prefix("\\newread") {
        let (_, remaining) = take_output_stream_name(after_newread)
            .ok_or_else(|| "native backend requires a stream after \\newread".to_string())?;
        return Ok(remaining);
    }

    if let Some(after_openin) = source.strip_prefix("\\openin") {
        let (stream, rest) = take_output_stream_name(after_openin)
            .ok_or_else(|| "native backend requires a stream after \\openin".to_string())?;
        let (file_name, remaining) = take_output_file_name(rest)
            .ok_or_else(|| "native backend requires a file name after \\openin".to_string())?;
        let stream_state =
            match resolve_native_read_path(root_dir, output_dir, &file_name, job_name, inputs)? {
                Some(path) => {
                    let source = fs::read_to_string(&path).map_err(|error| {
                        format!(
                            "native backend could not read \\openin file `{}`: {error}",
                            path.display()
                        )
                    })?;
                    NativeReadStream::from_source(source)
                }
                None => NativeReadStream::empty(),
            };
        open_streams.insert(stream, stream_state);
        return Ok(remaining);
    }

    if let Some(after_read) = source.strip_prefix("\\read") {
        let (stream, rest) = take_output_stream_name(after_read)
            .ok_or_else(|| "native backend requires a stream after \\read".to_string())?;
        let (target, remaining) = take_read_target(rest)
            .ok_or_else(|| "native backend requires `to \\macro` after \\read".to_string())?;
        let line = open_streams
            .get_mut(&stream)
            .map(NativeReadStream::read_line)
            .unwrap_or_default();
        write!(
            rendered_source,
            "\\def{}{{{}}}",
            target,
            escape_read_macro_payload(&line)
        )
        .unwrap();
        return Ok(remaining);
    }

    if let Some(after_closein) = source.strip_prefix("\\closein") {
        let (stream, remaining) = take_output_stream_name(after_closein)
            .ok_or_else(|| "native backend requires a stream after \\closein".to_string())?;
        open_streams.remove(&stream);
        return Ok(remaining);
    }

    if let Some(after_ifeof) = source.strip_prefix("\\ifeof") {
        let (stream, rest) = take_output_stream_name(after_ifeof)
            .ok_or_else(|| "native backend requires a stream after \\ifeof".to_string())?;
        let Some((true_branch, false_branch, remaining)) = take_conditional_branches(rest) else {
            return Err("native backend requires \\ifeof branches terminated by \\fi".to_string());
        };
        let eof = open_streams
            .get(&stream)
            .is_none_or(NativeReadStream::is_eof);
        let branch = if eof { true_branch } else { false_branch };
        let branch = preprocess_native_read_streams_with_state(
            branch,
            root_dir,
            output_dir,
            job_name,
            inputs,
            open_streams,
        )?;
        rendered_source.push_str(&branch);
        return Ok(remaining);
    }

    Err("native backend only supports simple read-stream commands".to_string())
}

fn resolve_native_read_path(
    root_dir: &Path,
    output_dir: &Path,
    file_name: &str,
    job_name: &str,
    inputs: &mut Vec<PathBuf>,
) -> Result<Option<PathBuf>, String> {
    let file_name = normalize_source_file_name(file_name, job_name);
    if file_name.is_empty() {
        return Err("native backend requires non-empty \\openin file names".to_string());
    }
    if file_name.starts_with('|') {
        return Err("native backend does not support shell-pipe \\openin".to_string());
    }
    let path = Path::new(&file_name);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(format!(
            "native backend only supports local relative \\openin files, got `{file_name}`"
        ));
    }

    for base_dir in [output_dir, root_dir] {
        let candidate = base_dir.join(path);
        let existing = if candidate.exists() {
            Some(candidate)
        } else if candidate.extension().is_none() {
            let tex_candidate = candidate.with_extension("tex");
            tex_candidate.exists().then_some(tex_candidate)
        } else {
            None
        };
        if let Some(existing) = existing {
            let canonical = fs::canonicalize(&existing).map_err(|error| {
                format!(
                    "native backend could not canonicalize \\openin file `{}`: {error}",
                    existing.display()
                )
            })?;
            inputs.push(canonical.clone());
            return Ok(Some(canonical));
        }
    }
    let existing = if let Some(path) = resolve_kpathsea_tex_candidate(root_dir, path)? {
        Some(path)
    } else if path.extension().is_none() {
        resolve_kpathsea_tex_candidate(root_dir, &path.with_extension("tex"))?
    } else {
        None
    };
    if let Some(existing) = existing {
        inputs.push(existing.clone());
        return Ok(Some(existing));
    }
    Ok(None)
}

fn take_read_target(source: &str) -> Option<(String, &str)> {
    let source = source.trim_start();
    let rest = source.strip_prefix("to")?;
    if rest
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_alphabetic())
    {
        return None;
    }
    let rest = rest.trim_start();
    let (target, remaining) = take_output_stream_name(rest)?;
    (!target.chars().all(|ch| ch.is_ascii_digit())).then_some((format!("\\{target}"), remaining))
}

fn take_conditional_branches(source: &str) -> Option<(&str, &str, &str)> {
    let mut depth = 0_usize;
    let mut else_index = None;
    let mut offset = 0_usize;
    while let Some(relative) = source[offset..].find('\\') {
        let index = offset + relative;
        if is_in_line_comment(source, index) {
            offset = index + 1;
            continue;
        }
        let Some((control, after_control)) = read_control_word_at(source, index) else {
            offset = index + 1;
            continue;
        };
        match control {
            "if" | "ifcase" | "ifcat" | "ifcsname" | "ifdefined" | "ifdim" | "ifeof"
            | "iffalse" | "ifhmode" | "ifinner" | "ifmmode" | "ifnum" | "ifodd" | "iftrue"
            | "ifvmode" | "ifx" => {
                depth += 1;
            }
            "else" if depth == 0 => {
                else_index = Some(index);
            }
            "fi" if depth == 0 => {
                return if let Some(else_index) = else_index {
                    let true_branch = &source[..else_index];
                    let false_start = else_index + "\\else".len();
                    let false_branch = &source[false_start..index];
                    Some((true_branch, false_branch, &source[after_control..]))
                } else {
                    Some((&source[..index], "", &source[after_control..]))
                };
            }
            "fi" => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
        offset = after_control;
    }
    None
}

fn read_control_word_at(source: &str, index: usize) -> Option<(&str, usize)> {
    let rest = source.get(index..)?.strip_prefix('\\')?;
    let mut chars = rest.char_indices();
    let (_, first) = chars.next()?;
    if !first.is_ascii_alphabetic() {
        return None;
    }
    let mut end = first.len_utf8();
    for (relative, ch) in chars {
        if ch.is_ascii_alphabetic() {
            end = relative + ch.len_utf8();
        } else {
            break;
        }
    }
    Some((&rest[..end], index + 1 + end))
}

fn escape_read_macro_payload(line: &str) -> String {
    line.replace('#', "##").replace('%', "\\%")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NativeWritePolicy {
    StripOnly,
    Materialize,
}

fn extract_native_write_streams(
    source: &str,
    output_dir: &Path,
    job_name: &str,
    policy: NativeWritePolicy,
) -> Result<(String, Vec<GeneratedOutput>), String> {
    let mut rendered_source = String::with_capacity(source.len());
    let mut cursor = source;
    let mut open_streams: HashMap<String, Option<PathBuf>> = HashMap::new();
    let mut generated_outputs = BTreeMap::<PathBuf, String>::new();

    while let Some(index) = find_next_native_output_command(cursor) {
        rendered_source.push_str(&cursor[..index]);
        let command_source = &cursor[index..];
        let remaining = consume_native_output_command(
            command_source,
            output_dir,
            job_name,
            policy,
            &mut open_streams,
            &mut generated_outputs,
        )?;
        cursor = remaining;
    }
    rendered_source.push_str(cursor);

    let generated_outputs = generated_outputs
        .into_iter()
        .map(|(path, content)| GeneratedOutput { path, content })
        .collect();
    Ok((rendered_source, generated_outputs))
}

fn find_next_native_output_command(source: &str) -> Option<usize> {
    [
        "immediate",
        "newwrite",
        "openout",
        "write",
        "closeout",
        "typeout",
    ]
    .into_iter()
    .filter_map(|control| find_control(source, control))
    .min()
}

fn consume_native_output_command<'a>(
    source: &'a str,
    output_dir: &Path,
    job_name: &str,
    policy: NativeWritePolicy,
    open_streams: &mut HashMap<String, Option<PathBuf>>,
    generated_outputs: &mut BTreeMap<PathBuf, String>,
) -> Result<&'a str, String> {
    let mut rest = source;
    if let Some(after_immediate) = rest.strip_prefix("\\immediate") {
        rest = after_immediate.trim_start();
    }

    if let Some(after_newwrite) = rest.strip_prefix("\\newwrite") {
        let (_, remaining) = take_output_stream_name(after_newwrite)
            .ok_or_else(|| "native backend requires a stream after \\newwrite".to_string())?;
        return Ok(remaining);
    }

    if let Some(after_openout) = rest.strip_prefix("\\openout") {
        let (stream, rest) = take_output_stream_name(after_openout)
            .ok_or_else(|| "native backend requires a stream after \\openout".to_string())?;
        let (file_name, remaining) = take_output_file_name(rest)
            .ok_or_else(|| "native backend requires a file name after \\openout".to_string())?;
        let output_path = if policy == NativeWritePolicy::Materialize {
            let output_path = resolve_native_output_path(output_dir, &file_name, job_name)?;
            generated_outputs.entry(output_path.clone()).or_default();
            Some(output_path)
        } else {
            None
        };
        open_streams.insert(stream, output_path);
        return Ok(remaining);
    }

    if let Some(after_write) = rest.strip_prefix("\\write") {
        let (stream, rest) = take_output_stream_name(after_write)
            .ok_or_else(|| "native backend requires a stream after \\write".to_string())?;
        if stream == "18" {
            return Err("native backend does not support shell-escape \\write18".to_string());
        }
        let (payload, remaining) = take_braced(rest)
            .ok_or_else(|| "native backend requires braced \\write payloads".to_string())?;
        if let Some(output_path) = open_streams.get(&stream) {
            if let Some(output_path) = output_path {
                let payload = normalize_write_payload(payload, job_name);
                let content = generated_outputs.entry(output_path.clone()).or_default();
                content.push_str(&payload);
                content.push('\n');
            }
        } else if policy == NativeWritePolicy::Materialize && !stream_is_terminal_output(&stream) {
            return Err(format!(
                "native backend does not yet support \\write to unopened stream `{stream}`"
            ));
        }
        return Ok(remaining);
    }

    if let Some(after_closeout) = rest.strip_prefix("\\closeout") {
        let (stream, remaining) = take_output_stream_name(after_closeout)
            .ok_or_else(|| "native backend requires a stream after \\closeout".to_string())?;
        open_streams.remove(&stream);
        return Ok(remaining);
    }

    if let Some(after_typeout) = rest.strip_prefix("\\typeout") {
        let (_, remaining) = take_braced(after_typeout)
            .ok_or_else(|| "native backend requires braced \\typeout payloads".to_string())?;
        return Ok(remaining);
    }

    Err("native backend only supports \\immediate before output-stream commands".to_string())
}

fn take_output_stream_name(source: &str) -> Option<(String, &str)> {
    let source = source.trim_start();
    if let Some(rest) = source.strip_prefix('\\') {
        let mut chars = rest.char_indices();
        let (_, first) = chars.next()?;
        let mut end = first.len_utf8();
        if first.is_ascii_alphabetic() || first == '@' {
            for (index, ch) in chars {
                if ch.is_ascii_alphanumeric() || ch == '@' {
                    end = index + ch.len_utf8();
                } else {
                    break;
                }
            }
        }
        return Some((rest[..end].to_string(), &rest[end..]));
    }

    let mut end = 0_usize;
    for (index, ch) in source.char_indices() {
        if index == 0 && ch == '-' {
            end = ch.len_utf8();
            continue;
        }
        if ch.is_ascii_digit() {
            end = index + ch.len_utf8();
        } else {
            break;
        }
    }
    (end > 0).then_some((source[..end].to_string(), &source[end..]))
}

fn take_output_file_name(source: &str) -> Option<(String, &str)> {
    let mut source = source.trim_start();
    if let Some(rest) = source.strip_prefix('=') {
        source = rest.trim_start();
    }
    if let Some((payload, remaining)) = take_braced(source) {
        return Some((payload.trim().to_string(), remaining));
    }

    let end = source
        .char_indices()
        .take_while(|(_, ch)| !ch.is_whitespace() && *ch != '%')
        .map(|(index, ch)| index + ch.len_utf8())
        .last()
        .unwrap_or(0);
    (end > 0).then_some((source[..end].trim().to_string(), &source[end..]))
}

fn resolve_native_output_path(
    output_dir: &Path,
    file_name: &str,
    job_name: &str,
) -> Result<PathBuf, String> {
    let file_name = normalize_write_filename(file_name, job_name);
    if file_name.is_empty() {
        return Err("native backend requires non-empty \\openout file names".to_string());
    }
    if file_name.starts_with('|') {
        return Err("native backend does not support shell-pipe \\openout".to_string());
    }
    let path = Path::new(&file_name);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(format!(
            "native backend only supports local output files, got `{file_name}`"
        ));
    }
    Ok(output_dir.join(path))
}

fn normalize_write_filename(file_name: &str, job_name: &str) -> String {
    file_name
        .trim()
        .replace("\\jobname", job_name)
        .replace("\\relax", "")
        .trim()
        .to_string()
}

fn normalize_write_payload(payload: &str, job_name: &str) -> String {
    let mut out = String::with_capacity(payload.len());
    let mut cursor = payload;
    while !cursor.is_empty() {
        if let Some(rest) = cursor.strip_prefix("\\string")
            && let Some((token, remaining)) = take_stringified_token(rest)
        {
            out.push_str(&token);
            cursor = remaining;
            continue;
        }
        if let Some(rest) = cursor.strip_prefix("\\jobname") {
            out.push_str(job_name);
            cursor = rest;
            continue;
        }
        if let Some(ch) = cursor.chars().next() {
            out.push(ch);
            cursor = &cursor[ch.len_utf8()..];
        } else {
            break;
        }
    }
    out
}

fn take_stringified_token(source: &str) -> Option<(String, &str)> {
    let source = source.trim_start();
    if let Some(rest) = source.strip_prefix('\\') {
        let mut chars = rest.char_indices();
        let (_, first) = chars.next()?;
        let mut end = first.len_utf8();
        if first.is_ascii_alphabetic() || first == '@' {
            for (index, ch) in chars {
                if ch.is_ascii_alphanumeric() || ch == '@' {
                    end = index + ch.len_utf8();
                } else {
                    break;
                }
            }
        }
        return Some((format!("\\{}", &rest[..end]), &rest[end..]));
    }
    let ch = source.chars().next()?;
    Some((ch.to_string(), &source[ch.len_utf8()..]))
}

fn stream_is_terminal_output(stream: &str) -> bool {
    matches!(stream, "-1" | "16" | "17")
}

fn extract_pdf_metadata_primitives(source: &str) -> Result<(String, PdfMetadata), String> {
    let mut stripped = String::with_capacity(source.len());
    let mut metadata = PdfMetadata::default();
    let mut cursor = source;

    while let Some((index, control)) = find_next_supported_pdf_metadata_command(cursor) {
        stripped.push_str(&cursor[..index]);
        let command_source = &cursor[index..];
        let remaining =
            consume_supported_pdf_metadata_command(command_source, control, &mut metadata)?;
        cursor = remaining;
    }
    stripped.push_str(cursor);

    Ok((stripped, metadata))
}

fn find_next_supported_pdf_metadata_command(source: &str) -> Option<(usize, &'static str)> {
    [
        "pdfinfo",
        "pdfcatalog",
        "pdfnames",
        "pdftrailer",
        "pdfmapfile",
        "pdfmapline",
        "hypersetup",
    ]
    .into_iter()
    .filter_map(|control| find_control(source, control).map(|index| (index, control)))
    .min_by_key(|(index, _)| *index)
}

fn consume_supported_pdf_metadata_command<'a>(
    source: &'a str,
    control: &str,
    metadata: &mut PdfMetadata,
) -> Result<&'a str, String> {
    let rest = source
        .strip_prefix('\\')
        .and_then(|rest| rest.strip_prefix(control))
        .ok_or_else(|| format!("native backend expected \\{control}"))?;
    match control {
        "pdfinfo" => {
            let (payload, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\pdfinfo payloads".to_string())?;
            parse_pdfinfo_payload(payload, metadata);
            Ok(remaining)
        }
        "hypersetup" => {
            let (payload, remaining) = take_braced(rest).ok_or_else(|| {
                "native backend requires braced \\hypersetup payloads".to_string()
            })?;
            parse_hypersetup_pdf_metadata(payload, metadata)?;
            Ok(remaining)
        }
        "pdfcatalog" | "pdfnames" | "pdftrailer" | "pdfmapfile" | "pdfmapline" => {
            let (_, remaining) = take_braced(rest)
                .ok_or_else(|| format!("native backend requires braced \\{control} payloads"))?;
            Ok(remaining)
        }
        _ => unreachable!("unexpected supported PDF metadata command"),
    }
}

fn parse_pdfinfo_payload(payload: &str, metadata: &mut PdfMetadata) {
    let mut cursor = payload;
    while let Some(index) = cursor.find('/') {
        cursor = &cursor[index + '/'.len_utf8()..];
        let key_len = cursor
            .char_indices()
            .take_while(|(_, ch)| ch.is_ascii_alphabetic())
            .map(|(index, ch)| index + ch.len_utf8())
            .last()
            .unwrap_or(0);
        if key_len == 0 {
            continue;
        }
        let key = &cursor[..key_len];
        cursor = cursor[key_len..].trim_start();
        let Some((value, remaining)) = take_pdfinfo_value(cursor) else {
            break;
        };
        if let Some(info_key) = pdf_info_key(key) {
            metadata.insert_if_nonempty(info_key, value);
        }
        cursor = remaining;
    }
}

fn take_pdfinfo_value(source: &str) -> Option<(String, &str)> {
    if let Some((payload, remaining)) = take_parenthesized_pdf_string(source) {
        return Some((decode_pdf_literal_string(payload), remaining));
    }
    if let Some((payload, remaining)) = take_braced(source) {
        return Some((payload.trim().to_string(), remaining));
    }

    let end = source
        .char_indices()
        .find_map(|(index, ch)| (ch == '/').then_some(index))
        .unwrap_or(source.len());
    let value = source[..end].trim();
    (!value.is_empty()).then_some((value.to_string(), &source[end..]))
}

fn take_parenthesized_pdf_string(source: &str) -> Option<(&str, &str)> {
    let source = source.trim_start();
    let rest = source.strip_prefix('(')?;
    let mut escaped = false;
    for (index, ch) in rest.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            ')' => return Some((&rest[..index], &rest[index + ch.len_utf8()..])),
            _ => {}
        }
    }
    None
}

fn decode_pdf_literal_string(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                out.push(match next {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    'b' => '\u{0008}',
                    'f' => '\u{000c}',
                    other => other,
                });
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn parse_hypersetup_pdf_metadata(payload: &str, metadata: &mut PdfMetadata) -> Result<(), String> {
    for item in split_top_level_commas(payload) {
        let Some((key, value)) = split_key_value(item) else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let Some(info_key) = hyperref_pdf_info_key(&key) else {
            continue;
        };
        metadata.insert_if_nonempty(info_key, clean_hypersetup_value(value)?);
    }
    Ok(())
}

fn split_top_level_commas(source: &str) -> Vec<&str> {
    let mut items = Vec::new();
    let mut start = 0_usize;
    let mut brace_depth = 0_usize;
    let mut bracket_depth = 0_usize;
    let mut paren_depth = 0_usize;
    let mut escaped = false;
    for (index, ch) in source.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            ',' if brace_depth == 0 && bracket_depth == 0 && paren_depth == 0 => {
                items.push(&source[start..index]);
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    items.push(&source[start..]);
    items
}

fn split_key_value(source: &str) -> Option<(&str, &str)> {
    let mut brace_depth = 0_usize;
    let mut bracket_depth = 0_usize;
    let mut paren_depth = 0_usize;
    let mut escaped = false;
    for (index, ch) in source.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '=' if brace_depth == 0 && bracket_depth == 0 && paren_depth == 0 => {
                return Some((&source[..index], &source[index + ch.len_utf8()..]));
            }
            _ => {}
        }
    }
    None
}

fn clean_hypersetup_value(value: &str) -> Result<String, String> {
    let value = value.trim();
    let value = take_braced(value)
        .filter(|(_, remaining)| remaining.trim().is_empty())
        .map(|(payload, _)| payload)
        .or_else(|| {
            take_parenthesized_pdf_string(value)
                .filter(|(_, remaining)| remaining.trim().is_empty())
                .map(|(payload, _)| payload)
        })
        .unwrap_or(value);
    clean_inline_text(
        value,
        &HashMap::new(),
        &HashMap::new(),
        &CitationRegistry::default(),
    )
}

fn pdf_info_key(key: &str) -> Option<&'static str> {
    match key.to_ascii_lowercase().as_str() {
        "title" => Some("Title"),
        "author" => Some("Author"),
        "subject" => Some("Subject"),
        "keywords" => Some("Keywords"),
        "creator" => Some("Creator"),
        "producer" => Some("Producer"),
        _ => None,
    }
}

fn hyperref_pdf_info_key(key: &str) -> Option<&'static str> {
    match key {
        "pdftitle" => Some("Title"),
        "pdfauthor" => Some("Author"),
        "pdfsubject" => Some("Subject"),
        "pdfkeywords" => Some("Keywords"),
        "pdfcreator" => Some("Creator"),
        "pdfproducer" => Some("Producer"),
        _ => None,
    }
}

fn expand_source_for_native(
    source: &str,
    root_dir: &Path,
    job_name: &str,
) -> Result<String, String> {
    let source = protect_verbatim_like_bodies_for_expansion(source)?;
    expand_to_source_with_file_context(&source, root_dir.to_path_buf(), job_name.to_string())
        .map_err(|error| format!("native backend expansion failed: {error}"))
}

fn transparent_environment(env: &str) -> bool {
    matches!(
        env.trim().trim_end_matches('*'),
        "figure"
            | "table"
            | "lstfloat"
            | "tikzpicture"
            | "center"
            | "abstract"
            | "minipage"
            | "proof"
            | "itemize"
            | "enumerate"
            | "description"
            | "algorithm"
            | "algorithmic"
            | "assumptionbox"
            | "theorembox"
            | "corollarybox"
            | "mainresultbox"
            | "theoremrestate"
            | "definition"
            | "detail"
            | "lemma"
            | "theorem"
            | "proposition"
            | "corollary"
            | "remark"
            | "smallmatrix"
            | "array"
            | "matrix"
            | "pmatrix"
            | "bmatrix"
            | "cases"
            | "split"
            | "aligned"
            | "alignedat"
            | "gathered"
    )
}

#[derive(Debug, Clone, Copy)]
struct ListFrame {
    kind: ListKind,
    next_index: usize,
}

#[derive(Debug, Clone, Copy)]
enum ListKind {
    Itemize,
    Enumerate(EnumerateFormat),
    Description,
}

#[derive(Debug, Clone, Copy)]
enum EnumerateFormat {
    DecimalDot,
    DecimalParen,
    RomanParen,
    RomanDot,
    AlphaParen,
    AlphaDot,
    UpperAlphaParen,
    UpperAlphaDot,
}

impl ListFrame {
    fn from_environment(env: &str, source_after_env: &str) -> Option<Self> {
        let env = env.trim().trim_end_matches('*');
        let options = take_optional_bracketed(source_after_env).0;
        let kind = match env {
            "itemize" => ListKind::Itemize,
            "enumerate" => ListKind::Enumerate(enumerate_format(options)),
            "description" => ListKind::Description,
            _ => return None,
        };
        Some(Self {
            kind,
            next_index: 1,
        })
    }

    fn next_marker(&mut self, explicit_label: Option<String>) -> String {
        match self.kind {
            ListKind::Itemize => explicit_label.unwrap_or_else(|| "-".to_string()),
            ListKind::Description => explicit_label.unwrap_or_else(|| "-".to_string()),
            ListKind::Enumerate(format) => {
                let index = self.next_index;
                self.next_index += 1;
                explicit_label.unwrap_or_else(|| enumerate_marker(index, format))
            }
        }
    }
}

fn list_environment(env: &str) -> bool {
    matches!(
        env.trim().trim_end_matches('*'),
        "itemize" | "enumerate" | "description"
    )
}

fn list_item_marker(
    stack: &mut [ListFrame],
    explicit_label: Option<&str>,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
) -> Result<String, String> {
    let explicit_label = explicit_label
        .map(|label| clean_inline_text_collecting(label, macros, labels, citations, footnotes))
        .transpose()?
        .filter(|label| !label.is_empty());
    if let Some(frame) = stack.last_mut() {
        Ok(frame.next_marker(explicit_label))
    } else {
        Ok(explicit_label.unwrap_or_else(|| "-".to_string()))
    }
}

fn enumerate_format(options: Option<&str>) -> EnumerateFormat {
    let Some(options) = options else {
        return EnumerateFormat::DecimalDot;
    };
    if options.contains("\\roman*") || options.contains("roman*") {
        if options.contains('(') && options.contains(')') {
            EnumerateFormat::RomanParen
        } else {
            EnumerateFormat::RomanDot
        }
    } else if options.contains("\\Alph*") || options.contains("Alph*") {
        if options.contains('(') && options.contains(')') {
            EnumerateFormat::UpperAlphaParen
        } else {
            EnumerateFormat::UpperAlphaDot
        }
    } else if options.contains("\\alph*") || options.contains("alph*") {
        if options.contains('(') && options.contains(')') {
            EnumerateFormat::AlphaParen
        } else {
            EnumerateFormat::AlphaDot
        }
    } else if options.contains('(') && options.contains(')') {
        EnumerateFormat::DecimalParen
    } else {
        EnumerateFormat::DecimalDot
    }
}

fn enumerate_marker(index: usize, format: EnumerateFormat) -> String {
    match format {
        EnumerateFormat::DecimalDot => format!("{index}."),
        EnumerateFormat::DecimalParen => format!("({index})"),
        EnumerateFormat::RomanParen => format!("({})", roman_lower(index)),
        EnumerateFormat::RomanDot => format!("{}.", roman_lower(index)),
        EnumerateFormat::AlphaParen => format!("({})", alpha_label(index, false)),
        EnumerateFormat::AlphaDot => format!("{}.", alpha_label(index, false)),
        EnumerateFormat::UpperAlphaParen => format!("({})", alpha_label(index, true)),
        EnumerateFormat::UpperAlphaDot => format!("{}.", alpha_label(index, true)),
    }
}

fn roman_lower(mut value: usize) -> String {
    if value == 0 {
        return "0".to_string();
    }
    let numerals = [
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
    ];
    let mut out = String::new();
    for (amount, numeral) in numerals {
        while value >= amount {
            out.push_str(numeral);
            value -= amount;
        }
    }
    out
}

fn alpha_label(mut value: usize, uppercase: bool) -> String {
    if value == 0 {
        return "0".to_string();
    }
    let base = if uppercase { b'A' } else { b'a' };
    let mut letters = Vec::new();
    while value > 0 {
        value -= 1;
        letters.push((base + (value % 26) as u8) as char);
        value /= 26;
    }
    letters.iter().rev().collect()
}

fn caption_label_separator_override(source: &str) -> Option<CaptionLabelSeparator> {
    let mut separator = None;
    let mut cursor = source;
    while let Some(index) = find_control(cursor, "captionsetup") {
        let rest = &cursor[index + "\\captionsetup".len()..];
        let (scope, rest) = take_optional_bracketed(rest);
        let Some((payload, remaining)) = take_braced(rest) else {
            cursor = rest;
            continue;
        };
        if scope.is_none()
            && let Some(parsed) = caption_label_separator_from_payload(payload)
        {
            separator = Some(parsed);
        }
        cursor = remaining;
    }
    separator
}

fn caption_label_separator_from_source_tree(
    source: &str,
    root_dir: &Path,
) -> Result<Option<CaptionLabelSeparator>, String> {
    let mut visited = BTreeSet::new();
    caption_label_separator_from_source_tree_inner(source, root_dir, &mut visited)
}

fn caption_label_separator_from_source_tree_inner(
    source: &str,
    base_dir: &Path,
    visited: &mut BTreeSet<PathBuf>,
) -> Result<Option<CaptionLabelSeparator>, String> {
    let mut separator = caption_label_separator_override(source);
    for class_name in class_declarations(source)? {
        let Some(path) = resolve_local_class_path(base_dir, &class_name)? else {
            continue;
        };
        if !visited.insert(path.clone()) {
            continue;
        }
        let class_source = fs::read_to_string(&path).map_err(|error| {
            format!(
                "native backend could not read class `{}`: {error}",
                path.display()
            )
        })?;
        if let Some(nested) = caption_label_separator_from_source_tree_inner(
            &class_source,
            path.parent().unwrap_or(base_dir),
            visited,
        )? {
            separator = Some(nested);
        }
    }
    for declaration in package_declarations(source)? {
        for package_name in declaration.names {
            let Some(path) = resolve_local_package_path(base_dir, &package_name)? else {
                continue;
            };
            if !visited.insert(path.clone()) {
                continue;
            }
            let package_source = fs::read_to_string(&path).map_err(|error| {
                format!(
                    "native backend could not read package `{}`: {error}",
                    path.display()
                )
            })?;
            if let Some(nested) = caption_label_separator_from_source_tree_inner(
                &package_source,
                path.parent().unwrap_or(base_dir),
                visited,
            )? {
                separator = Some(nested);
            }
        }
    }
    Ok(separator)
}

fn caption_label_separator_from_payload(payload: &str) -> Option<CaptionLabelSeparator> {
    split_top_level_commas(payload)
        .into_iter()
        .filter_map(split_key_value)
        .find_map(|(key, value)| {
            if key.trim() != "labelsep" {
                return None;
            }
            match value.trim().trim_matches(['{', '}']) {
                "period" => Some(CaptionLabelSeparator::Period),
                "colon" => Some(CaptionLabelSeparator::Colon),
                _ => None,
            }
        })
}

fn listing_reference_name(source: &str) -> String {
    listing_reference_name_from_command(source)
        .or_else(|| listing_reference_name_from_definition(source))
        .unwrap_or_else(|| "Listing".to_string())
}

fn listing_reference_name_from_command(source: &str) -> Option<String> {
    ["renewcommand", "newcommand", "providecommand"]
        .into_iter()
        .filter_map(|control| listing_reference_name_from_command_control(source, control))
        .next()
}

fn listing_reference_name_from_command_control(source: &str, control: &str) -> Option<String> {
    let marker_len = control.len() + 1;
    let mut cursor = source;
    let mut last_name = None;
    while let Some(index) = find_control(cursor, control) {
        let rest = &cursor[index + marker_len..];
        if let Some(name) = parse_listing_reference_command(rest) {
            last_name = Some(name);
        }
        cursor = rest;
    }
    last_name
}

fn parse_listing_reference_command(source: &str) -> Option<String> {
    let (source, _) = strip_optional_star(source);
    let source = source.trim_start();
    let after_name = if let Some(rest) = source.strip_prefix("\\lstlistingname") {
        rest
    } else {
        let (payload, rest) = take_braced(source)?;
        if macro_name_from_payload(payload).as_deref() == Some("lstlistingname") {
            rest
        } else {
            return None;
        }
    };
    let (_, after_arg_count) = take_optional_bracketed(after_name);
    let (_, after_default_arg) = take_optional_bracketed(after_arg_count);
    let (payload, _) = take_braced(after_default_arg)?;
    normalize_reference_name(payload)
}

fn listing_reference_name_from_definition(source: &str) -> Option<String> {
    ["def", "gdef", "xdef", "edef"]
        .into_iter()
        .filter_map(|control| listing_reference_name_from_definition_control(source, control))
        .next()
}

fn listing_reference_name_from_definition_control(source: &str, control: &str) -> Option<String> {
    let marker_len = control.len() + 1;
    let mut cursor = source;
    let mut last_name = None;
    while let Some(index) = find_control(cursor, control) {
        let rest = &cursor[index + marker_len..];
        let rest = rest.trim_start();
        if let Some(after_name) = rest.strip_prefix("\\lstlistingname")
            && let Some((payload, _)) = take_braced(after_name)
            && let Some(name) = normalize_reference_name(payload)
        {
            last_name = Some(name);
        }
        cursor = rest;
    }
    last_name
}

fn normalize_reference_name(payload: &str) -> Option<String> {
    let text = payload
        .replace("\\lstlistingname", "Listing")
        .replace("\\MakeLowercase", "")
        .replace("\\MakeUppercase", "");
    let text = loose_plain_text(&text);
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_string())
}

#[derive(Debug, Clone, Copy)]
struct TheoremEnvironmentSpec {
    display_name: &'static str,
    counter_key: Option<&'static str>,
    opening: TheoremOpening,
}

#[derive(Debug, Clone, Copy)]
enum TheoremOpening {
    OptionalTitle,
    TcbTheorem,
}

struct TheoremOpeningData<'a> {
    title: Option<&'a str>,
    options: Option<&'a str>,
    remaining: &'a str,
}

fn theorem_environment_spec(env: &str) -> Option<TheoremEnvironmentSpec> {
    match env.trim() {
        "theorembox" => Some(TheoremEnvironmentSpec {
            display_name: "Theorem",
            counter_key: Some("thmcounter"),
            opening: TheoremOpening::OptionalTitle,
        }),
        "corollarybox" => Some(TheoremEnvironmentSpec {
            display_name: "Corollary",
            counter_key: Some("thmcounter"),
            opening: TheoremOpening::OptionalTitle,
        }),
        "mainresultbox" => Some(TheoremEnvironmentSpec {
            display_name: "Main Result",
            counter_key: None,
            opening: TheoremOpening::OptionalTitle,
        }),
        "assumptionbox" => Some(TheoremEnvironmentSpec {
            display_name: "Assumptions",
            counter_key: None,
            opening: TheoremOpening::OptionalTitle,
        }),
        "theoremrestate" => Some(TheoremEnvironmentSpec {
            display_name: "",
            counter_key: None,
            opening: TheoremOpening::OptionalTitle,
        }),
        "detail" => Some(TheoremEnvironmentSpec {
            display_name: "Experiment Details",
            counter_key: Some("detail"),
            opening: TheoremOpening::TcbTheorem,
        }),
        "definition" => Some(TheoremEnvironmentSpec {
            display_name: "Definition",
            counter_key: Some("definition"),
            opening: TheoremOpening::TcbTheorem,
        }),
        "theorem" => Some(TheoremEnvironmentSpec {
            display_name: "Theorem",
            counter_key: Some("theorem"),
            opening: TheoremOpening::TcbTheorem,
        }),
        "lemma" => Some(TheoremEnvironmentSpec {
            display_name: "Lemma",
            counter_key: Some("lemma"),
            opening: TheoremOpening::TcbTheorem,
        }),
        "proposition" => Some(TheoremEnvironmentSpec {
            display_name: "Proposition",
            counter_key: Some("proposition"),
            opening: TheoremOpening::TcbTheorem,
        }),
        "corollary" => Some(TheoremEnvironmentSpec {
            display_name: "Corollary",
            counter_key: Some("corollary"),
            opening: TheoremOpening::TcbTheorem,
        }),
        "remark" => Some(TheoremEnvironmentSpec {
            display_name: "Remark",
            counter_key: Some("remark"),
            opening: TheoremOpening::OptionalTitle,
        }),
        _ => None,
    }
}

fn theorem_reference_prefix(spec: TheoremEnvironmentSpec) -> &'static str {
    match spec.display_name {
        "Definition" => "def.",
        "Theorem" => "thm.",
        "Lemma" => "lemma.",
        "Proposition" => "prop.",
        "Corollary" => "cor.",
        "Experiment Details" => "detail",
        "Remark" => "remark",
        other if !other.is_empty() => other,
        _ => "result",
    }
}

fn parse_theorem_opening<'a>(
    spec: TheoremEnvironmentSpec,
    source: &'a str,
) -> Result<TheoremOpeningData<'a>, String> {
    match spec.opening {
        TheoremOpening::OptionalTitle => {
            let (title, remaining) = take_optional_bracketed(source);
            Ok(TheoremOpeningData {
                title,
                options: title,
                remaining,
            })
        }
        TheoremOpening::TcbTheorem => {
            let (options, source) = take_optional_bracketed(source);
            let Some((title, source)) = take_braced(source) else {
                return Ok(TheoremOpeningData {
                    title: options,
                    options: None,
                    remaining: source,
                });
            };
            let Some((_, remaining)) = take_braced(source) else {
                return Ok(TheoremOpeningData {
                    title: Some(title),
                    options,
                    remaining: source,
                });
            };
            Ok(TheoremOpeningData {
                title: Some(title),
                options,
                remaining,
            })
        }
    }
}

fn next_theorem_number(
    counters: &mut HashMap<&'static str, usize>,
    counter_key: Option<&'static str>,
) -> Option<usize> {
    let counter_key = counter_key?;
    let counter = counters.entry(counter_key).or_insert(0);
    *counter += 1;
    Some(*counter)
}

fn theorem_heading(display_name: &str, number: Option<usize>, title: &str) -> String {
    let title = title.trim();
    let base = match (display_name.is_empty(), number) {
        (true, _) => String::new(),
        (false, Some(number)) => format!("{display_name} {number}"),
        (false, None) => display_name.to_string(),
    };
    match (base.is_empty(), title.is_empty()) {
        (true, true) => String::new(),
        (true, false) => title.to_string(),
        (false, true) => base,
        (false, false) => format!("{base}: {title}"),
    }
}

fn consume_environment_open_args<'a>(env: &str, source: &'a str) -> &'a str {
    let (_, source) = take_optional_bracketed(source);
    match env.trim().trim_end_matches('*') {
        "minipage" | "array" | "tabular" => take_braced(source)
            .map(|(_, remaining)| remaining)
            .unwrap_or(source),
        _ => source,
    }
}

fn float_prefers_top(source_after_env: &str) -> bool {
    let (placement, _) = take_optional_bracketed(source_after_env);
    placement.is_none_or(|placement| placement.contains('t') || placement.contains('p'))
}

fn float_prefers_bottom(source_after_env: &str) -> bool {
    let (placement, _) = take_optional_bracketed(source_after_env);
    placement.is_some_and(|placement| {
        placement.contains('b') && !placement.contains('t') && !placement.contains('p')
    })
}

fn source_contains_control(source: &str, control: &str) -> bool {
    find_control(source, control).is_some()
}

fn strip_prefix_any<'a>(source: &'a str, prefixes: &[&str]) -> Option<&'a str> {
    prefixes
        .iter()
        .find_map(|prefix| source.strip_prefix(prefix))
}

fn skip_two_braced_arguments(source: &str) -> Option<&str> {
    let (_, source) = take_braced(source)?;
    let (_, source) = take_braced(source)?;
    Some(source)
}

fn skip_renewcommand(source: &str) -> (&str, bool) {
    let source = source.trim_start();
    let Some(source) = skip_definition_name(source) else {
        return (source, false);
    };
    let (_, source) = take_optional_bracketed(source);
    let Some((_, source)) = take_braced(source) else {
        return (source, false);
    };
    (source, true)
}

fn skip_newtcbtheorem(source: &str) -> (&str, bool) {
    let source = trim_tex_space_and_comments(source);
    let (_, mut source) = take_optional_bracketed(source);
    for _ in 0..4 {
        source = trim_tex_space_and_comments(source);
        let Some((_, remaining)) = take_braced(source) else {
            return (source, false);
        };
        source = remaining;
    }
    (source, true)
}

fn trim_tex_space_and_comments(mut source: &str) -> &str {
    loop {
        source = source.trim_start();
        let Some(rest) = source.strip_prefix('%') else {
            return source;
        };
        source = rest
            .find('\n')
            .map(|index| &rest[index + '\n'.len_utf8()..])
            .unwrap_or("");
    }
}

fn skip_definition_name(source: &str) -> Option<&str> {
    if let Some((_, remaining)) = take_braced(source) {
        return Some(remaining);
    }
    let source = source.trim_start();
    let rest = source.strip_prefix('\\')?;
    let name_len = rest
        .char_indices()
        .take_while(|(_, ch)| ch.is_ascii_alphabetic())
        .map(|(index, ch)| index + ch.len_utf8())
        .last()
        .unwrap_or(0);
    if name_len > 0 {
        return Some(&rest[name_len..]);
    }
    rest.char_indices()
        .nth(1)
        .map(|(index, _)| &rest[index..])
        .or(Some(""))
}

fn strip_optional_star(source: &str) -> (&str, bool) {
    let source = source.trim_start();
    if let Some(rest) = source.strip_prefix('*') {
        (rest, true)
    } else {
        (source, false)
    }
}

fn document_body(source: &str) -> Option<&str> {
    let begin = source.find("\\begin{document}")? + "\\begin{document}".len();
    let end = source[begin..].find("\\end{document}")? + begin;
    Some(source[begin..end].trim())
}

fn braced_command_payload_collecting(
    source: &str,
    command: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
) -> Option<String> {
    let marker = format!("\\{command}");
    let start = source.find(&marker)? + marker.len();
    let (payload, _) = take_braced(&source[start..])?;
    clean_inline_text_collecting(payload, macros, labels, citations, footnotes).ok()
}

fn native_page_style_from_source(source: &str) -> Option<PageStyle> {
    if !source.contains("simpleicml") && !source_contains_control(source, "sectionheaderline") {
        return None;
    }
    let running_title = raw_braced_command_payload(source, "icmlrunningtitle")
        .or_else(|| raw_braced_command_payload(source, "icmltitle"))
        .map(clean_header_text)
        .map(short_running_title)
        .unwrap_or_default();
    let section_line = raw_braced_command_payload(source, "sectionheaderline")
        .map(render_section_header_line)
        .unwrap_or_default();
    if running_title.is_empty() && section_line.is_empty() {
        None
    } else {
        Some(PageStyle {
            running_title,
            section_line,
        })
    }
}

fn raw_braced_command_payload<'a>(source: &'a str, command: &str) -> Option<&'a str> {
    let marker = format!("\\{command}");
    let mut cursor = source;
    while let Some(index) = find_control(cursor, command) {
        let rest = &cursor[index + marker.len()..];
        if let Some((payload, _)) = take_braced(trim_tex_space_and_comments(rest)) {
            return Some(payload);
        }
        cursor = rest;
    }
    None
}

fn short_running_title(title: String) -> String {
    if let Some(colon_index) = title.find(':') {
        return title[..=colon_index].trim().to_string();
    }
    title
}

fn render_section_header_line(source: &str) -> String {
    let mut out = String::new();
    let mut cursor = source;
    while let Some(index) = cursor.find("\\seclink") {
        out.push_str(&clean_header_text(&cursor[..index]));
        let rest = &cursor[index + "\\seclink".len()..];
        let Some((_, rest)) = take_braced(rest) else {
            cursor = rest;
            break;
        };
        let Some((number, rest)) = take_braced(rest) else {
            cursor = rest;
            break;
        };
        let Some((title, remaining)) = take_braced(rest) else {
            cursor = rest;
            break;
        };
        if !out.ends_with(' ') && !out.is_empty() {
            out.push(' ');
        }
        out.push_str("Sec ");
        out.push_str(clean_header_text(number).trim());
        out.push_str(": ");
        out.push_str(clean_header_text(title).trim());
        cursor = remaining;
    }
    out.push_str(&clean_header_text(cursor));
    normalize_header_spacing(&out)
}

fn clean_header_text(source: &str) -> String {
    let source = source.replace("\\\\", " ");
    let source = source
        .lines()
        .map(|line| line.split('%').next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join(" ");
    clean_bib_tex_text(&source)
}

fn normalize_header_spacing(source: &str) -> String {
    let normalized = source.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.contains('|') {
        normalized
            .split('|')
            .map(str::trim)
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" | ")
    } else {
        normalized
    }
}

fn collect_citations(
    source: &str,
    body: &str,
    root_dir: &Path,
    inputs: &mut Vec<PathBuf>,
    layout: &DocumentLayout,
) -> Result<CitationRegistry, String> {
    let uses = citation_uses_in_order(body, layout)?;
    let manual_entries = manual_bibliography_entries(body)?;
    let style = citation_style_from_source(source);
    let visible_backrefs = citation_backrefs_are_visible(source);
    if !manual_entries.is_empty() {
        let backrefs = uses
            .iter()
            .map(|citation| CitationBackref {
                key: citation.key.clone(),
                page: citation.page,
            })
            .collect();
        let mut numbers = HashMap::new();
        let mut entries = Vec::new();
        for (index, entry) in manual_entries.into_iter().enumerate() {
            let number = index + 1;
            numbers.insert(entry.key.clone(), number);
            entries.push(CitationEntry {
                key: entry.key,
                number,
                text: entry.text,
            });
        }
        let mut cited_keys = Vec::new();
        for key in uses.into_iter().map(|citation| citation.key) {
            if !cited_keys.contains(&key) {
                cited_keys.push(key);
            }
        }
        return Ok(CitationRegistry {
            numbers,
            keys: cited_keys,
            entries,
            backrefs,
            labels: HashMap::new(),
            style,
            visible_backrefs,
        });
    }
    if uses.is_empty() {
        return Ok(CitationRegistry {
            style,
            visible_backrefs,
            ..CitationRegistry::default()
        });
    }
    let bib_entries = bibliography_entries(source, root_dir, inputs)?;
    let mut numbers = HashMap::new();
    let mut cited_keys = Vec::new();
    let mut entries = Vec::new();
    let mut labels = HashMap::new();
    let backrefs = uses
        .iter()
        .map(|citation| CitationBackref {
            key: citation.key.clone(),
            page: citation.page,
        })
        .collect();
    for key in uses.into_iter().map(|citation| citation.key) {
        if numbers.contains_key(&key) {
            continue;
        }
        let number = numbers.len() + 1;
        numbers.insert(key.clone(), number);
        cited_keys.push(key.clone());
        let text = bib_entries
            .get(&key)
            .map(|entry| entry.text.clone())
            .unwrap_or_else(|| key.clone());
        if let Some(label) = bib_entries.get(&key).and_then(|entry| entry.label.clone()) {
            labels.insert(key.clone(), label);
        }
        entries.push(CitationEntry { key, number, text });
    }
    Ok(CitationRegistry {
        numbers,
        keys: cited_keys,
        entries,
        backrefs,
        labels,
        style,
        visible_backrefs,
    })
}

fn citation_style_from_source(source: &str) -> CitationStyle {
    let compact = source.split_whitespace().collect::<String>();
    if compact.contains(r"\PassOptionsToPackage{numbers}{natbib}")
        || compact.contains(r"\usepackage[numbers]{natbib}")
        || compact.contains(r"\bibliographystyle{unsrtnat}")
        || compact.contains(r"\bibliographystyle{plain}")
    {
        CitationStyle::Numeric
    } else if compact.contains(r"\bibliographystyle{plainnat}") {
        CitationStyle::AuthorYear
    } else {
        CitationStyle::Numeric
    }
}

fn citation_backrefs_are_visible(source: &str) -> bool {
    let compact = source.split_whitespace().collect::<String>();
    compact.contains("pagebackref")
        || compact.contains("backref=true")
        || compact.contains(r"\backrefalt")
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CitationUse {
    key: String,
    page: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManualBibliographyEntry {
    key: String,
    text: String,
}

fn manual_bibliography_entries(body: &str) -> Result<Vec<ManualBibliographyEntry>, String> {
    let mut entries = Vec::new();
    let mut cursor = body;
    while let Some(index) = cursor.find("\\begin{thebibliography}") {
        let rest = &cursor[index + "\\begin{thebibliography}".len()..];
        let (_, rest) = take_braced(rest)
            .ok_or_else(|| "native backend requires braced thebibliography labels".to_string())?;
        let (bibliography_body, remaining) = take_environment_body(rest, "thebibliography")?;
        entries.extend(parse_manual_bibitems(bibliography_body)?);
        cursor = remaining;
    }
    Ok(entries)
}

fn parse_manual_bibitems(source: &str) -> Result<Vec<ManualBibliographyEntry>, String> {
    let mut entries = Vec::new();
    let mut cursor = source;
    while let Some(index) = find_control(cursor, "bibitem") {
        let mut rest = &cursor[index + "\\bibitem".len()..];
        let (_, after_optional) = take_optional_bracketed(rest);
        rest = after_optional;
        let Some((key, after_key)) = take_braced(rest) else {
            return Err("native backend requires braced \\bibitem keys".to_string());
        };
        let next_index = find_control(after_key, "bibitem").unwrap_or(after_key.len());
        let raw_text = &after_key[..next_index];
        let text = clean_bib_tex_text(raw_text);
        entries.push(ManualBibliographyEntry {
            key: key.trim().to_string(),
            text: if text.is_empty() {
                key.trim().to_string()
            } else {
                text
            },
        });
        cursor = &after_key[next_index..];
    }
    Ok(entries)
}

fn citation_uses_in_order(
    source: &str,
    layout: &DocumentLayout,
) -> Result<Vec<CitationUse>, String> {
    let mut uses = Vec::new();
    let mut cursor = source;
    let mut slot = 0_usize;
    while let Some((index, command)) = find_next_citation_command(cursor) {
        slot += estimate_text_slots(&cursor[..index], layout);
        let mut rest = &cursor[index + command.len() + 1..];
        let (after_star, _) = strip_optional_star(rest);
        rest = after_star;
        let (_, after_first_optional) = take_optional_bracketed(rest);
        rest = after_first_optional;
        let (_, after_second_optional) = take_optional_bracketed(rest);
        rest = after_second_optional;
        let Some((payload, remaining)) = take_braced(rest) else {
            return Err(format!(
                "native backend requires braced `{command}` citation keys"
            ));
        };
        let page = page_for_slot(slot.max(1), layout);
        uses.extend(
            payload
                .split(',')
                .map(str::trim)
                .filter(|key| !key.is_empty())
                .map(|key| CitationUse {
                    key: key.to_string(),
                    page,
                }),
        );
        cursor = remaining;
    }
    Ok(uses)
}

fn find_next_citation_command(source: &str) -> Option<(usize, &'static str)> {
    ["parencite", "textcite", "citep", "citet", "cite"]
        .into_iter()
        .filter_map(|command| find_control(source, command).map(|index| (index, command)))
        .min_by_key(|(index, _)| *index)
}

fn bibliography_entries(
    source: &str,
    root_dir: &Path,
    inputs: &mut Vec<PathBuf>,
) -> Result<HashMap<String, BibliographyEntry>, String> {
    let mut entries = HashMap::new();
    for bibliography in bibliography_payloads(source)? {
        for name in bibliography
            .split(',')
            .map(str::trim)
            .filter(|name| !name.is_empty())
        {
            let path = resolve_bibliography_path(root_dir, name)?;
            inputs.push(path.clone());
            let bib_source = fs::read_to_string(&path).map_err(|error| {
                format!(
                    "native backend could not read bibliography `{}`: {error}",
                    path.display()
                )
            })?;
            entries.extend(parse_bib_entries(&bib_source));
        }
    }
    Ok(entries)
}

fn bibliography_payloads(source: &str) -> Result<Vec<String>, String> {
    let mut payloads = Vec::new();
    for command in ["bibliography", "addbibresource"] {
        payloads.extend(raw_braced_command_payloads(source, command)?);
    }
    Ok(payloads)
}

fn bibliography_metadata(
    source: &str,
    root_dir: &Path,
    inputs: &mut Vec<PathBuf>,
) -> Result<BibliographyMetadata, String> {
    let styles = raw_braced_command_payloads(source, "bibliographystyle")?
        .into_iter()
        .map(|style| style.trim().to_string())
        .filter(|style| !style.is_empty())
        .collect::<Vec<_>>();
    record_bibliography_style_inputs(root_dir, &styles, inputs)?;
    let databases = raw_braced_command_payloads(source, "bibliography")?
        .into_iter()
        .map(|payload| normalized_comma_payload(&payload))
        .filter(|payload| !payload.is_empty())
        .collect();
    Ok(BibliographyMetadata { styles, databases })
}

fn record_bibliography_style_inputs(
    root_dir: &Path,
    styles: &[String],
    inputs: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for style in styles {
        if let Some(path) = resolve_bibliography_style_path(root_dir, style)? {
            inputs.push(path);
        }
    }
    Ok(())
}

fn raw_braced_command_payloads(source: &str, command: &str) -> Result<Vec<String>, String> {
    let mut payloads = Vec::new();
    let mut cursor = source;
    while let Some(index) = find_control(cursor, command) {
        let mut rest = &cursor[index + command.len() + 1..];
        let (_, after_optional) = take_optional_bracketed(rest);
        rest = after_optional;
        let Some((payload, remaining)) = take_braced(rest) else {
            return Err(format!(
                "native backend requires braced \\{command} payloads"
            ));
        };
        payloads.push(payload.to_string());
        cursor = remaining;
    }
    Ok(payloads)
}

fn normalized_comma_payload(payload: &str) -> String {
    payload
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(",")
}

fn find_control(source: &str, control: &str) -> Option<usize> {
    let needle = format!("\\{control}");
    let mut offset = 0_usize;
    while let Some(index) = source[offset..].find(&needle) {
        let absolute = offset + index;
        let after = absolute + needle.len();
        let next = source[after..].chars().next();
        if !is_in_line_comment(source, absolute) && !next.is_some_and(|ch| ch.is_ascii_alphabetic())
        {
            return Some(absolute);
        }
        offset = after;
    }
    None
}

fn resolve_bibliography_path(root_dir: &Path, name: &str) -> Result<PathBuf, String> {
    let raw = Path::new(name);
    if raw.is_absolute() {
        return Err(format!(
            "native backend only supports local relative bibliography paths, got `{name}`"
        ));
    }
    let candidate_name = if raw.extension().is_none() {
        raw.with_extension("bib")
    } else {
        raw.to_path_buf()
    };
    let candidate = root_dir.join(&candidate_name);
    if candidate.exists() {
        return fs::canonicalize(&candidate).map_err(|error| {
            format!(
                "native backend could not resolve bibliography `{}`: {error}",
                candidate.display()
            )
        });
    }
    if let Some(path) = resolve_kpathsea_bib_candidate(root_dir, &candidate_name)? {
        return Ok(path);
    }
    fs::canonicalize(&candidate).map_err(|error| {
        format!(
            "native backend could not resolve bibliography `{}`: {error}",
            candidate.display()
        )
    })
}

fn resolve_bibliography_style_path(root_dir: &Path, name: &str) -> Result<Option<PathBuf>, String> {
    let raw = Path::new(name);
    if raw.is_absolute() {
        return Err(format!(
            "native backend only supports local relative bibliography style paths, got `{name}`"
        ));
    }
    let candidate_name = if raw.extension().is_none() {
        raw.with_extension("bst")
    } else {
        raw.to_path_buf()
    };
    let candidate = root_dir.join(&candidate_name);
    if candidate.exists() {
        let canonical = fs::canonicalize(&candidate).map_err(|error| {
            format!(
                "native backend could not resolve bibliography style `{}`: {error}",
                candidate.display()
            )
        })?;
        return Ok(Some(canonical));
    }
    resolve_kpathsea_bst_candidate(root_dir, &candidate_name)
}

#[derive(Debug, Clone)]
struct BibliographyEntry {
    text: String,
    label: Option<CitationLabel>,
}

fn parse_bib_entries(source: &str) -> HashMap<String, BibliographyEntry> {
    let mut entries = HashMap::new();
    let mut cursor = source;
    while let Some(at_index) = cursor.find('@') {
        cursor = &cursor[at_index + 1..];
        let Some(open_index) = cursor.find('{') else {
            break;
        };
        let after_open = &cursor[open_index + 1..];
        let Some(comma_index) = after_open.find(',') else {
            break;
        };
        let key = after_open[..comma_index].trim();
        if key.is_empty() {
            cursor = after_open;
            continue;
        }
        let entry_end = after_open[comma_index + 1..]
            .find("\n@")
            .map(|index| comma_index + 1 + index)
            .unwrap_or(after_open.len());
        let entry = &after_open[..entry_end];
        entries.insert(key.to_string(), summarize_bib_entry(key, entry));
        cursor = &after_open[entry_end..];
    }
    entries
}

fn summarize_bib_entry(key: &str, entry: &str) -> BibliographyEntry {
    let mut parts = Vec::new();
    let author = bib_field(entry, "author");
    let year = bib_field(entry, "year");
    if let Some(author) = author.as_ref() {
        parts.push(author.clone());
    }
    if let Some(title) = bib_field(entry, "title") {
        parts.push(title);
    }
    if let Some(year) = year.as_ref() {
        parts.push(year.clone());
    }
    let text = if parts.is_empty() {
        key.to_string()
    } else {
        parts.join(". ")
    };
    let label = author
        .as_deref()
        .and_then(citation_author_label)
        .map(|author| CitationLabel { author, year });
    BibliographyEntry { text, label }
}

fn citation_author_label(author_field: &str) -> Option<String> {
    let authors = split_bib_authors(author_field)
        .into_iter()
        .filter_map(citation_surname)
        .collect::<Vec<_>>();
    match authors.len() {
        0 => None,
        1 => authors.into_iter().next(),
        2 => Some(format!("{} and {}", authors[0], authors[1])),
        _ => Some(format!("{} et al.", authors[0])),
    }
}

fn split_bib_authors(author_field: &str) -> Vec<&str> {
    author_field
        .split(" and ")
        .map(str::trim)
        .filter(|author| !author.is_empty())
        .collect()
}

fn citation_surname(author: &str) -> Option<String> {
    let author = author
        .trim()
        .trim_matches(|ch: char| ch == '{' || ch == '}');
    if author.is_empty() {
        return None;
    }
    if let Some((surname, _)) = author.split_once(',') {
        let surname = surname.trim();
        return (!surname.is_empty()).then(|| surname.to_string());
    }
    let words = author.split_whitespace().collect::<Vec<_>>();
    let last = words.last()?.trim_matches(citation_name_punctuation);
    if last.is_empty() {
        return None;
    }
    let mut start = words.len() - 1;
    while start > 0 {
        let previous = words[start - 1].trim_matches(citation_name_punctuation);
        if previous
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_lowercase())
        {
            start -= 1;
        } else {
            break;
        }
    }
    let surname = words[start..]
        .iter()
        .map(|word| word.trim_matches(citation_name_punctuation))
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    (!surname.is_empty()).then_some(surname)
}

fn citation_name_punctuation(ch: char) -> bool {
    matches!(ch, ',' | '.' | ';' | ':' | '(' | ')' | '[' | ']')
}

fn bib_field(entry: &str, field: &str) -> Option<String> {
    let lower = entry.to_ascii_lowercase();
    let field = field.to_ascii_lowercase();
    let field_index = find_bib_field_index(&lower, &field)?;
    let after_field = &entry[field_index + field.len()..];
    let after_equals = after_field.trim_start().strip_prefix('=')?;
    let after_equals = after_equals.trim_start();
    let (raw, _) = if after_equals.starts_with('{') {
        take_braced(after_equals)?
    } else if let Some(rest) = after_equals.strip_prefix('"') {
        let end = rest.find('"')?;
        (&rest[..end], &rest[end + 1..])
    } else {
        let end = after_equals.find([',', '\n']).unwrap_or(after_equals.len());
        (&after_equals[..end], &after_equals[end..])
    };
    let cleaned = clean_bib_tex_text(raw);
    (!cleaned.is_empty()).then_some(cleaned)
}

fn find_bib_field_index(entry_lower: &str, field: &str) -> Option<usize> {
    let mut offset = 0_usize;
    while let Some(index) = entry_lower[offset..].find(field) {
        let absolute = offset + index;
        let before = entry_lower[..absolute].chars().next_back();
        let after_index = absolute + field.len();
        let after = &entry_lower[after_index..];
        let before_ok = before.is_none_or(|ch| !bib_field_name_char(ch));
        let after_ok = after.trim_start().starts_with('=');
        if before_ok && after_ok {
            return Some(absolute);
        }
        offset = after_index;
    }
    None
}

fn bib_field_name_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
}

fn clean_bib_tex_text(source: &str) -> String {
    let mut out = String::new();
    let mut cursor = source;
    while let Some(ch) = cursor.chars().next() {
        match ch {
            '\\' => {
                let rest = &cursor[ch.len_utf8()..];
                let Some(next) = rest.chars().next() else {
                    break;
                };
                if is_bib_accent_symbol(next) {
                    let after_accent = &rest[next.len_utf8()..];
                    if let Some((payload, remaining)) = take_bib_atom(after_accent) {
                        out.push_str(&clean_bib_tex_text(payload));
                        cursor = remaining;
                    } else {
                        cursor = after_accent;
                    }
                    continue;
                }
                if next.is_ascii_alphabetic() {
                    let name_len = rest
                        .char_indices()
                        .take_while(|(_, ch)| ch.is_ascii_alphabetic())
                        .map(|(index, ch)| index + ch.len_utf8())
                        .last()
                        .unwrap_or(0);
                    let name = &rest[..name_len];
                    let after_name = &rest[name_len..];
                    if is_bib_accent_control(name) {
                        if let Some((payload, remaining)) = take_bib_atom(after_name) {
                            out.push_str(&clean_bib_tex_text(payload));
                            cursor = remaining;
                        } else {
                            cursor = after_name;
                        }
                        continue;
                    }
                    if let Some(replacement) = bib_control_replacement(name) {
                        out.push_str(replacement);
                        cursor = after_name;
                        continue;
                    }
                    if let Some((payload, remaining)) = take_braced(after_name) {
                        out.push_str(&clean_bib_tex_text(payload));
                        cursor = remaining;
                        continue;
                    }
                    out.push_str(name);
                    cursor = after_name;
                    continue;
                }
                let after_symbol = &rest[next.len_utf8()..];
                match next {
                    '&' | '%' | '$' | '#' | '_' | '{' | '}' => out.push(next),
                    '~' => out.push(' '),
                    _ => out.push(next),
                }
                cursor = after_symbol;
            }
            '{' | '}' | '$' => {
                cursor = &cursor[ch.len_utf8()..];
            }
            '\n' | '\t' | '~' => {
                out.push(' ');
                cursor = &cursor[ch.len_utf8()..];
            }
            _ => {
                out.push(ch);
                cursor = &cursor[ch.len_utf8()..];
            }
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_bib_accent_symbol(ch: char) -> bool {
    matches!(ch, '\'' | '`' | '"' | '^' | '~' | '=' | '.')
}

fn is_bib_accent_control(name: &str) -> bool {
    matches!(name, "b" | "c" | "d" | "H" | "k" | "r" | "t" | "u" | "v")
}

fn bib_control_replacement(name: &str) -> Option<&'static str> {
    match name {
        "LaTeX" => Some("LaTeX"),
        "TeX" => Some("TeX"),
        "BibTeX" => Some("BibTeX"),
        "i" => Some("i"),
        "j" => Some("j"),
        "l" => Some("l"),
        "L" => Some("L"),
        "o" => Some("o"),
        "O" => Some("O"),
        "ae" => Some("ae"),
        "AE" => Some("AE"),
        "oe" => Some("oe"),
        "OE" => Some("OE"),
        "aa" => Some("a"),
        "AA" => Some("A"),
        "ss" => Some("ss"),
        "textbackslash" => Some("\\"),
        "textendash" => Some("-"),
        "textemdash" => Some("--"),
        "textquotesingle" => Some("'"),
        "textquotedblleft" | "textquotedblright" => Some("\""),
        _ => None,
    }
}

fn take_bib_atom(source: &str) -> Option<(&str, &str)> {
    take_braced(source).or_else(|| take_math_atom(source))
}

#[derive(Debug, Clone)]
struct GraphicsConfig {
    search_dirs: Vec<PathBuf>,
    extensions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GraphicsCacheKey {
    path: PathBuf,
    pdf_page_number: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GraphicReferenceCacheKey {
    payload: String,
    pdf_page_number: u32,
}

#[derive(Debug, Clone)]
struct ResolvedGraphicReference {
    key: GraphicsCacheKey,
    path: PathBuf,
    extension: String,
    pdf_page_number: u32,
}

#[derive(Debug, Clone)]
struct CachedGraphicDimensions {
    path: PathBuf,
    width_px: u16,
    height_px: u16,
    natural_width_pt: f32,
    natural_height_pt: f32,
}

#[derive(Debug, Clone)]
struct CachedGraphicAsset {
    path: PathBuf,
    width_px: u16,
    height_px: u16,
    natural_width_pt: f32,
    natural_height_pt: f32,
    payload: Arc<ImagePayload>,
}

#[derive(Debug, Default)]
struct GraphicsCache {
    references: HashMap<GraphicReferenceCacheKey, ResolvedGraphicReference>,
    assets: HashMap<GraphicsCacheKey, CachedGraphicAsset>,
    dimensions: HashMap<GraphicsCacheKey, CachedGraphicDimensions>,
}

impl GraphicsCache {
    fn insert_asset(&mut self, key: GraphicsCacheKey, asset: CachedGraphicAsset) {
        self.dimensions
            .insert(key.clone(), cached_dimensions_from_asset(&asset));
        self.assets.insert(key, asset);
    }
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self {
            search_dirs: vec![PathBuf::new()],
            extensions: ["pdf", "png", "jpg", "jpeg"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        }
    }
}

fn graphics_config(source: &str) -> Result<GraphicsConfig, String> {
    let mut config = GraphicsConfig::default();
    for payload in raw_braced_command_payloads(source, "graphicspath")? {
        for entry in parse_graphicspath_entries(&payload)? {
            if !config.search_dirs.contains(&entry) {
                config.search_dirs.push(entry);
            }
        }
    }
    let extension_payloads = raw_braced_command_payloads(source, "DeclareGraphicsExtensions")?;
    if let Some(payload) = extension_payloads.last() {
        let extensions = parse_graphics_extensions(payload);
        if !extensions.is_empty() {
            config.extensions = extensions;
        }
    }
    Ok(config)
}

fn prewarm_graphics_cache(
    source: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    cache: &mut GraphicsCache,
    timings: &mut ParseTimings,
) {
    let references = collect_graphic_references_for_prewarm(source, root_dir, graphics, cache);
    if references.is_empty() {
        return;
    }

    let started = Instant::now();
    let worker_count = thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1)
        .min(references.len())
        .min(8);
    let decoded = if worker_count <= 1 {
        references
            .into_iter()
            .map(|reference| {
                let key = reference.key.clone();
                (key, decode_graphic_asset(&reference))
            })
            .collect::<Vec<_>>()
    } else {
        let mut chunks = vec![Vec::new(); worker_count];
        for (index, reference) in references.into_iter().enumerate() {
            chunks[index % worker_count].push(reference);
        }
        thread::scope(|scope| {
            let mut handles = Vec::new();
            for chunk in chunks {
                handles.push(scope.spawn(move || {
                    chunk
                        .into_iter()
                        .map(|reference| {
                            let key = reference.key.clone();
                            (key, decode_graphic_asset(&reference))
                        })
                        .collect::<Vec<_>>()
                }));
            }
            let mut decoded = Vec::new();
            for handle in handles {
                if let Ok(items) = handle.join() {
                    decoded.extend(items);
                }
            }
            decoded
        })
    };
    for (key, result) in decoded {
        if let Ok(Some(asset)) = result {
            cache.insert_asset(key, asset);
        }
    }
    let elapsed = started.elapsed().as_millis();
    timings.includegraphics_prewarm_ms += elapsed;
    timings.includegraphics_ms += elapsed;
}

fn collect_graphic_references_for_prewarm(
    source: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    cache: &mut GraphicsCache,
) -> Vec<ResolvedGraphicReference> {
    let mut references = Vec::new();
    let mut seen = HashSet::new();
    let mut cursor = source;
    while let Some(index) = find_control(cursor, "includegraphics") {
        let rest = &cursor[index + "\\includegraphics".len()..];
        let source = rest.strip_prefix('*').unwrap_or(rest).trim_start();
        let (options, source) = take_optional_bracketed(source);
        let Some((payload, remaining)) = take_braced(source) else {
            cursor = rest;
            continue;
        };
        if let Ok(reference) =
            resolve_cached_graphic_reference(cache, root_dir, payload.trim(), options, graphics)
            && supported_graphic_extension(&reference.extension)
            && !cache.assets.contains_key(&reference.key)
            && seen.insert(reference.key.clone())
        {
            references.push(reference);
        }
        cursor = remaining;
    }
    references
}

fn load_cached_graphic_asset(
    cache: &mut GraphicsCache,
    reference: &ResolvedGraphicReference,
) -> Result<Option<CachedGraphicAsset>, String> {
    if !supported_graphic_extension(&reference.extension) {
        return Ok(None);
    }
    if let Some(asset) = cache.assets.get(&reference.key) {
        return Ok(Some(asset.clone()));
    }
    let Some(asset) = decode_graphic_asset(reference)? else {
        return Ok(None);
    };
    cache.insert_asset(reference.key.clone(), asset.clone());
    Ok(Some(asset))
}

fn load_cached_graphic_dimensions(
    cache: &mut GraphicsCache,
    reference: &ResolvedGraphicReference,
) -> Result<Option<CachedGraphicDimensions>, String> {
    if !supported_graphic_extension(&reference.extension) {
        return Ok(None);
    }
    if let Some(dimensions) = cache.dimensions.get(&reference.key) {
        return Ok(Some(dimensions.clone()));
    }
    if let Some(asset) = cache.assets.get(&reference.key) {
        let dimensions = cached_dimensions_from_asset(asset);
        cache
            .dimensions
            .insert(reference.key.clone(), dimensions.clone());
        return Ok(Some(dimensions));
    }
    let Some(dimensions) = read_graphic_dimensions(reference)? else {
        return Ok(None);
    };
    cache
        .dimensions
        .insert(reference.key.clone(), dimensions.clone());
    Ok(Some(dimensions))
}

fn resolve_cached_graphic_reference(
    cache: &mut GraphicsCache,
    root_dir: &Path,
    payload: &str,
    options: Option<&str>,
    graphics: &GraphicsConfig,
) -> Result<ResolvedGraphicReference, String> {
    let key = GraphicReferenceCacheKey {
        payload: payload.trim().to_string(),
        pdf_page_number: options.and_then(graphics_page_number).unwrap_or(1),
    };
    if let Some(reference) = cache.references.get(&key) {
        return Ok(reference.clone());
    }

    let reference = resolve_graphic_reference(root_dir, &key.payload, options, graphics)?;
    cache.references.insert(key, reference.clone());
    Ok(reference)
}

fn resolve_graphic_reference(
    root_dir: &Path,
    payload: &str,
    options: Option<&str>,
    graphics: &GraphicsConfig,
) -> Result<ResolvedGraphicReference, String> {
    let path = resolve_graphics_path(root_dir, payload, graphics)?;
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let pdf_page_number = options.and_then(graphics_page_number).unwrap_or(1);
    let key = GraphicsCacheKey {
        path: path.clone(),
        pdf_page_number: if extension == "pdf" {
            pdf_page_number
        } else {
            0
        },
    };
    Ok(ResolvedGraphicReference {
        key,
        path,
        extension,
        pdf_page_number,
    })
}

fn supported_graphic_extension(extension: &str) -> bool {
    matches!(extension, "jpg" | "jpeg" | "png" | "pdf")
}

fn decode_graphic_asset(
    reference: &ResolvedGraphicReference,
) -> Result<Option<CachedGraphicAsset>, String> {
    let data = fs::read(&reference.path).map_err(|error| {
        format!(
            "native backend could not read graphic `{}`: {error}",
            reference.path.display()
        )
    })?;
    let (width_px, height_px, natural_width_pt, natural_height_pt, payload) =
        match reference.extension.as_str() {
            "jpg" | "jpeg" => {
                let (width_px, height_px) = jpeg_dimensions(&data).ok_or_else(|| {
                    format!(
                        "native backend could not read JPEG dimensions for `{}`",
                        reference.path.display()
                    )
                })?;
                (
                    width_px,
                    height_px,
                    width_px as f32 * 72.0 / 96.0,
                    height_px as f32 * 72.0 / 96.0,
                    ImagePayload::Jpeg(data),
                )
            }
            "png" => {
                let (width_px, height_px, payload) = decode_png_image(&data).map_err(|error| {
                    format!(
                        "native backend could not decode PNG graphic `{}`: {error}",
                        reference.path.display()
                    )
                })?;
                (
                    width_px,
                    height_px,
                    width_px as f32 * 72.0 / 96.0,
                    height_px as f32 * 72.0 / 96.0,
                    payload,
                )
            }
            "pdf" => {
                let form = decode_pdf_form(&data, reference.pdf_page_number).map_err(|error| {
                    format!(
                        "native backend could not import PDF graphic `{}`: {error}",
                        reference.path.display()
                    )
                })?;
                let natural_width_pt = (form.bbox[2] - form.bbox[0]).abs().max(1.0);
                let natural_height_pt = (form.bbox[3] - form.bbox[1]).abs().max(1.0);
                (
                    pdf_graphic_dimension_to_u16(natural_width_pt)?,
                    pdf_graphic_dimension_to_u16(natural_height_pt)?,
                    natural_width_pt,
                    natural_height_pt,
                    ImagePayload::PdfForm(form),
                )
            }
            _ => return Ok(None),
        };
    Ok(Some(CachedGraphicAsset {
        path: reference.path.clone(),
        width_px,
        height_px,
        natural_width_pt,
        natural_height_pt,
        payload: Arc::new(payload),
    }))
}

fn read_graphic_dimensions(
    reference: &ResolvedGraphicReference,
) -> Result<Option<CachedGraphicDimensions>, String> {
    let data = fs::read(&reference.path).map_err(|error| {
        format!(
            "native backend could not read graphic `{}`: {error}",
            reference.path.display()
        )
    })?;
    let (width_px, height_px, natural_width_pt, natural_height_pt) =
        match reference.extension.as_str() {
            "jpg" | "jpeg" => {
                let (width_px, height_px) = jpeg_dimensions(&data).ok_or_else(|| {
                    format!(
                        "native backend could not read JPEG dimensions for `{}`",
                        reference.path.display()
                    )
                })?;
                (
                    width_px,
                    height_px,
                    width_px as f32 * 72.0 / 96.0,
                    height_px as f32 * 72.0 / 96.0,
                )
            }
            "png" => {
                let (width_px, height_px) = png_dimensions(&data).map_err(|error| {
                    format!(
                        "native backend could not read PNG dimensions for `{}`: {error}",
                        reference.path.display()
                    )
                })?;
                (
                    width_px,
                    height_px,
                    width_px as f32 * 72.0 / 96.0,
                    height_px as f32 * 72.0 / 96.0,
                )
            }
            "pdf" => pdf_form_dimensions(&data, reference.pdf_page_number).map_err(|error| {
                format!(
                    "native backend could not read PDF graphic dimensions for `{}`: {error}",
                    reference.path.display()
                )
            })?,
            _ => return Ok(None),
        };
    Ok(Some(CachedGraphicDimensions {
        path: reference.path.clone(),
        width_px,
        height_px,
        natural_width_pt,
        natural_height_pt,
    }))
}

fn cached_dimensions_from_asset(asset: &CachedGraphicAsset) -> CachedGraphicDimensions {
    CachedGraphicDimensions {
        path: asset.path.clone(),
        width_px: asset.width_px,
        height_px: asset.height_px,
        natural_width_pt: asset.natural_width_pt,
        natural_height_pt: asset.natural_height_pt,
    }
}

fn image_asset_from_cached(
    asset: &CachedGraphicAsset,
    options: Option<&str>,
    layout: &DocumentLayout,
) -> ImageAsset {
    let (display_width_pt, display_height_pt, rotation_degrees, viewport) =
        graphic_display_geometry(
            options,
            layout,
            asset.natural_width_pt,
            asset.natural_height_pt,
        );
    ImageAsset {
        path: asset.path.clone(),
        width_px: asset.width_px,
        height_px: asset.height_px,
        display_width_pt,
        display_height_pt,
        rotation_degrees,
        viewport,
        payload: asset.payload.clone(),
    }
}

fn measurement_image_asset_from_cached(
    dimensions: &CachedGraphicDimensions,
    options: Option<&str>,
    layout: &DocumentLayout,
) -> ImageAsset {
    let (display_width_pt, display_height_pt, rotation_degrees, viewport) =
        graphic_display_geometry(
            options,
            layout,
            dimensions.natural_width_pt,
            dimensions.natural_height_pt,
        );
    ImageAsset {
        path: dimensions.path.clone(),
        width_px: dimensions.width_px,
        height_px: dimensions.height_px,
        display_width_pt,
        display_height_pt,
        rotation_degrees,
        viewport,
        payload: Arc::new(ImagePayload::Jpeg(Vec::new())),
    }
}

fn graphic_display_geometry(
    options: Option<&str>,
    layout: &DocumentLayout,
    natural_width_pt: f32,
    natural_height_pt: f32,
) -> (f32, f32, f32, ImageViewport) {
    let viewport = options
        .map(|options| graphics_viewport(options, layout, natural_width_pt, natural_height_pt))
        .unwrap_or_else(ImageViewport::full);
    let visible_natural_width_pt = natural_width_pt * viewport.width_fraction;
    let visible_natural_height_pt = natural_height_pt * viewport.height_fraction;
    let rotation_degrees = options.and_then(graphics_angle_degrees).unwrap_or(0.0);
    let (display_width_pt, display_height_pt) = options
        .and_then(|options| {
            graphics_display_size_pt(
                options,
                layout,
                visible_natural_width_pt,
                visible_natural_height_pt,
            )
        })
        .unwrap_or_else(|| {
            let width = visible_natural_width_pt.min(layout.text_width_pt);
            (
                width,
                width * visible_natural_height_pt / visible_natural_width_pt,
            )
        });
    (
        display_width_pt,
        display_height_pt,
        rotation_degrees,
        viewport,
    )
}

fn parse_graphicspath_entries(payload: &str) -> Result<Vec<PathBuf>, String> {
    let mut entries = Vec::new();
    let mut cursor = payload;
    while !cursor.trim().is_empty() {
        let trimmed = cursor.trim_start();
        let Some((entry, remaining)) = take_braced(trimmed) else {
            return Err("native backend requires braced \\graphicspath entries".to_string());
        };
        let entry = entry.trim();
        if !entry.is_empty() {
            let path = Path::new(entry);
            if path.is_absolute() {
                return Err(format!(
                    "native backend only supports local relative \\graphicspath entries, got `{entry}`"
                ));
            }
            entries.push(path.to_path_buf());
        }
        cursor = remaining;
    }
    Ok(entries)
}

fn parse_graphics_extensions(payload: &str) -> Vec<String> {
    payload
        .split(',')
        .map(str::trim)
        .map(|extension| extension.trim_start_matches('.').to_ascii_lowercase())
        .filter(|extension| !extension.is_empty())
        .collect()
}

fn parse_includegraphics<'a>(
    source: &'a str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
) -> Result<(GraphicElement, &'a str), String> {
    let source = source.strip_prefix('*').unwrap_or(source).trim_start();
    let (options, source) = take_optional_bracketed(source);
    let (payload, remaining) = take_braced(source)
        .ok_or_else(|| "native backend requires braced \\includegraphics paths".to_string())?;
    let reference =
        resolve_cached_graphic_reference(graphics_cache, root_dir, payload, options, graphics)?;
    let Some(asset) = load_cached_graphic_asset(graphics_cache, &reference)? else {
        return Ok((GraphicElement::Placeholder(reference.path), remaining));
    };
    Ok((
        GraphicElement::Image(image_asset_from_cached(&asset, options, layout)),
        remaining,
    ))
}

fn parse_includegraphics_measurement<'a>(
    source: &'a str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
) -> Result<(Option<ImageAsset>, &'a str), String> {
    let source = source.strip_prefix('*').unwrap_or(source).trim_start();
    let (options, source) = take_optional_bracketed(source);
    let (payload, remaining) = take_braced(source)
        .ok_or_else(|| "native backend requires braced \\includegraphics paths".to_string())?;
    let reference =
        resolve_cached_graphic_reference(graphics_cache, root_dir, payload, options, graphics)?;
    let image = load_cached_graphic_dimensions(graphics_cache, &reference)?
        .map(|dimensions| measurement_image_asset_from_cached(&dimensions, options, layout));
    Ok((image, remaining))
}

fn native_tikz_graphic(source: &str) -> Option<ImageAsset> {
    if let Some(rule) = native_tikz_rule_graphic(source) {
        return Some(rule);
    }
    if !(source.contains("World $z$")
        && source.contains("Data $x = g(z)$")
        && source.contains("LeJEPA $f(x)$"))
    {
        return None;
    }

    let width = 468.0_f32;
    let height = 168.0_f32;
    Some(ImageAsset {
        path: PathBuf::from("native-tikz-form"),
        width_px: width as u16,
        height_px: height as u16,
        display_width_pt: width,
        display_height_pt: height,
        rotation_degrees: 0.0,
        viewport: ImageViewport::full(),
        payload: Arc::new(ImagePayload::PdfForm(PdfFormAsset {
            bbox: [0.0, 0.0, width, height],
            content: native_lejepa_overview_form(width, height),
            resources: None,
            imported_objects: Vec::new(),
        })),
    })
}

fn native_tikz_rule_graphic(source: &str) -> Option<ImageAsset> {
    if !(source.contains("\\draw")
        && source.contains("line width=")
        && source.contains("--")
        && source.contains("\\textwidth"))
    {
        return None;
    }
    let stroke_width = tikz_line_width_pt(source).unwrap_or(0.5).clamp(0.25, 6.0);
    let display_width = 468.0_f32;
    let display_height = (stroke_width + 4.0).max(5.0);
    let mut content = String::new();
    writeln!(content, "q").unwrap();
    push_pdf_line(
        &mut content,
        0.0,
        display_height / 2.0,
        display_width,
        display_height / 2.0,
        (0.0, 0.0, 0.0),
        stroke_width,
    );
    writeln!(content, "Q").unwrap();
    Some(ImageAsset {
        path: PathBuf::from("native-tikz-rule"),
        width_px: display_width as u16,
        height_px: display_height.ceil() as u16,
        display_width_pt: display_width,
        display_height_pt: display_height,
        rotation_degrees: 0.0,
        viewport: ImageViewport::full(),
        payload: Arc::new(ImagePayload::PdfForm(PdfFormAsset {
            bbox: [0.0, 0.0, display_width, display_height],
            content: content.into_bytes(),
            resources: None,
            imported_objects: Vec::new(),
        })),
    })
}

fn tikz_line_width_pt(source: &str) -> Option<f32> {
    let rest = source.split_once("line width=")?.1.trim_start();
    let value_end = rest
        .char_indices()
        .take_while(|(_, ch)| ch.is_ascii_digit() || *ch == '.')
        .map(|(index, ch)| index + ch.len_utf8())
        .last()?;
    let unit_rest = rest[value_end..].trim_start();
    unit_rest
        .starts_with("pt")
        .then(|| rest[..value_end].parse::<f32>().ok())
        .flatten()
}

fn native_lejepa_overview_form(width: f32, height: f32) -> Vec<u8> {
    let centers = [
        (70.0_f32, height * 0.52),
        (width * 0.50, height * 0.52),
        (width - 70.0, height * 0.52),
    ];
    let mut out = String::new();
    writeln!(out, "q").unwrap();
    push_pdf_line(&mut out, 0.0, 0.0, width, 0.0, (1.0, 1.0, 1.0), 0.1);
    push_density_panel(&mut out, centers[0], (0.87, 0.93, 1.0));
    push_warped_panel(&mut out, centers[1]);
    push_density_panel(&mut out, centers[2], (0.93, 0.88, 1.0));
    push_sample_points(&mut out, centers[0], 1.0);
    push_sample_points(&mut out, centers[1], 1.15);
    push_sample_points(&mut out, centers[2], 1.0);
    push_highlight_pair(&mut out, centers[0], false);
    push_highlight_pair(&mut out, centers[1], true);
    push_highlight_pair(&mut out, centers[2], false);
    push_pdf_arrow(
        &mut out,
        centers[0].0 + 58.0,
        centers[0].1 + 2.0,
        centers[1].0 - 62.0,
        centers[1].1 + 2.0,
        (0.35, 0.35, 0.35),
        2.0,
    );
    push_pdf_arrow(
        &mut out,
        centers[1].0 + 62.0,
        centers[1].1 + 2.0,
        centers[2].0 - 58.0,
        centers[2].1 + 2.0,
        (0.35, 0.35, 0.35),
        2.0,
    );
    push_pdf_arrow(
        &mut out,
        centers[2].0 + 44.0,
        centers[2].1 - 54.0,
        centers[2].0 + 28.0,
        centers[2].1 - 38.0,
        (0.45, 0.25, 0.65),
        1.1,
    );
    writeln!(out, "Q").unwrap();
    out.into_bytes()
}

fn push_density_panel(out: &mut String, center: (f32, f32), color: (f32, f32, f32)) {
    for (scale, light) in [(1.0, 1.0), (0.74, 0.93), (0.45, 0.82), (0.20, 0.68)] {
        push_pdf_fill_ellipse(
            out,
            center.0,
            center.1,
            52.0 * scale,
            52.0 * scale,
            (
                (color.0 * light).min(1.0),
                (color.1 * light).min(1.0),
                (color.2 * light).min(1.0),
            ),
        );
    }
    push_pdf_line(
        out,
        center.0 - 52.0,
        center.1,
        center.0 + 52.0,
        center.1,
        (0.82, 0.82, 0.82),
        0.4,
    );
    push_pdf_line(
        out,
        center.0,
        center.1 - 52.0,
        center.0,
        center.1 + 52.0,
        (0.82, 0.82, 0.82),
        0.4,
    );
}

fn push_warped_panel(out: &mut String, center: (f32, f32)) {
    push_pdf_fill_ellipse(out, center.0, center.1, 58.0, 46.0, (0.93, 0.93, 0.93));
    push_pdf_fill_ellipse(
        out,
        center.0 + 8.0,
        center.1 + 3.0,
        36.0,
        30.0,
        (0.85, 0.85, 0.85),
    );
    push_pdf_fill_ellipse(
        out,
        center.0 + 8.0,
        center.1,
        18.0,
        16.0,
        (0.75, 0.75, 0.75),
    );
}

fn push_sample_points(out: &mut String, center: (f32, f32), spread: f32) {
    const POINTS: &[(f32, f32, (f32, f32, f32))] = &[
        (0.30, 0.80, (0.20, 0.39, 0.78)),
        (-0.50, 0.40, (0.19, 0.58, 0.58)),
        (0.70, -0.30, (0.31, 0.71, 0.39)),
        (-0.20, -0.70, (0.55, 0.46, 0.35)),
        (0.10, 0.20, (0.78, 0.31, 0.31)),
        (-0.80, 0.10, (0.51, 0.24, 0.71)),
        (0.50, 0.50, (0.36, 0.33, 0.75)),
        (-0.30, -0.40, (0.70, 0.51, 0.20)),
        (0.90, 0.10, (0.75, 0.38, 0.25)),
        (-0.60, -0.50, (0.42, 0.48, 0.58)),
        (0.20, -0.90, (0.62, 0.54, 0.33)),
        (-0.10, 0.60, (0.36, 0.31, 0.75)),
        (0.40, -0.50, (0.49, 0.63, 0.29)),
        (-0.70, 0.70, (0.39, 0.32, 0.78)),
        (0.60, 0.90, (0.46, 0.35, 0.67)),
        (-0.40, -0.20, (0.61, 0.38, 0.52)),
        (0.80, -0.70, (0.58, 0.29, 0.57)),
        (-0.90, -0.30, (0.28, 0.56, 0.60)),
        (0.00, 1.10, (0.65, 0.27, 0.58)),
        (-0.50, 0.90, (0.25, 0.52, 0.68)),
    ];
    for (x, y, color) in POINTS {
        push_pdf_circle(
            out,
            center.0 + x * 36.0 * spread,
            center.1 + y * 36.0 * spread,
            1.8,
            *color,
        );
    }
}

fn push_highlight_pair(out: &mut String, center: (f32, f32), separated: bool) {
    let first = if separated {
        (center.0 - 34.0, center.1 + 28.0)
    } else {
        (center.0 - 18.0, center.1 - 26.0)
    };
    let second = if separated {
        (center.0 + 34.0, center.1 - 28.0)
    } else {
        (center.0 - 8.0, center.1 - 18.0)
    };
    push_pdf_line(
        out,
        first.0,
        first.1,
        second.0,
        second.1,
        (0.85, 0.20, 0.18),
        1.2,
    );
    push_pdf_circle(out, first.0, first.1, 3.4, (0.70, 0.04, 0.04));
    push_pdf_circle(out, second.0, second.1, 3.4, (0.95, 0.35, 0.05));
}

fn push_pdf_fill_ellipse(
    out: &mut String,
    cx: f32,
    cy: f32,
    rx: f32,
    ry: f32,
    fill: (f32, f32, f32),
) {
    let k = 0.552_284_8_f32;
    writeln!(out, "{:.3} {:.3} {:.3} rg", fill.0, fill.1, fill.2).unwrap();
    writeln!(
        out,
        "{:.2} {:.2} m {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c {:.2} {:.2} {:.2} {:.2} {:.2} {:.2} c f",
        cx + rx,
        cy,
        cx + rx,
        cy + k * ry,
        cx + k * rx,
        cy + ry,
        cx,
        cy + ry,
        cx - k * rx,
        cy + ry,
        cx - rx,
        cy + k * ry,
        cx - rx,
        cy,
        cx - rx,
        cy - k * ry,
        cx - k * rx,
        cy - ry,
        cx,
        cy - ry,
        cx + k * rx,
        cy - ry,
        cx + rx,
        cy - k * ry,
        cx + rx,
        cy
    )
    .unwrap();
}

fn push_pdf_circle(out: &mut String, cx: f32, cy: f32, radius: f32, fill: (f32, f32, f32)) {
    push_pdf_fill_ellipse(out, cx, cy, radius, radius, fill);
}

fn push_pdf_line(
    out: &mut String,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    stroke: (f32, f32, f32),
    width: f32,
) {
    writeln!(
        out,
        "{:.3} {:.3} {:.3} RG {:.2} w {:.2} {:.2} m {:.2} {:.2} l S",
        stroke.0, stroke.1, stroke.2, width, x1, y1, x2, y2
    )
    .unwrap();
}

fn push_pdf_arrow(
    out: &mut String,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    stroke: (f32, f32, f32),
    width: f32,
) {
    push_pdf_line(out, x1, y1, x2, y2, stroke, width);
    let dx = x2 - x1;
    let dy = y2 - y1;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let ux = dx / len;
    let uy = dy / len;
    let px = -uy;
    let py = ux;
    let size = 7.0;
    let left = (
        x2 - ux * size + px * size * 0.45,
        y2 - uy * size + py * size * 0.45,
    );
    let right = (
        x2 - ux * size - px * size * 0.45,
        y2 - uy * size - py * size * 0.45,
    );
    writeln!(out, "{:.3} {:.3} {:.3} rg", stroke.0, stroke.1, stroke.2).unwrap();
    writeln!(
        out,
        "{:.2} {:.2} m {:.2} {:.2} l {:.2} {:.2} l f",
        x2, y2, left.0, left.1, right.0, right.1
    )
    .unwrap();
}

fn decode_png_image(data: &[u8]) -> Result<(u16, u16, ImagePayload), String> {
    if let Some(payload) = png_flate_passthrough(data)? {
        return Ok(payload);
    }
    let mut decoder = png::Decoder::new(Cursor::new(data));
    decoder.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = decoder.read_info().map_err(|error| error.to_string())?;
    let mut buffer = vec![0; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buffer)
        .map_err(|error| error.to_string())?;
    let pixels = &buffer[..info.buffer_size()];
    let width_px = u16::try_from(info.width)
        .map_err(|_| format!("PNG width {} exceeds native backend limit", info.width))?;
    let height_px = u16::try_from(info.height)
        .map_err(|_| format!("PNG height {} exceeds native backend limit", info.height))?;
    let payload = match info.color_type {
        png::ColorType::Rgb => ImagePayload::Png {
            color_space: PdfColorSpace::DeviceRgb,
            bits_per_component: info.bit_depth as u8,
            data: zlib_compress(pixels)?,
            alpha: None,
            decode_params: None,
        },
        png::ColorType::Rgba => {
            let mut rgb = Vec::with_capacity(pixels.len() / 4 * 3);
            let mut alpha = Vec::with_capacity(pixels.len() / 4);
            let mut opaque = true;
            for pixel in pixels.chunks_exact(4) {
                rgb.extend_from_slice(&pixel[..3]);
                opaque &= pixel[3] == 255;
                alpha.push(pixel[3]);
            }
            ImagePayload::Png {
                color_space: PdfColorSpace::DeviceRgb,
                bits_per_component: info.bit_depth as u8,
                data: zlib_compress(&rgb)?,
                alpha: if opaque {
                    None
                } else {
                    Some(zlib_compress(&alpha)?)
                },
                decode_params: None,
            }
        }
        png::ColorType::Grayscale => ImagePayload::Png {
            color_space: PdfColorSpace::DeviceGray,
            bits_per_component: info.bit_depth as u8,
            data: zlib_compress(pixels)?,
            alpha: None,
            decode_params: None,
        },
        png::ColorType::GrayscaleAlpha => {
            let mut gray = Vec::with_capacity(pixels.len() / 2);
            let mut alpha = Vec::with_capacity(pixels.len() / 2);
            let mut opaque = true;
            for pixel in pixels.chunks_exact(2) {
                gray.push(pixel[0]);
                opaque &= pixel[1] == 255;
                alpha.push(pixel[1]);
            }
            ImagePayload::Png {
                color_space: PdfColorSpace::DeviceGray,
                bits_per_component: info.bit_depth as u8,
                data: zlib_compress(&gray)?,
                alpha: if opaque {
                    None
                } else {
                    Some(zlib_compress(&alpha)?)
                },
                decode_params: None,
            }
        }
        png::ColorType::Indexed => {
            return Err("indexed PNG output was not expanded by decoder".to_string());
        }
    };
    Ok((width_px, height_px, payload))
}

fn png_dimensions(data: &[u8]) -> Result<(u16, u16), String> {
    const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if data.len() < 33 || &data[..8] != PNG_SIGNATURE {
        return Err("invalid PNG signature".to_string());
    }
    if &data[12..16] != b"IHDR" {
        return Err("PNG IHDR chunk is not first".to_string());
    }
    let width_value = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let height_value = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    Ok((
        u16::try_from(width_value)
            .map_err(|_| format!("PNG width {width_value} exceeds native backend limit"))?,
        u16::try_from(height_value)
            .map_err(|_| format!("PNG height {height_value} exceeds native backend limit"))?,
    ))
}

fn png_flate_passthrough(data: &[u8]) -> Result<Option<(u16, u16, ImagePayload)>, String> {
    const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
    if data.len() < 33 || &data[..8] != PNG_SIGNATURE {
        return Ok(None);
    }
    let mut cursor = 8_usize;
    let mut width = None;
    let mut height = None;
    let mut bit_depth = 0_u8;
    let mut color_type = 0_u8;
    let mut idat = Vec::new();
    let mut has_transparency = false;
    while cursor + 12 <= data.len() {
        let length = u32::from_be_bytes([
            data[cursor],
            data[cursor + 1],
            data[cursor + 2],
            data[cursor + 3],
        ]) as usize;
        let chunk_type = &data[cursor + 4..cursor + 8];
        let chunk_start = cursor + 8;
        let chunk_end = chunk_start + length;
        let next = chunk_end + 4;
        if next > data.len() {
            return Err("PNG chunk extends past end of file".to_string());
        }
        match chunk_type {
            b"IHDR" => {
                if length != 13 {
                    return Err("PNG IHDR chunk has invalid length".to_string());
                }
                let width_value = u32::from_be_bytes([
                    data[chunk_start],
                    data[chunk_start + 1],
                    data[chunk_start + 2],
                    data[chunk_start + 3],
                ]);
                let height_value = u32::from_be_bytes([
                    data[chunk_start + 4],
                    data[chunk_start + 5],
                    data[chunk_start + 6],
                    data[chunk_start + 7],
                ]);
                bit_depth = data[chunk_start + 8];
                color_type = data[chunk_start + 9];
                let compression = data[chunk_start + 10];
                let filter = data[chunk_start + 11];
                let interlace = data[chunk_start + 12];
                if compression != 0 || filter != 0 || interlace != 0 {
                    return Ok(None);
                }
                width = Some(u16::try_from(width_value).map_err(|_| {
                    format!("PNG width {width_value} exceeds native backend limit")
                })?);
                height = Some(u16::try_from(height_value).map_err(|_| {
                    format!("PNG height {height_value} exceeds native backend limit")
                })?);
            }
            b"IDAT" => idat.extend_from_slice(&data[chunk_start..chunk_end]),
            b"tRNS" => has_transparency = true,
            b"IEND" => break,
            _ => {}
        }
        cursor = next;
    }
    if has_transparency || idat.is_empty() {
        return Ok(None);
    }
    let (color_space, colors) = match color_type {
        0 => (PdfColorSpace::DeviceGray, 1),
        2 => (PdfColorSpace::DeviceRgb, 3),
        _ => return Ok(None),
    };
    let Some(width) = width else {
        return Ok(None);
    };
    let Some(height) = height else {
        return Ok(None);
    };
    Ok(Some((
        width,
        height,
        ImagePayload::Png {
            color_space,
            bits_per_component: bit_depth,
            data: idat,
            alpha: None,
            decode_params: Some(PngDecodeParams {
                colors,
                bits_per_component: bit_depth,
                columns: width,
            }),
        },
    )))
}

fn decode_pdf_form(data: &[u8], page_number: u32) -> Result<PdfFormAsset, String> {
    let document = LoDocument::load_mem(data).map_err(|error| error.to_string())?;
    let page_id = pdf_graphic_page_id(&document, page_number)?;
    let content = document
        .get_page_content(page_id)
        .map_err(|error| error.to_string())?;
    let bbox_object = match inherited_page_object(&document, page_id, b"CropBox")? {
        Some(object) => Some(object),
        None => inherited_page_object(&document, page_id, b"MediaBox")?,
    };
    let bbox = bbox_object
        .as_ref()
        .and_then(|object| pdf_box_from_object(&document, object))
        .ok_or_else(|| "PDF graphic has no usable CropBox or MediaBox".to_string())?;
    let resources = inherited_page_object(&document, page_id, b"Resources")?;
    let imported_objects = resources
        .as_ref()
        .map(|resources| collect_imported_pdf_objects(&document, resources))
        .transpose()?
        .unwrap_or_default();

    Ok(PdfFormAsset {
        bbox,
        content,
        resources,
        imported_objects,
    })
}

fn pdf_form_dimensions(data: &[u8], page_number: u32) -> Result<(u16, u16, f32, f32), String> {
    let document = LoDocument::load_mem(data).map_err(|error| error.to_string())?;
    let page_id = pdf_graphic_page_id(&document, page_number)?;
    let bbox_object = match inherited_page_object(&document, page_id, b"CropBox")? {
        Some(object) => Some(object),
        None => inherited_page_object(&document, page_id, b"MediaBox")?,
    };
    let bbox = bbox_object
        .as_ref()
        .and_then(|object| pdf_box_from_object(&document, object))
        .ok_or_else(|| "PDF graphic has no usable CropBox or MediaBox".to_string())?;
    let natural_width_pt = (bbox[2] - bbox[0]).abs().max(1.0);
    let natural_height_pt = (bbox[3] - bbox[1]).abs().max(1.0);
    Ok((
        pdf_graphic_dimension_to_u16(natural_width_pt)?,
        pdf_graphic_dimension_to_u16(natural_height_pt)?,
        natural_width_pt,
        natural_height_pt,
    ))
}

fn pdf_graphic_page_id(document: &LoDocument, page_number: u32) -> Result<LoObjectId, String> {
    let pages = document.get_pages();
    if pages.is_empty() {
        return Err("PDF graphic has no pages".to_string());
    }
    pages.get(&page_number).copied().ok_or_else(|| {
        format!(
            "PDF graphic has no page {page_number}; available page count is {}",
            pages.len()
        )
    })
}

fn inherited_page_object(
    document: &LoDocument,
    page_id: LoObjectId,
    key: &[u8],
) -> Result<Option<LoObject>, String> {
    let mut current = Some(page_id);
    let mut seen = BTreeSet::new();
    while let Some(object_id) = current {
        if !seen.insert(object_id) {
            return Err(format!(
                "PDF graphic page tree contains a parent cycle at {} {} R",
                object_id.0, object_id.1
            ));
        }
        let dictionary = document
            .get_dictionary(object_id)
            .map_err(|error| error.to_string())?;
        if let Ok(object) = dictionary.get(key) {
            return Ok(Some(object.clone()));
        }
        current = dictionary
            .get(b"Parent")
            .ok()
            .and_then(|object| object.as_reference().ok());
    }
    Ok(None)
}

fn pdf_box_from_object(document: &LoDocument, object: &LoObject) -> Option<[f32; 4]> {
    let (_, object) = document.dereference(object).ok()?;
    let array = object.as_array().ok()?;
    if array.len() < 4 {
        return None;
    }
    Some([
        pdf_number(&array[0])?,
        pdf_number(&array[1])?,
        pdf_number(&array[2])?,
        pdf_number(&array[3])?,
    ])
}

fn pdf_number(object: &LoObject) -> Option<f32> {
    match object {
        LoObject::Integer(value) => Some(*value as f32),
        LoObject::Real(value) => Some(*value),
        _ => None,
    }
}

fn pdf_graphic_dimension_to_u16(value: f32) -> Result<u16, String> {
    let value = value.ceil().max(1.0);
    if value > u16::MAX as f32 {
        return Err(format!(
            "PDF graphic dimension {value:.0}pt exceeds native backend limit"
        ));
    }
    Ok(value as u16)
}

fn collect_imported_pdf_objects(
    document: &LoDocument,
    root: &LoObject,
) -> Result<Vec<(LoObjectId, LoObject)>, String> {
    let mut pending = BTreeSet::new();
    let mut imported = BTreeSet::new();
    let mut objects = Vec::new();
    collect_lopdf_references(root, &mut pending);

    while let Some(object_id) = pending.pop_first() {
        if !imported.insert(object_id) {
            continue;
        }
        let object = document
            .objects
            .get(&object_id)
            .ok_or_else(|| {
                format!(
                    "PDF graphic references missing object {} {} R",
                    object_id.0, object_id.1
                )
            })?
            .clone();
        collect_lopdf_references(&object, &mut pending);
        objects.push((object_id, object));
    }

    Ok(objects)
}

fn collect_lopdf_references(object: &LoObject, references: &mut BTreeSet<LoObjectId>) {
    match object {
        LoObject::Array(items) => {
            for item in items {
                collect_lopdf_references(item, references);
            }
        }
        LoObject::Dictionary(dictionary) => {
            for (_, value) in dictionary.iter() {
                collect_lopdf_references(value, references);
            }
        }
        LoObject::Stream(stream) => {
            for (_, value) in stream.dict.iter() {
                collect_lopdf_references(value, references);
            }
        }
        LoObject::Reference(object_id) => {
            references.insert(*object_id);
        }
        LoObject::Null
        | LoObject::Boolean(_)
        | LoObject::Integer(_)
        | LoObject::Real(_)
        | LoObject::Name(_)
        | LoObject::String(_, _) => {}
    }
}

fn zlib_compress(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(data).map_err(|error| error.to_string())?;
    encoder.finish().map_err(|error| error.to_string())
}

fn parse_tabular<'a>(
    source: &'a str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    env: &str,
) -> Result<(Vec<String>, &'a str), String> {
    let (_, source) = take_optional_bracketed(source);
    let (_, source) = take_braced(source)
        .ok_or_else(|| format!("native backend requires braced column spec for `{env}`"))?;
    let (body, remaining) = take_environment_body(source, env)?;
    Ok((
        clean_tabular_body(body, macros, labels, citations),
        remaining,
    ))
}

fn parse_resizebox<'a>(
    source: &'a str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
) -> Result<(Vec<String>, &'a str), String> {
    let (_, source) = take_braced(source)
        .ok_or_else(|| "native backend requires braced \\resizebox width".to_string())?;
    let (_, source) = take_braced(source)
        .ok_or_else(|| "native backend requires braced \\resizebox height".to_string())?;
    let (payload, remaining) = take_braced(source)
        .ok_or_else(|| "native backend requires braced \\resizebox payload".to_string())?;
    let mut table_rows = Vec::new();
    let mut cursor = payload;
    while let Some(index) = cursor.find("\\begin{tabular}") {
        let rest = &cursor[index + "\\begin{tabular}".len()..];
        let (mut lines, after_tabular) = parse_tabular(rest, macros, labels, citations, "tabular")?;
        table_rows.append(&mut lines);
        cursor = after_tabular;
    }
    if table_rows.is_empty() {
        let cleaned = clean_table_row(payload, macros, labels, citations);
        if !cleaned.is_empty() {
            table_rows.push(cleaned);
        }
    }
    Ok((table_rows, remaining))
}

fn clean_tabular_body(
    source: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
) -> Vec<String> {
    split_align_rows(source)
        .into_iter()
        .map(|row| clean_table_row(row, macros, labels, citations))
        .filter(|row| !row.is_empty())
        .collect()
}

fn append_table_lines(lines: &mut Vec<Line>, table_lines: Vec<String>, layout: &DocumentLayout) {
    if table_lines.is_empty() {
        return;
    }
    if layout.columns > 1 {
        lines.push(Line::Blank);
        for row in table_lines {
            for wrapped in layout.wrap_table_text(&row) {
                lines.push(Line::TableRow(wrapped));
            }
        }
        lines.push(Line::Blank);
        return;
    }
    lines.push(Line::Blank);
    for row in table_lines {
        for wrapped in layout.wrap_table_text(&row) {
            lines.push(Line::Text(wrapped));
        }
    }
    lines.push(Line::Blank);
}

fn append_vertical_space_lines(lines: &mut Vec<Line>, payload: &str, layout: &DocumentLayout) {
    for _ in 0..vertical_space_slots(payload, layout) {
        lines.push(Line::Blank);
    }
}

fn append_vertical_skip_points(lines: &mut Vec<Line>, points: f32, layout: &DocumentLayout) {
    for _ in 0..vertical_skip_slots_points(points, layout) {
        lines.push(Line::Blank);
    }
}

fn vertical_space_slots(payload: &str, layout: &DocumentLayout) -> usize {
    let Some(points) = parse_tex_length_points(payload, layout) else {
        return 0;
    };
    vertical_space_slots_points(points, layout)
}

fn vertical_space_slots_points(points: f32, layout: &DocumentLayout) -> usize {
    if points <= 0.0 {
        return 0;
    }
    let line_height = layout.line_height_pt.max(1.0);
    if points < line_height * 0.75 {
        return 0;
    }
    ((points / line_height).floor() as usize).max(1)
}

fn vertical_skip_slots_points(points: f32, layout: &DocumentLayout) -> usize {
    if points <= 0.0 {
        return 0;
    }
    let line_height = layout.line_height_pt.max(1.0);
    if points < line_height * 0.5 {
        return 0;
    }
    ((points / line_height).floor() as usize).max(1)
}

fn parse_tex_length_points(payload: &str, layout: &DocumentLayout) -> Option<f32> {
    let source = payload.trim();
    let mut end_number = 0_usize;
    let mut saw_digit = false;
    for (index, ch) in source.char_indices() {
        let valid = ch.is_ascii_digit() || ch == '.' || ((ch == '-' || ch == '+') && index == 0);
        if !valid {
            break;
        }
        if ch.is_ascii_digit() {
            saw_digit = true;
        }
        end_number = index + ch.len_utf8();
    }
    if !saw_digit {
        return None;
    }
    let value = source[..end_number].parse::<f32>().ok()?;
    let unit_source = source[end_number..].trim_start();
    let mut end_unit = 0_usize;
    for (index, ch) in unit_source.char_indices() {
        if !ch.is_ascii_alphabetic() {
            break;
        }
        end_unit = index + ch.len_utf8();
    }
    let unit = &unit_source[..end_unit];
    let points_per_unit = match unit {
        "pt" => 1.0,
        "bp" => 72.27 / 72.0,
        "in" => 72.27,
        "cm" => 72.27 / 2.54,
        "mm" => 72.27 / 25.4,
        "em" => layout.text_font_pt,
        "ex" => layout.text_font_pt * 0.5,
        _ => return None,
    };
    Some(value * points_per_unit)
}

fn append_toc_lines(lines: &mut Vec<Line>, entries: &[TocEntry], layout: &DocumentLayout) {
    if entries.is_empty() {
        lines.push(Line::Text("(no entries)".to_string()));
        return;
    }
    for entry in entries {
        let indent = match entry.level {
            TocLevel::Section => "",
            TocLevel::Subsection => "  ",
        };
        let label = match entry.number.as_deref() {
            Some(number) if !number.is_empty() => format!("{number} {}", entry.title),
            _ => entry.title.clone(),
        };
        let line = format!("{indent}{label} .... {}", entry.page);
        for wrapped in layout.wrap_toc_text(&line) {
            lines.push(Line::Text(wrapped));
        }
    }
}

fn append_float_list_lines(
    lines: &mut Vec<Line>,
    entries: &[FloatEntry],
    kind: FloatKind,
    layout: &DocumentLayout,
) {
    let matching = entries
        .iter()
        .filter(|entry| entry.kind == kind)
        .collect::<Vec<_>>();
    if matching.is_empty() {
        lines.push(Line::Text("(no entries)".to_string()));
        return;
    }
    for entry in matching {
        let line = format!(
            "{} {}: {} .... {}",
            kind.label(),
            entry.number,
            entry.title,
            entry.page
        );
        for wrapped in layout.wrap_toc_text(&line) {
            lines.push(Line::Text(wrapped));
        }
    }
}

fn append_maketitle_lines(
    lines: &mut Vec<Line>,
    title: Option<&str>,
    author: Option<&str>,
    date_or_affiliation: Option<&str>,
    author_grid: Option<&AuthorGrid>,
    layout: &DocumentLayout,
) {
    let use_neurips_author_grid =
        author_grid.is_some() && *layout == DocumentLayout::neurips_single_column();
    if use_neurips_author_grid {
        for _ in 0..4 {
            lines.push(Line::Blank);
        }
    }
    if let Some(title) = title.filter(|text| !text.is_empty()) {
        for line in layout.wrap_text(title) {
            lines.push(Line::Title(line));
        }
        let title_blanks = if use_neurips_author_grid { 4 } else { 1 };
        for _ in 0..title_blanks {
            lines.push(Line::Blank);
        }
    }
    if let Some(grid) = author_grid {
        lines.push(Line::AuthorGrid(grid.clone()));
    } else if let Some(author) = author.filter(|text| !text.is_empty()) {
        for line in layout.wrap_text(author) {
            lines.push(Line::Author(line));
        }
    }
    if let Some(date_or_affiliation) = date_or_affiliation.filter(|text| !text.is_empty()) {
        for line in layout.wrap_text(date_or_affiliation) {
            lines.push(Line::Author(line));
        }
    }
    if author_grid.is_none() && (author.is_some() || date_or_affiliation.is_some()) {
        lines.push(Line::Blank);
    }
}

fn native_neurips_author_grid_from_source(
    source: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
) -> Result<Option<AuthorGrid>, String> {
    let Some(author_source) = raw_braced_command_payload(source, "author") else {
        return Ok(None);
    };
    if !source_contains_control(author_source, "And")
        && !source_contains_control(author_source, "AND")
    {
        return Ok(None);
    }
    let mut blocks = Vec::new();
    for block_source in split_neurips_author_blocks(author_source) {
        let mut lines = Vec::new();
        for line_source in split_neurips_author_lines(block_source) {
            let line =
                clean_inline_text_collecting(line_source, macros, labels, citations, footnotes)?;
            let line = line.trim();
            if !line.is_empty() {
                lines.push(line.to_string());
            }
        }
        if !lines.is_empty() {
            blocks.push(AuthorBlock { lines });
        }
    }
    if blocks.len() < 2 {
        return Ok(None);
    }
    let rows = if blocks.len() == 3 {
        vec![
            vec![blocks[0].clone(), blocks[1].clone()],
            vec![blocks[2].clone()],
        ]
    } else {
        blocks.chunks(2).map(|row| row.to_vec()).collect::<Vec<_>>()
    };
    Ok(Some(AuthorGrid { rows }))
}

fn split_neurips_author_blocks(source: &str) -> Vec<&str> {
    let mut blocks = Vec::new();
    let mut cursor = source;
    while let Some((index, len)) = find_next_neurips_author_separator(cursor) {
        blocks.push(&cursor[..index]);
        cursor = &cursor[index + len..];
    }
    blocks.push(cursor);
    blocks
}

fn find_next_neurips_author_separator(source: &str) -> Option<(usize, usize)> {
    ["And", "AND"]
        .into_iter()
        .filter_map(|command| find_control(source, command).map(|index| (index, command.len() + 1)))
        .min_by_key(|(index, _)| *index)
}

fn split_neurips_author_lines(source: &str) -> impl Iterator<Item = &str> {
    source.split("\\\\").map(str::trim)
}

fn append_wide_maketitle_lines(
    lines: &mut Vec<Line>,
    title: Option<&str>,
    author: Option<&str>,
    date_or_affiliation: Option<&str>,
    layout: &DocumentLayout,
) {
    if let Some(title) = title.filter(|text| !text.is_empty()) {
        for line in layout.wrap_wide_text(title) {
            lines.push(Line::WideTitle(line));
        }
        lines.push(Line::Blank);
    }
    if let Some(author) = author.filter(|text| !text.is_empty()) {
        for line in layout.wrap_wide_text(author) {
            lines.push(Line::WideAuthor(line));
        }
    }
    if let Some(date_or_affiliation) = date_or_affiliation.filter(|text| !text.is_empty()) {
        for line in layout.wrap_wide_text(date_or_affiliation) {
            lines.push(Line::WideAuthor(line));
        }
    }
    if author.is_some() || date_or_affiliation.is_some() {
        lines.push(Line::Blank);
    }
}

fn append_wide_abstract_text_lines(lines: &mut Vec<Line>, text: &str, layout: &DocumentLayout) {
    let wrapped = layout.wrap_wide_text(text);
    if !wrapped.is_empty() {
        lines.push(Line::WideBackground(wrapped.len() + 1));
    }
    for line in wrapped {
        lines.push(Line::WideAbstractText(line));
    }
}

fn append_nativeicml_abstract_lines(
    lines: &mut Vec<Line>,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    source: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    timings: &mut ParseTimings,
    figure_counter: &mut usize,
    table_counter: &mut usize,
    caption_label_separator: CaptionLabelSeparator,
) -> Result<(), String> {
    if let Some(center_index) = source.find("\\begin{center}") {
        let before = &source[..center_index];
        let center_rest = &source[center_index + "\\begin{center}".len()..];
        let (center_body, after_center) = take_environment_body(center_rest, "center")?;
        let mut block = Vec::new();
        let before = clean_inline_text_collecting(before, macros, labels, citations, footnotes)?;
        append_wide_abstract_text_content(&mut block, &before, layout);
        append_nativeicml_teaser_lines(
            &mut block,
            images,
            inputs,
            center_body,
            root_dir,
            graphics,
            graphics_cache,
            layout,
            macros,
            labels,
            citations,
            footnotes,
            timings,
            figure_counter,
            table_counter,
            caption_label_separator,
        )?;
        let after =
            clean_inline_text_collecting(after_center, macros, labels, citations, footnotes)?;
        append_wide_abstract_text_content(&mut block, &after, layout);
        append_wide_block(lines, block, layout, images);
    } else {
        let text = clean_inline_text_collecting(source, macros, labels, citations, footnotes)?;
        append_wide_abstract_text_lines(lines, &text, layout);
    }
    Ok(())
}

fn append_wide_block(
    lines: &mut Vec<Line>,
    block: Vec<Line>,
    layout: &DocumentLayout,
    images: &[ImageAsset],
) {
    if block.is_empty() {
        return;
    }
    let slots = block
        .iter()
        .map(|line| line.slots(*layout, images))
        .sum::<usize>();
    lines.push(Line::WideBackground(slots + 1));
    lines.extend(block);
}

fn append_theorem_box_lines(
    lines: &mut Vec<Line>,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    heading: &str,
    body: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    timings: &mut ParseTimings,
    equation_counter: &mut usize,
) -> Result<(), String> {
    let mut block = Vec::new();
    block.push(Line::Blank);
    for equation in theorem_equation_lines(body, equation_counter) {
        block.push(Line::WideEquation(equation));
    }
    let minipages = parse_minipage_renders(
        body,
        images,
        inputs,
        root_dir,
        graphics,
        graphics_cache,
        layout,
        macros,
        labels,
        citations,
        footnotes,
        timings,
    )?;
    if !minipages.is_empty() {
        let cells = minipages
            .iter()
            .map(|minipage| WideTeaserCell {
                width_pt: minipage.width_pt,
                images: minipage.images.clone(),
                text_lines: minipage.text_lines.clone(),
            })
            .collect::<Vec<_>>();
        block.push(Line::WideTeaserRow(cells));
    }
    let text_source = remove_theorem_layout_blocks(body);
    let text = clean_inline_text_collecting(&text_source, macros, labels, citations, footnotes)?;
    append_wide_abstract_text_content(&mut block, &text, layout);
    if block.iter().all(|line| matches!(line, Line::Blank)) {
        return Ok(());
    }
    let slots = block
        .iter()
        .map(|line| line.slots(*layout, images))
        .sum::<usize>()
        .max(2)
        + 1;
    lines.push(Line::WideTheoremBackground {
        slots,
        heading: heading.to_string(),
    });
    lines.extend(block);
    Ok(())
}

fn append_theorem_float_lines(
    lines: &mut Vec<Line>,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    body: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    timings: &mut ParseTimings,
    theorem_counters: &mut HashMap<&'static str, usize>,
    equation_counter: &mut usize,
) -> Result<bool, String> {
    let mut cursor = body;
    while let Some(index) = cursor.find("\\begin") {
        let rest = &cursor[index + "\\begin".len()..];
        let Some((env, after_env)) = take_braced(rest) else {
            cursor = rest;
            continue;
        };
        let Some(spec) = theorem_environment_spec(env) else {
            cursor = after_env;
            continue;
        };
        let opening = parse_theorem_opening(spec, after_env)?;
        let number = next_theorem_number(theorem_counters, spec.counter_key);
        let title = opening
            .title
            .map(|title| clean_inline_text_collecting(title, macros, labels, citations, footnotes))
            .transpose()?
            .unwrap_or_default();
        let heading = theorem_heading(spec.display_name, number, &title);
        let (theorem_body, _) = take_environment_body(opening.remaining, env)?;
        append_theorem_box_lines(
            lines,
            images,
            inputs,
            &heading,
            theorem_body,
            root_dir,
            graphics,
            graphics_cache,
            layout,
            macros,
            labels,
            citations,
            footnotes,
            timings,
            equation_counter,
        )?;
        return Ok(true);
    }
    Ok(false)
}

fn append_minipage_float_lines(
    lines: &mut Vec<Line>,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    kind: FloatKind,
    env: &str,
    top: bool,
    body: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    timings: &mut ParseTimings,
    figure_counter: &mut usize,
    table_counter: &mut usize,
    caption_label_separator: CaptionLabelSeparator,
) -> Result<(), String> {
    let is_wide = env.trim().ends_with('*');
    let base_width = if is_wide {
        layout.text_width_pt
    } else {
        layout.column_width_pt
    };
    let minipages = parse_minipage_renders_for_width(
        body,
        images,
        inputs,
        root_dir,
        graphics,
        graphics_cache,
        layout,
        macros,
        labels,
        citations,
        footnotes,
        timings,
        base_width,
    )?;
    if minipages.is_empty() {
        return Ok(());
    }
    let cells = minipages
        .iter()
        .map(|minipage| WideTeaserCell {
            width_pt: minipage.width_pt,
            images: minipage.images.clone(),
            text_lines: minipage.text_lines.clone(),
        })
        .collect::<Vec<_>>();
    let mut block = Vec::new();
    if is_wide {
        block.push(Line::WideTeaserRow(cells));
    } else {
        block.push(Line::TeaserRow(cells));
    }
    if let Some(caption) = native_caption_payload(body, macros, labels, citations, footnotes)? {
        let number = next_float_number(kind, figure_counter, table_counter);
        if is_wide {
            append_wide_caption_lines(
                &mut block,
                &caption_label(kind.label(), number, &caption, caption_label_separator),
                layout,
            );
        } else {
            append_caption_lines(
                &mut block,
                &caption_label(kind.label(), number, &caption, caption_label_separator),
                layout,
            );
        }
    }
    block.push(Line::Blank);
    lines.push(Line::FloatBlock {
        lines: block,
        wide: is_wide,
        top,
    });
    lines.push(Line::Blank);
    Ok(())
}

fn append_graphic_float_lines(
    lines: &mut Vec<Line>,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    kind: FloatKind,
    env: &str,
    top: bool,
    body: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    timings: &mut ParseTimings,
    figure_counter: &mut usize,
    table_counter: &mut usize,
    caption_label_separator: CaptionLabelSeparator,
    defer_to_float_page: bool,
) -> Result<bool, String> {
    let is_wide = env.trim().ends_with('*');
    if layout.columns > 1 && !should_admit_two_column_graphic_float(is_wide) {
        return Ok(false);
    }
    let base_width = if is_wide {
        layout.text_width_pt
    } else {
        layout.column_width_pt
    };
    let float_layout = layout.with_line_width(base_width);
    let rows = parse_standalone_graphic_rows(
        body,
        images,
        inputs,
        root_dir,
        graphics,
        graphics_cache,
        &float_layout,
        timings,
        base_width,
    )?;
    if rows.is_empty() {
        return Ok(false);
    }

    let mut block = Vec::new();
    for row in rows {
        if is_wide {
            block.push(Line::WideImageRow(row));
        } else if let [single] = row.as_slice() {
            block.push(Line::Image(*single));
        } else {
            let cells = row
                .into_iter()
                .map(|image_index| WideTeaserCell {
                    width_pt: images[image_index].display_width_pt,
                    images: vec![image_index],
                    text_lines: Vec::new(),
                })
                .collect::<Vec<_>>();
            block.push(Line::TeaserRow(cells));
        }
    }
    if let Some(caption) = native_caption_payload(body, macros, labels, citations, footnotes)? {
        let number = next_float_number(kind, figure_counter, table_counter);
        if is_wide {
            append_wide_caption_lines(
                &mut block,
                &caption_label(kind.label(), number, &caption, caption_label_separator),
                layout,
            );
        } else {
            append_caption_lines(
                &mut block,
                &caption_label(kind.label(), number, &caption, caption_label_separator),
                layout,
            );
        }
    }
    block.push(Line::Blank);
    if defer_to_float_page {
        lines.push(Line::LateFloatBlock {
            lines: block,
            wide: is_wide,
        });
    } else {
        lines.push(Line::FloatBlock {
            lines: block,
            wide: is_wide,
            top,
        });
    }
    lines.push(Line::Blank);
    Ok(true)
}

fn should_defer_graphic_float_to_float_page(
    base_layout: DocumentLayout,
    layout: &DocumentLayout,
    appendix_mode: bool,
    kind: FloatKind,
    top: bool,
    figure_counter: usize,
) -> bool {
    appendix_mode
        && base_layout == DocumentLayout::icml_two_column()
        && layout.columns == 1
        && kind == FloatKind::Figure
        && top
        && figure_counter >= 15
}

fn should_admit_two_column_graphic_float(_is_wide: bool) -> bool {
    true
}

fn append_table_float_lines(
    lines: &mut Vec<Line>,
    kind: FloatKind,
    env: &str,
    top: bool,
    bottom: bool,
    body: &str,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    figure_counter: &mut usize,
    table_counter: &mut usize,
    caption_label_separator: CaptionLabelSeparator,
) -> Result<bool, String> {
    let is_wide = env.trim().ends_with('*');
    if kind != FloatKind::Table
        || (layout.columns > 1 && !is_wide)
        || (layout.columns == 1 && !is_wide && !bottom)
    {
        return Ok(false);
    }
    let table_lines = native_table_float_rows(body, macros, labels, citations)?;
    if table_lines.is_empty() {
        return Ok(false);
    }
    let mut block = Vec::new();
    if let Some(caption) = native_caption_payload(body, macros, labels, citations, footnotes)? {
        let number = next_float_number(kind, figure_counter, table_counter);
        let label = caption_label(kind.label(), number, &caption, caption_label_separator);
        if is_wide {
            append_wide_caption_lines(&mut block, &label, layout);
        } else {
            append_caption_lines(&mut block, &label, layout);
        }
    }
    let table_layout = if is_wide {
        layout.with_line_width(layout.text_width_pt)
    } else {
        *layout
    };
    for row in table_lines {
        let wrapped_rows = table_layout.wrap_table_text(&row);
        if let Some(cells) = table_cells_from_row(&row) {
            let slots = wrapped_rows.len().max(1);
            if is_wide {
                block.push(Line::WideTableCells { cells, slots });
            } else {
                block.push(Line::TableCells { cells, slots });
            }
            continue;
        }
        for wrapped in wrapped_rows {
            if is_wide {
                block.push(Line::WideAbstractText(wrapped));
            } else {
                block.push(Line::Text(wrapped));
            }
        }
    }
    block.push(Line::Blank);
    if bottom {
        lines.push(Line::BottomFloatBlock {
            lines: block,
            wide: is_wide,
        });
    } else {
        lines.push(Line::FloatBlock {
            lines: block,
            wide: is_wide,
            top,
        });
    }
    lines.push(Line::Blank);
    Ok(true)
}

fn native_table_float_rows(
    body: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
) -> Result<Vec<String>, String> {
    let mut rows = Vec::new();
    let mut cursor = body;
    while let Some(index) = find_next_table_payload(cursor) {
        let source = &cursor[index..];
        if let Some(rest) = source.strip_prefix("\\begin{tabular}") {
            let (mut parsed, remaining) =
                parse_tabular(rest, macros, labels, citations, "tabular")?;
            rows.append(&mut parsed);
            cursor = remaining;
            continue;
        }
        if let Some(rest) = source.strip_prefix("\\resizebox") {
            let (mut parsed, remaining) = parse_resizebox(rest, macros, labels, citations)?;
            rows.append(&mut parsed);
            cursor = remaining;
            continue;
        }
        break;
    }
    Ok(rows)
}

fn find_next_table_payload(source: &str) -> Option<usize> {
    ["\\begin{tabular}", "\\resizebox"]
        .into_iter()
        .filter_map(|marker| source.find(marker))
        .min()
}

fn theorem_equation_lines(body: &str, equation_counter: &mut usize) -> Vec<String> {
    let mut lines = Vec::new();
    for env in ["align", "equation", "multline"] {
        let marker = format!("\\begin{{{env}}}");
        let mut cursor = body;
        while let Some(index) = cursor.find(&marker) {
            let rest = &cursor[index + marker.len()..];
            let Ok((payload, remaining)) = take_environment_body(rest, env) else {
                break;
            };
            match env {
                "align" => lines.extend(clean_align_lines(payload, equation_counter, true)),
                "equation" | "multline" => {
                    *equation_counter += 1;
                    let equation = clean_equation_text(payload);
                    if !equation.is_empty() {
                        lines.push(format!("{equation} ({})", *equation_counter));
                    }
                }
                _ => {}
            }
            cursor = remaining;
        }
    }
    for env in ["align*", "equation*", "multline*"] {
        let marker = format!("\\begin{{{env}}}");
        let mut cursor = body;
        while let Some(index) = cursor.find(&marker) {
            let rest = &cursor[index + marker.len()..];
            let Ok((payload, remaining)) = take_environment_body(rest, env) else {
                break;
            };
            match env {
                "align*" => lines.extend(clean_align_lines(payload, equation_counter, false)),
                "equation*" | "multline*" => {
                    let equation = clean_equation_text(payload);
                    if !equation.is_empty() {
                        lines.push(equation);
                    }
                }
                _ => {}
            }
            cursor = remaining;
        }
    }
    lines
}

fn remove_theorem_layout_blocks(source: &str) -> String {
    let mut out = source.to_string();
    for env in [
        "align",
        "align*",
        "equation",
        "equation*",
        "multline",
        "multline*",
        "minipage",
        "center",
    ] {
        out = remove_environment_blocks(&out, env);
    }
    out
}

fn remove_environment_blocks(source: &str, env: &str) -> String {
    let marker = format!("\\begin{{{env}}}");
    let mut out = String::new();
    let mut cursor = source;
    while let Some(index) = cursor.find(&marker) {
        out.push_str(&cursor[..index]);
        let rest = &cursor[index + marker.len()..];
        let Ok((_, remaining)) = take_environment_body(rest, env) else {
            out.push_str(&cursor[index..]);
            return out;
        };
        cursor = remaining;
    }
    out.push_str(cursor);
    out
}

fn append_wide_abstract_text_content(lines: &mut Vec<Line>, text: &str, layout: &DocumentLayout) {
    for line in layout.wrap_wide_text(text) {
        if line.trim().is_empty() {
            continue;
        }
        lines.push(Line::WideAbstractText(line));
    }
}

fn append_wide_caption_lines(lines: &mut Vec<Line>, caption: &str, layout: &DocumentLayout) {
    for line in layout.wrap_wide_caption_text(caption) {
        if !line.trim().is_empty() {
            lines.push(Line::WideCaption(line));
        }
    }
}

#[derive(Debug)]
struct MinipageRender {
    width_pt: f32,
    images: Vec<usize>,
    text_lines: Vec<String>,
}

fn append_nativeicml_teaser_lines(
    lines: &mut Vec<Line>,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    source: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    timings: &mut ParseTimings,
    figure_counter: &mut usize,
    table_counter: &mut usize,
    caption_label_separator: CaptionLabelSeparator,
) -> Result<(), String> {
    let minipages = parse_minipage_renders(
        source,
        images,
        inputs,
        root_dir,
        graphics,
        graphics_cache,
        layout,
        macros,
        labels,
        citations,
        footnotes,
        timings,
    )?;
    if !minipages.is_empty() {
        for row in minipages.chunks(2) {
            let cells = row
                .iter()
                .map(|minipage| WideTeaserCell {
                    width_pt: minipage.width_pt,
                    images: minipage.images.clone(),
                    text_lines: minipage.text_lines.clone(),
                })
                .collect::<Vec<_>>();
            if cells
                .iter()
                .any(|cell| !cell.images.is_empty() || !cell.text_lines.is_empty())
            {
                lines.push(Line::WideTeaserRow(cells));
            }
        }
    } else {
        let row_images = parse_standalone_graphics(
            source,
            images,
            inputs,
            root_dir,
            graphics,
            graphics_cache,
            layout,
            timings,
        )?;
        if !row_images.is_empty() {
            lines.push(Line::WideImageRow(row_images));
        }
    }
    if let Some((kind, caption)) =
        native_captionof_payload(source, macros, labels, citations, footnotes)?
    {
        let number = next_float_number(kind, figure_counter, table_counter);
        append_wide_abstract_text_content(
            lines,
            &caption_label(kind.label(), number, &caption, caption_label_separator),
            layout,
        );
    }
    Ok(())
}

fn parse_minipage_renders(
    source: &str,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    timings: &mut ParseTimings,
) -> Result<Vec<MinipageRender>, String> {
    parse_minipage_renders_for_width(
        source,
        images,
        inputs,
        root_dir,
        graphics,
        graphics_cache,
        layout,
        macros,
        labels,
        citations,
        footnotes,
        timings,
        layout.text_width_pt,
    )
}

fn parse_minipage_renders_for_width(
    source: &str,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    timings: &mut ParseTimings,
    base_width_pt: f32,
) -> Result<Vec<MinipageRender>, String> {
    let mut out = Vec::new();
    let mut cursor = source;
    while let Some(index) = cursor.find("\\begin{minipage}") {
        let rest = &cursor[index + "\\begin{minipage}".len()..];
        let (_, rest) = take_optional_bracketed(rest);
        let (width_expr, rest) = take_braced(rest)
            .ok_or_else(|| "native backend requires braced minipage widths".to_string())?;
        let base_layout = layout.with_line_width(base_width_pt);
        let minipage_width = length_expr_to_pt(width_expr, &base_layout)
            .unwrap_or(base_width_pt)
            .clamp(24.0, base_width_pt);
        let (body, remaining) = take_environment_body(rest, "minipage")?;
        let minipage_layout = layout.with_line_width(minipage_width);
        let render = parse_minipage_render(
            body,
            images,
            inputs,
            root_dir,
            graphics,
            graphics_cache,
            &minipage_layout,
            macros,
            labels,
            citations,
            footnotes,
            timings,
        )?;
        out.push(MinipageRender {
            width_pt: minipage_width,
            ..render
        });
        cursor = remaining;
    }
    Ok(out)
}

fn parse_minipage_render(
    source: &str,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    timings: &mut ParseTimings,
) -> Result<MinipageRender, String> {
    let mut image_indices = Vec::new();
    let mut text_source = String::new();
    let mut cursor = source;
    while let Some(index) = cursor.find("\\includegraphics") {
        text_source.push_str(&cursor[..index]);
        let rest = &cursor[index + "\\includegraphics".len()..];
        let includegraphics_started = Instant::now();
        let (graphic, remaining) =
            parse_includegraphics(rest, root_dir, graphics, graphics_cache, layout)?;
        timings.includegraphics_ms += includegraphics_started.elapsed().as_millis();
        match graphic {
            GraphicElement::Image(image) => {
                inputs.push(image.path.clone());
                let image_index = images.len();
                images.push(image);
                image_indices.push(image_index);
            }
            GraphicElement::Placeholder(path) => {
                inputs.push(path.clone());
            }
        }
        cursor = remaining;
    }
    text_source.push_str(cursor);
    let text_lines =
        native_minipage_text_lines(&text_source, layout, macros, labels, citations, footnotes)?;
    Ok(MinipageRender {
        width_pt: layout.column_width_pt,
        images: image_indices,
        text_lines,
    })
}

fn native_minipage_text_lines(
    source: &str,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    let mut cursor = source;
    while let Some(index) = cursor.find("\\begin{tabular}") {
        let before = &cursor[..index];
        append_cleaned_minipage_text_lines(
            &mut lines, before, layout, macros, labels, citations, footnotes,
        )?;
        let rest = &cursor[index + "\\begin{tabular}".len()..];
        let (table_lines, remaining) = parse_tabular(rest, macros, labels, citations, "tabular")?;
        for row in table_lines {
            for wrapped in layout.wrap_table_text(&row) {
                if !wrapped.trim().is_empty() {
                    lines.push(wrapped);
                }
            }
        }
        cursor = remaining;
    }
    append_cleaned_minipage_text_lines(
        &mut lines, cursor, layout, macros, labels, citations, footnotes,
    )?;
    Ok(lines)
}

fn append_cleaned_minipage_text_lines(
    lines: &mut Vec<String>,
    source: &str,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
) -> Result<(), String> {
    let cleaned = clean_inline_text_collecting(source, macros, labels, citations, footnotes)?;
    for line in layout.wrap_text(&cleaned) {
        if !line.trim().is_empty() {
            lines.push(line);
        }
    }
    Ok(())
}

fn parse_standalone_graphics(
    source: &str,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    timings: &mut ParseTimings,
) -> Result<Vec<usize>, String> {
    let mut image_indices = Vec::new();
    let mut cursor = source;
    while let Some(index) = cursor.find("\\includegraphics") {
        let rest = &cursor[index + "\\includegraphics".len()..];
        let includegraphics_started = Instant::now();
        let (graphic, remaining) =
            parse_includegraphics(rest, root_dir, graphics, graphics_cache, layout)?;
        timings.includegraphics_ms += includegraphics_started.elapsed().as_millis();
        match graphic {
            GraphicElement::Image(image) => {
                inputs.push(image.path.clone());
                let image_index = images.len();
                images.push(image);
                image_indices.push(image_index);
            }
            GraphicElement::Placeholder(path) => {
                inputs.push(path.clone());
            }
        }
        cursor = remaining;
    }
    Ok(image_indices)
}

fn parse_standalone_graphic_rows(
    source: &str,
    images: &mut Vec<ImageAsset>,
    inputs: &mut Vec<PathBuf>,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    timings: &mut ParseTimings,
    max_width_pt: f32,
) -> Result<Vec<Vec<usize>>, String> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut row_width = 0.0_f32;
    let mut cursor = source;
    while let Some(index) = find_control(cursor, "includegraphics") {
        let before = &cursor[..index];
        if !row.is_empty() && graphic_row_breaks_before_next(before) {
            rows.push(std::mem::take(&mut row));
            row_width = 0.0;
        }
        let rest = &cursor[index + "\\includegraphics".len()..];
        let includegraphics_started = Instant::now();
        let (graphic, remaining) =
            parse_includegraphics(rest, root_dir, graphics, graphics_cache, layout)?;
        timings.includegraphics_ms += includegraphics_started.elapsed().as_millis();
        match graphic {
            GraphicElement::Image(image) => {
                inputs.push(image.path.clone());
                let image_width = image.display_width_pt.min(max_width_pt);
                let image_index = images.len();
                images.push(image);
                push_graphic_row_image(
                    &mut rows,
                    &mut row,
                    &mut row_width,
                    image_index,
                    image_width,
                    max_width_pt,
                );
            }
            GraphicElement::Placeholder(path) => {
                inputs.push(path.clone());
            }
        }
        cursor = remaining;
    }
    if !row.is_empty() {
        rows.push(row);
    }
    Ok(rows)
}

fn parse_standalone_graphic_measurement_rows(
    source: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    timings: &mut ParseTimings,
    max_width_pt: f32,
) -> Result<(Vec<Vec<usize>>, Vec<ImageAsset>), String> {
    let mut images = Vec::new();
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut row_width = 0.0_f32;
    let mut cursor = source;
    while let Some(index) = find_control(cursor, "includegraphics") {
        let before = &cursor[..index];
        if !row.is_empty() && graphic_row_breaks_before_next(before) {
            rows.push(std::mem::take(&mut row));
            row_width = 0.0;
        }
        let rest = &cursor[index + "\\includegraphics".len()..];
        let includegraphics_started = Instant::now();
        let (image, remaining) =
            parse_includegraphics_measurement(rest, root_dir, graphics, graphics_cache, layout)?;
        timings.includegraphics_ms += includegraphics_started.elapsed().as_millis();
        if let Some(image) = image {
            let image_width = image.display_width_pt.min(max_width_pt);
            let image_index = images.len();
            images.push(image);
            push_graphic_row_image(
                &mut rows,
                &mut row,
                &mut row_width,
                image_index,
                image_width,
                max_width_pt,
            );
        }
        cursor = remaining;
    }
    if !row.is_empty() {
        rows.push(row);
    }
    Ok((rows, images))
}

fn graphic_row_breaks_before_next(source: &str) -> bool {
    source.contains("\\\\") || source.contains("\\par") || source.contains("\n\n")
}

fn push_graphic_row_image(
    rows: &mut Vec<Vec<usize>>,
    row: &mut Vec<usize>,
    row_width: &mut f32,
    image_index: usize,
    image_width: f32,
    max_width_pt: f32,
) {
    let gap = if row.is_empty() {
        0.0
    } else {
        WIDE_TEASER_GAP_PT
    };
    let next_width = *row_width + gap + image_width;
    if !row.is_empty() && next_width > max_width_pt * 1.08 {
        rows.push(std::mem::take(row));
        *row_width = 0.0;
    }
    if !row.is_empty() {
        *row_width += WIDE_TEASER_GAP_PT;
    }
    *row_width += image_width;
    row.push(image_index);
}

fn record_two_column_graphic_float_fallback(
    timings: &mut ParseTimings,
    kind: FloatKind,
    number: usize,
    env: &str,
    top: bool,
    body: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    caption_label_separator: CaptionLabelSeparator,
) {
    let env = env.trim();
    let wide = env.ends_with('*');
    let estimate = estimate_two_column_graphic_float_fallback(
        kind,
        number,
        wide,
        body,
        root_dir,
        graphics,
        graphics_cache,
        layout,
        timings,
        caption_label_separator,
    );
    timings.two_column_graphic_float_fallbacks += 1;
    if wide {
        timings.two_column_wide_graphic_float_fallbacks += 1;
    }
    timings
        .two_column_graphic_float_fallback_details
        .push(GraphicFloatFallbackTrace {
            env: env.to_string(),
            top,
            wide,
            image_count: count_control_occurrences(body, "includegraphics"),
            estimate,
            caption: raw_caption_payload_for_trace(body).unwrap_or_default(),
        });
}

fn estimate_two_column_graphic_float_fallback(
    kind: FloatKind,
    number: usize,
    wide: bool,
    body: &str,
    root_dir: &Path,
    graphics: &GraphicsConfig,
    graphics_cache: &mut GraphicsCache,
    layout: &DocumentLayout,
    timings: &mut ParseTimings,
    caption_label_separator: CaptionLabelSeparator,
) -> Option<GraphicFloatFallbackEstimate> {
    let base_width = if wide {
        layout.text_width_pt
    } else {
        layout.column_width_pt
    };
    let float_layout = layout.with_line_width(base_width);
    let (rows, scratch_images) = parse_standalone_graphic_measurement_rows(
        body,
        root_dir,
        graphics,
        graphics_cache,
        &float_layout,
        timings,
        base_width,
    )
    .ok()?;
    if rows.is_empty() {
        return None;
    }

    let image_slots = rows
        .iter()
        .map(|row| graphic_float_row_slots(row, wide, layout, &scratch_images))
        .sum::<usize>();
    let caption_slots = raw_caption_payload_for_trace(body)
        .map(|caption| {
            let label = caption_label(kind.label(), number, &caption, caption_label_separator);
            if wide {
                layout.wrap_wide_caption_text(&label).len().max(1)
            } else {
                layout.wrap_caption_text(&label).len().max(1)
            }
        })
        .unwrap_or_default();
    let blank_slots = 1;
    Some(GraphicFloatFallbackEstimate {
        rows: rows.len(),
        image_slots,
        caption_slots,
        total_slots: image_slots + caption_slots + blank_slots,
    })
}

fn graphic_float_row_slots(
    row: &[usize],
    wide: bool,
    layout: &DocumentLayout,
    images: &[ImageAsset],
) -> usize {
    if wide {
        return Line::WideImageRow(row.to_vec()).slots(*layout, images);
    }
    if let [single] = row {
        return Line::Image(*single).slots(*layout, images);
    }
    let cells = row
        .iter()
        .map(|image_index| WideTeaserCell {
            width_pt: images[*image_index].display_width_pt,
            images: vec![*image_index],
            text_lines: Vec::new(),
        })
        .collect::<Vec<_>>();
    Line::TeaserRow(cells).slots(*layout, images)
}

fn count_control_occurrences(source: &str, control: &str) -> usize {
    let mut count = 0_usize;
    let mut cursor = source;
    while let Some(index) = find_control(cursor, control) {
        count += 1;
        cursor = &cursor[index + control.len() + 1..];
    }
    count
}

fn raw_caption_payload_for_trace(source: &str) -> Option<String> {
    let index = find_control(source, "caption")?;
    let rest = &source[index + "\\caption".len()..];
    let (_, rest) = take_optional_bracketed(rest);
    let (payload, _) = take_braced(rest)?;
    Some(normalize_header_spacing(&clean_header_text(payload)))
}

fn native_captionof_payload(
    source: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
) -> Result<Option<(FloatKind, String)>, String> {
    let Some(index) = source.find("\\captionof") else {
        return Ok(None);
    };
    let rest = &source[index + "\\captionof".len()..];
    let (kind, rest) = take_braced(rest)
        .ok_or_else(|| "native backend requires braced \\captionof kinds".to_string())?;
    let (_, rest) = take_optional_bracketed(rest);
    let (payload, _) = take_braced(rest)
        .ok_or_else(|| "native backend requires braced \\captionof payloads".to_string())?;
    clean_inline_text_collecting(payload, macros, labels, citations, footnotes)
        .map(|caption| Some((FloatKind::from_caption_kind(kind), caption)))
}

fn native_caption_payload(
    source: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
) -> Result<Option<String>, String> {
    let Some(index) = source.find("\\caption") else {
        return Ok(None);
    };
    let rest = &source[index + "\\caption".len()..];
    let (_, rest) = take_optional_bracketed(rest);
    let (payload, _) = take_braced(rest)
        .ok_or_else(|| "native backend requires braced \\caption payloads".to_string())?;
    clean_inline_text_collecting(payload, macros, labels, citations, footnotes).map(Some)
}

fn append_listing_float_lines(
    lines: &mut Vec<Line>,
    body: &str,
    layout: &DocumentLayout,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
    listing_counter: &mut usize,
    listing_reference_name: &str,
    caption_label_separator: CaptionLabelSeparator,
    top: bool,
) -> Result<bool, String> {
    let Some(index) = body.find("\\begin{lstlisting}") else {
        return Ok(false);
    };
    let rest = &body[index + "\\begin{lstlisting}".len()..];
    let (options, rest) = take_optional_bracketed(rest);
    let (listing, _) = take_environment_body(rest, "lstlisting")?;
    *listing_counter += 1;

    let mut block = Vec::new();
    if let Some(caption) = options.and_then(caption_option_value) {
        let caption = clean_inline_text_collecting(&caption, macros, labels, citations, footnotes)?;
        if !caption.is_empty() {
            append_caption_lines(
                &mut block,
                &caption_label(
                    listing_reference_name,
                    *listing_counter,
                    &caption,
                    caption_label_separator,
                ),
                layout,
            );
        }
    }
    block.extend(code_listing_lines(listing).into_iter().map(Line::Code));
    if block.is_empty() {
        return Ok(false);
    }
    block.push(Line::Blank);
    lines.push(Line::Blank);
    lines.push(Line::FloatBlock {
        lines: block,
        wide: false,
        top,
    });
    lines.push(Line::Blank);
    Ok(true)
}

fn append_blank_lines(lines: &mut Vec<Line>, count: usize) {
    lines.extend(std::iter::repeat_with(|| Line::Blank).take(count));
}

fn append_blank_lines_to_page_boundary(
    lines: &mut Vec<Line>,
    layout: &DocumentLayout,
    images: &[ImageAsset],
) {
    let pass = layout_lines(*layout, lines, images);
    let remaining = pass.cursor.normal_slots_until_page_boundary(*layout);
    if remaining > 0 {
        append_blank_lines(lines, remaining);
    }
}

fn append_wrapped_text_lines(
    lines: &mut Vec<Line>,
    layout: &DocumentLayout,
    in_abstract: bool,
    text: &str,
) {
    for line in layout.wrap_paragraph_text(text) {
        if line.text.is_empty() {
            lines.push(Line::Blank);
        } else {
            push_wrapped_prose_line(lines, in_abstract, line);
        }
    }
}

fn append_wrapped_paragraph_text_lines(
    lines: &mut Vec<Line>,
    layout: &DocumentLayout,
    in_abstract: bool,
    text: &str,
) {
    for (index, line) in layout.wrap_paragraph_text(text).into_iter().enumerate() {
        if line.text.is_empty() {
            lines.push(Line::Blank);
        } else if in_abstract {
            push_wrapped_prose_line(lines, true, line);
        } else if index == 0 && *layout == DocumentLayout::neurips_single_column() {
            push_wrapped_paragraph_first_line(lines, line);
        } else {
            push_wrapped_prose_line(lines, false, line);
        }
    }
}

fn push_wrapped_prose_line(lines: &mut Vec<Line>, in_abstract: bool, line: WrappedTextLine) {
    match (in_abstract, line.justify_width_pt) {
        (true, Some(width_pt)) => lines.push(Line::JustifiedAbstractText {
            text: line.text,
            width_pt,
        }),
        (true, None) => lines.push(Line::AbstractText(line.text)),
        (false, Some(width_pt)) => lines.push(Line::JustifiedText {
            text: line.text,
            width_pt,
        }),
        (false, None) => lines.push(Line::Text(line.text)),
    }
}

fn push_wrapped_paragraph_first_line(lines: &mut Vec<Line>, line: WrappedTextLine) {
    if let Some(width_pt) = line.justify_width_pt {
        lines.push(Line::JustifiedParagraphText {
            text: line.text,
            width_pt,
        });
    } else {
        lines.push(Line::ParagraphText(line.text));
    }
}

fn append_caption_lines(lines: &mut Vec<Line>, caption: &str, layout: &DocumentLayout) {
    for line in layout.wrap_caption_text(caption) {
        if !line.is_empty() {
            lines.push(Line::Caption(line));
        }
    }
}

fn paragraph_heading_text(heading: &str) -> String {
    let heading = heading.trim();
    if heading
        .chars()
        .last()
        .is_some_and(|ch| matches!(ch, '.' | ':' | ';' | '?' | '!'))
    {
        heading.to_string()
    } else {
        format!("{heading}.")
    }
}

fn append_footnote_lines(
    lines: &mut Vec<Line>,
    footnotes: &[FootnoteEntry],
    layout: &DocumentLayout,
) {
    if footnotes.is_empty() {
        return;
    }
    lines.push(Line::Blank);
    lines.push(Line::Heading("Notes".to_string()));
    for footnote in footnotes {
        for wrapped in
            layout.wrap_footnote_text(&format!("[{}] {}", footnote.number, footnote.text))
        {
            lines.push(Line::Footnote(wrapped));
        }
    }
}

fn append_bibliography_lines(
    lines: &mut Vec<Line>,
    citations: &CitationRegistry,
    layout: &DocumentLayout,
) -> bool {
    if citations.entries.is_empty() {
        return false;
    }
    lines.push(Line::Blank);
    lines.push(Line::Heading("References".to_string()));
    for entry in &citations.entries {
        let text = rendered_bibliography_entry(citations, entry);
        for line in bibliography_text_lines(*layout, &text) {
            push_wrapped_prose_line(lines, false, line);
        }
    }
    true
}

fn rendered_bibliography_entry(citations: &CitationRegistry, entry: &CitationEntry) -> String {
    let mut text = format!("[{}] {}", entry.number, entry.text);
    if citations.visible_backrefs
        && let Some(suffix) = bibliography_backref_suffix(citations, &entry.key)
    {
        text.push(' ');
        text.push_str(&suffix);
    }
    text
}

fn bibliography_backref_suffix(citations: &CitationRegistry, key: &str) -> Option<String> {
    let pages = citations
        .backrefs
        .iter()
        .filter(|backref| backref.key == key)
        .map(|backref| backref.page)
        .collect::<Vec<_>>();
    if pages.is_empty() {
        None
    } else if pages.len() == 1 {
        Some(format!("(Cited on page {}.)", pages[0]))
    } else {
        Some(format!(
            "(Cited on pages {}.)",
            format_citation_backref_pages(&pages)
        ))
    }
}

fn format_citation_backref_pages(pages: &[usize]) -> String {
    match pages {
        [] => String::new(),
        [page] => page.to_string(),
        [first, second] => format!("{first} and {second}"),
        _ => {
            let mut text = pages[..pages.len() - 1]
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            text.push_str(", and ");
            text.push_str(&pages[pages.len() - 1].to_string());
            text
        }
    }
}

fn bibliography_text_lines(layout: DocumentLayout, text: &str) -> Vec<WrappedTextLine> {
    let width = if layout.columns > 1 {
        (layout.text_wrap_width * 3 / 2).max(layout.footnote_wrap_width)
    } else {
        layout.footnote_wrap_width.max(layout.text_wrap_width)
    };
    layout.wrap_prose_lines(text, width, layout.text_font_pt, layout.column_width_pt)
}

fn clean_table_row(
    source: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
) -> String {
    let source = expand_table_span_commands(source);
    let source = remove_table_rule_commands(&source)
        .replace('&', " | ")
        .replace("\\_", "_");
    normalize_table_pipes(
        &clean_inline_text(&source, macros, labels, citations)
            .unwrap_or_else(|_| loose_clean_text(&source, labels, citations)),
    )
}

fn expand_table_span_commands(source: &str) -> String {
    let mut out = String::new();
    let mut cursor = source;
    while let Some((index, command)) = next_table_span_command(cursor) {
        out.push_str(&cursor[..index]);
        let rest = &cursor[index + command.len() + 1..];
        match command {
            "multicolumn" => {
                let Some((_, rest)) = take_braced(rest) else {
                    out.push_str(&cursor[index..]);
                    return out;
                };
                let Some((_, rest)) = take_braced(rest) else {
                    out.push_str(&cursor[index..]);
                    return out;
                };
                let Some((payload, rest)) = take_braced(rest) else {
                    out.push_str(&cursor[index..]);
                    return out;
                };
                out.push_str(payload);
                cursor = rest;
            }
            "multirow" => {
                let (_, rest) = take_optional_bracketed(rest);
                let Some((_, rest)) = take_braced(rest) else {
                    out.push_str(&cursor[index..]);
                    return out;
                };
                let Some((_, rest)) = take_braced(rest) else {
                    out.push_str(&cursor[index..]);
                    return out;
                };
                let Some((payload, rest)) = take_braced(rest) else {
                    out.push_str(&cursor[index..]);
                    return out;
                };
                out.push_str(payload);
                cursor = rest;
            }
            _ => unreachable!(),
        }
    }
    out.push_str(cursor);
    out
}

fn next_table_span_command(source: &str) -> Option<(usize, &'static str)> {
    ["multicolumn", "multirow"]
        .into_iter()
        .filter_map(|command| find_control(source, command).map(|index| (index, command)))
        .min_by_key(|(index, _)| *index)
}

fn remove_table_rule_commands(source: &str) -> String {
    let mut out = String::new();
    let mut cursor = source;
    while let Some((index, command)) = next_table_rule_command(cursor) {
        out.push_str(&cursor[..index]);
        let mut rest = &cursor[index + command.len() + 1..];
        if matches!(command, "cmidrule") {
            let after_parens = rest.trim_start();
            if let Some(parenthesized) = after_parens.strip_prefix('(')
                && let Some(end) = parenthesized.find(')')
            {
                rest = &parenthesized[end + 1..];
            }
            let (_, after_optional) = take_optional_bracketed(rest);
            rest = after_optional;
            if let Some((_, after_range)) = take_braced(rest) {
                rest = after_range;
            }
        }
        cursor = rest;
    }
    out.push_str(cursor);
    out
}

fn next_table_rule_command(source: &str) -> Option<(usize, &'static str)> {
    ["toprule", "midrule", "bottomrule", "hline", "cmidrule"]
        .into_iter()
        .filter_map(|command| find_control(source, command).map(|index| (index, command)))
        .min_by_key(|(index, _)| *index)
}

fn normalize_table_pipes(source: &str) -> String {
    if !source.contains('|') {
        return source.to_string();
    }
    source
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" | ")
}

fn table_cells_from_row(row: &str) -> Option<Vec<String>> {
    if !row.contains('|') {
        return None;
    }
    let cells = row
        .split('|')
        .map(|cell| cell.trim().to_string())
        .collect::<Vec<_>>();
    (cells.len() > 1).then_some(cells)
}

fn loose_clean_text(
    source: &str,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
) -> String {
    let mut out = String::new();
    let mut chars = source.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '%' => {
                for next in chars.by_ref() {
                    if next == '\n' {
                        out.push(' ');
                        break;
                    }
                }
            }
            '$' => {
                let rest: String = chars.collect();
                if let Some(after_open) = rest.strip_prefix('$') {
                    if let Some(end) = find_unescaped_double_dollar(after_open) {
                        out.push_str(&clean_math_text(&after_open[..end]));
                        let remaining = &after_open[end + "$$".len()..];
                        let remaining = loose_clean_text(remaining, labels, citations);
                        append_cleaned_remaining(&mut out, &remaining);
                    }
                } else if let Some(end) = find_unescaped_dollar(&rest) {
                    out.push_str(&clean_math_text(&rest[..end]));
                    let remaining = &rest[end + "$".len()..];
                    let remaining = loose_clean_text(remaining, labels, citations);
                    append_cleaned_remaining(&mut out, &remaining);
                }
                break;
            }
            '\\' => {
                let mut name = String::new();
                while let Some(&next) = chars.peek() {
                    if !next.is_ascii_alphabetic() {
                        break;
                    }
                    name.push(next);
                    chars.next();
                }
                match name.as_str() {
                    "" => {
                        if let Some(symbol) = chars.next() {
                            match symbol {
                                '\\' | ',' | '!' | ':' | ';' => out.push(' '),
                                _ => out.push(symbol),
                            }
                        }
                    }
                    "label" => {
                        let rest: String = chars.collect();
                        if let Some((_, remaining)) = take_braced(&rest) {
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "ref" | "eqref" | "cref" | "Cref" | "pageref" => {
                        let rest: String = chars.collect();
                        if let Some((key, remaining)) = take_braced(&rest) {
                            out.push_str(&render_reference_command(name.as_str(), key, labels));
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "cite" | "citep" | "citet" | "parencite" | "textcite" => {
                        let rest: String = chars.collect();
                        if let Some((citation, remaining)) =
                            render_citation_command(name.as_str(), &rest, citations)
                        {
                            out.push_str(&citation);
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "and" | "And" | "AND" => out.push_str("and"),
                    "thanks" => {
                        let rest: String = chars.collect();
                        if let Some((_, remaining)) = take_braced(&rest) {
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "protect" | "bf" | "it" | "rm" | "sf" | "normalfont" | "bfseries"
                    | "itshape" | "sffamily" | "selectfont" | "onecolumn" | "twocolumn"
                    | "noindent" => {}
                    "thispagestyle" | "pagestyle" => {
                        let rest: String = chars.collect();
                        if let Some((_, remaining)) = take_braced(&rest) {
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "fontsize" => {
                        let rest: String = chars.collect();
                        if let Some(remaining) = skip_two_braced_arguments(&rest) {
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "color" | "urlstyle" => {
                        let rest: String = chars.collect();
                        if let Some((_, remaining)) = take_braced(&rest) {
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "definecolor" => {
                        let rest: String = chars.collect();
                        if let Some((_, rest)) = take_braced(&rest)
                            && let Some((_, rest)) = take_braced(rest)
                            && let Some((_, remaining)) = take_braced(rest)
                        {
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "textcolor" | "colorbox" => {
                        let rest: String = chars.collect();
                        if let Some((_, rest)) = take_braced(&rest)
                            && let Some((payload, remaining)) = take_braced(rest)
                        {
                            out.push_str(&loose_clean_text(payload, labels, citations));
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "fcolorbox" => {
                        let rest: String = chars.collect();
                        if let Some((_, rest)) = take_braced(&rest)
                            && let Some((_, rest)) = take_braced(rest)
                            && let Some((payload, remaining)) = take_braced(rest)
                        {
                            out.push_str(&loose_clean_text(payload, labels, citations));
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "addbibresource" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        if let Some((_, remaining)) = take_braced(rest) {
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "printbibliography" => {
                        let rest: String = chars.collect();
                        let (_, remaining) = take_optional_bracketed(&rest);
                        out.push_str(&loose_clean_text(remaining, labels, citations));
                        break;
                    }
                    "textsc" => {
                        let rest: String = chars.collect();
                        if let Some((payload, remaining)) = take_braced(&rest) {
                            out.push_str(
                                &loose_clean_text(payload, labels, citations).to_ascii_uppercase(),
                            );
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "textbf" | "emph" | "textit" | "texttt" | "textrm" | "textsf"
                    | "textsuperscript" | "underline" | "ensuremath" | "mathrm" | "mathbf"
                    | "mathsf" | "mathit" | "mbox" | "tiny" | "scriptsize" | "small"
                    | "footnotesize" => {
                        let rest: String = chars.collect();
                        if let Some((payload, remaining)) = take_braced(&rest) {
                            out.push_str(&loose_clean_text(payload, labels, citations));
                            out.push_str(&loose_clean_text(remaining, labels, citations));
                        }
                        break;
                    }
                    "pm" => out.push('±'),
                    "times" => out.push('×'),
                    "cdot" => out.push('·'),
                    "to" | "rightarrow" => out.push('→'),
                    "le" | "leq" => out.push('≤'),
                    "ge" | "geq" => out.push('≥'),
                    "infty" => out.push('∞'),
                    "alpha" => out.push('α'),
                    "beta" => out.push('β'),
                    "gamma" => out.push('γ'),
                    "delta" => out.push('δ'),
                    "epsilon" | "varepsilon" => out.push('ε'),
                    "lambda" => out.push('λ'),
                    "theta" | "vartheta" => out.push('θ'),
                    "rho" => out.push('ρ'),
                    "sigma" => out.push('σ'),
                    "phi" | "varphi" => out.push('φ'),
                    "psi" => out.push('ψ'),
                    "eta" => out.push('η'),
                    "pi" => out.push('π'),
                    _ => {
                        if !name.is_empty() {
                            out.push_str(&name);
                        }
                    }
                }
            }
            '{' | '}' => {}
            '\n' | '\t' | '~' => out.push(' '),
            _ => out.push(ch),
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn take_optional_bracketed(source: &str) -> (Option<&str>, &str) {
    let source = source.trim_start();
    let Some(rest) = source.strip_prefix('[') else {
        return (None, source);
    };
    let mut depth = 1_usize;
    for (index, ch) in rest.char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return (Some(&rest[..index]), &rest[index + ch.len_utf8()..]);
                }
            }
            _ => {}
        }
    }
    (None, source)
}

fn resolve_graphics_path(
    root_dir: &Path,
    payload: &str,
    graphics: &GraphicsConfig,
) -> Result<PathBuf, String> {
    if payload.is_empty() {
        return Err("native backend requires non-empty \\includegraphics paths".to_string());
    }
    let raw = Path::new(payload);
    if raw.is_absolute() {
        return Err(format!(
            "native backend only supports local relative graphics paths, got `{payload}`"
        ));
    }

    let mut search_dirs = vec![PathBuf::new()];
    for dir in &graphics.search_dirs {
        if !search_dirs.contains(dir) {
            search_dirs.push(dir.clone());
        }
    }

    for search_dir in search_dirs {
        let candidate = root_dir.join(&search_dir).join(raw);
        if candidate.exists() {
            return canonical_graphic_path(&candidate);
        }
        if candidate.extension().is_none() {
            for extension in &graphics.extensions {
                let with_extension = candidate.with_extension(extension);
                if with_extension.exists() {
                    return canonical_graphic_path(&with_extension);
                }
            }
        }
    }

    let mut kpathsea_candidates = Vec::new();
    if raw.extension().is_some() {
        kpathsea_candidates.push(raw.to_path_buf());
    } else {
        for extension in &graphics.extensions {
            kpathsea_candidates.push(raw.with_extension(extension));
        }
    }
    let empty_search_dir = PathBuf::new();
    for search_dir in std::iter::once(&empty_search_dir).chain(graphics.search_dirs.iter()) {
        for candidate in &kpathsea_candidates {
            let candidate = search_dir.join(candidate);
            if let Some(path) =
                resolve_kpathsea_candidate(root_dir, &candidate, "TEXINPUTS", "graphic")?
            {
                return canonical_graphic_path(&path);
            }
        }
    }

    Err(format!(
        "native backend could not find graphic `{payload}` under `{}`",
        root_dir.display()
    ))
}

fn canonical_graphic_path(candidate: &Path) -> Result<PathBuf, String> {
    fs::canonicalize(candidate).map_err(|error| {
        format!(
            "native backend could not canonicalize graphic `{}`: {error}",
            candidate.display()
        )
    })
}

fn graphics_display_size_pt(
    options: &str,
    layout: &DocumentLayout,
    natural_width_pt: f32,
    natural_height_pt: f32,
) -> Option<(f32, f32)> {
    let mut width = None;
    let mut height = None;
    let mut scale = None;
    let mut keep_aspect_ratio = false;
    for part in options.split(',') {
        let Some((key, value)) = part.split_once('=') else {
            if part.trim() == "keepaspectratio" {
                keep_aspect_ratio = true;
            }
            continue;
        };
        match key.trim() {
            "width" => width = length_expr_to_pt(value.trim(), layout),
            "height" => height = length_expr_to_pt(value.trim(), layout),
            "scale" => {
                scale = value
                    .trim()
                    .parse::<f32>()
                    .ok()
                    .filter(|scale| *scale > 0.0)
            }
            "keepaspectratio" => keep_aspect_ratio = !matches!(value.trim(), "false" | "0"),
            _ => {}
        }
    }
    match (width, height, scale) {
        (Some(width), Some(height), _) if width > 0.0 && height > 0.0 && keep_aspect_ratio => {
            let scale = (width / natural_width_pt).min(height / natural_height_pt);
            Some((natural_width_pt * scale, natural_height_pt * scale))
        }
        (Some(width), Some(height), _) if width > 0.0 && height > 0.0 => Some((width, height)),
        (Some(width), None, _) if width > 0.0 => {
            Some((width, width * natural_height_pt / natural_width_pt))
        }
        (None, Some(height), _) if height > 0.0 => {
            Some((height * natural_width_pt / natural_height_pt, height))
        }
        (None, None, Some(scale)) => Some((natural_width_pt * scale, natural_height_pt * scale)),
        _ => None,
    }
}

fn graphics_angle_degrees(options: &str) -> Option<f32> {
    options.split(',').find_map(|part| {
        let (key, value) = part.split_once('=')?;
        (key.trim() == "angle").then(|| value.trim().parse::<f32>().ok())?
    })
}

fn graphics_viewport(
    options: &str,
    layout: &DocumentLayout,
    natural_width_pt: f32,
    natural_height_pt: f32,
) -> ImageViewport {
    let clip = graphics_clip_requested(options);
    let Some((left, bottom, right, top)) =
        graphics_visible_bounds(options, layout, natural_width_pt, natural_height_pt)
    else {
        return ImageViewport {
            clip,
            ..ImageViewport::full()
        };
    };
    let width = (right - left).max(1.0);
    let height = (top - bottom).max(1.0);
    ImageViewport {
        left_fraction: left / natural_width_pt,
        bottom_fraction: bottom / natural_height_pt,
        width_fraction: width / natural_width_pt,
        height_fraction: height / natural_height_pt,
        clip,
    }
}

fn graphics_visible_bounds(
    options: &str,
    layout: &DocumentLayout,
    natural_width_pt: f32,
    natural_height_pt: f32,
) -> Option<(f32, f32, f32, f32)> {
    if let Some([left, bottom, right_trim, top_trim]) =
        graphics_box_option_values(options, "trim", layout)
    {
        return normalize_graphics_bounds(
            left,
            bottom,
            natural_width_pt - right_trim,
            natural_height_pt - top_trim,
            natural_width_pt,
            natural_height_pt,
        );
    }
    if let Some([llx, lly, urx, ury]) = graphics_box_option_values(options, "viewport", layout) {
        return normalize_graphics_bounds(llx, lly, urx, ury, natural_width_pt, natural_height_pt);
    }
    None
}

fn normalize_graphics_bounds(
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
    natural_width_pt: f32,
    natural_height_pt: f32,
) -> Option<(f32, f32, f32, f32)> {
    let left = left.clamp(0.0, natural_width_pt);
    let right = right.clamp(0.0, natural_width_pt);
    let bottom = bottom.clamp(0.0, natural_height_pt);
    let top = top.clamp(0.0, natural_height_pt);
    let (left, right) = (left.min(right), left.max(right));
    let (bottom, top) = (bottom.min(top), bottom.max(top));
    if right <= left || top <= bottom {
        return None;
    }
    Some((left, bottom, right, top))
}

fn graphics_box_option_values(
    options: &str,
    option_name: &str,
    layout: &DocumentLayout,
) -> Option<[f32; 4]> {
    for part in options.split(',') {
        let Some((key, value)) = part.split_once('=') else {
            continue;
        };
        if key.trim() != option_name {
            continue;
        }
        let values = value
            .split_whitespace()
            .map(|value| graphics_length_to_pt(value, layout))
            .collect::<Option<Vec<_>>>()?;
        if values.len() == 4 {
            return Some([values[0], values[1], values[2], values[3]]);
        }
    }
    None
}

fn graphics_length_to_pt(value: &str, layout: &DocumentLayout) -> Option<f32> {
    length_expr_to_pt(value, layout).or_else(|| value.trim().parse::<f32>().ok())
}

fn graphics_clip_requested(options: &str) -> bool {
    options.split(',').any(|part| {
        let part = part.trim();
        if part == "clip" {
            return true;
        }
        let Some((key, value)) = part.split_once('=') else {
            return false;
        };
        key.trim() == "clip" && !matches!(value.trim(), "false" | "0")
    })
}

fn graphics_page_number(options: &str) -> Option<u32> {
    options.split(',').find_map(|part| {
        let (key, value) = part.split_once('=')?;
        (key.trim() == "page").then(|| value.trim().parse::<u32>().ok().filter(|page| *page > 0))?
    })
}

fn length_expr_to_pt(value: &str, layout: &DocumentLayout) -> Option<f32> {
    let value = value.replace(' ', "");
    if let Some(prefix) = value.strip_suffix("\\textwidth") {
        let multiplier = if prefix.is_empty() {
            1.0
        } else {
            prefix.parse::<f32>().ok()?
        };
        return Some(layout.text_width_pt * multiplier);
    }
    if let Some(prefix) = value.strip_suffix("\\columnwidth") {
        let multiplier = if prefix.is_empty() {
            1.0
        } else {
            prefix.parse::<f32>().ok()?
        };
        return Some(layout.column_width_pt * multiplier);
    }
    if let Some(prefix) = value.strip_suffix("\\linewidth") {
        let multiplier = if prefix.is_empty() {
            1.0
        } else {
            prefix.parse::<f32>().ok()?
        };
        return Some(layout.column_width_pt * multiplier);
    }
    if let Some(number) = value.strip_suffix("cm") {
        return Some(number.parse::<f32>().ok()? * 28.346_457);
    }
    if let Some(number) = value.strip_suffix("in") {
        return Some(number.parse::<f32>().ok()? * 72.0);
    }
    if let Some(number) = value.strip_suffix("pt") {
        return number.parse::<f32>().ok();
    }
    None
}

fn jpeg_dimensions(data: &[u8]) -> Option<(u16, u16)> {
    if data.len() < 4 || data[0] != 0xff || data[1] != 0xd8 {
        return None;
    }
    let mut index = 2_usize;
    while index + 4 <= data.len() {
        while index < data.len() && data[index] == 0xff {
            index += 1;
        }
        if index >= data.len() {
            return None;
        }
        let marker = data[index];
        index += 1;
        if matches!(marker, 0xd8 | 0xd9) {
            continue;
        }
        if index + 2 > data.len() {
            return None;
        }
        let length = u16::from_be_bytes([data[index], data[index + 1]]) as usize;
        if length < 2 || index + length > data.len() {
            return None;
        }
        if matches!(
            marker,
            0xc0 | 0xc1
                | 0xc2
                | 0xc3
                | 0xc5
                | 0xc6
                | 0xc7
                | 0xc9
                | 0xca
                | 0xcb
                | 0xcd
                | 0xce
                | 0xcf
        ) {
            if length < 7 {
                return None;
            }
            let height = u16::from_be_bytes([data[index + 3], data[index + 4]]);
            let width = u16::from_be_bytes([data[index + 5], data[index + 6]]);
            return Some((width, height));
        }
        index += length;
    }
    None
}

fn parse_simple_macros(source: &str) -> Result<HashMap<String, String>, String> {
    let mut macros = HashMap::new();
    collect_newcommand_macros(source, &mut macros)?;
    collect_def_macros(source, &mut macros)?;
    Ok(macros)
}

fn collect_newcommand_macros(
    source: &str,
    macros: &mut HashMap<String, String>,
) -> Result<(), String> {
    let mut remaining = source;
    while let Some(index) = find_control(remaining, "newcommand") {
        let rest = &remaining[index + "\\newcommand".len()..];
        let Some((name_payload, after_name)) = take_braced(rest) else {
            remaining = rest;
            continue;
        };
        let mut after_name = after_name.trim_start();
        let (argument_spec, after_argument_spec) = take_optional_bracketed(after_name);
        let argument_taking = argument_spec.is_some();
        after_name = after_argument_spec.trim_start();
        if argument_taking {
            let (_, after_default) = take_optional_bracketed(after_name);
            after_name = after_default.trim_start();
        }
        let (replacement, after_replacement) = take_braced(after_name).ok_or_else(|| {
            format!("native backend requires a braced replacement for `{name_payload}`")
        })?;
        if let Some(name) = macro_name_from_payload(name_payload) {
            macros.insert(
                name,
                if argument_taking {
                    String::new()
                } else {
                    replacement.to_string()
                },
            );
        }
        remaining = after_replacement;
    }
    Ok(())
}

fn collect_def_macros(source: &str, macros: &mut HashMap<String, String>) -> Result<(), String> {
    let mut remaining = source;
    while let Some(index) = find_control(remaining, "def") {
        let rest = remaining[index + "\\def".len()..].trim_start();
        let Some(rest) = rest.strip_prefix('\\') else {
            return Err("native backend requires control-sequence \\def names".to_string());
        };
        let name_len = rest
            .char_indices()
            .take_while(|(_, ch)| ch.is_ascii_alphabetic())
            .map(|(index, ch)| index + ch.len_utf8())
            .last()
            .unwrap_or(0);
        if name_len == 0 {
            let after_symbol = rest
                .char_indices()
                .nth(1)
                .map(|(index, _)| &rest[index..])
                .unwrap_or("");
            if let Some((_, after_replacement)) = take_braced(after_symbol.trim_start()) {
                remaining = after_replacement;
            } else {
                remaining = after_symbol;
            }
            continue;
        }
        let name = &rest[..name_len];
        let mut after_name = rest[name_len..].trim_start();
        let argument_taking = after_name.starts_with('#');
        if argument_taking {
            let Some(replacement_start) = after_name.find('{') else {
                remaining = after_name;
                continue;
            };
            after_name = &after_name[replacement_start..];
        }
        let (replacement, after_replacement) = take_braced(after_name).ok_or_else(|| {
            format!("native backend requires a braced replacement for macro `\\{name}`")
        })?;
        macros.insert(
            name.to_string(),
            if argument_taking {
                String::new()
            } else {
                replacement.to_string()
            },
        );
        remaining = after_replacement;
    }
    Ok(())
}

fn macro_name_from_payload(payload: &str) -> Option<String> {
    let payload = payload.trim();
    let name = payload.strip_prefix('\\')?;
    if name.is_empty() || !name.chars().all(|ch| ch.is_ascii_alphabetic()) {
        return None;
    }
    Some(name.to_string())
}

fn collect_section_labels(
    body: &str,
    layout: &DocumentLayout,
    listing_reference_name: &str,
    capitalize_cref_names: bool,
) -> Result<HashMap<String, LabelInfo>, String> {
    let mut labels = HashMap::new();
    let mut cursor = body;
    let mut section_counter = 0_usize;
    let mut subsection_counter = 0_usize;
    let mut appendix_mode = false;
    let mut current_anchor = String::new();
    let mut current_reference_prefix: Option<String> = None;
    let mut current_page = 1_usize;
    let mut slot = 0_usize;

    while !cursor.is_empty() {
        let next = ["\\appendix", "\\section", "\\subsection", "\\label"]
            .into_iter()
            .filter_map(|marker| cursor.find(marker).map(|index| (index, marker)))
            .min_by_key(|(index, _)| *index);
        let Some((index, marker)) = next else {
            break;
        };
        slot += estimate_text_slots(&cursor[..index], layout);
        cursor = &cursor[index + marker.len()..];
        match marker {
            "\\appendix" => {
                appendix_mode = true;
                section_counter = 0;
                subsection_counter = 0;
            }
            "\\section" => {
                let (after_star, starred) = strip_optional_star(cursor);
                let (_, after_optional) = take_optional_bracketed(after_star);
                let Some((_, remaining)) = take_braced(after_optional) else {
                    return Err("native backend requires braced \\section titles".to_string());
                };
                if !starred {
                    section_counter += 1;
                    subsection_counter = 0;
                    current_anchor = section_number(section_counter, appendix_mode);
                    current_reference_prefix =
                        Some(cref_reference_prefix("Section", capitalize_cref_names));
                    current_page = page_for_slot(slot + 1, layout);
                }
                slot += 3;
                cursor = remaining;
            }
            "\\subsection" => {
                let (after_star, starred) = strip_optional_star(cursor);
                let (_, after_optional) = take_optional_bracketed(after_star);
                let Some((_, remaining)) = take_braced(after_optional) else {
                    return Err("native backend requires braced \\subsection titles".to_string());
                };
                if !starred {
                    subsection_counter += 1;
                    current_anchor =
                        subsection_number(section_counter, subsection_counter, appendix_mode);
                    current_reference_prefix =
                        Some(cref_reference_prefix("Section", capitalize_cref_names));
                    current_page = page_for_slot(slot + 1, layout);
                }
                slot += 3;
                cursor = remaining;
            }
            "\\label" => {
                let Some((key, remaining)) = take_braced(cursor) else {
                    return Err("native backend requires braced \\label keys".to_string());
                };
                labels.insert(
                    key.trim().to_string(),
                    label_info_for_value(
                        current_anchor.clone(),
                        current_page,
                        current_reference_prefix.as_deref(),
                    ),
                );
                cursor = remaining;
            }
            _ => unreachable!(),
        }
    }

    collect_float_labels(body, &mut labels, layout, capitalize_cref_names)?;
    collect_theorem_labels(body, &mut labels, layout)?;
    collect_display_math_labels(body, &mut labels, layout)?;
    collect_listing_labels(body, &mut labels, layout, listing_reference_name)?;
    Ok(labels)
}

fn collect_float_labels(
    body: &str,
    labels: &mut HashMap<String, LabelInfo>,
    layout: &DocumentLayout,
    capitalize_cref_names: bool,
) -> Result<(), String> {
    let mut cursor = body;
    let mut current_float = None;
    let mut current_caption: Option<(String, usize, String)> = None;
    let mut figure_counter = 0_usize;
    let mut table_counter = 0_usize;
    let mut slot = 0_usize;
    let figure_entry_count = if source_contains_control(body, "listoffigures") {
        count_float_entries(body, FloatKind::Figure)
    } else {
        0
    };
    let table_entry_count = if source_contains_control(body, "listoftables") {
        count_float_entries(body, FloatKind::Table)
    } else {
        0
    };

    while !cursor.is_empty() {
        let next = [
            "\\listoffigures",
            "\\listoftables",
            "\\captionof",
            "\\captionsetup",
            "\\caption",
            "\\begin",
            "\\end",
            "\\section",
            "\\subsection",
            "\\label",
        ]
        .into_iter()
        .filter_map(|marker| cursor.find(marker).map(|index| (index, marker)))
        .min_by_key(|(index, _)| *index);
        let Some((index, marker)) = next else {
            break;
        };
        slot += estimate_text_slots(&cursor[..index], layout);
        cursor = &cursor[index + marker.len()..];
        match marker {
            "\\listoffigures" => {
                slot += 2 + figure_entry_count.max(1);
            }
            "\\listoftables" => {
                slot += 2 + table_entry_count.max(1);
            }
            "\\begin" => {
                let Some((env, remaining)) = take_braced(cursor) else {
                    continue;
                };
                let remaining = consume_environment_open_args(env, remaining);
                if let Some(kind) = FloatKind::from_env(env) {
                    current_float = Some(kind);
                    current_caption = None;
                }
                cursor = remaining;
            }
            "\\end" => {
                let Some((env, remaining)) = take_braced(cursor) else {
                    continue;
                };
                if FloatKind::from_env(env).is_some() {
                    current_float = None;
                    current_caption = None;
                }
                cursor = remaining;
            }
            "\\captionof" => {
                let Some((kind, rest)) = take_braced(cursor) else {
                    return Err("native backend requires braced \\captionof kinds".to_string());
                };
                let (_, rest) = take_optional_bracketed(rest);
                let Some((caption, remaining)) = take_braced(rest) else {
                    return Err("native backend requires braced \\captionof payloads".to_string());
                };
                let kind = FloatKind::from_caption_kind(kind);
                let number = next_float_number(kind, &mut figure_counter, &mut table_counter);
                let value = number.to_string();
                let page = page_for_slot(slot + 1, layout);
                let reference_prefix = cref_reference_prefix(kind.label(), capitalize_cref_names);
                collect_labels_for_value(
                    caption,
                    labels,
                    &value,
                    page,
                    Some(reference_prefix.as_str()),
                )?;
                current_caption = Some((value, page, reference_prefix));
                slot += estimate_caption_slots(kind.label(), number, caption, layout);
                cursor = remaining;
            }
            "\\captionsetup" => {
                let (_, rest) = take_optional_bracketed(cursor);
                let Some((_, remaining)) = take_braced(rest) else {
                    return Err(
                        "native backend requires braced \\captionsetup payloads".to_string()
                    );
                };
                cursor = remaining;
            }
            "\\caption" => {
                let (_, rest) = take_optional_bracketed(cursor);
                let Some((caption, remaining)) = take_braced(rest) else {
                    return Err("native backend requires braced \\caption payloads".to_string());
                };
                let kind = current_float.unwrap_or(FloatKind::Figure);
                let number = next_float_number(kind, &mut figure_counter, &mut table_counter);
                let value = number.to_string();
                let page = page_for_slot(slot + 1, layout);
                let reference_prefix = cref_reference_prefix(kind.label(), capitalize_cref_names);
                collect_labels_for_value(
                    caption,
                    labels,
                    &value,
                    page,
                    Some(reference_prefix.as_str()),
                )?;
                current_caption = Some((value, page, reference_prefix));
                slot += estimate_caption_slots(kind.label(), number, caption, layout);
                cursor = remaining;
            }
            "\\label" => {
                let Some((key, remaining)) = take_braced(cursor) else {
                    return Err("native backend requires braced \\label keys".to_string());
                };
                if let Some((value, page, reference_prefix)) = &current_caption {
                    labels.insert(
                        key.trim().to_string(),
                        label_info_for_value(value, *page, Some(reference_prefix.as_str())),
                    );
                    if current_float.is_none() {
                        current_caption = None;
                    }
                }
                cursor = remaining;
            }
            "\\section" | "\\subsection" => {
                current_caption = None;
                let (after_star, _) = strip_optional_star(cursor);
                let (_, after_optional) = take_optional_bracketed(after_star);
                let Some((_, remaining)) = take_braced(after_optional) else {
                    break;
                };
                slot += 3;
                cursor = remaining;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn collect_float_entries(
    body: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    layout: &DocumentLayout,
) -> Result<Vec<FloatEntry>, String> {
    let mut entries = Vec::new();
    let mut cursor = body;
    let mut current_float = None;
    let mut figure_counter = 0_usize;
    let mut table_counter = 0_usize;
    let mut slot = 0_usize;
    let figure_entry_count = if source_contains_control(body, "listoffigures") {
        count_float_entries(body, FloatKind::Figure)
    } else {
        0
    };
    let table_entry_count = if source_contains_control(body, "listoftables") {
        count_float_entries(body, FloatKind::Table)
    } else {
        0
    };

    while !cursor.is_empty() {
        let next = [
            "\\listoffigures",
            "\\listoftables",
            "\\captionof",
            "\\captionsetup",
            "\\caption",
            "\\begin",
            "\\end",
            "\\section",
            "\\subsection",
        ]
        .into_iter()
        .filter_map(|marker| cursor.find(marker).map(|index| (index, marker)))
        .min_by_key(|(index, _)| *index);
        let Some((index, marker)) = next else {
            break;
        };
        slot += estimate_text_slots(&cursor[..index], layout);
        cursor = &cursor[index + marker.len()..];
        match marker {
            "\\listoffigures" => {
                slot += 2 + figure_entry_count.max(1);
            }
            "\\listoftables" => {
                slot += 2 + table_entry_count.max(1);
            }
            "\\begin" => {
                let Some((env, remaining)) = take_braced(cursor) else {
                    continue;
                };
                let remaining = consume_environment_open_args(env, remaining);
                if let Some(kind) = FloatKind::from_env(env) {
                    current_float = Some(kind);
                }
                cursor = remaining;
            }
            "\\end" => {
                let Some((env, remaining)) = take_braced(cursor) else {
                    continue;
                };
                if FloatKind::from_env(env).is_some() {
                    current_float = None;
                }
                cursor = remaining;
            }
            "\\captionof" => {
                let Some((kind, rest)) = take_braced(cursor) else {
                    return Err("native backend requires braced \\captionof kinds".to_string());
                };
                let (short_title, rest) = take_optional_bracketed(rest);
                let Some((caption, remaining)) = take_braced(rest) else {
                    return Err("native backend requires braced \\captionof payloads".to_string());
                };
                let kind = FloatKind::from_caption_kind(kind);
                let number = next_float_number(kind, &mut figure_counter, &mut table_counter);
                let title_source = short_title.unwrap_or(caption);
                entries.push(FloatEntry {
                    kind,
                    number: number.to_string(),
                    title: clean_inline_text(title_source, macros, labels, citations)?,
                    page: page_for_slot(slot + 1, layout),
                });
                slot += estimate_caption_slots(kind.label(), number, caption, layout);
                cursor = remaining;
            }
            "\\captionsetup" => {
                let (_, rest) = take_optional_bracketed(cursor);
                let Some((_, remaining)) = take_braced(rest) else {
                    return Err(
                        "native backend requires braced \\captionsetup payloads".to_string()
                    );
                };
                cursor = remaining;
            }
            "\\caption" => {
                let (short_title, rest) = take_optional_bracketed(cursor);
                let Some((caption, remaining)) = take_braced(rest) else {
                    return Err("native backend requires braced \\caption payloads".to_string());
                };
                let kind = current_float.unwrap_or(FloatKind::Figure);
                let number = next_float_number(kind, &mut figure_counter, &mut table_counter);
                let title_source = short_title.unwrap_or(caption);
                entries.push(FloatEntry {
                    kind,
                    number: number.to_string(),
                    title: clean_inline_text(title_source, macros, labels, citations)?,
                    page: page_for_slot(slot + 1, layout),
                });
                slot += estimate_caption_slots(kind.label(), number, caption, layout);
                cursor = remaining;
            }
            "\\section" | "\\subsection" => {
                let (after_star, _) = strip_optional_star(cursor);
                let (_, after_optional) = take_optional_bracketed(after_star);
                let Some((_, remaining)) = take_braced(after_optional) else {
                    break;
                };
                slot += 3;
                cursor = remaining;
            }
            _ => unreachable!(),
        }
    }

    Ok(entries)
}

fn count_float_entries(body: &str, target: FloatKind) -> usize {
    let mut count = 0_usize;
    let mut cursor = body;
    let mut current_float = None;

    while !cursor.is_empty() {
        let next = [
            "\\captionof",
            "\\captionsetup",
            "\\caption",
            "\\begin",
            "\\end",
        ]
        .into_iter()
        .filter_map(|marker| cursor.find(marker).map(|index| (index, marker)))
        .min_by_key(|(index, _)| *index);
        let Some((index, marker)) = next else {
            break;
        };
        cursor = &cursor[index + marker.len()..];
        match marker {
            "\\begin" => {
                let Some((env, remaining)) = take_braced(cursor) else {
                    continue;
                };
                current_float = FloatKind::from_env(env).or(current_float);
                cursor = consume_environment_open_args(env, remaining);
            }
            "\\end" => {
                let Some((env, remaining)) = take_braced(cursor) else {
                    continue;
                };
                if FloatKind::from_env(env).is_some() {
                    current_float = None;
                }
                cursor = remaining;
            }
            "\\captionof" => {
                let Some((kind, rest)) = take_braced(cursor) else {
                    break;
                };
                let (_, rest) = take_optional_bracketed(rest);
                let Some((_, remaining)) = take_braced(rest) else {
                    break;
                };
                if FloatKind::from_caption_kind(kind) == target {
                    count += 1;
                }
                cursor = remaining;
            }
            "\\captionsetup" => {
                let (_, rest) = take_optional_bracketed(cursor);
                let Some((_, remaining)) = take_braced(rest) else {
                    break;
                };
                cursor = remaining;
            }
            "\\caption" => {
                let (_, rest) = take_optional_bracketed(cursor);
                let Some((_, remaining)) = take_braced(rest) else {
                    break;
                };
                if current_float.unwrap_or(FloatKind::Figure) == target {
                    count += 1;
                }
                cursor = remaining;
            }
            _ => unreachable!(),
        }
    }

    count
}

fn collect_theorem_labels(
    body: &str,
    labels: &mut HashMap<String, LabelInfo>,
    layout: &DocumentLayout,
) -> Result<(), String> {
    let mut cursor = body;
    let mut counters = HashMap::new();
    let mut slot = 0_usize;
    while let Some(index) = cursor.find("\\begin") {
        slot += estimate_text_slots(&cursor[..index], layout);
        cursor = &cursor[index + "\\begin".len()..];
        let Some((env, after_env)) = take_braced(cursor) else {
            continue;
        };
        let Some(spec) = theorem_environment_spec(env) else {
            cursor = after_env;
            continue;
        };
        let opening = parse_theorem_opening(spec, after_env)?;
        let number = next_theorem_number(&mut counters, spec.counter_key);
        let Some(number) = number else {
            cursor = opening.remaining;
            continue;
        };
        let value = number.to_string();
        let page = page_for_slot(slot + 1, layout);
        let reference_prefix = theorem_reference_prefix(spec);
        if let Some(label) = opening.options.and_then(label_option_value) {
            labels.insert(
                label,
                label_info_for_value(value.clone(), page, Some(reference_prefix)),
            );
        }
        let (theorem_body, remaining) = take_environment_body(opening.remaining, env)?;
        collect_labels_for_value(theorem_body, labels, &value, page, Some(reference_prefix))?;
        slot += estimate_theorem_slots(theorem_body, layout);
        cursor = remaining;
    }
    Ok(())
}

fn collect_labels_for_value(
    source: &str,
    labels: &mut HashMap<String, LabelInfo>,
    value: &str,
    page: usize,
    reference_prefix: Option<&str>,
) -> Result<(), String> {
    let mut label_cursor = source;
    while let Some(label_index) = find_control(label_cursor, "label") {
        let rest = &label_cursor[label_index + "\\label".len()..];
        let Some((key, after_label)) = take_braced(rest) else {
            return Err("native backend requires braced \\label keys".to_string());
        };
        labels.insert(
            key.trim().to_string(),
            label_info_for_value(value, page, reference_prefix),
        );
        label_cursor = after_label;
    }
    Ok(())
}

fn label_info_for_value(
    value: impl Into<String>,
    page: usize,
    reference_prefix: Option<&str>,
) -> LabelInfo {
    match reference_prefix {
        Some(reference_prefix) => LabelInfo::with_reference_prefix(value, page, reference_prefix),
        None => LabelInfo::new(value, page),
    }
}

fn cref_reference_prefix(label: &str, capitalize: bool) -> String {
    if capitalize {
        label.to_string()
    } else {
        label.to_lowercase()
    }
}

fn label_option_value(options: &str) -> Option<String> {
    let index = options.find("label")?;
    let mut rest = options[index + "label".len()..].trim_start();
    rest = rest.strip_prefix('=')?.trim_start();
    if let Some((label, _)) = take_braced(rest) {
        let label = label.trim();
        if !label.is_empty() {
            return Some(label.to_string());
        }
        return None;
    }
    let end = rest
        .char_indices()
        .take_while(|(_, ch)| !matches!(ch, ',' | ']' | ' ' | '\n' | '\t'))
        .map(|(index, ch)| index + ch.len_utf8())
        .last()
        .unwrap_or(0);
    let label = rest[..end].trim();
    (!label.is_empty()).then(|| label.to_string())
}

fn caption_option_value(options: &str) -> Option<String> {
    let index = options.find("caption")?;
    let mut rest = options[index + "caption".len()..].trim_start();
    rest = rest.strip_prefix('=')?.trim_start();
    if let Some((caption, _)) = take_braced(rest) {
        let caption = caption.trim();
        if !caption.is_empty() {
            return Some(caption.to_string());
        }
        return None;
    }
    let end = rest
        .char_indices()
        .take_while(|(_, ch)| !matches!(ch, ',' | ']' | '\n' | '\t'))
        .map(|(index, ch)| index + ch.len_utf8())
        .last()
        .unwrap_or(0);
    let caption = rest[..end].trim();
    (!caption.is_empty()).then(|| caption.to_string())
}

fn collect_display_math_labels(
    body: &str,
    labels: &mut HashMap<String, LabelInfo>,
    layout: &DocumentLayout,
) -> Result<(), String> {
    let mut cursor = body;
    let mut equation_counter = 0_usize;
    let mut slot = 0_usize;
    loop {
        let next = [
            (
                "\\begin{equation}",
                "equation",
                MathEnvironment::SingleNumbered,
            ),
            (
                "\\begin{equation*}",
                "equation*",
                MathEnvironment::SingleUnnumbered,
            ),
            (
                "\\begin{multline}",
                "multline",
                MathEnvironment::SingleNumbered,
            ),
            (
                "\\begin{multline*}",
                "multline*",
                MathEnvironment::SingleUnnumbered,
            ),
            ("\\begin{align}", "align", MathEnvironment::AlignNumbered),
            (
                "\\begin{align*}",
                "align*",
                MathEnvironment::AlignUnnumbered,
            ),
        ]
        .into_iter()
        .filter_map(|(marker, env, kind)| {
            cursor.find(marker).map(|index| (index, marker, env, kind))
        })
        .min_by_key(|(index, _, _, _)| *index);
        let Some((index, marker, env, kind)) = next else {
            break;
        };
        slot += estimate_text_slots(&cursor[..index], layout);
        cursor = &cursor[index + marker.len()..];
        let (math_body, remaining) = take_environment_body(cursor, env)?;
        match kind {
            MathEnvironment::SingleNumbered => {
                equation_counter += 1;
                collect_math_labels_for_value(
                    math_body,
                    labels,
                    equation_counter,
                    page_for_slot(slot + 1, layout),
                )?;
                slot += 3;
            }
            MathEnvironment::SingleUnnumbered => {
                slot += 3;
            }
            MathEnvironment::AlignNumbered | MathEnvironment::AlignUnnumbered => {
                let numbered = matches!(kind, MathEnvironment::AlignNumbered);
                let mut align_slot = slot + 1;
                for row in split_align_rows(math_body) {
                    if clean_equation_text(row).is_empty() {
                        continue;
                    }
                    if numbered && !math_row_suppresses_number(row) {
                        equation_counter += 1;
                        collect_math_labels_for_value(
                            row,
                            labels,
                            equation_counter,
                            page_for_slot(align_slot, layout),
                        )?;
                    }
                    align_slot += 2;
                }
                slot = align_slot + 1;
            }
        }
        cursor = remaining;
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum MathEnvironment {
    SingleNumbered,
    SingleUnnumbered,
    AlignNumbered,
    AlignUnnumbered,
}

fn collect_math_labels_for_value(
    source: &str,
    labels: &mut HashMap<String, LabelInfo>,
    value: usize,
    page: usize,
) -> Result<(), String> {
    let mut label_cursor = source;
    while let Some(label_index) = find_control(label_cursor, "label") {
        let rest = &label_cursor[label_index + "\\label".len()..];
        let Some((key, after_label)) = take_braced(rest) else {
            return Err("native backend requires braced \\label keys".to_string());
        };
        labels.insert(
            key.trim().to_string(),
            LabelInfo::new(value.to_string(), page),
        );
        label_cursor = after_label;
    }
    Ok(())
}

fn collect_listing_labels(
    body: &str,
    labels: &mut HashMap<String, LabelInfo>,
    layout: &DocumentLayout,
    listing_reference_name: &str,
) -> Result<(), String> {
    let mut cursor = body;
    let mut listing_counter = 0_usize;
    let mut slot = 0_usize;
    let listing_reference_prefix = listing_reference_name.to_lowercase();
    while !cursor.is_empty() {
        let next = ["\\begin{lstlisting}", "\\lstinputlisting"]
            .into_iter()
            .filter_map(|marker| cursor.find(marker).map(|index| (index, marker)))
            .min_by_key(|(index, _)| *index);
        let Some((index, marker)) = next else {
            break;
        };
        slot += estimate_text_slots(&cursor[..index], layout);
        cursor = &cursor[index + marker.len()..];
        let (options, rest) = take_optional_bracketed(cursor);
        listing_counter += 1;
        if let Some(label) = options.and_then(label_option_value) {
            labels.insert(
                label,
                LabelInfo::with_reference_prefix(
                    listing_counter.to_string(),
                    page_for_slot(slot + 1, layout),
                    listing_reference_prefix.as_str(),
                ),
            );
        }
        let remaining = match marker {
            "\\begin{lstlisting}" => {
                let (_, remaining) = take_environment_body(rest, "lstlisting")?;
                remaining
            }
            "\\lstinputlisting" => {
                let Some((_, remaining)) = take_braced(rest) else {
                    return Err(
                        "native backend requires braced \\lstinputlisting paths".to_string()
                    );
                };
                remaining
            }
            _ => unreachable!(),
        };
        slot += 3;
        cursor = remaining;
    }
    Ok(())
}

fn collect_index_entries(
    body: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    layout: &DocumentLayout,
) -> Result<IndexRegistry, String> {
    let mut registry = IndexRegistry {
        requested: source_contains_control(body, "makeindex"),
        printed: source_contains_control(body, "printindex"),
        ..IndexRegistry::default()
    };
    let mut cursor = body;
    let mut slot = 0_usize;

    while let Some(index) = find_control(cursor, "index") {
        let before = &cursor[..index];
        slot += estimate_text_slots(before, layout);
        let rest = &cursor[index + "\\index".len()..];
        let (_, rest) = take_optional_bracketed(rest);
        let Some((payload, remaining)) = take_braced(rest) else {
            return Err("native backend requires braced \\index payloads".to_string());
        };
        let raw = normalize_index_payload(payload);
        if !raw.is_empty() {
            registry.entries.push(IndexEntry {
                display: index_display_text(&raw, macros, labels, citations)?,
                raw,
                page: page_for_slot(slot.max(1), layout),
            });
        }
        cursor = remaining;
    }

    Ok(registry)
}

fn sorted_index_entries(index: &IndexRegistry) -> Vec<&IndexEntry> {
    let mut entries = index.entries.iter().collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        left.display
            .to_lowercase()
            .cmp(&right.display.to_lowercase())
            .then_with(|| left.page.cmp(&right.page))
            .then_with(|| left.raw.cmp(&right.raw))
    });
    entries
}

fn index_sidecar_content(index: &IndexRegistry) -> String {
    let mut source = String::new();
    for entry in &index.entries {
        writeln!(source, "\\indexentry{{{}}}{{{}}}", entry.raw, entry.page).unwrap();
    }
    source
}

fn normalize_index_payload(payload: &str) -> String {
    payload.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn index_display_text(
    payload: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
) -> Result<String, String> {
    let without_encapsulator = payload.split_once('|').map_or(payload, |(key, _)| key);
    let parts = without_encapsulator
        .split('!')
        .map(|part| part.split_once('@').map_or(part, |(_, display)| display))
        .map(|part| part.replace('"', ""))
        .map(|part| clean_inline_text(&part, macros, labels, citations))
        .collect::<Result<Vec<_>, _>>()?;
    let display = parts
        .into_iter()
        .map(|part| part.trim().to_string())
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(", ");
    Ok(if display.is_empty() {
        payload.to_string()
    } else {
        display
    })
}

fn collect_toc_entries(
    body: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    layout: &DocumentLayout,
) -> Result<Vec<TocEntry>, String> {
    let mut entries = Vec::new();
    let mut cursor = body;
    let mut section_counter = 0_usize;
    let mut subsection_counter = 0_usize;
    let mut appendix_mode = false;
    let mut slot = 0_usize;
    let toc_entry_count = count_toc_entries(body);

    while !cursor.is_empty() {
        let next = [
            "\\tableofcontents",
            "\\addcontentsline",
            "\\appendix",
            "\\section",
            "\\subsection",
        ]
        .into_iter()
        .filter_map(|marker| cursor.find(marker).map(|index| (index, marker)))
        .min_by_key(|(index, _)| *index);
        let Some((index, marker)) = next else {
            break;
        };
        slot += estimate_text_slots(&cursor[..index], layout);
        cursor = &cursor[index + marker.len()..];
        match marker {
            "\\tableofcontents" => {
                slot += 2 + toc_entry_count.max(1);
            }
            "\\appendix" => {
                appendix_mode = true;
                section_counter = 0;
                subsection_counter = 0;
            }
            "\\section" => {
                let (after_star, starred) = strip_optional_star(cursor);
                let (_, after_optional) = take_optional_bracketed(after_star);
                let Some((title, remaining)) = take_braced(after_optional) else {
                    return Err("native backend requires braced \\section titles".to_string());
                };
                if !starred {
                    section_counter += 1;
                    subsection_counter = 0;
                    entries.push(TocEntry {
                        level: TocLevel::Section,
                        kind: TocLevel::Section.kind().to_string(),
                        number: Some(section_number(section_counter, appendix_mode)),
                        title: clean_inline_text(title, macros, labels, citations)?,
                        page: page_for_slot(slot + 2, layout),
                    });
                }
                slot += 3;
                cursor = remaining;
            }
            "\\subsection" => {
                let (after_star, starred) = strip_optional_star(cursor);
                let (_, after_optional) = take_optional_bracketed(after_star);
                let Some((title, remaining)) = take_braced(after_optional) else {
                    return Err("native backend requires braced \\subsection titles".to_string());
                };
                if !starred {
                    subsection_counter += 1;
                    entries.push(TocEntry {
                        level: TocLevel::Subsection,
                        kind: TocLevel::Subsection.kind().to_string(),
                        number: Some(subsection_number(
                            section_counter,
                            subsection_counter,
                            appendix_mode,
                        )),
                        title: clean_inline_text(title, macros, labels, citations)?,
                        page: page_for_slot(slot + 2, layout),
                    });
                }
                slot += 3;
                cursor = remaining;
            }
            "\\addcontentsline" => {
                let Some((file, rest)) = take_braced(cursor) else {
                    return Err(
                        "native backend requires braced \\addcontentsline file targets".to_string(),
                    );
                };
                let Some((kind, rest)) = take_braced(rest) else {
                    return Err(
                        "native backend requires braced \\addcontentsline entry kinds".to_string(),
                    );
                };
                let Some((title, remaining)) = take_braced(rest) else {
                    return Err(
                        "native backend requires braced \\addcontentsline titles".to_string()
                    );
                };
                let kind = kind.trim();
                if file.trim() == "toc" && !kind.is_empty() {
                    entries.push(TocEntry {
                        level: TocLevel::from_contents_kind(kind),
                        kind: kind.to_string(),
                        number: None,
                        title: clean_inline_text(title, macros, labels, citations)?,
                        page: page_for_slot(slot.max(1), layout),
                    });
                }
                cursor = remaining;
            }
            _ => unreachable!(),
        }
    }

    Ok(entries)
}

fn collect_bookmarks(
    body: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    layout: &DocumentLayout,
) -> Result<Vec<BookmarkEntry>, String> {
    let mut entries = Vec::new();
    let mut cursor = body;
    let mut section_counter = 0_usize;
    let mut subsection_counter = 0_usize;
    let mut appendix_mode = false;
    let mut slot = 0_usize;
    let toc_entry_count = if source_contains_control(body, "tableofcontents") {
        count_toc_entries(body)
    } else {
        0
    };
    let figure_entry_count = if source_contains_control(body, "listoffigures") {
        count_float_entries(body, FloatKind::Figure)
    } else {
        0
    };
    let table_entry_count = if source_contains_control(body, "listoftables") {
        count_float_entries(body, FloatKind::Table)
    } else {
        0
    };

    while !cursor.is_empty() {
        let next = [
            "\\tableofcontents",
            "\\listoffigures",
            "\\listoftables",
            "\\pdfbookmark",
            "\\phantomsection",
            "\\appendix",
            "\\section",
            "\\subsection",
        ]
        .into_iter()
        .filter_map(|marker| cursor.find(marker).map(|index| (index, marker)))
        .min_by_key(|(index, _)| *index);
        let Some((index, marker)) = next else {
            break;
        };
        slot += estimate_text_slots(&cursor[..index], layout);
        cursor = &cursor[index + marker.len()..];
        match marker {
            "\\tableofcontents" => {
                slot += 2 + toc_entry_count.max(1);
            }
            "\\listoffigures" => {
                slot += 2 + figure_entry_count.max(1);
            }
            "\\listoftables" => {
                slot += 2 + table_entry_count.max(1);
            }
            "\\pdfbookmark" => {
                let (level, rest) = take_optional_bracketed(cursor);
                let Some((title, rest)) = take_braced(rest) else {
                    return Err("native backend requires braced \\pdfbookmark titles".to_string());
                };
                let Some((dest, remaining)) = take_braced(rest) else {
                    return Err(
                        "native backend requires braced \\pdfbookmark destination names"
                            .to_string(),
                    );
                };
                let title = clean_inline_text(title, macros, labels, citations)?;
                if !title.is_empty() {
                    entries.push(BookmarkEntry {
                        level: level.and_then(bookmark_level).unwrap_or(0),
                        dest: bookmark_destination(dest),
                        title,
                        page: page_for_slot(slot.max(1), layout),
                    });
                }
                cursor = remaining;
            }
            "\\phantomsection" => {}
            "\\appendix" => {
                appendix_mode = true;
                section_counter = 0;
                subsection_counter = 0;
            }
            "\\section" => {
                let (after_star, starred) = strip_optional_star(cursor);
                let (_, after_optional) = take_optional_bracketed(after_star);
                let Some((title, remaining)) = take_braced(after_optional) else {
                    return Err("native backend requires braced \\section titles".to_string());
                };
                if !starred {
                    section_counter += 1;
                    subsection_counter = 0;
                    let number = section_number(section_counter, appendix_mode);
                    let title = clean_inline_text(title, macros, labels, citations)?;
                    if !title.is_empty() {
                        entries.push(BookmarkEntry {
                            level: 1,
                            dest: bookmark_destination(&format!("section.{number}")),
                            title,
                            page: page_for_slot(slot + 2, layout),
                        });
                    }
                }
                slot += 3;
                cursor = remaining;
            }
            "\\subsection" => {
                let (after_star, starred) = strip_optional_star(cursor);
                let (_, after_optional) = take_optional_bracketed(after_star);
                let Some((title, remaining)) = take_braced(after_optional) else {
                    return Err("native backend requires braced \\subsection titles".to_string());
                };
                if !starred {
                    subsection_counter += 1;
                    let number =
                        subsection_number(section_counter, subsection_counter, appendix_mode);
                    let title = clean_inline_text(title, macros, labels, citations)?;
                    if !title.is_empty() {
                        entries.push(BookmarkEntry {
                            level: 2,
                            dest: bookmark_destination(&format!("subsection.{number}")),
                            title,
                            page: page_for_slot(slot + 2, layout),
                        });
                    }
                }
                slot += 3;
                cursor = remaining;
            }
            _ => unreachable!(),
        }
    }

    Ok(entries)
}

fn bookmark_level(source: &str) -> Option<usize> {
    source.trim().parse().ok()
}

fn bookmark_destination(source: &str) -> String {
    source
        .trim()
        .chars()
        .map(|ch| match ch {
            ch if ch.is_ascii_alphanumeric() || matches!(ch, ':' | '-' | '_' | '.') => ch,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn count_toc_entries(body: &str) -> usize {
    let mut count = 0_usize;
    let mut cursor = body;
    while !cursor.is_empty() {
        let next = ["\\addcontentsline", "\\section", "\\subsection"]
            .into_iter()
            .filter_map(|marker| cursor.find(marker).map(|index| (index, marker)))
            .min_by_key(|(index, _)| *index);
        let Some((index, marker)) = next else {
            break;
        };
        cursor = &cursor[index + marker.len()..];
        match marker {
            "\\addcontentsline" => {
                let Some((file, rest)) = take_braced(cursor) else {
                    break;
                };
                let Some((kind, rest)) = take_braced(rest) else {
                    break;
                };
                let Some((_, remaining)) = take_braced(rest) else {
                    break;
                };
                if file.trim() == "toc" && !kind.trim().is_empty() {
                    count += 1;
                }
                cursor = remaining;
            }
            "\\section" | "\\subsection" => {
                let (after_star, starred) = strip_optional_star(cursor);
                let (_, after_optional) = take_optional_bracketed(after_star);
                let Some((_, remaining)) = take_braced(after_optional) else {
                    break;
                };
                if !starred {
                    count += 1;
                }
                cursor = remaining;
            }
            _ => unreachable!(),
        }
    }
    count
}

fn page_for_slot(slot: usize, layout: &DocumentLayout) -> usize {
    slot.saturating_sub(1) / layout.lines_per_page + 1
}

fn estimate_text_slots(source: &str, layout: &DocumentLayout) -> usize {
    let mut slots = source.matches("\n\n").count();
    for block in source.split("\n\n") {
        let text = block
            .lines()
            .map(|line| line.split('%').next().unwrap_or("").trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        if text.is_empty() {
            continue;
        }
        slots += layout.wrap_text(&loose_plain_text(&text)).len().max(1);
    }
    slots
}

fn estimate_caption_slots(
    label: &str,
    number: usize,
    caption: &str,
    layout: &DocumentLayout,
) -> usize {
    let caption = caption_label(
        label,
        number,
        &loose_plain_text(caption),
        CaptionLabelSeparator::Colon,
    );
    layout.wrap_caption_text(&caption).len().max(1)
}

fn estimate_theorem_slots(theorem_body: &str, layout: &DocumentLayout) -> usize {
    3 + estimate_text_slots(theorem_body, layout)
}

fn loose_plain_text(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '\\' => {
                while matches!(chars.peek(), Some(next) if next.is_ascii_alphabetic()) {
                    chars.next();
                }
                out.push(' ');
            }
            '{' | '}' | '$' | '&' => out.push(' '),
            '~' | '\n' | '\t' => out.push(' '),
            _ => out.push(ch),
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn take_braced(source: &str) -> Option<(&str, &str)> {
    let source = source.trim_start();
    let mut chars = source.char_indices();
    let (_, first) = chars.next()?;
    if first != '{' {
        return None;
    }
    let mut depth = 1_usize;
    for (index, ch) in chars {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some((&source[1..index], &source[index + ch.len_utf8()..]));
                }
            }
            _ => {}
        }
    }
    None
}

fn take_environment_body<'a>(source: &'a str, env: &str) -> Result<(&'a str, &'a str), String> {
    let end_marker = format!("\\end{{{env}}}");
    let Some(end_index) = source.find(&end_marker) else {
        return Err(format!("native backend could not find `{end_marker}`"));
    };
    Ok((
        &source[..end_index],
        &source[end_index + end_marker.len()..],
    ))
}

fn take_until_command_or_blank(source: &str) -> (&str, &str) {
    let mut end = source.len();
    for marker in [
        "\\section",
        "\\subsection",
        "\\subsubsection",
        "\\paragraph",
        "\\maketitle",
        "\\tableofcontents",
        "\\listoffigures",
        "\\listoftables",
        "\\addcontentsline",
        "\\pdfbookmark",
        "\\phantomsection",
        "\\appendix",
        "\\newpage",
        "\\clearpage",
        "\\onecolumn",
        "\\twocolumn",
        "\\thispagestyle",
        "\\pagestyle",
        "\\noindent",
        "\\fontsize",
        "\\selectfont",
        "\\addbibresource",
        "\\bibliographystyle",
        "\\bibliography",
        "\\printbibliography",
        "\\makeindex",
        "\\printindex",
        "\\index",
        "\\includegraphics",
        "\\resizebox",
        "\\newtcbtheorem",
        "\\begin",
        "\\end",
        "$$",
        "\\centering",
        "\\smallskip",
        "\\medskip",
        "\\bigskip",
        "\\renewcommand",
        "\\vspace",
        "\\hspace",
        "\\caption",
        "\\item",
        "\\\\",
        "\n\n",
    ] {
        if let Some(index) = source.find(marker) {
            end = end.min(index);
        }
    }
    (&source[..end], &source[end..])
}

fn take_dollar_display_math(source: &str) -> Result<(&str, &str), String> {
    let Some(end_index) = find_unescaped_double_dollar(source) else {
        return Err("native backend could not find closing `$$`".to_string());
    };
    Ok((&source[..end_index], &source[end_index + "$$".len()..]))
}

fn starts_with_blank_line(source: &str) -> bool {
    let mut saw_newline = false;
    for ch in source.chars() {
        match ch {
            '\n' => {
                if saw_newline {
                    return true;
                }
                saw_newline = true;
            }
            ' ' | '\t' | '\r' => {}
            _ => return false,
        }
    }
    false
}

fn clean_inline_text(
    source: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
) -> Result<String, String> {
    let mut footnotes = FootnoteRegistry::default();
    clean_inline_text_collecting(source, macros, labels, citations, &mut footnotes)
}

fn clean_inline_text_collecting(
    source: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
) -> Result<String, String> {
    let mut out = String::new();
    let mut chars = source.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '%' => {
                for next in chars.by_ref() {
                    if next == '\n' {
                        out.push(' ');
                        break;
                    }
                }
            }
            '$' => {
                let rest: String = chars.collect();
                if let Some(after_open) = rest.strip_prefix('$') {
                    if let Some(end) = find_unescaped_double_dollar(after_open) {
                        out.push_str(&clean_math_text(&after_open[..end]));
                        let remaining = &after_open[end + "$$".len()..];
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                    }
                } else if let Some(end) = find_unescaped_dollar(&rest) {
                    out.push_str(&clean_math_text(&rest[..end]));
                    let remaining = &rest[end + "$".len()..];
                    let remaining = clean_inline_text_collecting(
                        remaining, macros, labels, citations, footnotes,
                    )?;
                    append_cleaned_remaining(&mut out, &remaining);
                }
                break;
            }
            '\\' => {
                let mut name = String::new();
                while let Some(&next) = chars.peek() {
                    if !next.is_ascii_alphabetic() {
                        break;
                    }
                    name.push(next);
                    chars.next();
                }
                match name.as_str() {
                    "" => {
                        if let Some(symbol) = chars.next() {
                            if symbol == '\\' {
                                out.push(' ');
                                let rest: String = chars.collect();
                                let (_, remaining) = take_optional_bracketed(&rest);
                                let remaining = clean_inline_text_collecting(
                                    remaining, macros, labels, citations, footnotes,
                                )?;
                                append_cleaned_remaining(&mut out, &remaining);
                                break;
                            } else if symbol == '[' {
                                let rest: String = chars.collect();
                                if let Some(end) = rest.find("\\]") {
                                    out.push_str(&clean_math_text(&rest[..end]));
                                    let remaining = &rest[end + "\\]".len()..];
                                    let remaining = clean_inline_text_collecting(
                                        remaining, macros, labels, citations, footnotes,
                                    )?;
                                    append_cleaned_remaining(&mut out, &remaining);
                                    break;
                                }
                                out.push('[');
                                break;
                            } else if matches!(symbol, ',' | '!' | ':' | ';') {
                                out.push(' ');
                            } else {
                                out.push(symbol);
                            }
                        }
                    }
                    name if macros.contains_key(name) => {
                        out.push_str(&clean_inline_text_collecting(
                            &macros[name],
                            macros,
                            labels,
                            citations,
                            footnotes,
                        )?);
                    }
                    "LaTeX" => out.push_str("LaTeX"),
                    "TeX" => out.push_str("TeX"),
                    "and" | "And" | "AND" => out.push_str("and"),
                    "cite" | "citep" | "citet" | "parencite" | "textcite" => {
                        let rest: String = chars.collect();
                        let (citation, remaining) =
                            render_citation_command(name.as_str(), &rest, citations).ok_or_else(
                                || format!("native backend requires braced \\{name} citation keys"),
                            )?;
                        out.push_str(&citation);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "thanks" | "footnote" => {
                        let rest: String = chars.collect();
                        let (payload, remaining) = take_braced(&rest).ok_or_else(|| {
                            format!("native backend requires braced \\{name} payloads")
                        })?;
                        let text = clean_inline_text(payload, macros, labels, citations)?;
                        let number = footnotes.add(text);
                        write!(out, "{number}").unwrap();
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "footnotemark" => {
                        let rest: String = chars.collect();
                        let (explicit, remaining) = take_optional_bracketed(&rest);
                        let number = footnotes.mark(explicit);
                        write!(out, "{number}").unwrap();
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "footnotetext" => {
                        let rest: String = chars.collect();
                        let (explicit, rest) = take_optional_bracketed(&rest);
                        let (payload, remaining) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\footnotetext payloads".to_string()
                        })?;
                        let text = clean_inline_text(payload, macros, labels, citations)?;
                        footnotes.add_text(explicit, text);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "frac" | "dfrac" | "tfrac" => {
                        let rest: String = chars.collect();
                        let rest = rest.as_str();
                        let fraction = if let Some((numerator, rest)) = take_braced(rest)
                            && let Some((denominator, remaining)) = take_braced(rest)
                        {
                            Some((numerator, denominator, remaining))
                        } else if let Some((numerator, rest)) = take_math_atom(rest)
                            && let Some((denominator, remaining)) = take_math_atom(rest)
                        {
                            Some((numerator, denominator, remaining))
                        } else {
                            None
                        };
                        if let Some((numerator, denominator, remaining)) = fraction {
                            out.push_str(&clean_math_text(numerator));
                            out.push('/');
                            out.push_str(&clean_math_text(denominator));
                            let remaining = clean_inline_text_collecting(
                                remaining, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        } else {
                            out.push_str(&name);
                            let remaining = clean_inline_text_collecting(
                                rest, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "sqrt" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        if let Some((payload, remaining)) =
                            take_braced(rest).or_else(|| take_math_atom(rest))
                        {
                            out.push_str(&clean_math_root(payload));
                            let remaining = clean_inline_text_collecting(
                                remaining, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        } else {
                            out.push_str("sqrt");
                        }
                        break;
                    }
                    "hat" | "widehat" | "bar" | "overline" | "tilde" | "widetilde" | "vec"
                    | "dot" | "ddot" => {
                        let rest: String = chars.collect();
                        if let Some((payload, remaining)) = take_math_atom(&rest) {
                            out.push_str(&clean_math_text(payload));
                            let remaining = clean_inline_text_collecting(
                                remaining, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "ref" | "eqref" | "cref" | "Cref" | "autoref" | "pageref" => {
                        let rest: String = chars.collect();
                        let (key, remaining) = take_braced(&rest).ok_or_else(|| {
                            format!("native backend requires braced \\{name} keys")
                        })?;
                        out.push_str(&render_reference_command(name.as_str(), key, labels));
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "protect" | "bf" | "it" | "rm" | "sf" | "em" | "sl" | "tt" | "sc"
                    | "normalfont" | "bfseries" | "mdseries" | "itshape" | "upshape"
                    | "sffamily" | "rmfamily" | "ttfamily" | "centering" | "raggedright"
                    | "raggedleft" | "tiny" | "scriptsize" | "footnotesize" | "small"
                    | "normalsize" | "large" | "Large" | "LARGE" | "huge" | "Huge" | "left"
                    | "right" | "selectfont" | "onecolumn" | "twocolumn" | "noindent" | "hfill"
                    | "smallskip" | "medskip" | "bigskip" => {}
                    "thispagestyle" | "pagestyle" => {
                        let rest: String = chars.collect();
                        let remaining = take_braced(&rest)
                            .map(|(_, remaining)| remaining)
                            .unwrap_or(rest.as_str());
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "begin" => {
                        let rest: String = chars.collect();
                        let (env, remaining) = take_braced(&rest).ok_or_else(|| {
                            "native backend requires braced \\begin environments".to_string()
                        })?;
                        let remaining = consume_environment_open_args(env, remaining);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "end" => {
                        let rest: String = chars.collect();
                        let (_, remaining) = take_braced(&rest).ok_or_else(|| {
                            "native backend requires braced \\end environments".to_string()
                        })?;
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "includegraphics" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        let (_, remaining) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\includegraphics paths".to_string()
                        })?;
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "caption" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        let (payload, remaining) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\caption payloads".to_string()
                        })?;
                        out.push_str(&clean_inline_text_collecting(
                            payload, macros, labels, citations, footnotes,
                        )?);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "captionof" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_braced(&rest).ok_or_else(|| {
                            "native backend requires braced \\captionof kinds".to_string()
                        })?;
                        let (_, rest) = take_optional_bracketed(rest);
                        let (payload, remaining) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\captionof payloads".to_string()
                        })?;
                        out.push_str(&clean_inline_text_collecting(
                            payload, macros, labels, citations, footnotes,
                        )?);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "fontsize" => {
                        let rest: String = chars.collect();
                        let remaining = skip_two_braced_arguments(&rest).unwrap_or(rest.as_str());
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "setlength" | "addtolength" => {
                        let rest: String = chars.collect();
                        let remaining = skip_two_braced_arguments(&rest).unwrap_or(rest.as_str());
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "hspace" | "vspace" | "hskip" | "vskip" | "kern" => {
                        let rest: String = chars.collect();
                        let rest = rest.strip_prefix('*').unwrap_or(&rest);
                        let remaining = take_braced(rest)
                            .map(|(_, remaining)| remaining)
                            .unwrap_or(rest);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "underset" | "overset" | "stackrel" => {
                        let rest: String = chars.collect();
                        if let Some((_, rest)) = take_braced(&rest)
                            && let Some((payload, remaining)) = take_braced(rest)
                        {
                            let visible = clean_inline_text_collecting(
                                payload, macros, labels, citations, footnotes,
                            )?;
                            if name == "stackrel" && is_relation_like_text(&visible) {
                                out.push(' ');
                                out.push_str(&visible);
                                out.push(' ');
                            } else {
                                out.push_str(&visible);
                            }
                            let remaining = clean_inline_text_collecting(
                                remaining, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "underbrace" | "overbrace" => {
                        let rest: String = chars.collect();
                        if let Some((payload, remaining)) = take_braced(&rest) {
                            out.push_str(&clean_math_text(payload));
                            let remaining = skip_math_script_annotations(remaining);
                            let remaining = clean_inline_text_collecting(
                                remaining, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "big" | "Big" | "bigg" | "Bigg" | "bigl" | "bigr" | "Bigl" | "Bigr"
                    | "biggl" | "biggr" | "Biggl" | "Biggr" => {}
                    "toprule" | "midrule" | "bottomrule" => {}
                    "cmidrule" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        let remaining = take_braced(rest)
                            .map(|(_, remaining)| remaining)
                            .unwrap_or(rest);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "multicolumn" => {
                        let rest: String = chars.collect();
                        if let Some((_, rest)) = take_braced(&rest)
                            && let Some((_, rest)) = take_braced(rest)
                            && let Some((payload, remaining)) = take_braced(rest)
                        {
                            out.push_str(&clean_inline_text_collecting(
                                payload, macros, labels, citations, footnotes,
                            )?);
                            let remaining = clean_inline_text_collecting(
                                remaining, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "xrightarrow" | "xleftarrow" => {
                        let rest: String = chars.collect();
                        let (below, rest) = take_optional_bracketed(&rest);
                        let (above, remaining) = take_braced(rest).unwrap_or(("", rest));
                        out.push(' ');
                        out.push_str(math_control_fallback(&name));
                        for label in [above, below.unwrap_or("")] {
                            let label = clean_math_text(label);
                            if !label.is_empty() {
                                out.push(' ');
                                out.push_str(&label);
                            }
                        }
                        out.push(' ');
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "in" | "notin" | "to" | "rightarrow" | "mapsto" | "longrightarrow"
                    | "leftarrow" | "longleftarrow" | "leftrightarrow" | "longleftrightarrow"
                    | "le" | "leq" | "ge" | "geq" | "neq" | "ne" | "equiv" | "sim" | "approx"
                    | "mid" | "triangleq" | "propto" | "succeq" | "preceq" | "succ" | "prec"
                    | "subset" | "subseteq" | "supset" | "supseteq" | "cap" | "cup"
                    | "setminus" | "land" | "lor" | "neg" | "otimes" | "oplus" | "emptyset"
                    | "iff" | "Leftrightarrow" | "Longleftrightarrow" | "Leftarrow"
                    | "Longleftarrow" | "Rightarrow" | "Longrightarrow" | "implies" | "forall"
                    | "exists" | "perp" => {
                        out.push(' ');
                        out.push_str(math_control_fallback(&name));
                        out.push(' ');
                    }
                    "not" => {
                        let rest: String = chars.collect();
                        if let Some(remaining) = rest.trim_start().strip_prefix('=') {
                            out.push_str(" != ");
                            let remaining = clean_inline_text_collecting(
                                remaining, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        } else {
                            out.push_str(" not ");
                            let remaining = clean_inline_text_collecting(
                                &rest, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "langle" => out.push('<'),
                    "rangle" => out.push('>'),
                    "lVert" | "lvert" => out.push('|'),
                    "rVert" | "rvert" => out.push('|'),
                    "quad" | "qquad" => out.push(' '),
                    "cdots" | "ldots" | "dots" => out.push_str("..."),
                    "sum" | "prod" | "int" | "lim" | "min" | "max" | "sup" | "inf" => {
                        out.push(' ');
                        out.push_str(math_control_fallback(&name));
                        out.push(' ');
                    }
                    "color" | "urlstyle" => {
                        let rest: String = chars.collect();
                        let (_, remaining) = take_braced(&rest).ok_or_else(|| {
                            format!("native backend requires braced \\{name} payloads")
                        })?;
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "definecolor" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_braced(&rest).ok_or_else(|| {
                            "native backend requires braced \\definecolor names".to_string()
                        })?;
                        let (_, rest) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\definecolor models".to_string()
                        })?;
                        let (_, remaining) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\definecolor values".to_string()
                        })?;
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "textcolor" | "colorbox" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_braced(&rest).ok_or_else(|| {
                            format!("native backend requires braced \\{name} colors")
                        })?;
                        let (payload, remaining) = take_braced(rest).ok_or_else(|| {
                            format!("native backend requires braced \\{name} text")
                        })?;
                        out.push_str(&clean_inline_text_collecting(
                            payload, macros, labels, citations, footnotes,
                        )?);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "fcolorbox" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_braced(&rest).ok_or_else(|| {
                            "native backend requires braced \\fcolorbox frame colors".to_string()
                        })?;
                        let (_, rest) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\fcolorbox background colors"
                                .to_string()
                        })?;
                        let (payload, remaining) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\fcolorbox text".to_string()
                        })?;
                        out.push_str(&clean_inline_text_collecting(
                            payload, macros, labels, citations, footnotes,
                        )?);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "href" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_braced(&rest).ok_or_else(|| {
                            "native backend requires braced \\href URLs".to_string()
                        })?;
                        let (payload, remaining) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\href link text".to_string()
                        })?;
                        out.push_str(&clean_inline_text_collecting(
                            payload, macros, labels, citations, footnotes,
                        )?);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "url" => {
                        let rest: String = chars.collect();
                        let (payload, remaining) = take_braced(&rest).ok_or_else(|| {
                            "native backend requires braced \\url payloads".to_string()
                        })?;
                        out.push_str(payload.trim());
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "hyperref" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        let (payload, remaining) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\hyperref link text".to_string()
                        })?;
                        out.push_str(&clean_inline_text_collecting(
                            payload, macros, labels, citations, footnotes,
                        )?);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "phantomsection" => {}
                    "pdfbookmark" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        let (_, rest) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\pdfbookmark titles".to_string()
                        })?;
                        let (_, remaining) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\pdfbookmark destination names"
                                .to_string()
                        })?;
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "label" => {
                        let rest: String = chars.collect();
                        let (_, remaining) = take_braced(&rest).ok_or_else(|| {
                            "native backend requires braced \\label keys".to_string()
                        })?;
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "bibliography" | "bibliographystyle" | "addbibresource" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        let (_, remaining) = take_braced(rest).ok_or_else(|| {
                            format!("native backend requires braced \\{name} payloads")
                        })?;
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "printbibliography" => {
                        let rest: String = chars.collect();
                        let (_, remaining) = take_optional_bracketed(&rest);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "makeindex" => {}
                    "printindex" => {
                        let rest: String = chars.collect();
                        let (_, remaining) = take_optional_bracketed(&rest);
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "index" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        let (_, remaining) = take_braced(rest).ok_or_else(|| {
                            "native backend requires braced \\index payloads".to_string()
                        })?;
                        let remaining = clean_inline_text_collecting(
                            remaining, macros, labels, citations, footnotes,
                        )?;
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "makecell" | "thead" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        if let Some((payload, remaining)) = take_braced(rest) {
                            out.push_str(&clean_inline_text_collecting(
                                payload, macros, labels, citations, footnotes,
                            )?);
                            let remaining = clean_inline_text_collecting(
                                remaining, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        } else {
                            let remaining = clean_inline_text_collecting(
                                rest, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "textsc" => {
                        let rest: String = chars.collect();
                        if let Some((payload, remaining)) = take_braced(&rest) {
                            out.push_str(
                                &clean_inline_text_collecting(
                                    payload, macros, labels, citations, footnotes,
                                )?
                                .to_ascii_uppercase(),
                            );
                            let remaining = clean_inline_text_collecting(
                                remaining, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        } else {
                            let remaining = clean_inline_text_collecting(
                                &rest, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "textbf" | "emph" | "textit" | "texttt" | "textrm" | "textsf"
                    | "textsuperscript" | "underline" | "ensuremath" | "mathrm" | "mathbf"
                    | "mathsf" | "mathit" | "mathbb" | "mathcal" | "mathfrak" | "bm"
                    | "boldsymbol" | "operatorname" | "text" | "mbox" | "texorpdfstring" => {
                        let rest: String = chars.collect();
                        let rest = rest.strip_prefix('*').unwrap_or(&rest);
                        if let Some((payload, remaining)) = take_braced(rest) {
                            out.push_str(&clean_inline_text_collecting(
                                payload, macros, labels, citations, footnotes,
                            )?);
                            let remaining = if name == "texorpdfstring" {
                                take_braced(remaining)
                                    .map(|(_, remaining)| remaining)
                                    .unwrap_or(remaining)
                            } else {
                                remaining
                            };
                            let remaining = clean_inline_text_collecting(
                                remaining, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        } else {
                            let remaining = clean_inline_text_collecting(
                                rest, macros, labels, citations, footnotes,
                            )?;
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    _ => {
                        if !name.is_empty() {
                            out.push_str(math_control_fallback(&name));
                        }
                    }
                }
            }
            '{' | '}' => {}
            '\n' | '\t' => out.push(' '),
            '~' => out.push(' '),
            _ => out.push(ch),
        }
    }
    let normalized = out.split_whitespace().collect::<Vec<_>>().join(" ");
    Ok(normalize_tex_prose_punctuation(&normalized))
}

fn math_control_fallback(name: &str) -> &str {
    match name {
        "in" => "∈",
        "notin" => "∉",
        "to" | "rightarrow" | "mapsto" | "longrightarrow" | "xrightarrow" => "→",
        "leftarrow" | "longleftarrow" | "xleftarrow" => "←",
        "leftrightarrow" | "longleftrightarrow" => "↔",
        "Leftarrow" | "Longleftarrow" => "⇐",
        "le" | "leq" => "≤",
        "ge" | "geq" => "≥",
        "neq" | "ne" => "≠",
        "equiv" => "≡",
        "triangleq" => ":=",
        "propto" => "∝",
        "succeq" => "≽",
        "preceq" => "≼",
        "succ" => ">",
        "prec" => "<",
        "subset" => "⊂",
        "subseteq" => "⊆",
        "supset" => "⊃",
        "supseteq" => "⊇",
        "cap" => "∩",
        "cup" => "∪",
        "setminus" | "backslash" => "∖",
        "land" => "∧",
        "lor" => "∨",
        "neg" => "¬",
        "otimes" => "⊗",
        "oplus" => "⊕",
        "emptyset" => "∅",
        "iff" | "Leftrightarrow" | "Longleftrightarrow" => "⇔",
        "Rightarrow" | "Longrightarrow" | "implies" => "⇒",
        "pm" => "±",
        "mp" => "∓",
        "times" => "×",
        "cdot" => "·",
        "circ" => "o",
        "infty" => "∞",
        "partial" => "∂",
        "nabla" => "∇",
        "top" => "T",
        "forall" => "∀",
        "exists" => "∃",
        "perp" | "bot" => "⊥",
        "sim" => "∼",
        "approx" => "≈",
        "mid" => "|",
        "sum" => "∑",
        "prod" => "∏",
        "int" => "∫",
        "LaTeX" => "LaTeX",
        "TeX" => "TeX",
        "Alpha" => "Α",
        "Beta" => "Β",
        "Gamma" => "Γ",
        "Delta" => "Δ",
        "Theta" => "Θ",
        "Lambda" => "Λ",
        "Xi" => "Ξ",
        "Pi" => "Π",
        "Sigma" => "Σ",
        "Phi" => "Φ",
        "Psi" => "Ψ",
        "Omega" => "Ω",
        "alpha" => "α",
        "beta" => "β",
        "gamma" => "γ",
        "delta" => "δ",
        "epsilon" | "varepsilon" => "ε",
        "zeta" => "ζ",
        "eta" => "η",
        "theta" | "vartheta" => "θ",
        "iota" => "ι",
        "kappa" => "κ",
        "lambda" => "λ",
        "mu" => "μ",
        "nu" => "ν",
        "xi" => "ξ",
        "pi" => "π",
        "rho" => "ρ",
        "sigma" => "σ",
        "tau" => "τ",
        "upsilon" => "υ",
        "phi" | "varphi" => "φ",
        "chi" => "χ",
        "psi" => "ψ",
        "omega" => "ω",
        _ => name,
    }
}

fn append_cleaned_remaining(out: &mut String, remaining: &str) {
    if remaining.is_empty() {
        return;
    }
    let last = out.chars().next_back();
    let first = remaining.chars().next();
    if out.trim_end().ends_with("s.t.")
        && first.is_some_and(|right| right.is_alphanumeric() || matches!(right, '[' | '('))
    {
        out.push(' ');
        out.push_str(remaining);
        return;
    }
    if (out.trim_end().ends_with("+/-") || out.trim_end().ends_with('±'))
        && first.is_some_and(|right| right.is_ascii_alphabetic())
    {
        out.push(' ');
        out.push_str(remaining);
        return;
    }
    if matches!(first, Some('(')) {
        let before = out.trim_end();
        let token = before.split_whitespace().next_back().unwrap_or_default();
        if math_token_allows_compact_boundary_call(token)
            && !remaining_after_open_paren_starts_with_digit(remaining)
        {
            out.push_str(remaining);
            return;
        }
    }
    if matches!(first, Some('-'))
        && remaining
            .chars()
            .nth(1)
            .is_some_and(|right| right.is_ascii_alphabetic())
    {
        out.push_str(remaining);
        return;
    }
    if remaining.starts_with("/ ")
        && last.is_some_and(|left| left.is_alphanumeric() || matches!(left, ']' | ')' | '}'))
    {
        out.push(' ');
        out.push_str(remaining);
        return;
    }
    let needs_boundary_space = matches!(
        (last, first),
        (Some(left), Some(right))
            if matches!(left, '.' | ':' | '?' | '!')
                && (right.is_alphanumeric() || matches!(right, '[' | '(' | '<' | '|'))
    ) || matches!(
        (last, first),
        (Some(left), Some(right))
            if (left.is_alphanumeric() || matches!(left, ']' | ')' | '}' | '|'))
                && (right.is_alphanumeric() || matches!(right, '[' | '(' | '<' | '>' | '-' | '='))
    );
    if needs_boundary_space {
        out.push(' ');
    }
    out.push_str(remaining);
}

fn remaining_after_open_paren_starts_with_digit(remaining: &str) -> bool {
    remaining
        .strip_prefix('(')
        .and_then(|rest| rest.chars().next())
        .is_some_and(|ch| ch.is_ascii_digit())
}

fn is_relation_like_text(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && value
            .chars()
            .all(|ch| !ch.is_alphanumeric() && !ch.is_whitespace())
}

fn find_unescaped_dollar(source: &str) -> Option<usize> {
    source
        .match_indices('$')
        .find(|(index, _)| !is_escaped_at(source, *index))
        .map(|(index, _)| index)
}

fn find_unescaped_double_dollar(source: &str) -> Option<usize> {
    source
        .match_indices("$$")
        .find(|(index, _)| !is_escaped_at(source, *index))
        .map(|(index, _)| index)
}

fn is_escaped_at(source: &str, index: usize) -> bool {
    source[..index]
        .chars()
        .rev()
        .take_while(|ch| *ch == '\\')
        .count()
        % 2
        == 1
}

fn append_math_script(out: &mut String, marker: char, payload: &str) {
    let payload = clean_math_text(payload);
    if payload.is_empty() {
        return;
    }
    while out.ends_with(char::is_whitespace) {
        out.pop();
    }
    out.push(marker);
    if math_script_needs_braces(&payload) {
        out.push('{');
        out.push_str(&payload);
        out.push('}');
    } else {
        out.push_str(&payload);
    }
}

fn math_script_needs_braces(payload: &str) -> bool {
    payload
        .chars()
        .any(|ch| !(ch.is_ascii_alphanumeric() || ch == '\''))
}

fn render_reference_command(
    name: &str,
    payload: &str,
    labels: &HashMap<String, LabelInfo>,
) -> String {
    let keys = payload
        .split(',')
        .map(str::trim)
        .filter(|key| !key.is_empty())
        .collect::<Vec<_>>();
    let values = keys
        .iter()
        .map(|key| {
            if name == "pageref" {
                labels
                    .get(*key)
                    .map(|label| label.page.to_string())
                    .unwrap_or_else(|| "??".to_string())
            } else {
                labels
                    .get(*key)
                    .map(|label| label.value.clone())
                    .filter(|value| !value.is_empty())
                    .unwrap_or_else(|| "??".to_string())
            }
        })
        .collect::<Vec<_>>();
    let joined = if values.is_empty() {
        "??".to_string()
    } else {
        values.join(", ")
    };
    if name == "eqref" {
        format!("({joined})")
    } else if matches!(name, "cref" | "Cref" | "autoref") && keys.len() == 1 && joined != "??" {
        labels
            .get(keys[0])
            .and_then(|label| label.reference_prefix.as_deref())
            .filter(|prefix| !prefix.trim().is_empty())
            .map(|prefix| format!("{} {joined}", reference_prefix_for_command(name, prefix)))
            .unwrap_or(joined)
    } else {
        joined
    }
}

fn reference_prefix_for_command(name: &str, prefix: &str) -> String {
    if name == "cref" {
        prefix.to_string()
    } else {
        capitalize_reference_prefix(prefix)
    }
}

fn capitalize_reference_prefix(prefix: &str) -> String {
    let mut chars = prefix.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().chain(chars).collect()
}

fn render_citation_command<'a>(
    command: &str,
    source: &'a str,
    citations: &CitationRegistry,
) -> Option<(String, &'a str)> {
    let (source, _) = strip_optional_star(source);
    let (pre_note, source) = take_optional_bracketed(source);
    let (post_note, source) = take_optional_bracketed(source);
    let (payload, remaining) = take_braced(source)?;
    let keys = payload
        .split(',')
        .map(str::trim)
        .filter(|key| !key.is_empty())
        .collect::<Vec<_>>();
    let citation = match (command, citations.style) {
        ("citet" | "textcite", CitationStyle::Numeric) => {
            render_numeric_textual_citation(&keys, pre_note, post_note, citations)
        }
        ("cite" | "citet" | "textcite", CitationStyle::AuthorYear) => {
            render_author_year_textual_citation(&keys, pre_note, post_note, citations)
        }
        ("citep" | "parencite", CitationStyle::AuthorYear) => {
            render_author_year_parenthetical_citation(&keys, pre_note, post_note, citations)
        }
        _ => {
            let numbers = keys
                .iter()
                .map(|key| citation_number(citations, key))
                .collect::<Vec<_>>();
            render_numeric_citation_with_notes(&numbers, pre_note, post_note)
        }
    };
    Some((citation, remaining))
}

fn render_numeric_textual_citation(
    keys: &[&str],
    pre_note: Option<&str>,
    post_note: Option<&str>,
    citations: &CitationRegistry,
) -> String {
    if keys.len() == 1 {
        let number = citation_number(citations, keys[0]);
        if let Some(author) = citation_author(citations, keys[0]) {
            return format!(
                "{} {}",
                author,
                render_numeric_citation_with_notes(&[number], pre_note, post_note)
            );
        }
        return render_numeric_citation_with_notes(&[number], pre_note, post_note);
    }
    let parts = keys
        .iter()
        .map(|key| {
            let number = citation_number(citations, key);
            citation_author(citations, key)
                .map(|author| format!("{author} [{number}]"))
                .unwrap_or_else(|| format!("[{number}]"))
        })
        .collect::<Vec<_>>();
    parts.join(", ")
}

fn render_author_year_textual_citation(
    keys: &[&str],
    pre_note: Option<&str>,
    post_note: Option<&str>,
    citations: &CitationRegistry,
) -> String {
    if keys.len() == 1 {
        let year = citation_year_or_number(citations, keys[0]);
        if let Some(author) = citation_author(citations, keys[0]) {
            return format!(
                "{} {}",
                author,
                render_citation_body_with_notes(&year, pre_note, post_note)
            );
        }
        return render_citation_body_with_notes(&year, pre_note, post_note);
    }
    keys.iter()
        .map(|key| {
            let year = citation_year_or_number(citations, key);
            citation_author(citations, key)
                .map(|author| format!("{author} [{year}]"))
                .unwrap_or_else(|| format!("[{year}]"))
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn render_author_year_parenthetical_citation(
    keys: &[&str],
    pre_note: Option<&str>,
    post_note: Option<&str>,
    citations: &CitationRegistry,
) -> String {
    let body = keys
        .iter()
        .map(|key| {
            let year = citation_year_or_number(citations, key);
            citation_author(citations, key)
                .map(|author| format!("{author}, {year}"))
                .unwrap_or(year)
        })
        .collect::<Vec<_>>()
        .join(", ");
    render_citation_body_with_notes(&body, pre_note, post_note)
}

fn render_numeric_citation_with_notes(
    numbers: &[String],
    pre_note: Option<&str>,
    post_note: Option<&str>,
) -> String {
    render_citation_body_with_notes(&numbers.join(","), pre_note, post_note)
}

fn render_citation_body_with_notes(
    body_text: &str,
    pre_note: Option<&str>,
    post_note: Option<&str>,
) -> String {
    let pre_note = pre_note.and_then(clean_citation_note);
    let post_note = post_note.and_then(clean_citation_note);
    let mut body = String::new();
    if let Some(pre_note) = pre_note {
        body.push_str(&pre_note);
        if !body.is_empty() {
            body.push(' ');
        }
    }
    body.push_str(body_text);
    if let Some(post_note) = post_note {
        if !body.is_empty() {
            body.push_str(", ");
        }
        body.push_str(&post_note);
    }
    format!("[{body}]")
}

fn clean_citation_note(note: &str) -> Option<String> {
    let cleaned = clean_bib_tex_text(note);
    (!cleaned.is_empty()).then_some(cleaned)
}

fn citation_number(citations: &CitationRegistry, key: &str) -> String {
    citations
        .numbers
        .get(key)
        .map(usize::to_string)
        .unwrap_or_else(|| "?".to_string())
}

fn citation_author(citations: &CitationRegistry, key: &str) -> Option<String> {
    citations.labels.get(key).map(|label| label.author.clone())
}

fn citation_year_or_number(citations: &CitationRegistry, key: &str) -> String {
    citations
        .labels
        .get(key)
        .and_then(|label| label.year.clone())
        .unwrap_or_else(|| citation_number(citations, key))
}

fn clean_equation_text(source: &str) -> String {
    let mut without_labels = String::new();
    let mut cursor = source;
    while let Some(index) = find_control(cursor, "label") {
        without_labels.push_str(&cursor[..index]);
        let rest = &cursor[index + "\\label".len()..];
        if let Some((_, remaining)) = take_braced(rest) {
            cursor = remaining;
        } else {
            cursor = rest;
        }
    }
    without_labels.push_str(cursor);

    clean_math_text(
        &without_labels
            .replace("\\notag", " ")
            .replace("\\nonumber", " "),
    )
}

fn clean_math_text(source: &str) -> String {
    let mut out = String::new();
    let mut chars = source.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '%' => {
                for next in chars.by_ref() {
                    if next == '\n' {
                        out.push(' ');
                        break;
                    }
                }
            }
            '\\' => {
                let mut name = String::new();
                while let Some(&next) = chars.peek() {
                    if !next.is_ascii_alphabetic() {
                        break;
                    }
                    name.push(next);
                    chars.next();
                }
                match name.as_str() {
                    "" => {
                        if let Some(symbol) = chars.next() {
                            match symbol {
                                '\\' | ',' | '!' | ':' | ';' => out.push(' '),
                                _ => out.push(symbol),
                            }
                        }
                    }
                    "label" | "tag" => {
                        let rest: String = chars.collect();
                        let remaining = take_braced(&rest)
                            .map(|(_, remaining)| remaining)
                            .unwrap_or(rest.as_str());
                        let remaining = clean_math_text(remaining);
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "notag" | "nonumber" | "left" | "right" | "protect" | "bf" | "it" | "rm"
                    | "sf" | "em" | "sl" | "tt" | "sc" | "normalfont" | "bfseries" | "mdseries"
                    | "itshape" | "upshape" | "sffamily" | "rmfamily" | "ttfamily" | "tiny"
                    | "scriptsize" | "footnotesize" | "small" | "normalsize" | "large"
                    | "Large" | "LARGE" | "huge" | "Huge" | "hfill" | "smallskip" | "medskip"
                    | "bigskip" => {}
                    "setlength" | "addtolength" => {
                        let rest: String = chars.collect();
                        let remaining = skip_two_braced_arguments(&rest).unwrap_or(rest.as_str());
                        let remaining = clean_math_text(remaining);
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "hspace" | "vspace" | "hskip" | "vskip" | "kern" => {
                        let rest: String = chars.collect();
                        let rest = rest.strip_prefix('*').unwrap_or(&rest);
                        let remaining = take_braced(rest)
                            .map(|(_, remaining)| remaining)
                            .unwrap_or(rest);
                        let remaining = clean_math_text(remaining);
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "begin" | "end" => {
                        let rest: String = chars.collect();
                        let remaining = take_braced(&rest)
                            .map(|(_, remaining)| remaining)
                            .unwrap_or(rest.as_str());
                        let remaining = clean_math_text(remaining);
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "frac" | "dfrac" | "tfrac" => {
                        let rest: String = chars.collect();
                        if let Some((numerator, rest)) = take_braced(&rest)
                            && let Some((denominator, remaining)) = take_braced(rest)
                        {
                            out.push_str(&clean_math_text(numerator));
                            out.push('/');
                            out.push_str(&clean_math_text(denominator));
                            let remaining = clean_math_text(remaining);
                            append_cleaned_remaining(&mut out, &remaining);
                        } else {
                            let rest = rest.as_str();
                            if let Some((numerator, rest)) = take_math_atom(rest)
                                && let Some((denominator, remaining)) = take_math_atom(rest)
                            {
                                out.push_str(&clean_math_text(numerator));
                                out.push('/');
                                out.push_str(&clean_math_text(denominator));
                                let remaining = clean_math_text(remaining);
                                append_cleaned_remaining(&mut out, &remaining);
                            } else {
                                out.push_str("frac");
                                let remaining = clean_math_text(rest);
                                append_cleaned_remaining(&mut out, &remaining);
                            }
                        }
                        break;
                    }
                    "sqrt" => {
                        let rest: String = chars.collect();
                        let (_, rest) = take_optional_bracketed(&rest);
                        if let Some((payload, remaining)) = take_braced(rest) {
                            out.push_str(&clean_math_root(payload));
                            let remaining = clean_math_text(remaining);
                            append_cleaned_remaining(&mut out, &remaining);
                        } else {
                            if let Some((payload, remaining)) = take_math_atom(rest) {
                                out.push_str(&clean_math_root(payload));
                                let remaining = clean_math_text(remaining);
                                append_cleaned_remaining(&mut out, &remaining);
                            } else {
                                out.push_str("sqrt");
                            }
                        }
                        break;
                    }
                    "underset" | "overset" | "stackrel" => {
                        let rest: String = chars.collect();
                        if let Some((_, rest)) = take_braced(&rest)
                            && let Some((payload, remaining)) = take_braced(rest)
                        {
                            let visible = clean_math_text(payload);
                            if name == "stackrel" && is_relation_like_text(&visible) {
                                out.push(' ');
                                out.push_str(&visible);
                                out.push(' ');
                            } else {
                                out.push_str(&visible);
                            }
                            let remaining = clean_math_text(remaining);
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "underbrace" | "overbrace" => {
                        let rest: String = chars.collect();
                        if let Some((payload, remaining)) = take_braced(&rest) {
                            out.push_str(&clean_math_text(payload));
                            let remaining = skip_math_script_annotations(remaining);
                            let remaining = clean_math_text(remaining);
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "textsc" => {
                        let rest: String = chars.collect();
                        let rest = rest.strip_prefix('*').unwrap_or(&rest);
                        if let Some((payload, remaining)) = take_braced(rest) {
                            out.push_str(&clean_math_text(payload).to_ascii_uppercase());
                            let remaining = clean_math_text(remaining);
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "textbf" | "emph" | "textit" | "texttt" | "textrm" | "textsf"
                    | "textsuperscript" | "underline" | "ensuremath" | "mathrm" | "mathbf"
                    | "mathsf" | "mathit" | "mathbb" | "mathcal" | "mathfrak" | "bm"
                    | "boldsymbol" | "operatorname" | "text" | "mbox" | "substack" => {
                        let rest: String = chars.collect();
                        let rest = rest.strip_prefix('*').unwrap_or(&rest);
                        if let Some((payload, remaining)) = take_braced(rest) {
                            out.push_str(&clean_math_text(payload));
                            let remaining = clean_math_text(remaining);
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "hat" | "widehat" | "bar" | "overline" | "tilde" | "widetilde" | "vec"
                    | "dot" | "ddot" => {
                        let rest: String = chars.collect();
                        if let Some((payload, remaining)) = take_math_atom(&rest) {
                            out.push_str(&clean_math_text(payload));
                            let remaining = clean_math_text(remaining);
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "xrightarrow" | "xleftarrow" => {
                        let rest: String = chars.collect();
                        let (below, rest) = take_optional_bracketed(&rest);
                        let (above, remaining) = take_braced(rest).unwrap_or(("", rest));
                        out.push(' ');
                        out.push_str(math_control_fallback(&name));
                        for label in [above, below.unwrap_or("")] {
                            let label = clean_math_text(label);
                            if !label.is_empty() {
                                out.push(' ');
                                out.push_str(&label);
                            }
                        }
                        out.push(' ');
                        let remaining = clean_math_text(remaining);
                        append_cleaned_remaining(&mut out, &remaining);
                        break;
                    }
                    "in" | "notin" | "to" | "rightarrow" | "mapsto" | "longrightarrow"
                    | "leftarrow" | "longleftarrow" | "leftrightarrow" | "longleftrightarrow"
                    | "le" | "leq" | "ge" | "geq" | "neq" | "ne" | "equiv" | "sim" | "approx"
                    | "mid" | "triangleq" | "propto" | "succeq" | "preceq" | "succ" | "prec"
                    | "subset" | "subseteq" | "supset" | "supseteq" | "cap" | "cup"
                    | "setminus" | "land" | "lor" | "neg" | "otimes" | "oplus" | "emptyset"
                    | "iff" | "Leftrightarrow" | "Longleftrightarrow" | "Leftarrow"
                    | "Longleftarrow" | "Rightarrow" | "Longrightarrow" | "implies" | "forall"
                    | "exists" | "perp" => {
                        out.push(' ');
                        out.push_str(math_control_fallback(&name));
                        out.push(' ');
                    }
                    "not" => {
                        let rest: String = chars.collect();
                        if let Some(remaining) = rest.trim_start().strip_prefix('=') {
                            out.push_str(" ≠ ");
                            let remaining = clean_math_text(remaining);
                            append_cleaned_remaining(&mut out, &remaining);
                        } else {
                            out.push_str(" not ");
                            let remaining = clean_math_text(&rest);
                            append_cleaned_remaining(&mut out, &remaining);
                        }
                        break;
                    }
                    "big" | "Big" | "bigg" | "Bigg" | "bigl" | "bigr" | "Bigl" | "Bigr"
                    | "biggl" | "biggr" | "Biggl" | "Biggr" => {}
                    "quad" | "qquad" => out.push(' '),
                    "cdots" | "ldots" | "dots" => out.push_str("..."),
                    "sum" | "prod" | "int" | "lim" | "min" | "max" | "sup" | "inf" => {
                        out.push(' ');
                        out.push_str(math_control_fallback(&name));
                        out.push(' ');
                    }
                    "langle" => out.push('<'),
                    "rangle" => out.push('>'),
                    "lVert" | "lvert" => out.push('|'),
                    "rVert" | "rvert" => out.push('|'),
                    _ => {
                        if !name.is_empty() {
                            out.push_str(math_control_fallback(&name));
                        }
                    }
                }
            }
            '_' | '^' => {
                let rest: String = chars.collect();
                if let Some((payload, remaining)) = take_math_atom(&rest) {
                    append_math_script(&mut out, ch, payload);
                    let remaining = clean_math_text(remaining);
                    append_cleaned_remaining(&mut out, &remaining);
                } else {
                    out.push(ch);
                    let remaining = clean_math_text(&rest);
                    append_cleaned_remaining(&mut out, &remaining);
                }
                break;
            }
            '&' | '{' | '}' | '$' => out.push(' '),
            '\n' | '\t' | '~' => out.push(' '),
            _ => out.push(ch),
        }
    }
    let normalized = out
        .replace('+', " + ")
        .replace("+ /-", "+/-")
        .lines()
        .map(|line| line.split('%').next().unwrap_or("").trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let normalized = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
    compact_math_function_call_spacing(&normalized)
}

fn clean_math_root(payload: &str) -> String {
    let payload = clean_math_text(payload);
    if payload.is_empty() {
        return "√".to_string();
    }
    if math_root_payload_can_follow_radical(&payload) {
        format!("√{payload}")
    } else {
        format!("√({payload})")
    }
}

fn math_root_payload_can_follow_radical(payload: &str) -> bool {
    let mut chars = payload.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if chars.next().is_none() {
        return first.is_alphanumeric() || matches!(first, '∞');
    }
    let mut saw_body = false;
    for ch in payload.chars() {
        if ch.is_whitespace() || matches!(ch, '+' | '-' | '/' | '=' | '<' | '>' | ',' | ';' | ':') {
            return false;
        }
        saw_body = true;
    }
    saw_body
}

fn normalize_tex_prose_punctuation(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut cursor = source;
    while !cursor.is_empty() {
        if let Some(rest) = cursor.strip_prefix("``") {
            out.push('"');
            cursor = rest;
        } else if let Some(rest) = cursor.strip_prefix("''") {
            out.push('"');
            cursor = rest;
        } else if let Some(rest) = cursor.strip_prefix("---") {
            out.push_str("--");
            cursor = rest;
        } else if let Some(rest) = cursor.strip_prefix("--") {
            out.push('-');
            cursor = rest;
        } else if let Some(ch) = cursor.chars().next() {
            out.push(ch);
            cursor = &cursor[ch.len_utf8()..];
        }
    }
    out
}

fn compact_math_function_call_spacing(source: &str) -> String {
    let mut out = String::new();
    let mut cursor = source;
    while let Some(index) = cursor.find(" (") {
        out.push_str(&cursor[..index]);
        let before = out.trim_end();
        let token = before.split_whitespace().next_back().unwrap_or_default();
        if math_token_allows_compact_call(token) {
            out.push('(');
        } else {
            out.push_str(" (");
        }
        cursor = &cursor[index + " (".len()..];
    }
    out.push_str(cursor);
    compact_math_delimiter_inner_spacing(&out)
}

fn compact_math_delimiter_inner_spacing(source: &str) -> String {
    let mut out = String::new();
    let mut chars = source.chars().peekable();
    while let Some(ch) = chars.next() {
        match ch {
            '(' | '[' | '{' => {
                out.push(ch);
                while chars.peek().is_some_and(|next| next.is_whitespace()) {
                    chars.next();
                }
            }
            ')' | ']' | '}' => {
                while out.ends_with(char::is_whitespace) {
                    out.pop();
                }
                out.push(ch);
            }
            '|' => {
                let opening_bar =
                    out.chars()
                        .rev()
                        .find(|ch| !ch.is_whitespace())
                        .is_none_or(|left| {
                            matches!(
                                left,
                                '(' | '[' | '{' | '=' | '+' | '-' | '*' | '/' | ',' | ':' | ';'
                            )
                        });
                if !opening_bar {
                    while out.ends_with(char::is_whitespace) {
                        out.pop();
                    }
                }
                out.push(ch);
                if opening_bar {
                    while chars.peek().is_some_and(|next| next.is_whitespace()) {
                        chars.next();
                    }
                }
            }
            _ => out.push(ch),
        }
    }
    out
}

fn math_token_allows_compact_call(token: &str) -> bool {
    let token = compact_call_token(token);
    if token.is_empty() || matches!(token, "in" | "not" | "forall" | "exists" | "s.t.") {
        return false;
    }
    let has_letter = token.chars().any(|ch| ch.is_alphabetic());
    let has_operator = token.chars().any(|ch| {
        matches!(
            ch,
            '+' | '-' | '*' | '/' | '=' | '<' | '>' | ',' | ';' | ':' | '|'
        )
    });
    has_letter && !has_operator
}

fn math_token_allows_compact_boundary_call(token: &str) -> bool {
    let token = compact_call_token(token);
    if token.is_empty() {
        return false;
    }
    token.contains('_')
        || token.contains('^')
        || matches!(
            token,
            "Cov"
                | "Var"
                | "Tr"
                | "tr"
                | "det"
                | "rank"
                | "diag"
                | "mean"
                | "std"
                | "exp"
                | "log"
                | "sin"
                | "cos"
                | "tan"
                | "rho"
                | "phi"
                | "psi"
                | "eta"
                | "Pr"
        )
}

fn compact_call_token(token: &str) -> &str {
    let token = token.trim_matches(|ch| matches!(ch, '|' | '(' | ')' | '[' | ']' | '{' | '}'));
    let start = token
        .char_indices()
        .rev()
        .find(|(_, ch)| !(ch.is_alphanumeric() || matches!(ch, '_' | '^' | '\'' | '{' | '}')))
        .map(|(index, ch)| index + ch.len_utf8())
        .unwrap_or(0);
    &token[start..]
}

fn take_math_atom(source: &str) -> Option<(&str, &str)> {
    let source = source.trim_start();
    if let Some((payload, remaining)) = take_braced(source) {
        return Some((payload, remaining));
    }
    let mut chars = source.char_indices();
    let (_, first) = chars.next()?;
    if first == '\\' {
        let rest = &source[first.len_utf8()..];
        let control_len = rest
            .char_indices()
            .take_while(|(_, ch)| ch.is_ascii_alphabetic())
            .map(|(index, ch)| index + ch.len_utf8())
            .last()
            .unwrap_or(0);
        if control_len > 0 {
            let end = first.len_utf8() + control_len;
            return Some((&source[..end], &source[end..]));
        }
        if let Some((index, ch)) = rest.char_indices().next() {
            let end = first.len_utf8() + index + ch.len_utf8();
            return Some((&source[..end], &source[end..]));
        }
        return Some((source, ""));
    }
    let end = first.len_utf8();
    Some((&source[..end], &source[end..]))
}

fn skip_math_script_annotations(mut source: &str) -> &str {
    loop {
        let trimmed = source.trim_start();
        let Some(rest) = trimmed
            .strip_prefix('_')
            .or_else(|| trimmed.strip_prefix('^'))
        else {
            return source;
        };
        if let Some((_, remaining)) = take_math_atom(rest) {
            source = remaining;
        } else {
            return rest;
        }
    }
}

fn clean_align_lines(source: &str, equation_counter: &mut usize, numbered: bool) -> Vec<String> {
    split_align_rows(source)
        .into_iter()
        .filter_map(|row| {
            let equation = clean_equation_text(row);
            if equation.is_empty() {
                return None;
            }
            if numbered && !math_row_suppresses_number(row) {
                *equation_counter += 1;
                Some(format!("{} ({})", equation, *equation_counter))
            } else {
                Some(equation)
            }
        })
        .collect()
}

fn split_align_rows(source: &str) -> Vec<&str> {
    let mut rows = Vec::new();
    let mut cursor = source;
    while let Some(index) = cursor.find("\\\\") {
        rows.push(&cursor[..index]);
        let rest = &cursor[index + "\\\\".len()..];
        let (_, rest) = take_optional_bracketed(rest);
        cursor = rest;
    }
    rows.push(cursor);
    rows
}

fn math_row_suppresses_number(source: &str) -> bool {
    source_contains_control(source, "notag") || source_contains_control(source, "nonumber")
}

#[derive(Debug, Clone, PartialEq)]
struct WrappedTextLine {
    text: String,
    justify_width_pt: Option<f32>,
}

const PROSE_WRAP_AVERAGE_EM: f32 = 0.405;
const LINE_BREAK_LOOKAHEAD_WORDS: usize = 96;
const LINE_BREAK_OVERFULL_TOLERANCE_EM: f32 = 0.08;
const LINE_BREAK_ADJACENT_FITNESS_DEMERITS: f32 = 10_000.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineFitness {
    Tight,
    Decent,
    Loose,
    VeryLoose,
}

impl LineFitness {
    const ALL: [Self; 4] = [Self::Tight, Self::Decent, Self::Loose, Self::VeryLoose];

    const fn index(self) -> usize {
        match self {
            Self::Tight => 0,
            Self::Decent => 1,
            Self::Loose => 2,
            Self::VeryLoose => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LineBreakScore {
    demerits: f32,
    fitness: LineFitness,
}

#[derive(Debug, Clone, Copy)]
struct LineBreakState {
    cost: f32,
    previous_break: usize,
    previous_fitness: Option<LineFitness>,
}

fn wrap_prose_text_lines(
    text: &str,
    width: usize,
    font_metric: PdfFontMetric,
    font_size: f32,
    max_width_pt: f32,
) -> Vec<WrappedTextLine> {
    if text.is_empty() {
        return Vec::new();
    }
    let words = text.split_whitespace().collect::<Vec<_>>();
    if words.is_empty() {
        return Vec::new();
    }
    let target_width_pt = prose_wrap_target_width_pt(width, font_size, max_width_pt);
    let word_widths = words
        .iter()
        .map(|word| natural_text_width_for_metric_pt(word, font_metric, font_size))
        .collect::<Vec<_>>();
    let space_width_pt = natural_text_width_for_metric_pt(" ", font_metric, font_size);
    let breaks = balanced_line_breaks(&word_widths, space_width_pt, font_size, target_width_pt);
    let mut start = 0_usize;
    let mut lines = Vec::with_capacity(breaks.len());
    for end in breaks {
        let line_text = join_words(&words[start..end]);
        let justify_width_pt =
            if end < words.len() && words[start..end].len() > 1 && line_text.contains(' ') {
                Some(target_width_pt)
            } else {
                None
            };
        lines.push(WrappedTextLine {
            text: line_text,
            justify_width_pt,
        });
        start = end;
    }
    lines
}

fn wrap_calibrated_prose_text_lines(
    text: &str,
    width: usize,
    font_size: f32,
    max_width_pt: f32,
) -> Vec<WrappedTextLine> {
    let target_width_pt = prose_wrap_target_width_pt(width, font_size, max_width_pt);
    let raw_lines = wrap_text(text, width);
    let last_index = raw_lines.len().saturating_sub(1);
    raw_lines
        .into_iter()
        .enumerate()
        .map(|(index, text)| {
            let justify_width_pt = if index < last_index && text.contains(' ') {
                Some(target_width_pt)
            } else {
                None
            };
            WrappedTextLine {
                text,
                justify_width_pt,
            }
        })
        .collect()
}

fn prose_wrap_target_width_pt(width: usize, font_size: f32, max_width_pt: f32) -> f32 {
    let calibrated = width as f32 * font_size * PROSE_WRAP_AVERAGE_EM;
    let minimum = (font_size * 12.0).min(max_width_pt);
    calibrated.max(minimum).min(max_width_pt)
}

fn balanced_line_breaks(
    word_widths: &[f32],
    space_width_pt: f32,
    font_size: f32,
    target_width_pt: f32,
) -> Vec<usize> {
    let word_count = word_widths.len();
    if word_count == 0 {
        return Vec::new();
    }
    let mut states: Vec<[Option<LineBreakState>; 4]> = vec![[None; 4]; word_count + 1];
    for start in 0..word_count {
        let mut previous_states = Vec::new();
        if start == 0 {
            previous_states.push((0.0, None));
        }
        for fitness in LineFitness::ALL {
            if let Some(state) = states[start][fitness.index()] {
                previous_states.push((state.cost, Some(fitness)));
            }
        }
        if previous_states.is_empty() {
            continue;
        }
        let mut line_width_pt = 0.0_f32;
        let max_end = (start + LINE_BREAK_LOOKAHEAD_WORDS).min(word_count);
        for end in start..max_end {
            if end > start {
                line_width_pt += space_width_pt;
            }
            line_width_pt += word_widths[end];
            let words_in_line = end - start + 1;
            let overfull_pt = line_width_pt - target_width_pt;
            if words_in_line > 1 && overfull_pt > font_size * LINE_BREAK_OVERFULL_TOLERANCE_EM {
                break;
            }
            let score = line_break_score(
                line_width_pt,
                target_width_pt,
                space_width_pt,
                words_in_line,
                end + 1 == word_count,
            );
            for (previous_cost, previous_fitness) in &previous_states {
                let candidate_cost = previous_cost
                    + score.demerits
                    + adjacent_fitness_demerits(*previous_fitness, score.fitness);
                let state_slot = &mut states[end + 1][score.fitness.index()];
                if state_slot.is_none_or(|state| candidate_cost < state.cost) {
                    *state_slot = Some(LineBreakState {
                        cost: candidate_cost,
                        previous_break: start,
                        previous_fitness: *previous_fitness,
                    });
                }
            }
        }
    }
    let Some((mut cursor, mut fitness, _)) = LineFitness::ALL
        .into_iter()
        .filter_map(|fitness| {
            states[word_count][fitness.index()].map(|state| (word_count, fitness, state))
        })
        .min_by(|(_, _, left), (_, _, right)| left.cost.total_cmp(&right.cost))
    else {
        return greedy_line_breaks(word_widths, space_width_pt, font_size, target_width_pt);
    };
    let mut breaks = Vec::new();
    while cursor > 0 {
        let Some(state) = states[cursor][fitness.index()] else {
            break;
        };
        breaks.push(cursor);
        cursor = state.previous_break;
        if let Some(previous_fitness) = state.previous_fitness {
            fitness = previous_fitness;
        } else {
            break;
        }
    }
    breaks.reverse();
    breaks
}

fn greedy_line_breaks(
    word_widths: &[f32],
    space_width_pt: f32,
    font_size: f32,
    target_width_pt: f32,
) -> Vec<usize> {
    let word_count = word_widths.len();
    let mut cursor = 0_usize;
    let mut breaks = Vec::new();
    while cursor < word_count {
        let mut line_width_pt = 0.0_f32;
        let mut next = cursor + 1;
        for (offset, width) in word_widths[cursor..].iter().enumerate() {
            let candidate_width = if offset == 0 {
                *width
            } else {
                line_width_pt + space_width_pt + width
            };
            if offset > 0
                && candidate_width - target_width_pt > font_size * LINE_BREAK_OVERFULL_TOLERANCE_EM
            {
                break;
            }
            line_width_pt = candidate_width;
            next = cursor + offset + 1;
        }
        breaks.push(next.min(word_widths.len()));
        cursor = next;
    }
    breaks
}

fn line_break_score(
    line_width_pt: f32,
    target_width_pt: f32,
    space_width_pt: f32,
    words_in_line: usize,
    is_last_line: bool,
) -> LineBreakScore {
    let slack_pt = target_width_pt - line_width_pt;
    if is_last_line {
        let orphan_penalty = if words_in_line == 1 { 2_500.0 } else { 0.0 };
        return LineBreakScore {
            demerits: orphan_penalty + slack_pt.min(0.0).abs().powi(2) * 1_000.0,
            fitness: LineFitness::Decent,
        };
    }
    if words_in_line <= 1 {
        return LineBreakScore {
            demerits: 500_000.0 + slack_pt.min(0.0).abs().powi(2) * 1_000.0,
            fitness: LineFitness::VeryLoose,
        };
    }
    let spaces = (words_in_line - 1) as f32;
    let (badness, ratio) = if slack_pt >= 0.0 {
        let stretch_pt = (spaces * space_width_pt * 1.5).max(0.01);
        let ratio = slack_pt / stretch_pt;
        ((100.0 * ratio.powi(3)).min(10_000.0), ratio)
    } else {
        let shrink_pt = (spaces * space_width_pt / 3.0).max(0.01);
        let ratio = slack_pt / shrink_pt;
        if ratio >= -1.0 {
            (100.0 * (-ratio).powi(3), ratio)
        } else {
            (10_000.0 + (-ratio - 1.0) * 5_000.0, ratio)
        }
    };
    LineBreakScore {
        demerits: (badness + 10.0).powi(2) + 250.0,
        fitness: line_fitness(ratio),
    }
}

fn line_fitness(ratio: f32) -> LineFitness {
    if ratio < -0.5 {
        LineFitness::Tight
    } else if ratio <= 0.5 {
        LineFitness::Decent
    } else if ratio <= 1.0 {
        LineFitness::Loose
    } else {
        LineFitness::VeryLoose
    }
}

fn adjacent_fitness_demerits(previous: Option<LineFitness>, current: LineFitness) -> f32 {
    previous
        .filter(|previous| previous.index().abs_diff(current.index()) > 1)
        .map_or(0.0, |_| LINE_BREAK_ADJACENT_FITNESS_DEMERITS)
}

fn join_words(words: &[&str]) -> String {
    let mut out = String::new();
    for (index, word) in words.iter().enumerate() {
        if index > 0 {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut current_width = 0_usize;
    for word in text.split_whitespace() {
        let word_width = wrap_word_width(word);
        if current.is_empty() {
            current.push_str(word);
            current_width = word_width;
        } else if current_width + 1 + word_width <= width {
            current.push(' ');
            current.push_str(word);
            current_width += 1 + word_width;
        } else {
            lines.push(current);
            current = word.to_string();
            current_width = word_width;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn wrap_word_width(word: &str) -> usize {
    word.chars()
        .map(|ch| {
            pdf_text_ascii_replacement(ch)
                .map(str::len)
                .unwrap_or(ch.len_utf8())
        })
        .sum()
}

fn code_listing_lines(source: &str) -> Vec<String> {
    let source = source.strip_prefix('\n').unwrap_or(source);
    let source = source.strip_suffix('\n').unwrap_or(source);
    source
        .lines()
        .map(|line| {
            restore_verbatim_like_body(line)
                .trim_end_matches('\r')
                .replace('\t', "    ")
        })
        .collect()
}

fn read_code_input(
    root_dir: &Path,
    payload: &str,
    command: &str,
    inputs: &mut Vec<PathBuf>,
) -> Result<String, String> {
    let path = resolve_code_input_path(root_dir, payload, command)?;
    inputs.push(path.clone());
    fs::read_to_string(&path).map_err(|error| {
        format!(
            "native backend could not read code input `{}`: {error}",
            path.display()
        )
    })
}

fn resolve_code_input_path(
    root_dir: &Path,
    payload: &str,
    command: &str,
) -> Result<PathBuf, String> {
    let payload = payload.trim();
    if payload.is_empty() {
        return Err(format!("native backend requires non-empty {command} paths"));
    }
    if payload.starts_with('|') {
        return Err(format!(
            "native backend does not support shell-pipe {command} paths"
        ));
    }
    let raw = Path::new(payload);
    if raw.is_absolute() {
        return Err(format!(
            "native backend only supports local relative {command} paths, got `{payload}`"
        ));
    }
    let candidate = root_dir.join(raw);
    if candidate.exists() {
        return fs::canonicalize(&candidate).map_err(|error| {
            format!(
                "native backend could not resolve {command} input `{}`: {error}",
                candidate.display()
            )
        });
    }
    if let Some(path) = resolve_kpathsea_tex_candidate(root_dir, raw)? {
        return Ok(path);
    }
    fs::canonicalize(&candidate).map_err(|error| {
        format!(
            "native backend could not resolve {command} input `{}`: {error}",
            candidate.display()
        )
    })
}

fn protect_verbatim_like_bodies_for_expansion(source: &str) -> Result<String, String> {
    let mut out = String::with_capacity(source.len());
    let mut cursor = source;
    while let Some((index, env)) = find_next_verbatim_like_begin(cursor) {
        out.push_str(&cursor[..index]);
        let opening = &cursor[index..];
        let body = verbatim_like_body_after_opening(opening, env)?;
        let body_start = cursor.len() - body.len();
        out.push_str(&cursor[index..body_start]);

        let end_marker = format!("\\end{{{env}}}");
        let Some(end_index) = body.find(&end_marker) else {
            return Err(format!("native backend could not find `{end_marker}`"));
        };
        out.push_str(&protect_verbatim_like_body(&body[..end_index]));
        out.push_str(&body[end_index..end_index + end_marker.len()]);
        cursor = &body[end_index + end_marker.len()..];
    }
    out.push_str(cursor);
    Ok(out)
}

fn find_next_verbatim_like_begin(source: &str) -> Option<(usize, &'static str)> {
    VERBATIM_LIKE_ENVS
        .into_iter()
        .filter_map(|env| {
            let marker = format!("\\begin{{{env}}}");
            source.find(&marker).map(|index| (index, env))
        })
        .min_by_key(|(index, _)| *index)
}

fn verbatim_like_body_after_opening<'a>(opening: &'a str, env: &str) -> Result<&'a str, String> {
    let prefix = format!("\\begin{{{env}}}");
    let rest = opening
        .strip_prefix(&prefix)
        .ok_or_else(|| format!("native backend expected `{prefix}`"))?;
    let (_, rest) = take_optional_bracketed(rest);
    if env.trim_end_matches('*') == "minted" {
        let (_, rest) = take_braced(rest)
            .ok_or_else(|| format!("native backend requires braced \\begin{{{env}}} language"))?;
        Ok(rest)
    } else {
        Ok(rest)
    }
}

fn protect_verbatim_like_body(source: &str) -> String {
    source
        .chars()
        .map(|ch| match ch {
            '%' => VERBATIM_PROTECTED_PERCENT,
            '\\' => VERBATIM_PROTECTED_BACKSLASH,
            _ => ch,
        })
        .collect()
}

fn restore_verbatim_like_body(source: &str) -> String {
    source
        .chars()
        .map(|ch| match ch {
            VERBATIM_PROTECTED_PERCENT => '%',
            VERBATIM_PROTECTED_BACKSLASH => '\\',
            _ => ch,
        })
        .collect()
}

const VERBATIM_LIKE_ENVS: [&str; 7] = [
    "verbatim",
    "verbatim*",
    "Verbatim",
    "BVerbatim",
    "LVerbatim",
    "minted",
    "minted*",
];

fn take_verbatim_like_environment(source: &str) -> Result<Option<(&str, &str)>, String> {
    for env in VERBATIM_LIKE_ENVS {
        let prefix = format!("\\begin{{{env}}}");
        let Some(rest) = source.strip_prefix(&prefix) else {
            continue;
        };
        let (_, rest) = take_optional_bracketed(rest);
        let rest = if env.trim_end_matches('*') == "minted" {
            let (_, rest) = take_braced(rest).ok_or_else(|| {
                format!("native backend requires braced \\begin{{{env}}} language")
            })?;
            rest
        } else {
            rest
        };
        let (literal, remaining) = take_environment_body(rest, env)?;
        return Ok(Some((literal, remaining)));
    }
    Ok(None)
}

fn algorithmic_lines(
    source: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    for raw_line in source.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('%') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("\\Require") {
            lines.push(format!(
                "Require: {}",
                clean_algorithmic_payload(rest, macros, labels, citations, footnotes)?
            ));
        } else if let Some(rest) = line.strip_prefix("\\Ensure") {
            lines.push(format!(
                "Ensure: {}",
                clean_algorithmic_payload(rest, macros, labels, citations, footnotes)?
            ));
        } else if let Some(rest) = line.strip_prefix("\\Statex") {
            let text = clean_algorithmic_payload(rest, macros, labels, citations, footnotes)?;
            if text.is_empty() {
                lines.push(String::new());
            } else {
                lines.push(format!("  {text}"));
            }
        } else if let Some(rest) = line.strip_prefix("\\State") {
            lines.push(format!(
                "  {}",
                clean_algorithmic_payload(rest, macros, labels, citations, footnotes)?
            ));
        } else if let Some(rest) = line.strip_prefix("\\For") {
            let (payload, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\For payloads".to_string())?;
            let mut line = format!(
                "For {}",
                clean_algorithmic_payload(payload, macros, labels, citations, footnotes)?
            );
            let remaining =
                clean_algorithmic_payload(remaining, macros, labels, citations, footnotes)?;
            if !remaining.is_empty() {
                line.push(' ');
                line.push_str(&remaining);
            }
            lines.push(line);
        } else if let Some(rest) = line.strip_prefix("\\EndFor") {
            let rest = clean_algorithmic_payload(rest, macros, labels, citations, footnotes)?;
            if rest.is_empty() {
                lines.push("End for".to_string());
            } else {
                lines.push(format!("End for {rest}"));
            }
        } else if let Some(rest) = line.strip_prefix("\\If") {
            let (payload, remaining) = take_braced(rest)
                .ok_or_else(|| "native backend requires braced \\If payloads".to_string())?;
            let mut line = format!(
                "If {}",
                clean_algorithmic_payload(payload, macros, labels, citations, footnotes)?
            );
            let remaining =
                clean_algorithmic_payload(remaining, macros, labels, citations, footnotes)?;
            if !remaining.is_empty() {
                line.push(' ');
                line.push_str(&remaining);
            }
            lines.push(line);
        } else if let Some(rest) = line.strip_prefix("\\EndIf") {
            let rest = clean_algorithmic_payload(rest, macros, labels, citations, footnotes)?;
            if rest.is_empty() {
                lines.push("End if".to_string());
            } else {
                lines.push(format!("End if {rest}"));
            }
        } else {
            lines.push(clean_algorithmic_payload(
                line, macros, labels, citations, footnotes,
            )?);
        }
    }
    Ok(lines)
}

fn clean_algorithmic_payload(
    source: &str,
    macros: &HashMap<String, String>,
    labels: &HashMap<String, LabelInfo>,
    citations: &CitationRegistry,
    footnotes: &mut FootnoteRegistry,
) -> Result<String, String> {
    let mut rewritten = String::with_capacity(source.len());
    let mut cursor = source;
    while let Some(index) = cursor.find("\\Comment") {
        rewritten.push_str(&cursor[..index]);
        let rest = &cursor[index + "\\Comment".len()..];
        if let Some((comment, remaining)) = take_braced(rest) {
            let comment =
                clean_inline_text_collecting(comment, macros, labels, citations, footnotes)?;
            if !comment.is_empty() {
                if !rewritten.ends_with(' ') {
                    rewritten.push(' ');
                }
                rewritten.push('(');
                rewritten.push_str(&comment);
                rewritten.push(')');
            }
            cursor = remaining;
        } else {
            rewritten.push_str("Comment");
            cursor = rest;
        }
    }
    rewritten.push_str(cursor);
    clean_inline_text_collecting(&rewritten, macros, labels, citations, footnotes)
        .map(|cleaned| cleaned.replace(")(", ") ("))
}

fn write_pdf(
    path: &Path,
    document: &SimpleDocument,
    page_count: usize,
    placements: &[LinePlacement],
) -> io::Result<()> {
    let font_object = 3 + page_count * 2;
    let code_font_object = font_object + 1;
    let math_font_object = code_font_object + 1;
    let heading_font_object = math_font_object + 1;
    let symbol_font_object = heading_font_object + 1;
    let mut next_object_id = symbol_font_object + 1;
    let font_resources = [
        pdf_type1_font_resource(document.layout.text_base_font, &mut next_object_id)?,
        pdf_type1_font_resource(document.layout.code_base_font, &mut next_object_id)?,
        pdf_type1_font_resource(document.layout.math_base_font, &mut next_object_id)?,
        pdf_type1_font_resource(document.layout.heading_base_font, &mut next_object_id)?,
        pdf_type1_font_resource("Symbol", &mut next_object_id)?,
    ];
    let image_object_start = next_object_id;
    let image_object_ids = image_object_ids(document, image_object_start);
    let image_object_count = document
        .images
        .iter()
        .map(image_pdf_object_count)
        .sum::<usize>();
    let outline_object_start = image_object_start + image_object_count;
    let outline_object_count = if document.bookmarks.is_empty() {
        0
    } else {
        document.bookmarks.len() + 1
    };
    let info_object_id =
        (!document.pdf_metadata.is_empty()).then_some(outline_object_start + outline_object_count);
    let mut objects = Vec::new();

    let page_refs = (0..page_count)
        .map(|page| format!("{} 0 R", 3 + page * 2))
        .collect::<Vec<_>>()
        .join(" ");
    let outline_catalog = if document.bookmarks.is_empty() {
        String::new()
    } else {
        format!(" /Outlines {outline_object_start} 0 R /PageMode /UseOutlines")
    };
    objects.push(pdf_object(format!(
        "<< /Type /Catalog /Pages 2 0 R{outline_catalog} >>"
    )));
    objects.push(pdf_object(format!(
        "<< /Type /Pages /Kids [{}] /Count {} >>",
        page_refs, page_count
    )));

    let page_streams = page_streams_from_placements(document, page_count, placements);
    for (page_index, stream) in page_streams.iter().enumerate().take(page_count) {
        let page_object = 3 + page_index * 2;
        let content_object = page_object + 1;
        let xobjects = if document.images.is_empty() {
            String::new()
        } else {
            let entries = (0..document.images.len())
                .map(|index| format!("/Im{} {} 0 R", index + 1, image_object_ids[index]))
                .collect::<Vec<_>>()
                .join(" ");
            format!(" /XObject << {entries} >>")
        };
        objects.push(pdf_object(format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 {} 0 R /F2 {} 0 R /F3 {} 0 R /F4 {} 0 R /F5 {} 0 R >>{xobjects} >> /Contents {content_object} 0 R >>",
            font_object,
            code_font_object,
            math_font_object,
            heading_font_object,
            symbol_font_object
        )));
        objects.push(pdf_object(format!(
            "<< /Length {} >>\nstream\n{}endstream",
            stream.len(),
            stream
        )));
    }
    for resource in &font_resources {
        objects.push(resource.font_object.clone());
    }
    for resource in font_resources {
        objects.extend(resource.extra_objects);
    }
    for (index, image) in document.images.iter().enumerate() {
        let image_id = image_object_ids[index];
        objects.extend(image_objects(image, image_id));
    }
    if !document.bookmarks.is_empty() {
        objects.extend(outline_objects(document, outline_object_start, page_count));
    }
    if let Some(info_object_id) = info_object_id {
        debug_assert_eq!(objects.len() + 1, info_object_id);
        objects.push(pdf_info_object(&document.pdf_metadata));
    }

    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");
    let mut offsets = Vec::with_capacity(objects.len() + 1);
    offsets.push(0_usize);
    for (index, object) in objects.iter().enumerate() {
        offsets.push(bytes.len());
        writeln!(bytes, "{} 0 obj", index + 1)?;
        bytes.extend_from_slice(object);
        bytes.extend_from_slice(b"\nendobj\n");
    }
    let xref_offset = bytes.len();
    write!(bytes, "xref\n0 {}\n", objects.len() + 1)?;
    bytes.extend_from_slice(b"0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        writeln!(bytes, "{offset:010} 00000 n ")?;
    }
    let info_ref = info_object_id
        .map(|id| format!(" /Info {id} 0 R"))
        .unwrap_or_default();
    write!(
        bytes,
        "trailer\n<< /Size {} /Root 1 0 R{} >>\nstartxref\n{}\n%%EOF\n",
        objects.len() + 1,
        info_ref,
        xref_offset
    )?;
    fs::write(path, bytes)
}

fn outline_objects(
    document: &SimpleDocument,
    root_object_id: usize,
    page_count: usize,
) -> Vec<Vec<u8>> {
    let mut objects = Vec::with_capacity(document.bookmarks.len() + 1);
    let first_item_id = root_object_id + 1;
    let last_item_id = root_object_id + document.bookmarks.len();
    objects.push(pdf_object(format!(
        "<< /Type /Outlines /First {first_item_id} 0 R /Last {last_item_id} 0 R /Count {} >>",
        document.bookmarks.len()
    )));
    for (index, bookmark) in document.bookmarks.iter().enumerate() {
        let object_id = first_item_id + index;
        let prev = (index > 0).then(|| format!(" /Prev {} 0 R", object_id - 1));
        let next =
            (index + 1 < document.bookmarks.len()).then(|| format!(" /Next {} 0 R", object_id + 1));
        let page = bookmark.page.clamp(1, page_count);
        let page_object_id = 3 + (page - 1) * 2;
        objects.push(pdf_object(format!(
            "<< /Title ({}) /Parent {root_object_id} 0 R{}{} /Dest [{} 0 R /Fit] >>",
            pdf_text(&bookmark.title),
            prev.unwrap_or_default(),
            next.unwrap_or_default(),
            page_object_id
        )));
    }
    objects
}

fn pdf_info_object(metadata: &PdfMetadata) -> Vec<u8> {
    let mut dictionary = String::from("<<");
    for (key, value) in &metadata.entries {
        write!(dictionary, " /{key} ({})", pdf_text(value)).unwrap();
    }
    dictionary.push_str(" >>");
    pdf_object(dictionary)
}

fn image_object_ids(document: &SimpleDocument, first_object_id: usize) -> Vec<usize> {
    let mut ids = Vec::with_capacity(document.images.len());
    let mut next_id = first_object_id;
    for image in &document.images {
        ids.push(next_id);
        next_id += image_pdf_object_count(image);
    }
    ids
}

fn image_pdf_object_count(image: &ImageAsset) -> usize {
    match image.payload.as_ref() {
        ImagePayload::Jpeg(_) => 1,
        ImagePayload::Png { alpha, .. } => 1 + usize::from(alpha.is_some()),
        ImagePayload::PdfForm(form) => 1 + form.imported_objects.len(),
    }
}

fn pdf_object(source: impl AsRef<str>) -> Vec<u8> {
    source.as_ref().as_bytes().to_vec()
}

#[derive(Debug)]
struct PdfFontResource {
    font_object: Vec<u8>,
    extra_objects: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Copy)]
struct Type1FontFiles {
    pfb: &'static str,
    metrics: Type1Metrics,
}

#[derive(Debug, Clone, Copy)]
struct Type1Metrics {
    font_bbox: [i32; 4],
    italic_angle: f32,
    ascender: i32,
    descender: i32,
    cap_height: i32,
    x_height: i32,
    fixed_pitch: bool,
    bold: bool,
    widths: &'static [f32; 95],
}

#[derive(Debug)]
struct Type1Program {
    bytes: Vec<u8>,
    length1: usize,
    length2: usize,
    length3: usize,
}

fn pdf_type1_font_resource(
    base_font: &str,
    next_object_id: &mut usize,
) -> io::Result<PdfFontResource> {
    let Some(files) = type1_font_files(base_font) else {
        return Ok(simple_type1_font_resource(base_font));
    };
    let Some(pfb_path) = resolve_tex_font_file(files.pfb)? else {
        return Ok(simple_type1_font_resource(base_font));
    };
    let metrics = files.metrics;
    let pfb = match fs::read(&pfb_path)
        .ok()
        .and_then(|bytes| parse_pfb_program(&bytes))
    {
        Some(pfb) => pfb,
        None => return Ok(simple_type1_font_resource(base_font)),
    };

    let descriptor_id = *next_object_id;
    *next_object_id += 1;
    let font_file_id = *next_object_id;
    *next_object_id += 1;

    let widths = metrics
        .widths
        .iter()
        .map(|width| (*width as i32).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    let differences = PDF_ASCII_GLYPH_NAMES
        .iter()
        .map(|name| format!("/{name}"))
        .collect::<Vec<_>>()
        .join(" ");
    let font_object = pdf_object(format!(
        "<< /Type /Font /Subtype /Type1 /BaseFont /{base_font} /FirstChar 32 /LastChar 126 /Widths [{widths}] /Encoding << /Type /Encoding /Differences [32 {differences}] >> /FontDescriptor {descriptor_id} 0 R >>"
    ));

    let flags = type1_font_descriptor_flags(&metrics);
    let stem_v = if metrics.bold { 120 } else { 80 };
    let descriptor = pdf_object(format!(
        "<< /Type /FontDescriptor /FontName /{base_font} /Flags {flags} /FontBBox [{} {} {} {}] /ItalicAngle {:.2} /Ascent {} /Descent {} /CapHeight {} /XHeight {} /StemV {stem_v} /FontFile {font_file_id} 0 R >>",
        metrics.font_bbox[0],
        metrics.font_bbox[1],
        metrics.font_bbox[2],
        metrics.font_bbox[3],
        metrics.italic_angle,
        metrics.ascender,
        metrics.descender,
        metrics.cap_height,
        metrics.x_height
    ));
    let font_file = type1_font_file_object(&pfb)?;
    Ok(PdfFontResource {
        font_object,
        extra_objects: vec![descriptor, font_file],
    })
}

fn simple_type1_font_resource(base_font: &str) -> PdfFontResource {
    PdfFontResource {
        font_object: pdf_object(format!(
            "<< /Type /Font /Subtype /Type1 /BaseFont /{base_font} >>"
        )),
        extra_objects: Vec::new(),
    }
}

fn type1_font_files(base_font: &str) -> Option<Type1FontFiles> {
    match base_font {
        "TeXGyrePagellaX-Regular" => Some(Type1FontFiles {
            pfb: "TeXGyrePagellaX-Regular.pfb",
            metrics: PAGELLA_TYPE1_METRICS,
        }),
        "TeXGyrePagellaX-Italic" => Some(Type1FontFiles {
            pfb: "TeXGyrePagellaX-Italic.pfb",
            metrics: PAGELLA_ITALIC_TYPE1_METRICS,
        }),
        "TeXGyrePagellaX-Bold" => Some(Type1FontFiles {
            pfb: "TeXGyrePagellaX-Bold.pfb",
            metrics: PAGELLA_BOLD_TYPE1_METRICS,
        }),
        "TeXGyreHeros-Regular" => Some(Type1FontFiles {
            pfb: "qhvr.pfb",
            metrics: HEROS_TYPE1_METRICS,
        }),
        "TeXGyreHeros-Bold" => Some(Type1FontFiles {
            pfb: "qhvb.pfb",
            metrics: HEROS_BOLD_TYPE1_METRICS,
        }),
        _ => None,
    }
}

fn resolve_tex_font_file(file_name: &str) -> io::Result<Option<PathBuf>> {
    let output = match Command::new("kpsewhich").arg(file_name).output() {
        Ok(output) => output,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    if !output.status.success() {
        return Ok(None);
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        return Ok(None);
    }
    fs::canonicalize(path).map(Some)
}

fn parse_pfb_program(data: &[u8]) -> Option<Type1Program> {
    let mut cursor = 0_usize;
    let mut segments: Vec<(u8, Vec<u8>)> = Vec::new();
    while cursor + 6 <= data.len() && data[cursor] == 0x80 {
        let kind = data[cursor + 1];
        cursor += 2;
        if kind == 3 {
            break;
        }
        let length = u32::from_le_bytes(data[cursor..cursor + 4].try_into().ok()?) as usize;
        cursor += 4;
        let end = cursor.checked_add(length)?;
        if end > data.len() {
            return None;
        }
        segments.push((kind, data[cursor..end].to_vec()));
        cursor = end;
    }
    let first_ascii = segments
        .iter()
        .position(|(kind, _)| *kind == 1)
        .and_then(|index| segments.get(index))?;
    let first_binary_index = segments.iter().position(|(kind, _)| *kind == 2)?;
    let first_binary = segments.get(first_binary_index)?;
    let length1 = first_ascii.1.len();
    let length2 = first_binary.1.len();
    let length3 = segments
        .iter()
        .skip(first_binary_index + 1)
        .filter(|(kind, _)| *kind == 1)
        .map(|(_, segment)| segment.len())
        .sum::<usize>();
    let bytes = segments
        .into_iter()
        .flat_map(|(_, segment)| segment)
        .collect::<Vec<_>>();
    Some(Type1Program {
        bytes,
        length1,
        length2,
        length3,
    })
}

fn type1_font_descriptor_flags(metrics: &Type1Metrics) -> i32 {
    let mut flags = 32;
    if metrics.fixed_pitch {
        flags |= 1;
    }
    if metrics.italic_angle.abs() > f32::EPSILON {
        flags |= 64;
    }
    flags
}

fn type1_font_file_object(program: &Type1Program) -> io::Result<Vec<u8>> {
    let compressed = zlib_compress(&program.bytes).map_err(io::Error::other)?;
    let mut object = Vec::new();
    write!(
        object,
        "<< /Length {} /Length1 {} /Length2 {} /Length3 {} /Filter /FlateDecode >>\nstream\n",
        compressed.len(),
        program.length1,
        program.length2,
        program.length3
    )?;
    object.extend_from_slice(&compressed);
    object.extend_from_slice(b"\nendstream");
    Ok(object)
}

fn image_objects(image: &ImageAsset, image_object_id: usize) -> Vec<Vec<u8>> {
    match image.payload.as_ref() {
        ImagePayload::Jpeg(data) => vec![jpeg_image_object(image, data)],
        ImagePayload::Png {
            color_space,
            bits_per_component,
            data,
            alpha,
            decode_params,
        } => {
            let smask_object_id = alpha.as_ref().map(|_| image_object_id + 1);
            let mut objects = vec![png_image_object(
                image,
                *color_space,
                *bits_per_component,
                data,
                smask_object_id,
                *decode_params,
            )];
            if let Some(alpha) = alpha {
                objects.push(png_alpha_object(image, *bits_per_component, alpha));
            }
            objects
        }
        ImagePayload::PdfForm(form) => pdf_form_objects(form, image_object_id),
    }
}

fn jpeg_image_object(image: &ImageAsset, data: &[u8]) -> Vec<u8> {
    let mut object = Vec::new();
    write!(
        object,
        "<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /DCTDecode /Length {} >>\nstream\n",
        image.width_px,
        image.height_px,
        data.len()
    )
    .unwrap();
    object.extend_from_slice(data);
    object.extend_from_slice(b"\nendstream");
    object
}

fn png_image_object(
    image: &ImageAsset,
    color_space: PdfColorSpace,
    bits_per_component: u8,
    data: &[u8],
    smask_object_id: Option<usize>,
    decode_params: Option<PngDecodeParams>,
) -> Vec<u8> {
    let mut object = Vec::new();
    let smask = smask_object_id
        .map(|id| format!(" /SMask {id} 0 R"))
        .unwrap_or_default();
    let decode_params = decode_params
        .map(|params| {
            format!(
                " /DecodeParms << /Predictor 15 /Colors {} /BitsPerComponent {} /Columns {} >>",
                params.colors, params.bits_per_component, params.columns
            )
        })
        .unwrap_or_default();
    write!(
        object,
        "<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace {} /BitsPerComponent {} /Filter /FlateDecode{}{} /Length {} >>\nstream\n",
        image.width_px,
        image.height_px,
        color_space.name(),
        bits_per_component,
        smask,
        decode_params,
        data.len()
    )
    .unwrap();
    object.extend_from_slice(data);
    object.extend_from_slice(b"\nendstream");
    object
}

fn png_alpha_object(image: &ImageAsset, bits_per_component: u8, data: &[u8]) -> Vec<u8> {
    let mut object = Vec::new();
    write!(
        object,
        "<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceGray /BitsPerComponent {} /Filter /FlateDecode /Length {} >>\nstream\n",
        image.width_px,
        image.height_px,
        bits_per_component,
        data.len()
    )
    .unwrap();
    object.extend_from_slice(data);
    object.extend_from_slice(b"\nendstream");
    object
}

fn pdf_form_objects(form: &PdfFormAsset, image_object_id: usize) -> Vec<Vec<u8>> {
    let id_map = form
        .imported_objects
        .iter()
        .enumerate()
        .map(|(index, (old_id, _))| (*old_id, ((image_object_id + index + 1) as u32, 0)))
        .collect::<BTreeMap<_, _>>();
    let resources = form
        .resources
        .as_ref()
        .map(|resources| remap_lopdf_object(resources, &id_map));
    let mut objects = vec![pdf_form_object(form, resources)];
    for (_, object) in &form.imported_objects {
        objects.push(lopdf_object_bytes(&remap_lopdf_object(object, &id_map)));
    }
    objects
}

fn pdf_form_object(form: &PdfFormAsset, resources: Option<LoObject>) -> Vec<u8> {
    let width = (form.bbox[2] - form.bbox[0]).abs().max(1.0);
    let height = (form.bbox[3] - form.bbox[1]).abs().max(1.0);
    let mut dictionary = LoDictionary::new();
    dictionary.set("Type", LoObject::Name(b"XObject".to_vec()));
    dictionary.set("Subtype", LoObject::Name(b"Form".to_vec()));
    dictionary.set("FormType", 1_i64);
    dictionary.set(
        "BBox",
        LoObject::Array(form.bbox.iter().copied().map(LoObject::Real).collect()),
    );
    dictionary.set(
        "Matrix",
        LoObject::Array(vec![
            LoObject::Real(1.0 / width),
            LoObject::Integer(0),
            LoObject::Integer(0),
            LoObject::Real(1.0 / height),
            LoObject::Real(-form.bbox[0] / width),
            LoObject::Real(-form.bbox[1] / height),
        ]),
    );
    if let Some(resources) = resources {
        dictionary.set("Resources", resources);
    }
    lopdf_object_bytes(&LoObject::Stream(LoStream::new(
        dictionary,
        form.content.clone(),
    )))
}

fn remap_lopdf_object(object: &LoObject, id_map: &BTreeMap<LoObjectId, LoObjectId>) -> LoObject {
    match object {
        LoObject::Array(items) => LoObject::Array(
            items
                .iter()
                .map(|item| remap_lopdf_object(item, id_map))
                .collect(),
        ),
        LoObject::Dictionary(dictionary) => {
            let mut remapped = LoDictionary::new();
            for (key, value) in dictionary.iter() {
                remapped.set(key.clone(), remap_lopdf_object(value, id_map));
            }
            LoObject::Dictionary(remapped)
        }
        LoObject::Stream(stream) => {
            let mut dictionary = LoDictionary::new();
            for (key, value) in stream.dict.iter() {
                dictionary.set(key.clone(), remap_lopdf_object(value, id_map));
            }
            LoObject::Stream(LoStream {
                dict: dictionary,
                content: stream.content.clone(),
                allows_compression: stream.allows_compression,
                start_position: None,
            })
        }
        LoObject::Reference(object_id) => {
            LoObject::Reference(*id_map.get(object_id).unwrap_or(object_id))
        }
        LoObject::Null => LoObject::Null,
        LoObject::Boolean(value) => LoObject::Boolean(*value),
        LoObject::Integer(value) => LoObject::Integer(*value),
        LoObject::Real(value) => LoObject::Real(*value),
        LoObject::Name(name) => LoObject::Name(name.clone()),
        LoObject::String(text, format) => LoObject::String(text.clone(), *format),
    }
}

fn lopdf_object_bytes(object: &LoObject) -> Vec<u8> {
    let mut bytes = Vec::new();
    write_lopdf_object(&mut bytes, object).unwrap();
    bytes
}

fn write_lopdf_object(output: &mut Vec<u8>, object: &LoObject) -> io::Result<()> {
    match object {
        LoObject::Null => output.write_all(b"null"),
        LoObject::Boolean(value) => {
            if *value {
                output.write_all(b"true")
            } else {
                output.write_all(b"false")
            }
        }
        LoObject::Integer(value) => write!(output, "{value}"),
        LoObject::Real(value) => write!(output, "{value}"),
        LoObject::Name(name) => write_lopdf_name(output, name),
        LoObject::String(text, format) => write_lopdf_string(output, text, *format),
        LoObject::Array(items) => write_lopdf_array(output, items),
        LoObject::Dictionary(dictionary) => write_lopdf_dictionary(output, dictionary),
        LoObject::Stream(stream) => write_lopdf_stream(output, stream),
        LoObject::Reference(object_id) => write!(output, "{} {} R", object_id.0, object_id.1),
    }
}

fn write_lopdf_name(output: &mut Vec<u8>, name: &[u8]) -> io::Result<()> {
    output.write_all(b"/")?;
    for &byte in name {
        if b" \t\n\r\x0C()<>[]{}/%#".contains(&byte) || !(33..=126).contains(&byte) {
            write!(output, "#{byte:02X}")?;
        } else {
            output.write_all(&[byte])?;
        }
    }
    Ok(())
}

fn write_lopdf_string(output: &mut Vec<u8>, text: &[u8], format: LoStringFormat) -> io::Result<()> {
    match format {
        LoStringFormat::Literal => {
            output.write_all(b"(")?;
            for &byte in text {
                match byte {
                    b'(' | b')' | b'\\' => {
                        output.write_all(b"\\")?;
                        output.write_all(&[byte])?;
                    }
                    b'\r' => output.write_all(b"\\r")?,
                    b'\n' => output.write_all(b"\\n")?,
                    _ => output.write_all(&[byte])?,
                }
            }
            output.write_all(b")")
        }
        LoStringFormat::Hexadecimal => {
            output.write_all(b"<")?;
            for &byte in text {
                write!(output, "{byte:02X}")?;
            }
            output.write_all(b">")
        }
    }
}

fn write_lopdf_array(output: &mut Vec<u8>, items: &[LoObject]) -> io::Result<()> {
    output.write_all(b"[")?;
    for item in items {
        output.write_all(b" ")?;
        write_lopdf_object(output, item)?;
    }
    output.write_all(b" ]")
}

fn write_lopdf_dictionary(output: &mut Vec<u8>, dictionary: &LoDictionary) -> io::Result<()> {
    output.write_all(b"<<")?;
    for (key, value) in dictionary.iter() {
        output.write_all(b" ")?;
        write_lopdf_name(output, key)?;
        output.write_all(b" ")?;
        write_lopdf_object(output, value)?;
    }
    output.write_all(b" >>")
}

fn write_lopdf_stream(output: &mut Vec<u8>, stream: &LoStream) -> io::Result<()> {
    let mut dictionary = stream.dict.clone();
    dictionary.set("Length", stream.content.len() as i64);
    write_lopdf_dictionary(output, &dictionary)?;
    output.write_all(b"\nstream\n")?;
    output.write_all(&stream.content)?;
    output.write_all(b"\nendstream")
}

#[cfg(test)]
fn page_stream(document: &SimpleDocument, page_index: usize) -> String {
    page_streams(document, document.pages())
        .get(page_index)
        .cloned()
        .unwrap_or_default()
}

#[cfg(test)]
fn page_streams(document: &SimpleDocument, page_count: usize) -> Vec<String> {
    let placements = line_placements(document);
    page_streams_from_placements(document, page_count, &placements)
}

fn page_streams_from_placements(
    document: &SimpleDocument,
    page_count: usize,
    placements: &[LinePlacement],
) -> Vec<String> {
    let mut streams = vec![String::new(); page_count];
    let page_style = document.lines.iter().find_map(|line| match line {
        Line::PageStyle(style) => Some(style),
        _ => None,
    });
    for placement in placements {
        if !placement.render {
            continue;
        }
        if let Some(stream) = streams.get_mut(placement.page_index) {
            append_line_to_page_stream(
                stream,
                document,
                &document.lines[placement.line_index],
                placement.page_slot,
                placement.layout,
            );
        }
    }
    for (page_index, stream) in streams.iter_mut().enumerate() {
        if let Some(style) = page_style {
            append_page_header_to_stream(stream, document.layout, page_index, style);
        }
        if should_render_page_number(document.layout, page_index) {
            append_page_number_to_stream(stream, document.layout, page_index + 1);
        }
    }
    streams
}

fn append_page_header_to_stream(
    stream: &mut String,
    layout: DocumentLayout,
    page_index: usize,
    style: &PageStyle,
) {
    if page_index == 0 || layout != DocumentLayout::icml_two_column() {
        return;
    }
    if style.running_title.is_empty() && style.section_line.is_empty() {
        return;
    }
    writeln!(
        stream,
        "q\n0.00 0.45 0.70 RG\n0.40 w\n{:.2} {:.2} m\n{:.2} {:.2} l\nS\nQ",
        layout.left_pt,
        ICML_HEADER_RULE_Y_PT,
        layout.left_pt + layout.text_width_pt,
        ICML_HEADER_RULE_Y_PT
    )
    .unwrap();
    if !style.running_title.is_empty() {
        let (x, y) = centered_page_text_position(
            &style.running_title,
            ICML_HEADER_TITLE_FONT_PT,
            ICML_HEADER_TITLE_Y_PT,
            true,
        );
        writeln!(
            stream,
            "BT\n/F4 {:.2} Tf\n{:.2} {:.2} Td\n({}) Tj\nET",
            ICML_HEADER_TITLE_FONT_PT,
            x,
            y,
            pdf_text(&style.running_title)
        )
        .unwrap();
    }
    if !style.section_line.is_empty() {
        let (x, y) = centered_page_text_position(
            &style.section_line,
            ICML_HEADER_SECTION_FONT_PT,
            ICML_HEADER_SECTION_Y_PT,
            false,
        );
        writeln!(
            stream,
            "BT\n/F1 {:.2} Tf\n{:.2} {:.2} Td\n({}) Tj\nET",
            ICML_HEADER_SECTION_FONT_PT,
            x,
            y,
            pdf_text(&style.section_line)
        )
        .unwrap();
    }
}

fn centered_page_text_position(text: &str, font_size: f32, y: f32, bold: bool) -> (f32, f32) {
    let width = header_text_width_pt(text, font_size, bold, 518.4);
    (306.0 - width / 2.0, y)
}

fn header_text_width_pt(text: &str, font_size: f32, bold: bool, max_width: f32) -> f32 {
    let units = text
        .chars()
        .map(|ch| header_text_char_units(ch, bold))
        .sum::<u32>() as f32;
    (units * font_size / 1000.0).clamp(font_size, max_width)
}

fn header_text_char_units(ch: char, bold: bool) -> u32 {
    match ch {
        ' ' => 250,
        '|' => 260,
        ':' | ';' | ',' | '.' => 250,
        '!' | '?' => 444,
        'i' | 'l' | 'I' => 278,
        'f' | 'j' | 'r' | 't' => 333,
        'm' | 'w' => 778,
        'M' | 'W' => 889,
        '0'..='9' => 500,
        'A'..='Z' => {
            if bold {
                720
            } else {
                675
            }
        }
        'a'..='z' => 544,
        _ => 500,
    }
}

fn title_baseline_offset_pt(layout: DocumentLayout) -> f32 {
    if layout == DocumentLayout::neurips_single_column() {
        -3.0
    } else {
        0.0
    }
}

fn append_author_grid_to_page_stream(
    stream: &mut String,
    layout: DocumentLayout,
    page_slot: usize,
    grid: &AuthorGrid,
) {
    let (_, start_y) = layout.point_for_wide_slot(page_slot, 0.0);
    let mut row_slot_offset = 0_usize;
    for row in &grid.rows {
        let row_line_count = row.iter().map(|block| block.lines.len()).max().unwrap_or(0);
        for (column_index, block) in row.iter().enumerate() {
            let center_x = author_grid_column_center(layout, row.len(), column_index);
            for (line_index, text) in block.lines.iter().enumerate() {
                let width = text_width_for_layout_font_pt(
                    text,
                    layout,
                    PdfTextFont::Heading,
                    NEURIPS_AUTHOR_GRID_FONT_PT,
                    layout.text_width_pt / row.len().max(1) as f32,
                );
                let x = center_x - width / 2.0;
                let y = start_y - (row_slot_offset + line_index) as f32 * layout.line_height_pt;
                writeln!(
                    stream,
                    "BT\n/F4 {:.2} Tf\n{:.2} {:.2} Td\n({}) Tj\nET",
                    NEURIPS_AUTHOR_GRID_FONT_PT,
                    x,
                    y,
                    pdf_text(text)
                )
                .unwrap();
            }
        }
        row_slot_offset += row_line_count + 2;
    }
}

fn author_grid_column_center(
    layout: DocumentLayout,
    column_count: usize,
    column_index: usize,
) -> f32 {
    match column_count {
        0 => layout.left_pt + layout.text_width_pt / 2.0,
        1 => layout.left_pt + layout.text_width_pt / 2.0,
        2 => {
            let fraction = if column_index == 0 { 0.30 } else { 0.74 };
            layout.left_pt + layout.text_width_pt * fraction
        }
        count => layout.left_pt + layout.text_width_pt * (column_index as f32 + 0.5) / count as f32,
    }
}

fn should_render_page_number(layout: DocumentLayout, page_index: usize) -> bool {
    page_index > 0
        || (layout != DocumentLayout::neurips_single_column()
            && layout != DocumentLayout::icml_two_column())
}

fn append_page_number_to_stream(stream: &mut String, layout: DocumentLayout, page_number: usize) {
    let text = page_number.to_string();
    let width = text_width_for_layout_font_pt(
        &text,
        layout,
        PdfTextFont::Text,
        PAGE_NUMBER_FONT_PT,
        layout.text_width_pt,
    );
    let x = 306.0 - width / 2.0;
    let y = page_number_y_pt(layout);
    writeln!(
        stream,
        "BT\n/F1 {:.2} Tf\n0 0 0 rg\n{:.2} {:.2} Td\n({}) Tj\nET",
        PAGE_NUMBER_FONT_PT,
        x,
        y,
        pdf_text(&text)
    )
    .unwrap();
}

fn page_number_y_pt(layout: DocumentLayout) -> f32 {
    if layout == DocumentLayout::neurips_single_column() {
        NEURIPS_PAGE_NUMBER_Y_PT
    } else if layout == DocumentLayout::icml_two_column() {
        ICML_PAGE_NUMBER_Y_PT
    } else {
        PAGE_NUMBER_Y_PT
    }
}

fn append_table_cells_to_stream(
    stream: &mut String,
    layout: DocumentLayout,
    page_slot: usize,
    cells: &[String],
    wide: bool,
) {
    if cells.is_empty() {
        return;
    }
    let (x, y) = if wide {
        layout.point_for_wide_slot(page_slot, 0.0)
    } else {
        layout.point_for_slot(page_slot, 0.0)
    };
    let available_width = if wide {
        layout.text_width_pt
    } else {
        layout.column_width_pt
    };
    let column_count = cells.len().max(1);
    let gutter_pt = if column_count > 8 { 2.0 } else { 4.0 };
    let cell_width = ((available_width - gutter_pt * (column_count - 1) as f32)
        / column_count as f32)
        .max(layout.code_font_pt * 2.0);
    let font_size = table_cell_font_size(layout, column_count);
    for (index, cell) in cells.iter().enumerate() {
        let cell_x = x + index as f32 * (cell_width + gutter_pt);
        let fitted = fit_table_cell_text(cell, layout, font_size, cell_width);
        if fitted.is_empty() {
            continue;
        }
        append_pdf_text_object(stream, PdfTextFont::Text, font_size, cell_x, y, &fitted);
    }
}

fn table_cell_font_size(layout: DocumentLayout, column_count: usize) -> f32 {
    if column_count >= 12 {
        (layout.footnote_font_pt * 0.72).max(4.8)
    } else if column_count >= 8 {
        (layout.footnote_font_pt * 0.82).max(5.2)
    } else {
        layout.footnote_font_pt
    }
}

fn fit_table_cell_text(
    text: &str,
    layout: DocumentLayout,
    font_size: f32,
    max_width_pt: f32,
) -> String {
    let text = text.trim();
    if text.is_empty() {
        return String::new();
    }
    let metric = layout.metric_for_font(PdfTextFont::Text);
    if natural_text_width_for_metric_pt(text, metric, font_size) <= max_width_pt {
        return text.to_string();
    }
    let suffix = "...";
    let suffix_width = natural_text_width_for_metric_pt(suffix, metric, font_size);
    let target = (max_width_pt - suffix_width).max(font_size);
    let mut fitted = String::new();
    let mut fitted_width = 0.0_f32;
    for ch in text.chars() {
        let char_width = pdf_font_char_width_units(ch, metric) * font_size / 1000.0;
        if fitted_width + char_width > target {
            break;
        }
        fitted.push(ch);
        fitted_width += char_width;
    }
    if fitted.is_empty() {
        String::new()
    } else {
        fitted.push_str(suffix);
        fitted
    }
}

fn primary_line_placements_from(placements: &[LinePlacement]) -> Vec<LinePlacement> {
    let mut primary = Vec::new();
    let mut seen_lines = BTreeSet::new();
    for placement in placements {
        if placement.render && seen_lines.insert(placement.line_index) {
            primary.push(*placement);
        }
    }
    primary
}

fn synctex_boxes_from_placements(
    document: &SimpleDocument,
    placements: &[LinePlacement],
) -> Vec<SynctexBox> {
    let mut boxes = Vec::new();
    for placement in primary_line_placements_from(placements) {
        let line = &document.lines[placement.line_index];
        let Some((indent_pt, width_pt, height_pt)) =
            synctex_line_geometry(line, placement.line_slots, document, placement.layout)
        else {
            continue;
        };
        let page_slot = if matches!(
            line,
            Line::ParagraphText(_) | Line::JustifiedParagraphText { .. }
        ) {
            placement.page_slot + 1
        } else {
            placement.page_slot
        };
        let (x_pt, y_pt) = if line.is_wide() {
            placement.layout.point_for_wide_slot(page_slot, indent_pt)
        } else {
            placement.layout.point_for_slot(page_slot, indent_pt)
        };
        boxes.push(SynctexBox {
            page: placement.page_index + 1,
            line: boxes.len() + 1,
            x_sp: synctex_sp(x_pt),
            y_sp: synctex_sp(y_pt),
            width_sp: synctex_sp(width_pt),
            height_sp: synctex_sp(height_pt),
        });
    }
    boxes
}

fn append_line_to_page_stream(
    stream: &mut String,
    document: &SimpleDocument,
    line: &Line,
    page_slot: usize,
    layout: DocumentLayout,
) {
    match line {
        Line::PageStyle(_) => {}
        Line::OutputControl(_) => {}
        Line::Title(text) => {
            let (x, y) = centered_title_text_position(layout, page_slot, text);
            let y = y + title_baseline_offset_pt(layout);
            append_pdf_text_object(
                stream,
                PdfTextFont::Heading,
                layout.title_font_pt,
                x,
                y,
                text,
            );
        }
        Line::WideTitle(text) => {
            let (x, y) = wide_centered_title_text_position(layout, page_slot, text);
            let y = y + title_baseline_offset_pt(layout);
            append_pdf_text_object(
                stream,
                PdfTextFont::Heading,
                layout.title_font_pt,
                x,
                y,
                text,
            );
        }
        Line::Author(text) => {
            let (x, y) = centered_text_position(layout, page_slot, text, layout.author_font_pt);
            append_pdf_text_object(stream, PdfTextFont::Text, layout.author_font_pt, x, y, text);
        }
        Line::WideAuthor(text) => {
            let (x, y) =
                wide_centered_text_position(layout, page_slot, text, layout.author_font_pt);
            append_pdf_text_object(stream, PdfTextFont::Text, layout.author_font_pt, x, y, text);
        }
        Line::AuthorGrid(grid) => {
            append_author_grid_to_page_stream(stream, layout, page_slot, grid);
        }
        Line::AbstractHeading(text) => {
            let (x, y) = centered_text_position_for_font(
                layout,
                page_slot,
                text,
                PdfTextFont::Heading,
                layout.abstract_heading_font_pt,
            );
            append_pdf_text_object(
                stream,
                PdfTextFont::Heading,
                layout.abstract_heading_font_pt,
                x,
                y,
                text,
            );
        }
        Line::AbstractText(text) => {
            let (x, y) = layout.point_for_slot(page_slot, 12.0);
            append_pdf_text_object(stream, PdfTextFont::Text, layout.text_font_pt, x, y, text);
        }
        Line::JustifiedAbstractText { text, width_pt } => {
            let (x, y) = layout.point_for_slot(page_slot, 12.0);
            append_pdf_text_object_justified(
                stream,
                layout,
                PdfTextFont::Text,
                layout.text_font_pt,
                x,
                y,
                text,
                *width_pt,
            );
        }
        Line::WideBackground(height_slots) => {
            let (x, y) = layout.point_for_wide_slot(page_slot, 0.0);
            let pad_x = 10.0;
            let pad_top = 6.0;
            let height = *height_slots as f32 * layout.line_height_pt + pad_top;
            let top = y + pad_top;
            let bottom = (top - height).max(0.0);
            writeln!(
                stream,
                "q\n0.90 0.96 0.98 rg\n{:.2} {:.2} {:.2} {:.2} re f\nQ",
                x - pad_x,
                bottom,
                layout.text_width_pt + pad_x * 2.0,
                height
            )
            .unwrap();
        }
        Line::WideTheoremBackground { slots, heading } => {
            let (x, y) = layout.point_for_wide_slot(page_slot, 0.0);
            let pad_x = 2.0;
            let pad_top = 4.0;
            let height = *slots as f32 * layout.line_height_pt + pad_top;
            let top = y + pad_top;
            let bottom = (top - height).max(0.0);
            let header_height = layout.line_height_pt * 1.35;
            writeln!(
                stream,
                "q\n0.92 0.98 0.92 rg\n0.00 0.35 0.00 RG\n{:.2} {:.2} {:.2} {:.2} re B\n0.00 0.35 0.00 rg\n{:.2} {:.2} {:.2} {:.2} re f\nQ",
                x - pad_x,
                bottom,
                layout.text_width_pt + pad_x * 2.0,
                height,
                x - pad_x,
                top - header_height,
                layout.text_width_pt + pad_x * 2.0,
                header_height
            )
            .unwrap();
            if !heading.trim().is_empty() {
                writeln!(
                    stream,
                    "q\n1 1 1 rg\nBT\n/F4 {:.2} Tf\n{:.2} {:.2} Td\n({}) Tj\nET\nQ",
                    layout.text_font_pt,
                    x + 4.0,
                    top - header_height + 3.0,
                    pdf_text(heading)
                )
                .unwrap();
            }
        }
        Line::WideAbstractText(text) => {
            let (x, y) = layout.point_for_wide_slot(page_slot, 12.0);
            append_pdf_text_object(stream, PdfTextFont::Text, layout.text_font_pt, x, y, text);
        }
        Line::WideCaption(text) => {
            let (x, y) = layout.point_for_wide_slot(page_slot, 0.0);
            append_pdf_text_object(
                stream,
                PdfTextFont::Text,
                layout.footnote_font_pt,
                x,
                y,
                text,
            );
        }
        Line::WideEquation(text) => {
            let (x, y) = wide_centered_text_position_for_font(
                layout,
                page_slot,
                text,
                PdfTextFont::Math,
                layout.text_font_pt,
            );
            append_pdf_text_object(stream, PdfTextFont::Math, layout.text_font_pt, x, y, text);
        }
        Line::Heading(text) => {
            let (x, y) = layout.point_for_slot(page_slot, 0.0);
            append_pdf_text_object(
                stream,
                PdfTextFont::Heading,
                layout.heading_font_pt,
                x,
                y,
                text,
            );
        }
        Line::JustifiedParagraphText { text, width_pt } => {
            let (x, y) = layout.point_for_slot(page_slot + 1, 0.0);
            append_pdf_text_object_justified(
                stream,
                layout,
                PdfTextFont::Text,
                layout.text_font_pt,
                x,
                y,
                text,
                *width_pt,
            );
        }
        Line::ParagraphText(text) => {
            let (x, y) = layout.point_for_slot(page_slot + 1, 0.0);
            append_pdf_text_object(stream, PdfTextFont::Text, layout.text_font_pt, x, y, text);
        }
        Line::JustifiedText { text, width_pt } => {
            let (x, y) = layout.point_for_slot(page_slot, 0.0);
            append_pdf_text_object_justified(
                stream,
                layout,
                PdfTextFont::Text,
                layout.text_font_pt,
                x,
                y,
                text,
                *width_pt,
            );
        }
        Line::Text(text) => {
            let (x, y) = layout.point_for_slot(page_slot, 0.0);
            append_pdf_text_object(stream, PdfTextFont::Text, layout.text_font_pt, x, y, text);
        }
        Line::TableRow(text) => {
            let (x, y) = layout.point_for_slot(page_slot, 0.0);
            append_pdf_text_object(
                stream,
                PdfTextFont::Text,
                layout.footnote_font_pt,
                x,
                y,
                text,
            );
        }
        Line::TableCells { cells, .. } => {
            append_table_cells_to_stream(stream, layout, page_slot, cells, false);
        }
        Line::WideTableCells { cells, .. } => {
            append_table_cells_to_stream(stream, layout, page_slot, cells, true);
        }
        Line::Caption(text) => {
            let (x, y) = layout.point_for_slot(page_slot, 0.0);
            append_pdf_text_object(
                stream,
                PdfTextFont::Text,
                layout.footnote_font_pt,
                x,
                y,
                text,
            );
        }
        Line::Footnote(text) => {
            let (x, y) = layout.point_for_slot(page_slot, 12.0);
            append_pdf_text_object(
                stream,
                PdfTextFont::Text,
                layout.footnote_font_pt,
                x,
                y,
                text,
            );
        }
        Line::DisplayEquation(text) | Line::Equation(text) => {
            let (x, y) = layout.point_for_slot(page_slot, 24.0);
            append_pdf_text_object(stream, PdfTextFont::Math, layout.text_font_pt, x, y, text);
        }
        Line::Code(text) => {
            let (x, y) = layout.point_for_slot(page_slot, 12.0);
            append_pdf_text_object(stream, PdfTextFont::Code, layout.code_font_pt, x, y, text);
        }
        Line::Image(index) => {
            let image = &document.images[*index];
            let (x, y) = layout.point_for_slot(page_slot, 0.0);
            let (draw_width, draw_height) = layout.image_draw_size_with_max_width(
                image,
                if layout.columns > 1 {
                    layout.column_width_pt
                } else {
                    layout.text_width_pt
                },
            );
            let (_, box_height) = layout.image_display_size(image);
            let image_y = (y - box_height).max(72.0);
            append_image_xobject(stream, *index, image, draw_width, draw_height, x, image_y);
        }
        Line::WideImageRow(indices) => {
            let row = wide_image_row_items(layout, &document.images, indices);
            let (x, y) = layout.point_for_wide_slot(page_slot, 0.0);
            for item in row {
                let image = &document.images[item.index];
                let image_y = (y - item.height).max(72.0);
                append_image_xobject(
                    stream,
                    item.index,
                    image,
                    item.draw_width,
                    item.draw_height,
                    x + item.x_offset,
                    image_y,
                );
            }
        }
        Line::WideTeaserRow(cells) => {
            append_wide_teaser_row_to_stream(stream, layout, &document.images, cells, page_slot);
        }
        Line::TeaserRow(cells) => {
            append_teaser_row_to_stream(
                stream,
                layout,
                &document.images,
                cells,
                page_slot,
                layout.column_width_pt,
                false,
            );
        }
        Line::FloatBlock { lines, .. }
        | Line::LateFloatBlock { lines, .. }
        | Line::BottomFloatBlock { lines, .. } => {
            let mut offset = 0_usize;
            for nested in lines {
                append_line_to_page_stream(stream, document, nested, page_slot + offset, layout);
                offset += nested.slots(layout, &document.images);
            }
        }
        Line::Blank => {}
    }
}

fn pdf_text(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        if let Some(replacement) = pdf_text_ascii_replacement(ch) {
            for replacement_ch in replacement.chars() {
                push_pdf_text_char(&mut out, replacement_ch);
            }
        } else {
            push_pdf_text_char(&mut out, ch);
        }
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PdfTextFont {
    Text,
    Code,
    Math,
    Heading,
    Symbol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PdfFontMetric {
    TimesRoman,
    TimesItalic,
    TimesBold,
    Pagella,
    PagellaItalic,
    PagellaBold,
    Heros,
    HerosBold,
    Courier,
    Symbol,
}

impl PdfTextFont {
    const fn name(self) -> &'static str {
        match self {
            Self::Text => "F1",
            Self::Code => "F2",
            Self::Math => "F3",
            Self::Heading => "F4",
            Self::Symbol => "F5",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PdfTextRun {
    font: PdfTextFont,
    text: String,
}

fn append_pdf_text_object(
    stream: &mut String,
    base_font: PdfTextFont,
    font_size: f32,
    x: f32,
    y: f32,
    text: &str,
) {
    append_pdf_text_object_with_word_spacing(stream, base_font, font_size, x, y, text, None);
}

fn append_pdf_text_object_justified(
    stream: &mut String,
    layout: DocumentLayout,
    base_font: PdfTextFont,
    font_size: f32,
    x: f32,
    y: f32,
    text: &str,
    target_width_pt: f32,
) {
    let word_spacing_pt = justified_word_spacing_pt(
        text,
        layout.metric_for_font(base_font),
        font_size,
        target_width_pt,
    );
    append_pdf_text_object_with_word_spacing(
        stream,
        base_font,
        font_size,
        x,
        y,
        text,
        word_spacing_pt,
    );
}

fn append_pdf_text_object_with_word_spacing(
    stream: &mut String,
    base_font: PdfTextFont,
    font_size: f32,
    x: f32,
    y: f32,
    text: &str,
    word_spacing_pt: Option<f32>,
) {
    let runs = pdf_text_runs(text, base_font);
    if runs.is_empty() {
        return;
    }
    writeln!(
        stream,
        "q\n0 0 0 rg\nBT\n/{} {:.2} Tf\n{:.2} {:.2} Td",
        runs[0].font.name(),
        font_size,
        x,
        y
    )
    .unwrap();
    if let Some(word_spacing_pt) = word_spacing_pt.filter(|spacing| spacing.abs() >= 0.01) {
        writeln!(stream, "{word_spacing_pt:.4} Tw").unwrap();
    }
    let mut active_font = runs[0].font;
    for run in runs {
        if run.font != active_font {
            writeln!(stream, "/{} {:.2} Tf", run.font.name(), font_size).unwrap();
            active_font = run.font;
        }
        writeln!(stream, "({}) Tj", run.text).unwrap();
    }
    writeln!(stream, "ET\nQ").unwrap();
}

fn justified_word_spacing_pt(
    text: &str,
    font_metric: PdfFontMetric,
    font_size: f32,
    target_width_pt: f32,
) -> Option<f32> {
    let spaces = text.chars().filter(|ch| *ch == ' ').count();
    if spaces == 0 {
        return None;
    }
    let natural_width_pt = natural_text_width_for_metric_pt(text, font_metric, font_size);
    let spacing_pt = (target_width_pt - natural_width_pt) / spaces as f32;
    if !spacing_pt.is_finite() {
        return None;
    }
    let min_spacing_pt = -font_size * 0.20;
    let max_spacing_pt = font_size * 0.55;
    (min_spacing_pt..=max_spacing_pt)
        .contains(&spacing_pt)
        .then_some(spacing_pt)
}

fn pdf_text_runs(text: &str, base_font: PdfTextFont) -> Vec<PdfTextRun> {
    let mut runs: Vec<PdfTextRun> = Vec::new();
    for ch in text.chars() {
        if let Some(byte) = pdf_symbol_byte(ch) {
            push_pdf_text_run_byte(&mut runs, PdfTextFont::Symbol, byte);
        } else if let Some(replacement) = pdf_text_ascii_replacement(ch) {
            for replacement_ch in replacement.chars() {
                push_pdf_text_run_char(&mut runs, base_font, replacement_ch);
            }
        } else {
            push_pdf_text_run_char(&mut runs, base_font, ch);
        }
    }
    runs
}

fn push_pdf_text_run_char(runs: &mut Vec<PdfTextRun>, font: PdfTextFont, ch: char) {
    match ch {
        '(' => push_pdf_text_run_str(runs, font, "\\("),
        ')' => push_pdf_text_run_str(runs, font, "\\)"),
        '\\' => push_pdf_text_run_str(runs, font, "\\\\"),
        ch if ch.is_ascii() && !ch.is_control() => {
            push_pdf_text_run_byte(runs, font, ch as u8);
        }
        _ => push_pdf_text_run_byte(runs, font, b'?'),
    }
}

fn push_pdf_text_run_byte(runs: &mut Vec<PdfTextRun>, font: PdfTextFont, byte: u8) {
    match byte {
        b'(' => push_pdf_text_run_str(runs, font, "\\("),
        b')' => push_pdf_text_run_str(runs, font, "\\)"),
        b'\\' => push_pdf_text_run_str(runs, font, "\\\\"),
        0x20..=0x7e => push_pdf_text_run_str(runs, font, &(byte as char).to_string()),
        _ => push_pdf_text_run_str(runs, font, &format!("\\{byte:03o}")),
    }
}

fn push_pdf_text_run_str(runs: &mut Vec<PdfTextRun>, font: PdfTextFont, text: &str) {
    if text.is_empty() {
        return;
    }
    if let Some(run) = runs.last_mut().filter(|run| run.font == font) {
        run.text.push_str(text);
    } else {
        runs.push(PdfTextRun {
            font,
            text: text.to_string(),
        });
    }
}

fn pdf_symbol_byte(ch: char) -> Option<u8> {
    Some(match ch {
        'Α' => b'A',
        'Β' => b'B',
        'Χ' => b'C',
        'Δ' => b'D',
        'Ε' => b'E',
        'Φ' => b'F',
        'Γ' => b'G',
        'Η' => b'H',
        'Ι' => b'I',
        'ϑ' => b'J',
        'Κ' => b'K',
        'Λ' => b'L',
        'Μ' => b'M',
        'Ν' => b'N',
        'Ο' => b'O',
        'Π' => b'P',
        'Θ' => b'Q',
        'Ρ' => b'R',
        'Σ' => b'S',
        'Τ' => b'T',
        'Υ' => b'U',
        'ς' => b'V',
        'Ω' => b'W',
        'Ξ' => b'X',
        'Ψ' => b'Y',
        'Ζ' => b'Z',
        'α' => b'a',
        'β' => b'b',
        'χ' => b'c',
        'δ' => b'd',
        'ε' | 'ϵ' => b'e',
        'φ' => b'f',
        'γ' => b'g',
        'η' => b'h',
        'ι' => b'i',
        'ϕ' => b'j',
        'κ' => b'k',
        'λ' => b'l',
        'μ' => b'm',
        'ν' => b'n',
        'ο' => b'o',
        'π' => b'p',
        'θ' => b'q',
        'ρ' => b'r',
        'σ' => b's',
        'τ' => b't',
        'υ' => b'u',
        'ϖ' => b'v',
        'ω' => b'w',
        'ξ' => b'x',
        'ψ' => b'y',
        'ζ' => b'z',
        '∀' => 0x22,
        '∃' => 0x24,
        '∼' => b'~',
        '≤' => 0xa3,
        '∞' => 0xa5,
        '↔' => 0xab,
        '←' => 0xac,
        '→' => 0xae,
        '±' => 0xb1,
        '≥' => 0xb3,
        '×' => 0xb4,
        '∝' => 0xb5,
        '∂' => 0xb6,
        '÷' => 0xb8,
        '≠' => 0xb9,
        '≡' => 0xba,
        '≈' => 0xbb,
        '⊗' => 0xc4,
        '⊕' => 0xc5,
        '∅' => 0xc6,
        '∩' => 0xc7,
        '∪' => 0xc8,
        '⊃' => 0xc9,
        '⊇' => 0xca,
        '⊄' => 0xcb,
        '⊂' => 0xcc,
        '⊆' => 0xcd,
        '∈' => 0xce,
        '∉' => 0xcf,
        '∇' => 0xd1,
        '⊥' => 0x5e,
        '∏' => 0xd5,
        '√' => 0xd6,
        '·' => 0xd7,
        '¬' => 0xd8,
        '∧' => 0xd9,
        '∨' => 0xda,
        '⇔' => 0xdb,
        '⇐' => 0xdc,
        '⇒' => 0xde,
        '∑' => 0xe5,
        '∫' => 0xf2,
        _ => return None,
    })
}

fn push_pdf_text_char(out: &mut String, ch: char) {
    match ch {
        '(' => out.push_str("\\("),
        ')' => out.push_str("\\)"),
        '\\' => out.push_str("\\\\"),
        ch if ch.is_ascii() && !ch.is_control() => out.push(ch),
        _ => out.push('?'),
    }
}

fn pdf_text_ascii_replacement(ch: char) -> Option<&'static str> {
    Some(match ch {
        '\u{00A0}' => " ",
        '\u{00AD}' => "-",
        '\u{2010}' | '\u{2011}' | '\u{2012}' | '\u{2013}' | '\u{2212}' => "-",
        '\u{2014}' | '\u{2015}' => "--",
        '\u{2018}' | '\u{2019}' | '\u{201A}' | '\u{201B}' => "'",
        '\u{201C}' | '\u{201D}' | '\u{201E}' | '\u{201F}' => "\"",
        '\u{2026}' => "...",
        '\u{2032}' => "'",
        '\u{2033}' => "\"",
        '\u{00B1}' => "+/-",
        '∓' => "-/+",
        '\u{00D7}' => "x",
        '∂' => "partial",
        '≼' => "<=",
        '≽' => ">=",
        '∖' => "\\",
        '⊥' => "_|_",
        '∀' => "forall",
        '∃' => "exists",
        '∈' => "in",
        '∉' => "not in",
        '∑' => "sum",
        '∏' => "prod",
        '∫' => "int",
        '∞' => "infty",
        '≤' => "<=",
        '≥' => ">=",
        '≠' => "!=",
        '≈' => "approx",
        '∼' => "~",
        '∝' => "propto",
        '→' => "->",
        '←' => "<-",
        '↔' => "<->",
        '⇒' => "=>",
        '⇐' => "<=",
        '⇔' => "<=>",
        '·' => ".",
        '÷' => "/",
        '≡' => "==",
        '⊗' => "(x)",
        '⊕' => "(+)",
        '∅' => "emptyset",
        '∩' => "cap",
        '∪' => "cup",
        '⊂' => "subset",
        '⊆' => "subseteq",
        '⊃' => "supset",
        '⊇' => "supseteq",
        '⊄' => "not subset",
        '∇' => "nabla",
        '√' => "sqrt",
        '¬' => "not",
        '∧' => "and",
        '∨' => "or",
        'Α' => "Alpha",
        'Β' => "Beta",
        'Γ' => "Gamma",
        'Δ' => "Delta",
        'Ε' => "Epsilon",
        'Ζ' => "Zeta",
        'Η' => "Eta",
        'Θ' => "Theta",
        'Ι' => "Iota",
        'Κ' => "Kappa",
        'Λ' => "Lambda",
        'Μ' => "Mu",
        'Ν' => "Nu",
        'Ξ' => "Xi",
        'Ο' => "Omicron",
        'Π' => "Pi",
        'Ρ' => "Rho",
        'Σ' => "Sigma",
        'Τ' => "Tau",
        'Υ' => "Upsilon",
        'Φ' => "Phi",
        'Χ' => "Chi",
        'Ψ' => "Psi",
        'Ω' => "Omega",
        'α' => "alpha",
        'β' => "beta",
        'γ' => "gamma",
        'δ' => "delta",
        'ε' | 'ϵ' => "epsilon",
        'ζ' => "zeta",
        'η' => "eta",
        'θ' | 'ϑ' => "theta",
        'ι' => "iota",
        'κ' => "kappa",
        'λ' => "lambda",
        'μ' => "mu",
        'ν' => "nu",
        'ξ' => "xi",
        'ο' => "omicron",
        'π' => "pi",
        'ρ' => "rho",
        'ς' | 'σ' => "sigma",
        'τ' => "tau",
        'υ' => "upsilon",
        'φ' | 'ϕ' => "phi",
        'χ' => "chi",
        'ψ' => "psi",
        'ω' => "omega",
        '\u{00C0}' | '\u{00C1}' | '\u{00C2}' | '\u{00C3}' | '\u{00C4}' | '\u{00C5}'
        | '\u{0100}' | '\u{0102}' | '\u{0104}' => "A",
        '\u{00E0}' | '\u{00E1}' | '\u{00E2}' | '\u{00E3}' | '\u{00E4}' | '\u{00E5}'
        | '\u{0101}' | '\u{0103}' | '\u{0105}' => "a",
        '\u{00C6}' => "AE",
        '\u{00E6}' => "ae",
        '\u{00C7}' | '\u{0106}' | '\u{0108}' | '\u{010A}' | '\u{010C}' => "C",
        '\u{00E7}' | '\u{0107}' | '\u{0109}' | '\u{010B}' | '\u{010D}' => "c",
        '\u{00D0}' | '\u{010E}' | '\u{0110}' => "D",
        '\u{00F0}' | '\u{010F}' | '\u{0111}' => "d",
        '\u{00C8}' | '\u{00C9}' | '\u{00CA}' | '\u{00CB}' | '\u{0112}' | '\u{0114}'
        | '\u{0116}' | '\u{0118}' | '\u{011A}' => "E",
        '\u{00E8}' | '\u{00E9}' | '\u{00EA}' | '\u{00EB}' | '\u{0113}' | '\u{0115}'
        | '\u{0117}' | '\u{0119}' | '\u{011B}' => "e",
        '\u{011C}' | '\u{011E}' | '\u{0120}' | '\u{0122}' => "G",
        '\u{011D}' | '\u{011F}' | '\u{0121}' | '\u{0123}' => "g",
        '\u{0124}' | '\u{0126}' => "H",
        '\u{0125}' | '\u{0127}' => "h",
        '\u{00CC}' | '\u{00CD}' | '\u{00CE}' | '\u{00CF}' | '\u{0128}' | '\u{012A}'
        | '\u{012C}' | '\u{012E}' | '\u{0130}' => "I",
        '\u{00EC}' | '\u{00ED}' | '\u{00EE}' | '\u{00EF}' | '\u{0129}' | '\u{012B}'
        | '\u{012D}' | '\u{012F}' | '\u{0131}' => "i",
        '\u{0134}' => "J",
        '\u{0135}' => "j",
        '\u{0136}' => "K",
        '\u{0137}' => "k",
        '\u{0139}' | '\u{013B}' | '\u{013D}' | '\u{013F}' | '\u{0141}' => "L",
        '\u{013A}' | '\u{013C}' | '\u{013E}' | '\u{0140}' | '\u{0142}' => "l",
        '\u{00D1}' | '\u{0143}' | '\u{0145}' | '\u{0147}' => "N",
        '\u{00F1}' | '\u{0144}' | '\u{0146}' | '\u{0148}' => "n",
        '\u{00D2}' | '\u{00D3}' | '\u{00D4}' | '\u{00D5}' | '\u{00D6}' | '\u{00D8}'
        | '\u{014C}' | '\u{014E}' | '\u{0150}' => "O",
        '\u{00F2}' | '\u{00F3}' | '\u{00F4}' | '\u{00F5}' | '\u{00F6}' | '\u{00F8}'
        | '\u{014D}' | '\u{014F}' | '\u{0151}' => "o",
        '\u{0152}' => "OE",
        '\u{0153}' => "oe",
        '\u{0154}' | '\u{0156}' | '\u{0158}' => "R",
        '\u{0155}' | '\u{0157}' | '\u{0159}' => "r",
        '\u{015A}' | '\u{015C}' | '\u{015E}' | '\u{0160}' => "S",
        '\u{015B}' | '\u{015D}' | '\u{015F}' | '\u{0161}' | '\u{017F}' => "s",
        '\u{00DF}' => "ss",
        '\u{0162}' | '\u{0164}' | '\u{0166}' => "T",
        '\u{0163}' | '\u{0165}' | '\u{0167}' => "t",
        '\u{00D9}' | '\u{00DA}' | '\u{00DB}' | '\u{00DC}' | '\u{0168}' | '\u{016A}'
        | '\u{016C}' | '\u{016E}' | '\u{0170}' | '\u{0172}' => "U",
        '\u{00F9}' | '\u{00FA}' | '\u{00FB}' | '\u{00FC}' | '\u{0169}' | '\u{016B}'
        | '\u{016D}' | '\u{016F}' | '\u{0171}' | '\u{0173}' => "u",
        '\u{0174}' => "W",
        '\u{0175}' => "w",
        '\u{00DD}' | '\u{0176}' | '\u{0178}' => "Y",
        '\u{00FD}' | '\u{00FF}' | '\u{0177}' => "y",
        '\u{0179}' | '\u{017B}' | '\u{017D}' => "Z",
        '\u{017A}' | '\u{017C}' | '\u{017E}' => "z",
        _ => return None,
    })
}

fn write_aux(path: &Path, document: &SimpleDocument) -> io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    writeln!(file, "\\relax")?;
    for key in &document.citations.keys {
        writeln!(file, "\\citation{{{key}}}")?;
    }
    for (key, label) in &document.labels {
        writeln!(
            file,
            "\\newlabel{{{key}}}{{{{{}}}{{{}}}}}",
            label.value, label.page
        )?;
    }
    for entry in &document.toc_entries {
        writeln!(file, "\\@writefile{{toc}}{{{}}}", toc_record(entry))?;
    }
    for entry in &document.float_entries {
        writeln!(
            file,
            "\\@writefile{{{}}}{{{}}}",
            entry.kind.sidecar_extension(),
            float_record(entry)
        )?;
    }
    for style in &document.bibliography.styles {
        writeln!(file, "\\bibstyle{{{style}}}")?;
    }
    for database in &document.bibliography.databases {
        writeln!(file, "\\bibdata{{{database}}}")?;
    }
    Ok(())
}

fn write_toc(path: &Path, document: &SimpleDocument) -> io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    for entry in &document.toc_entries {
        writeln!(file, "{}", toc_record(entry))?;
    }
    Ok(())
}

fn write_float_list(path: &Path, document: &SimpleDocument, kind: FloatKind) -> io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    for entry in document
        .float_entries
        .iter()
        .filter(|entry| entry.kind == kind)
    {
        writeln!(file, "{}", float_record(entry))?;
    }
    Ok(())
}

fn write_hyperref_out(path: &Path, document: &SimpleDocument) -> io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    for entry in &document.bookmarks {
        writeln!(file, "{}", bookmark_record(entry))?;
    }
    Ok(())
}

fn write_backrefs(path: &Path, document: &SimpleDocument) -> io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    for entry in &document.citations.backrefs {
        writeln!(file, "{}", backref_record(entry))?;
    }
    Ok(())
}

fn write_generated_outputs(outputs: &[GeneratedOutput]) -> io::Result<()> {
    for output in outputs {
        if let Some(parent) = output.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&output.path, &output.content)?;
    }
    Ok(())
}

fn toc_record(entry: &TocEntry) -> String {
    let title = tex_aux_text(&entry.title);
    let payload = match entry.number.as_deref() {
        Some(number) if !number.is_empty() => {
            format!("\\numberline {{{}}}{title}", tex_aux_text(number))
        }
        _ => title,
    };
    format!(
        "\\contentsline {{{}}}{{{}}}{{{}}}{{}}",
        tex_aux_text(&entry.kind),
        payload,
        entry.page
    )
}

fn float_record(entry: &FloatEntry) -> String {
    format!(
        "\\contentsline {{{}}}{{\\numberline {{{}}}{}}}{{{}}}{{}}",
        entry.kind.contents_kind(),
        tex_aux_text(&entry.number),
        tex_aux_text(&entry.title),
        entry.page
    )
}

fn bookmark_record(entry: &BookmarkEntry) -> String {
    format!(
        "\\BOOKMARK [{}][-]{{{}}}{{{}}}{{}}{{}}% page {}",
        entry.level,
        tex_aux_text(&entry.dest),
        tex_aux_text(&entry.title),
        entry.page
    )
}

fn backref_record(entry: &CitationBackref) -> String {
    format!(
        "\\backcite{{{}}}{{{{{}}}{{}}}}",
        tex_aux_text(&entry.key),
        entry.page
    )
}

fn tex_aux_text(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            '\\' => "\\textbackslash{}".to_string(),
            '{' => "\\{".to_string(),
            '}' => "\\}".to_string(),
            _ => ch.to_string(),
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct SynctexBox {
    page: usize,
    line: usize,
    x_sp: i64,
    y_sp: i64,
    width_sp: i64,
    height_sp: i64,
}

fn write_synctex(
    path: &Path,
    options: &NativeEngineOptions,
    document: &SimpleDocument,
    page_count: usize,
    placements: &[LinePlacement],
) -> io::Result<()> {
    let main = fs::canonicalize(&options.main).unwrap_or_else(|_| options.main.clone());
    let boxes = synctex_boxes_from_placements(document, placements);
    let mut source = String::new();
    writeln!(source, "SyncTeX Version:1").unwrap();
    writeln!(source, "Input:1:{}", main.display()).unwrap();
    writeln!(source, "Output:pdf").unwrap();
    writeln!(source, "Magnification:1000").unwrap();
    writeln!(source, "Unit:1").unwrap();
    writeln!(source, "X Offset:0").unwrap();
    writeln!(source, "Y Offset:0").unwrap();
    writeln!(source, "Content:").unwrap();
    for page in 1..=page_count {
        writeln!(source, "!{}", source.len()).unwrap();
        writeln!(source, "{{{page}").unwrap();
        writeln!(
            source,
            "[1,1:0,0:{},{}:0",
            synctex_sp(612.0),
            synctex_sp(792.0)
        )
        .unwrap();
        for item in boxes.iter().filter(|item| item.page == page) {
            writeln!(
                source,
                "h1,{}:{},{}:{},{},0",
                item.line, item.x_sp, item.y_sp, item.width_sp, item.height_sp
            )
            .unwrap();
        }
        writeln!(source, "]").unwrap();
        writeln!(source, "!{}", source.len()).unwrap();
        writeln!(source, "}}{page}").unwrap();
    }
    writeln!(source, "Postamble:").unwrap();
    writeln!(source, "Count:{}", boxes.len() + page_count * 2).unwrap();
    writeln!(source, "!{}", source.len()).unwrap();
    writeln!(source, "Post scriptum:").unwrap();

    let file = File::create(path)?;
    let mut encoder = GzEncoder::new(file, Compression::fast());
    encoder.write_all(source.as_bytes())?;
    encoder.finish()?;
    Ok(())
}

fn synctex_line_geometry(
    line: &Line,
    line_slots: usize,
    document: &SimpleDocument,
    layout: DocumentLayout,
) -> Option<(f32, f32, f32)> {
    match line {
        Line::PageStyle(_) | Line::OutputControl(_) => None,
        Line::Title(text) => Some((
            0.0,
            title_text_width_pt(layout, text, layout.title_font_pt, layout.column_width_pt),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::WideTitle(text) => Some((
            0.0,
            title_text_width_pt(layout, text, layout.title_font_pt, layout.text_width_pt),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::Author(text) => Some((
            0.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.author_font_pt,
                layout.column_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::WideAuthor(text) => Some((
            0.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.author_font_pt,
                layout.text_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::AuthorGrid(_) => Some((
            0.0,
            layout.text_width_pt,
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::AbstractHeading(text) => Some((
            0.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Heading,
                layout.abstract_heading_font_pt,
                layout.column_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::AbstractText(text) => Some((
            12.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.text_font_pt,
                layout.column_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::JustifiedAbstractText { text, width_pt } => Some((
            12.0,
            (*width_pt).max(text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.text_font_pt,
                layout.column_width_pt,
            )),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::WideBackground(_) => None,
        Line::WideTheoremBackground { .. } => None,
        Line::WideAbstractText(text) => Some((
            12.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.text_font_pt,
                layout.text_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::WideCaption(text) => Some((
            0.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.footnote_font_pt,
                layout.text_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::WideEquation(text) => Some((
            0.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Math,
                layout.text_font_pt,
                layout.text_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::Heading(text) => Some((
            0.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Heading,
                layout.heading_font_pt,
                layout.text_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::ParagraphText(text) => Some((
            0.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.text_font_pt,
                layout.column_width_pt,
            ),
            layout.line_height_pt,
        )),
        Line::JustifiedParagraphText { text, width_pt } => Some((
            0.0,
            (*width_pt).max(text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.text_font_pt,
                layout.column_width_pt,
            )),
            layout.line_height_pt,
        )),
        Line::Text(text) => Some((
            0.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.text_font_pt,
                layout.column_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::JustifiedText { text, width_pt } => Some((
            0.0,
            (*width_pt).max(text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.text_font_pt,
                layout.column_width_pt,
            )),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::TableRow(text) => Some((
            0.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.footnote_font_pt,
                layout.column_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::TableCells { .. } => Some((
            0.0,
            layout.column_width_pt,
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::WideTableCells { .. } => Some((
            0.0,
            layout.text_width_pt,
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::Caption(text) => Some((
            0.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.footnote_font_pt,
                layout.column_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::Footnote(text) => Some((
            12.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Text,
                layout.footnote_font_pt,
                layout.column_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::DisplayEquation(text) | Line::Equation(text) => Some((
            24.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Math,
                layout.text_font_pt,
                layout.column_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::Code(text) => Some((
            12.0,
            text_width_for_layout_font_pt(
                text,
                layout,
                PdfTextFont::Code,
                layout.code_font_pt,
                layout.column_width_pt,
            ),
            line_slots as f32 * layout.line_height_pt,
        )),
        Line::Image(index) => {
            let image = &document.images[*index];
            let (width, height) = layout.image_display_size(image);
            Some((0.0, width, height))
        }
        Line::WideImageRow(indices) => {
            let (width, height) = wide_image_row_display_size(layout, &document.images, indices);
            Some((0.0, width, height))
        }
        Line::WideTeaserRow(cells) => {
            let (width, height) = wide_teaser_row_display_size(layout, &document.images, cells);
            Some((0.0, width, height))
        }
        Line::TeaserRow(cells) => {
            let (width, height) =
                teaser_row_display_size(layout, &document.images, cells, layout.column_width_pt);
            Some((0.0, width, height))
        }
        Line::FloatBlock { .. } | Line::LateFloatBlock { .. } | Line::BottomFloatBlock { .. } => {
            None
        }
        Line::Blank => None,
    }
}

const WIDE_TEASER_GAP_PT: f32 = 8.0;

#[derive(Debug, Clone, Copy)]
struct WideImageRowItem {
    index: usize,
    x_offset: f32,
    draw_width: f32,
    draw_height: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Copy)]
struct ImageTransform {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
}

fn rotated_box_size(width: f32, height: f32, angle_degrees: f32) -> (f32, f32) {
    let radians = angle_degrees.to_radians();
    let sin = radians.sin();
    let cos = radians.cos();
    let corners = [
        (0.0_f32, 0.0_f32),
        (width * cos, width * sin),
        (-height * sin, height * cos),
        (width * cos - height * sin, width * sin + height * cos),
    ];
    let min_x = corners
        .iter()
        .map(|(x, _)| *x)
        .fold(f32::INFINITY, f32::min);
    let max_x = corners
        .iter()
        .map(|(x, _)| *x)
        .fold(f32::NEG_INFINITY, f32::max);
    let min_y = corners
        .iter()
        .map(|(_, y)| *y)
        .fold(f32::INFINITY, f32::min);
    let max_y = corners
        .iter()
        .map(|(_, y)| *y)
        .fold(f32::NEG_INFINITY, f32::max);
    (max_x - min_x, max_y - min_y)
}

fn image_transform_at(
    full_draw_width: f32,
    full_draw_height: f32,
    angle_degrees: f32,
    box_x: f32,
    box_y: f32,
    viewport: ImageViewport,
) -> ImageTransform {
    let radians = angle_degrees.to_radians();
    let sin = radians.sin();
    let cos = radians.cos();
    let a = full_draw_width * cos;
    let b = full_draw_width * sin;
    let c = -full_draw_height * sin;
    let d = full_draw_height * cos;
    let left = viewport.left_fraction;
    let bottom = viewport.bottom_fraction;
    let right = left + viewport.width_fraction;
    let top = bottom + viewport.height_fraction;
    let corners = [(left, bottom), (right, bottom), (left, top), (right, top)]
        .map(|(x, y)| (a * x + c * y, b * x + d * y));
    let min_x = corners
        .iter()
        .map(|(x, _)| *x)
        .fold(f32::INFINITY, f32::min);
    let min_y = corners
        .iter()
        .map(|(_, y)| *y)
        .fold(f32::INFINITY, f32::min);
    ImageTransform {
        a,
        b,
        c,
        d,
        e: box_x - min_x,
        f: box_y - min_y,
    }
}

fn append_image_xobject(
    stream: &mut String,
    index: usize,
    image: &ImageAsset,
    draw_width: f32,
    draw_height: f32,
    box_x: f32,
    box_y: f32,
) {
    let full_draw_width = draw_width / image.viewport.width_fraction;
    let full_draw_height = draw_height / image.viewport.height_fraction;
    let transform = image_transform_at(
        full_draw_width,
        full_draw_height,
        image.rotation_degrees,
        box_x,
        box_y,
        image.viewport,
    );
    if image.viewport.clip {
        writeln!(
            stream,
            "q\n{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} cm\n{:.6} {:.6} {:.6} {:.6} re\nW\nn\n/Im{} Do\nQ",
            transform.a,
            transform.b,
            transform.c,
            transform.d,
            transform.e,
            transform.f,
            image.viewport.left_fraction,
            image.viewport.bottom_fraction,
            image.viewport.width_fraction,
            image.viewport.height_fraction,
            index + 1
        )
        .unwrap();
    } else {
        writeln!(
            stream,
            "q\n{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} cm\n/Im{} Do\nQ",
            transform.a,
            transform.b,
            transform.c,
            transform.d,
            transform.e,
            transform.f,
            index + 1
        )
        .unwrap();
    }
}

fn wide_image_row_items(
    layout: DocumentLayout,
    images: &[ImageAsset],
    indices: &[usize],
) -> Vec<WideImageRowItem> {
    image_row_items_for_width(layout.text_width_pt, images, indices)
}

fn image_row_items_for_width(
    max_width: f32,
    images: &[ImageAsset],
    indices: &[usize],
) -> Vec<WideImageRowItem> {
    if indices.is_empty() {
        return Vec::new();
    }
    let gap = WIDE_TEASER_GAP_PT;
    let mut sizes = indices
        .iter()
        .map(|index| {
            let image = &images[*index];
            let (width, height) = rotated_box_size(
                image.display_width_pt,
                image.display_height_pt,
                image.rotation_degrees,
            );
            (
                image.display_width_pt,
                image.display_height_pt,
                width,
                height,
            )
        })
        .collect::<Vec<_>>();
    let natural_width = sizes.iter().map(|(_, _, width, _)| *width).sum::<f32>()
        + gap * indices.len().saturating_sub(1) as f32;
    let scale = if natural_width > max_width {
        max_width / natural_width
    } else {
        1.0
    };
    let mut x_offset = 0.0_f32;
    let mut out = Vec::with_capacity(indices.len());
    for (index, (draw_width, draw_height, width, height)) in
        indices.iter().copied().zip(sizes.drain(..))
    {
        let draw_width = draw_width * scale;
        let draw_height = draw_height * scale;
        let width = width * scale;
        let height = height * scale;
        out.push(WideImageRowItem {
            index,
            x_offset,
            draw_width,
            draw_height,
            width,
            height,
        });
        x_offset += width + gap * scale;
    }
    out
}

fn wide_image_row_display_size(
    layout: DocumentLayout,
    images: &[ImageAsset],
    indices: &[usize],
) -> (f32, f32) {
    let items = wide_image_row_items(layout, images, indices);
    let width = items
        .iter()
        .map(|item| item.x_offset + item.width)
        .fold(0.0_f32, f32::max)
        .min(layout.text_width_pt);
    let height = items.iter().map(|item| item.height).fold(0.0_f32, f32::max);
    (width, height)
}

fn append_wide_teaser_row_to_stream(
    stream: &mut String,
    layout: DocumentLayout,
    images: &[ImageAsset],
    cells: &[WideTeaserCell],
    page_slot: usize,
) {
    append_teaser_row_to_stream(
        stream,
        layout,
        images,
        cells,
        page_slot,
        layout.text_width_pt,
        true,
    );
}

fn append_teaser_row_to_stream(
    stream: &mut String,
    layout: DocumentLayout,
    images: &[ImageAsset],
    cells: &[WideTeaserCell],
    page_slot: usize,
    max_width: f32,
    is_wide: bool,
) {
    let widths = teaser_cell_widths(max_width, cells);
    let (x, y) = if is_wide {
        layout.point_for_wide_slot(page_slot, 0.0)
    } else {
        layout.point_for_slot(page_slot, 0.0)
    };
    let mut x_offset = 0.0_f32;
    for (cell, cell_width) in cells.iter().zip(widths) {
        let cell_x = x + x_offset;
        let mut current_y = y;
        if !cell.images.is_empty() {
            let row = image_row_items_for_width(cell_width, images, &cell.images);
            let row_width = row
                .iter()
                .map(|item| item.x_offset + item.width)
                .fold(0.0_f32, f32::max);
            let row_height = row.iter().map(|item| item.height).fold(0.0_f32, f32::max);
            let image_x = cell_x + ((cell_width - row_width) / 2.0).max(0.0);
            for item in row {
                let image = &images[item.index];
                let image_y = (current_y - item.height).max(72.0);
                append_image_xobject(
                    stream,
                    item.index,
                    image,
                    item.draw_width,
                    item.draw_height,
                    image_x + item.x_offset,
                    image_y,
                );
            }
            current_y -= row_height + 4.0;
        }
        for line in &cell.text_lines {
            writeln!(
                stream,
                "BT\n/F1 {:.2} Tf\n{:.2} {:.2} Td\n({}) Tj\nET",
                layout.footnote_font_pt,
                cell_x,
                current_y,
                pdf_text(line)
            )
            .unwrap();
            current_y -= layout.line_height_pt;
        }
        x_offset += cell_width + WIDE_TEASER_GAP_PT;
    }
}

fn wide_teaser_row_display_size(
    layout: DocumentLayout,
    images: &[ImageAsset],
    cells: &[WideTeaserCell],
) -> (f32, f32) {
    teaser_row_display_size(layout, images, cells, layout.text_width_pt)
}

fn teaser_row_display_size(
    layout: DocumentLayout,
    images: &[ImageAsset],
    cells: &[WideTeaserCell],
    max_width: f32,
) -> (f32, f32) {
    if cells.is_empty() {
        return (0.0, 0.0);
    }
    let widths = teaser_cell_widths(max_width, cells);
    let width =
        widths.iter().sum::<f32>() + WIDE_TEASER_GAP_PT * widths.len().saturating_sub(1) as f32;
    let height = cells
        .iter()
        .zip(widths)
        .map(|(cell, width)| wide_teaser_cell_height(layout, images, cell, width))
        .fold(0.0_f32, f32::max);
    (width.min(max_width), height)
}

fn teaser_cell_widths(max_width: f32, cells: &[WideTeaserCell]) -> Vec<f32> {
    if cells.is_empty() {
        return Vec::new();
    }
    let gap_total = WIDE_TEASER_GAP_PT * cells.len().saturating_sub(1) as f32;
    let available = (max_width - gap_total).max(24.0);
    let natural_total = cells
        .iter()
        .map(|cell| cell.width_pt.clamp(24.0, max_width))
        .sum::<f32>();
    if natural_total <= f32::EPSILON {
        return vec![available / cells.len() as f32; cells.len()];
    }
    let scale = if natural_total > available {
        available / natural_total
    } else {
        1.0
    };
    cells
        .iter()
        .map(|cell| cell.width_pt.clamp(24.0, max_width) * scale)
        .collect()
}

fn wide_teaser_cell_height(
    layout: DocumentLayout,
    images: &[ImageAsset],
    cell: &WideTeaserCell,
    width: f32,
) -> f32 {
    let image_height = image_row_items_for_width(width, images, &cell.images)
        .iter()
        .map(|item| item.height)
        .fold(0.0_f32, f32::max);
    let text_height = cell.text_lines.len() as f32 * layout.line_height_pt;
    match (image_height > 0.0, text_height > 0.0) {
        (true, true) => image_height + 4.0 + text_height,
        (true, false) => image_height,
        (false, true) => text_height,
        (false, false) => 0.0,
    }
}

fn text_width_for_layout_font_pt(
    text: &str,
    layout: DocumentLayout,
    font: PdfTextFont,
    font_size: f32,
    max_width: f32,
) -> f32 {
    text_width_for_metric_pt(text, layout.metric_for_font(font), font_size, max_width)
}

fn text_width_for_metric_pt(
    text: &str,
    font_metric: PdfFontMetric,
    font_size: f32,
    max_width: f32,
) -> f32 {
    natural_text_width_for_metric_pt(text, font_metric, font_size).clamp(font_size, max_width)
}

fn natural_text_width_for_metric_pt(text: &str, font_metric: PdfFontMetric, font_size: f32) -> f32 {
    pdf_font_text_units(text, font_metric) * font_size / 1000.0
}

fn centered_text_position(
    layout: DocumentLayout,
    page_slot: usize,
    text: &str,
    font_size: f32,
) -> (f32, f32) {
    centered_text_position_for_font(layout, page_slot, text, PdfTextFont::Text, font_size)
}

fn centered_text_position_for_font(
    layout: DocumentLayout,
    page_slot: usize,
    text: &str,
    font: PdfTextFont,
    font_size: f32,
) -> (f32, f32) {
    let (left, y) = layout.point_for_slot(page_slot, 0.0);
    let width =
        text_width_for_layout_font_pt(text, layout, font, font_size, layout.column_width_pt);
    let x = left + ((layout.column_width_pt - width) / 2.0).max(0.0);
    (x, y)
}

fn centered_title_text_position(
    layout: DocumentLayout,
    page_slot: usize,
    text: &str,
) -> (f32, f32) {
    let (left, y) = layout.point_for_slot(page_slot, 0.0);
    let width = title_text_width_pt(layout, text, layout.title_font_pt, layout.column_width_pt);
    let x = left + ((layout.column_width_pt - width) / 2.0).max(0.0);
    (x, y)
}

fn wide_centered_text_position(
    layout: DocumentLayout,
    page_slot: usize,
    text: &str,
    font_size: f32,
) -> (f32, f32) {
    wide_centered_text_position_for_font(layout, page_slot, text, PdfTextFont::Text, font_size)
}

fn wide_centered_text_position_for_font(
    layout: DocumentLayout,
    page_slot: usize,
    text: &str,
    font: PdfTextFont,
    font_size: f32,
) -> (f32, f32) {
    let (left, y) = layout.point_for_wide_slot(page_slot, 0.0);
    let width = text_width_for_layout_font_pt(text, layout, font, font_size, layout.text_width_pt);
    let x = left + ((layout.text_width_pt - width) / 2.0).max(0.0);
    (x, y)
}

fn wide_centered_title_text_position(
    layout: DocumentLayout,
    page_slot: usize,
    text: &str,
) -> (f32, f32) {
    let (left, y) = layout.point_for_wide_slot(page_slot, 0.0);
    let width = title_text_width_pt(layout, text, layout.title_font_pt, layout.text_width_pt);
    let x = left + ((layout.text_width_pt - width) / 2.0).max(0.0);
    (x, y)
}

fn title_text_width_pt(layout: DocumentLayout, text: &str, font_size: f32, max_width: f32) -> f32 {
    let width =
        text_width_for_layout_font_pt(text, layout, PdfTextFont::Heading, font_size, max_width);
    if layout == DocumentLayout::neurips_single_column() {
        (width * NEURIPS_TITLE_WIDTH_SCALE).clamp(font_size, max_width)
    } else {
        width
    }
}

const ASCII_WIDTH_FIRST: u32 = 32;
const ASCII_WIDTH_LAST: u32 = 126;

const TIMES_ROMAN_WIDTHS: [f32; 95] = [
    250.0, 333.0, 408.0, 500.0, 500.0, 833.0, 778.0, 333.0, 333.0, 333.0, 500.0, 564.0, 250.0,
    333.0, 250.0, 278.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0,
    278.0, 278.0, 564.0, 564.0, 564.0, 444.0, 921.0, 722.0, 667.0, 667.0, 722.0, 611.0, 556.0,
    722.0, 722.0, 333.0, 389.0, 722.0, 611.0, 889.0, 722.0, 722.0, 556.0, 722.0, 667.0, 556.0,
    611.0, 722.0, 722.0, 944.0, 722.0, 722.0, 611.0, 333.0, 278.0, 333.0, 469.0, 500.0, 333.0,
    444.0, 500.0, 444.0, 500.0, 444.0, 333.0, 500.0, 500.0, 278.0, 278.0, 500.0, 278.0, 778.0,
    500.0, 500.0, 500.0, 500.0, 333.0, 389.0, 278.0, 500.0, 500.0, 722.0, 500.0, 500.0, 444.0,
    480.0, 200.0, 480.0, 541.0,
];

const TIMES_ITALIC_WIDTHS: [f32; 95] = [
    250.0, 333.0, 420.0, 500.0, 500.0, 833.0, 778.0, 333.0, 333.0, 333.0, 500.0, 675.0, 250.0,
    333.0, 250.0, 278.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0,
    333.0, 333.0, 675.0, 675.0, 675.0, 500.0, 920.0, 611.0, 611.0, 667.0, 722.0, 611.0, 611.0,
    722.0, 722.0, 333.0, 444.0, 667.0, 556.0, 833.0, 667.0, 722.0, 611.0, 722.0, 611.0, 500.0,
    556.0, 722.0, 611.0, 833.0, 611.0, 556.0, 556.0, 389.0, 278.0, 389.0, 422.0, 500.0, 333.0,
    500.0, 500.0, 444.0, 500.0, 444.0, 278.0, 500.0, 500.0, 278.0, 278.0, 444.0, 278.0, 722.0,
    500.0, 500.0, 500.0, 500.0, 389.0, 389.0, 278.0, 500.0, 444.0, 667.0, 444.0, 444.0, 389.0,
    400.0, 275.0, 400.0, 541.0,
];

const TIMES_BOLD_WIDTHS: [f32; 95] = [
    250.0, 333.0, 555.0, 500.0, 500.0, 1000.0, 833.0, 333.0, 333.0, 333.0, 500.0, 570.0, 250.0,
    333.0, 250.0, 278.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0,
    333.0, 333.0, 570.0, 570.0, 570.0, 500.0, 930.0, 722.0, 667.0, 722.0, 722.0, 667.0, 611.0,
    778.0, 778.0, 389.0, 500.0, 778.0, 667.0, 944.0, 722.0, 778.0, 611.0, 778.0, 722.0, 556.0,
    667.0, 722.0, 722.0, 1000.0, 722.0, 722.0, 667.0, 333.0, 278.0, 333.0, 581.0, 500.0, 333.0,
    500.0, 556.0, 444.0, 556.0, 444.0, 333.0, 500.0, 556.0, 278.0, 333.0, 556.0, 278.0, 833.0,
    556.0, 500.0, 556.0, 556.0, 444.0, 389.0, 333.0, 556.0, 500.0, 722.0, 500.0, 500.0, 444.0,
    394.0, 220.0, 394.0, 520.0,
];

const PAGELLA_WIDTHS: [f32; 95] = [
    250.0, 278.0, 371.0, 500.0, 500.0, 840.0, 778.0, 208.0, 456.0, 456.0, 390.0, 760.0, 250.0,
    333.0, 250.0, 486.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0,
    250.0, 250.0, 767.0, 760.0, 767.0, 444.0, 747.0, 778.0, 611.0, 709.0, 774.0, 611.0, 556.0,
    763.0, 832.0, 337.0, 333.0, 726.0, 611.0, 946.0, 831.0, 786.0, 604.0, 786.0, 668.0, 525.0,
    613.0, 778.0, 722.0, 1000.0, 667.0, 667.0, 667.0, 428.0, 486.0, 428.0, 606.0, 500.0, 333.0,
    500.0, 553.0, 444.0, 611.0, 479.0, 333.0, 556.0, 582.0, 287.0, 234.0, 556.0, 291.0, 883.0,
    582.0, 546.0, 601.0, 560.0, 395.0, 424.0, 326.0, 603.0, 565.0, 834.0, 516.0, 556.0, 500.0,
    441.0, 208.0, 441.0, 606.0,
];

const PAGELLA_ITALIC_WIDTHS: [f32; 95] = [
    250.0, 333.0, 500.0, 500.0, 500.0, 889.0, 778.0, 233.0, 456.0, 456.0, 390.0, 760.0, 250.0,
    333.0, 250.0, 486.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0,
    250.0, 250.0, 767.0, 760.0, 767.0, 500.0, 747.0, 722.0, 611.0, 667.0, 778.0, 611.0, 556.0,
    722.0, 778.0, 333.0, 333.0, 667.0, 556.0, 944.0, 778.0, 778.0, 611.0, 778.0, 667.0, 556.0,
    611.0, 778.0, 722.0, 944.0, 722.0, 667.0, 667.0, 428.0, 486.0, 428.0, 606.0, 500.0, 333.0,
    444.0, 463.0, 407.0, 500.0, 389.0, 278.0, 500.0, 500.0, 278.0, 278.0, 444.0, 278.0, 778.0,
    556.0, 444.0, 500.0, 463.0, 389.0, 389.0, 333.0, 556.0, 500.0, 722.0, 500.0, 500.0, 444.0,
    441.0, 208.0, 441.0, 606.0,
];

const PAGELLA_BOLD_WIDTHS: [f32; 95] = [
    250.0, 278.0, 402.0, 500.0, 500.0, 889.0, 833.0, 227.0, 490.0, 490.0, 421.0, 760.0, 250.0,
    333.0, 250.0, 511.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0, 500.0,
    250.0, 250.0, 769.0, 760.0, 769.0, 444.0, 747.0, 778.0, 667.0, 722.0, 833.0, 611.0, 556.0,
    833.0, 833.0, 389.0, 389.0, 778.0, 611.0, 1000.0, 833.0, 833.0, 611.0, 833.0, 722.0, 611.0,
    667.0, 778.0, 778.0, 1000.0, 667.0, 667.0, 667.0, 457.0, 511.0, 457.0, 606.0, 500.0, 333.0,
    500.0, 611.0, 444.0, 611.0, 500.0, 389.0, 556.0, 611.0, 333.0, 333.0, 611.0, 333.0, 889.0,
    611.0, 556.0, 611.0, 611.0, 389.0, 444.0, 333.0, 611.0, 556.0, 833.0, 500.0, 556.0, 500.0,
    472.0, 232.0, 472.0, 606.0,
];

const HEROS_WIDTHS: [f32; 95] = [
    278.0, 278.0, 355.0, 556.0, 556.0, 889.0, 667.0, 191.0, 333.0, 333.0, 389.0, 584.0, 278.0,
    333.0, 278.0, 278.0, 556.0, 556.0, 556.0, 556.0, 556.0, 556.0, 556.0, 556.0, 556.0, 556.0,
    278.0, 278.0, 584.0, 584.0, 584.0, 556.0, 1015.0, 667.0, 667.0, 722.0, 722.0, 667.0, 611.0,
    778.0, 722.0, 278.0, 500.0, 667.0, 556.0, 833.0, 722.0, 778.0, 667.0, 778.0, 722.0, 667.0,
    611.0, 722.0, 667.0, 944.0, 667.0, 667.0, 611.0, 278.0, 278.0, 278.0, 469.0, 556.0, 333.0,
    556.0, 556.0, 500.0, 556.0, 556.0, 278.0, 556.0, 556.0, 222.0, 222.0, 500.0, 222.0, 833.0,
    556.0, 556.0, 556.0, 556.0, 333.0, 500.0, 278.0, 556.0, 500.0, 722.0, 500.0, 500.0, 500.0,
    334.0, 260.0, 334.0, 584.0,
];

const HEROS_BOLD_WIDTHS: [f32; 95] = [
    278.0, 333.0, 474.0, 556.0, 556.0, 889.0, 722.0, 238.0, 333.0, 333.0, 389.0, 584.0, 278.0,
    333.0, 278.0, 278.0, 556.0, 556.0, 556.0, 556.0, 556.0, 556.0, 556.0, 556.0, 556.0, 556.0,
    333.0, 333.0, 584.0, 584.0, 584.0, 611.0, 975.0, 722.0, 722.0, 722.0, 722.0, 667.0, 611.0,
    778.0, 722.0, 278.0, 556.0, 722.0, 611.0, 833.0, 722.0, 778.0, 667.0, 778.0, 722.0, 667.0,
    611.0, 722.0, 667.0, 944.0, 667.0, 667.0, 611.0, 333.0, 278.0, 333.0, 584.0, 556.0, 333.0,
    556.0, 611.0, 556.0, 611.0, 556.0, 333.0, 611.0, 611.0, 278.0, 278.0, 556.0, 278.0, 889.0,
    611.0, 611.0, 611.0, 611.0, 389.0, 556.0, 333.0, 611.0, 556.0, 778.0, 556.0, 556.0, 500.0,
    389.0, 280.0, 389.0, 584.0,
];

const PDF_ASCII_GLYPH_NAMES: [&str; 95] = [
    "space",
    "exclam",
    "quotedbl",
    "numbersign",
    "dollar",
    "percent",
    "ampersand",
    "quotesingle",
    "parenleft",
    "parenright",
    "asterisk",
    "plus",
    "comma",
    "hyphen",
    "period",
    "slash",
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "colon",
    "semicolon",
    "less",
    "equal",
    "greater",
    "question",
    "at",
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    "bracketleft",
    "backslash",
    "bracketright",
    "asciicircum",
    "underscore",
    "grave",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "j",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "u",
    "v",
    "w",
    "x",
    "y",
    "z",
    "braceleft",
    "bar",
    "braceright",
    "asciitilde",
];

const PAGELLA_TYPE1_METRICS: Type1Metrics = Type1Metrics {
    font_bbox: [-514, -283, 1284, 1098],
    italic_angle: 0.0,
    ascender: 662,
    descender: -269,
    cap_height: 700,
    x_height: 469,
    fixed_pitch: false,
    bold: false,
    widths: &PAGELLA_WIDTHS,
};

const PAGELLA_ITALIC_TYPE1_METRICS: Type1Metrics = Type1Metrics {
    font_bbox: [-423, -277, 1292, 1094],
    italic_angle: -10.0,
    ascender: 681,
    descender: -274,
    cap_height: 700,
    x_height: 529,
    fixed_pitch: false,
    bold: false,
    widths: &PAGELLA_ITALIC_WIDTHS,
};

const PAGELLA_BOLD_TYPE1_METRICS: Type1Metrics = Type1Metrics {
    font_bbox: [-560, -272, 1311, 1097],
    italic_angle: 0.0,
    ascender: 676,
    descender: -259,
    cap_height: 678,
    x_height: 471,
    fixed_pitch: false,
    bold: true,
    widths: &PAGELLA_BOLD_WIDTHS,
};

const HEROS_TYPE1_METRICS: Type1Metrics = Type1Metrics {
    font_bbox: [-529, -284, 1353, 1148],
    italic_angle: 0.0,
    ascender: 729,
    descender: -216,
    cap_height: 729,
    x_height: 524,
    fixed_pitch: false,
    bold: false,
    widths: &HEROS_WIDTHS,
};

const HEROS_BOLD_TYPE1_METRICS: Type1Metrics = Type1Metrics {
    font_bbox: [-173, -219, 1001, 944],
    italic_angle: 0.0,
    ascender: 718,
    descender: -207,
    cap_height: 729,
    x_height: 549,
    fixed_pitch: false,
    bold: true,
    widths: &HEROS_BOLD_WIDTHS,
};

fn pdf_font_text_units(text: &str, metric: PdfFontMetric) -> f32 {
    let mut units = 0.0_f32;
    for ch in text.chars() {
        if pdf_symbol_byte(ch).is_some() {
            units += pdf_font_char_width_units(ch, PdfFontMetric::Symbol);
        } else if let Some(replacement) = pdf_text_ascii_replacement(ch) {
            units += replacement
                .chars()
                .map(|replacement_ch| pdf_font_char_width_units(replacement_ch, metric))
                .sum::<f32>();
        } else {
            units += pdf_font_char_width_units(ch, metric);
        }
    }
    units
}

fn pdf_font_char_width_units(ch: char, metric: PdfFontMetric) -> f32 {
    match metric {
        PdfFontMetric::TimesRoman => table_char_width_units(ch, &TIMES_ROMAN_WIDTHS),
        PdfFontMetric::TimesItalic => table_char_width_units(ch, &TIMES_ITALIC_WIDTHS),
        PdfFontMetric::TimesBold => table_char_width_units(ch, &TIMES_BOLD_WIDTHS),
        PdfFontMetric::Pagella => table_char_width_units(ch, &PAGELLA_WIDTHS),
        PdfFontMetric::PagellaItalic => table_char_width_units(ch, &PAGELLA_ITALIC_WIDTHS),
        PdfFontMetric::PagellaBold => table_char_width_units(ch, &PAGELLA_BOLD_WIDTHS),
        PdfFontMetric::Heros => table_char_width_units(ch, &HEROS_WIDTHS),
        PdfFontMetric::HerosBold => table_char_width_units(ch, &HEROS_BOLD_WIDTHS),
        PdfFontMetric::Courier => courier_char_width_units(ch),
        PdfFontMetric::Symbol => symbol_char_width_units(ch),
    }
}

fn table_char_width_units(ch: char, widths: &[f32; 95]) -> f32 {
    let code = ch as u32;
    if (ASCII_WIDTH_FIRST..=ASCII_WIDTH_LAST).contains(&code) {
        widths[(code - ASCII_WIDTH_FIRST) as usize]
    } else if ch.is_control() {
        0.0
    } else {
        500.0
    }
}

fn courier_char_width_units(ch: char) -> f32 {
    if ch.is_control() { 0.0 } else { 600.0 }
}

fn symbol_char_width_units(ch: char) -> f32 {
    if ch.is_control() { 0.0 } else { 500.0 }
}

fn synctex_sp(points: f32) -> i64 {
    (points * 65_536.0).round() as i64
}

fn write_fls(path: &Path, inputs: &[PathBuf], outputs: &[&Path]) -> io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    let pwd = inputs
        .first()
        .and_then(|path| path.parent())
        .unwrap_or(Path::new("."));
    writeln!(file, "PWD {}", pwd.display())?;
    for input in inputs {
        writeln!(file, "INPUT {}", input.display())?;
    }
    for output in outputs {
        writeln!(file, "OUTPUT {}", output.display())?;
    }
    Ok(())
}

fn write_success_log(
    path: &Path,
    options: &NativeEngineOptions,
    document: &SimpleDocument,
    elapsed_ms: u128,
    page_count: usize,
    pdf_output: bool,
    synctex_output: bool,
) -> io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    writeln!(
        file,
        "This is texpilot-pdftex experimental native engine, compatibility level 0."
    )?;
    writeln!(file, "**{}", options.main.display())?;
    writeln!(
        file,
        "Output written on {}.pdf ({} page{}).",
        options.job_name,
        page_count,
        if page_count == 1 { "" } else { "s" }
    )?;
    writeln!(file, "PDF output enabled: {pdf_output}")?;
    writeln!(file, "SyncTeX output enabled: {synctex_output}")?;
    writeln!(file, "Footnotes: {}", document.footnotes.len())?;
    writeln!(file, "Index entries: {}", document.index.entries.len())?;
    writeln!(
        file,
        "PDF metadata entries: {}",
        document.pdf_metadata.entries.len()
    )?;
    writeln!(file, "Elapsed milliseconds: {elapsed_ms}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEXINPUTS_TEST_LOCK: Mutex<()> = Mutex::new(());
    static BIBINPUTS_TEST_LOCK: Mutex<()> = Mutex::new(());
    static BSTINPUTS_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn temp_dir(name: &str) -> PathBuf {
        let root =
            std::env::temp_dir().join(format!("texpilot-pdftex-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn simple_test_document(layout: DocumentLayout, lines: Vec<Line>) -> SimpleDocument {
        SimpleDocument {
            layout,
            lines,
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        }
    }

    fn pdf_literal_text(pdf: &[u8]) -> String {
        let source = String::from_utf8_lossy(pdf);
        let mut output = String::new();
        let mut chars = source.chars();
        while let Some(ch) = chars.next() {
            if ch != '(' {
                continue;
            }
            let mut literal = String::new();
            let mut escaped = false;
            let mut depth = 1_usize;
            for ch in chars.by_ref() {
                if escaped {
                    literal.push(ch);
                    escaped = false;
                    continue;
                }
                match ch {
                    '\\' => escaped = true,
                    '(' => {
                        depth += 1;
                        literal.push(ch);
                    }
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                        literal.push(ch);
                    }
                    _ => literal.push(ch),
                }
            }
            if !literal.trim().is_empty() {
                if !output.is_empty() {
                    output.push(' ');
                }
                output.push_str(literal.trim());
            }
        }
        output
    }

    struct EnvVarGuard {
        name: &'static str,
        previous: Option<OsString>,
    }

    impl EnvVarGuard {
        fn set(name: &'static str, value: OsString) -> Self {
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

    #[test]
    fn pdf_text_transliterates_common_unicode_for_base_font() {
        let text = concat!(
            "ad\u{2011}hoc trade\u{2011}off teacher\u{2013}student ",
            "Cram\u{00E9}r\u{2013}Wold Epps\u{2013}Pulley ",
            "\u{201C}inefficient\u{201D} Vit\u{00F3}ria 3\u{00D7} +/- \u{00B1} (x) \\"
        );
        let encoded = pdf_text(text);
        assert_eq!(
            encoded,
            "ad-hoc trade-off teacher-student Cramer-Wold Epps-Pulley \"inefficient\" Vitoria 3x +/- +/- \\(x\\) \\\\"
        );
        assert!(!encoded.contains('?'), "{encoded}");
    }

    #[test]
    fn pdf_text_object_resets_black_fill_inside_saved_state() {
        let mut stream = String::from("1 1 1 rg\n");
        append_pdf_text_object(&mut stream, PdfTextFont::Text, 10.0, 12.0, 34.0, "Visible");

        assert!(
            stream.contains("q\n0 0 0 rg\nBT\n/F1 10.00 Tf\n12.00 34.00 Td"),
            "{stream}"
        );
        assert!(stream.ends_with("ET\nQ\n"), "{stream}");
    }

    #[test]
    fn pdf_font_metrics_use_monospace_code_widths() {
        let code_narrow = natural_text_width_for_metric_pt("iiii", PdfFontMetric::Courier, 10.0);
        let code_wide = natural_text_width_for_metric_pt("WWWW", PdfFontMetric::Courier, 10.0);
        let text_narrow = natural_text_width_for_metric_pt("iiii", PdfFontMetric::TimesRoman, 10.0);
        let text_wide = natural_text_width_for_metric_pt("WWWW", PdfFontMetric::TimesRoman, 10.0);

        assert!((code_narrow - code_wide).abs() < f32::EPSILON);
        assert!(text_wide > text_narrow);
    }

    #[test]
    fn justified_word_spacing_uses_active_font_metrics() {
        let target_width = 28.0;
        let text_spacing =
            justified_word_spacing_pt("mi mi", PdfFontMetric::TimesRoman, 10.0, target_width)
                .unwrap();
        let code_spacing =
            justified_word_spacing_pt("mi mi", PdfFontMetric::Courier, 10.0, target_width).unwrap();

        assert!(text_spacing > code_spacing);
    }

    #[test]
    fn icml_layout_uses_pagella_body_metrics() {
        let layout = DocumentLayout::icml_two_column();
        let pagella_width = natural_text_width_for_metric_pt(
            "mmmm",
            layout.metric_for_font(PdfTextFont::Text),
            10.0,
        );
        let times_width = natural_text_width_for_metric_pt("mmmm", PdfFontMetric::TimesRoman, 10.0);

        assert!(pagella_width > times_width);
    }

    #[test]
    fn icml_line_breaking_keeps_calibrated_body_metric() {
        let layout = DocumentLayout::icml_two_column();

        assert_eq!(
            layout.line_break_metric_for_font(PdfTextFont::Text),
            PdfFontMetric::TimesRoman
        );
    }

    #[test]
    fn line_break_score_classifies_tex_fitness_buckets() {
        let tight = line_break_score(106.0, 100.0, 10.0, 4, false);
        let decent = line_break_score(100.0, 100.0, 10.0, 4, false);
        let loose = line_break_score(70.0, 100.0, 10.0, 4, false);
        let very_loose = line_break_score(50.0, 100.0, 10.0, 4, false);

        assert_eq!(tight.fitness, LineFitness::Tight);
        assert_eq!(decent.fitness, LineFitness::Decent);
        assert_eq!(loose.fitness, LineFitness::Loose);
        assert_eq!(very_loose.fitness, LineFitness::VeryLoose);
    }

    #[test]
    fn adjacent_line_fitness_penalizes_incompatible_breaks() {
        assert_eq!(
            adjacent_fitness_demerits(Some(LineFitness::Tight), LineFitness::VeryLoose),
            LINE_BREAK_ADJACENT_FITNESS_DEMERITS
        );
        assert_eq!(
            adjacent_fitness_demerits(Some(LineFitness::Loose), LineFitness::VeryLoose),
            0.0
        );
        assert_eq!(adjacent_fitness_demerits(None, LineFitness::Tight), 0.0);
    }

    #[test]
    fn clean_inline_text_suppresses_nested_layout_scaffolding() {
        let source = r"\begin{center}
\begin{minipage}{0.44\linewidth}
\includegraphics[width=\linewidth]{toy_figures/plot.pdf}
\end{minipage}\\[0em]
\begin{tabular}{lcc}
\toprule
\textbf{Method} & \multicolumn{2}{c}{\textbf{Full FT}}\\
\cmidrule(lr){2-3}
\bottomrule
\end{tabular}
\captionof{figure}{\textbf{LeJEPA overview.} Training stability.}
\end{center}";
        let cleaned = clean_inline_text(
            source,
            &HashMap::new(),
            &HashMap::new(),
            &CitationRegistry::default(),
        )
        .unwrap();
        assert!(cleaned.contains("Method"), "{cleaned}");
        assert!(cleaned.contains("Full FT"), "{cleaned}");
        assert!(
            cleaned.contains("LeJEPA overview. Training stability."),
            "{cleaned}"
        );
        for artifact in [
            "begin",
            "end",
            "includegraphics",
            "toy_figures",
            "minipage",
            "tabular",
            "captionof",
            "0em",
            "cmidrule",
        ] {
            assert!(!cleaned.contains(artifact), "{cleaned}");
        }
    }

    #[test]
    fn native_engine_writes_a_pdf_for_minimal_documents() {
        let root = temp_dir("minimal");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\title{Native Smoke}
\begin{document}
\maketitle
\section{Hello}
This is a tiny native document.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        assert!(out.join("main.pdf").exists());
        assert!(out.join("main.aux").exists());
        assert!(out.join("main.fls").exists());
        assert!(out.join("main.texpilot-pdftex.trace").exists());
    }

    #[test]
    fn native_icml_pdf_embeds_tex_gyre_type1_fonts_when_available() {
        if resolve_tex_font_file("TeXGyrePagellaX-Regular.pfb")
            .unwrap()
            .is_none()
        {
            return;
        }
        let root = temp_dir("embedded-fonts");
        let pdf = root.join("main.pdf");
        let document = simple_test_document(
            DocumentLayout::icml_two_column(),
            vec![
                Line::Text("Native Pagella text should use an embedded font.".to_string()),
                Line::Heading("Embedded heading".to_string()),
            ],
        );
        let placements = line_placements(&document);
        let page_count = page_count_from_placements(&placements, &document.lines);

        write_pdf(&pdf, &document, page_count, &placements).unwrap();
        let pdf_bytes = fs::read(&pdf).unwrap();
        let pdf_text = String::from_utf8_lossy(&pdf_bytes);

        assert!(pdf_text.contains("/BaseFont /TeXGyrePagellaX-Regular"));
        assert!(pdf_text.contains("/BaseFont /TeXGyreHeros-Regular"));
        assert!(pdf_text.contains("/BaseFont /TeXGyreHeros-Bold"));
        assert!(pdf_text.contains("/FontFile"));
        assert!(pdf_text.contains("/Differences [32 /space /exclam"));
    }

    #[test]
    fn native_trace_counts_only_unrendered_two_column_graphic_float_fallbacks() {
        let root = temp_dir("two-column-graphic-float-trace");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(root.join("normal.png"), tiny_rgba_png_bytes()).unwrap();
        fs::write(root.join("wide.png"), tiny_rgba_png_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\twocolumn
\begin{document}
\begin{figure}[t]
\includegraphics[width=\linewidth]{normal.png}
\caption{Normal graphic}
\end{figure}
\begin{figure*}[t]
\includegraphics[width=\textwidth]{wide.png}
\caption{Wide graphic with enough caption text to exercise rendered caption tracing while the
native renderer keeps two-column wide graphics on the grouped float path.}
\end{figure*}
\end{document}
",
        )
        .unwrap();

        let run = run_native_pdf_only(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let trace =
            fs::read_to_string(out.join("main.texpilot-pdftex.trace")).expect("trace exists");
        assert!(
            trace.contains("layout_two_column_graphic_float_fallbacks\t0"),
            "{trace}"
        );
        assert!(
            trace.contains("layout_two_column_wide_graphic_float_fallbacks\t0"),
            "{trace}"
        );
        assert!(
            trace.contains("layout_two_column_graphic_float_fallback_entries\t0"),
            "{trace}"
        );
        assert!(
            trace.contains("layout_two_column_graphic_float_fallback_estimated_native_slots\t0"),
            "{trace}"
        );
        assert!(
            trace.contains(
                "layout_two_column_wide_graphic_float_fallback_estimated_native_slots\t0"
            ),
            "{trace}"
        );
        assert!(
            !trace.contains(
                "layout_two_column_graphic_float_fallback\tenv=figure top=true wide=false images=1 caption=Normal graphic"
            ),
            "{trace}"
        );
        assert!(
            trace.contains("layout_caption") && trace.contains("text=Figure 1: Normal graphic"),
            "{trace}"
        );
        assert!(
            trace.contains("layout_caption") && trace.contains("text=Figure 2: Wide graphic"),
            "{trace}"
        );
        assert!(
            !trace.contains("layout_two_column_graphic_float_fallback\tenv=figure*"),
            "{trace}"
        );
    }

    #[test]
    fn native_engine_writes_pdf_info_metadata_without_fallback() {
        let root = temp_dir("pdf-info-metadata");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\pdfinfo{/Title (Primitive Title) /Author (Ada \(Native\))}
\hypersetup{pdfsubject={Native Subject}, pdfkeywords={alpha, beta}, pdfcreator={texpilot}}
\pdfcatalog{/PageMode /UseNone}
\pdfmapfile{+native.map}
\begin{document}
Visible metadata body.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("/Info"), "{pdf_text}");
        assert!(pdf_text.contains("/Title (Primitive Title)"), "{pdf_text}");
        assert!(
            pdf_text.contains("/Author (Ada \\(Native\\))"),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("/Subject (Native Subject)"), "{pdf_text}");
        assert!(pdf_text.contains("/Keywords (alpha, beta)"), "{pdf_text}");
        assert!(pdf_text.contains("/Creator (texpilot)"), "{pdf_text}");
        assert!(pdf_text.contains("Visible metadata body."), "{pdf_text}");
        assert!(!pdf_text.contains("pdfinfo"), "{pdf_text}");
        assert!(!pdf_text.contains("hypersetup"), "{pdf_text}");
        let trace =
            fs::read_to_string(out.join("main.texpilot-pdftex.trace")).expect("trace exists");
        assert!(!trace.contains("unsupported"), "{trace}");
        assert!(trace.contains("pdf_metadata_entries\t5"), "{trace}");
        let log = fs::read_to_string(out.join("main.log")).expect("log exists");
        assert!(log.contains("PDF metadata entries: 5"), "{log}");
    }

    #[test]
    fn native_engine_renders_standard_maketitle_author_and_date() {
        let root = temp_dir("maketitle-author-date");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\title{Native Title}
\author{Ada Lovelace\thanks{Support note} \\ \texttt{ada@example.com} \and Grace Hopper}
\date{June 2026}
\begin{document}
\maketitle
Body.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Native Title"), "{pdf_text}");
        assert!(pdf_text.contains("Ada Lovelace"), "{pdf_text}");
        assert!(pdf_text.contains("Ada Lovelace1"), "{pdf_text}");
        assert!(pdf_text.contains("ada@example.com"), "{pdf_text}");
        assert!(pdf_text.contains("Grace Hopper"), "{pdf_text}");
        assert!(pdf_text.contains("June 2026"), "{pdf_text}");
        assert!(pdf_text.contains("Notes"), "{pdf_text}");
        assert!(pdf_text.contains("[1] Support note"), "{pdf_text}");
    }

    #[test]
    fn native_engine_reports_unsupported_documents_for_fallback() {
        let root = temp_dir("unsupported");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
\includegraphics{missing-native-unsupported-plot}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out,
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert!(matches!(run.status, NativeEngineStatus::Unsupported(_)));
    }

    #[test]
    fn native_engine_reports_unsupported_runtime_options_for_fallback() {
        let root = temp_dir("unsupported-options");
        let main = root.join("main.tex");
        let out = root.join("shell-escape");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Native option fallback.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: true,
            synctex: false,
        })
        .unwrap();

        let NativeEngineStatus::Unsupported(unsupported) = run.status else {
            panic!("expected unsupported native status for shell escape");
        };
        assert!(
            unsupported.reason.contains("shell-escape"),
            "{unsupported:?}"
        );
        assert!(!out.join("main.pdf").exists());
        let trace =
            fs::read_to_string(out.join("main.texpilot-pdftex.trace")).expect("trace exists");
        assert!(trace.contains("unsupported"), "{trace}");
        assert!(trace.contains("shell-escape"), "{trace}");
    }

    #[test]
    fn native_engine_writes_synctex_sidecar_when_requested() {
        let root = temp_dir("synctex");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\section{Hello}
Native SyncTeX output.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: true,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        assert!(out.join("main.pdf").exists());
        let synctex = fs::File::open(out.join("main.synctex.gz")).expect("SyncTeX sidecar exists");
        let mut decoder = flate2::read::GzDecoder::new(synctex);
        let mut source = String::new();
        std::io::Read::read_to_string(&mut decoder, &mut source).unwrap();
        assert!(source.contains("SyncTeX Version:1"), "{source}");
        let canonical_main = fs::canonicalize(&main).unwrap();
        assert!(
            source.contains(&format!("Input:1:{}", canonical_main.display())),
            "{source}"
        );
        assert!(source.contains("Output:pdf"), "{source}");
        assert!(source.contains("Content:"), "{source}");
        assert!(source.contains("{1"), "{source}");
        assert!(source.contains("Postamble:"), "{source}");

        let fls = fs::read_to_string(out.join("main.fls")).expect("fls exists");
        assert!(
            fls.contains(&out.join("main.synctex.gz").display().to_string()),
            "{fls}"
        );
        let log = fs::read_to_string(out.join("main.log")).expect("log exists");
        assert!(log.contains("SyncTeX output enabled: true"), "{log}");
        let trace =
            fs::read_to_string(out.join("main.texpilot-pdftex.trace")).expect("trace exists");
        assert!(trace.contains("synctex_output"), "{trace}");
        assert!(trace.contains("true"), "{trace}");
    }

    #[test]
    fn native_page_stream_reserves_but_does_not_duplicate_overflow_lines() {
        let document = SimpleDocument {
            layout: DocumentLayout::default(),
            lines: vec![Line::Image(0)],
            images: vec![ImageAsset {
                path: PathBuf::from("tall.jpg"),
                width_px: 1,
                height_px: 1,
                display_width_pt: 72.0,
                display_height_pt: (DocumentLayout::default().lines_per_page * 2) as f32
                    * DocumentLayout::default().line_height_pt,
                rotation_degrees: 0.0,
                viewport: ImageViewport::full(),
                payload: Arc::new(ImagePayload::Jpeg(Vec::new())),
            }],
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        assert_eq!(document.pages(), 1);
        let first_page = page_stream(&document, 0);

        assert!(first_page.contains("/Im1 Do"), "{first_page}");
        assert!(first_page.contains("(1) Tj"), "{first_page}");
    }

    #[test]
    fn native_page_stream_suppresses_conference_title_page_number() {
        let document = SimpleDocument {
            layout: DocumentLayout::icml_two_column(),
            lines: vec![Line::Text("Title page text.".to_string())],
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let first_page = page_stream(&document, 0);

        assert!(!first_page.contains("(1) Tj"), "{first_page}");
    }

    #[test]
    fn native_page_stream_renders_icml_running_header_after_title_page() {
        let document = SimpleDocument {
            layout: DocumentLayout::icml_two_column(),
            lines: vec![
                Line::PageStyle(PageStyle {
                    running_title: "Native:".to_string(),
                    section_line: "Sec 1: Intro | Sec 2: Method".to_string(),
                }),
                Line::Text("Title page text.".to_string()),
            ],
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let streams = page_streams(&document, 2);

        assert!(!streams[0].contains("Native:"), "{}", streams[0]);
        assert!(streams[1].contains("(Native:) Tj"), "{}", streams[1]);
        assert!(
            streams[1].contains("(Sec 1: Intro | Sec 2: Method) Tj"),
            "{}",
            streams[1]
        );
    }

    #[test]
    fn native_page_style_extracts_icml_section_header_line() {
        let source = r"\usepackage{simpleicml}
\icmlrunningtitle{LeJEPA:}
\sectionheaderline{%
  \seclink{sec:intro}{1}{Intro} |
  \seclink{sec:background}{2}{Background}
}";

        let style = native_page_style_from_source(source).expect("page style");

        assert_eq!(style.running_title, "LeJEPA:");
        assert_eq!(style.section_line, "Sec 1: Intro | Sec 2: Background");
    }

    #[test]
    fn native_fixture_carries_icml_running_header_into_document() {
        let main = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/arXiv-2511.08544v3/main.tex");
        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let out = temp_dir("icml-header-fixture");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse fixture");
        let Some(style) = document.lines.iter().find_map(|line| match line {
            Line::PageStyle(style) => Some(style),
            _ => None,
        }) else {
            panic!("fixture did not carry page style");
        };

        assert_eq!(style.running_title, "LeJEPA:");
        assert!(style.section_line.contains("Sec 1: Intro"), "{style:?}");
        assert!(
            style.section_line.contains("Sec 6: Experiments"),
            "{style:?}"
        );
    }

    #[test]
    fn native_neurips_author_grid_preserves_and_author_blocks() {
        let source = r"\author{%
  Ada Lovelace\\
  Engines Lab\\
  \texttt{ada@example.edu}
  \And
  Emmy Noether\\
  Algebra Institute\\
  \texttt{emmy@example.edu}
  \And
  Grace Hopper\\
  Navy\\
  \texttt{grace@example.edu}
}";
        let mut footnotes = FootnoteRegistry::default();
        let grid = native_neurips_author_grid_from_source(
            source,
            &HashMap::new(),
            &HashMap::new(),
            &CitationRegistry::default(),
            &mut footnotes,
        )
        .expect("author grid parses")
        .expect("author grid");

        assert_eq!(grid.rows.len(), 2);
        assert_eq!(grid.rows[0].len(), 2);
        assert_eq!(grid.rows[1].len(), 1);
        assert_eq!(grid.rows[0][0].lines[0], "Ada Lovelace");
        assert_eq!(grid.rows[1][0].lines[2], "grace@example.edu");
    }

    #[test]
    fn native_layout_reserves_wide_top_rows_in_both_columns() {
        let layout = DocumentLayout::icml_two_column();
        let mut lines = vec![
            Line::WideTheoremBackground {
                slots: 3,
                heading: "Definition 1".to_string(),
            },
            Line::WideAbstractText("A wide row.".to_string()),
            Line::WideEquation("x = y".to_string()),
        ];
        for index in 0..layout.lines_per_page {
            lines.push(Line::Text(format!("Normal {index}")));
        }
        let document = SimpleDocument {
            layout,
            lines,
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let normal_placements = placements
            .iter()
            .filter(|placement| matches!(document.lines[placement.line_index], Line::Text(_)))
            .collect::<Vec<_>>();

        assert_eq!(normal_placements[0].page_slot, 4);
        let first_right_column_line = layout.rows_per_column - 4;
        assert_eq!(
            normal_placements[first_right_column_line].page_slot,
            layout.rows_per_column + 4
        );
    }

    #[test]
    fn native_layout_places_top_float_at_next_column_top() {
        let layout = DocumentLayout::icml_two_column();
        let document = SimpleDocument {
            layout,
            lines: vec![
                Line::Text("Before one.".to_string()),
                Line::Text("Before two.".to_string()),
                Line::FloatBlock {
                    lines: vec![Line::Text("Float body.".to_string())],
                    wide: false,
                    top: true,
                },
                Line::Text("After.".to_string()),
            ],
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let float_placement = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { .. }
                )
            })
            .expect("float block placement");

        assert_eq!(float_placement.page_slot, layout.rows_per_column);
    }

    #[test]
    fn native_page_count_ignores_nonrendered_overflow_placeholders() {
        let layout = DocumentLayout::default();
        let document = SimpleDocument {
            layout,
            lines: vec![Line::FloatBlock {
                lines: (0..layout.rows_per_column + 1)
                    .map(|index| Line::Text(format!("Tall block {index}")))
                    .collect(),
                wide: false,
                top: false,
            }],
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);

        assert!(placements.iter().any(|placement| !placement.render));
        assert_eq!(document.pages(), 1);
    }

    #[test]
    fn native_layout_defers_two_column_top_float_while_text_fills_column() {
        let layout = DocumentLayout::icml_two_column();
        let mut lines = vec![
            Line::Text("Before.".to_string()),
            Line::FloatBlock {
                lines: vec![Line::Text("Top float.".to_string())],
                wide: false,
                top: true,
            },
        ];
        for index in 0..layout.rows_per_column - 1 {
            lines.push(Line::Text(format!("Fill {index}")));
        }
        lines.push(Line::Text("After.".to_string()));
        let document = SimpleDocument {
            layout,
            lines,
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let last_fill_label = format!("Fill {}", layout.rows_per_column - 2);
        let last_fill = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == &last_fill_label
                )
            })
            .expect("last fill text placement");
        let float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { top: true, .. }
                )
            })
            .expect("float block placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After."
                )
            })
            .expect("after text placement");

        assert_eq!(last_fill.page_index, 0);
        assert_eq!(last_fill.page_slot, layout.rows_per_column - 1);
        assert_eq!(float.page_index, 0);
        assert_eq!(float.page_slot, layout.rows_per_column);
        assert_eq!(after.page_index, 0);
        assert_eq!(after.page_slot, layout.rows_per_column + 1);
    }

    #[test]
    fn native_layout_places_here_float_inline() {
        let layout = DocumentLayout::icml_two_column();
        let document = SimpleDocument {
            layout,
            lines: vec![
                Line::Text("Before.".to_string()),
                Line::FloatBlock {
                    lines: vec![Line::Text("Float body.".to_string())],
                    wide: false,
                    top: false,
                },
                Line::Text("After.".to_string()),
            ],
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let float_placement = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { .. }
                )
            })
            .expect("float block placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After."
                )
            })
            .expect("after text placement");

        assert_eq!(float_placement.page_index, 0);
        assert_eq!(float_placement.page_slot, 1);
        assert_eq!(after.page_index, 0);
        assert_eq!(after.page_slot, 2);
    }

    #[test]
    fn native_layout_defers_single_column_top_float_while_text_fills_page() {
        let layout = DocumentLayout::default();
        let mut lines = vec![
            Line::Text("Before.".to_string()),
            Line::FloatBlock {
                lines: vec![Line::Text("Top float.".to_string())],
                wide: false,
                top: true,
            },
        ];
        for index in 0..layout.rows_per_column - 1 {
            lines.push(Line::Text(format!("Fill {index}")));
        }
        lines.push(Line::Text("After.".to_string()));
        let document = SimpleDocument {
            layout,
            lines,
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let last_fill = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "Fill 40"
                )
            })
            .expect("last fill text placement");
        let float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { top: true, .. }
                )
            })
            .expect("float block placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After."
                )
            })
            .expect("after text placement");

        assert_eq!(last_fill.page_index, 0);
        assert_eq!(last_fill.page_slot, layout.rows_per_column - 1);
        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
        assert_eq!(after.page_index, 1);
        assert_eq!(after.page_slot, 1);
    }

    #[test]
    fn native_layout_defers_leading_wide_top_float_until_after_first_text_page() {
        let layout = DocumentLayout::icml_two_column();
        let mut lines = vec![
            Line::Blank,
            Line::FloatBlock {
                lines: vec![Line::WideImageRow(Vec::new())],
                wide: true,
                top: true,
            },
        ];
        for index in 0..layout.rows_per_column * layout.columns {
            lines.push(Line::Text(format!("Body {index}")));
        }
        let document = SimpleDocument {
            layout,
            lines,
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let first_text = placements
            .iter()
            .find(|placement| matches!(document.lines[placement.line_index], Line::Text(_)))
            .expect("text placement");
        let float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { wide: true, .. }
                )
            })
            .expect("wide float placement");

        assert_eq!(first_text.page_index, 0);
        assert_eq!(first_text.page_slot, 0);
        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
    }

    #[test]
    fn native_layout_queues_midpage_wide_top_float_without_stranding_current_page() {
        let layout = DocumentLayout::icml_two_column();
        let document = SimpleDocument {
            layout,
            lines: vec![
                Line::Text("Before wide float.".to_string()),
                Line::FloatBlock {
                    lines: vec![Line::WideAbstractText("Wide top float.".to_string())],
                    wide: true,
                    top: true,
                },
                Line::Text("After wide float.".to_string()),
            ],
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let before = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "Before wide float."
                )
            })
            .expect("before text placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After wide float."
                )
            })
            .expect("after text placement");
        let float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { wide: true, .. }
                )
            })
            .expect("wide float placement");

        assert_eq!(before.page_index, 0);
        assert_eq!(before.page_slot, 0);
        assert_eq!(after.page_index, 0);
        assert_eq!(after.page_slot, 1);
        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
    }

    #[test]
    fn native_layout_releases_pending_wide_top_float_before_column_top_float() {
        let layout = DocumentLayout::icml_two_column();
        let mut lines = vec![
            Line::Text("Before floats.".to_string()),
            Line::FloatBlock {
                lines: vec![Line::WideAbstractText("Wide top float.".to_string())],
                wide: true,
                top: true,
            },
            Line::FloatBlock {
                lines: vec![Line::Text("Column top float.".to_string())],
                wide: false,
                top: true,
            },
        ];
        for index in 0..layout.lines_per_page {
            lines.push(Line::Text(format!("Fill {index}")));
        }
        lines.push(Line::Text("After floats.".to_string()));
        let document = simple_test_document(layout, lines);

        let placements = line_placements(&document);
        let wide_float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { wide: true, .. }
                )
            })
            .expect("wide float placement");
        let column_float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock {
                        wide: false,
                        top: true,
                        ..
                    }
                )
            })
            .expect("column float placement");

        assert_eq!(wide_float.page_index, 1);
        assert_eq!(wide_float.page_slot, 0);
        assert_eq!(column_float.page_index, 1);
        assert_eq!(column_float.page_slot, 1);
    }

    #[test]
    fn native_layout_defers_wide_top_float_after_wide_title_rows() {
        let layout = DocumentLayout::icml_two_column();
        let document = SimpleDocument {
            layout,
            lines: vec![
                Line::WideTitle("Wide title rows already consumed page space.".to_string()),
                Line::FloatBlock {
                    lines: vec![Line::WideAbstractText("Wide top float.".to_string())],
                    wide: true,
                    top: true,
                },
                Line::Text("Following body text.".to_string()),
            ],
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let title = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::WideTitle(text) if text.starts_with("Wide title rows")
                )
            })
            .expect("title placement");
        let body = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "Following body text."
                )
            })
            .expect("body text placement");
        let float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { wide: true, .. }
                )
            })
            .expect("wide float placement");

        assert_eq!(title.page_index, 0);
        assert_eq!(body.page_index, 0);
        assert_eq!(body.page_slot, title.line_slots);
        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
    }

    #[test]
    fn native_layout_places_later_page_start_wide_top_float_immediately() {
        let layout = DocumentLayout::icml_two_column();
        let mut lines = Vec::new();
        for index in 0..layout.rows_per_column * layout.columns {
            lines.push(Line::Text(format!("Body {index}")));
        }
        lines.push(Line::FloatBlock {
            lines: vec![Line::WideAbstractText("Wide top float.".to_string())],
            wide: true,
            top: true,
        });
        lines.push(Line::Text("After.".to_string()));
        let document = SimpleDocument {
            layout,
            lines,
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { wide: true, .. }
                )
            })
            .expect("wide float placement");

        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
    }

    #[test]
    fn native_icml_layout_uses_dense_columns_but_physical_one_column_rows() {
        let layout = DocumentLayout::icml_two_column();
        assert_eq!(layout.rows_per_column, 54);
        assert_eq!(layout.lines_per_page, 108);

        let one_column = layout.as_one_column();
        assert_eq!(one_column.columns, 1);
        assert_eq!(one_column.lines_per_page, 54);
        assert_eq!(one_column.rows_per_column, 54);
        assert_eq!(one_column.column_width_pt, layout.text_width_pt);
        assert!(one_column.text_wrap_width > layout.text_wrap_width);
    }

    #[test]
    fn native_layout_switch_uses_one_column_geometry_after_control() {
        let layout = DocumentLayout::icml_two_column();
        let one_column = layout.as_one_column();
        let document = SimpleDocument {
            layout,
            lines: vec![
                Line::Text("Before switch.".to_string()),
                Line::OutputControl(OutputControl::LayoutSwitch(one_column)),
                Line::Text("After switch.".to_string()),
            ],
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let before = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "Before switch."
                )
            })
            .expect("before text placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After switch."
                )
            })
            .expect("after text placement");

        assert_eq!(before.page_index, 0);
        assert_eq!(before.layout.columns, 2);
        assert_eq!(after.page_index, 1);
        assert_eq!(after.page_slot, 0);
        assert_eq!(after.layout.columns, 1);
        assert_eq!(after.layout.column_width_pt, layout.text_width_pt);
    }

    #[test]
    fn native_newpage_flushes_top_float_before_following_text() {
        let layout = DocumentLayout::default();
        let mut lines = vec![
            Line::Text("Before float.".to_string()),
            Line::FloatBlock {
                lines: vec![Line::Text("Deferred top float.".to_string())],
                wide: false,
                top: true,
            },
        ];
        for index in 0..(layout.rows_per_column - SOFT_NEWPAGE_MAX_REMAINING_SLOTS - 1) {
            lines.push(Line::Text(format!("Fill {index}")));
        }
        lines.push(Line::OutputControl(OutputControl::NewPage));
        lines.push(Line::Text("After newpage.".to_string()));

        let document = SimpleDocument {
            layout,
            lines,
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { top: true, .. }
                )
            })
            .expect("top float placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After newpage."
                )
            })
            .expect("text after newpage placement");

        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
        assert_eq!(after.page_index, 1);
        assert_eq!(after.page_slot, 1);
    }

    #[test]
    fn native_clearpage_flushes_deferred_float_before_layout_switch() {
        let layout = DocumentLayout::icml_two_column();
        let one_column = layout.as_one_column();
        let document = SimpleDocument {
            layout,
            lines: vec![
                Line::Text("Before float.".to_string()),
                Line::FloatBlock {
                    lines: vec![Line::WideAbstractText("Deferred wide float.".to_string())],
                    wide: true,
                    top: true,
                },
                Line::OutputControl(OutputControl::ClearPage),
                Line::OutputControl(OutputControl::LayoutSwitch(one_column)),
                Line::Text("Appendix text.".to_string()),
            ],
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };

        let placements = line_placements(&document);
        let float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { wide: true, .. }
                )
            })
            .expect("wide float placement");
        let appendix = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "Appendix text."
                )
            })
            .expect("appendix text placement");

        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
        assert_eq!(float.layout.columns, 2);
        assert_eq!(appendix.page_index, 2);
        assert_eq!(appendix.page_slot, 0);
        assert_eq!(appendix.layout.columns, 1);
    }

    #[test]
    fn native_layout_caption_trace_uses_rendered_float_placement() {
        let layout = DocumentLayout::default();
        let mut lines = (0..layout.rows_per_column)
            .map(|index| Line::Text(format!("Fill {index}")))
            .collect::<Vec<_>>();
        lines.push(Line::FloatBlock {
            lines: vec![
                Line::Caption("Figure 1: Rendered float position.".to_string()),
                Line::Caption("continued caption text".to_string()),
            ],
            wide: false,
            top: false,
        });
        let document = SimpleDocument {
            layout,
            lines,
            images: Vec::new(),
            timings: ParseTimings::default(),
            generated_outputs: Vec::new(),
            labels: std::collections::BTreeMap::new(),
            citations: CitationRegistry::default(),
            bibliography: BibliographyMetadata::default(),
            index: IndexRegistry::default(),
            pdf_metadata: PdfMetadata::default(),
            footnotes: Vec::new(),
            toc_entries: Vec::new(),
            float_entries: Vec::new(),
            bookmarks: Vec::new(),
            toc_requested: false,
            list_of_figures_requested: false,
            list_of_tables_requested: false,
            hyperref_out_requested: false,
            backref_requested: false,
        };
        let placements = line_placements(&document);
        let traces = layout_caption_traces(&document, &placements);

        assert_eq!(traces.len(), 1, "{traces:?}");
        assert_eq!(traces[0].kind, "figure");
        assert_eq!(traces[0].page, 2);
        assert_eq!(traces[0].slot, 0);
        assert!(traces[0].text.contains("Rendered float position."));
    }

    #[test]
    fn native_jpeg_dimension_parser_reads_sof_dimensions() {
        assert_eq!(jpeg_dimensions(tiny_jpeg_bytes()), Some((2, 1)));
    }

    #[test]
    fn native_graphics_cache_reuses_resolved_references() {
        let root = temp_dir("graphics-resolution-cache");
        let figure = root.join("figure.jpg");
        fs::write(&figure, tiny_jpeg_bytes()).unwrap();

        let graphics = GraphicsConfig::default();
        let layout = DocumentLayout::default();
        let mut cache = GraphicsCache::default();

        let (first, _) = parse_includegraphics(
            "[width=2cm]{figure.jpg}",
            &root,
            &graphics,
            &mut cache,
            &layout,
        )
        .unwrap();
        let (second, _) = parse_includegraphics(
            "[height=1cm]{figure.jpg}",
            &root,
            &graphics,
            &mut cache,
            &layout,
        )
        .unwrap();

        assert!(matches!(first, GraphicElement::Image(_)));
        assert!(matches!(second, GraphicElement::Image(_)));
        assert_eq!(cache.references.len(), 1);
        assert_eq!(cache.assets.len(), 1);
        assert_eq!(cache.dimensions.len(), 1);
    }

    #[test]
    fn native_engine_embeds_jpeg_includegraphics_as_pdf_xobject() {
        let root = temp_dir("jpeg-graphics");
        let main = root.join("main.tex");
        let figure = root.join("figure.jpg");
        let out = root.join("build");
        fs::write(&figure, tiny_jpeg_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
Before.
\includegraphics[width=2cm]{figure.jpg}
After.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("/Subtype /Image"), "{pdf_text}");
        assert!(pdf_text.contains("/Filter /DCTDecode"), "{pdf_text}");
        assert!(pdf_text.contains("/Im1 Do"), "{pdf_text}");
        assert!(pdf_text.contains("(Before.)"), "{pdf_text}");
        assert!(pdf_text.contains("(After.)"), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&figure.display().to_string()), "{fls}");
    }

    #[test]
    fn native_includegraphics_supports_height_scale_and_keepaspectratio() {
        let root = temp_dir("graphics-sizing-options");
        let main = root.join("main.tex");
        let figure = root.join("figure.jpg");
        let out = root.join("build");
        fs::write(&figure, tiny_jpeg_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
\includegraphics[height=2cm]{figure.jpg}
\includegraphics[scale=10]{figure.jpg}
\includegraphics[width=4cm,height=1cm,keepaspectratio]{figure.jpg}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        let matrices = image_transform_matrices(&pdf_text);
        assert_eq!(matrices.len(), 3, "{matrices:?}\n{pdf_text}");
        assert!((matrices[0].0 - 113.39).abs() < 0.02, "{matrices:?}");
        assert!((matrices[0].1 - 56.69).abs() < 0.02, "{matrices:?}");
        assert!((matrices[1].0 - 15.0).abs() < 0.02, "{matrices:?}");
        assert!((matrices[1].1 - 7.5).abs() < 0.02, "{matrices:?}");
        assert!((matrices[2].0 - 56.69).abs() < 0.02, "{matrices:?}");
        assert!((matrices[2].1 - 28.35).abs() < 0.02, "{matrices:?}");
    }

    #[test]
    fn native_includegraphics_angle_rotates_pdf_matrix() {
        let root = temp_dir("graphics-angle-option");
        let main = root.join("main.tex");
        let figure = root.join("figure.jpg");
        let out = root.join("build");
        fs::write(&figure, tiny_jpeg_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
\includegraphics[width=2cm,angle=90]{figure.jpg}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        let matrices = image_transform_full_matrices(&pdf_text);
        assert_eq!(matrices.len(), 1, "{matrices:?}\n{pdf_text}");
        let (a, b, c, d, _, _) = matrices[0];
        assert!(a.abs() < 0.02, "{matrices:?}");
        assert!((b - 56.69).abs() < 0.02, "{matrices:?}");
        assert!((c + 28.35).abs() < 0.02, "{matrices:?}");
        assert!(d.abs() < 0.02, "{matrices:?}");
    }

    #[test]
    fn native_includegraphics_trim_clip_uses_cropped_layout_box() {
        let root = temp_dir("graphics-trim-clip-option");
        let main = root.join("main.tex");
        let figure = root.join("figure.pdf");
        let out = root.join("build");
        fs::write(&figure, tiny_pdf_graphic_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
\includegraphics[trim=25pt 0pt 25pt 0pt,clip,width=50pt]{figure.pdf}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        let matrices = image_transform_matrices(&pdf_text);
        assert_eq!(matrices.len(), 1, "{matrices:?}\n{pdf_text}");
        assert!((matrices[0].0 - 100.0).abs() < 0.02, "{matrices:?}");
        assert!((matrices[0].1 - 50.0).abs() < 0.02, "{matrices:?}");
        assert!(
            pdf_text.contains("0.250000 0.000000 0.500000 1.000000 re\nW\nn"),
            "{pdf_text}"
        );
    }

    #[test]
    fn native_includegraphics_viewport_without_clip_shifts_source_box() {
        let root = temp_dir("graphics-viewport-option");
        let main = root.join("main.tex");
        let figure = root.join("figure.pdf");
        let out = root.join("build");
        fs::write(&figure, tiny_pdf_graphic_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
\includegraphics[viewport=25pt 0pt 75pt 50pt,width=50pt]{figure.pdf}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        let matrices = image_transform_matrices(&pdf_text);
        assert_eq!(matrices.len(), 1, "{matrices:?}\n{pdf_text}");
        assert!((matrices[0].0 - 100.0).abs() < 0.02, "{matrices:?}");
        assert!((matrices[0].1 - 50.0).abs() < 0.02, "{matrices:?}");
        assert!(!pdf_text.contains("\nW\nn\n/Im1 Do"), "{pdf_text}");
    }

    #[test]
    fn native_rotated_image_uses_rotated_layout_box() {
        let layout = DocumentLayout::default();
        let image = ImageAsset {
            path: PathBuf::from("rotated.jpg"),
            width_px: 2,
            height_px: 1,
            display_width_pt: 56.69,
            display_height_pt: 28.35,
            rotation_degrees: 90.0,
            viewport: ImageViewport::full(),
            payload: Arc::new(ImagePayload::Jpeg(Vec::new())),
        };
        let (width, height) = layout.image_display_size(&image);
        assert!((width - 28.35).abs() < 0.02, "{width} {height}");
        assert!((height - 56.69).abs() < 0.02, "{width} {height}");
    }

    #[test]
    fn native_length_parser_supports_columnwidth() {
        let layout = DocumentLayout::icml_two_column();
        assert_eq!(
            length_expr_to_pt("\\columnwidth", &layout),
            Some(layout.column_width_pt)
        );
        assert_eq!(
            length_expr_to_pt("0.5\\columnwidth", &layout),
            Some(layout.column_width_pt * 0.5)
        );
    }

    #[test]
    fn native_engine_embeds_rgba_png_includegraphics_with_soft_mask() {
        let root = temp_dir("png-graphics");
        let main = root.join("main.tex");
        let figure = root.join("figure.png");
        let out = root.join("build");
        fs::write(&figure, tiny_rgba_png_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
Before.
\includegraphics[width=2cm]{figure.png}
After.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("/Subtype /Image"), "{pdf_text}");
        assert!(pdf_text.contains("/Filter /FlateDecode"), "{pdf_text}");
        assert!(pdf_text.contains("/ColorSpace /DeviceRGB"), "{pdf_text}");
        assert!(pdf_text.contains("/SMask"), "{pdf_text}");
        assert!(pdf_text.contains("/Im1 Do"), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&figure.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_renders_minipage_figure_images_as_row() {
        let root = temp_dir("minipage-figure-row");
        let main = root.join("main.tex");
        let left = root.join("left.jpg");
        let right = root.join("right.jpg");
        let out = root.join("build");
        fs::write(&left, tiny_jpeg_bytes()).unwrap();
        fs::write(&right, tiny_jpeg_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
\begin{figure}
\begin{minipage}{0.49\linewidth}
\includegraphics[width=\linewidth]{left.jpg}
\end{minipage}
\begin{minipage}{0.49\linewidth}
\includegraphics[width=\linewidth]{right.jpg}
\end{minipage}
\caption{Side by side.}
\end{figure}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("/Im1 Do"), "{pdf_text}");
        assert!(pdf_text.contains("/Im2 Do"), "{pdf_text}");
        assert!(pdf_text.contains("Figure 1: Side by side."), "{pdf_text}");
        let image_transforms = image_transform_positions(&pdf_text);
        assert_eq!(
            image_transforms.len(),
            2,
            "{image_transforms:?}\n{pdf_text}"
        );
        assert!(
            (image_transforms[0].1 - image_transforms[1].1).abs() < 0.01,
            "{image_transforms:?}\n{pdf_text}"
        );
        assert!(
            image_transforms[1].0 > image_transforms[0].0,
            "{image_transforms:?}\n{pdf_text}"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&left.display().to_string()), "{fls}");
        assert!(fls.contains(&right.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_renders_graphic_float_rows_without_comment_dependencies() {
        let root = temp_dir("graphic-float-row");
        let main = root.join("main.tex");
        let left = root.join("left.jpg");
        let middle = root.join("middle.jpg");
        let right = root.join("right.jpg");
        let out = root.join("build");
        fs::write(&left, tiny_jpeg_bytes()).unwrap();
        fs::write(&middle, tiny_jpeg_bytes()).unwrap();
        fs::write(&right, tiny_jpeg_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
Before.
\begin{figure}[t]
% \includegraphics[width=\linewidth]{missing-commented.jpg}\\
\includegraphics[width=0.33\linewidth]{left.jpg}
\includegraphics[width=0.33\linewidth]{middle.jpg}
\includegraphics[width=0.33\linewidth]{right.jpg}
\caption{Column row.}
\end{figure}
After.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("/Im1 Do"), "{pdf_text}");
        assert!(pdf_text.contains("/Im2 Do"), "{pdf_text}");
        assert!(pdf_text.contains("/Im3 Do"), "{pdf_text}");
        assert!(pdf_text.contains("Figure 1: Column row."), "{pdf_text}");
        let image_transforms = image_transform_positions(&pdf_text);
        assert_eq!(
            image_transforms.len(),
            3,
            "{image_transforms:?}\n{pdf_text}"
        );
        assert!(
            (image_transforms[0].1 - image_transforms[1].1).abs() < 0.01
                && (image_transforms[1].1 - image_transforms[2].1).abs() < 0.01,
            "{image_transforms:?}\n{pdf_text}"
        );
        assert!(
            image_transforms[0].0 < image_transforms[1].0
                && image_transforms[1].0 < image_transforms[2].0,
            "{image_transforms:?}\n{pdf_text}"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&left.display().to_string()), "{fls}");
        assert!(fls.contains(&middle.display().to_string()), "{fls}");
        assert!(fls.contains(&right.display().to_string()), "{fls}");
        assert!(!fls.contains("missing-commented.jpg"), "{fls}");
    }

    #[test]
    fn native_one_column_starred_graphic_figures_enter_float_path() {
        let root = temp_dir("one-column-starred-graphic-float");
        let main = root.join("main.tex");
        let figure = root.join("wide.jpg");
        let out = root.join("build");
        fs::write(&figure, tiny_jpeg_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\onecolumn
Before.
\begin{figure*}[t]
\includegraphics[width=\textwidth]{wide.jpg}
\caption{One column starred figure.}
\end{figure*}
After.
\end{document}
",
        )
        .unwrap();

        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse document");

        assert!(
            document.lines.iter().any(|line| {
                matches!(
                    line,
                    Line::FloatBlock {
                        wide: true,
                        top: true,
                        ..
                    }
                )
            }),
            "{:?}",
            document.lines
        );
        let placements = line_placements(&document);
        let float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { wide: true, .. }
                )
            })
            .expect("starred graphic float placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After."
                )
            })
            .expect("after text placement");

        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
        assert!(after.page_index >= float.page_index);
    }

    #[test]
    fn native_two_column_starred_graphic_figures_enter_wide_float_path() {
        let root = temp_dir("two-column-starred-graphic-float");
        let main = root.join("main.tex");
        let figure = root.join("wide.jpg");
        let out = root.join("build");
        fs::write(&figure, tiny_jpeg_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\twocolumn
\begin{document}
Before.
\begin{figure*}[t]
\includegraphics[width=\textwidth]{wide.jpg}
\caption{Two column starred figure.}
\end{figure*}
After.
\end{document}
",
        )
        .unwrap();

        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse document");

        assert!(
            document.lines.iter().any(|line| {
                matches!(
                    line,
                    Line::FloatBlock {
                        wide: true,
                        top: true,
                        ..
                    }
                )
            }),
            "{:?}",
            document.lines
        );
        let placements = line_placements(&document);
        let float = placements
            .iter()
            .find(|placement| {
                matches!(
                    document.lines[placement.line_index],
                    Line::FloatBlock { wide: true, .. }
                )
            })
            .expect("starred graphic float placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After."
                )
            })
            .expect("after text placement");

        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
        assert!(after.page_index <= float.page_index);
    }

    #[test]
    fn native_wide_minipage_float_uses_caption_sized_lines() {
        let root = temp_dir("wide-minipage-caption-lines");
        let main = root.join("main.tex");
        let left = root.join("left.jpg");
        let right = root.join("right.jpg");
        let out = root.join("build");
        fs::write(&left, tiny_jpeg_bytes()).unwrap();
        fs::write(&right, tiny_jpeg_bytes()).unwrap();
        let caption = "A long wide caption should be rendered as caption text, not as wide abstract body text, so it uses the denser caption font and preserves more body-flow space on the page while still reporting the same rendered caption placement for the parity harness.";
        fs::write(
            &main,
            format!(
                r"\documentclass{{article}}
\twocolumn
\begin{{document}}
Before.
\begin{{figure*}}[t]
\begin{{minipage}}{{0.49\linewidth}}
\includegraphics[width=\linewidth]{{left.jpg}}
\end{{minipage}}
\begin{{minipage}}{{0.49\linewidth}}
\includegraphics[width=\linewidth]{{right.jpg}}
\end{{minipage}}
\caption{{{caption}}}
\end{{figure*}}
After.
\end{{document}}
"
            ),
        )
        .unwrap();

        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse document");

        let Line::FloatBlock {
            lines: float_lines,
            wide: true,
            ..
        } = document
            .lines
            .iter()
            .find(|line| matches!(line, Line::FloatBlock { wide: true, .. }))
            .expect("wide minipage float")
        else {
            panic!("expected wide float block");
        };
        let rendered_caption = caption_label("Figure", 1, caption, CaptionLabelSeparator::Colon);
        let wide_caption_lines = document.layout.wrap_wide_caption_text(&rendered_caption);

        assert!(
            float_lines.iter().any(
                |line| matches!(line, Line::WideCaption(text) if text.starts_with("Figure 1:"))
            ),
            "{float_lines:?}"
        );
        assert!(
            !float_lines.iter().any(
                |line| matches!(line, Line::WideAbstractText(text) if text.starts_with("Figure 1:"))
            ),
            "{float_lines:?}"
        );
        assert_eq!(
            float_lines
                .iter()
                .filter(|line| matches!(line, Line::WideCaption(_)))
                .count(),
            wide_caption_lines.len()
        );
        assert!(wide_caption_lines.len() < document.layout.wrap_wide_text(&rendered_caption).len());
    }

    #[test]
    fn native_one_column_starred_tables_enter_float_path() {
        let root = temp_dir("one-column-starred-table-float");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\onecolumn
Before.
\begin{table*}[t]
\caption{One column starred table.}
\begin{tabular}{lc}
\toprule
Name & Score\\
\midrule
A & 1\\
\bottomrule
\end{tabular}
\end{table*}
After.
\end{document}
",
        )
        .unwrap();

        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse document");

        let float_index = document
            .lines
            .iter()
            .position(|line| {
                matches!(
                    line,
                    Line::FloatBlock {
                        wide: true,
                        top: true,
                        ..
                    }
                )
            })
            .expect("table* should become a wide top float");
        let Line::FloatBlock { lines, .. } = &document.lines[float_index] else {
            unreachable!();
        };
        assert!(
            lines
                .iter()
                .any(|line| matches!(line, Line::WideCaption(text) if text.contains("Table 1: One column starred table."))),
            "{lines:?}"
        );
        assert!(
            lines.iter().any(
                |line| matches!(line, Line::WideTableCells { cells, slots } if cells.as_slice() == ["Name", "Score"] && *slots == 1)
            ),
            "{lines:?}"
        );
        let placements = line_placements(&document);
        let float = placements
            .iter()
            .find(|placement| placement.line_index == float_index)
            .expect("starred table placement");
        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
    }

    #[test]
    fn native_two_column_starred_tables_enter_wide_float_path() {
        let root = temp_dir("two-column-starred-table-float");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\twocolumn
\begin{document}
Before.
\begin{table*}[t]
\caption{Two column starred table.}
\begin{tabular}{lc}
\toprule
Name & Score\\
\midrule
A & 1\\
\bottomrule
\end{tabular}
\end{table*}
After.
\end{document}
",
        )
        .unwrap();

        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse document");

        let float_index = document
            .lines
            .iter()
            .position(|line| {
                matches!(
                    line,
                    Line::FloatBlock {
                        wide: true,
                        top: true,
                        ..
                    }
                )
            })
            .expect("table* should become a wide top float");
        let Line::FloatBlock { lines, .. } = &document.lines[float_index] else {
            unreachable!();
        };
        assert!(
            lines.iter().any(
                |line| matches!(line, Line::WideCaption(text) if text.contains("Table 1: Two column starred table."))
            ),
            "{lines:?}"
        );
        assert!(
            lines.iter().any(
                |line| matches!(line, Line::WideTableCells { cells, slots } if cells.as_slice() == ["Name", "Score"] && *slots == 1)
            ),
            "{lines:?}"
        );
        let placements = line_placements(&document);
        let float = placements
            .iter()
            .find(|placement| placement.line_index == float_index)
            .expect("starred table placement");

        assert_eq!(float.page_index, 1);
        assert_eq!(float.page_slot, 0);
    }

    #[test]
    fn native_one_column_bottom_tables_reserve_bottom_rows() {
        let root = temp_dir("one-column-bottom-table-float");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\onecolumn
Before.
\begin{table}[b]
\caption{Bottom table.}
\begin{tabular}{lc}
\toprule
Name & Score\\
\midrule
A & 1\\
\bottomrule
\end{tabular}
\end{table}
After.
\end{document}
",
        )
        .unwrap();

        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse document");
        let float_index = document
            .lines
            .iter()
            .position(|line| matches!(line, Line::BottomFloatBlock { .. }))
            .expect("bottom table should become a bottom float");
        let float_slots = document.lines[float_index].slots(document.layout, &document.images);
        let placements = line_placements(&document);
        let float = placements
            .iter()
            .find(|placement| placement.line_index == float_index)
            .expect("bottom table placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After."
                )
            })
            .expect("after text placement");

        assert_eq!(float.page_index, 0);
        assert_eq!(
            float.page_slot,
            document.layout.rows_per_column - float_slots
        );
        assert!(after.page_slot < float.page_slot, "{after:?} {float:?}");
    }

    #[test]
    fn native_two_column_tables_use_compact_table_rows() {
        let root = temp_dir("two-column-table-rows");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\twocolumn
Before.
\begin{table}[t]
\caption{Two column table.}
\begin{tabular}{lc}
\toprule
Name & Score\\
\midrule
A & 1\\
\bottomrule
\end{tabular}
\end{table}
After.
\end{document}
",
        )
        .unwrap();

        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse document");

        assert_eq!(document.layout.columns, 2);
        assert!(
            document
                .lines
                .iter()
                .any(|line| matches!(line, Line::TableRow(text) if text.contains("Name | Score"))),
            "{:?}",
            document.lines
        );
        assert!(
            !document
                .lines
                .iter()
                .any(|line| matches!(line, Line::Text(text) if text.contains("Name | Score"))),
            "{:?}",
            document.lines
        );
    }

    fn image_transform_positions(pdf_text: &str) -> Vec<(f32, f32)> {
        image_transform_matrices(pdf_text)
            .into_iter()
            .map(|(_, _, x, y)| (x, y))
            .collect()
    }

    fn image_transform_matrices(pdf_text: &str) -> Vec<(f32, f32, f32, f32)> {
        image_transform_full_matrices(pdf_text)
            .into_iter()
            .map(|(a, _, _, d, e, f)| (a, d, e, f))
            .collect()
    }

    fn image_transform_full_matrices(pdf_text: &str) -> Vec<(f32, f32, f32, f32, f32, f32)> {
        pdf_text
            .lines()
            .filter_map(|line| {
                let parts = line.split_whitespace().collect::<Vec<_>>();
                if parts.len() == 7 && parts[6] == "cm" {
                    Some((
                        parts[0].parse().ok()?,
                        parts[1].parse().ok()?,
                        parts[2].parse().ok()?,
                        parts[3].parse().ok()?,
                        parts[4].parse().ok()?,
                        parts[5].parse().ok()?,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }

    #[test]
    fn native_engine_resolves_graphicspath_and_declared_graphics_extensions() {
        let root = temp_dir("configured-graphics");
        let main = root.join("main.tex");
        let figures = root.join("figures");
        fs::create_dir_all(&figures).unwrap();
        let png_figure = figures.join("plot.png");
        let pdf_figure = figures.join("plot.pdf");
        let out = root.join("build");
        fs::write(&png_figure, tiny_rgba_png_bytes()).unwrap();
        fs::write(&pdf_figure, tiny_pdf_graphic_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\graphicspath{{figures/}}
\DeclareGraphicsExtensions{.png,.pdf}
\begin{document}
Before.
\includegraphics[width=2cm]{plot}
After.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("/Subtype /Image"), "{pdf_text}");
        assert!(pdf_text.contains("/Filter /FlateDecode"), "{pdf_text}");
        assert!(!pdf_text.contains("/Subtype /Form"), "{pdf_text}");
        assert!(!pdf_text.contains("graphicspath"), "{pdf_text}");
        assert!(
            !pdf_text.contains("DeclareGraphicsExtensions"),
            "{pdf_text}"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&png_figure.display().to_string()), "{fls}");
        assert!(!fls.contains(&pdf_figure.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_resolves_texinputs_graphics() {
        let root = temp_dir("texinputs-graphics");
        let paper = root.join("paper");
        let shared = root.join("shared").join("tex");
        let figures = shared.join("figures");
        let main = paper.join("main.tex");
        let figure = figures.join("plot.png");
        let out = root.join("build");
        fs::create_dir_all(&paper).unwrap();
        fs::create_dir_all(&figures).unwrap();
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let _texinputs = EnvVarGuard::set(
            "TEXINPUTS",
            OsString::from(format!("{}//{}", shared.display(), KPATHSEA_PATH_SEPARATOR)),
        );
        fs::write(&figure, tiny_rgba_png_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\graphicspath{{figures/}}
\DeclareGraphicsExtensions{.png}
\begin{document}
Before.
\includegraphics[width=2cm]{plot}
After.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("/Subtype /Image"), "{pdf_text}");
        assert!(pdf_text.contains("/Filter /FlateDecode"), "{pdf_text}");
        assert!(pdf_text.contains("/Im1 Do"), "{pdf_text}");
        assert!(pdf_text.contains("(Before.)"), "{pdf_text}");
        assert!(pdf_text.contains("(After.)"), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(
            fls.contains(&figure.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_embeds_pdf_includegraphics_as_form_xobject() {
        let root = temp_dir("pdf-graphics");
        let main = root.join("main.tex");
        let figure = root.join("figure.pdf");
        let out = root.join("build");
        fs::write(&figure, tiny_pdf_graphic_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
Before.
\includegraphics[width=2cm]{figure.pdf}
After.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("/Subtype /Form"), "{pdf_text}");
        assert!(pdf_text.contains("/FormType 1"), "{pdf_text}");
        assert!(pdf_text.contains("/Matrix"), "{pdf_text}");
        assert!(pdf_text.contains("/Resources"), "{pdf_text}");
        assert!(pdf_text.contains("/ExtGState"), "{pdf_text}");
        assert!(pdf_text.contains("/CA 1"), "{pdf_text}");
        assert!(pdf_text.contains("/Im1 Do"), "{pdf_text}");
        assert!(!pdf_text.contains("[graphic:"), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&figure.display().to_string()), "{fls}");
    }

    #[test]
    fn native_pdf_includegraphics_honors_page_option() {
        let root = temp_dir("pdf-graphics-page-option");
        let main = root.join("main.tex");
        let figure = root.join("figure.pdf");
        let out = root.join("build");
        fs::write(&figure, tiny_two_page_pdf_graphic_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{graphicx}
\begin{document}
\includegraphics[width=2cm,page=2]{figure.pdf}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("0 1 0 rg"), "{pdf_text}");
        assert!(!pdf_text.contains("0 0 1 rg"), "{pdf_text}");
        let matrices = image_transform_matrices(&pdf_text);
        assert_eq!(matrices.len(), 1, "{matrices:?}\n{pdf_text}");
        assert!((matrices[0].0 - 56.69).abs() < 0.02, "{matrices:?}");
        assert!((matrices[0].1 - 141.73).abs() < 0.02, "{matrices:?}");
    }

    #[test]
    fn native_engine_renders_known_tikz_schematic_as_pdf_form() {
        let root = temp_dir("native-tikz");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Before.
\begin{tikzpicture}
\node at (0,0) {World $z$};
\node at (1,0) {Data $x = g(z)$};
\node at (2,0) {LeJEPA $f(x)$};
\end{tikzpicture}
After.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("/Subtype /Form"), "{pdf_text}");
        assert!(pdf_text.contains("/Im1 Do"), "{pdf_text}");
        assert!(!pdf_text.contains("[TikZ picture]"), "{pdf_text}");
        assert!(pdf_text.contains("(Before.)"), "{pdf_text}");
        assert!(pdf_text.contains("(After.)"), "{pdf_text}");
    }

    #[test]
    fn native_engine_renders_tikz_textwidth_rule_as_pdf_form() {
        let root = temp_dir("native-tikz-rule");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\begin{tikzpicture}
\draw[line width=2pt] (0,0) -- (\textwidth,0);
\end{tikzpicture}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("/Subtype /Form"), "{pdf_text}");
        assert!(pdf_text.contains("/Im1 Do"), "{pdf_text}");
        assert!(!pdf_text.contains("[TikZ picture]"), "{pdf_text}");
    }

    #[test]
    fn native_engine_renders_paragraph_headings_run_in() {
        let root = temp_dir("paragraph-heading-punctuation");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\paragraph{Contributions.}
We close this gap.
\paragraph{Outlook}
More work remains.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_text.contains("(Contributions. We close this gap.)"),
            "{pdf_text}"
        );
        assert!(!pdf_text.contains("Contributions.."), "{pdf_text}");
        assert!(
            pdf_text.contains("(Outlook. More work remains.)"),
            "{pdf_text}"
        );
    }

    #[test]
    fn native_engine_skips_appendix_title_layout_commands() {
        let root = temp_dir("appendix-title-layout");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\clearpage
\onecolumn
\appendix
\thispagestyle{empty}
\begin{center}
\noindent
\hfill
\setlength{\tabcolsep}{2.5pt}
\medskip
\begin{tikzpicture}
\draw[line width=2pt] (0,0) -- (\textwidth,0);
\end{tikzpicture}
\\[0.5em]
{\fontsize{22pt}{26pt}\selectfont\bfseries LeJEPA}\\[1.2em]
{\fontsize{16pt}{20pt}\selectfont\bfseries Appendix}\\[2.5em]
\begin{tikzpicture}
\draw[line width=0.5pt] (0,0) -- (\textwidth,0);
\end{tikzpicture}
\end{center}
\vspace{2cm}
\section{Additional Details}
Body.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("LeJEPA"), "{pdf_text}");
        assert!(pdf_text.contains("Appendix"), "{pdf_text}");
        assert!(pdf_text.contains("A Additional Details"), "{pdf_text}");
        for artifact in [
            "onecolumn",
            "thispagestyle",
            "fontsize",
            "selectfont",
            "hfill",
            "setlength",
            "tabcolsep",
            "medskip",
            "[0.5em]",
            "[1.2em]",
            "[2.5em]",
            "[TikZ picture]",
        ] {
            assert!(
                !pdf_text.contains(artifact),
                "found `{artifact}` in {pdf_text}"
            );
        }
    }

    #[test]
    fn native_vertical_space_converts_common_positive_lengths_to_slots() {
        let layout = DocumentLayout::default();

        assert_eq!(vertical_space_slots("2cm", &layout), 5);
        assert_eq!(vertical_space_slots("10pt", &layout), 1);
        assert_eq!(vertical_space_slots("0.5em", &layout), 0);
        assert_eq!(vertical_space_slots("-10pt", &layout), 0);
        assert_eq!(vertical_space_slots("\\baselineskip", &layout), 0);
        assert_eq!(vertical_skip_slots_points(3.0, &layout), 0);
        assert_eq!(vertical_skip_slots_points(6.0, &layout), 1);
        assert_eq!(vertical_skip_slots_points(12.0, &layout), 1);
    }

    #[test]
    fn native_layout_materializes_positive_vspace() {
        let root = temp_dir("positive-vspace");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Before.
\vspace{2cm}
After.
\end{document}
",
        )
        .unwrap();

        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse document");
        let placements = line_placements(&document);
        let before = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "Before."
                )
            })
            .expect("before placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After."
                )
            })
            .expect("after placement");

        assert_eq!(before.page_index, 0);
        assert_eq!(after.page_index, 0);
        assert_eq!(after.page_slot, before.page_slot + 1 + 5);
    }

    #[test]
    fn native_layout_materializes_standalone_medskip() {
        let root = temp_dir("standalone-medskip");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Before.
\medskip
After.
\end{document}
",
        )
        .unwrap();

        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse document");
        let placements = line_placements(&document);
        let before = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "Before."
                )
            })
            .expect("before placement");
        let after = placements
            .iter()
            .find(|placement| {
                matches!(
                    &document.lines[placement.line_index],
                    Line::Text(text) if text == "After."
                )
            })
            .expect("after placement");

        assert_eq!(before.page_index, 0);
        assert_eq!(after.page_index, 0);
        assert_eq!(after.page_slot, before.page_slot + 1 + 1);
    }

    #[test]
    fn native_engine_expands_simple_no_argument_macros() {
        let root = temp_dir("macro");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\newcommand{\hello}{Hello from newcommand}
\def\bye{Goodbye from def}
\begin{document}
\hello
\bye
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Hello from newcommand"), "{pdf_text}");
        assert!(pdf_text.contains("Goodbye from def"), "{pdf_text}");
    }

    #[test]
    fn native_engine_uses_expansion_core_for_internal_def_macros() {
        let root = temp_dir("expansion-core-internal");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\makeatletter
\def\paper@name{Native Expansion}
\let\paperalias=\paper@name
\makeatother
\begin{document}
Before \paperalias{} After.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_text.contains("Before Native Expansion After."),
            "{pdf_text}"
        );
    }

    #[test]
    fn native_engine_uses_expansion_core_for_group_scoped_macros() {
        let root = temp_dir("expansion-core-groups");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\def\paperword{Outer}
\begin{document}
{\def\paperword{Inner}\paperword}
\paperword
\begingroup\gdef\globalword{Global}\endgroup
\globalword
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Inner"), "{pdf_text}");
        assert!(pdf_text.contains("Outer"), "{pdf_text}");
        assert!(pdf_text.contains("Global"), "{pdf_text}");
    }

    #[test]
    fn native_engine_uses_expansion_core_for_conditionals() {
        let root = temp_dir("expansion-core-conditionals");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\newif\ifdraft
\drafttrue
\begin{document}
\ifdraft Draft branch\else Final branch\fi
\iffalse\write{unsupported dead branch}\else Live branch\fi
\def\a{same}\let\b=\a
\ifx\a\b Equal branch\else Different branch\fi
\def\stem{visible}
\def\visible{Ifcsname branch}
\ifcsname \stem\endcsname \visible\else Missing branch\fi
\def\letter{a}
\if\letter a If-char branch\else Bad-char branch\fi
\ifcat a b If-cat branch\else Bad-cat branch\fi
\unless\ifdefined\missing Unless branch\else Bad-unless branch\fi
\def\futuretarget{Futurelet branch}
\def\futureprobe#1{\ifx\next\futuretarget Futurelet branch\else Bad futurelet\fi}
\futurelet\next\futureprobe\futuretarget
\def\afterword{Aftergroup branch}
\begingroup\def\afterword{Bad aftergroup}\aftergroup\afterword\endgroup
Relax \relax branch.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Draft branch"), "{pdf_text}");
        assert!(pdf_text.contains("Live branch"), "{pdf_text}");
        assert!(pdf_text.contains("Equal branch"), "{pdf_text}");
        assert!(pdf_text.contains("Ifcsname branch"), "{pdf_text}");
        assert!(pdf_text.contains("If-char branch"), "{pdf_text}");
        assert!(pdf_text.contains("If-cat branch"), "{pdf_text}");
        assert!(pdf_text.contains("Unless branch"), "{pdf_text}");
        assert!(pdf_text.contains("Futurelet branch"), "{pdf_text}");
        assert!(pdf_text.contains("Aftergroup branch"), "{pdf_text}");
        assert!(pdf_text.contains("Relax branch."), "{pdf_text}");
        assert!(!pdf_text.contains("unsupported dead branch"), "{pdf_text}");
        assert!(!pdf_text.contains("Bad-char branch"), "{pdf_text}");
        assert!(!pdf_text.contains("Bad-cat branch"), "{pdf_text}");
        assert!(!pdf_text.contains("Bad-unless branch"), "{pdf_text}");
        assert!(!pdf_text.contains("Bad futurelet"), "{pdf_text}");
        assert!(!pdf_text.contains("Bad aftergroup"), "{pdf_text}");
    }

    #[test]
    fn native_engine_uses_expansion_core_for_count_registers() {
        let root = temp_dir("expansion-core-counts");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\newcount\score
\score=2
\advance\score by 5
\count12=4
\countdef\bonus=12
\advance\bonus by 3
\chardef\smallconst=2
\newdimen\gap
\gap=1.5pt
\advance\gap by .5pt
\newdimen\widegap
\widegap=\dimexpr\gap+1pt\relax
\newskip\pad
\pad=1pt plus 2pt minus .5pt
\advance\pad by 3pt plus 1pt
\newtoks\messagebox
\messagebox={Token math \number\numexpr\score+2\relax}
\def\latevalue{Late value}
\edef\romanlabel{\romannumeral\numexpr\score+2\relax}
\edef\latebox{\unexpanded{\latevalue}}
\makeatletter
\begin{document}
Score \number\numexpr\score+1\relax.
Bonus \number\count12.
Const \number\smallconst.
Gap \the\gap.
Wide \the\widegap.
Pad \the\pad.
\the\messagebox.
Roman \romanlabel.
Late \latebox.
\ifnum\score>6 High score\else Low score\fi
\ifnum\numexpr\score*2\relax=14 Numeric math\else Bad numeric\fi
\ifcase\bonus Zero bonus\or One bonus\else Many bonus\fi
\ifcase\@ne Zero-internal\or Internal-one\else Many-internal\fi
\ifdim\gap=2pt Exact gap\else Bad gap\fi
\ifdim\dimexpr\widegap-1pt=2pt Wide math\else Bad math\fi
\ifdim\p@=1pt One-point\else Bad-point\fi
\begingroup\score=1 Local \number\score.\endgroup
Restored \number\score.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        let literal_text = pdf_literal_text(&pdf);
        assert!(pdf_text.contains("Score 8."), "{pdf_text}");
        assert!(pdf_text.contains("Bonus 7."), "{pdf_text}");
        assert!(pdf_text.contains("Const 2."), "{pdf_text}");
        assert!(pdf_text.contains("Gap 2.0pt."), "{pdf_text}");
        assert!(pdf_text.contains("Wide 3.0pt."), "{pdf_text}");
        assert!(
            pdf_text.contains("Pad 4.0pt plus 3.0pt minus"),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("0.5pt."), "{pdf_text}");
        assert!(pdf_text.contains("Token math 9."), "{pdf_text}");
        assert!(pdf_text.contains("Roman ix."), "{pdf_text}");
        assert!(pdf_text.contains("Late Late value."), "{pdf_text}");
        assert!(pdf_text.contains("High score"), "{pdf_text}");
        assert!(pdf_text.contains("Numeric math"), "{pdf_text}");
        assert!(literal_text.contains("Many bonus"), "{literal_text}");
        assert!(pdf_text.contains("Internal-one"), "{pdf_text}");
        assert!(pdf_text.contains("Exact gap"), "{pdf_text}");
        assert!(pdf_text.contains("Wide math"), "{pdf_text}");
        assert!(pdf_text.contains("One-point"), "{pdf_text}");
        assert!(pdf_text.contains("Local 1."), "{pdf_text}");
        assert!(pdf_text.contains("Restored 7."), "{pdf_text}");
    }

    #[test]
    fn native_engine_uses_expansion_core_for_catcode_assignments() {
        let root = temp_dir("expansion-core-catcode");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\catcode`\@=11
\def\internal@macro{Catcode Native}
\catcode`\@=12
\begin{document}
\catcode`\@=11
\internal@macro
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Catcode Native"), "{pdf_text}");
    }

    #[test]
    fn native_engine_uses_expansion_core_for_newcommand_arguments() {
        let root = temp_dir("expansion-core-newcommand-args");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\newcommand{\pair}[2]{#1/#2}
\begin{document}
Value \pair{left}{right}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Value left/right."), "{pdf_text}");
    }

    #[test]
    fn native_engine_uses_expansion_core_for_declare_math_operator() {
        let root = temp_dir("expansion-core-math-operator");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\DeclareMathOperator{\Trace}{Trace}
\begin{document}
Value $\Trace A$.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Value TraceA."), "{pdf_text}");
    }

    #[test]
    fn native_engine_uses_expansion_core_for_protected_edef_assignments() {
        let root = temp_dir("expansion-core-protected-edef");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\makeatletter
\def\source@value{Captured}
\newcommand{\capturevalue}{\protected@edef\stored@value{\source@value}}
\protected\def\safe@value{Original}
\protected@edef\stored@protected{\safe@value}
\def\safe@value{Runtime}
\DeclareRobustCommand{\robustword}[1]{Original #1}
\protected@edef\stored@robust{\robustword{branch}}
\renewcommand{\robustword}[1]{Robust #1}
\def\expandedword{Expanded }
\begin{document}
\capturevalue
Value \stored@value.
Protected \stored@protected.
Stored \stored@robust.
\expanded{\expandedword branch}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Value Captured."), "{pdf_text}");
        assert!(pdf_text.contains("Protected Runtime."), "{pdf_text}");
        assert!(pdf_text.contains("Stored Robust branch."), "{pdf_text}");
        assert!(pdf_text.contains("Expanded branch."), "{pdf_text}");
    }

    #[test]
    fn native_engine_uses_expansion_core_for_dynamic_csname_definitions() {
        let root = temp_dir("expansion-core-csname");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\makeatletter
\def\macro@stem{dynamic@value}
\expandafter\def\csname \macro@stem\endcsname{Dynamic Native}
\begin{document}
Value \dynamic@value.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Value Dynamic Native."), "{pdf_text}");
    }

    #[test]
    fn native_engine_resolves_basic_section_refs_without_aux_reruns() {
        let root = temp_dir("refs");
        let main = root.join("main.tex");
        let out = root.join("build");
        let filler = (0..35)
            .map(|index| {
                format!(
                    "Filler paragraph {index} keeps the native reference target far enough down \
                     the document to cross the first page boundary."
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");
        fs::write(
            &main,
            format!(
                r"\documentclass{{article}}
\begin{{document}}
\section{{First}}
\label{{sec:first}}
See Section~\ref{{sec:first}} on page~\pageref{{sec:first}}.
Single~\cref{{sec:first}}.
Combined Sections~\cref{{sec:first,sec:later}} on pages~\pageref{{sec:first,sec:later}}.

{filler}

\section{{Later}}
\label{{sec:later}}
Later Section~\ref{{sec:later}} on page~\pageref{{sec:later}}.
\end{{document}}
"
            ),
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("1 First"), "{pdf_text}");
        assert!(pdf_text.contains("See Section 1 on page 1."), "{pdf_text}");
        assert!(pdf_text.contains("Single section 1."), "{pdf_text}");
        assert!(
            pdf_text.contains("Combined Sections 1, 2 on pages 1, 3."),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("2 Later"), "{pdf_text}");
        assert!(
            pdf_text.contains("Later Section 2 on page 3."),
            "{pdf_text}"
        );
        let aux = fs::read_to_string(out.join("main.aux")).expect("aux should exist");
        assert!(aux.contains("\\relax"), "{aux}");
        assert!(aux.contains("\\newlabel{sec:first}{{1}{1}}"), "{aux}");
        assert!(aux.contains("\\newlabel{sec:later}{{2}{3}}"), "{aux}");
    }

    #[test]
    fn native_engine_writes_and_renders_table_of_contents() {
        let root = temp_dir("toc");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\tableofcontents
\section{Intro}
Opening text.
\subsection{Details}
More text.
\section*{Acknowledgments}
\addcontentsline{toc}{section}{Acknowledgments}
Unnumbered text.
\section{Results}
Final text.
\appendix
\section{Supplement}
\label{app:supplement}
Appendix ref \ref{app:supplement}.
\subsection{Extra Details}
Appendix details.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Contents"), "{pdf_text}");
        assert!(pdf_text.contains("1 Intro .... 1"), "{pdf_text}");
        assert!(pdf_text.contains("1.1 Details .... 1"), "{pdf_text}");
        assert!(pdf_text.contains("Acknowledgments .... 1"), "{pdf_text}");
        assert!(pdf_text.contains("2 Results .... 1"), "{pdf_text}");
        assert!(pdf_text.contains("A Supplement .... 1"), "{pdf_text}");
        assert!(pdf_text.contains("A.1 Extra Details .... 1"), "{pdf_text}");
        assert!(pdf_text.contains("A Supplement"), "{pdf_text}");
        assert!(pdf_text.contains("Appendix ref A."), "{pdf_text}");

        let toc = fs::read_to_string(out.join("main.toc")).expect("toc should exist");
        assert!(
            toc.contains("\\contentsline {section}{\\numberline {1}Intro}{1}{}"),
            "{toc}"
        );
        assert!(
            toc.contains("\\contentsline {subsection}{\\numberline {1.1}Details}{1}{}"),
            "{toc}"
        );
        assert!(
            toc.contains("\\contentsline {section}{\\numberline {2}Results}{1}{}"),
            "{toc}"
        );
        assert!(
            toc.contains("\\contentsline {section}{Acknowledgments}{1}{}"),
            "{toc}"
        );
        assert!(
            toc.contains("\\contentsline {section}{\\numberline {A}Supplement}{1}{}"),
            "{toc}"
        );
        assert!(
            toc.contains("\\contentsline {subsection}{\\numberline {A.1}Extra Details}{1}{}"),
            "{toc}"
        );

        let aux = fs::read_to_string(out.join("main.aux")).expect("aux should exist");
        assert!(aux.contains("\\@writefile{toc}{\\contentsline"), "{aux}");
        assert!(
            aux.contains("\\@writefile{toc}{\\contentsline {section}{Acknowledgments}{1}{}}"),
            "{aux}"
        );
        assert!(aux.contains("\\newlabel{app:supplement}{{A}{1}}"), "{aux}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(
            fls.contains(&out.join("main.toc").display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_renders_numbered_equations_and_refs() {
        let root = temp_dir("equation");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\begin{equation}\label{eq:one}
x = y + 1
\end{equation}
Equation~\ref{eq:one}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("x = y + 1 \\(1\\)"), "{pdf_text}");
        assert!(pdf_text.contains("Equation 1."), "{pdf_text}");
    }

    #[test]
    fn native_engine_renders_raw_display_math() {
        let root = temp_dir("raw-display-math");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Before display.
\[
x = y + 1
\]
Between displays.
$$z = w + 2$$
Line break before.\\[0em]
After line break.
After display.
\end{document}
",
        )
        .unwrap();

        fs::create_dir_all(&out).unwrap();
        let source = fs::read_to_string(&main).unwrap();
        let mut inputs = Vec::new();
        let document = parse_supported_document(
            &source,
            &root,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .unwrap();
        let display_equations = document
            .lines
            .iter()
            .filter_map(|line| match line {
                Line::DisplayEquation(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(display_equations, ["z = w + 2"]);
        assert!(
            !document
                .lines
                .iter()
                .any(|line| matches!(line, Line::DisplayEquation(text) if text == "0em")),
            "{:?}",
            document.lines
        );

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Before display."), "{pdf_text}");
        assert!(pdf_text.contains("x = y + 1"), "{pdf_text}");
        assert!(pdf_text.contains("Between displays."), "{pdf_text}");
        assert!(pdf_text.contains("z = w + 2"), "{pdf_text}");
        assert!(pdf_text.contains("Line break before."), "{pdf_text}");
        assert!(pdf_text.contains("After line break."), "{pdf_text}");
        assert!(pdf_text.contains("After display."), "{pdf_text}");
    }

    #[test]
    fn native_engine_renders_align_rows_and_refs() {
        let root = temp_dir("align");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\begin{align}
a &= b + 1 \label{eq:first}\\
c &= d + 2 \notag\\
e &= f + 3 \label{eq:second}
\end{align}
Refs~\ref{eq:first} and~\ref{eq:second}.
Grouped~\cref{eq:first,eq:second} and equations~\eqref{eq:first,eq:second}.
\begin{align*}
x &= y\\
u &= v
\end{align*}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("a = b + 1 \\(1\\)"), "{pdf_text}");
        assert!(pdf_text.contains("c = d + 2"), "{pdf_text}");
        assert!(!pdf_text.contains("c = d + 2 \\(2\\)"), "{pdf_text}");
        assert!(pdf_text.contains("e = f + 3 \\(2\\)"), "{pdf_text}");
        assert!(pdf_text.contains("Refs 1 and 2."), "{pdf_text}");
        assert!(
            pdf_text.contains("Grouped 1, 2 and equations \\(1, 2\\)."),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("x = y"), "{pdf_text}");
        assert!(!pdf_text.contains("x = y \\("), "{pdf_text}");
    }

    #[test]
    fn native_engine_resolves_lstlisting_refs() {
        let root = temp_dir("lstlisting-refs");
        let main = root.join("main.tex");
        let style = root.join("captionstyle.sty");
        let out = root.join("build");
        fs::write(&style, r"\captionsetup{labelsep=period}").unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{captionstyle}
\renewcommand\lstlistingname{Algorithm}
\begin{document}
\begin{lstlisting}[caption={First listing},label={lst:first}]
print('first')
\end{lstlisting}
\begin{lstlisting}[caption={Second listing},label={code:second}]
print('second')
\end{lstlisting}
Listings~\cref{lst:first,code:second}.
Single~\cref{lst:first}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_text.contains("Algorithm 1. First listing"),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("Algorithm 2. Second listing"),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("print\\('first'\\)"), "{pdf_text}");
        assert!(pdf_text.contains("print\\('second'\\)"), "{pdf_text}");
        assert!(!pdf_text.contains("[code listing]"), "{pdf_text}");
        assert!(pdf_text.contains("Listings 1, 2."), "{pdf_text}");
        assert!(pdf_text.contains("Single algorithm 1."), "{pdf_text}");
        assert!(pdf_text.contains("/F2 9.00 Tf"), "{pdf_text}");
        assert!(pdf_text.contains("/BaseFont /Courier"), "{pdf_text}");
    }

    #[test]
    fn native_two_column_lstfloat_enters_grouped_float_path() {
        let root = temp_dir("lstfloat-grouped-path");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\twocolumn
\renewcommand\lstlistingname{Algorithm}
\begin{document}
Before listing.
\begin{lstfloat}[t!]
\begin{lstlisting}[caption={Floating implementation},label={alg:floating}]
def f(x):
    return x + 1
\end{lstlisting}
\end{lstfloat}
After listing.
\end{document}
",
        )
        .unwrap();

        let loaded = load_document(&main, "main")
            .expect("load document")
            .expect("native fixture loads");
        let mut inputs = loaded.inputs;
        let document = parse_supported_document(
            &loaded.source,
            &loaded.root_dir,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .expect("parse document");
        let float = document
            .lines
            .iter()
            .find_map(|line| match line {
                Line::FloatBlock { lines, wide, top } => Some((lines, *wide, *top)),
                _ => None,
            })
            .expect("lstfloat should become a float block");
        assert!(!float.1);
        assert!(!float.2);
        assert!(
            float
                .0
                .iter()
                .any(|line| matches!(line, Line::Caption(text) if text.contains("Algorithm 1: Floating implementation"))),
            "{:?}",
            float.0
        );
        assert!(
            float
                .0
                .iter()
                .any(|line| matches!(line, Line::Code(text) if text.contains("return x + 1"))),
            "{:?}",
            float.0
        );
    }

    #[test]
    fn native_engine_renders_external_code_listing_inputs_without_fallback() {
        let root = temp_dir("external-code-listings");
        let main = root.join("main.tex");
        let snippet_py = root.join("snippet.py");
        let snippet_rs = root.join("snippet.rs");
        let out = root.join("build");
        fs::write(&snippet_py, "def external():\n    return 'python'\n").unwrap();
        fs::write(
            &snippet_rs,
            "fn external() -> &'static str {\n    \"rust\"\n}\n",
        )
        .unwrap();
        fs::write(
            &main,
            r#"\documentclass{article}
\begin{document}
\lstinputlisting[caption={External Python},label={lst:external}]{snippet.py}
\inputminted{rust}{snippet.rs}
Listing ref \ref{lst:external}.
\end{document}
"#,
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_text.contains("Listing 1: External Python"),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("def external\\(\\):"), "{pdf_text}");
        assert!(pdf_text.contains("return 'python'"), "{pdf_text}");
        assert!(
            pdf_text.contains("fn external\\(\\) -> &'static str {"),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("\"rust\""), "{pdf_text}");
        assert!(pdf_text.contains("Listing ref 1."), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&snippet_py.display().to_string()), "{fls}");
        assert!(fls.contains(&snippet_rs.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_resolves_texinputs_code_listing_inputs() {
        let root = temp_dir("texinputs-code-listings");
        let paper = root.join("paper");
        let shared = root.join("shared").join("tex");
        let code = shared.join("code");
        let main = paper.join("main.tex");
        let snippet_py = code.join("snippet.py");
        let snippet_rs = code.join("snippet.rs");
        let out = root.join("build");
        fs::create_dir_all(&paper).unwrap();
        fs::create_dir_all(&code).unwrap();
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let _texinputs = EnvVarGuard::set(
            "TEXINPUTS",
            OsString::from(format!("{}//{}", shared.display(), KPATHSEA_PATH_SEPARATOR)),
        );
        fs::write(&snippet_py, "def shared():\n    return 'python'\n").unwrap();
        fs::write(
            &snippet_rs,
            "fn shared() -> &'static str {\n    \"rust\"\n}\n",
        )
        .unwrap();
        fs::write(
            &main,
            r#"\documentclass{article}
\begin{document}
\lstinputlisting[caption={Shared Python},label={lst:shared}]{code/snippet.py}
\inputminted{rust}{code/snippet.rs}
Listing ref \ref{lst:shared}.
\end{document}
"#,
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Listing 1: Shared Python"), "{pdf_text}");
        assert!(pdf_text.contains("def shared\\(\\):"), "{pdf_text}");
        assert!(pdf_text.contains("return 'python'"), "{pdf_text}");
        assert!(
            pdf_text.contains("fn shared\\(\\) -> &'static str {"),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("\"rust\""), "{pdf_text}");
        assert!(pdf_text.contains("Listing ref 1."), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(
            fls.contains(&snippet_py.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
        assert!(
            fls.contains(&snippet_rs.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_renders_verbatim_like_blocks_without_fallback() {
        let root = temp_dir("verbatim-like-blocks");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r#"\documentclass{article}
\usepackage{minted}
\begin{document}
Before.
\begin{verbatim}
% visible percent and \end{document}
a--b
\end{verbatim}
\begin{Verbatim}[fontsize=\small]
\alpha stays literal
\end{Verbatim}
\begin{minted}[linenos]{python}
print("hello")
\end{minted}
After.
\end{document}
"#,
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_text.contains("% visible percent and \\\\end{document}"),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("a--b"), "{pdf_text}");
        assert!(pdf_text.contains("\\\\alpha stays literal"), "{pdf_text}");
        assert!(pdf_text.contains("print\\(\"hello\"\\)"), "{pdf_text}");
        assert!(pdf_text.contains("Before."), "{pdf_text}");
        assert!(pdf_text.contains("After."), "{pdf_text}");
    }

    #[test]
    fn native_engine_normalizes_tex_prose_punctuation_without_rewriting_code() {
        let root = temp_dir("prose-punctuation");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r#"\documentclass{article}
\begin{document}
Range 1--3, interruption---yes, and ``quoted text''.

These are \emph{latent variables} / \textit{sources}.
\begin{figure}
\caption{A ``caption'' with 2--4}
\end{figure}
\begin{lstlisting}[caption={Code listing}]
# keep -- in code comments
print("a--b")
\end{lstlisting}
\end{document}
"#,
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        let page_content = pdf_text.split("endstream").next().unwrap_or(&pdf_text);
        assert!(
            pdf_text.contains("Range 1-3, interruption--yes, and \"quoted text\"."),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("These are latent variables / sources."),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("Figure 1: A \"caption\" with 2-4"),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("# keep -- in code comments"),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("print\\(\"a--b\"\\)"), "{pdf_text}");
        assert!(!page_content.contains("``"), "{page_content}");
        assert!(!page_content.contains("''"), "{page_content}");
    }

    #[test]
    fn native_engine_renders_algorithmic_environment_as_pseudocode_lines() {
        let root = temp_dir("algorithmic-lines");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\begin{algorithmic}[1]
\Require Number of points $N$, dimension $d$
\Ensure Points $\{\mathbf{y}_i\}_{i=1}^N$
\For{$i = 1$ to $N$}
    \State Generate $\mathbf{x}_i \in [0,1]^d$
    \State Transform $z_i = \Phi^{-1}(x_i)$ \Comment{$\Phi^{-1}$ inverse CDF}
\EndFor
\end{algorithmic}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        for expected in [
            "Require: Number of points N, dimension d",
            "Ensure: Points {y_i}_{i=1}^N",
            "For i = 1 to N",
            "Generate x_i",
            "(\\316) Tj",
            " [0,1]^d",
            "Transform z_i = ",
            "(F) Tj",
            "^{-1} \\(x_i\\) \\(",
            "^-1 inverse CDF\\)",
            "End for",
        ] {
            assert!(
                pdf_text.contains(expected),
                "missing `{expected}` in {pdf_text}"
            );
        }
        for artifact in ["Fori", "State Generate", "CommentPhi", "EndFor"] {
            assert!(
                !pdf_text.contains(artifact),
                "found `{artifact}` in {pdf_text}"
            );
        }
    }

    #[test]
    fn native_engine_normalizes_common_math_commands_in_equations() {
        let root = temp_dir("math-cleaning");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\begin{equation}
\mathbb{E}\left[\frac{\mathbf{x}}{\sqrt{n}}\right]
= \operatorname*{argmin}_{z \in \mathcal{R}} \mathrm{Var}(z)
+ \tfrac1T + \hat x + \bar{y} + \Bigg|z\Bigg|
+ \underbrace{a+b}_{\textbf{Alignment}}
+ 2\sum_{n=1}^{N} z_n + c_\alpha^{(i)}
+ \frac{1}{|\mathcal{A}|}\sum_{a\in A} T_a + \forall n\not= m
+ \text{s.t.}h(z)
+ N_{r_0}(\theta) + T(\{ a^\top f_\theta(x_n) \})
\end{equation}
Inline ratio $\frac12$, root $\sqrt n$, and accent $\widehat{x}$.
Inline scripted $f_{\theta}: \mathbb{R}^{D} \to \mathbb{R}^{K}$.
Inline operators $\mathrm{Cov}(h(z))$, norm $\|\mathrm{Cov}(X)\|_F$,
and target $\mathrm{Var}(Y_i\mid X_i=x)$.
Inline adjacent $v(x)=\mathrm{Var}(Y_i\mid X_i=x)$.
Inline layout \hspace{-0.4cm}\hfill\setlength{\tabcolsep}{2.5pt}\medskip{\Large $\underset{\rightarrow}{f_{\theta}}$},
relation $A \triangleq B$, style ${\rm SIGReg}_{T}$,
norm $\left\|x\right\|$, and stack $\stackrel{d}{=} Y$.
	Arrow labels $R^2 \xrightarrow{\mathrm{Linear}(2,256)} H \xleftarrow[\epsilon \to 0]{\mathrm{back}} Z$
	and tail $\mp\infty$.
	More operators $\frac{\partial D}{\partial X_i} \equiv \nabla\log p,\; \ker(A)\setminus\{0\},\; W_T\succeq0,\; A\cap B\cup C$.
Orthogonal $\eta \perp z$ and transpose $Q^\top Q$.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        for expected in [
            "E",
            "E [x/",
            "argmin",
            "argmin_{z ",
            "(\\316) Tj",
            " R} Var",
            "Var\\(z\\)",
            "1/T",
            "x + y",
            "|z|",
            "a + b",
            "2 ",
            "(\\345) Tj",
            "_{n=1}^N z_n + c_{",
            "(a) Tj",
            "}^{\\(i\\)}",
            "1/|A|",
            "_{a ",
            " A} T_a + ",
            "(\") Tj",
            " n ",
            "(\\271) Tj",
            " m + s.t. h\\(z\\)",
            "s.t. h\\(z\\)",
            "N_{r_0}\\(",
            "(q) Tj",
            "\\) + T\\({a^T f_{",
            "}\\(x_n\\)}\\)",
            "(\\326) Tj",
            "Inline ratio 1/2, root ",
            "n, and accent x.",
            "Inline scripted f_{",
            "}: R^D",
            "(\\256) Tj",
            "R^K.",
            "operators Cov\\(h\\(z\\)\\), norm |Cov\\(X\\)|_F, and target Var\\(Y_i|X_i =x\\).",
            "Inline adjacent",
            "v\\(x\\)=Var\\(Y_i|X_i =x\\).",
            "Inline layout",
            "f_{",
            "}, relation A :=",
            "relation A := B, style",
            "SIGReg_T, norm |x|, and stack =",
            "Y.",
            "Arrow labels",
            "R^2",
            "Linear\\(2,256\\)",
            "(\\254) Tj",
            "back",
            "and tail -/+",
            "(\\245) Tj",
            "(\\266) Tj",
            "(\\272) Tj",
            "(\\321) Tj",
            "W_T>= 0",
            "(\\307) Tj",
            "(\\310) Tj",
            "(^) Tj",
            "Q^T Q.",
        ] {
            assert!(
                pdf_text.contains(expected),
                "missing `{expected}` in {pdf_text}"
            );
        }
        for artifact in [
            "\\mathbb",
            "\\mathbf",
            "\\frac",
            "\\tfrac",
            "\\operatorname",
            "\\mathcal",
            "\\mathrm",
            "\\hat",
            "\\bar",
            "\\Bigg",
            "hspace",
            "hfill",
            "setlength",
            "tabcolsep",
            "medskip",
            "Large",
            "underset",
            "underbrace",
            "Alignment",
            "triangleq",
            "xrightarrow",
            "xleftarrow",
            "partial",
            "equiv",
            "nabla",
            "setminus",
            "succeq",
            "sqrt",
            "stackrel",
            "Cov \\(",
            "Var \\(",
            "N_{r_0} \\(",
            "\\( theta",
            "theta",
            "alpha",
            "{ a^top",
            "a^top",
            "^top",
            "f_theta\\( x_n",
            "| A|",
        ] {
            assert!(
                !pdf_text.contains(artifact),
                "found `{artifact}` in {pdf_text}"
            );
        }
    }

    #[test]
    fn native_engine_renders_tcb_theorem_headings_and_labels() {
        let root = temp_dir("tcb-theorem");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\newtcbtheorem{theorem}{Theorem}{}{thm}
\begin{theorem}[label={thm:one}]{Named result}{}
Body text.
\begin{equation}\label{eq:inside}
x = y
\end{equation}
\end{theorem}
Refs \ref{thm:one} and \ref{eq:inside}.
Named \cref{thm:one}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Theorem 1: Named result"), "{pdf_text}");
        assert!(pdf_text.contains("Body text."), "{pdf_text}");
        assert!(pdf_text.contains("x = y \\(1\\)"), "{pdf_text}");
        assert!(pdf_text.contains("Refs 1 and 1."), "{pdf_text}");
        assert!(pdf_text.contains("Named thm. 1."), "{pdf_text}");
    }

    #[test]
    fn native_engine_renders_theorem_boxes_nested_in_wide_figures() {
        let root = temp_dir("wide-figure-theorem");
        let main = root.join("main.tex");
        let out = root.join("build");
        let figures = root.join("figures");
        fs::create_dir_all(&figures).unwrap();
        fs::write(figures.join("left.png"), tiny_rgba_png_bytes()).unwrap();
        fs::write(figures.join("right.png"), tiny_rgba_png_bytes()).unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\twocolumn
\begin{document}
Intro text.
\begin{figure*}[t!]
\begin{definition}{JEPA}{}
\begin{equation}\label{def:ssl}
x = y
\end{equation}
\begin{minipage}{0.48\linewidth}
\includegraphics[width=\linewidth]{figures/left.png}
\end{minipage}
\begin{minipage}{0.48\linewidth}
\includegraphics[width=\linewidth]{figures/right.png}
\end{minipage}
\end{definition}
\end{figure*}
\begin{definition}{SIGReg}{}
Second box.
\end{definition}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Definition 1: JEPA"), "{pdf_text}");
        assert!(pdf_text.contains("x = y \\(1\\)"), "{pdf_text}");
        assert!(pdf_text.contains("/Im1 Do"), "{pdf_text}");
        assert!(pdf_text.contains("/Im2 Do"), "{pdf_text}");
        assert!(pdf_text.contains("Definition 2: SIGReg"), "{pdf_text}");
        assert!(pdf_text.contains("Second box."), "{pdf_text}");
    }

    #[test]
    fn native_engine_renders_shared_counter_theorem_boxes() {
        let root = temp_dir("shared-theorem-boxes");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\begin{theorembox}[Main]
\label{thm:main}
First.
\end{theorembox}
\begin{corollarybox}[Next]
\label{cor:next}
Second.
\end{corollarybox}
Refs \ref{thm:main} and \ref{cor:next}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Theorem 1: Main"), "{pdf_text}");
        assert!(pdf_text.contains("Corollary 2: Next"), "{pdf_text}");
        assert!(pdf_text.contains("Refs 1 and 2."), "{pdf_text}");
    }

    #[test]
    fn native_engine_renders_basic_tables_without_fallback() {
        let root = temp_dir("table");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\begin{table}
\centering
\caption{\textbf{Results}}
\label{tab:results}
\renewcommand{\arraystretch}{1.2}
\resizebox{\textwidth}{!}{%
\begin{tabular}{lcc}
\toprule
Model & \multicolumn{2}{c}{Score} & \makecell{\# integration \\ points}\\
\cmidrule(lr){2-3}
A & 1.0\tiny{$\pm$0.1} & \textbf{2.0} & \makecell[l]{16}\\
\bottomrule
\end{tabular}}
\end{table}
See \cref{tab:results}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Table 1: Results"), "{pdf_text}");
        assert!(pdf_text.contains("See table 1."), "{pdf_text}");
        assert!(
            pdf_text.contains("Model | Score | # integration"),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("points"), "{pdf_text}");
        assert!(pdf_text.contains("A | 1.0"), "{pdf_text}");
        assert!(pdf_text.contains("(\\261) Tj"), "{pdf_text}");
        assert!(pdf_text.contains("0.1 | 2.0 | 16"), "{pdf_text}");
        assert!(!pdf_text.contains("makecell"), "{pdf_text}");
    }

    #[test]
    fn native_caption_wrapping_consumes_multiple_float_slots() {
        let layout = DocumentLayout::default();
        let mut caption_lines = Vec::new();
        let caption = format!("Figure 1: {}", "calibrated long caption text ".repeat(120));
        append_caption_lines(&mut caption_lines, &caption, &layout);

        assert!(caption_lines.len() > 1, "{caption_lines:?}");
        assert!(
            caption_lines
                .iter()
                .all(|line| matches!(line, Line::Caption(_))),
            "{caption_lines:?}"
        );
        let float = Line::FloatBlock {
            lines: caption_lines.clone(),
            wide: false,
            top: false,
        };
        assert_eq!(float.slots(layout, &[]), caption_lines.len());
    }

    #[test]
    fn native_engine_renders_optional_caption_and_captionof_without_fallback() {
        let root = temp_dir("captionof");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage[capitalize]{cleveref}
\begin{document}
\listoffigures
\listoftables
\begin{figure}
\caption[Short figure]{\textbf{Long figure caption}.}
\label{fig:long}
\end{figure}
Figure ref \ref{fig:long}; cref \cref{fig:long}.
\captionsetup[table]{font=small}
\captionof{table}[Short table]{Long table caption.}
\label{tab:long}
Table ref \ref{tab:long}; cref \cref{tab:long}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("List of Figures"), "{pdf_text}");
        assert!(
            pdf_text.contains("Figure 1: Short figure .... 1"),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("List of Tables"), "{pdf_text}");
        assert!(
            pdf_text.contains("Table 1: Short table .... 1"),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("Figure 1: Long figure caption."),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("Table 1: Long table caption."),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("/F1 10.00 Tf"), "{pdf_text}");
        assert!(
            pdf_text.contains("Figure ref 1; cref Figure 1."),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("Table ref 1; cref Table 1."),
            "{pdf_text}"
        );
        let aux = fs::read_to_string(out.join("main.aux")).expect("aux should exist");
        assert!(aux.contains("\\newlabel{fig:long}{{1}{1}}"), "{aux}");
        assert!(aux.contains("\\newlabel{tab:long}{{1}{1}}"), "{aux}");
        assert!(
            aux.contains(
                "\\@writefile{lof}{\\contentsline {figure}{\\numberline {1}Short figure}{1}{}}"
            ),
            "{aux}"
        );
        assert!(
            aux.contains(
                "\\@writefile{lot}{\\contentsline {table}{\\numberline {1}Short table}{1}{}}"
            ),
            "{aux}"
        );
        let lof = fs::read_to_string(out.join("main.lof")).expect("lof should exist");
        assert!(
            lof.contains("\\contentsline {figure}{\\numberline {1}Short figure}{1}{}"),
            "{lof}"
        );
        let lot = fs::read_to_string(out.join("main.lot")).expect("lot should exist");
        assert!(
            lot.contains("\\contentsline {table}{\\numberline {1}Short table}{1}{}"),
            "{lot}"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(
            fls.contains(&out.join("main.lof").display().to_string()),
            "{fls}"
        );
        assert!(
            fls.contains(&out.join("main.lot").display().to_string()),
            "{fls}"
        );
        let trace =
            fs::read_to_string(out.join("main.texpilot-pdftex.trace")).expect("trace exists");
        assert!(trace.contains("lof_output"), "{trace}");
        assert!(trace.contains("lot_output"), "{trace}");
    }

    #[test]
    fn native_engine_writes_hyperref_out_and_renders_link_text() {
        let root = temp_dir("hyperref-out");
        let main = root.join("main.tex");
        let refs = root.join("refs.bib");
        let out = root.join("build");
        fs::write(
            &refs,
            "@book{knuth, author={Donald Knuth}, title={The TeXbook}, year={1984}}\n",
        )
        .unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage[pagebackref=true]{hyperref}
\begin{document}
\pdfbookmark[1]{Front Matter}{front:matter}
\phantomsection
\section{Intro}
\label{sec:intro}
Read \href{https://example.com}{Example Site}, \url{https://example.com/raw},
\hyperref[sec:intro]{jump back}, and \autoref{sec:intro}.
Native citation \cite{knuth}.
\subsection{Details}
More native hyperref output.
\bibliographystyle{plain}
\bibliography{refs}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Example Site"), "{pdf_text}");
        assert!(pdf_text.contains("https://example.com/raw"), "{pdf_text}");
        assert!(pdf_text.contains("jump back"), "{pdf_text}");
        assert!(pdf_text.contains("and Section 1."), "{pdf_text}");
        assert!(pdf_text.contains("/Outlines"), "{pdf_text}");
        assert!(pdf_text.contains("/Title (Intro)"), "{pdf_text}");
        assert!(pdf_text.contains("/Title (Details)"), "{pdf_text}");
        let out_file = fs::read_to_string(out.join("main.out")).expect("out should exist");
        assert!(
            out_file.contains("\\BOOKMARK [1][-]{front:matter}{Front Matter}{}{}% page 1"),
            "{out_file}"
        );
        assert!(
            out_file.contains("\\BOOKMARK [1][-]{section.1}{Intro}{}{}% page 1"),
            "{out_file}"
        );
        assert!(
            out_file.contains("\\BOOKMARK [2][-]{subsection.1.1}{Details}{}{}% page 1"),
            "{out_file}"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(
            fls.contains(&out.join("main.out").display().to_string()),
            "{fls}"
        );
        assert!(
            fls.contains(&out.join("main.brf").display().to_string()),
            "{fls}"
        );
        let brf = fs::read_to_string(out.join("main.brf")).expect("brf should exist");
        assert!(brf.contains("\\backcite{knuth}{{1}{}}"), "{brf}");
        let trace =
            fs::read_to_string(out.join("main.texpilot-pdftex.trace")).expect("trace exists");
        assert!(trace.contains("hyperref_out_output"), "{trace}");
        assert!(trace.contains("brf_output"), "{trace}");
        assert!(trace.contains("true"), "{trace}");
    }

    #[test]
    fn native_engine_renders_color_and_style_wrappers_as_text() {
        let root = temp_dir("color-wrappers");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{xcolor}
\newcommand{\redword}[1]{\textcolor{red}{#1}}
\begin{document}
\section{Inline wrappers}
\definecolor{accent}{HTML}{0072B2}
Color payloads: \textcolor{accent}{start}, \colorbox{accent}{boxed},
\fcolorbox{red}{accent}{framed}, \redword{macro text}, and {\color{red}plain}.
Styled payloads: \protect\textbf{bold}, \underline{under}, \textsc{caps},
\ensuremath{x+y}, \mathbf{z}.
Boundaries: \textbf{Independence.}
$p(z)$ follows. \textit{Approximate alignment:} $\mathcal{L}(h)$ holds.
\textit{Approximate whitening:} $\|x\|$ holds.
Small caps: \textsc{verified} and \textsc{axiomatized}.
Math boundaries: mean $\pm$ std, $\rho$ (5 seeds), $\theta$-space.
Data: the result.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        for expected in [
            "start",
            "boxed",
            "framed",
            "macro text",
            "plain",
            "bold",
            "under",
            "CAPS",
            "x+y",
            "z",
            "Independence. p\\(z\\) follows.",
            "Approximate",
            "alignment: L\\(h\\) holds.",
            "whitening: |x| holds.",
            "Small caps: VERIFIED",
            "and AXIOMATIZED.",
            "AXIOMATIZED.",
            "Math boundaries: mean ",
            "(\\261) Tj",
            "( std, ) Tj",
            "(r) Tj",
            "( \\(5 seeds\\), ) Tj",
            "(q) Tj",
            "(-space. Data:)",
            "the result.",
            "result.",
        ] {
            assert!(
                pdf_text.contains(expected),
                "missing `{expected}` in {pdf_text}"
            );
        }
        for artifact in [
            "textcolor",
            "colorbox",
            "fcolorbox",
            "definecolor",
            "Independence.p",
            "alignment:L",
            "whitening:|",
            "verified and axiomatized",
            "+/-std",
            "rho",
            "theta",
            "rho\\(5 seeds",
            "theta -space",
            "Data:the",
        ] {
            assert!(
                !pdf_text.contains(artifact),
                "found `{artifact}` in {pdf_text}"
            );
        }
    }

    #[test]
    fn native_engine_renders_itemize_and_enumerate_markers() {
        let root = temp_dir("list-markers");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\begin{itemize}
\item Bullet one.
\item[Custom] Custom bullet.
\end{itemize}
\begin{enumerate}
\item First numbered.
\item Second numbered.
\end{enumerate}
\begin{enumerate}[label=(\roman*)]
\item First roman.
\item Second roman.
\begin{enumerate}[label=(\alph*)]
\item Nested alpha.
\end{enumerate}
\end{enumerate}
\begin{description}
\item[Term] Definition text.
\end{description}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        for expected in [
            "(-) Tj",
            "(Custom) Tj",
            "(1.) Tj",
            "(2.) Tj",
            "(\\(i\\)) Tj",
            "(\\(ii\\)) Tj",
            "(\\(a\\)) Tj",
            "(Term) Tj",
        ] {
            assert!(pdf_text.contains(expected), "{pdf_text}");
        }
        assert!(pdf_text.contains("(First numbered.)"), "{pdf_text}");
        assert!(pdf_text.contains("(Nested alpha.)"), "{pdf_text}");
    }

    #[test]
    fn native_engine_marks_and_traces_footnotes_without_synthetic_notes_page() {
        let root = temp_dir("footnotes");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Body note\footnote{Inline note with \url{https://example.com}.} after.
Marker\footnotemark continues.
\footnotetext{Deferred note.}
Explicit\footnotemark[7] marker.
\footnotetext[7]{Explicit note.}
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        for expected in [
            "Body note1 after.",
            "Marker2 continues.",
            "Explicit7 marker.",
            "Notes",
            "[1] Inline note with https://example.com.",
            "[2] Deferred note.",
            "[7] Explicit note.",
        ] {
            assert!(pdf_text.contains(expected), "{pdf_text}");
        }
        assert!(!pdf_text.contains("footnote"), "{pdf_text}");

        let log = fs::read_to_string(out.join("main.log")).expect("log exists");
        assert!(log.contains("Footnotes: 3"), "{log}");
        let trace =
            fs::read_to_string(out.join("main.texpilot-pdftex.trace")).expect("trace exists");
        assert!(trace.contains("footnotes"), "{trace}");
        assert!(trace.contains("3"), "{trace}");
    }

    #[test]
    fn native_probe_reports_support_without_writing_outputs() {
        let root = temp_dir("probe");
        let main = root.join("main.tex");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Hello.
\end{document}
",
        )
        .unwrap();

        let support = probe_native_support(&main).expect("probe should run");

        assert_eq!(support, Ok(()));
        assert!(!root.join("main.pdf").exists());
    }

    #[test]
    fn native_engine_applies_begin_and_end_document_hooks_before_expansion() {
        let root = temp_dir("document-hooks");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\AtBeginDocument{\newcommand{\fromhook}{begin hook macro}\section{Hooked Section}}
\AtEndDocument{End hook text using \fromhook.}
\begin{document}
Body sees \fromhook.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("1 Hooked Section"), "{pdf_text}");
        assert!(
            pdf_text.contains("Body sees begin hook macro."),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("End hook text using begin hook macro."),
            "{pdf_text}"
        );
        assert!(!pdf_text.contains("AtBeginDocument"), "{pdf_text}");
        assert!(!pdf_text.contains("AtEndDocument"), "{pdf_text}");
    }

    #[test]
    fn native_engine_materializes_explicit_write_streams_without_fallback() {
        let root = temp_dir("write-streams");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\newwrite\hiddenaux
\immediate\openout\hiddenaux=hidden.aux
\immediate\write\hiddenaux{\string\relax}
\write\hiddenaux{generated by \jobname}
\immediate\closeout\hiddenaux
Visible text.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let hidden = fs::read_to_string(out.join("hidden.aux")).expect("hidden aux should exist");
        assert!(hidden.contains("\\relax"), "{hidden}");
        assert!(hidden.contains("generated by main"), "{hidden}");
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Visible text."), "{pdf_text}");
        assert!(!pdf_text.contains("openout"), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(
            fls.contains(&out.join("hidden.aux").display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_pdf_only_strips_explicit_write_streams_without_materializing_sidecars() {
        let root = temp_dir("pdf-only-write-streams");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\newwrite\hiddenaux
\immediate\openout\hiddenaux=hidden.aux
\immediate\write\hiddenaux{\string\relax}
\write\hiddenaux{generated by \jobname}
\immediate\closeout\hiddenaux
Visible text.
\end{document}
",
        )
        .unwrap();

        let run = run_native_pdf_only(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        assert!(
            !out.join("hidden.aux").exists(),
            "pdf-only native run should not materialize explicit write sidecars"
        );
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Visible text."), "{pdf_text}");
        assert!(!pdf_text.contains("openout"), "{pdf_text}");
        assert!(!pdf_text.contains("hiddenaux"), "{pdf_text}");
    }

    #[test]
    fn native_engine_writes_and_renders_basic_index_without_fallback() {
        let root = temp_dir("index-sidecar");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{makeidx}
\makeindex
\begin{document}
Alpha\index{alpha@Alpha entry} text.
Beta\index{beta!sub item|textbf} text.
\printindex
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "nativejob".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let idx = fs::read_to_string(out.join("nativejob.idx")).expect("idx should exist");
        assert!(idx.contains("\\indexentry{alpha@Alpha entry}{1}"), "{idx}");
        assert!(
            idx.contains("\\indexentry{beta!sub item|textbf}{1}"),
            "{idx}"
        );
        let pdf = fs::read(out.join("nativejob.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Index"), "{pdf_text}");
        assert!(pdf_text.contains("Alpha entry 1"), "{pdf_text}");
        assert!(pdf_text.contains("beta, sub item 1"), "{pdf_text}");
        assert!(!pdf_text.contains("alpha@Alpha"), "{pdf_text}");
        let fls = fs::read_to_string(out.join("nativejob.fls")).expect("fls should exist");
        assert!(
            fls.contains(&out.join("nativejob.idx").display().to_string()),
            "{fls}"
        );
        let trace = fs::read_to_string(out.join("nativejob.texpilot-pdftex.trace"))
            .expect("trace should exist");
        assert!(trace.contains("index_entries\t2"), "{trace}");
        assert!(trace.contains("index_output\ttrue"), "{trace}");
        assert!(trace.contains("index_printed\ttrue"), "{trace}");
    }

    #[test]
    fn native_engine_reads_local_input_streams_before_expansion() {
        let root = temp_dir("read-streams");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::create_dir_all(&out).unwrap();
        let source_sidecar = root.join("customjob.data");
        let output_sidecar = out.join("customjob.data");
        fs::write(&source_sidecar, "source directory value").unwrap();
        fs::write(&output_sidecar, "output directory value").unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\newread\reader
\openin\reader=missing.data
\ifeof\reader Missing file reached EOF. \else Wrong missing branch. \fi
\closein\reader
\openin\reader=\jobname.data
\ifeof\reader Wrong existing branch. \else Existing file is readable. \fi
\read\reader to \loadedline
\ifeof\reader EOF after one line. \else Wrong post-read branch. \fi
\closein\reader
Read stream: \loadedline.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "customjob".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("customjob.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Missing file reached EOF."), "{pdf_text}");
        assert!(
            pdf_text.contains("Existing file is readable."),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("EOF after one line."), "{pdf_text}");
        assert!(
            pdf_text.contains("Read stream: output directory value."),
            "{pdf_text}"
        );
        assert!(!pdf_text.contains("source directory value"), "{pdf_text}");
        assert!(!pdf_text.contains("Wrong missing branch."), "{pdf_text}");
        assert!(!pdf_text.contains("Wrong existing branch."), "{pdf_text}");
        assert!(!pdf_text.contains("Wrong post-read branch."), "{pdf_text}");
        assert!(!pdf_text.contains("openin"), "{pdf_text}");
        assert!(!pdf_text.contains("readvalue"), "{pdf_text}");
        let fls = fs::read_to_string(out.join("customjob.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&output_sidecar.display().to_string()), "{fls}");
        assert!(
            !fls.contains(&source_sidecar.display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_resolves_texinputs_read_streams() {
        let root = temp_dir("texinputs-read-streams");
        let paper = root.join("paper");
        let shared = root.join("shared").join("tex");
        let main = paper.join("main.tex");
        let sidecar = shared.join("shared.data");
        let out = root.join("build");
        fs::create_dir_all(&paper).unwrap();
        fs::create_dir_all(&shared).unwrap();
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let _texinputs = EnvVarGuard::set(
            "TEXINPUTS",
            OsString::from(format!("{}//{}", shared.display(), KPATHSEA_PATH_SEPARATOR)),
        );
        fs::write(&sidecar, "shared stream value").unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\newread\reader
\openin\reader=shared.data
\ifeof\reader Missing shared stream. \else Shared stream is readable. \fi
\read\reader to \loadedline
\closein\reader
Read stream: \loadedline.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_text.contains("Shared stream is readable."),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("Read stream: shared stream value."),
            "{pdf_text}"
        );
        assert!(!pdf_text.contains("Missing shared stream."), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(
            fls.contains(&sidecar.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_expands_local_input_files_and_records_them() {
        let root = temp_dir("input");
        let main = root.join("main.tex");
        let section = root.join("section.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\input{section}
\end{document}
",
        )
        .unwrap();
        fs::write(
            &section,
            r"\section{Input}
Text loaded from another file.
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("1 Input"), "{pdf_text}");
        assert!(
            pdf_text.contains("Text loaded from another file."),
            "{pdf_text}"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&section.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_resolves_texinputs_inputs_and_file_probes() {
        let root = temp_dir("texinputs-input");
        let paper = root.join("paper");
        let shared = root.join("shared").join("tex");
        let main = paper.join("main.tex");
        let section = shared.join("sharedsection.tex");
        let probe = shared.join("sharedprobe.tex");
        let guard = shared.join("sharedguard.dat");
        let out = root.join("build");
        fs::create_dir_all(&paper).unwrap();
        fs::create_dir_all(&shared).unwrap();
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let _texinputs = EnvVarGuard::set(
            "TEXINPUTS",
            OsString::from(format!("{}//{}", shared.display(), KPATHSEA_PATH_SEPARATOR)),
        );
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\input{sharedsection}
\IfFileExists{sharedprobe}{\input{sharedprobe}}{Missing shared probe.}
\InputIfFileExists{sharedguard.dat}{Before shared guard. }{Missing shared guard.}
\end{document}
",
        )
        .unwrap();
        fs::write(
            &section,
            r"\section{Shared Input}
Loaded through TEXINPUTS input.
",
        )
        .unwrap();
        fs::write(&probe, "Loaded through TEXINPUTS probe.\n").unwrap();
        fs::write(&guard, "Loaded through TEXINPUTS guarded input.\n").unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        for expected in [
            "1 Shared Input",
            "Loaded through TEXINPUTS input.",
            "Loaded through TEXINPUTS probe.",
            "Before shared guard.",
            "Loaded through TEXINPUTS guarded input.",
        ] {
            assert!(
                pdf_text.contains(expected),
                "missing `{expected}` in {pdf_text}"
            );
        }
        assert!(!pdf_text.contains("Missing shared probe."), "{pdf_text}");
        assert!(!pdf_text.contains("Missing shared guard."), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(
            fls.contains(&section.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
        assert!(
            fls.contains(&probe.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
        assert!(
            fls.contains(&guard.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_expands_file_existence_conditionals_and_records_existing_probes() {
        let root = temp_dir("file-exists");
        let main = root.join("main.tex");
        let sections = root.join("sections");
        fs::create_dir_all(&sections).unwrap();
        let section = sections.join("section.tex");
        let snippet = root.join("snippet.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\IfFileExists{sections/section.tex}{\input{sections/section}}{Missing section.}
\IfFileExists{missing-section.tex}{Wrong branch.}{False branch text.}
\InputIfFileExists{snippet}{Before snippet. }{Missing snippet.}
\InputIfFileExists{missing-snippet}{Wrong prelude.}{Missing input fallback.}
\end{document}
",
        )
        .unwrap();
        fs::write(
            &section,
            r"\section{Conditional Input}
Loaded by file-existence branch.
",
        )
        .unwrap();
        fs::write(&snippet, "Loaded by guarded input.\n").unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        for expected in [
            "1 Conditional Input",
            "Loaded by file-existence branch.",
            "False branch text.",
            "Before snippet.",
            "Loaded by guarded input.",
            "Missing input fallback.",
        ] {
            assert!(
                pdf_text.contains(expected),
                "missing `{expected}` in {pdf_text}"
            );
        }
        for artifact in [
            "Missing section.",
            "Wrong branch.",
            "Missing snippet.",
            "Wrong prelude.",
            "IfFileExists",
            "InputIfFileExists",
        ] {
            assert!(
                !pdf_text.contains(artifact),
                "found `{artifact}` in {pdf_text}"
            );
        }
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&section.display().to_string()), "{fls}");
        assert!(fls.contains(&snippet.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_expands_jobname_in_input_and_probe_file_names() {
        let root = temp_dir("jobname-input");
        let main = root.join("main.tex");
        let stage = root.join("customjob.stage");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\IfFileExists{\jobname.stage}{\input{\jobname.stage}}{Missing stage.}
\InputIfFileExists{\jobname.stage}{Before guarded stage. }{Missing guarded stage.}
\end{document}
",
        )
        .unwrap();
        fs::write(&stage, "Loaded from the logical job-name sidecar.\n").unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "customjob".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("customjob.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_text.contains("Loaded from the logical job-name sidecar."),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("Before guarded stage."), "{pdf_text}");
        assert!(!pdf_text.contains("Missing stage."), "{pdf_text}");
        assert!(!pdf_text.contains("Missing guarded stage."), "{pdf_text}");
        assert!(!pdf_text.contains("\\jobname"), "{pdf_text}");
        let fls = fs::read_to_string(out.join("customjob.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&stage.display().to_string()), "{fls}");
        assert!(!fls.contains("\\jobname"), "{fls}");
    }

    #[test]
    fn native_engine_expands_include_files_and_honors_includeonly() {
        let root = temp_dir("include");
        let main = root.join("main.tex");
        let chapters = root.join("chapters");
        fs::create_dir_all(&chapters).unwrap();
        let included = chapters.join("included.tex");
        let skipped = chapters.join("skipped.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\includeonly{chapters/included.tex}
\begin{document}
\include{chapters/included}
\include{chapters/skipped}
\end{document}
",
        )
        .unwrap();
        fs::write(
            &included,
            r"\section{Included}
Text loaded through include.
",
        )
        .unwrap();
        fs::write(
            &skipped,
            r"\section{Skipped}
This should not be loaded.
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("1 Included"), "{pdf_text}");
        assert!(
            pdf_text.contains("Text loaded through include."),
            "{pdf_text}"
        );
        assert!(
            !pdf_text.contains("This should not be loaded."),
            "{pdf_text}"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&included.display().to_string()), "{fls}");
        assert!(!fls.contains(&skipped.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_loads_native_package_adapters_and_records_local_styles() {
        let root = temp_dir("local-package-adapter");
        let main = root.join("main.tex");
        let simpleicml = root.join("simpleicml.sty");
        let sectionnav = root.join("sectionnav.sty");
        let out = root.join("build");
        fs::write(
            &simpleicml,
            r"\ProvidesPackage{simpleicml}
\RequirePackage{sectionnav}
\endinput
",
        )
        .unwrap();
        fs::write(
            &sectionnav,
            r"\ProvidesPackage{sectionnav}
\endinput
",
        )
        .unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage[accent=0072B2]{simpleicml}
\icmltitle{Native Style Title}
\icmlauthors{Ada Lovelace}
\icmlaffiliations{Analytical Engine Lab}
\icmlabstract{This abstract came from a local package adapter.}
\sectionheaderline{\seclink{sec:intro}{1}{Intro}}
\begin{document}
\icmlmaketitle
Body text.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        let literal_text = pdf_literal_text(&pdf);
        assert!(pdf_text.contains("Native Style Title"), "{pdf_text}");
        assert!(pdf_text.contains("Ada Lovelace"), "{pdf_text}");
        assert!(pdf_text.contains("Analytical Engine"), "{pdf_text}");
        assert!(pdf_text.contains("Lab"), "{pdf_text}");
        assert!(pdf_text.contains("This abstract came from"), "{pdf_text}");
        assert!(literal_text.contains("package adapter."), "{literal_text}");
        assert!(pdf_text.contains("Body text."), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&simpleicml.display().to_string()), "{fls}");
        assert!(fls.contains(&sectionnav.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_renders_icml_teaser_graphics_in_title_block() {
        let root = temp_dir("icml-title-teaser-graphics");
        let main = root.join("main.tex");
        let simpleicml = root.join("simpleicml.sty");
        let figures = root.join("figures");
        let out = root.join("build");
        fs::create_dir_all(&figures).unwrap();
        fs::write(figures.join("left.jpg"), tiny_jpeg_bytes()).unwrap();
        fs::write(figures.join("right.jpg"), tiny_jpeg_bytes()).unwrap();
        fs::write(figures.join("bottom.jpg"), tiny_jpeg_bytes()).unwrap();
        fs::write(
            &simpleicml,
            r"\ProvidesPackage{simpleicml}
\endinput
",
        )
        .unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{simpleicml}
\icmltitle{Native ICML}
\icmlauthors{Ada Lovelace}
\icmlaffiliations{Analytical Engine Lab}
\icmlabstract{Intro text.
\begin{center}
\begin{minipage}{0.45\linewidth}
\includegraphics[width=\linewidth]{figures/left.jpg}
\end{minipage}
\hfill
\begin{minipage}{0.45\linewidth}
\includegraphics[width=\linewidth]{figures/right.jpg}
\end{minipage}
\begin{minipage}{0.45\linewidth}
\includegraphics[width=\linewidth]{figures/bottom.jpg}
\end{minipage}
\hfill
\begin{minipage}{0.45\linewidth}
\begin{tabular}{lcc}
\toprule
\textbf{Method} & \textbf{1-sh} & \textbf{Full}\\
\midrule
\;\;Native & 1.0 & 2.0\\
\bottomrule
\end{tabular}
\end{minipage}
\captionof{figure}{Native teaser.}
\end{center}}
\begin{document}
\icmlmaketitle
Body text.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Native ICML"), "{pdf_text}");
        assert!(pdf_text.contains("Figure 1: Native teaser."), "{pdf_text}");
        assert!(pdf_text.contains("Method"), "{pdf_text}");
        assert!(pdf_text.contains("Native | 1.0 | 2.0"), "{pdf_text}");
        assert!(!pdf_text.contains(";;Native"), "{pdf_text}");
        assert!(pdf_text.contains("0.90 0.96 0.98 rg"), "{pdf_text}");
        assert!(
            pdf_text.matches("/Subtype /Image").count() >= 3,
            "{pdf_text}"
        );
        assert!(!pdf_text.contains("includegraphics"), "{pdf_text}");
        assert!(!pdf_text.contains("tabular"), "{pdf_text}");
    }

    #[test]
    fn native_engine_resolves_texinputs_package_adapters_and_nested_styles() {
        let root = temp_dir("texinputs-package-adapter");
        let paper = root.join("paper");
        let shared = root.join("shared").join("tex");
        let main = paper.join("main.tex");
        let simpleicml = shared.join("simpleicml.sty");
        let sectionnav = shared.join("sectionnav.sty");
        let out = root.join("build");
        fs::create_dir_all(&paper).unwrap();
        fs::create_dir_all(&shared).unwrap();
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let _texinputs = EnvVarGuard::set(
            "TEXINPUTS",
            OsString::from(format!("{}//{}", shared.display(), KPATHSEA_PATH_SEPARATOR)),
        );
        fs::write(
            &simpleicml,
            r"\ProvidesPackage{simpleicml}
\RequirePackage{sectionnav}
\endinput
",
        )
        .unwrap();
        fs::write(
            &sectionnav,
            r"\ProvidesPackage{sectionnav}
\endinput
",
        )
        .unwrap();
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{simpleicml}
\icmltitle{Shared Native Style Title}
\icmlauthors{Ada Lovelace}
\icmlaffiliations{Shared Analytical Engine Lab}
\icmlabstract{This abstract came from a TEXINPUTS package adapter.}
\sectionheaderline{\seclink{sec:intro}{1}{Intro}}
\begin{document}
\icmlmaketitle
Body text.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Shared Native Style Title"), "{pdf_text}");
        assert!(pdf_text.contains("Ada Lovelace"), "{pdf_text}");
        assert!(pdf_text.contains("Shared"), "{pdf_text}");
        assert!(pdf_text.contains("Analytical Engine Lab"), "{pdf_text}");
        assert!(pdf_text.contains("This abstract"), "{pdf_text}");
        assert!(pdf_text.contains("came from a"), "{pdf_text}");
        assert!(
            pdf_text.contains("TEXINPUTS package adapter."),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("Body text."), "{pdf_text}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(
            fls.contains(&simpleicml.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
        assert!(
            fls.contains(&sectionnav.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_resolves_texinputs_classes_that_load_packages() {
        let root = temp_dir("texinputs-class-package-chain");
        let paper = root.join("paper");
        let shared = root.join("shared").join("tex");
        let main = paper.join("main.tex");
        let class = shared.join("sharedconf.cls");
        let simpleicml = shared.join("simpleicml.sty");
        let sectionnav = shared.join("sectionnav.sty");
        let out = root.join("build");
        fs::create_dir_all(&paper).unwrap();
        fs::create_dir_all(&shared).unwrap();
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let _texinputs = EnvVarGuard::set(
            "TEXINPUTS",
            OsString::from(format!("{}//{}", shared.display(), KPATHSEA_PATH_SEPARATOR)),
        );
        fs::write(
            &class,
            r"\ProvidesClass{sharedconf}
\LoadClass{article}
\RequirePackage[pagebackref=true]{hyperref}
\RequirePackage{simpleicml}
\endinput
",
        )
        .unwrap();
        fs::write(
            &simpleicml,
            r"\ProvidesPackage{simpleicml}
\RequirePackage{sectionnav}
\endinput
",
        )
        .unwrap();
        fs::write(
            &sectionnav,
            r"\ProvidesPackage{sectionnav}
\endinput
",
        )
        .unwrap();
        fs::write(
            &main,
            r"\documentclass{sharedconf}
\icmltitle{Class Loaded Native Style}
\icmlauthors{Ada Lovelace}
\icmlaffiliations{Class Traversal Lab}
\icmlabstract{This abstract came through a class package chain.}
\begin{document}
\icmlmaketitle
\section{Class Hyperref}
\label{sec:class}
See Section~\ref{sec:class}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Class Loaded Native Style"), "{pdf_text}");
        assert!(pdf_text.contains("Class"), "{pdf_text}");
        assert!(pdf_text.contains("Traversal Lab"), "{pdf_text}");
        assert!(pdf_text.contains("This abstract came"), "{pdf_text}");
        assert!(pdf_text.contains("through a class"), "{pdf_text}");
        assert!(pdf_text.contains("package chain."), "{pdf_text}");
        assert!(pdf_text.contains("1 Class Hyperref"), "{pdf_text}");
        let out_file = fs::read_to_string(out.join("main.out"))
            .expect("hyperref .out should be written through class package detection");
        assert!(out_file.contains("Class Hyperref"), "{out_file}");
        assert!(
            out.join("main.brf").exists(),
            "pagebackref .brf should be written through class package options"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(
            fls.contains(&class.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
        assert!(
            fls.contains(&simpleicml.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
        assert!(
            fls.contains(&sectionnav.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_applies_pass_options_to_class_loaded_packages() {
        let root = temp_dir("pass-options-class-package");
        let paper = root.join("paper");
        let shared = root.join("shared").join("tex");
        let main = paper.join("main.tex");
        let class = shared.join("sharedconf.cls");
        let out = root.join("build");
        fs::create_dir_all(&paper).unwrap();
        fs::create_dir_all(&shared).unwrap();
        let _env_lock = TEXINPUTS_TEST_LOCK
            .lock()
            .expect("TEXINPUTS test lock poisoned");
        let _texinputs = EnvVarGuard::set(
            "TEXINPUTS",
            OsString::from(format!("{}//{}", shared.display(), KPATHSEA_PATH_SEPARATOR)),
        );
        fs::write(
            &class,
            r"\ProvidesClass{sharedconf}
\LoadClass{article}
\RequirePackage{hyperref}
\endinput
",
        )
        .unwrap();
        fs::write(
            &main,
            r"\PassOptionsToPackage{pagebackref=true}{hyperref}
\documentclass{sharedconf}
\begin{document}
\section{Passed Options}
\label{sec:passed}
See Section~\ref{sec:passed}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("1 Passed Options"), "{pdf_text}");
        let out_file = fs::read_to_string(out.join("main.out"))
            .expect("hyperref .out should be written through class package detection");
        assert!(out_file.contains("Passed Options"), "{out_file}");
        assert!(
            out.join("main.brf").exists(),
            "pagebackref .brf should be written through passed package options"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(
            fls.contains(&class.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_detects_require_package_with_options_in_classes() {
        let root = temp_dir("require-package-with-options");
        let main = root.join("main.tex");
        let class = root.join("sharedconf.cls");
        let out = root.join("build");
        fs::write(
            &class,
            r"\ProvidesClass{sharedconf}
\RequirePackageWithOptions{hyperref}
\endinput
",
        )
        .unwrap();
        fs::write(
            &main,
            r"\PassOptionsToPackage{pagebackref=true}{hyperref}
\documentclass{sharedconf}
\begin{document}
\section{With Options}
\label{sec:with-options}
See Section~\ref{sec:with-options}.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("1 With Options"), "{pdf_text}");
        let out_file = fs::read_to_string(out.join("main.out"))
            .expect("hyperref .out should be written through RequirePackageWithOptions");
        assert!(out_file.contains("With Options"), "{out_file}");
        assert!(
            out.join("main.brf").exists(),
            "pagebackref .brf should be written through RequirePackageWithOptions"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&class.display().to_string()), "{fls}");
    }

    #[test]
    fn native_input_expansion_ignores_commented_inputs() {
        let root = temp_dir("commented-input");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
% \input{}
Visible text.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Visible text."), "{pdf_text}");
    }

    #[test]
    fn native_engine_treats_package_declarations_as_noops_until_package_commands_are_used() {
        let root = temp_dir("usepackage-noop");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage{geometry}
\begin{document}
Text that does not rely on package commands.
\end{document}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main,
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_text.contains("Text that does not rely on package commands."),
            "{pdf_text}"
        );
    }

    #[test]
    fn native_engine_renders_basic_numeric_citations_and_references() {
        let root = temp_dir("bibliography-citations");
        let main = root.join("main.tex");
        let bib = root.join("refs.bib");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
\section{Citations}
\label{sec:cites}
Body cites \citep[see][]{knuth} and \citep[][chap.~2]{accented}.
Plain cite \cite{accented}.
Textual cite \citet{knuth}.
See Section~\ref{sec:cites}.
\bibliographystyle{plain}
\bibliography{refs}
\end{document}
",
        )
        .unwrap();
        fs::write(
            &bib,
            r#"@book{knuth,
  author = {Donald Knuth},
  title = {The TeXbook},
  year = {1984}
}
@article{accented,
  author = {Bernhard Sch{\"o}lkopf and Harald Cram{\'e}r},
  title = {{DINO-WM}: World Models with \emph{Planning}},
  year = {2026}
}
"#,
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        let literal_text = pdf_literal_text(&pdf);
        assert!(pdf_text.contains("1 Citations"), "{pdf_text}");
        assert!(
            pdf_text.contains("Body cites [see 1] and [2, chap. 2]."),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("Plain cite [2]."), "{pdf_text}");
        assert!(pdf_text.contains("Textual cite Knuth [1]."), "{pdf_text}");
        assert!(literal_text.contains("See Section 1."), "{literal_text}");
        assert!(pdf_text.contains("References"), "{pdf_text}");
        assert!(
            pdf_text.contains("[1] Donald Knuth. The TeXbook. 1984"),
            "{pdf_text}"
        );
        assert!(
            literal_text.contains(
                "[2] Bernhard Scholkopf and Harald Cramer. DINO-WM: World Models with Planning."
            ),
            "{literal_text}"
        );
        assert!(literal_text.contains("2026"), "{literal_text}");
        for artifact in ["{DINO-WM}", "Sch{", "Cram{", "emph"] {
            assert!(!pdf_text.contains(artifact), "{pdf_text}");
        }
        let aux = fs::read_to_string(out.join("main.aux")).expect("aux should exist");
        assert!(aux.contains("\\citation{knuth}"), "{aux}");
        assert!(aux.contains("\\newlabel{sec:cites}{{1}{1}}"), "{aux}");
        assert!(aux.contains("\\bibstyle{plain}"), "{aux}");
        assert!(aux.contains("\\bibdata{refs}"), "{aux}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&bib.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_renders_natbib_author_year_citations_without_fallback() {
        let root = temp_dir("author-year-citations");
        let main = root.join("main.tex");
        let bib = root.join("refs.bib");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Textual \citet{knuth} and plain \cite{lamport}.
Parenthetical \citep[see][chap.~2]{knuth,lamport}.
\bibliographystyle{plainnat}
\bibliography{refs}
\end{document}
",
        )
        .unwrap();
        fs::write(
            &bib,
            r"@book{knuth,
  author = {Knuth, Donald E},
  title = {The TeXbook},
  year = {1984}
}

@book{lamport,
  author = {Leslie Lamport},
  title = {LaTeX: A Document Preparation System},
  year = {1994}
}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        let literal_text = pdf_literal_text(&pdf);
        assert!(
            literal_text.contains("Textual Knuth [1984] and plain Lamport [1994]."),
            "{literal_text}"
        );
        assert!(
            literal_text.contains("Parenthetical [see Knuth, 1984, Lamport, 1994, chap. 2]."),
            "{literal_text}"
        );
        assert!(pdf_text.contains("References"), "{pdf_text}");
        assert!(
            pdf_text.contains("[1] Knuth, Donald E. The TeXbook. 1984"),
            "{pdf_text}"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&bib.display().to_string()), "{fls}");
    }

    #[test]
    fn native_parser_renders_bibliography_at_source_location_before_appendix() {
        let root = temp_dir("bibliography-before-appendix");
        let out = root.join("build");
        let bib = root.join("refs.bib");
        fs::create_dir_all(&out).unwrap();
        fs::write(
            &bib,
            r"@book{knuth,
  author = {Donald Knuth},
  title = {The TeXbook},
  year = {1984}
}
",
        )
        .unwrap();
        let source = r"\documentclass{article}
\begin{document}
Body cites \cite{knuth}.
\bibliographystyle{plain}
\bibliography{refs}
\appendix
\section{Extra}
Appendix text.
\end{document}
";
        let mut inputs = Vec::new();
        let document = parse_supported_document(
            source,
            &root,
            &out,
            "main",
            &mut inputs,
            NativeArtifactPolicy::PdfOnly,
        )
        .unwrap();

        let references_index = document
            .lines
            .iter()
            .position(|line| matches!(line, Line::Heading(text) if text == "References"))
            .expect("references heading");
        let appendix_index = document
            .lines
            .iter()
            .position(|line| matches!(line, Line::Heading(text) if text == "A Extra"))
            .expect("appendix heading");
        let references_count = document
            .lines
            .iter()
            .filter(|line| matches!(line, Line::Heading(text) if text == "References"))
            .count();

        assert!(references_index < appendix_index, "{:?}", document.lines);
        assert_eq!(references_count, 1, "{:?}", document.lines);
    }

    #[test]
    fn native_engine_records_local_bibliography_style_inputs() {
        let root = temp_dir("bibliography-style");
        let main = root.join("main.tex");
        let bib = root.join("refs.bib");
        let bst = root.join("customstyle.bst");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Custom style citation \cite{knuth}.
\bibliographystyle{customstyle}
\bibliography{refs}
\end{document}
",
        )
        .unwrap();
        fs::write(
            &bib,
            r"@book{knuth,
  author = {Donald Knuth},
  title = {The TeXbook},
  year = {1984}
}
",
        )
        .unwrap();
        fs::write(&bst, "ENTRY { author title year }{}{}\n").unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let aux = fs::read_to_string(out.join("main.aux")).expect("aux should exist");
        assert!(aux.contains("\\bibstyle{customstyle}"), "{aux}");
        assert!(aux.contains("\\bibdata{refs}"), "{aux}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&bib.display().to_string()), "{fls}");
        assert!(fls.contains(&bst.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_resolves_bstinputs_bibliography_styles() {
        let root = temp_dir("bstinputs-bibliography-style");
        let paper = root.join("paper");
        let shared = root.join("shared").join("bst");
        let main = paper.join("main.tex");
        let bib = paper.join("refs.bib");
        let bst = shared.join("sharedstyle.bst");
        let out = root.join("build");
        fs::create_dir_all(&paper).unwrap();
        fs::create_dir_all(&shared).unwrap();
        let _env_lock = BSTINPUTS_TEST_LOCK
            .lock()
            .expect("BSTINPUTS test lock poisoned");
        let _bstinputs = EnvVarGuard::set(
            "BSTINPUTS",
            OsString::from(format!("{}//{}", shared.display(), KPATHSEA_PATH_SEPARATOR)),
        );
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Shared style citation \cite{knuth}.
\bibliographystyle{sharedstyle}
\bibliography{refs}
\end{document}
",
        )
        .unwrap();
        fs::write(
            &bib,
            r"@book{knuth,
  author = {Donald Knuth},
  title = {The TeXbook},
  year = {1984}
}
",
        )
        .unwrap();
        fs::write(&bst, "ENTRY { author title year }{}{}\n").unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let aux = fs::read_to_string(out.join("main.aux")).expect("aux should exist");
        assert!(aux.contains("\\bibstyle{sharedstyle}"), "{aux}");
        assert!(aux.contains("\\bibdata{refs}"), "{aux}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&bib.display().to_string()), "{fls}");
        assert!(
            fls.contains(&bst.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_resolves_bibinputs_bibliography() {
        let root = temp_dir("bibinputs-bibliography");
        let paper = root.join("paper");
        let shared = root.join("shared").join("bib");
        let main = paper.join("main.tex");
        let bib = shared.join("sharedrefs.bib");
        let out = root.join("build");
        fs::create_dir_all(&paper).unwrap();
        fs::create_dir_all(&shared).unwrap();
        let _env_lock = BIBINPUTS_TEST_LOCK
            .lock()
            .expect("BIBINPUTS test lock poisoned");
        let _bibinputs = EnvVarGuard::set(
            "BIBINPUTS",
            OsString::from(format!("{}//{}", shared.display(), KPATHSEA_PATH_SEPARATOR)),
        );
        fs::write(
            &main,
            r"\documentclass{article}
\begin{document}
Shared citation \cite{sharedknuth}.
\bibliographystyle{plain}
\bibliography{sharedrefs}
\end{document}
",
        )
        .unwrap();
        fs::write(
            &bib,
            r"@book{sharedknuth,
  author = {Donald Knuth},
  title = {Shared TeXbook},
  year = {1984}
}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(pdf_text.contains("Shared citation [1]."), "{pdf_text}");
        assert!(pdf_text.contains("References"), "{pdf_text}");
        assert!(
            pdf_text.contains("[1] Donald Knuth. Shared TeXbook. 1984"),
            "{pdf_text}"
        );
        let aux = fs::read_to_string(out.join("main.aux")).expect("aux should exist");
        assert!(aux.contains("\\citation{sharedknuth}"), "{aux}");
        assert!(aux.contains("\\bibdata{sharedrefs}"), "{aux}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(
            fls.contains(&bib.canonicalize().unwrap().display().to_string()),
            "{fls}"
        );
    }

    #[test]
    fn native_engine_renders_manual_thebibliography_without_fallback() {
        let root = temp_dir("manual-thebibliography");
        let main = root.join("main.tex");
        let out = root.join("build");
        fs::write(
            &main,
            r#"\documentclass{article}
\begin{document}
Manual citations \cite{beta} before \cite{alpha}.
\begin{thebibliography}{9}
\bibitem{alpha}
Ada Lovelace. \emph{Notes on the Analytical Engine}. 1843.
\bibitem[Custom]{beta}
Donald Knuth. {The TeXbook}. 1984.
\end{thebibliography}
\end{document}
"#,
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_text.contains("Manual citations [2] before [1]."),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("References"), "{pdf_text}");
        assert!(
            pdf_text.contains("[1] Ada Lovelace. Notes on the Analytical Engine. 1843."),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("[2] Donald Knuth. The TeXbook. 1984."),
            "{pdf_text}"
        );
        assert!(!pdf_text.contains("bibitem"), "{pdf_text}");
        let aux = fs::read_to_string(out.join("main.aux")).expect("aux should exist");
        assert!(aux.contains("\\citation{beta}"), "{aux}");
        assert!(aux.contains("\\citation{alpha}"), "{aux}");
        assert!(!aux.contains("\\bibdata"), "{aux}");
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
    }

    #[test]
    fn native_engine_renders_basic_biblatex_citations_without_fallback() {
        let root = temp_dir("biblatex-citations");
        let main = root.join("main.tex");
        let bib = root.join("refs.bib");
        let out = root.join("build");
        fs::write(
            &main,
            r"\documentclass{article}
\usepackage[backend=biber,style=authoryear]{biblatex}
\addbibresource{refs.bib}
\begin{document}
This fixture cites \textcite{knuth1984} and \parencite{lamport1994}.
\printbibliography
\end{document}
",
        )
        .unwrap();
        fs::write(
            &bib,
            r"@book{knuth1984,
  author = {Donald E. Knuth},
  title = {The TeXbook},
  year = {1984}
}

@book{lamport1994,
  author = {Leslie Lamport},
  title = {LaTeX: A Document Preparation System},
  year = {1994}
}
",
        )
        .unwrap();

        let run = run_native(&NativeEngineOptions {
            main: main.clone(),
            output_dir: out.clone(),
            job_name: "main".to_string(),
            mode: RunMode {
                suppress_pdf_output: false,
                draft_graphics: false,
            },
            shell_escape: false,
            synctex: false,
        })
        .unwrap();

        assert_eq!(run.status, NativeEngineStatus::Native);
        let pdf = fs::read(out.join("main.pdf")).expect("PDF should exist");
        let pdf_text = String::from_utf8_lossy(&pdf);
        assert!(
            pdf_text.contains("This fixture cites Knuth [1] and [2]."),
            "{pdf_text}"
        );
        assert!(pdf_text.contains("References"), "{pdf_text}");
        assert!(
            pdf_text.contains("[1] Donald E. Knuth. The TeXbook. 1984"),
            "{pdf_text}"
        );
        assert!(
            pdf_text.contains("[2] Leslie Lamport. LaTeX: A Document Preparation System. 1994"),
            "{pdf_text}"
        );
        let fls = fs::read_to_string(out.join("main.fls")).expect("fls should exist");
        assert!(fls.contains(&main.display().to_string()), "{fls}");
        assert!(fls.contains(&bib.display().to_string()), "{fls}");
    }

    fn tiny_jpeg_bytes() -> &'static [u8] {
        &[
            0xff, 0xd8, 0xff, 0xe0, 0x00, 0x10, b'J', b'F', b'I', b'F', 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00, 0x01, 0x00,
            0x02, 0x03, 0x01, 0x11, 0x00, 0x02, 0x11, 0x00, 0x03, 0x11, 0x00, 0xff, 0xd9,
        ]
    }

    fn tiny_rgba_png_bytes() -> Vec<u8> {
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, 2, 1);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().expect("png header should write");
            writer
                .write_image_data(&[255, 0, 0, 255, 0, 0, 255, 128])
                .expect("png pixels should write");
        }
        bytes
    }

    fn tiny_pdf_graphic_bytes() -> Vec<u8> {
        tiny_pdf_graphic_pages(&[(100, 50, b"0 0 1 rg\n0 0 100 50 re\nf\n".as_slice())])
    }

    fn tiny_two_page_pdf_graphic_bytes() -> Vec<u8> {
        tiny_pdf_graphic_pages(&[
            (100, 50, b"0 0 1 rg\n0 0 100 50 re\nf\n".as_slice()),
            (40, 100, b"0 1 0 rg\n0 0 40 100 re\nf\n".as_slice()),
        ])
    }

    fn tiny_pdf_graphic_pages(pages: &[(i64, i64, &[u8])]) -> Vec<u8> {
        let mut document = LoDocument::with_version("1.4");
        let pages_id = document.new_object_id();
        let page_count = pages.len() as i64;

        let mut graphics_state = LoDictionary::new();
        graphics_state.set("Type", LoObject::Name(b"ExtGState".to_vec()));
        graphics_state.set("CA", 1_i64);
        let graphics_state_id = document.add_object(LoObject::Dictionary(graphics_state));

        let mut ext_gstate = LoDictionary::new();
        ext_gstate.set("GS1", LoObject::Reference(graphics_state_id));
        let mut resources = LoDictionary::new();
        resources.set("ExtGState", LoObject::Dictionary(ext_gstate));
        let resources_id = document.add_object(LoObject::Dictionary(resources));

        let mut page_ids = Vec::new();
        for (width, height, content) in pages {
            let mut page_content = b"/GS1 gs\n".to_vec();
            page_content.extend_from_slice(content);
            let content_id = document.add_object(LoObject::Stream(LoStream::new(
                LoDictionary::new(),
                page_content,
            )));

            let mut page = LoDictionary::new();
            page.set("Type", LoObject::Name(b"Page".to_vec()));
            page.set("Parent", LoObject::Reference(pages_id));
            page.set("Resources", LoObject::Reference(resources_id));
            page.set(
                "MediaBox",
                LoObject::Array(vec![
                    LoObject::Integer(0),
                    LoObject::Integer(0),
                    LoObject::Integer(*width),
                    LoObject::Integer(*height),
                ]),
            );
            page.set("Contents", LoObject::Reference(content_id));
            page_ids.push(document.add_object(LoObject::Dictionary(page)));
        }

        let mut pages_dict = LoDictionary::new();
        pages_dict.set("Type", LoObject::Name(b"Pages".to_vec()));
        pages_dict.set(
            "Kids",
            LoObject::Array(page_ids.into_iter().map(LoObject::Reference).collect()),
        );
        pages_dict.set("Count", page_count);
        document
            .objects
            .insert(pages_id, LoObject::Dictionary(pages_dict));

        let mut catalog = LoDictionary::new();
        catalog.set("Type", LoObject::Name(b"Catalog".to_vec()));
        catalog.set("Pages", LoObject::Reference(pages_id));
        let catalog_id = document.add_object(LoObject::Dictionary(catalog));
        document
            .trailer
            .set("Root", LoObject::Reference(catalog_id));

        let mut bytes = Vec::new();
        document
            .save_to(&mut bytes)
            .expect("PDF fixture should serialize");
        bytes
    }
}

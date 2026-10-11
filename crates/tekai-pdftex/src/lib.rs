//! Experimental pdfTeX rewrite track for `tekai`.
//!
//! This crate contains the active experimental expansion core and approximate
//! native PDF renderer. It does not establish general rendered-PDF equivalence.

pub mod expand;
pub mod native;
pub mod token;
pub mod trace;

pub use expand::{
    ExpandError, ExpansionEngine, MacroDefinition, expand_to_source,
    expand_to_source_with_file_context, expand_to_text, expand_to_text_with_file_context,
    expand_to_tokens, expand_to_tokens_with_file_context, tokens_to_source, tokens_to_text,
};
pub use native::{
    NativeArtifactPolicy, NativeEngineOptions, NativeEngineRun, NativeEngineStatus,
    NativeUnsupported, RunMode, probe_native_support, run_native, run_native_pdf_only,
    run_native_with_artifact_policy,
};
pub use token::{CatCode, CatCodeTable, Token, tokenize};
pub use trace::{TraceEvent, TraceWriter};

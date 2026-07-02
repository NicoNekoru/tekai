//! Experimental pdfTeX rewrite track for `texpilot`.
//!
//! This crate carries the rendered-PDF equivalence contract, expansion core,
//! and native PDF renderer for a future engine that can replace external
//! `pdflatex` in the hot path without shrinking `texpilot`'s final-build
//! objective.

pub mod contract;
pub mod expand;
pub mod native;
pub mod roadmap;
pub mod token;
pub mod trace;

pub use contract::{
    CURRENT_ENGINE_BOUNDARY, Capability, EngineBoundary, IntegrationPoint, NonSolution,
    PdfTexResponsibility, PerformanceStrategy, RewritePlan, TexpilotResponsibility,
};
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
pub use roadmap::{Milestone, ROADMAP, RoadmapPhase};
pub use token::{CatCode, CatCodeTable, Token, tokenize};
pub use trace::{TraceEvent, TraceWriter};

/// Returns the default rewrite plan for the pdfTeX replacement track.
pub const fn rewrite_plan() -> RewritePlan {
    RewritePlan::new()
}

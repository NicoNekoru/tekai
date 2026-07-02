/// Responsibilities that stay in the current `texpilot` orchestrator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TexpilotResponsibility {
    SourceDiscovery,
    BuildStateCache,
    HotPathScheduling,
    OptionalInteropToolScheduling,
    ExternalAssetPreparation,
    FastPreviewPatching,
    WatchInvalidation,
    CliAndJsonReporting,
}

/// Work currently delegated to external `pdflatex`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PdfTexResponsibility {
    CatcodeAwareTokenization,
    MacroExpansion,
    PrimitiveExecution,
    RegistersGroupingAndAssignments,
    InputStackAndFileIo,
    KpathseaCompatibleResolution,
    InMemoryDocumentState,
    ParagraphLineBreaking,
    PageBreakingAndOutputRoutine,
    MathTypesetting,
    FontMetricsEncodingsAndMaps,
    ImageInclusion,
    PdfObjectGeneration,
    OptionalInteropArtifacts,
    ShellEscapeSemantics,
    Diagnostics,
}

/// The current boundary: orchestration is ours; TeX execution and PDF
/// generation are still mostly pdfTeX's. A high-performance rewrite should
/// move the functional right-hand list behind a Rust API without treating
/// legacy sidecar formats as the architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EngineBoundary {
    pub texpilot_owned: &'static [TexpilotResponsibility],
    pub pdftex_owned: &'static [PdfTexResponsibility],
}

pub const CURRENT_ENGINE_BOUNDARY: EngineBoundary = EngineBoundary {
    texpilot_owned: &[
        TexpilotResponsibility::SourceDiscovery,
        TexpilotResponsibility::BuildStateCache,
        TexpilotResponsibility::HotPathScheduling,
        TexpilotResponsibility::OptionalInteropToolScheduling,
        TexpilotResponsibility::ExternalAssetPreparation,
        TexpilotResponsibility::FastPreviewPatching,
        TexpilotResponsibility::WatchInvalidation,
        TexpilotResponsibility::CliAndJsonReporting,
    ],
    pdftex_owned: &[
        PdfTexResponsibility::CatcodeAwareTokenization,
        PdfTexResponsibility::MacroExpansion,
        PdfTexResponsibility::PrimitiveExecution,
        PdfTexResponsibility::RegistersGroupingAndAssignments,
        PdfTexResponsibility::InputStackAndFileIo,
        PdfTexResponsibility::KpathseaCompatibleResolution,
        PdfTexResponsibility::InMemoryDocumentState,
        PdfTexResponsibility::ParagraphLineBreaking,
        PdfTexResponsibility::PageBreakingAndOutputRoutine,
        PdfTexResponsibility::MathTypesetting,
        PdfTexResponsibility::FontMetricsEncodingsAndMaps,
        PdfTexResponsibility::ImageInclusion,
        PdfTexResponsibility::PdfObjectGeneration,
        PdfTexResponsibility::OptionalInteropArtifacts,
        PdfTexResponsibility::ShellEscapeSemantics,
        PdfTexResponsibility::Diagnostics,
    ],
};

/// Required capability slices for a functional TeX/STY-to-PDF replacement.
/// The output PDF is the compatibility contract; legacy files such as `.aux`,
/// `.fls`, logs, and BibTeX/Biber control files are optional adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    RenderedPdfEquivalence,
    CatcodeTokenizer,
    ExpandableMacroEngine,
    PdfTexPrimitiveSet,
    ETexPrimitiveSet,
    LatexKernelBoot,
    FormatSnapshot,
    KpathseaResolver,
    InMemoryDocumentModel,
    InProcessBibliographyModel,
    LineBreaker,
    PageBuilder,
    OutputRoutine,
    MathLayout,
    FontSubsystem,
    GraphicsSubsystem,
    PdfBackend,
    ShellEscapePolicy,
    Diagnostics,
    OptionalInteropArtifacts,
}

/// Performance strategy needed for clean full builds. These are not mere
/// preview/cache wins; they change the amount of engine work done per build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PerformanceStrategy {
    PreindexedDistributionFiles,
    MemoryMappedFormatSnapshots,
    SingleProcessConvergence,
    InMemoryDocumentFixedPoint,
    InProcessBibliographyForCommonStyles,
    IncrementalLayoutGraph,
    PageObjectReuse,
    ParallelAssetPipeline,
    ParallelPdfObjectWriter,
    ProfileGuidedPrimitiveFastPaths,
}

/// Integration points a replacement engine must satisfy before it can become a
/// `texpilot` runner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntegrationPoint {
    BuildOptions,
    TexPassReport,
    BuildStateFingerprints,
    OutputDirectoryContract,
    JobNameContract,
    EnvironmentContract,
    PdfEquivalenceGate,
    OptionalArtifactExport,
}

/// Tempting approaches that do not solve the user's stated target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NonSolution {
    FasterSourceParserOnly,
    PreambleCacheOnly,
    WrapperSchedulingOnly,
    PreviewOnlyRenderer,
    PageReuseWithoutCleanBuildSpeedup,
    IncompatibleLatexSubsetByDefault,
    LegacySidecarFidelityAsGoal,
    ExactPdflatexLogCompatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RewritePlan {
    pub boundary: EngineBoundary,
    pub capabilities: &'static [Capability],
    pub clean_build_strategies: &'static [PerformanceStrategy],
    pub integration_points: &'static [IntegrationPoint],
    pub non_solutions: &'static [NonSolution],
}

impl RewritePlan {
    pub const fn new() -> Self {
        Self {
            boundary: CURRENT_ENGINE_BOUNDARY,
            capabilities: REQUIRED_CAPABILITIES,
            clean_build_strategies: CLEAN_BUILD_STRATEGIES,
            integration_points: INTEGRATION_POINTS,
            non_solutions: NON_SOLUTIONS,
        }
    }

    pub fn moves_pdftex_work_in_process(&self) -> bool {
        self.boundary
            .pdftex_owned
            .contains(&PdfTexResponsibility::MacroExpansion)
            && self
                .boundary
                .pdftex_owned
                .contains(&PdfTexResponsibility::PageBreakingAndOutputRoutine)
            && self
                .boundary
                .pdftex_owned
                .contains(&PdfTexResponsibility::PdfObjectGeneration)
    }

    pub fn has_clean_subsecond_strategy(&self) -> bool {
        self.clean_build_strategies
            .contains(&PerformanceStrategy::SingleProcessConvergence)
            && self
                .clean_build_strategies
                .contains(&PerformanceStrategy::InMemoryDocumentFixedPoint)
            && self
                .clean_build_strategies
                .contains(&PerformanceStrategy::MemoryMappedFormatSnapshots)
            && self
                .clean_build_strategies
                .contains(&PerformanceStrategy::ParallelPdfObjectWriter)
    }

    pub fn uses_pdf_as_compatibility_contract(&self) -> bool {
        self.capabilities
            .contains(&Capability::RenderedPdfEquivalence)
            && self
                .integration_points
                .contains(&IntegrationPoint::PdfEquivalenceGate)
    }

    pub fn keeps_legacy_artifacts_optional(&self) -> bool {
        self.capabilities
            .contains(&Capability::OptionalInteropArtifacts)
            && self
                .integration_points
                .contains(&IntegrationPoint::OptionalArtifactExport)
            && self
                .non_solutions
                .contains(&NonSolution::LegacySidecarFidelityAsGoal)
    }
}

impl Default for RewritePlan {
    fn default() -> Self {
        Self::new()
    }
}

const REQUIRED_CAPABILITIES: &[Capability] = &[
    Capability::RenderedPdfEquivalence,
    Capability::CatcodeTokenizer,
    Capability::ExpandableMacroEngine,
    Capability::PdfTexPrimitiveSet,
    Capability::ETexPrimitiveSet,
    Capability::LatexKernelBoot,
    Capability::FormatSnapshot,
    Capability::KpathseaResolver,
    Capability::InMemoryDocumentModel,
    Capability::InProcessBibliographyModel,
    Capability::LineBreaker,
    Capability::PageBuilder,
    Capability::OutputRoutine,
    Capability::MathLayout,
    Capability::FontSubsystem,
    Capability::GraphicsSubsystem,
    Capability::PdfBackend,
    Capability::ShellEscapePolicy,
    Capability::Diagnostics,
    Capability::OptionalInteropArtifacts,
];

const CLEAN_BUILD_STRATEGIES: &[PerformanceStrategy] = &[
    PerformanceStrategy::PreindexedDistributionFiles,
    PerformanceStrategy::MemoryMappedFormatSnapshots,
    PerformanceStrategy::SingleProcessConvergence,
    PerformanceStrategy::InMemoryDocumentFixedPoint,
    PerformanceStrategy::InProcessBibliographyForCommonStyles,
    PerformanceStrategy::IncrementalLayoutGraph,
    PerformanceStrategy::PageObjectReuse,
    PerformanceStrategy::ParallelAssetPipeline,
    PerformanceStrategy::ParallelPdfObjectWriter,
    PerformanceStrategy::ProfileGuidedPrimitiveFastPaths,
];

const INTEGRATION_POINTS: &[IntegrationPoint] = &[
    IntegrationPoint::BuildOptions,
    IntegrationPoint::TexPassReport,
    IntegrationPoint::BuildStateFingerprints,
    IntegrationPoint::OutputDirectoryContract,
    IntegrationPoint::JobNameContract,
    IntegrationPoint::EnvironmentContract,
    IntegrationPoint::PdfEquivalenceGate,
    IntegrationPoint::OptionalArtifactExport,
];

const NON_SOLUTIONS: &[NonSolution] = &[
    NonSolution::FasterSourceParserOnly,
    NonSolution::PreambleCacheOnly,
    NonSolution::WrapperSchedulingOnly,
    NonSolution::PreviewOnlyRenderer,
    NonSolution::PageReuseWithoutCleanBuildSpeedup,
    NonSolution::IncompatibleLatexSubsetByDefault,
    NonSolution::LegacySidecarFidelityAsGoal,
    NonSolution::ExactPdflatexLogCompatibility,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_moves_pdftex_critical_path_into_the_rewrite() {
        let plan = RewritePlan::new();

        assert!(plan.moves_pdftex_work_in_process());
    }

    #[test]
    fn clean_subsecond_plan_is_not_only_a_cache_or_scheduler_plan() {
        let plan = RewritePlan::new();

        assert!(plan.has_clean_subsecond_strategy());
        assert!(
            plan.non_solutions
                .contains(&NonSolution::WrapperSchedulingOnly)
        );
        assert!(plan.non_solutions.contains(&NonSolution::PreambleCacheOnly));
    }

    #[test]
    fn integration_uses_pdf_equivalence_as_the_contract() {
        let plan = RewritePlan::new();

        assert!(plan.uses_pdf_as_compatibility_contract());
        assert!(plan.keeps_legacy_artifacts_optional());
    }
}

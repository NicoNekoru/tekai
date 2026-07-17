use crate::contract::{Capability, IntegrationPoint, PerformanceStrategy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoadmapPhase {
    pub id: &'static str,
    pub name: &'static str,
    pub objective: &'static str,
    pub exit_criteria: &'static [&'static str],
    pub unlocks: &'static [Milestone],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Milestone {
    RenderedPdfBaseline,
    LatexKernelBoot,
    InMemoryDocumentConvergence,
    NearIdenticalPdfOutput,
    FullBuildFasterThanPdfTexOnePass,
    SubsecondTargetCandidate,
    DefaultRunnerCandidate,
}

pub const ROADMAP: &[RoadmapPhase] = &[
    RoadmapPhase {
        id: "p0",
        name: "PDF equivalence harness",
        objective: "Measure rendered PDF similarity, input coverage, and timing for current native and external pdfTeX builds.",
        exit_criteria: &[
            "Can compare rendered pages from native and external pdfTeX outputs for the large examples.",
            "Records TeX/STY primitive and package coverage required by the example corpus.",
            "Treats logs, recorder files, and aux-style sidecars as optional diagnostics, not correctness targets.",
        ],
        unlocks: &[Milestone::RenderedPdfBaseline],
    },
    RoadmapPhase {
        id: "p1",
        name: "Expansion core",
        objective: "Implement catcode-aware tokenization, macro expansion, grouping, registers, conditionals, and the pdfTeX/e-TeX primitive surface needed by LaTeX.",
        exit_criteria: &[
            "Boots enough of the LaTeX kernel and package layer to execute real .sty files used by the target corpus.",
            "Matches external pdfTeX token and assignment behavior for focused fixtures.",
            "Feeds a typed document/layout model directly instead of routing correctness through aux/log sidecars.",
        ],
        unlocks: &[Milestone::LatexKernelBoot],
    },
    RoadmapPhase {
        id: "p2",
        name: "Snapshot and file system layer",
        objective: "Make clean builds fast before document execution starts: preindex TeX trees, memory-map format snapshots, and resolve TeX/STY/font/image inputs directly.",
        exit_criteria: &[
            "Loads a LaTeX-ready snapshot without reparsing the distribution.",
            "Resolves source, package, bibliography-data, font, and image inputs needed to produce the PDF.",
            "Invalidates snapshots by engine version, environment, and distribution file fingerprints.",
        ],
        unlocks: &[Milestone::FullBuildFasterThanPdfTexOnePass],
    },
    RoadmapPhase {
        id: "p3",
        name: "In-memory document convergence",
        objective: "Collapse reruns into one engine process by keeping labels, citations, contents, indexes, bibliography data, and package state in typed memory.",
        exit_criteria: &[
            "Avoids process restarts for ordinary LaTeX fixed points.",
            "Handles the bibliography and index forms used by the large examples without BibTeX/Biber/MakeIndex on the hot path.",
            "Can export legacy sidecars only when explicitly requested for debugging or interoperability.",
        ],
        unlocks: &[Milestone::InMemoryDocumentConvergence],
    },
    RoadmapPhase {
        id: "p4",
        name: "Layout and PDF backend",
        objective: "Implement paragraph breaking, page building, output routines, math layout, fonts, images, and pdfTeX PDF primitives behind a parallel PDF writer.",
        exit_criteria: &[
            "Rendered native PDFs are near-identical to external pdfTeX for the examples and existing parity fixtures.",
            "PDF object generation is deterministic enough for tekai cache fingerprints.",
            "Asset decode, font lookup, and PDF object compression run off the expansion thread where legal.",
        ],
        unlocks: &[Milestone::NearIdenticalPdfOutput],
    },
    RoadmapPhase {
        id: "p5",
        name: "Sub-second candidate",
        objective: "Use the native engine pipeline to beat the external pdfTeX one-pass lower bound on clean final builds, then push below one second on the target examples.",
        exit_criteria: &[
            "Clean final builds are faster than one external pdfTeX PDF-producing pass.",
            "Both bundled large examples build final PDFs below one second on the benchmark host.",
            "Default native output is the PDF plus a compact machine report; legacy artifacts are opt-in.",
        ],
        unlocks: &[
            Milestone::SubsecondTargetCandidate,
            Milestone::DefaultRunnerCandidate,
        ],
    },
];

pub const fn capabilities_for_subsecond_clean_builds() -> &'static [Capability] {
    &[
        Capability::FormatSnapshot,
        Capability::ExpandableMacroEngine,
        Capability::RenderedPdfEquivalence,
        Capability::InMemoryDocumentModel,
        Capability::InProcessBibliographyModel,
        Capability::LineBreaker,
        Capability::PageBuilder,
        Capability::OutputRoutine,
        Capability::PdfBackend,
        Capability::Diagnostics,
    ]
}

pub const fn strategies_for_subsecond_clean_builds() -> &'static [PerformanceStrategy] {
    &[
        PerformanceStrategy::MemoryMappedFormatSnapshots,
        PerformanceStrategy::SingleProcessConvergence,
        PerformanceStrategy::InMemoryDocumentFixedPoint,
        PerformanceStrategy::InProcessBibliographyForCommonStyles,
        PerformanceStrategy::ParallelAssetPipeline,
        PerformanceStrategy::ParallelPdfObjectWriter,
        PerformanceStrategy::ProfileGuidedPrimitiveFastPaths,
    ]
}

pub const fn runner_integration_points() -> &'static [IntegrationPoint] {
    &[
        IntegrationPoint::BuildOptions,
        IntegrationPoint::TexPassReport,
        IntegrationPoint::BuildStateFingerprints,
        IntegrationPoint::PdfEquivalenceGate,
        IntegrationPoint::OptionalArtifactExport,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roadmap_ends_in_a_clean_final_build_target() {
        let last = ROADMAP.last().expect("roadmap should have phases");

        assert_eq!(last.id, "p5");
        assert!(last.unlocks.contains(&Milestone::SubsecondTargetCandidate));
        assert!(
            last.exit_criteria
                .iter()
                .any(|criterion| criterion.to_lowercase().contains("clean final builds"))
        );
    }

    #[test]
    fn subsecond_strategy_contains_more_than_incremental_page_reuse() {
        let strategies = strategies_for_subsecond_clean_builds();

        assert!(strategies.contains(&PerformanceStrategy::SingleProcessConvergence));
        assert!(strategies.contains(&PerformanceStrategy::InMemoryDocumentFixedPoint));
        assert!(strategies.contains(&PerformanceStrategy::ParallelPdfObjectWriter));
        assert!(!strategies.contains(&PerformanceStrategy::PageObjectReuse));
    }

    #[test]
    fn roadmap_treats_sidecars_as_optional_artifacts() {
        let runner_points = runner_integration_points();

        assert!(runner_points.contains(&IntegrationPoint::PdfEquivalenceGate));
        assert!(runner_points.contains(&IntegrationPoint::OptionalArtifactExport));
        assert!(
            ROADMAP
                .iter()
                .flat_map(|phase| phase.exit_criteria)
                .any(|criterion| criterion.contains("legacy artifacts are opt-in"))
        );
    }
}

//! One compiler integration-test binary shares the embedded engine and assets.

use std::sync::{Mutex, MutexGuard};

// Several suites temporarily change process-wide search paths or caches.
// Keep every compiler test isolated now that they share a process.
fn lock() -> MutexGuard<'static, ()> {
    static TEST_LOCK: Mutex<()> = Mutex::new(());
    TEST_LOCK.lock().unwrap_or_else(|error| error.into_inner())
}

#[path = "compiler/compiler_asymptote.rs"]
mod compiler_asymptote;
#[path = "compiler/compiler_aux_tools.rs"]
mod compiler_aux_tools;
#[path = "compiler/compiler_bib2gls.rs"]
mod compiler_bib2gls;
#[path = "compiler/compiler_biber.rs"]
mod compiler_biber;
#[path = "compiler/compiler_bibliography.rs"]
mod compiler_bibliography;
#[path = "compiler/compiler_cache.rs"]
mod compiler_cache;
#[path = "compiler/compiler_convergence.rs"]
mod compiler_convergence;
#[path = "compiler/compiler_dependencies.rs"]
mod compiler_dependencies;
#[path = "compiler/compiler_eps.rs"]
mod compiler_eps;
#[path = "compiler/compiler_fast.rs"]
mod compiler_fast;
#[path = "compiler/compiler_glossary.rs"]
mod compiler_glossary;
#[path = "compiler/compiler_gnuplottex.rs"]
mod compiler_gnuplottex;
#[path = "compiler/compiler_include_dirs.rs"]
mod compiler_include_dirs;
#[path = "compiler/compiler_index.rs"]
mod compiler_index;
#[path = "compiler/compiler_jobname.rs"]
mod compiler_jobname;
#[path = "compiler/compiler_max_runs.rs"]
mod compiler_max_runs;
#[path = "compiler/compiler_metapost.rs"]
mod compiler_metapost;
#[path = "compiler/compiler_minted.rs"]
mod compiler_minted;
#[path = "compiler/compiler_nomenclature.rs"]
mod compiler_nomenclature;
#[path = "compiler/compiler_pgf_external.rs"]
mod compiler_pgf_external;
#[path = "compiler/compiler_pythontex.rs"]
mod compiler_pythontex;
#[path = "compiler/compiler_svg.rs"]
mod compiler_svg;
#[path = "compiler/compiler_tekai_pdftex.rs"]
mod compiler_tekai_pdftex;

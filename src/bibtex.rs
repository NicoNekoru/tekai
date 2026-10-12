//! In-process BibTeX with project files and Tekai's bundled styles.

use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Component, Path};

use anyhow::{Context, Result, bail};
use tectonic_bridge_core::{CoreBridgeLauncher, MinimalDriver};
use tectonic_engine_bibtex::{BibtexEngine, BibtexOutcome};
use tectonic_io_base::{InputHandle, InputOrigin, IoProvider, OpenResult, OutputHandle};
use tectonic_status_base::{NoopStatusBackend, StatusBackend};

pub fn run(
    doc_dir: &Path,
    out_dir: &Path,
    aux: &str,
    options: &[String],
    quiet: bool,
) -> Result<()> {
    // Worker threads do not inherit the parent's scoped lookup context. Own a
    // complete one here so bundled styles/indexes are not rebuilt per input.
    tekai_engine::lookup::with_lookup_session(|| {
        let mut engine = BibtexEngine::default();
        let mut args = options.iter();
        while let Some(arg) = args.next() {
            let value = arg
                .strip_prefix("-min-crossrefs=")
                .or_else(|| arg.strip_prefix("--min-crossrefs="))
                .or_else(|| {
                    matches!(arg.as_str(), "-min-crossrefs" | "--min-crossrefs")
                        .then(|| args.next().map(String::as_str))
                        .flatten()
                });
            if let Some(value) = value {
                engine.min_crossrefs(value.parse().context("invalid BibTeX min-crossrefs")?);
            } else if !matches!(arg.as_str(), "-terse" | "--terse") {
                bail!("unsupported option for built-in BibTeX: {arg}");
            }
        }
        let mut driver = MinimalDriver::new(BibIo {
            doc_dir,
            out_dir,
            quiet,
        });
        let mut status = NoopStatusBackend {};
        let mut launcher = CoreBridgeLauncher::new(&mut driver, &mut status);
        let aux = if aux.ends_with(".aux") {
            aux.to_string()
        } else {
            format!("{aux}.aux")
        };
        let outcome = engine
            .process(&mut launcher, &aux)
            .context("built-in BibTeX failed")?;
        if outcome == BibtexOutcome::Errors {
            bail!(
                "built-in BibTeX reported errors; see {}",
                out_dir
                    .join(Path::new(&aux).with_extension("blg"))
                    .display()
            );
        }
        Ok(())
    })
}

struct BibIo<'a> {
    doc_dir: &'a Path,
    out_dir: &'a Path,
    quiet: bool,
}

impl IoProvider for BibIo<'_> {
    fn input_open_name(
        &mut self,
        name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        let output_path = self.out_dir.join(name);
        let path = if output_path.is_file() {
            Some(output_path)
        } else {
            let extension = Path::new(name)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
            match tekai_engine::kpathsea::resolve_input(self.doc_dir, name, extension) {
                Ok(path) => path,
                Err(error) => return OpenResult::Err(error.into()),
            }
        };
        let Some(path) = path else {
            return OpenResult::NotAvailable;
        };
        match File::open(path) {
            Ok(file) => OpenResult::Ok(InputHandle::new(
                name,
                BufReader::new(file),
                InputOrigin::Filesystem,
            )),
            Err(error) => OpenResult::Err(error.into()),
        }
    }

    fn output_open_name(&mut self, name: &str) -> OpenResult<OutputHandle> {
        if Path::new(name)
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
        {
            return OpenResult::Err(anyhow::anyhow!("invalid BibTeX output path: {name}"));
        }
        match File::create(self.out_dir.join(name)) {
            Ok(file) => OpenResult::Ok(OutputHandle::new(name, file)),
            Err(error) => OpenResult::Err(error.into()),
        }
    }

    fn output_open_stdout(&mut self) -> OpenResult<OutputHandle> {
        OpenResult::Ok(if self.quiet {
            OutputHandle::new("", io::sink())
        } else {
            OutputHandle::new("", io::stderr())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct Project(PathBuf);

    impl Project {
        fn new(aux: &str) -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let root = std::env::temp_dir().join(format!(
                "tekai-rust-bibtex-{}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&root).unwrap();
            std::fs::write(root.join("main.aux"), aux).unwrap();
            std::fs::write(
                root.join("refs.bib"),
                "@article{key, title={Rust bibliography}}\n",
            )
            .unwrap();
            std::fs::write(
                root.join("style.bst"),
                "ENTRY { title } {} {}\nFUNCTION {article} { title write$ newline$ }\nREAD\nITERATE {call.type$}\n",
            )
            .unwrap();
            Self(root)
        }

        fn outcome(&self, aux: &str) -> Result<BibtexOutcome> {
            let mut driver = MinimalDriver::new(BibIo {
                doc_dir: &self.0,
                out_dir: &self.0,
                quiet: true,
            });
            let mut status = NoopStatusBackend {};
            let mut launcher = CoreBridgeLauncher::new(&mut driver, &mut status);
            BibtexEngine::default().process(&mut launcher, aux)
        }
    }

    impl Drop for Project {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn rust_bibtex_success_preserves_exact_bbl_and_wrapper_acceptance() {
        let project = Project::new("\\citation{key}\n\\bibstyle{style}\n\\bibdata{refs}\n");
        assert_eq!(
            project.outcome("main.aux").unwrap(),
            BibtexOutcome::Spotless
        );
        assert_eq!(
            std::fs::read_to_string(project.0.join("main.bbl")).unwrap(),
            "Rust bibliography\n"
        );
        run(&project.0, &project.0, "main", &[], true).unwrap();
    }

    #[test]
    fn rust_bibtex_warnings_are_reported_and_accepted_by_wrapper() {
        let project = Project::new("\\citation{missing}\n\\bibstyle{style}\n\\bibdata{refs}\n");
        assert_eq!(
            project.outcome("main.aux").unwrap(),
            BibtexOutcome::Warnings
        );
        run(&project.0, &project.0, "main", &[], true).unwrap();
        assert!(
            std::fs::read_to_string(project.0.join("main.blg"))
                .unwrap()
                .contains("didn't find a database entry for \"missing\"")
        );
    }

    #[test]
    fn rust_bibtex_errors_remain_errors_and_wrapper_points_to_log() {
        let project = Project::new("\\citation{key}\n\\bibstyle{style}\n\\bibdata{missing}\n");
        assert_eq!(project.outcome("main.aux").unwrap(), BibtexOutcome::Errors);
        let error = run(&project.0, &project.0, "main", &[], true).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("built-in BibTeX reported errors")
        );
        assert!(error.to_string().contains("main.blg"));
    }

    #[test]
    fn rust_bibtex_fatal_failure_keeps_rust_error_and_next_run_succeeds() {
        let project = Project::new("\\citation{key}\n\\bibstyle{style}\n\\bibdata{refs}\n");
        let error = project.outcome("nonexistent.aux").unwrap_err();
        assert_eq!(error.to_string(), "unspecified fatal bibtex error");
        let wrapper_error = run(&project.0, &project.0, "nonexistent", &[], true).unwrap_err();
        assert_eq!(
            format!("{wrapper_error:#}"),
            "built-in BibTeX failed: unspecified fatal bibtex error"
        );
        assert_eq!(
            project.outcome("main.aux").unwrap(),
            BibtexOutcome::Spotless
        );
    }
}

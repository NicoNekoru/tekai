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

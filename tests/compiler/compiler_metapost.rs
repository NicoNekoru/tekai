use std::fs;
use std::path::{Path, PathBuf};

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const FEYNMP_DOC: &str = r#"\documentclass{article}
\usepackage{feynmp}
\DeclareGraphicsRule{*}{mps}{*}{}
\unitlength=1mm
\begin{document}
Before.
\begin{fmffile}{diagram}
\begin{fmfgraph}(30,20)
  \fmfleft{i}
  \fmfright{o}
  \fmf{plain}{i,o}
\end{fmfgraph}
\end{fmffile}
After.
\end{document}
"#;

#[cfg(unix)]
#[test]
fn bundled_runner_reuses_metapost_output_without_caching_generated_script_inputs() {
    let _test_guard = super::lock();
    let _environment = super::compiler_cache::bundled_environment();
    let fixture = MetapostFixture::new();
    let main = fixture.root.join("main.tex");
    let out_dir = fixture.root.join("out");
    let source = r"\documentclass{article}
\newwrite\metapostfile
\begin{document}
\immediate\openout\metapostfile=diagram.mp
\immediate\write\metapostfile{beginfig(1); alpha; endfig; end.}
\immediate\closeout\metapostfile
\IfFileExists{diagram.1}{\input{diagram.1}}{Pending figure.}
\end{document}
";
    fs::write(&main, source).expect("failed to write bundled MetaPost fixture");
    let options = options(&main, &out_dir);

    let first = build(&options).expect("generated MetaPost script must not fail publication");
    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert!(!first.skipped, "{first:#?}");
    assert!(out_dir.join("main.pdf").is_file());
    assert!(!out_dir.join(".tekai-main.state.json").exists());
    let script = fs::read(out_dir.join("diagram.mp")).unwrap();
    assert_eq!(fs::read(out_dir.join("diagram.1")).unwrap(), script);
    let recorder = fs::read_to_string(out_dir.join("main.fls")).unwrap();
    assert!(
        recorder
            .lines()
            .any(|line| line.starts_with("OUTPUT ") && line.ends_with("diagram.mp"))
    );
    // This script reaches publication only through the auxiliary tool state,
    // after the compiler has collected the TeX recorder's own inputs.
    assert!(
        !recorder
            .lines()
            .any(|line| line.starts_with("INPUT ") && line.ends_with("diagram.mp"))
    );
    assert_eq!(fixture.invocations(&out_dir), 1);

    let rebuilt = build(&options).expect("unchanged MetaPost fixture rebuild failed");
    assert_eq!(rebuilt.tex_runs, 1, "{rebuilt:#?}");
    assert_eq!(rebuilt.external_runs, 0, "{rebuilt:#?}");
    assert!(!rebuilt.skipped, "{rebuilt:#?}");
    assert!(!out_dir.join(".tekai-main.state.json").exists());
    assert_eq!(fs::read(out_dir.join("diagram.1")).unwrap(), script);
    assert_eq!(fixture.invocations(&out_dir), 1);

    fs::write(&main, source.replace("alpha", "beta")).unwrap();
    let edited = build(&options).expect("edited MetaPost fixture build failed");
    assert_eq!(edited.tex_runs, 2, "{edited:#?}");
    assert_eq!(edited.external_runs, 1, "{edited:#?}");
    assert!(!edited.skipped, "{edited:#?}");
    assert!(!out_dir.join(".tekai-main.state.json").exists());
    let edited_script = fs::read(out_dir.join("diagram.mp")).unwrap();
    assert_ne!(edited_script, script);
    assert_eq!(fs::read(out_dir.join("diagram.1")).unwrap(), edited_script);
    assert_eq!(fixture.invocations(&out_dir), 2);
}

#[test]
fn direct_runner_builds_and_caches_metapost_figures() {
    let _test_guard = super::lock();
    if !command_available("pdflatex")
        || !command_available("mpost")
        || !tex_file_available("feynmp.sty")
    {
        eprintln!("skipping MetaPost test; pdflatex, mpost, or feynmp.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("tekai-metapost-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, FEYNMP_DOC).expect("failed to write MetaPost document");

    let first = build(&options(&main, &out_dir)).expect("initial MetaPost build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("diagram.mp").exists());
    assert!(out_dir.join("diagram.1").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    assert!(log.contains("diagram.1 Graphic file"), "{log}");
    assert!(!log.contains("File diagram.1 not found"), "{log}");
    assert!(!log.contains("Process diagram.mp with MetaPost"), "{log}");

    // The auxiliary cache can reuse verified MetaPost output, but the whole
    // PDF cannot be cached without provenance for its generated .mp input.
    let rebuilt = build(&options(&main, &out_dir)).expect("MetaPost rebuild failed");
    assert!(!rebuilt.skipped, "{rebuilt:#?}");
    assert_eq!(rebuilt.tex_runs, 1, "{rebuilt:#?}");
    assert_eq!(rebuilt.external_runs, 0, "{rebuilt:#?}");
    assert!(!out_dir.join(".tekai-main.state.json").exists());

    fs::write(
        &main,
        FEYNMP_DOC.replace(r"\fmf{plain}{i,o}", r"\fmf{dashes}{i,o}"),
    )
    .expect("failed to update MetaPost source");
    let edited = build(&options(&main, &out_dir)).expect("edited MetaPost build failed");
    assert_eq!(edited.external_runs, 1, "{edited:#?}");
    assert_eq!(edited.tex_runs, 2, "{edited:#?}");

    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
struct MetapostFixture {
    root: PathBuf,
    previous_path: Option<std::ffi::OsString>,
}

#[cfg(unix)]
impl MetapostFixture {
    fn new() -> Self {
        use std::os::unix::fs::PermissionsExt;

        let fixture = Self {
            root: unique_temp_dir("tekai-bundled-metapost-test"),
            previous_path: std::env::var_os("PATH"),
        };
        let bin = fixture.root.join("bin");
        fs::create_dir_all(&bin).unwrap();
        let tool = bin.join("mpost");
        fs::write(
            &tool,
            "#!/bin/sh\nset -eu\ninput=\"${1:?missing MetaPost script}\"\n\
             printf '%s\\n' \"$input\" >> mpost.invocations\n\
             cat \"$input\" > \"${input%.mp}.1\"\n",
        )
        .unwrap();
        fs::set_permissions(&tool, fs::Permissions::from_mode(0o755)).unwrap();
        let mut paths = vec![bin];
        if let Some(previous) = &fixture.previous_path {
            paths.extend(std::env::split_paths(previous));
        }
        let path = std::env::join_paths(paths).unwrap();
        unsafe {
            std::env::set_var("PATH", path);
        }
        fixture
    }

    fn invocations(&self, out_dir: &Path) -> usize {
        fs::read_to_string(out_dir.join("mpost.invocations"))
            .expect("fake MetaPost invocation log disappeared")
            .lines()
            .count()
    }
}

#[cfg(unix)]
impl Drop for MetapostFixture {
    fn drop(&mut self) {
        unsafe {
            if let Some(previous) = &self.previous_path {
                std::env::set_var("PATH", previous);
            } else {
                std::env::remove_var("PATH");
            }
        }
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn options(main: &Path, out_dir: &Path) -> BuildOptions {
    BuildOptions {
        main: main.to_path_buf(),
        job_name: None,
        engine: Engine::PdfLatex,
        runner: Runner::Direct,
        bib_mode: BibMode::Auto,
        out_dir: out_dir.to_path_buf(),
        fast: false,
        draft_prepass: DraftPrepass::Never,
        once: false,
        max_runs: 8,
        force: false,
        precompile_preamble: false,
        synctex: false,
        shell_escape: false,
        external_tools: true,
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

fn tex_file_available(name: &str) -> bool {
    std::process::Command::new("kpsewhich")
        .arg(name)
        .status()
        .is_ok_and(|status| status.success())
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock before UNIX epoch")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

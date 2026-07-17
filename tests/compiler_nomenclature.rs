use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use tekai::compiler::{
    BibMode, BuildOptions, DraftPrepass, Engine, Runner, build, build_dependency_paths,
};

const NOMENCLATURE_DOC: &str = r#"\documentclass{article}
\usepackage{nomencl}
\makenomenclature
\begin{document}
Symbol $x$\nomenclature{$x$}{Position}
\printnomenclature
\end{document}
"#;

const INDEX_AND_NOMENCLATURE_DOC: &str = r#"\documentclass{article}
\usepackage{makeidx}
\usepackage{nomencl}
\makeindex
\makenomenclature
\begin{document}
Alpha\index{alpha}
Symbol $x$\nomenclature{$x$}{Position}
\printindex
\printnomenclature
\end{document}
"#;

static INDEXSTYLE_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn direct_runner_builds_and_caches_nomenclature_makeindex_output() {
    let _env_guard = indexstyle_test_guard();
    if !command_available("pdflatex")
        || !command_available("makeindex")
        || !tex_file_available("nomencl.sty")
        || !tex_file_available("nomencl.ist")
    {
        eprintln!(
            "skipping nomenclature build test; pdflatex, makeindex, nomencl.sty, or nomencl.ist is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("tekai-nomenclature-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, NOMENCLATURE_DOC).expect("failed to write nomenclature document");

    let first = build(&options(&main, &out_dir)).expect("initial nomenclature build failed");
    assert_eq!(first.index_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let nls_path = out_dir.join("main.nls");
    let nls = fs::read_to_string(&nls_path).expect("failed to read generated nomenclature");
    assert!(nls.contains("Position"), "{nls}");

    let cached = build(&options(&main, &out_dir)).expect("cached nomenclature build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.index_runs, 0, "{cached:#?}");

    fs::write(
        &main,
        format!("{NOMENCLATURE_DOC}\n% source-only edit that leaves nomenclature unchanged\n"),
    )
    .expect("failed to update nomenclature document");
    let text_edit = build(&options(&main, &out_dir)).expect("text edit build failed");
    assert_eq!(text_edit.index_runs, 0, "{text_edit:#?}");

    fs::write(&main, NOMENCLATURE_DOC.replace("Position", "Velocity"))
        .expect("failed to update nomenclature entry");
    let nomenclature_edit =
        build(&options(&main, &out_dir)).expect("nomenclature edit build failed");
    assert_eq!(nomenclature_edit.index_runs, 1, "{nomenclature_edit:#?}");
    let nls = fs::read_to_string(nls_path).expect("failed to read regenerated nomenclature");
    assert!(nls.contains("Velocity"), "{nls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_builds_multiple_makeindex_family_outputs() {
    let _env_guard = indexstyle_test_guard();
    if !command_available("pdflatex")
        || !command_available("makeindex")
        || !tex_file_available("nomencl.sty")
        || !tex_file_available("nomencl.ist")
    {
        eprintln!(
            "skipping multi MakeIndex-family test; pdflatex, makeindex, nomencl.sty, or nomencl.ist is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("tekai-multi-index-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, INDEX_AND_NOMENCLATURE_DOC)
        .expect("failed to write index and nomenclature document");

    let first = build(&options(&main, &out_dir)).expect("initial multi-index build failed");
    assert_eq!(first.index_runs, 2, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let ind = fs::read_to_string(out_dir.join("main.ind")).expect("failed to read index output");
    let nls =
        fs::read_to_string(out_dir.join("main.nls")).expect("failed to read nomenclature output");
    assert!(ind.contains("alpha"), "{ind}");
    assert!(nls.contains("Position"), "{nls}");

    let cached = build(&options(&main, &out_dir)).expect("cached multi-index build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.index_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn direct_runner_tracks_external_nomenclature_style_from_indexstyle() {
    let _env_guard = indexstyle_test_guard();
    if !command_available("pdflatex")
        || !command_available("makeindex")
        || !tex_file_available("nomencl.sty")
        || !tex_file_available("nomencl.ist")
    {
        eprintln!(
            "skipping INDEXSTYLE nomenclature test; pdflatex, makeindex, nomencl.sty, or nomencl.ist is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("tekai-nomenclature-indexstyle-test");
    let style_root = unique_temp_dir("tekai-nomenclature-indexstyle-external");
    fs::create_dir_all(&root).expect("failed to create test directory");
    fs::create_dir_all(&style_root).expect("failed to create external style directory");
    let main = root.join("main.tex");
    let style_path = style_root.join("nomencl.ist");
    let out_dir = root.join("out");
    fs::write(&main, NOMENCLATURE_DOC).expect("failed to write nomenclature document");
    fs::copy(
        tex_file_path("nomencl.ist").expect("failed to locate system nomencl.ist"),
        &style_path,
    )
    .expect("failed to write external nomenclature style");

    let _texindexstyle = EnvVarGuard::unset("TEXINDEXSTYLE");
    let _indexstyle = EnvVarGuard::set(
        "INDEXSTYLE",
        OsString::from(format!("{}//:", style_root.display())),
    );

    let first = build(&options(&main, &out_dir)).expect("initial INDEXSTYLE build failed");
    assert_eq!(first.index_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let dependency_paths =
        build_dependency_paths(&options(&main, &out_dir)).expect("failed to read dependencies");
    assert!(
        dependency_paths
            .iter()
            .any(|path| path == &style_path.canonicalize().unwrap()),
        "{dependency_paths:#?}"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached INDEXSTYLE build failed");
    assert!(cached.skipped, "{cached:#?}");

    let original_nls =
        fs::read_to_string(out_dir.join("main.nls")).expect("failed to read nomenclature output");
    fs::write(
        &style_path,
        format!(
            "{}\n% tekai external style cache invalidation marker\n",
            fs::read_to_string(&style_path).expect("failed to read external style")
        ),
    )
    .expect("failed to update external nomenclature style");
    let style_edit = build(&options(&main, &out_dir)).expect("style edit build failed");
    assert_eq!(style_edit.index_runs, 1, "{style_edit:#?}");
    assert_eq!(style_edit.tex_runs, 0, "{style_edit:#?}");
    assert!(style_edit.aux_preflight_used, "{style_edit:#?}");
    let updated_nls =
        fs::read_to_string(out_dir.join("main.nls")).expect("failed to read nomenclature output");
    assert_eq!(updated_nls, original_nls);

    let _ = fs::remove_dir_all(root);
    let _ = fs::remove_dir_all(style_root);
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
    Command::new("kpsewhich")
        .arg(name)
        .output()
        .map(|output| output.status.success() && !output.stdout.is_empty())
        .unwrap_or(false)
}

fn tex_file_path(name: &str) -> Option<PathBuf> {
    let output = Command::new("kpsewhich").arg(name).output().ok()?;
    if !output.status.success() || output.stdout.is_empty() {
        return None;
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!path.is_empty()).then(|| PathBuf::from(path))
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

fn indexstyle_test_guard() -> std::sync::MutexGuard<'static, ()> {
    INDEXSTYLE_TEST_LOCK
        .lock()
        .expect("INDEXSTYLE test lock poisoned")
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

    fn unset(name: &'static str) -> Self {
        let previous = std::env::var_os(name);
        unsafe {
            std::env::remove_var(name);
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

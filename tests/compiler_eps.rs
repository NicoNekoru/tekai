use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use texpilot::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const EPS_DOC: &str = r#"\documentclass{article}
\usepackage{graphicx}
\begin{document}
\includegraphics[width=1cm]{fig.eps}
\end{document}
"#;

const KPATHSEA_EPS_DOC: &str = r#"\documentclass{article}
\usepackage{graphicx}
\begin{document}
\includegraphics[width=1cm]{shared-fig}
\end{document}
"#;

const MULTILINE_EPS_DOC: &str = r#"\documentclass{article}
\usepackage{graphicx}
\graphicspath{
  {figures/}
}
\begin{document}
\includegraphics*[
  width=1cm
]{fig}
\end{document}
"#;

const DECLARED_GRAPHICS_EXTENSIONS_EPS_DOC: &str = r#"\documentclass{article}
\usepackage{graphicx}
\DeclareGraphicsExtensions{.eps,.png,.pdf}
\begin{document}
\includegraphics[width=1cm]{fig}
\end{document}
"#;

const SUBFILE_EPS_DOC: &str = r#"\documentclass{article}
\usepackage{graphicx}
\newcommand{\subfile}[1]{\input{#1}}
\begin{document}
\subfile{sections/figure-section}
\end{document}
"#;

const IF_FILE_EXISTS_EPS_DOC: &str = r#"\documentclass{article}
\usepackage{graphicx}
\begin{document}
\IfFileExists{sections/figure-section.tex}{\input{sections/figure-section}}{Missing section.}
\end{document}
"#;

const IMPORT_EPS_DOC: &str = r#"\documentclass{article}
\usepackage{graphicx}
\newcommand{\import}[2]{\begingroup\graphicspath{{#1}}\input{#1#2}\endgroup}
\begin{document}
\import{sections/}{figure-section}
\end{document}
"#;

const INCLUDEONLY_EPS_DOC: &str = r#"\documentclass{article}
\usepackage{graphicx}
\includeonly{active}
\begin{document}
\include{active}
\include{excluded}
\end{document}
"#;

const SUBFILE_SECTION_TEX: &str = r#"Before.
\includegraphics[width=1cm]{sections/fig}
After.
"#;

const IMPORT_SECTION_TEX: &str = r#"Before.
\includegraphics[width=1cm]{fig}
After.
"#;

const EPS_SOURCE: &str = r#"%!PS-Adobe-3.0 EPSF-3.0
%%BoundingBox: 0 0 100 100
newpath 10 10 moveto 90 90 lineto 10 90 lineto closepath stroke
showpage
%%EOF
"#;

const ONE_BY_ONE_PNG: &[u8] = &[
    0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, b'I', b'H', b'D', b'R',
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0d, b'I', b'D', b'A', b'T', 0x78, 0x9c, 0x63, 0xf8, 0xff, 0xff, 0xff,
    0x7f, 0x00, 0x09, 0xfb, 0x03, 0xfd, 0x05, 0x43, 0x45, 0xca, 0x00, 0x00, 0x00, 0x00, b'I', b'E',
    b'N', b'D', 0xae, 0x42, 0x60, 0x82,
];

static EPS_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
#[cfg(unix)]
fn direct_runner_builds_and_caches_eps_conversions() {
    if !command_available("pdflatex") || !command_available("epstopdf") {
        eprintln!("skipping EPS conversion test; pdflatex or epstopdf is unavailable");
        return;
    }

    let _guard = EPS_TEST_LOCK.lock().expect("EPS test lock poisoned");
    let root = unique_temp_dir("texpilot-eps-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let eps = root.join("fig.eps");
    let fixture_eps = root.join("fixture.eps");
    let fixture_pdf = root.join("fixture.pdf");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, EPS_DOC).expect("failed to write EPS document");
    fs::write(&eps, EPS_SOURCE).expect("failed to write EPS figure");
    fs::write(&fixture_eps, EPS_SOURCE).expect("failed to write fixture EPS");
    let status = Command::new("epstopdf")
        .arg(&fixture_eps)
        .arg(format!("--outfile={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture epstopdf");
    assert!(status.success(), "fixture epstopdf failed with {status}");

    write_fake_epstopdf(&fake_bin.join("epstopdf"), &fixture_pdf);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial EPS build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("fig-eps-converted-to.pdf").exists());
    assert!(!root.join("fig-eps-converted-to.pdf").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    assert!(log.contains("fig-eps-converted-to.pdf"), "{log}");
    assert_eq!(fake_invocations(&out_dir), 1);

    let cached = build(&options(&main, &out_dir)).expect("cached EPS build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir), 1);

    fs::write(
        &eps,
        format!("{EPS_SOURCE}\n% source edit that leaves fake output unchanged\n"),
    )
    .expect("failed to update EPS figure");
    let eps_edit = build(&options(&main, &out_dir)).expect("EPS edit build failed");
    assert_eq!(eps_edit.external_runs, 1, "{eps_edit:#?}");
    assert_eq!(eps_edit.tex_runs, 0, "{eps_edit:#?}");
    assert!(eps_edit.aux_preflight_used, "{eps_edit:#?}");
    assert_eq!(fake_invocations(&out_dir), 2);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_converts_eps_discovered_through_kpathsea() {
    if !command_available("pdflatex") || !command_available("epstopdf") {
        eprintln!("skipping Kpathsea EPS conversion test; pdflatex or epstopdf is unavailable");
        return;
    }

    let _guard = EPS_TEST_LOCK.lock().expect("EPS test lock poisoned");
    let root = unique_temp_dir("texpilot-kpathsea-eps-test");
    let shared_figures = root.join("shared").join("figures");
    fs::create_dir_all(&shared_figures).expect("failed to create shared figure directory");
    let main = root.join("main.tex");
    let eps = shared_figures.join("shared-fig.eps");
    let fixture_eps = root.join("fixture.eps");
    let fixture_pdf = root.join("fixture.pdf");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, KPATHSEA_EPS_DOC).expect("failed to write Kpathsea EPS document");
    fs::write(&eps, EPS_SOURCE).expect("failed to write Kpathsea EPS figure");
    fs::write(&fixture_eps, EPS_SOURCE).expect("failed to write fixture EPS");
    let status = Command::new("epstopdf")
        .arg(&fixture_eps)
        .arg(format!("--outfile={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture epstopdf");
    assert!(status.success(), "fixture epstopdf failed with {status}");

    write_fake_epstopdf(&fake_bin.join("epstopdf"), &fixture_pdf);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial Kpathsea EPS build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(
        out_dir
            .join("shared")
            .join("figures")
            .join("shared-fig.pdf")
            .exists()
    );
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    assert!(log.contains("shared-fig.pdf"), "{log}");
    assert_eq!(fake_invocations(&out_dir.join("shared").join("figures")), 1);

    let cached = build(&options(&main, &out_dir)).expect("cached Kpathsea EPS build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("shared").join("figures")), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_detects_starred_multiline_graphics_commands_for_eps_conversion() {
    if !command_available("pdflatex") || !command_available("epstopdf") {
        eprintln!(
            "skipping starred multiline EPS conversion test; pdflatex or epstopdf is unavailable"
        );
        return;
    }

    let _guard = EPS_TEST_LOCK.lock().expect("EPS test lock poisoned");
    let root = unique_temp_dir("texpilot-starred-multiline-eps-test");
    let figures = root.join("figures");
    fs::create_dir_all(&figures).expect("failed to create figure directory");
    let main = root.join("main.tex");
    let eps = figures.join("fig.eps");
    let fixture_eps = root.join("fixture.eps");
    let fixture_pdf = root.join("fixture.pdf");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, MULTILINE_EPS_DOC).expect("failed to write EPS document");
    fs::write(&eps, EPS_SOURCE).expect("failed to write EPS figure");
    fs::write(&fixture_eps, EPS_SOURCE).expect("failed to write fixture EPS");
    let status = Command::new("epstopdf")
        .arg(&fixture_eps)
        .arg(format!("--outfile={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture epstopdf");
    assert!(status.success(), "fixture epstopdf failed with {status}");

    write_fake_epstopdf(&fake_bin.join("epstopdf"), &fixture_pdf);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first =
        build(&options(&main, &out_dir)).expect("initial starred multiline EPS build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("figures").join("fig.pdf").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    assert!(log.contains("fig.pdf"), "{log}");
    assert_eq!(fake_invocations(&out_dir.join("figures")), 1);

    let cached =
        build(&options(&main, &out_dir)).expect("cached starred multiline EPS build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("figures")), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_honors_declared_graphics_extension_order_for_eps() {
    if !command_available("pdflatex") || !command_available("epstopdf") {
        eprintln!(
            "skipping declared graphics extension EPS test; pdflatex or epstopdf is unavailable"
        );
        return;
    }

    let _guard = EPS_TEST_LOCK.lock().expect("EPS test lock poisoned");
    let root = unique_temp_dir("texpilot-declared-extensions-eps-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let eps = root.join("fig.eps");
    let png = root.join("fig.png");
    let fixture_eps = root.join("fixture.eps");
    let fixture_pdf = root.join("fixture.pdf");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, DECLARED_GRAPHICS_EXTENSIONS_EPS_DOC).expect("failed to write EPS document");
    fs::write(&eps, EPS_SOURCE).expect("failed to write EPS figure");
    fs::write(&png, ONE_BY_ONE_PNG).expect("failed to write sibling PNG figure");
    fs::write(&fixture_eps, EPS_SOURCE).expect("failed to write fixture EPS");
    let status = Command::new("epstopdf")
        .arg(&fixture_eps)
        .arg(format!("--outfile={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture epstopdf");
    assert!(status.success(), "fixture epstopdf failed with {status}");

    write_fake_epstopdf(&fake_bin.join("epstopdf"), &fixture_pdf);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first =
        build(&options(&main, &out_dir)).expect("initial declared extension EPS build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("fig.pdf").exists());
    assert_eq!(fake_invocations(&out_dir), 1);

    let cached =
        build(&options(&main, &out_dir)).expect("cached declared extension EPS build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_converts_eps_discovered_through_subfile_sources() {
    if !command_available("pdflatex") || !command_available("epstopdf") {
        eprintln!("skipping subfile EPS conversion test; pdflatex or epstopdf is unavailable");
        return;
    }

    let _guard = EPS_TEST_LOCK.lock().expect("EPS test lock poisoned");
    let root = unique_temp_dir("texpilot-subfile-eps-test");
    let sections = root.join("sections");
    fs::create_dir_all(&sections).expect("failed to create section directory");
    let main = root.join("main.tex");
    let section = sections.join("figure-section.tex");
    let eps = sections.join("fig.eps");
    let fixture_eps = root.join("fixture.eps");
    let fixture_pdf = root.join("fixture.pdf");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, SUBFILE_EPS_DOC).expect("failed to write EPS document");
    fs::write(&section, SUBFILE_SECTION_TEX).expect("failed to write subfile section");
    fs::write(&eps, EPS_SOURCE).expect("failed to write EPS figure");
    fs::write(&fixture_eps, EPS_SOURCE).expect("failed to write fixture EPS");
    let status = Command::new("epstopdf")
        .arg(&fixture_eps)
        .arg(format!("--outfile={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture epstopdf");
    assert!(status.success(), "fixture epstopdf failed with {status}");

    write_fake_epstopdf(&fake_bin.join("epstopdf"), &fixture_pdf);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial subfile EPS build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("sections").join("fig.pdf").exists());
    assert_eq!(fake_invocations(&out_dir.join("sections")), 1);

    let cached = build(&options(&main, &out_dir)).expect("cached subfile EPS build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("sections")), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_converts_eps_discovered_through_if_file_exists_sources() {
    if !command_available("pdflatex") || !command_available("epstopdf") {
        eprintln!("skipping IfFileExists EPS conversion test; pdflatex or epstopdf is unavailable");
        return;
    }

    let _guard = EPS_TEST_LOCK.lock().expect("EPS test lock poisoned");
    let root = unique_temp_dir("texpilot-if-file-exists-eps-test");
    let sections = root.join("sections");
    fs::create_dir_all(&sections).expect("failed to create section directory");
    let main = root.join("main.tex");
    let section = sections.join("figure-section.tex");
    let eps = sections.join("fig.eps");
    let fixture_eps = root.join("fixture.eps");
    let fixture_pdf = root.join("fixture.pdf");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, IF_FILE_EXISTS_EPS_DOC).expect("failed to write EPS document");
    fs::write(&section, SUBFILE_SECTION_TEX).expect("failed to write conditional section");
    fs::write(&eps, EPS_SOURCE).expect("failed to write EPS figure");
    fs::write(&fixture_eps, EPS_SOURCE).expect("failed to write fixture EPS");
    let status = Command::new("epstopdf")
        .arg(&fixture_eps)
        .arg(format!("--outfile={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture epstopdf");
    assert!(status.success(), "fixture epstopdf failed with {status}");

    write_fake_epstopdf(&fake_bin.join("epstopdf"), &fixture_pdf);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial IfFileExists EPS build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("sections").join("fig.pdf").exists());
    assert_eq!(fake_invocations(&out_dir.join("sections")), 1);

    let cached = build(&options(&main, &out_dir)).expect("cached IfFileExists EPS build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("sections")), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_converts_eps_discovered_through_import_sources() {
    if !command_available("pdflatex") || !command_available("epstopdf") {
        eprintln!("skipping import EPS conversion test; pdflatex or epstopdf is unavailable");
        return;
    }

    let _guard = EPS_TEST_LOCK.lock().expect("EPS test lock poisoned");
    let root = unique_temp_dir("texpilot-import-eps-test");
    let sections = root.join("sections");
    fs::create_dir_all(&sections).expect("failed to create section directory");
    let main = root.join("main.tex");
    let section = sections.join("figure-section.tex");
    let eps = sections.join("fig.eps");
    let fixture_eps = root.join("fixture.eps");
    let fixture_pdf = root.join("fixture.pdf");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, IMPORT_EPS_DOC).expect("failed to write EPS document");
    fs::write(&section, IMPORT_SECTION_TEX).expect("failed to write imported section");
    fs::write(&eps, EPS_SOURCE).expect("failed to write EPS figure");
    fs::write(&fixture_eps, EPS_SOURCE).expect("failed to write fixture EPS");
    let status = Command::new("epstopdf")
        .arg(&fixture_eps)
        .arg(format!("--outfile={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture epstopdf");
    assert!(status.success(), "fixture epstopdf failed with {status}");

    write_fake_epstopdf(&fake_bin.join("epstopdf"), &fixture_pdf);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial import EPS build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("sections").join("fig.pdf").exists());
    assert_eq!(fake_invocations(&out_dir.join("sections")), 1);

    let cached = build(&options(&main, &out_dir)).expect("cached import EPS build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("sections")), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_ignores_eps_in_includeonly_excluded_sources() {
    if !command_available("pdflatex") || !command_available("epstopdf") {
        eprintln!("skipping includeonly EPS exclusion test; pdflatex or epstopdf is unavailable");
        return;
    }

    let _guard = EPS_TEST_LOCK.lock().expect("EPS test lock poisoned");
    let root = unique_temp_dir("texpilot-includeonly-eps-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let active = root.join("active.tex");
    let excluded = root.join("excluded.tex");
    let eps = root.join("excluded.eps");
    let fixture_eps = root.join("fixture.eps");
    let fixture_pdf = root.join("fixture.pdf");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, INCLUDEONLY_EPS_DOC).expect("failed to write includeonly EPS document");
    fs::write(&active, "Active chapter.\n").expect("failed to write active chapter");
    fs::write(
        &excluded,
        "Excluded chapter.\\includegraphics[width=1cm]{excluded.eps}\n",
    )
    .expect("failed to write excluded chapter");
    fs::write(&eps, EPS_SOURCE).expect("failed to write excluded EPS");
    fs::write(&fixture_eps, EPS_SOURCE).expect("failed to write fixture EPS");
    let status = Command::new("epstopdf")
        .arg(&fixture_eps)
        .arg(format!("--outfile={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture epstopdf");
    assert!(status.success(), "fixture epstopdf failed with {status}");

    write_fake_epstopdf(&fake_bin.join("epstopdf"), &fixture_pdf);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("includeonly EPS build failed");
    assert_eq!(first.external_runs, 0, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(!out_dir.join("excluded-eps-converted-to.pdf").exists());
    assert_eq!(fake_invocations(&out_dir), 0);

    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
fn write_fake_epstopdf(path: &Path, fixture_pdf: &Path) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(
        path,
        format!(
            r#"#!/bin/sh
set -eu
outfile=""
for arg in "$@"; do
  case "$arg" in
    --outfile=*) outfile="${{arg#--outfile=}}" ;;
  esac
done
if [ -z "$outfile" ]; then
  echo "missing --outfile" >&2
  exit 2
fi
printf '%s\n' "$*" >> "$(dirname "$outfile")/epstopdf.invocations"
cp '{}' "$outfile"
"#,
            fixture_pdf.display()
        ),
    )
    .expect("failed to write fake epstopdf");
    let mut permissions = fs::metadata(path)
        .expect("failed to stat fake epstopdf")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("failed to chmod fake epstopdf");
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
        max_runs: 6,
        force: false,
        precompile_preamble: false,
        synctex: false,
        shell_escape: false,
        quiet: true,
        print_command: false,
    }
}

fn fake_invocations(out_dir: &Path) -> usize {
    fs::read_to_string(out_dir.join("epstopdf.invocations"))
        .map(|source| source.lines().count())
        .unwrap_or(0)
}

fn command_available(program: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|directory| directory.join(program).is_file())
        })
        .unwrap_or(false)
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

#[cfg(unix)]
struct PathGuard {
    previous: Option<OsString>,
}

#[cfg(unix)]
impl PathGuard {
    fn prepend(directory: &Path) -> Self {
        let previous = std::env::var_os("PATH");
        let mut paths = vec![directory.to_path_buf()];
        if let Some(previous_paths) = previous.as_ref() {
            paths.extend(std::env::split_paths(previous_paths));
        }
        let joined = std::env::join_paths(paths).expect("failed to join PATH");
        unsafe {
            std::env::set_var("PATH", joined);
        }
        Self { previous }
    }
}

#[cfg(unix)]
impl Drop for PathGuard {
    fn drop(&mut self) {
        unsafe {
            if let Some(previous) = &self.previous {
                std::env::set_var("PATH", previous);
            } else {
                std::env::remove_var("PATH");
            }
        }
    }
}

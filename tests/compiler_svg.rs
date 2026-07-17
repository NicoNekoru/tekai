use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const SVG_DOC: &str = r#"\documentclass{article}
\usepackage{svg}
\begin{document}
\includesvg[width=2cm]{fig}
\end{document}
"#;

const SVG_SETUP_RAW_DOC: &str = r#"\documentclass{article}
\usepackage{svg}
\svgsetup{latex=false,inkscapearea=nocrop}
\begin{document}
\includesvg[width=2cm]{fig}
\end{document}
"#;

const SVG_CUSTOM_NAME_DOC: &str = r#"\documentclass{article}
\usepackage{svg}
\svgsetup{inkscapename=custom-figure}
\begin{document}
\includesvg[width=2cm]{fig}
\end{document}
"#;

const SVG_CUSTOM_EXE_DOC: &str = r#"\documentclass{article}
\usepackage{svg}
\svgsetup{inkscapeexe=custom-inkscape,latex=false}
\begin{document}
\includesvg[width=2cm]{fig}
\end{document}
"#;

const SVG_PNG_DOC: &str = r#"\documentclass{article}
\usepackage{svg}
\svgsetup{inkscapeformat=png,inkscapearea=page,inkscapedensity=144dpi}
\begin{document}
\includesvg[width=2cm]{fig}
\end{document}
"#;

const SVG_EXTENSION_DOC: &str = r#"\documentclass{article}
\usepackage{svg}
\svgsetup{ext=svgz,latex=false}
\begin{document}
\includesvg[width=2cm]{fig}
\end{document}
"#;

const SVG_SOURCE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="40" viewBox="0 0 100 40">
<rect width="100" height="40" fill="white"/>
<text x="10" y="25" font-size="20">SVG</text>
</svg>
"#;

static SVG_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
#[cfg(unix)]
fn direct_runner_builds_and_caches_svg_conversions() {
    if !command_available("pdflatex")
        || !command_available("inkscape")
        || !tex_file_available("svg.sty")
    {
        eprintln!("skipping SVG conversion test; pdflatex, inkscape, or svg.sty is unavailable");
        return;
    }

    let _guard = SVG_TEST_LOCK.lock().expect("SVG test lock poisoned");
    let root = unique_temp_dir("tekai-svg-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let svg = root.join("fig.svg");
    let fixture_svg = root.join("fixture.svg");
    let fixture_pdf = root.join("fixture.pdf");
    let fixture_pdf_tex = root.join("fixture.pdf_tex");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, SVG_DOC).expect("failed to write SVG document");
    fs::write(&svg, SVG_SOURCE).expect("failed to write SVG figure");
    fs::write(&fixture_svg, SVG_SOURCE).expect("failed to write fixture SVG");
    let status = Command::new("inkscape")
        .arg(&fixture_svg)
        .arg("--export-area-drawing")
        .arg("--export-latex")
        .arg(format!("--export-filename={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture Inkscape");
    assert!(status.success(), "fixture Inkscape failed with {status}");
    assert!(fixture_pdf.exists(), "fixture PDF was not created");
    assert!(fixture_pdf_tex.exists(), "fixture PDF_TEX was not created");

    write_fake_inkscape(&fake_bin.join("inkscape"), &fixture_pdf, &fixture_pdf_tex);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial SVG build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("svg-inkscape/fig_svg-tex.pdf").exists());
    assert!(out_dir.join("svg-inkscape/fig_svg-tex.pdf_tex").exists());
    assert!(!root.join("svg-inkscape").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    let compact_log = log.split_whitespace().collect::<String>();
    assert!(
        compact_log.contains("svg-inkscape/fig_svg-tex.pdf_tex"),
        "{log}"
    );
    assert_eq!(fake_invocations(&out_dir.join("svg-inkscape")), 1);

    let cached = build(&options(&main, &out_dir)).expect("cached SVG build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("svg-inkscape")), 1);

    fs::write(&svg, SVG_SOURCE.replace("SVG", "SVG edit")).expect("failed to update SVG figure");
    let svg_edit = build(&options(&main, &out_dir)).expect("SVG edit build failed");
    assert_eq!(svg_edit.external_runs, 1, "{svg_edit:#?}");
    assert_eq!(svg_edit.tex_runs, 0, "{svg_edit:#?}");
    assert!(svg_edit.aux_preflight_used, "{svg_edit:#?}");
    assert_eq!(fake_invocations(&out_dir.join("svg-inkscape")), 2);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_honors_svgsetup_for_raw_page_exports() {
    if !command_available("pdflatex")
        || !command_available("inkscape")
        || !tex_file_available("svg.sty")
    {
        eprintln!("skipping SVG setup test; pdflatex, inkscape, or svg.sty is unavailable");
        return;
    }

    let _guard = SVG_TEST_LOCK.lock().expect("SVG test lock poisoned");
    let root = unique_temp_dir("tekai-svgsetup-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let svg = root.join("fig.svg");
    let fixture_svg = root.join("fixture.svg");
    let fixture_pdf = root.join("fixture.pdf");
    let fixture_pdf_tex = root.join("fixture.pdf_tex");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, SVG_SETUP_RAW_DOC).expect("failed to write SVG setup document");
    fs::write(&svg, SVG_SOURCE).expect("failed to write SVG figure");
    fs::write(&fixture_svg, SVG_SOURCE).expect("failed to write fixture SVG");
    let status = Command::new("inkscape")
        .arg(&fixture_svg)
        .arg("--export-area-page")
        .arg(format!("--export-filename={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture Inkscape");
    assert!(status.success(), "fixture Inkscape failed with {status}");
    assert!(fixture_pdf.exists(), "fixture PDF was not created");
    assert!(
        !fixture_pdf_tex.exists(),
        "raw fixture should not create PDF_TEX"
    );

    write_fake_inkscape(&fake_bin.join("inkscape"), &fixture_pdf, &fixture_pdf_tex);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial SVG setup build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("svg-inkscape/fig_svg-raw.pdf").exists());
    assert!(!out_dir.join("svg-inkscape/fig_svg-raw.pdf_tex").exists());

    let invocation_log = fake_invocation_log(&out_dir.join("svg-inkscape"));
    assert!(
        invocation_log.contains("--export-area-page"),
        "{invocation_log}"
    );
    assert!(
        !invocation_log.contains("--export-latex"),
        "{invocation_log}"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached SVG setup build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("svg-inkscape")), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_honors_svgsetup_png_exports() {
    if !command_available("pdflatex")
        || !command_available("inkscape")
        || !tex_file_available("svg.sty")
    {
        eprintln!("skipping SVG PNG test; pdflatex, inkscape, or svg.sty is unavailable");
        return;
    }

    let _guard = SVG_TEST_LOCK.lock().expect("SVG test lock poisoned");
    let root = unique_temp_dir("tekai-svgpng-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let svg = root.join("fig.svg");
    let fixture_svg = root.join("fixture.svg");
    let fixture_png = root.join("fixture.png");
    let fixture_png_tex = root.join("fixture.png_tex");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, SVG_PNG_DOC).expect("failed to write SVG PNG document");
    fs::write(&svg, SVG_SOURCE).expect("failed to write SVG figure");
    fs::write(&fixture_svg, SVG_SOURCE).expect("failed to write fixture SVG");
    let status = Command::new("inkscape")
        .arg(&fixture_svg)
        .arg("--export-area-page")
        .arg("--export-dpi=144")
        .arg(format!("--export-filename={}", fixture_png.display()))
        .status()
        .expect("failed to launch fixture Inkscape");
    assert!(status.success(), "fixture Inkscape failed with {status}");
    assert!(fixture_png.exists(), "fixture PNG was not created");
    assert!(
        !fixture_png_tex.exists(),
        "PNG fixture should not create a TeX overlay"
    );

    write_fake_inkscape(&fake_bin.join("inkscape"), &fixture_png, &fixture_png_tex);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial SVG PNG build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("svg-inkscape/fig_svg-raw.png").exists());
    assert!(!out_dir.join("svg-inkscape/fig_svg-raw.png_tex").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    let compact_log = log.split_whitespace().collect::<String>();
    assert!(compact_log.contains("fig_svg-raw.png"), "{log}");

    let invocation_log = fake_invocation_log(&out_dir.join("svg-inkscape"));
    assert!(
        invocation_log.contains("--export-area-page"),
        "{invocation_log}"
    );
    assert!(
        invocation_log.contains("fig_svg-raw.png"),
        "{invocation_log}"
    );
    assert!(
        invocation_log.contains("--export-dpi=144"),
        "{invocation_log}"
    );
    assert!(
        !invocation_log.contains("--export-latex"),
        "{invocation_log}"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached SVG PNG build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("svg-inkscape")), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_honors_svgextension_for_source_and_output_suffix() {
    if !command_available("pdflatex")
        || !command_available("inkscape")
        || !tex_file_available("svg.sty")
    {
        eprintln!("skipping SVG extension test; pdflatex, inkscape, or svg.sty is unavailable");
        return;
    }

    let _guard = SVG_TEST_LOCK.lock().expect("SVG test lock poisoned");
    let root = unique_temp_dir("tekai-svgextension-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let svgz = root.join("fig.svgz");
    let fixture_svg = root.join("fixture.svg");
    let fixture_pdf = root.join("fixture.pdf");
    let fixture_pdf_tex = root.join("fixture.pdf_tex");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, SVG_EXTENSION_DOC).expect("failed to write SVG extension document");
    fs::write(&svgz, SVG_SOURCE).expect("failed to write SVGZ-named figure");
    fs::write(&fixture_svg, SVG_SOURCE).expect("failed to write fixture SVG");
    let status = Command::new("inkscape")
        .arg(&fixture_svg)
        .arg("--export-area-drawing")
        .arg(format!("--export-filename={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture Inkscape");
    assert!(status.success(), "fixture Inkscape failed with {status}");
    assert!(fixture_pdf.exists(), "fixture PDF was not created");
    assert!(
        !fixture_pdf_tex.exists(),
        "raw extension fixture should not create PDF_TEX"
    );

    write_fake_inkscape(&fake_bin.join("inkscape"), &fixture_pdf, &fixture_pdf_tex);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial SVG extension build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("svg-inkscape/fig_svgz-raw.pdf").exists());
    assert!(!out_dir.join("svg-inkscape/fig_svg-raw.pdf").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    let compact_log = log.split_whitespace().collect::<String>();
    assert!(compact_log.contains("fig_svgz-raw.pdf"), "{log}");

    let invocation_log = fake_invocation_log(&out_dir.join("svg-inkscape"));
    assert!(invocation_log.contains("fig.svgz"), "{invocation_log}");
    assert!(
        invocation_log.contains("fig_svgz-raw.pdf"),
        "{invocation_log}"
    );
    assert!(
        !invocation_log.contains("--export-latex"),
        "{invocation_log}"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached SVG extension build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("svg-inkscape")), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_honors_svgsetup_inkscapename() {
    if !command_available("pdflatex")
        || !command_available("inkscape")
        || !tex_file_available("svg.sty")
    {
        eprintln!("skipping SVG name test; pdflatex, inkscape, or svg.sty is unavailable");
        return;
    }

    let _guard = SVG_TEST_LOCK.lock().expect("SVG test lock poisoned");
    let root = unique_temp_dir("tekai-svgname-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let svg = root.join("fig.svg");
    let fixture_svg = root.join("fixture.svg");
    let fixture_pdf = root.join("fixture.pdf");
    let fixture_pdf_tex = root.join("fixture.pdf_tex");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, SVG_CUSTOM_NAME_DOC).expect("failed to write SVG name document");
    fs::write(&svg, SVG_SOURCE).expect("failed to write SVG figure");
    fs::write(&fixture_svg, SVG_SOURCE).expect("failed to write fixture SVG");
    let status = Command::new("inkscape")
        .arg(&fixture_svg)
        .arg("--export-area-drawing")
        .arg("--export-latex")
        .arg(format!("--export-filename={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture Inkscape");
    assert!(status.success(), "fixture Inkscape failed with {status}");
    assert!(fixture_pdf.exists(), "fixture PDF was not created");
    assert!(fixture_pdf_tex.exists(), "fixture PDF_TEX was not created");

    write_fake_inkscape(&fake_bin.join("inkscape"), &fixture_pdf, &fixture_pdf_tex);
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial SVG name build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(
        out_dir
            .join("svg-inkscape/custom-figure_svg-tex.pdf")
            .exists()
    );
    assert!(
        out_dir
            .join("svg-inkscape/custom-figure_svg-tex.pdf_tex")
            .exists()
    );
    assert!(!out_dir.join("svg-inkscape/fig_svg-tex.pdf").exists());

    let invocation_log = fake_invocation_log(&out_dir.join("svg-inkscape"));
    assert!(invocation_log.contains("custom-figure_svg-tex.pdf"));

    let cached = build(&options(&main, &out_dir)).expect("cached SVG name build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("svg-inkscape")), 1);

    let _ = fs::remove_dir_all(root);
}

#[test]
#[cfg(unix)]
fn direct_runner_honors_svgsetup_inkscapeexe_command_name() {
    if !command_available("pdflatex")
        || !command_available("inkscape")
        || !tex_file_available("svg.sty")
    {
        eprintln!("skipping SVG executable test; pdflatex, inkscape, or svg.sty is unavailable");
        return;
    }

    let _guard = SVG_TEST_LOCK.lock().expect("SVG test lock poisoned");
    let root = unique_temp_dir("tekai-svgexe-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let svg = root.join("fig.svg");
    let fixture_svg = root.join("fixture.svg");
    let fixture_pdf = root.join("fixture.pdf");
    let fixture_pdf_tex = root.join("fixture.pdf_tex");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, SVG_CUSTOM_EXE_DOC).expect("failed to write SVG executable document");
    fs::write(&svg, SVG_SOURCE).expect("failed to write SVG figure");
    fs::write(&fixture_svg, SVG_SOURCE).expect("failed to write fixture SVG");
    let status = Command::new("inkscape")
        .arg(&fixture_svg)
        .arg("--export-area-drawing")
        .arg(format!("--export-filename={}", fixture_pdf.display()))
        .status()
        .expect("failed to launch fixture Inkscape");
    assert!(status.success(), "fixture Inkscape failed with {status}");
    assert!(fixture_pdf.exists(), "fixture PDF was not created");
    assert!(
        !fixture_pdf_tex.exists(),
        "raw custom executable fixture should not create PDF_TEX"
    );

    write_fake_inkscape(
        &fake_bin.join("custom-inkscape"),
        &fixture_pdf,
        &fixture_pdf_tex,
    );
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial SVG executable build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("svg-inkscape/fig_svg-raw.pdf").exists());
    assert!(!out_dir.join("svg-inkscape/fig_svg-raw.pdf_tex").exists());

    let invocation_log = fake_invocation_log(&out_dir.join("svg-inkscape"));
    assert!(invocation_log.contains("fig.svg"), "{invocation_log}");
    assert!(
        invocation_log.contains("fig_svg-raw.pdf"),
        "{invocation_log}"
    );
    assert!(
        !invocation_log.contains("--export-latex"),
        "{invocation_log}"
    );

    let cached = build(&options(&main, &out_dir)).expect("cached SVG executable build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir.join("svg-inkscape")), 1);

    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
fn write_fake_inkscape(path: &Path, fixture_pdf: &Path, fixture_pdf_tex: &Path) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(
        path,
        format!(
            r#"#!/bin/sh
set -eu
outfile=""
latex=0
for arg in "$@"; do
  case "$arg" in
    --export-filename=*) outfile="${{arg#--export-filename=}}" ;;
    --export-latex) latex=1 ;;
  esac
done
if [ -z "$outfile" ]; then
  echo "missing --export-filename" >&2
  exit 2
fi
printf '%s\n' "$*" >> "$(dirname "$outfile")/inkscape.invocations"
cp '{}' "$outfile"
if [ "$latex" -eq 1 ]; then
  cp '{}' "$outfile"_tex
fi
"#,
            fixture_pdf.display(),
            fixture_pdf_tex.display()
        ),
    )
    .expect("failed to write fake Inkscape");
    let mut permissions = fs::metadata(path)
        .expect("failed to stat fake Inkscape")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("failed to chmod fake Inkscape");
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
    fake_invocation_log(out_dir).lines().count()
}

fn fake_invocation_log(out_dir: &Path) -> String {
    fs::read_to_string(out_dir.join("inkscape.invocations")).unwrap_or_default()
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

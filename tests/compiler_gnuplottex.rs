use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const GNUPLOTTEX_DOC: &str = r#"\documentclass{article}
\usepackage[noshell]{gnuplottex}
\begin{document}
Before.
\begin{gnuplot}
plot "../points.dat" using 1:2 with lines title "sample"
\end{gnuplot}
After.
\end{document}
"#;

static GNUPLOTTEX_TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
#[cfg(unix)]
fn direct_runner_builds_and_caches_gnuplottex_outputs() {
    if !command_available("pdflatex") || !tex_file_available("gnuplottex.sty") {
        eprintln!("skipping gnuplottex test; pdflatex or gnuplottex.sty is unavailable");
        return;
    }

    let _guard = GNUPLOTTEX_TEST_LOCK
        .lock()
        .expect("gnuplottex test lock poisoned");
    let root = unique_temp_dir("tekai-gnuplottex-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let points = root.join("points.dat");
    let out_dir = root.join("out");
    let fake_bin = root.join("bin");
    fs::create_dir_all(&fake_bin).expect("failed to create fake bin directory");

    fs::write(&main, GNUPLOTTEX_DOC).expect("failed to write gnuplottex document");
    fs::write(&points, "0 0\n1 1\n").expect("failed to write gnuplot data");
    write_fake_gnuplot(&fake_bin.join("gnuplot"));
    let _path_guard = PathGuard::prepend(&fake_bin);

    let first = build(&options(&main, &out_dir)).expect("initial gnuplottex build failed");
    assert_eq!(first.external_runs, 1, "{first:#?}");
    assert_eq!(first.tex_runs, 2, "{first:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main-gnuplottex-fig1.gnuplot").exists());
    assert!(out_dir.join("main-gnuplottex-fig1.tex").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("failed to read TeX log");
    let compact_log = log.split_whitespace().collect::<String>();
    assert!(compact_log.contains("main-gnuplottex-fig1.tex"), "{log}");
    assert_eq!(fake_invocations(&out_dir), 1);

    let cached = build(&options(&main, &out_dir)).expect("cached gnuplottex build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.external_runs, 0, "{cached:#?}");
    assert_eq!(fake_invocations(&out_dir), 1);

    fs::write(&points, "0 0\n1 2\n").expect("failed to update gnuplot data");
    let data_edit = build(&options(&main, &out_dir)).expect("data edit gnuplottex build failed");
    assert_eq!(data_edit.external_runs, 1, "{data_edit:#?}");
    assert_eq!(fake_invocations(&out_dir), 2);

    fs::write(&main, GNUPLOTTEX_DOC.replace("sample", "updated"))
        .expect("failed to update gnuplottex source");
    let edited = build(&options(&main, &out_dir)).expect("edited gnuplottex build failed");
    assert_eq!(edited.external_runs, 1, "{edited:#?}");
    assert_eq!(edited.tex_runs, 1, "{edited:#?}");
    assert_eq!(fake_invocations(&out_dir), 3);

    let _ = fs::remove_dir_all(root);
}

#[cfg(unix)]
fn write_fake_gnuplot(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(
        path,
        r#"#!/bin/sh
set -eu
script="${1:?missing script}"
outfile="$(sed -n "s/^set output ['\"]\(.*\)['\"].*/\1/p" "$script" | head -n 1)"
if [ -z "$outfile" ]; then
  echo "missing set output" >&2
  exit 2
fi
case "$outfile" in
  /*) target="$outfile" ;;
  *) target="$PWD/$outfile" ;;
esac
mkdir -p "$(dirname "$target")"
printf 'Fake gnuplot figure from %s\n' "$(basename "$script")" > "$target"
datafile="$(sed -n "s/^plot ['\"]\([^'\"]*\)['\"].*/\1/p" "$script" | head -n 1)"
if [ -n "$datafile" ]; then
  case "$datafile" in
    /*) datapath="$datafile" ;;
    *) datapath="$PWD/$datafile" ;;
  esac
  if [ -f "$datapath" ]; then
    cat "$datapath" >> "$target"
  fi
fi
printf '%s\n' "$*" >> "$PWD/gnuplot.invocations"
"#,
    )
    .expect("failed to write fake gnuplot");
    let mut permissions = fs::metadata(path)
        .expect("failed to stat fake gnuplot")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("failed to chmod fake gnuplot");
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
    fs::read_to_string(out_dir.join("gnuplot.invocations"))
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

use std::fs;
use std::path::{Path, PathBuf};

use tekai::compiler::{
    BibMode, BuildOptions, DraftPrepass, Engine, Runner, build, build_dependency_paths,
};

const DATA_DOC: &str = r#"\documentclass{article}
\begin{document}
\input{values.dat}
\end{document}
"#;
const DATA_SOURCE: &str = "Recorded data dependency.\n";

#[test]
fn direct_build_state_tracks_nonstandard_source_dependencies() {
    if !command_available("pdflatex") {
        eprintln!("skipping dependency test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-dependency-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let data = root.join("values.dat");
    let out_dir = root.join("out");
    fs::write(&main, DATA_DOC).expect("failed to write test document");
    fs::write(&data, DATA_SOURCE).expect("failed to write data file");

    let options = options(&main, &out_dir);
    let first = build(&options).expect("initial build failed");
    assert!(!first.skipped, "{first:#?}");

    let dependency_paths = build_dependency_paths(&options).expect("failed to read dependencies");
    let data_path = data
        .canonicalize()
        .expect("failed to canonicalize data file");
    assert!(
        dependency_paths.iter().any(|path| path == &data_path),
        "{dependency_paths:#?}"
    );

    let cached = build(&options).expect("cached build failed");
    assert!(cached.skipped, "{cached:#?}");

    rewrite_same_contents_with_different_mtime(&data, DATA_SOURCE);
    let touched = build(&options).expect("touched dependency build failed");
    assert!(touched.skipped, "{touched:#?}");
    assert_eq!(touched.tex_runs, 0, "{touched:#?}");

    fs::write(&data, "Changed recorded data dependency.\n").expect("failed to update data file");
    let changed = build(&options).expect("changed dependency build failed");
    assert!(!changed.skipped, "{changed:#?}");
    assert_eq!(changed.tex_runs, 1, "{changed:#?}");

    let _ = fs::remove_dir_all(root);
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

fn rewrite_same_contents_with_different_mtime(path: &Path, contents: &str) {
    let before = modified_ns(path);
    for _ in 0..5 {
        std::thread::sleep(std::time::Duration::from_millis(20));
        fs::write(path, contents).expect("failed to rewrite dependency");
        if modified_ns(path) != before {
            return;
        }
    }
    panic!("failed to produce a metadata-only dependency change");
}

fn modified_ns(path: &Path) -> u128 {
    fs::metadata(path)
        .expect("failed to stat dependency")
        .modified()
        .expect("failed to read dependency mtime")
        .duration_since(std::time::UNIX_EPOCH)
        .expect("dependency mtime before UNIX epoch")
        .as_nanos()
}

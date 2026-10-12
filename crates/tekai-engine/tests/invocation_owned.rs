//! Exercise the public entry in one process, not through CLI subprocesses.

use lopdf::{dictionary, Document, Object, Stream};
use proptest::prelude::*;
use proptest::test_runner::{contextualize_config, TestRunner};
use std::cell::Cell;
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use tekai_engine::{EngineCompletion, InvocationError, InvocationOptions};

struct Project(PathBuf, Option<PathBuf>);

impl Project {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        for _ in 0..128 {
            let path = std::env::temp_dir().join(format!(
                "tekai-owned-invocation-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed),
            ));
            match fs::create_dir(&path) {
                Ok(()) => {
                    let project = Self(path, None);
                    fs::create_dir(project.0.join("empty-bin")).unwrap();
                    fs::write(
                        project.0.join("good.tex"),
                        concat!(
                            "\\documentclass{article}\n",
                            "\\usepackage{graphicx}\n",
                            "\\begin{document}\n",
                            "Owned invocation. \\includegraphics[width=10pt]{image.png} ",
                            "\\includegraphics[width=10pt]{image.pdf}\n",
                            "\\typeout{OWNED-DATE=\\the\\year/\\the\\month/\\the\\day}\n",
                            "\\end{document}\n",
                        ),
                    )
                    .unwrap();
                    fs::write(
                        project.0.join("bad.tex"),
                        concat!(
                            "\\documentclass{article}\n\\usepackage{graphicx}\n\\begin{document}\n",
                            "\\includegraphics{image.png}\\undefinedOwnedInvocation\n",
                            "\\end{document}\n",
                        ),
                    )
                    .unwrap();
                    project.write_images();
                    return project;
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("cannot create {}: {error}", path.display()),
            }
        }
        panic!("cannot reserve owned invocation fixture")
    }

    fn with_engine_cache(cache: &Path) -> Self {
        assert!(cache.is_absolute());
        let mut project = Self::new();
        project.1 = Some(cache.to_owned());
        project
    }

    fn write_images(&self) {
        let file = fs::File::create(self.0.join("image.png")).unwrap();
        let mut encoder = png::Encoder::new(file, 1, 1);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .write_header()
            .unwrap()
            .write_image_data(&[20, 80, 160])
            .unwrap();

        let mut doc = Document::with_version("1.5");
        let pages = doc.new_object_id();
        let content = doc.add_object(Stream::new(
            dictionary! {},
            b"0 0 1 rg 0 0 20 20 re f\n".to_vec(),
        ));
        let page = doc.add_object(dictionary! {
            "Type" => "Page", "Parent" => pages,
            "MediaBox" => vec![0.into(), 0.into(), 20.into(), 20.into()],
            "Resources" => dictionary! {}, "Contents" => content,
        });
        doc.objects.insert(
            pages,
            Object::Dictionary(dictionary! {
                "Type" => "Pages", "Kids" => vec![page.into()], "Count" => 1,
            }),
        );
        let catalog = doc.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages });
        doc.trailer.set("Root", catalog);
        doc.save(self.0.join("image.pdf")).unwrap();
    }

    fn options(&self, epoch: &str) -> InvocationOptions {
        let overrides = [
            ("PATH", self.0.join("empty-bin").into_os_string()),
            ("TEKAI_TEXMF_MODE", OsString::from("bundled")),
            // Relative roots must follow the invocation directory, not the host.
            (
                "TEKAI_ENGINE_CACHE",
                self.1
                    .as_ref()
                    .map(|cache| cache.as_os_str().to_owned())
                    .unwrap_or_else(|| OsString::from("cache/engine")),
            ),
            ("TEXMFDIST", self.0.join("missing-texlive").into_os_string()),
            ("TEXMFVAR", self.0.join("missing-texlive").into_os_string()),
            (
                "TEXMFCONFIG",
                self.0.join("missing-texlive").into_os_string(),
            ),
            ("SOURCE_DATE_EPOCH", OsString::from(epoch)),
            ("FORCE_SOURCE_DATE", OsString::from("1")),
            ("TZ", OsString::from("UTC0")),
        ];
        InvocationOptions {
            working_directory: Some(self.0.clone()),
            environment: overrides
                .into_iter()
                .map(|(key, value)| (key.into(), Some(value)))
                .chain([("TEXINPUTS".into(), None), ("TEXFORMATS".into(), None)])
                .collect(),
            ..InvocationOptions::default()
        }
    }

    fn run(&self, args: &[&str], epoch: &str) -> EngineCompletion {
        let arguments = std::iter::once("pdflatex")
            .chain(args.iter().copied())
            .map(|arg| arg.as_bytes().to_vec())
            .collect();
        // SAFETY All inputs and formats in this fixture are trusted. Shell
        // commands are disabled and no internal engine interfaces are called.
        unsafe { tekai_engine::run_with_options(arguments, self.options(epoch)) }.unwrap()
    }

    fn compile(&self, name: &str, epoch: &str) -> EngineCompletion {
        self.run(
            &[
                "-fmt=pdflatex",
                "-interaction=nonstopmode",
                "-halt-on-error",
                "-no-shell-escape",
                "-recorder",
                "-synctex=1",
                name,
            ],
            epoch,
        )
    }

    fn creation_date(&self) -> Vec<u8> {
        self.creation_date_for("good")
    }

    fn creation_date_for(&self, job: &str) -> Vec<u8> {
        let doc = Document::load(self.0.join(format!("{job}.pdf"))).unwrap();
        assert_eq!(doc.get_pages().len(), 1);
        let info = doc.trailer.get(b"Info").unwrap().as_reference().unwrap();
        doc.get_object(info)
            .unwrap()
            .as_dict()
            .unwrap()
            .get(b"CreationDate")
            .unwrap()
            .as_str()
            .unwrap()
            .to_vec()
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        // This fixture exclusively created the directory. Never remove a
        // parent directory or a cache belonging to another invocation.
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CacheEntry {
    path: PathBuf,
    directory: bool,
    length: u64,
    modified: (i64, i64),
    change_identity: (u64, u64, i64, i64),
}

fn cache_inventory(root: &Path) -> Vec<CacheEntry> {
    let mut pending = vec![root.to_owned()];
    let mut entries = Vec::new();
    while let Some(path) = pending.pop() {
        let metadata = fs::symlink_metadata(&path).unwrap();
        assert!(metadata.is_file() || metadata.is_dir());
        if metadata.is_dir() {
            pending.extend(
                fs::read_dir(&path)
                    .unwrap()
                    .map(|entry| entry.unwrap().path()),
            );
        }
        entries.push(CacheEntry {
            path: path.strip_prefix(root).unwrap().to_owned(),
            directory: metadata.is_dir(),
            length: metadata.len(),
            modified: (metadata.mtime(), metadata.mtime_nsec()),
            change_identity: (
                metadata.dev(),
                metadata.ino(),
                metadata.ctime(),
                metadata.ctime_nsec(),
            ),
        });
    }
    entries.sort_unstable_by(|left, right| left.path.cmp(&right.path));
    entries
}

#[test]
fn completion_failures_and_changed_configuration_leave_clean_reusable_entries() {
    let parent_directory = std::env::current_dir().unwrap();
    let parent_epoch = std::env::var_os("SOURCE_DATE_EPOCH");
    let parent_cache = std::env::var_os("TEKAI_ENGINE_CACHE");
    let first = Project::new();
    let second = Project::new();
    assert_eq!(first.run(&["--help"], "0"), EngineCompletion::Clean);
    assert_eq!(first.run(&["--version"], "0"), EngineCompletion::Clean);
    assert_eq!(
        first.run(&["--unknown-owned-option"], "0"),
        EngineCompletion::Errors
    );
    assert_eq!(first.compile("bad.tex", "0"), EngineCompletion::Errors);
    assert_eq!(first.compile("good.tex", "0"), EngineCompletion::Clean);
    let first_pdf = fs::read(first.0.join("good.pdf")).unwrap();
    assert_eq!(first.creation_date(), b"D:19700101000000Z");
    assert!(fs::read_to_string(first.0.join("good.log"))
        .unwrap()
        .contains("OWNED-DATE=1970/1/1"));
    assert!(first.0.join("good.synctex.gz").is_file());
    assert_eq!(
        second.compile("good.tex", "946684800"),
        EngineCompletion::Clean
    );
    assert_eq!(second.creation_date(), b"D:20000101000000Z");
    assert!(fs::read_to_string(second.0.join("good.log"))
        .unwrap()
        .contains("OWNED-DATE=2000/1/1"));
    assert_eq!(first.compile("good.tex", "0"), EngineCompletion::Clean);
    assert_eq!(fs::read(first.0.join("good.pdf")).unwrap(), first_pdf);
    for project in [&first, &second] {
        assert!(project.0.join("cache/engine").is_dir());
        let recorder = fs::read_to_string(project.0.join("good.fls")).unwrap();
        assert!(recorder.contains(&format!("PWD {}", project.0.display())));
        let other = if project.0 == first.0 {
            &second
        } else {
            &first
        };
        assert!(!recorder.contains(other.0.to_string_lossy().as_ref()));
    }
    assert_eq!(std::env::current_dir().unwrap(), parent_directory);
    assert_eq!(std::env::var_os("SOURCE_DATE_EPOCH"), parent_epoch);
    assert_eq!(std::env::var_os("TEKAI_ENGINE_CACHE"), parent_cache);
}

#[test]
fn validation_and_setup_failure_do_not_poison_the_next_entry() {
    assert_eq!(
        unsafe { tekai_engine::run_from_args(Vec::new()) },
        Err(InvocationError::MissingProgramName)
    );
    assert!(matches!(
        unsafe { tekai_engine::run_from_args(vec![b"bad\0name".to_vec()]) },
        Err(InvocationError::ArgumentContainsNul { .. })
    ));
    let project = Project::new();
    let mut options = project.options("0");
    options
        .environment
        .push(("bad=name".into(), Some("value".into())));
    assert!(matches!(
        unsafe { tekai_engine::run_with_options(vec![b"pdflatex".to_vec()], options) },
        Err(InvocationError::ResourceSetup {
            resource: "environment override",
            ..
        })
    ));
    let mut options = project.options("0");
    options.working_directory = Some(project.0.join("not-a-directory"));
    assert!(matches!(
        unsafe { tekai_engine::run_with_options(vec![b"pdflatex".to_vec()], options) },
        Err(InvocationError::ResourceSetup {
            resource: "working directory",
            ..
        })
    ));
    assert_eq!(project.compile("good.tex", "0"), EngineCompletion::Clean);
}

#[test]
fn independent_threads_compile_with_separate_directories_dates_and_handles() {
    let start = std::sync::Arc::new(std::sync::Barrier::new(2));
    let threads = ["0", "946684800"].map(|epoch| {
        let start = start.clone();
        std::thread::spawn(move || {
            let project = Project::new();
            start.wait();
            for _ in 0..3 {
                assert_eq!(project.compile("good.tex", epoch), EngineCompletion::Clean);
                assert_eq!(
                    project.creation_date(),
                    if epoch == "0" {
                        b"D:19700101000000Z".as_slice()
                    } else {
                        b"D:20000101000000Z".as_slice()
                    }
                );
            }
        })
    });
    for thread in threads {
        thread.join().unwrap();
    }
}

#[test]
fn concurrent_distinct_jobs_share_readonly_inputs_without_recorder_collisions() {
    let project = std::sync::Arc::new(Project::new());
    // Prepare immutable bundle/format caches before the concurrent workload.
    assert_eq!(project.compile("good.tex", "0"), EngineCompletion::Clean);
    let start = std::sync::Arc::new(std::sync::Barrier::new(2));
    let threads = [("left", "0"), ("right", "946684800")].map(|(job, epoch)| {
        let project = project.clone();
        let start = start.clone();
        std::thread::spawn(move || {
            start.wait();
            for _ in 0..2 {
                assert_eq!(
                    project.run(
                        &[
                            "-fmt=pdflatex",
                            "-interaction=nonstopmode",
                            "-halt-on-error",
                            "-no-shell-escape",
                            "-recorder",
                            "-synctex=1",
                            &format!("-jobname={job}"),
                            "good.tex"
                        ],
                        epoch
                    ),
                    EngineCompletion::Clean
                );
                assert_eq!(
                    project.creation_date_for(job),
                    if epoch == "0" {
                        b"D:19700101000000Z".as_slice()
                    } else {
                        b"D:20000101000000Z".as_slice()
                    }
                );
                let recorder = fs::read_to_string(project.0.join(format!("{job}.fls"))).unwrap();
                assert!(
                    recorder.contains(&format!("OUTPUT {job}.pdf")),
                    "{recorder}"
                );
                let other = if job == "left" { "right" } else { "left" };
                assert!(
                    !recorder.contains(&format!("OUTPUT {other}.pdf")),
                    "{recorder}"
                );
            }
        })
    });
    for thread in threads {
        thread.join().unwrap();
    }
}

#[test]
fn arbitrary_exit_sequences_do_not_change_a_later_success() {
    // Prepare published, immutable bundle/outline/format files once. The owner
    // is local to this property run. Every case still creates a new project,
    // environment and invocation, and the fixed isolation tests keep separate
    // relative cache roots. No TeX state or resource owner is shared here.
    let prepared = Project::new();
    assert_eq!(prepared.compile("good.tex", "0"), EngineCompletion::Clean);
    let cache = prepared.0.join("cache/engine").canonicalize().unwrap();
    let installed = cache_inventory(&cache);
    let bundles = installed
        .iter()
        .filter(|entry| entry.path.file_name() == Some(std::ffi::OsStr::new(".complete")))
        .count();
    let formats = installed
        .iter()
        .filter(|entry| entry.path.to_string_lossy().ends_with(".fmt.raw"))
        .count();
    assert_eq!(bundles, 1);
    assert_eq!(formats, 1);

    // Match proptest!'s function wrapper, including environment overrides,
    // deterministic seeds and source-file failure persistence. In particular,
    // PROPTEST_CASES still overrides the small local default in CI.
    let mut config = contextualize_config(ProptestConfig::with_cases(12));
    config.source_file = Some(file!());
    config.test_name = Some(concat!(
        module_path!(),
        "::arbitrary_exit_sequences_do_not_change_a_later_success"
    ));
    let cases = config.cases;
    let completed = Cell::new(0usize);
    let mut runner = TestRunner::new(config);
    runner
        .run(&prop::collection::vec(0u8..4, 1..8), |operations| {
            let project = Project::with_engine_cache(&cache);
            for operation in operations {
                let result = match operation {
                    0 => project.run(&["--help"], "0"),
                    1 => project.run(&["--version"], "0"),
                    2 => project.compile("bad.tex", "0"),
                    _ => project.run(
                        &[
                            "-fmt=pdflatex",
                            "-interaction=nonstopmode",
                            "-halt-on-error",
                            "missing.tex",
                        ],
                        "0",
                    ),
                };
                prop_assert_eq!(result.exit_code(), if operation < 2 { 0 } else { 1 });
            }
            prop_assert_eq!(project.compile("good.tex", "0"), EngineCompletion::Clean);
            prop_assert_eq!(project.creation_date(), b"D:19700101000000Z");
            prop_assert!(!project.0.join("cache").exists());
            completed.set(completed.get() + 1);
            Ok(())
        })
        .unwrap_or_else(|error| panic!("{error}\n{runner}"));
    // Atomic republication or in-place changes would alter this inventory,
    // including native inode/change-time identities. Reads do not change it.
    assert_eq!(cache_inventory(&cache), installed);
    eprintln!(
        "exit-sequence property requested {cases} cases, observed {} successful PDFs in this process, reused {bundles} bundle and {formats} format with {} unchanged cache entries",
        completed.get(),
        installed.len(),
    );
}

//! Bundled TeX data and native file lookup. Shared addition trees are read
//! without subprocesses; distribution data is pinned when the binary is built.

use std::cell::{Cell, RefCell};
use std::fs;
use std::io;
use std::marker::{PhantomData, PhantomPinned};
use std::path::{Component, Path, PathBuf};
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub const BUNDLE_ID: &str = include_str!("../../../runtime/bundle-id.txt");
pub const FORMAT_ID: &str = include_str!("../../../formats/format-id.txt");
pub use crate::time::LocalPdfDateFormatter;
/// Owns one cache-root selection and its lazy installation result.
pub(crate) struct RuntimeSession {
    cache_root: PathBuf,
    root: RefCell<Option<Result<PathBuf, String>>>,
    _thread: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

thread_local! {
    static ACTIVE: Cell<Option<NonNull<RuntimeSession>>> = const { Cell::new(None) };
}

impl RuntimeSession {
    pub(crate) fn new() -> Pin<Box<Self>> {
        Box::pin(Self {
            cache_root: cache_root(),
            root: RefCell::new(None),
            _thread: PhantomData,
            _pinned: PhantomPinned,
        })
    }

    pub(crate) fn scope<T>(session: Pin<&mut Self>, action: impl FnOnce() -> T) -> T {
        struct Unregister {
            pointer: NonNull<RuntimeSession>,
            previous: Option<NonNull<RuntimeSession>>,
        }
        impl Drop for Unregister {
            fn drop(&mut self) {
                let _ = ACTIVE.try_with(|active| {
                    if active.get() == Some(self.pointer) {
                        active.set(self.previous);
                    }
                });
            }
        }
        // SAFETY The private guard stays inside this call, and the exclusive
        // pinned borrow keeps the owner alive until action and guard return.
        let pointer = NonNull::from(unsafe { session.get_unchecked_mut() });
        let previous = ACTIVE.with(|active| {
            if let Some(previous) = active.get() {
                assert!(
                    unsafe { previous.as_ref() }.root.try_borrow_mut().is_ok(),
                    "recursive runtime cache scope"
                );
            }
            active.replace(Some(pointer))
        });
        let _unregister = Unregister { pointer, previous };
        action()
    }
}

pub fn texmf_root() -> io::Result<PathBuf> {
    crate::search::mode()?;
    ACTIVE.with(|active| {
        let Some(pointer) = active.get() else {
            // Utility calls have no invocation owner. The on-disk completion
            // receipt stays fast, but no previous environment root is retained.
            return install_bundle(&cache_root());
        };
        // SAFETY Only the private closure scope registers this pinned owner.
        // Root storage uses RefCell and never exposes a borrowed path.
        let session = unsafe { pointer.as_ref() };
        let mut root = session
            .root
            .try_borrow_mut()
            .map_err(|_| io::Error::other("recursive runtime installation"))?;
        root.get_or_insert_with(|| {
            install_bundle(&session.cache_root).map_err(|error| error.to_string())
        })
        .as_ref()
        .cloned()
        .map_err(|error| io::Error::other(error.clone()))
    })
}

fn cache_root() -> PathBuf {
    if let Some(path) = crate::environment::var_os("TEKAI_ENGINE_CACHE") {
        return PathBuf::from(path);
    }
    if cfg!(target_os = "macos") {
        if let Some(home) = crate::environment::var_os("HOME") {
            return PathBuf::from(home).join("Library/Caches/tekai/engine");
        }
    }
    if let Some(path) = crate::environment::var_os("XDG_CACHE_HOME") {
        return PathBuf::from(path).join("tekai/engine");
    }
    if let Some(home) = crate::environment::var_os("HOME") {
        return PathBuf::from(home).join(".cache/tekai/engine");
    }
    std::env::temp_dir().join("tekai/engine")
}

pub(crate) fn selected_cache_root() -> PathBuf {
    ACTIVE.with(|active| {
        active
            .get()
            .map(|pointer| {
                // SAFETY Private scope retains the pinned owner's immutable root.
                unsafe { pointer.as_ref() }.cache_root.clone()
            })
            .unwrap_or_else(cache_root)
    })
}

fn install_bundle(cache: &Path) -> io::Result<PathBuf> {
    let cache = crate::working_directory::resolve(cache)?;
    fs::create_dir_all(&cache)?;
    // Absolute paths keep lookups valid after a child changes its working dir.
    let cache = cache.canonicalize()?;
    let destination = cache.join(format!("texmf-{}", BUNDLE_ID.trim()));
    if bundle_is_complete(&destination) {
        return Ok(destination.join("texmf-dist"));
    }
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let staging = cache.join(format!(".texmf-{}-{nonce}", std::process::id()));
    fs::create_dir(&staging)?;
    let result = (|| {
        unpack_bundle(crate::embedded::texmf_bytes(), &staging)?;
        unpack_bundle(crate::embedded::font_outlines_bytes(), &staging)?;
        fs::write(staging.join(".complete"), BUNDLE_ID)?;
        match fs::rename(&staging, &destination) {
            Ok(()) => {}
            Err(_) if bundle_is_complete(&destination) => {
                // A concurrent build finished installing the same bundle.
            }
            Err(error) => return Err(error),
        }
        Ok(destination.join("texmf-dist"))
    })();
    // Only our newly created staging directory is removed, never user data.
    if staging.exists() {
        let _ = fs::remove_dir_all(&staging);
    }
    result
}

fn bundle_is_complete(root: &Path) -> bool {
    fs::read_to_string(root.join(".complete")).is_ok_and(|id| id == BUNDLE_ID)
        && root.join("texmf-dist/ls-R").is_file()
}

fn unpack_bundle(bytes: &[u8], destination: &Path) -> io::Result<()> {
    let decoder = flate2::read::GzDecoder::new(bytes);
    // Tar reads headers and padding in tiny pieces. Buffer decoded bytes so
    // these reads do not each re-enter the inflater. Keep the outer reader
    // through the final drain, including buffered bytes and gzip validation.
    let mut archive = tar::Archive::new(io::BufReader::with_capacity(64 * 1024, decoder));
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if !entry.header().entry_type().is_file()
            || path
                .components()
                .any(|part| !matches!(part, Component::Normal(_)))
        {
            return Err(io::Error::other("unsafe file in embedded TeX bundle"));
        }
        if !entry.unpack_in(destination)? {
            return Err(io::Error::other("invalid path in embedded TeX bundle"));
        }
    }
    // tar stops at its end marker before gzip necessarily checks its footer.
    io::copy(&mut archive.into_inner(), &mut io::sink())?;
    Ok(())
}

pub(crate) fn find_in_path(base: &Path, candidate: &str, entry: &Path) -> Option<PathBuf> {
    crate::lookup::find(base, candidate, entry)
}

pub(crate) fn installed_bundle_root() -> Option<PathBuf> {
    // Installing lazily on the first classification/lookup keeps compiler
    // policy owners consistent even if preparation occurred outside a scope.
    ACTIVE
        .with(|active| active.get().is_some())
        .then(texmf_root)?
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::io::Write;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct UnpackFixture(PathBuf);

    impl UnpackFixture {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let root = std::env::temp_dir().join(format!(
                "tekai-unpack-buffer-{}-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                NEXT.fetch_add(1, Ordering::Relaxed),
            ));
            fs::create_dir(&root).unwrap();
            Self(root)
        }
    }

    impl Drop for UnpackFixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn compressed_files(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut archive = tar::Builder::new(Vec::new());
        for &(name, bytes) in files {
            let mut header = tar::Header::new_gnu();
            header.set_size(bytes.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            archive.append_data(&mut header, name, bytes).unwrap();
        }
        let tar = archive.into_inner().unwrap();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        encoder.write_all(&tar).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn decoded_buffer_preserves_nested_files_across_header_and_buffer_boundaries() {
        let fixture = UnpackFixture::new();
        let sizes = [
            0, 1, 2, 3, 7, 511, 512, 513, 8191, 8192, 8193, 65535, 65536, 65537,
        ];
        let files: Vec<_> = sizes
            .into_iter()
            .enumerate()
            .map(|(index, size)| {
                let name = format!("texmf-dist/nested{index}/file{index}");
                let bytes: Vec<_> = (0..size)
                    .map(|offset| (offset * 37 + index * 19) as u8)
                    .collect();
                (name, bytes)
            })
            .collect();
        let borrowed: Vec<_> = files
            .iter()
            .map(|(name, bytes)| (name.as_str(), bytes.as_slice()))
            .collect();
        unpack_bundle(&compressed_files(&borrowed), &fixture.0).unwrap();
        for (name, bytes) in files {
            assert_eq!(fs::read(fixture.0.join(name)).unwrap(), bytes);
        }
    }

    proptest! {
        #[test]
        fn decoded_buffer_preserves_arbitrary_regular_file_bytes(payload in prop::collection::vec(any::<u8>(), 0..4097)) {
            let fixture = UnpackFixture::new();
            let files = [("texmf-dist/empty", &[][..]), ("texmf-dist/nested/payload", payload.as_slice()),
                         ("texmf-dist/last", &b"last"[..])];
            unpack_bundle(&compressed_files(&files), &fixture.0).unwrap();
            for (name, bytes) in files {
                prop_assert_eq!(fs::read(fixture.0.join(name)).unwrap(), bytes);
            }
        }
    }

    #[test]
    fn scoped_roots_restore_after_nesting_and_unwind_without_retaining_paths() {
        use std::panic::{catch_unwind, AssertUnwindSafe};
        let mut first = RuntimeSession::new();
        let mut second = RuntimeSession::new();
        let first_path = PathBuf::from("/first/texmf-dist");
        let second_path = PathBuf::from("/second/texmf-dist");
        first.root.replace(Some(Ok(first_path.clone())));
        second.root.replace(Some(Ok(second_path.clone())));
        let owned = RuntimeSession::scope(first.as_mut(), || {
            assert_eq!(texmf_root().unwrap(), first_path);
            let outcome = catch_unwind(AssertUnwindSafe(|| {
                RuntimeSession::scope(second.as_mut(), || {
                    assert_eq!(installed_bundle_root(), Some(second_path.clone()));
                    panic!("partial invocation");
                });
            }));
            assert!(outcome.is_err());
            assert_eq!(installed_bundle_root(), Some(first_path.clone()));
            texmf_root().unwrap()
        });
        assert!(ACTIVE.with(Cell::get).is_none());
        drop(first);
        drop(second);
        assert_eq!(owned, first_path);
        assert_eq!(installed_bundle_root(), None);
    }

    #[test]
    fn selected_cache_root_is_consistent_with_format_cache_within_an_invocation() {
        let mut environment = crate::environment::EnvironmentSession::new().unwrap();
        crate::environment::EnvironmentSession::scope(environment.as_mut(), || {
            crate::environment::set(
                std::ffi::OsStr::new("TEKAI_ENGINE_CACHE"),
                std::ffi::OsStr::new("first-cache"),
            )
            .unwrap();
            let mut owner = RuntimeSession::new();
            RuntimeSession::scope(owner.as_mut(), || {
                crate::environment::set(
                    std::ffi::OsStr::new("TEKAI_ENGINE_CACHE"),
                    std::ffi::OsStr::new("second-cache"),
                )
                .unwrap();
                assert_eq!(selected_cache_root(), Path::new("first-cache"));
                assert_eq!(
                    crate::kpathsea::engine_cache_root(),
                    Path::new("first-cache")
                );
            });
            assert_eq!(selected_cache_root(), Path::new("second-cache"));
        })
        .unwrap();
    }

    #[test]
    fn explicit_paths_support_recursive_search_and_suffixes() {
        let root = std::env::temp_dir().join(format!("tekai-native-search-{}", std::process::id()));
        let child = root.join("tree/deep/tex");
        fs::create_dir_all(&child).unwrap();
        let root = root.canonicalize().unwrap();
        let child = child.canonicalize().unwrap();
        fs::write(child.join("shared.sty"), "test").unwrap();
        assert_eq!(
            find_in_path(&root, "shared.sty", Path::new("tree//tex")),
            Some(child.join("shared.sty"))
        );
        assert_eq!(
            find_in_path(&root, "shared.sty", Path::new("!!tree//")),
            None
        );
        fs::write(root.join("tree/ls-R"), "./deep/tex:\nshared.sty\n").unwrap();
        assert_eq!(
            find_in_path(&root, "shared.sty", Path::new("!!tree//")),
            Some(child.join("shared.sty"))
        );
        assert_eq!(
            find_in_path(&root, "shared.sty", Path::new("!!tree//tex//")),
            Some(child.join("shared.sty"))
        );
        assert_eq!(
            find_in_path(&root, "shared.sty", Path::new("tree//tex//")),
            Some(child.join("shared.sty"))
        );
        assert_eq!(find_in_path(&root, "shared.sty", Path::new("tree")), None);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn corrupt_gzip_footer_is_rejected() {
        let root =
            std::env::temp_dir().join(format!("tekai-corrupt-bundle-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&[0; 1024]).unwrap();
        let bytes = encoder.finish().unwrap();
        for offset in [8, 4] {
            let mut corrupt = bytes.clone();
            let index = corrupt.len() - offset;
            corrupt[index] ^= 1;
            assert!(unpack_bundle(&corrupt, &root).is_err());
        }
        assert!(unpack_bundle(&bytes[..bytes.len() - 1], &root).is_err());
        fs::remove_dir(root).unwrap();
    }

    #[test]
    fn bundle_contains_packages_fonts_and_notices() {
        let root = texmf_root().unwrap();
        assert!(root.join("tex/latex/base/article.cls").is_file());
        assert!(root.join("fonts/tfm/public/cm/cmr10.tfm").is_file());
        assert!(root.parent().unwrap().join("packages.lock.json").is_file());
        assert!(root.join("source/latex/base").is_dir());
    }
}

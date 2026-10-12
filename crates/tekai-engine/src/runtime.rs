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
    let mut archive = tar::Archive::new(decoder);
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
        use std::io::Write;
        let root =
            std::env::temp_dir().join(format!("tekai-corrupt-bundle-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&[0; 1024]).unwrap();
        let mut bytes = encoder.finish().unwrap();
        let crc_offset = bytes.len() - 8;
        bytes[crc_offset] ^= 1;
        assert!(unpack_bundle(&bytes, &root).is_err());
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

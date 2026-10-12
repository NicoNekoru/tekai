//! An invocation's directory without process-wide chdir.

#![warn(clippy::all)]

use std::cell::Cell;
use std::io;
use std::marker::{PhantomData, PhantomPinned};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::ptr;
use std::rc::Rc;

thread_local! {
    static ACTIVE: Cell<*const WorkingDirectory> = const { Cell::new(ptr::null()) };
}

pub(crate) struct WorkingDirectory {
    path: PathBuf,
    _thread_confined: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

impl WorkingDirectory {
    pub(crate) fn new(path: Option<PathBuf>) -> io::Result<Pin<Box<Self>>> {
        let path = match path {
            Some(path) if path.is_absolute() => path,
            Some(path) => std::env::current_dir()?.join(path),
            None => std::env::current_dir()?,
        };
        if !std::fs::metadata(&path)?.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                "engine working directory is not a directory",
            ));
        }
        Ok(Box::pin(Self {
            path,
            _thread_confined: PhantomData,
            _pinned: PhantomPinned,
        }))
    }

    pub(crate) fn scope<T>(self: Pin<&mut Self>, operation: impl FnOnce() -> T) -> io::Result<T> {
        struct Guard;
        impl Drop for Guard {
            fn drop(&mut self) {
                ACTIVE.with(|active| active.set(ptr::null()));
            }
        }
        ACTIVE.with(|active| {
            if !active.get().is_null() {
                return Err(io::Error::other("nested engine working directory scope"));
            }
            active.set(self.as_ref().get_ref());
            Ok(())
        })?;
        let _guard = Guard;
        Ok(operation())
    }
}

pub(crate) fn current() -> io::Result<PathBuf> {
    ACTIVE.with(|active| {
        let pointer = active.get();
        if pointer.is_null() {
            return std::env::current_dir();
        }
        // SAFETY: the private guard keeps the pinned immutable path alive and
        // unregisters it before the owner may be dropped, including on unwind.
        Ok(unsafe { &*pointer }.path.clone())
    })
}

pub(crate) fn resolve(path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(current()?.join(path))
    }
}

pub(crate) fn with_resolved<T>(
    path: &Path,
    operation: impl FnOnce(&Path) -> io::Result<T>,
) -> io::Result<T> {
    if path.is_absolute() {
        operation(path)
    } else {
        operation(&current()?.join(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn successive_directories_do_not_change_parent_or_leak_between_scopes() {
        let parent = std::env::current_dir().unwrap();
        let alternate = std::env::temp_dir();
        for directory in [&alternate, &parent, &alternate] {
            let mut owner = WorkingDirectory::new(Some(directory.clone())).unwrap();
            owner
                .as_mut()
                .scope(|| {
                    assert_eq!(current().unwrap(), *directory);
                    assert_eq!(
                        resolve(Path::new("input.tex")).unwrap(),
                        directory.join("input.tex")
                    );
                    assert_eq!(resolve(&parent).unwrap(), parent);
                    assert_eq!(std::env::current_dir().unwrap(), parent);
                })
                .unwrap();
            assert_eq!(current().unwrap(), parent);
        }
    }

    #[test]
    fn rejected_nesting_and_unwind_preserve_routing() {
        let parent = std::env::current_dir().unwrap();
        let mut owner = WorkingDirectory::new(None).unwrap();
        let mut other = WorkingDirectory::new(Some(std::env::temp_dir())).unwrap();
        assert!(catch_unwind(AssertUnwindSafe(|| {
            owner
                .as_mut()
                .scope(|| {
                    assert!(other.as_mut().scope(|| unreachable!()).is_err());
                    assert_eq!(current().unwrap(), parent);
                    panic!("exercise invocation unwind");
                })
                .unwrap();
        }))
        .is_err());
        assert_eq!(current().unwrap(), parent);
        other
            .as_mut()
            .scope(|| assert_eq!(current().unwrap(), std::env::temp_dir()))
            .unwrap();
    }

    #[test]
    fn concurrent_threads_have_independent_directory_snapshots() {
        let parent = std::env::current_dir().unwrap();
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let threads = [parent.clone(), std::env::temp_dir()].map(|path| {
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                let mut owner = WorkingDirectory::new(Some(path.clone())).unwrap();
                owner
                    .as_mut()
                    .scope(|| {
                        barrier.wait();
                        assert_eq!(current().unwrap(), path);
                        barrier.wait();
                        assert_eq!(resolve(Path::new("child")).unwrap(), path.join("child"));
                    })
                    .unwrap();
            })
        });
        for thread in threads {
            thread.join().unwrap();
        }
        assert_eq!(std::env::current_dir().unwrap(), parent);
    }
}

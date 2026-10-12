//! Owned TeXview IPC connection and bounded, dynamically sized protocol messages.

#![warn(clippy::all)]

use std::cell::{Cell, RefCell};
use std::io::{self, Write};
use std::marker::{PhantomData, PhantomPinned};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::ptr;
use std::rc::Rc;

thread_local! {
    static ACTIVE: Cell<*const IpcSession> = const { Cell::new(ptr::null()) };
}

#[derive(Default)]
struct State {
    stream: Option<UnixStream>,
    begun: bool,
}

pub(crate) struct IpcSession {
    state: RefCell<State>,
    _thread_confined: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

impl IpcSession {
    pub(crate) fn new() -> Pin<Box<Self>> {
        Box::pin(Self {
            state: RefCell::new(State::default()),
            _thread_confined: PhantomData,
            _pinned: PhantomPinned,
        })
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
                return Err(io::Error::other("nested engine IPC scope"));
            }
            active.set(self.as_ref().get_ref());
            Ok(())
        })?;
        let _guard = Guard;
        Ok(operation())
    }
}

fn with_state<T>(operation: impl FnOnce(&mut State) -> T) -> T {
    ACTIVE.with(|active| {
        let pointer = active.get();
        assert!(
            !pointer.is_null(),
            "engine IPC outside its invocation scope"
        );
        // SAFETY: only the private closure scope installs this pinned owner.
        // RefCell confines the mutable borrow to this operation and rejects
        // recursive access before creating any overlapping mutable reference.
        let session = unsafe { &*pointer };
        operation(&mut session.state.borrow_mut())
    })
}

fn connect(state: &mut State, path: &Path) {
    if state.stream.is_some() {
        return;
    }
    if let Ok(stream) = UnixStream::connect(path) {
        if stream.set_nonblocking(true).is_ok() {
            state.stream = Some(stream);
        }
    }
}

pub(crate) fn open() {
    let Some(directory) = crate::environment::var_os("HOME") else {
        return;
    };
    let path = PathBuf::from(directory).join(".TeXview_Pipe");
    if let Ok(path) = crate::working_directory::resolve(&path) {
        with_state(|state| connect(state, &path));
    }
}

pub(crate) fn is_open() -> bool {
    with_state(|state| state.stream.is_some())
}

pub(crate) fn wants_name() -> bool {
    with_state(|state| !state.begun)
}

fn packet(name: &[u8], eof: i32) -> io::Result<Vec<u8>> {
    let count =
        i32::try_from(name.len()).map_err(|_| io::Error::other("IPC filename is too long"))?;
    let capacity = name
        .len()
        .checked_add(8)
        .ok_or_else(|| io::Error::other("IPC message size overflow"))?;
    let mut packet = Vec::new();
    packet
        .try_reserve_exact(capacity)
        .map_err(io::Error::other)?;
    // TeXview's local protocol uses two native-endian 32-bit integer fields.
    // The filename has no terminator in the transmitted byte count.
    packet.extend_from_slice(&count.to_ne_bytes());
    packet.extend_from_slice(&eof.to_ne_bytes());
    packet.extend_from_slice(name);
    Ok(packet)
}

pub(crate) fn page(name: Option<&Path>, eof: i32) {
    if wants_name() {
        open();
    }
    with_state(|state| {
        let name = if state.begun { None } else { name };
        state.begun = true;
        let Some(stream) = state.stream.as_mut() else {
            return;
        };
        let Ok(packet) = packet(name.map_or(&[], |path| path.as_os_str().as_bytes()), eof) else {
            state.stream = None;
            return;
        };
        // Preserve the legacy short-write/error policy. The optional preview
        // connection is discarded rather than blocking the typesetter.
        if stream.write(&packet).ok() != Some(packet.len()) {
            state.stream = None;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::io::Read;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    proptest! {
        #[test]
        fn protocol_preserves_long_byte_names_without_a_fixed_stack_buffer(
            name in proptest::collection::vec(1u8..=255, 0..8192), eof in any::<i32>(),
        ) {
            let bytes = packet(&name, eof).unwrap();
            prop_assert_eq!(bytes.len(), name.len() + 8);
            prop_assert_eq!(&bytes[..4], &(name.len() as i32).to_ne_bytes());
            prop_assert_eq!(&bytes[4..8], &eof.to_ne_bytes());
            prop_assert_eq!(&bytes[8..], name.as_slice());
        }
    }

    #[test]
    fn successful_pages_send_the_name_once_and_owner_drop_closes_connection() {
        let (writer, mut reader) = UnixStream::pair().unwrap();
        let mut owner = IpcSession::new();
        owner.as_ref().state.borrow_mut().stream = Some(writer);
        owner
            .as_mut()
            .scope(|| {
                assert!(wants_name());
                page(Some(Path::new("/project/output.dvi")), 0);
                assert!(!wants_name());
                page(Some(Path::new("ignored-name")), 1);
            })
            .unwrap();
        drop(owner);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).unwrap();
        let mut expected = packet(b"/project/output.dvi", 0).unwrap();
        expected.extend(packet(b"", 1).unwrap());
        assert_eq!(bytes, expected);
        let mut next = IpcSession::new();
        next.as_mut()
            .scope(|| {
                assert!(wants_name());
                assert!(!is_open());
            })
            .unwrap();
    }

    #[test]
    fn unwind_unregisters_and_releases_live_socket_without_finalization() {
        let (writer, reader) = UnixStream::pair().unwrap();
        let mut owner = IpcSession::new();
        owner.as_ref().state.borrow_mut().stream = Some(writer);
        assert!(catch_unwind(AssertUnwindSafe(|| {
            owner
                .as_mut()
                .scope(|| panic!("partial engine failure"))
                .unwrap();
        }))
        .is_err());
        assert!(ACTIVE.with(|active| active.get().is_null()));
        drop(owner);
        let mut reader = reader;
        let mut byte = [0];
        assert_eq!(reader.read(&mut byte).unwrap(), 0);
    }
}

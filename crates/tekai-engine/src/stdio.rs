//! Owned Rust streams for the translated engine's remaining pointer-based IO.
//!
//! Files, console handles and pipe endpoints use Rust IO. No libc FILE object
//! enters this module. A pinned invocation session owns streams and console
//! buffers. The thread-local routing pointer borrows it only within its scope.

#![warn(clippy::all)]

use flate2::{read::MultiGzDecoder, write::GzEncoder, Compression};
use rustix::process::{waitid, Pid, Signal, WaitId, WaitIdOptions};
use std::cell::{Cell, RefCell, UnsafeCell};
use std::ffi::{c_char, c_int, c_long, c_void, CStr, OsStr};
use std::fs::{File, Metadata, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::marker::{PhantomData, PhantomPinned};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::pin::Pin;
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

const EOF: c_int = -1;
const BUFFER_SIZE: usize = 8192;
const MAX_BUFFER_SIZE: usize = 64 * 1024;
// Native ungetc only guarantees one byte. Current engine callers need one;
// eight leaves a small explicit allowance without unbounded retained storage.
const MAX_PUSHBACK: usize = 8;

/// Ownership of pipe commands launched during an engine invocation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PipeProcessPolicy {
    /// Isolate each command in a group owned by this invocation. Abandonment
    /// stops the group before reaping its leader. Host callers must not reap
    /// these children independently or ignore SIGCHLD.
    #[default]
    InvocationOwned,
    /// Inherit the host process group and its existing terminal job control.
    /// This invocation owns and reaps only each direct shell child. The host
    /// accepts responsibility for descendants and cancellation of those jobs.
    CallerManagedProcess,
    /// Inherit the engine worker's process group. An external supervisor must
    /// own that isolated worker group and stop its descendants before reaping
    /// the worker, including cancellation and worker faults. Local abandonment
    /// stops and reaps only the direct pipe child, never the inherited group.
    SupervisedProcess,
}

enum Backing {
    File(File),
    Stdin(io::Stdin),
    Stdout(io::Stdout),
    Stderr(io::Stderr),
    ReadPipe(PipeChild, ChildStdout),
    WritePipe(PipeChild, ChildStdin),
    GzipRead(MultiGzDecoder<File>),
    GzipWrite(File, Compression),
    GzipEncoder(GzEncoder<File>),
    Closed,
    #[cfg(test)]
    TestReader(Box<dyn Read>),
    #[cfg(test)]
    TestWriter(Box<dyn Write>),
}

/// Own a pipe command and, in InvocationOwned mode, its process group until its
/// direct child is reaped. The host must not reap it or ignore SIGCHLD.
struct PipeChild {
    child: Child,
    group: Option<Pid>,
    owned: bool,
    monitor: Option<ChildMonitor>,
}

#[derive(Clone, Copy)]
enum InheritedIo {
    ReadPipe,
    WritePipe,
    All,
}

impl InheritedIo {
    fn has_controlling_terminal(self) -> io::Result<bool> {
        // stderr is inherited in both pipe modes. TOSTOP can stop a command
        // writing it even when neither of its other endpoints is a terminal.
        for (inherited, fd) in [
            (
                matches!(self, Self::ReadPipe | Self::All),
                rustix::stdio::stdin(),
            ),
            (
                matches!(self, Self::WritePipe | Self::All),
                rustix::stdio::stdout(),
            ),
            (true, rustix::stdio::stderr()),
        ] {
            if inherited && controlling_terminal(fd)? {
                return Ok(true);
            }
        }
        // The session's controlling terminal survives redirected standard
        // descriptors. A command may reopen it through /dev/tty. This probe
        // reads metadata only and cannot acquire or transfer terminal control.
        let terminal = loop {
            match OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_NOCTTY)
                .open("/dev/tty")
            {
                Ok(terminal) => break terminal,
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) if error.raw_os_error() == Some(libc::ENXIO) => return Ok(false),
                Err(error) => return Err(error),
            }
        };
        controlling_terminal(&terminal)
    }
}

fn controlling_terminal(fd: impl rustix::fd::AsFd) -> io::Result<bool> {
    loop {
        match rustix::termios::tcgetpgrp(fd.as_fd()) {
            Ok(_) => return Ok(true),
            Err(rustix::io::Errno::NOTTY | rustix::io::Errno::BADF) => return Ok(false),
            Err(rustix::io::Errno::INTR) => continue,
            Err(error) => return Err(error.into()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PrivateTerminalStop(i32);

impl std::fmt::Display for PrivateTerminalStop {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let signal = match self.0 {
            libc::SIGTTIN => "SIGTTIN",
            libc::SIGTTOU => "SIGTTOU",
            _ => unreachable!("only terminal-stop signals are admitted"),
        };
        write!(formatter, "private engine command stopped by {signal} while inheriting a controlling terminal. Use CallerManagedProcess when the host owns terminal job control.")
    }
}

impl std::error::Error for PrivateTerminalStop {}

struct ChildObservation {
    // The PipeChild's unreaped Child reserves this PID until after observer
    // join. Only this mutex-protected authority may signal its owned group.
    authority: Option<Pid>,
    terminal_stop: Option<PrivateTerminalStop>,
    error: Option<i32>,
    reported: bool,
}

struct ChildMonitor {
    state: Arc<Mutex<ChildObservation>>,
    thread: Option<JoinHandle<()>>,
}

fn observation_lock(state: &Mutex<ChildObservation>) -> MutexGuard<'_, ChildObservation> {
    state
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn stop_observed_child(observation: &mut ChildObservation) {
    if let Some(child) = observation.authority {
        if let Err(error) = crate::process_group::signal_owned_group(child, Signal::KILL) {
            observation
                .error
                .get_or_insert(error.raw_os_error().unwrap_or(libc::EIO));
        }
        // This is the exact unreaped direct child, not a listed descendant.
        // It may have changed its own group while its original group vanished.
        loop {
            match rustix::process::kill_process(child, Signal::KILL) {
                Ok(()) | Err(rustix::io::Errno::SRCH) => break,
                Err(rustix::io::Errno::INTR) => continue,
                Err(error) => {
                    observation.error.get_or_insert(error.raw_os_error());
                    break;
                }
            }
        }
    }
}

fn observe_child(group: Pid, state: Arc<Mutex<ChildObservation>>) {
    let mut options = WaitIdOptions::EXITED | WaitIdOptions::STOPPED | WaitIdOptions::NOWAIT;
    loop {
        match waitid(WaitId::Pid(group), options) {
            Ok(Some(status)) if status.stopped() => {
                if let Some(signal @ (libc::SIGTTIN | libc::SIGTTOU)) = status.stopping_signal() {
                    let mut observation = observation_lock(&state);
                    observation.terminal_stop = Some(PrivateTerminalStop(signal));
                    stop_observed_child(&mut observation);
                    // The group kill wakes blocked pipe reads/writes. Keep the
                    // reservation until exit is observed and the owner joins.
                    options = WaitIdOptions::EXITED | WaitIdOptions::NOWAIT;
                } else {
                    // Consume only the observed stop, never an exit. Otherwise
                    // NOWAIT repeats the same event indefinitely and misses a
                    // terminal stop after the host legitimately resumes it.
                    loop {
                        match waitid(
                            WaitId::Pid(group),
                            WaitIdOptions::STOPPED | WaitIdOptions::NOHANG,
                        ) {
                            Err(rustix::io::Errno::INTR) => continue,
                            Err(rustix::io::Errno::CHILD) => {
                                let mut observation = observation_lock(&state);
                                observation.authority = None;
                                observation.error = Some(libc::ECHILD);
                                return;
                            }
                            Err(error) => {
                                let mut observation = observation_lock(&state);
                                observation.error = Some(error.raw_os_error());
                                stop_observed_child(&mut observation);
                                return;
                            }
                            Ok(_) => break,
                        }
                    }
                }
            }
            Ok(Some(_)) => return,
            Err(rustix::io::Errno::INTR) => continue,
            result => {
                let mut observation = observation_lock(&state);
                let error = result.err().map_or(libc::EIO, |error| error.raw_os_error());
                observation.error = Some(error);
                if error == libc::ECHILD {
                    // An external reaper ended the reservation. Never signal
                    // that numerical ID after learning that ownership was lost.
                    observation.authority = None;
                } else {
                    stop_observed_child(&mut observation);
                }
                return;
            }
        }
    }
}

impl ChildMonitor {
    fn start(group: Pid) -> io::Result<Self> {
        let state = Arc::new(Mutex::new(ChildObservation {
            authority: Some(group),
            terminal_stop: None,
            error: None,
            reported: false,
        }));
        let observer_state = Arc::clone(&state);
        let thread = thread::Builder::new()
            .name("tekai-pipe-status".into())
            .spawn(move || observe_child(group, observer_state))?;
        Ok(Self {
            state,
            thread: Some(thread),
        })
    }

    fn join(&mut self) -> io::Result<()> {
        self.thread.take().map_or(Ok(()), |thread| {
            thread
                .join()
                .map_err(|_| io::Error::other("engine child status observer panicked"))
        })
    }

    fn failure(&self) -> Option<io::Error> {
        let mut observation = observation_lock(&self.state);
        if let Some(stop) = observation.terminal_stop {
            if !observation.reported {
                observation.reported = true;
                let _ = writeln!(io::stderr().lock(), "tekai engine IO: {stop}");
            }
            // A genuine signalling/query error remains an OS error. Reporting
            // the terminal stop must not turn uncertain cleanup into success
            // or suppress EPERM from the strict owned-group classifier.
            Some(
                observation
                    .error
                    .map_or_else(|| io::Error::other(stop), io::Error::from_raw_os_error),
            )
        } else {
            observation.error.map(io::Error::from_raw_os_error)
        }
    }
}

impl PipeChild {
    fn spawn(
        command: &mut Command,
        policy: PipeProcessPolicy,
        inherited: InheritedIo,
    ) -> io::Result<Self> {
        let monitored =
            policy == PipeProcessPolicy::InvocationOwned && inherited.has_controlling_terminal()?;
        if policy == PipeProcessPolicy::InvocationOwned {
            command.process_group(0);
        }
        let child = command.spawn()?;
        let raw = i32::try_from(child.id()).expect("Unix child PID fits in i32");
        let group = Pid::from_raw(raw)
            .filter(|pid| *pid != Pid::INIT)
            .expect("spawned child has a positive non-init PID");
        let mut owner = Self {
            child,
            group: (policy == PipeProcessPolicy::InvocationOwned).then_some(group),
            owned: true,
            monitor: None,
        };
        // Own the child before fallible thread setup, so a setup error kills
        // and reaps instead of losing it through Child's non-killing Drop.
        if monitored {
            owner.monitor = Some(ChildMonitor::start(group)?);
        }
        Ok(owner)
    }

    fn wait(mut self, terminate: bool) -> io::Result<ExitStatus> {
        if terminate {
            self.terminate_and_reap()
        } else {
            // Explicit pclose preserves POSIX shell/background semantics. A
            // successful normal wait relinquishes ownership without signalling
            // descendants deliberately left behind by the shell command.
            self.reap()
        }
    }

    fn terminate_and_reap(&mut self) -> io::Result<ExitStatus> {
        // The unreaped leader protects its process/group ID from reuse. Stop
        // descendants before releasing that authority, even if it has exited.
        let group_error = if let Some(monitor) = &self.monitor {
            let observation = observation_lock(&monitor.state);
            observation.authority.and_then(|group| {
                crate::process_group::signal_owned_group(group, Signal::KILL).err()
            })
        } else {
            self.group.and_then(|group| {
                crate::process_group::signal_owned_group(group, Signal::KILL).err()
            })
        };
        // Also stop the direct child if the command changed its own group.
        // Child::kill accepts a child that has already exited.
        let child_error = if let Some(monitor) = &self.monitor {
            let observation = observation_lock(&monitor.state);
            observation.authority.and_then(|_| self.child.kill().err())
        } else {
            self.child.kill().err()
        };
        let status = self.reap();
        // Always attempt the reap, including after a signalling error. A
        // remaining owner retries cleanup from Drop after other wait failures.
        if let Some(error) = group_error.or(child_error) {
            Err(error)
        } else {
            status
        }
    }

    fn reap(&mut self) -> io::Result<ExitStatus> {
        let mut observer_error = None;
        if let Some(monitor) = &mut self.monitor {
            if let Err(error) = monitor.join() {
                // No observer survives join. Stop a still-owned child before
                // waiting even when the observer unexpectedly panicked.
                let observation = observation_lock(&monitor.state);
                if let Some(group) = observation.authority {
                    let _ = crate::process_group::signal_owned_group(group, Signal::KILL);
                    let _ = self.child.kill();
                }
                observer_error = Some(error);
            }
        }
        let status = loop {
            match self.child.wait() {
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                result => break result,
            }
        };
        if status.is_ok()
            || status.as_ref().is_err_and(|error| {
                error.raw_os_error() == Some(rustix::io::Errno::CHILD.raw_os_error())
            })
        {
            self.owned = false;
            if let Some(monitor) = &self.monitor {
                observation_lock(&monitor.state).authority = None;
            }
        }
        match observer_error.or_else(|| self.io_error()) {
            Some(error) => Err(error),
            None => status,
        }
    }

    fn io_error(&self) -> Option<io::Error> {
        self.monitor.as_ref().and_then(ChildMonitor::failure)
    }

    #[cfg(test)]
    fn id(&self) -> u32 {
        self.child.id()
    }
}

impl Drop for PipeChild {
    fn drop(&mut self) {
        if self.owned {
            let _ = self.terminate_and_reap();
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Buffering {
    Full,
    Line,
    None,
}

pub struct Stream {
    backing: Backing,
    readable: bool,
    writable: bool,
    append: bool,
    owner: *const StreamSession,
    buffering: Buffering,
    buffer_size: usize,
    input: Vec<u8>,
    input_at: usize,
    input_end: usize,
    pushback: Vec<u8>,
    output: Vec<u8>,
    position: u64,
    eof: bool,
    error: bool,
}

thread_local! {
    static ACTIVE_SESSION: Cell<*const StreamSession> = const { Cell::new(ptr::null()) };
    // Low-level unit tests exercise adapters without entering the engine. This
    // test-only owner is never available to a production invocation.
    #[cfg(test)]
    static TEST_SESSION: Pin<Box<StreamSession>> = StreamSession::new();
}

struct Console {
    input: UnsafeCell<Stream>,
    output: UnsafeCell<Stream>,
    error: UnsafeCell<Stream>,
}

/// Own every stream, including read-ahead and console state, for one invocation.
/// Typed raw entries own Box::into_raw allocations without moving or mutably
/// reborrowing a Box while its FILE aliases are live. Pinning protects console
/// addresses and the owner pointers held by streams.
pub(crate) struct StreamSession {
    streams: RefCell<Vec<NonNull<Stream>>>,
    console: Console,
    pipe_policy: PipeProcessPolicy,
    next_temp: Cell<u64>,
    temp_nonce: u128,
    _thread_confined: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

impl StreamSession {
    pub(crate) fn new() -> Pin<Box<Self>> {
        Self::new_with_pipe_policy(PipeProcessPolicy::default())
    }

    pub(crate) fn new_with_pipe_policy(policy: PipeProcessPolicy) -> Pin<Box<Self>> {
        Box::pin(Self {
            streams: RefCell::new(Vec::new()),
            console: Console {
                input: UnsafeCell::new(Stream::new(
                    Backing::Stdin(io::stdin()),
                    true,
                    false,
                    false,
                    Buffering::Full,
                )),
                output: UnsafeCell::new(Stream::new(
                    Backing::Stdout(io::stdout()),
                    false,
                    true,
                    false,
                    Buffering::Line,
                )),
                error: UnsafeCell::new(Stream::new(
                    Backing::Stderr(io::stderr()),
                    false,
                    true,
                    false,
                    Buffering::None,
                )),
            },
            pipe_policy: policy,
            next_temp: Cell::new(0),
            temp_nonce: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |elapsed| elapsed.as_nanos()),
            _thread_confined: PhantomData,
            _pinned: PhantomPinned,
        })
    }

    pub(crate) fn scope<T>(self: Pin<&mut Self>, operation: impl FnOnce() -> T) -> T {
        struct RoutingGuard;
        impl Drop for RoutingGuard {
            fn drop(&mut self) {
                ACTIVE_SESSION.with(|active| active.set(ptr::null()));
            }
        }
        ACTIVE_SESSION.with(|active| {
            assert!(active.get().is_null(), "nested engine stream session");
            active.set(self.as_ref().get_ref());
        });
        let _routing = RoutingGuard;
        operation()
    }

    /// Flush and close residual files and reap residual pipe children. Explicit
    /// pclose still flushes and waits for the child's normal status. Teardown
    /// discards abandoned pipe output and reaps its direct child. The selected
    /// policy determines whether this owner or the external worker supervisor
    /// stops the command's descendants.
    pub(crate) fn finish(self: Pin<&mut Self>) -> io::Result<()> {
        let session = self.as_ref().get_ref();
        let mut first_error = None;
        for pointer in session.streams.borrow_mut().drain(..) {
            // SAFETY each registry entry exclusively owns one into_raw Box.
            // Teardown runs outside routing, after all FILE accesses ended.
            let mut stream = unsafe { Box::from_raw(pointer.as_ptr()) };
            if let Err(error) = stream.close_for_teardown() {
                first_error.get_or_insert(error);
            }
        }
        for stream in [session.console.output.get(), session.console.error.get()] {
            // SAFETY: the session is pinned, thread-confined and outside its
            // routing scope. No callback retains a mutable Stream reference.
            if let Err(error) = unsafe { &mut *stream }.flush_output() {
                first_error.get_or_insert(error);
            }
        }
        first_error.map_or(Ok(()), Err)
    }
}

impl Drop for StreamSession {
    fn drop(&mut self) {
        // SAFETY self is being destroyed in place. No scope can still borrow
        // it, and finish drains every owner exactly once, including on unwind.
        let _ = unsafe { Pin::new_unchecked(self) }.finish();
    }
}

fn with_session<T>(operation: impl FnOnce(&StreamSession) -> T) -> T {
    ACTIVE_SESSION.with(|active| {
        let pointer = active.get();
        if !pointer.is_null() {
            // SAFETY: scope keeps the pinned owner alive, excludes nesting and
            // clears this thread's routing pointer even during unwinding.
            return operation(unsafe { &*pointer });
        }
        #[cfg(test)]
        return TEST_SESSION.with(|session| operation(session.as_ref().get_ref()));
        #[cfg(not(test))]
        panic!("engine IO outside its invocation session");
    })
}

// Closing a console consumes its invocation-local state, not the process-global
// OS descriptor. A subsequent getter does not reopen it within that invocation.

pub(crate) fn stdin() -> *mut Stream {
    with_session(|session| session.console.input.get())
}

pub(crate) fn stdout() -> *mut Stream {
    with_session(|session| session.console.output.get())
}

pub(crate) fn stderr() -> *mut Stream {
    with_session(|session| session.console.error.get())
}

fn set_errno(code: c_int) {
    #[cfg(target_os = "macos")]
    unsafe {
        *libc::__error() = code;
    }
    #[cfg(target_os = "linux")]
    unsafe {
        *libc::__errno_location() = code;
    }
}

pub(crate) fn clear_errno() {
    set_errno(0);
}

pub(crate) fn errno() -> c_int {
    io::Error::last_os_error().raw_os_error().unwrap_or(0)
}

fn io_error(code: c_int) -> io::Error {
    io::Error::from_raw_os_error(code)
}

impl Stream {
    fn new(
        backing: Backing,
        readable: bool,
        writable: bool,
        append: bool,
        buffering: Buffering,
    ) -> Self {
        Self {
            backing,
            readable,
            writable,
            append,
            owner: ptr::null(),
            buffering,
            buffer_size: BUFFER_SIZE,
            input: Vec::new(),
            input_at: 0,
            input_end: 0,
            pushback: Vec::new(),
            output: Vec::new(),
            position: 0,
            eof: false,
            error: false,
        }
    }

    fn into_handle(mut self) -> *mut Self {
        with_session(|session| {
            self.owner = session;
            let mut streams = session.streams.borrow_mut();
            // Reserve before conversion, so no panicking Vec growth can leave
            // the newly transferred typed allocation without an owner.
            streams.reserve(1);
            let pointer = Box::into_raw(Box::new(self));
            streams.push(NonNull::new(pointer).expect("Box allocation is non-null"));
            pointer
        })
    }

    fn record_error(&mut self, error: io::Error) {
        self.error = true;
        set_errno(error.raw_os_error().unwrap_or(libc::EIO));
    }

    fn read_backing(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        loop {
            let result = match &mut self.backing {
                Backing::File(file) => file.read(buffer),
                Backing::Stdin(input) => input.read(buffer),
                Backing::ReadPipe(child, output) => {
                    let result = output.read(buffer);
                    if !matches!(result, Ok(count) if count > 0) {
                        child.io_error().map_or(result, Err)
                    } else {
                        result
                    }
                }
                Backing::GzipRead(decoder) => decoder.read(buffer),
                #[cfg(test)]
                Backing::TestReader(reader) => reader.read(buffer),
                _ => Err(io_error(libc::EBADF)),
            };
            if !matches!(&result, Err(error) if error.kind() == io::ErrorKind::Interrupted) {
                return result;
            }
        }
    }

    fn write_backing(&mut self, buffer: &[u8]) -> io::Result<usize> {
        if matches!(self.backing, Backing::GzipWrite(..)) {
            let Backing::GzipWrite(file, compression) =
                std::mem::replace(&mut self.backing, Backing::Closed)
            else {
                unreachable!()
            };
            self.backing = Backing::GzipEncoder(GzEncoder::new(file, compression));
        }
        loop {
            let result = match &mut self.backing {
                Backing::File(file) => file.write(buffer),
                Backing::Stdout(output) => output.write(buffer),
                Backing::Stderr(error) => error.write(buffer),
                Backing::WritePipe(child, input) => {
                    let result = input.write(buffer);
                    if !matches!(result, Ok(count) if count > 0) {
                        child.io_error().map_or(result, Err)
                    } else {
                        result
                    }
                }
                Backing::GzipEncoder(encoder) => encoder.write(buffer),
                #[cfg(test)]
                Backing::TestWriter(writer) => writer.write(buffer),
                _ => Err(io_error(libc::EBADF)),
            };
            if !matches!(&result, Err(error) if error.kind() == io::ErrorKind::Interrupted) {
                return result;
            }
        }
    }

    fn drain_output(&mut self) -> io::Result<()> {
        let mut pending = std::mem::take(&mut self.output);
        let mut written = 0;
        while written < pending.len() {
            match self.write_backing(&pending[written..]) {
                Ok(0) => {
                    pending.drain(..written);
                    self.output = pending;
                    return Err(io::Error::from(io::ErrorKind::WriteZero));
                }
                Ok(count) => written += count,
                Err(error) => {
                    pending.drain(..written);
                    self.output = pending;
                    return Err(error);
                }
            }
        }
        pending.clear();
        self.output = pending;
        Ok(())
    }

    fn flush_backing(&mut self) -> io::Result<()> {
        match &mut self.backing {
            Backing::File(file) => file.flush(),
            Backing::Stdout(output) => output.flush(),
            Backing::Stderr(error) => error.flush(),
            Backing::WritePipe(_, input) => input.flush(),
            Backing::GzipEncoder(encoder) => encoder.flush(),
            #[cfg(test)]
            Backing::TestWriter(writer) => writer.flush(),
            Backing::Closed => Err(io_error(libc::EBADF)),
            _ => Ok(()),
        }
    }

    fn flush_output(&mut self) -> io::Result<()> {
        self.drain_output()?;
        self.flush_backing()
    }

    fn flush_buffer(&mut self) -> io::Result<()> {
        self.drain_output()?;
        // Full gzip buffers only drain. A codec flush would insert a deflate
        // sync marker at each chunk boundary and waste work and compressed
        // space. Console wrappers may retain their own buffers, so full, line
        // and unbuffered console drains still reach the OS explicitly.
        if self.buffering != Buffering::Full
            || matches!(self.backing, Backing::Stdout(_) | Backing::Stderr(_))
        {
            self.flush_backing()?;
        }
        Ok(())
    }

    fn sync_input(&mut self) -> io::Result<()> {
        // Seekable input flush discards both read-ahead and pushback at the
        // owned logical position. This is deliberate even where Darwin's
        // native backup-buffer flush reports an unrelated offset instead.
        if self.input_at != self.input_end || !self.pushback.is_empty() {
            match &mut self.backing {
                Backing::File(file) => {
                    file.seek(SeekFrom::Start(self.position))?;
                }
                _ => return Err(io_error(libc::ESPIPE)),
            }
        }
        self.input_at = 0;
        self.input_end = 0;
        self.pushback.clear();
        Ok(())
    }

    fn read_bytes(&mut self, output: &mut [u8]) -> usize {
        if output.is_empty() {
            return 0;
        }
        if !self.readable {
            self.record_error(io_error(libc::EBADF));
            return 0;
        }
        if self.eof && self.pushback.is_empty() {
            return 0;
        }
        if let Err(error) = self.flush_output() {
            self.record_error(error);
            return 0;
        }
        let mut copied = 0;
        while copied < output.len() {
            if let Some(byte) = self.pushback.pop() {
                output[copied] = byte;
                copied += 1;
                self.position = self.position.saturating_add(1);
                continue;
            }
            if self.input_at == self.input_end {
                let size = if self.buffering == Buffering::None {
                    1
                } else {
                    self.buffer_size
                };
                let mut input = std::mem::take(&mut self.input);
                input.resize(size, 0);
                let read = self.read_backing(&mut input);
                self.input = input;
                self.input_at = 0;
                self.input_end = 0;
                match read {
                    Ok(0) => {
                        self.eof = true;
                        break;
                    }
                    Ok(count) => self.input_end = count,
                    Err(error) => {
                        self.record_error(error);
                        break;
                    }
                }
            }
            let count = (self.input_end - self.input_at).min(output.len() - copied);
            output[copied..copied + count]
                .copy_from_slice(&self.input[self.input_at..self.input_at + count]);
            self.input_at += count;
            copied += count;
            self.position = self.position.saturating_add(count as u64);
        }
        copied
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> usize {
        if bytes.is_empty() {
            return 0;
        }
        if !self.writable {
            self.record_error(io_error(libc::EBADF));
            return 0;
        }
        if let Err(error) = self.sync_input() {
            self.record_error(error);
            return 0;
        }
        if self.append && self.output.is_empty() {
            if let Backing::File(file) = &self.backing {
                if let Ok(metadata) = file.metadata() {
                    self.position = metadata.len();
                }
            }
        }
        let mut accepted = 0;
        while accepted < bytes.len() {
            // A previous failed flush may retain a full buffer. Retry it
            // before accepting more bytes, keeping retained memory bounded.
            if self.output.len() >= self.buffer_size {
                if let Err(error) = self.flush_buffer() {
                    self.record_error(error);
                    break;
                }
            }
            let remaining = &bytes[accepted..];
            let mut count = remaining
                .len()
                .min(self.buffer_size.saturating_sub(self.output.len()).max(1));
            if self.buffering == Buffering::Line {
                if let Some(newline) = remaining[..count].iter().position(|byte| *byte == b'\n') {
                    count = newline + 1;
                }
            }
            self.output.extend_from_slice(&remaining[..count]);
            accepted += count;
            self.position = self.position.saturating_add(count as u64);
            if self.buffering == Buffering::None
                || self.output.len() >= self.buffer_size
                || self.buffering == Buffering::Line && remaining[count - 1] == b'\n'
            {
                if let Err(error) = self.flush_buffer() {
                    self.record_error(error);
                    // Unwritten bytes from an earlier call precede this
                    // chunk. Count only this chunk's unwritten suffix.
                    accepted -= self.output.len().min(count);
                    break;
                }
            }
        }
        accepted
    }

    fn seek(&mut self, offset: i64, origin: c_int) -> io::Result<()> {
        self.flush_output()?;
        let target = match origin {
            libc::SEEK_SET => {
                SeekFrom::Start(u64::try_from(offset).map_err(|_| io_error(libc::EINVAL))?)
            }
            libc::SEEK_CUR => {
                let position = self
                    .position
                    .checked_add_signed(offset)
                    .ok_or_else(|| io_error(libc::EINVAL))?;
                SeekFrom::Start(position)
            }
            libc::SEEK_END => SeekFrom::End(offset),
            _ => return Err(io_error(libc::EINVAL)),
        };
        let Backing::File(file) = &mut self.backing else {
            return Err(io_error(libc::ESPIPE));
        };
        self.position = file.seek(target)?;
        self.input_at = 0;
        self.input_end = 0;
        self.pushback.clear();
        self.eof = false;
        Ok(())
    }

    pub(crate) fn metadata(&self) -> io::Result<Metadata> {
        match &self.backing {
            Backing::File(file) | Backing::GzipWrite(file, _) => file.metadata(),
            Backing::GzipRead(decoder) => decoder.get_ref().metadata(),
            Backing::GzipEncoder(encoder) => encoder.get_ref().metadata(),
            _ => Err(io_error(libc::EBADF)),
        }
    }

    fn close(&mut self) -> io::Result<c_int> {
        self.close_impl(false)
    }

    fn close_for_teardown(&mut self) -> io::Result<c_int> {
        self.close_impl(true)
    }

    fn close_impl(&mut self, terminate_pipe: bool) -> io::Result<c_int> {
        let flushed = if terminate_pipe && matches!(self.backing, Backing::WritePipe(..)) {
            // Abandoned output must not block on a child that never reads.
            // Explicit fflush/fclose/pclose retain their normal flush rules.
            Ok(())
        } else if matches!(
            self.backing,
            Backing::GzipWrite(..) | Backing::GzipEncoder(_)
        ) {
            // finish emits the trailer without a redundant sync-flush first.
            self.drain_output()
        } else {
            self.flush_output()
        };
        let backing = std::mem::replace(&mut self.backing, Backing::Closed);
        self.readable = false;
        self.writable = false;
        self.input_at = 0;
        self.input_end = 0;
        self.pushback.clear();
        self.output.clear();
        let status = match backing {
            Backing::ReadPipe(child, output) => {
                drop(output);
                child.wait(terminate_pipe)?.into_raw()
            }
            Backing::WritePipe(child, input) => {
                drop(input);
                child.wait(terminate_pipe)?.into_raw()
            }
            Backing::GzipEncoder(encoder) => {
                encoder.finish()?.flush()?;
                0
            }
            Backing::GzipWrite(file, compression) => {
                GzEncoder::new(file, compression).finish()?.flush()?;
                0
            }
            _ => 0,
        };
        flushed?;
        Ok(status)
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        if !matches!(self.backing, Backing::Closed) {
            let _ = self.close_for_teardown();
        }
    }
}

/// The raw adapters require a live stream on the calling engine thread. A close
/// consumes owned handles exactly once. Caller buffers must cover their stated
/// lengths and remain exclusively borrowed during the operation.
pub(crate) unsafe fn fopen(name: *const c_char, mode: *const c_char) -> *mut Stream {
    let name = unsafe { OsStr::from_bytes(CStr::from_ptr(name).to_bytes()) };
    let mode = unsafe { CStr::from_ptr(mode).to_bytes() };
    let Some(&kind) = mode.first() else {
        set_errno(libc::EINVAL);
        return ptr::null_mut();
    };
    if !matches!(kind, b'r' | b'w' | b'a')
        || mode[1..]
            .iter()
            .any(|byte| !matches!(byte, b'b' | b'+' | b'e' | b'x'))
    {
        set_errno(libc::EINVAL);
        return ptr::null_mut();
    }
    let read = kind == b'r' || mode.contains(&b'+');
    let write = kind != b'r' || mode.contains(&b'+');
    let append = kind == b'a';
    let mut options = OpenOptions::new();
    options
        .read(read)
        .write(write && !append)
        .append(append)
        .create(kind != b'r')
        .truncate(kind == b'w');
    if mode.contains(&b'x') {
        options.create_new(true);
    }
    match crate::working_directory::with_resolved(std::path::Path::new(name), |path| {
        options.open(path)
    }) {
        Ok(mut file) => {
            let position = if append && !read {
                file.seek(SeekFrom::End(0)).unwrap_or(0)
            } else {
                0
            };
            let mut stream = Stream::new(Backing::File(file), read, write, append, Buffering::Full);
            stream.position = position;
            stream.into_handle()
        }
        Err(error) => {
            set_errno(error.raw_os_error().unwrap_or(libc::EIO));
            ptr::null_mut()
        }
    }
}

pub(crate) unsafe fn fread(
    buffer: *mut c_void,
    size: usize,
    count: usize,
    stream: *mut Stream,
) -> usize {
    if size == 0 || count == 0 {
        return 0;
    }
    let Some(stream) = (unsafe { stream.as_mut() }) else {
        set_errno(libc::EBADF);
        return 0;
    };
    let Some(bytes) = size
        .checked_mul(count)
        .filter(|total| *total <= isize::MAX as usize)
    else {
        stream.record_error(io_error(libc::EOVERFLOW));
        return 0;
    };
    stream.read_bytes(unsafe { std::slice::from_raw_parts_mut(buffer.cast(), bytes) }) / size
}

pub(crate) unsafe fn fwrite(
    buffer: *const c_void,
    size: usize,
    count: usize,
    stream: *mut Stream,
) -> usize {
    if size == 0 || count == 0 {
        return 0;
    }
    let Some(stream) = (unsafe { stream.as_mut() }) else {
        set_errno(libc::EBADF);
        return 0;
    };
    let Some(bytes) = size
        .checked_mul(count)
        .filter(|total| *total <= isize::MAX as usize)
    else {
        stream.record_error(io_error(libc::EOVERFLOW));
        return 0;
    };
    stream.write_bytes(unsafe { std::slice::from_raw_parts(buffer.cast(), bytes) }) / size
}

pub(crate) unsafe fn fgetc(stream: *mut Stream) -> c_int {
    if let Some(stream) = unsafe { stream.as_mut() } {
        if stream.readable
            && !stream.eof
            && stream.pushback.is_empty()
            && stream.input_at < stream.input_end
        {
            let byte = stream.input[stream.input_at];
            stream.input_at += 1;
            stream.position = stream.position.saturating_add(1);
            return byte as c_int;
        }
    }
    let mut byte = 0u8;
    if unsafe { fread((&mut byte as *mut u8).cast(), 1, 1, stream) } == 1 {
        byte as c_int
    } else {
        EOF
    }
}

/// Read the ASCII decimal prefix used by web2c's interactive integer prompts.
/// Decimal overflow clamps to signed-long bounds before the legacy i32 cast.
pub(crate) unsafe fn scan_integer(stream: *mut Stream) -> Option<c_int> {
    let mut byte = unsafe { fgetc(stream) };
    while byte >= 0 && crate::integer_parse::is_c_whitespace(byte as u8) {
        byte = unsafe { fgetc(stream) };
    }
    let negative = byte == b'-' as c_int;
    if negative || byte == b'+' as c_int {
        byte = unsafe { fgetc(stream) };
    }
    if byte < 0 || !(byte as u8).is_ascii_digit() {
        if byte >= 0 {
            unsafe {
                ungetc(byte, stream);
            }
        }
        return None;
    }
    let ceiling = if negative {
        i64::MAX as u64 + 1
    } else {
        i64::MAX as u64
    };
    let mut magnitude = 0u64;
    while byte >= 0 && (byte as u8).is_ascii_digit() {
        magnitude = magnitude
            .saturating_mul(10)
            .saturating_add((byte as u8 - b'0') as u64)
            .min(ceiling);
        byte = unsafe { fgetc(stream) };
    }
    if byte >= 0 {
        unsafe {
            ungetc(byte, stream);
        }
    }
    let value = if negative {
        magnitude.wrapping_neg() as i64
    } else {
        magnitude as i64
    };
    Some(value as c_int)
}

pub(crate) unsafe fn putc(byte: c_int, stream: *mut Stream) -> c_int {
    let byte = byte as u8;
    if unsafe { fwrite((&byte as *const u8).cast(), 1, 1, stream) } == 1 {
        byte as c_int
    } else {
        EOF
    }
}

pub(crate) unsafe fn ungetc(byte: c_int, stream: *mut Stream) -> c_int {
    if byte == EOF {
        return EOF;
    }
    let Some(stream) = (unsafe { stream.as_mut() }) else {
        set_errno(libc::EBADF);
        return EOF;
    };
    if !stream.readable {
        return EOF;
    }
    if stream.pushback.len() >= MAX_PUSHBACK {
        set_errno(libc::ENOMEM);
        return EOF;
    }
    if stream.pushback.try_reserve(1).is_err() {
        set_errno(libc::ENOMEM);
        return EOF;
    }
    stream.pushback.push(byte as u8);
    stream.position = stream.position.saturating_sub(1);
    stream.eof = false;
    byte as u8 as c_int
}

pub(crate) unsafe fn feof(stream: *mut Stream) -> c_int {
    unsafe { stream.as_ref() }.is_some_and(|stream| stream.eof) as c_int
}

pub(crate) unsafe fn ferror(stream: *mut Stream) -> c_int {
    unsafe { stream.as_ref() }.is_none_or(|stream| stream.error) as c_int
}

pub(crate) unsafe fn clearerr(stream: *mut Stream) {
    if let Some(stream) = unsafe { stream.as_mut() } {
        stream.eof = false;
        stream.error = false;
    }
}

pub(crate) unsafe fn fflush(stream: *mut Stream) -> c_int {
    if stream.is_null() {
        return unsafe { flush_all() };
    }
    let stream = unsafe { &mut *stream };
    let result = stream.flush_output().and_then(|_| {
        if matches!(stream.backing, Backing::File(_)) {
            stream.sync_input()
        } else {
            Ok(())
        }
    });
    match result {
        Ok(()) => 0,
        Err(error) => {
            stream.record_error(error);
            EOF
        }
    }
}

pub(crate) unsafe fn fseeko(stream: *mut Stream, offset: i64, origin: c_int) -> c_int {
    let Some(stream) = (unsafe { stream.as_mut() }) else {
        set_errno(libc::EBADF);
        return EOF;
    };
    match stream.seek(offset, origin) {
        Ok(()) => 0,
        Err(error) => {
            stream.record_error(error);
            EOF
        }
    }
}

pub(crate) unsafe fn fseek(stream: *mut Stream, offset: c_long, origin: c_int) -> c_int {
    unsafe { fseeko(stream, offset, origin) }
}

pub(crate) unsafe fn ftello(stream: *mut Stream) -> i64 {
    let Some(stream) = (unsafe { stream.as_mut() }) else {
        set_errno(libc::EBADF);
        return -1;
    };
    match i64::try_from(stream.position) {
        Ok(position) => position,
        Err(_) => {
            set_errno(libc::EOVERFLOW);
            -1
        }
    }
}

pub(crate) unsafe fn ftell(stream: *mut Stream) -> c_long {
    unsafe { ftello(stream) as c_long }
}

pub(crate) unsafe fn rewind(stream: *mut Stream) {
    unsafe {
        fseeko(stream, 0, libc::SEEK_SET);
        clearerr(stream);
    }
}

pub(crate) unsafe fn fputs(text: *const c_char, stream: *mut Stream) -> c_int {
    let bytes = unsafe { CStr::from_ptr(text).to_bytes() };
    if unsafe { fwrite(bytes.as_ptr().cast(), 1, bytes.len(), stream) } == bytes.len() {
        0
    } else {
        EOF
    }
}

pub(crate) unsafe fn puts(text: *const c_char) -> c_int {
    if unsafe { fputs(text, stdout()) } == EOF {
        return EOF;
    }
    unsafe { putc(b'\n' as c_int, stdout()) }
}

pub(crate) fn error_text(code: c_int) -> String {
    let text = io_error(code).to_string();
    let suffix = format!(" (os error {code})");
    text.strip_suffix(&suffix).unwrap_or(&text).to_owned()
}

/// Write the saved OS error through the owned Rust stderr stream. The optional
/// prefix must be null or a valid NUL-terminated string on the engine thread.
pub(crate) unsafe fn perror(prefix: *const c_char) {
    let code = io::Error::last_os_error()
        .raw_os_error()
        .unwrap_or(libc::EIO);
    let message = error_text(code);
    let stream = stderr();
    if !prefix.is_null() {
        let prefix = unsafe { CStr::from_ptr(prefix).to_bytes() };
        if !prefix.is_empty() {
            unsafe {
                fwrite(prefix.as_ptr().cast(), 1, prefix.len(), stream);
                fwrite(b": ".as_ptr().cast(), 1, 2, stream);
            }
        }
    }
    unsafe {
        fwrite(message.as_ptr().cast(), 1, message.len(), stream);
        putc(b'\n' as c_int, stream);
    }
    set_errno(code);
}

pub(crate) unsafe fn fprintf_args(
    stream: *mut Stream,
    format: *const c_char,
    args: &[crate::utils::PrintfArg],
) -> c_int {
    let bytes = unsafe { crate::utils::format_printf(format, args) };
    if unsafe { fwrite(bytes.as_ptr().cast(), 1, bytes.len(), stream) } != bytes.len() {
        return EOF;
    }
    c_int::try_from(bytes.len()).unwrap_or_else(|_| {
        set_errno(libc::EOVERFLOW);
        EOF
    })
}

pub(crate) unsafe fn printf_args(format: *const c_char, args: &[crate::utils::PrintfArg]) -> c_int {
    unsafe { fprintf_args(stdout(), format, args) }
}

pub(crate) unsafe fn fgets(buffer: *mut c_char, size: c_int, stream: *mut Stream) -> *mut c_char {
    if size <= 0 {
        set_errno(libc::EINVAL);
        return ptr::null_mut();
    }
    let mut count = 0;
    while count < size as usize - 1 {
        let byte = unsafe { fgetc(stream) };
        if byte == EOF {
            break;
        }
        unsafe {
            *buffer.add(count) = byte as c_char;
        }
        count += 1;
        if byte == b'\n' as c_int {
            break;
        }
    }
    if count == 0 && size != 1 {
        return ptr::null_mut();
    }
    unsafe {
        *buffer.add(count) = 0;
    }
    buffer
}

pub(crate) unsafe fn setvbuf(
    stream: *mut Stream,
    _buffer: *mut c_char,
    mode: c_int,
    size: usize,
) -> c_int {
    let Some(stream) = (unsafe { stream.as_mut() }) else {
        set_errno(libc::EBADF);
        return EOF;
    };
    let buffering = match mode {
        0 => Buffering::Full,
        1 => Buffering::Line,
        2 => Buffering::None,
        _ => {
            set_errno(libc::EINVAL);
            return EOF;
        }
    };
    if let Err(error) = stream.flush_output().and_then(|_| stream.sync_input()) {
        stream.record_error(error);
        return EOF;
    }
    stream.buffering = buffering;
    stream.buffer_size = if size == 0 {
        BUFFER_SIZE
    } else {
        size.clamp(1, MAX_BUFFER_SIZE)
    };
    0
}

pub(crate) unsafe fn fclose(stream: *mut Stream) -> c_int {
    unsafe { close_handle(stream, false) }
}

unsafe fn close_handle(pointer: *mut Stream, pipe_status: bool) -> c_int {
    let Some(stream) = (unsafe { pointer.as_mut() }) else {
        set_errno(libc::EBADF);
        return EOF;
    };
    let result = stream.close();
    let owner = stream.owner;
    let status = match result {
        Ok(status) => {
            if pipe_status {
                status
            } else {
                0
            }
        }
        Err(error) => {
            stream.record_error(error);
            EOF
        }
    };
    if !owner.is_null() {
        // SAFETY: owned handles are registered by this pinned thread-confined
        // session. Every access must precede session teardown.
        let owner = unsafe { &*owner };
        let mut streams = owner.streams.borrow_mut();
        let index = streams
            .iter()
            .position(|stream| stream.as_ptr() == pointer)
            .expect("stream absent from its invocation owner");
        let removed = streams.swap_remove(index);
        // The last mutable Stream borrow ended before removing its owner.
        // Reconstruct only now, when no later FILE access is permitted.
        unsafe { drop(Box::from_raw(removed.as_ptr())) };
    }
    status
}

pub(crate) unsafe fn popen(command: *const c_char, mode: *const c_char) -> *mut Stream {
    let command = unsafe { OsStr::from_bytes(CStr::from_ptr(command).to_bytes()) };
    let mode = unsafe { CStr::from_ptr(mode).to_bytes() };
    let Some(&kind) = mode.first() else {
        set_errno(libc::EINVAL);
        return ptr::null_mut();
    };
    if !matches!(kind, b'r' | b'w') || mode[1..].iter().any(|byte| *byte != b'e') {
        set_errno(libc::EINVAL);
        return ptr::null_mut();
    }
    let mut builder = Command::new("/bin/sh");
    builder.arg("-c").arg(command);
    match crate::working_directory::current() {
        Ok(directory) => {
            builder.current_dir(directory);
        }
        Err(error) => {
            set_errno(error.raw_os_error().unwrap_or(libc::EIO));
            return ptr::null_mut();
        }
    }
    if let Err(error) = crate::environment::configure_command(&mut builder) {
        crate::environment::report(&error);
        set_errno(libc::EINVAL);
        return ptr::null_mut();
    }
    if kind == b'r' {
        builder.stdout(Stdio::piped());
    } else {
        builder.stdin(Stdio::piped());
    }
    let policy = with_session(|session| session.pipe_policy);
    let inherited = if kind == b'r' {
        InheritedIo::ReadPipe
    } else {
        InheritedIo::WritePipe
    };
    match PipeChild::spawn(&mut builder, policy, inherited) {
        Ok(mut child) => {
            let backing = if kind == b'r' {
                let output = child.child.stdout.take().unwrap();
                Backing::ReadPipe(child, output)
            } else {
                let input = child.child.stdin.take().unwrap();
                Backing::WritePipe(child, input)
            };
            Stream::new(backing, kind == b'r', kind == b'w', false, Buffering::Full).into_handle()
        }
        Err(error) => {
            set_errno(error.raw_os_error().unwrap_or(libc::EIO));
            ptr::null_mut()
        }
    }
}

/// # Safety
/// A non-null command must point to a terminated C string. Child environment
/// values are copied from the active invocation, never written to the parent.
/// Normal shell background commands remain caller-managed side effects, as do
/// descendants deliberately left by an explicitly closed pipe command.
pub(crate) unsafe fn system(command: *const c_char) -> c_int {
    if command.is_null() {
        return i32::from(std::path::Path::new("/bin/sh").is_file());
    }
    let command = OsStr::from_bytes(unsafe { CStr::from_ptr(command) }.to_bytes());
    let mut builder = Command::new("/bin/sh");
    builder.arg("-c").arg(command);
    match crate::working_directory::current() {
        Ok(directory) => {
            builder.current_dir(directory);
        }
        Err(error) => {
            set_errno(error.raw_os_error().unwrap_or(libc::EIO));
            return -1;
        }
    }
    if let Err(error) = crate::environment::configure_command(&mut builder) {
        crate::environment::report(&error);
        set_errno(libc::EINVAL);
        return -1;
    }
    let policy = with_session(|session| session.pipe_policy);
    match PipeChild::spawn(&mut builder, policy, InheritedIo::All)
        .and_then(|child| child.wait(false))
    {
        Ok(status) => status.into_raw(),
        Err(error) => {
            set_errno(error.raw_os_error().unwrap_or(libc::EIO));
            -1
        }
    }
}

pub(crate) unsafe fn pclose(stream: *mut Stream) -> c_int {
    unsafe { close_handle(stream, true) }
}

pub(crate) unsafe fn fileno(stream: *mut Stream) -> c_int {
    let Some(stream) = (unsafe { stream.as_ref() }) else {
        set_errno(libc::EBADF);
        return -1;
    };
    match &stream.backing {
        Backing::File(file) | Backing::GzipWrite(file, _) => file.as_raw_fd(),
        Backing::Stdin(input) => input.as_raw_fd(),
        Backing::Stdout(output) => output.as_raw_fd(),
        Backing::Stderr(error) => error.as_raw_fd(),
        Backing::ReadPipe(_, output) => output.as_raw_fd(),
        Backing::WritePipe(_, input) => input.as_raw_fd(),
        Backing::GzipRead(decoder) => decoder.get_ref().as_raw_fd(),
        Backing::GzipEncoder(encoder) => encoder.get_ref().as_raw_fd(),
        _ => {
            set_errno(libc::EBADF);
            -1
        }
    }
}

pub(crate) unsafe fn tmpfile() -> *mut Stream {
    let directory = crate::environment::var_os("TMPDIR")
        .filter(|value| !value.is_empty())
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"));
    let directory = match crate::working_directory::resolve(&directory) {
        Ok(path) => path,
        Err(error) => {
            set_errno(error.raw_os_error().unwrap_or(libc::EIO));
            return ptr::null_mut();
        }
    };
    for _ in 0..100 {
        let Some((serial, nonce)) = with_session(|session| {
            let serial = session.next_temp.get();
            session.next_temp.set(serial.checked_add(1)?);
            Some((serial, session.temp_nonce))
        }) else {
            set_errno(libc::EOVERFLOW);
            return ptr::null_mut();
        };
        // create_new and mode 0600 remain the authority for uniqueness and
        // access. The nonce only separates independent invocation namespaces.
        let path = directory.join(format!(
            "tekai-stream-{}-{nonce:x}-{serial}",
            std::process::id()
        ));
        match OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
        {
            Ok(file) => {
                if let Err(error) = std::fs::remove_file(path) {
                    set_errno(error.raw_os_error().unwrap_or(libc::EIO));
                    return ptr::null_mut();
                }
                return Stream::new(Backing::File(file), true, true, false, Buffering::Full)
                    .into_handle();
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                set_errno(error.raw_os_error().unwrap_or(libc::EIO));
                return ptr::null_mut();
            }
        }
    }
    set_errno(libc::EEXIST);
    ptr::null_mut()
}

/// Transfer the existing file into a transparent raw/gzip reader or gzip writer.
/// The stream retains ownership of the descriptor through final close.
pub(crate) unsafe fn gzip_open(pointer: *mut Stream, mode: *const c_char) -> *mut Stream {
    let Some(stream) = (unsafe { pointer.as_mut() }) else {
        set_errno(libc::EBADF);
        return ptr::null_mut();
    };
    let mode = unsafe { CStr::from_ptr(mode).to_bytes() };
    if let Err(error) = stream.flush_output().and_then(|_| stream.sync_input()) {
        stream.record_error(error);
        return ptr::null_mut();
    }
    let mut file = match std::mem::replace(&mut stream.backing, Backing::Closed) {
        Backing::File(file) => file,
        original => {
            stream.backing = original;
            set_errno(libc::EINVAL);
            return ptr::null_mut();
        }
    };
    if mode.first() == Some(&b'r') {
        let mut signature = [0; 2];
        let result = file
            .read(&mut signature)
            .and_then(|count| file.seek(SeekFrom::Start(stream.position)).map(|_| count));
        match result {
            Ok(2) if signature == [0x1f, 0x8b] => {
                stream.backing = Backing::GzipRead(MultiGzDecoder::new(file))
            }
            Ok(_) => stream.backing = Backing::File(file),
            Err(error) => {
                stream.backing = Backing::File(file);
                stream.record_error(error);
                return ptr::null_mut();
            }
        }
    } else if mode.first() == Some(&b'w') {
        stream.backing = Backing::GzipWrite(file, Compression::default());
    } else {
        stream.backing = Backing::File(file);
        set_errno(libc::EINVAL);
        return ptr::null_mut();
    }
    pointer
}

pub(crate) unsafe fn gzread(stream: *mut Stream, buffer: *mut c_void, len: u32) -> c_int {
    if len > c_int::MAX as u32 {
        set_errno(libc::EINVAL);
        return -1;
    }
    let read = unsafe { fread(buffer, 1, len as usize, stream) };
    // zlib reports -1 for a read error even when decompressed bytes were
    // placed in the caller's buffer before a footer/checksum failure.
    if unsafe { ferror(stream) } != 0 {
        -1
    } else {
        read as c_int
    }
}

pub(crate) unsafe fn gzwrite(stream: *mut Stream, buffer: *const c_void, len: u32) -> c_int {
    if len > c_int::MAX as u32 {
        set_errno(libc::EINVAL);
        return 0;
    }
    unsafe { fwrite(buffer, 1, len as usize, stream) as c_int }
}

pub(crate) unsafe fn gzsetparams(stream: *mut Stream, level: c_int, strategy: c_int) -> c_int {
    let Some(stream) = (unsafe { stream.as_mut() }) else {
        return -2;
    };
    if strategy != 0 || !(-1..=9).contains(&level) {
        return -2;
    }
    let Backing::GzipWrite(_, compression) = &mut stream.backing else {
        return -2;
    };
    *compression = if level == -1 {
        Compression::default()
    } else {
        Compression::new(level as u32)
    };
    0
}

pub(crate) unsafe fn gzclose(stream: *mut Stream) -> c_int {
    unsafe { fclose(stream) }
}

pub(crate) unsafe fn flush_all() -> c_int {
    unsafe { flush_outputs(true) }
}

/// Flush ordinary files and consoles before an invocation exits. Pipe output
/// belongs to explicit fflush/pclose only, so implicit cleanup cannot wait for
/// an abandoned command to consume buffered bytes.
pub(crate) unsafe fn flush_for_exit() -> c_int {
    unsafe { flush_outputs(false) }
}

unsafe fn flush_outputs(include_pipes: bool) -> c_int {
    let mut result = 0;
    with_session(|session| {
        for pointer in session.streams.borrow().iter() {
            // SAFETY FILE adapters and this sweep use the same into_raw
            // provenance. No parent Box reborrow invalidates a retained FILE.
            let stream = unsafe { &mut *pointer.as_ptr() };
            if !include_pipes
                && matches!(
                    stream.backing,
                    Backing::ReadPipe(..) | Backing::WritePipe(..)
                )
            {
                continue;
            }
            // fflush(NULL) flushes pending output, not read-ahead buffers.
            if let Err(error) = stream.flush_output() {
                stream.record_error(error);
                result = EOF;
            }
        }
    });
    for stream in [stdout(), stderr()] {
        if unsafe { fflush(stream) } != 0 {
            result = EOF;
        }
    }
    result
}

pub(crate) fn exit(status: c_int) -> ! {
    unsafe {
        flush_for_exit();
    }
    crate::termination::exit(status)
}

#[cfg(test)]
mod tests {
    use super::*;
    mod terminal_tests {
        include!("stdio_terminal_tests.rs");
    }
    use proptest::prelude::*;
    use rustix::process::kill_process_group;
    use std::os::fd::FromRawFd;
    use std::rc::Rc;

    struct Handle(*mut Stream);
    impl Drop for Handle {
        fn drop(&mut self) {
            unsafe {
                fclose(self.0);
            }
        }
    }
    fn temporary() -> Handle {
        let handle = Handle(unsafe { tmpfile() });
        assert!(!handle.0.is_null());
        handle
    }

    #[test]
    fn session_releases_unclosed_files_and_console_state() {
        let mut session = StreamSession::new();
        let (descriptor, original) = session.as_mut().scope(|| unsafe {
            let stream = tmpfile();
            assert!(!stream.is_null());
            assert_eq!(fwrite(b"pending".as_ptr().cast(), 1, 7, stream), 7);
            let input = stdin();
            (*input).pushback.push(b'x');
            (*input).eof = true;
            assert_eq!(session_count(), 1);
            let Backing::File(file) = &(*stream).backing else {
                panic!()
            };
            (fileno(stream), file.try_clone().unwrap())
        });
        session.as_mut().finish().unwrap();
        assert_eq!(session.streams.borrow().len(), 0);
        assert_descriptor_released(descriptor, &original);
        drop(session);

        let mut next = StreamSession::new();
        next.as_mut().scope(|| unsafe {
            assert!((*stdin()).pushback.is_empty());
            assert!(!(*stdin()).eof);
            assert_eq!(session_count(), 0);
        });
        next.as_mut().finish().unwrap();
    }

    fn session_count() -> usize {
        with_session(|session| session.streams.borrow().len())
    }

    fn assert_descriptor_released(descriptor: c_int, original: &File) {
        use std::os::unix::fs::MetadataExt;
        let mut metadata = std::mem::MaybeUninit::<libc::stat>::uninit();
        let result = unsafe { libc::fstat(descriptor, metadata.as_mut_ptr()) };
        if result == -1 {
            assert_eq!(io::Error::last_os_error().raw_os_error(), Some(libc::EBADF));
        } else {
            // Another parallel test can reuse the released descriptor number.
            // Holding a duplicate keeps this unlinked temporary file's inode
            // live, so a different identity proves this handle was released.
            let metadata = unsafe { metadata.assume_init() };
            let original = original.metadata().unwrap();
            // libc's field widths vary across supported Unix targets.
            #[allow(clippy::unnecessary_cast)]
            let identity = (metadata.st_dev as u64, metadata.st_ino as u64);
            assert_ne!(
                identity,
                (original.dev(), original.ino()),
                "invocation retained its open file descriptor",
            );
        }
    }

    #[test]
    fn explicit_close_removes_the_owner_without_double_close_at_teardown() {
        let mut session = StreamSession::new();
        session.as_mut().scope(|| unsafe {
            let handles = (0..32).map(|_| tmpfile()).collect::<Vec<_>>();
            assert!(handles.iter().all(|handle| !handle.is_null()));
            assert_eq!(session_count(), 32);
            for (remaining, handle) in handles.into_iter().enumerate() {
                assert_eq!(fclose(handle), 0);
                assert_eq!(session_count(), 31 - remaining);
            }
        });
        session.as_mut().finish().unwrap();
    }

    #[test]
    fn session_unwinding_restores_routing_and_drops_remaining_handles() {
        use std::panic::{catch_unwind, AssertUnwindSafe};

        let mut descriptor = -1;
        let mut original = None;
        let result = catch_unwind(AssertUnwindSafe(|| {
            let mut session = StreamSession::new();
            session.as_mut().scope(|| unsafe {
                let handle = tmpfile();
                assert!(!handle.is_null());
                descriptor = fileno(handle);
                let Backing::File(file) = &(*handle).backing else {
                    panic!()
                };
                original = Some(file.try_clone().unwrap());
                panic!("test stream session unwind");
            });
        }));
        assert!(result.is_err());
        assert!(ACTIVE_SESSION.with(Cell::get).is_null());
        assert_descriptor_released(descriptor, original.as_ref().unwrap());
    }

    #[test]
    fn nested_session_does_not_replace_outer_routing() {
        use std::panic::{catch_unwind, AssertUnwindSafe};

        let mut outer = StreamSession::new();
        let mut inner = StreamSession::new();
        outer.as_mut().scope(|| {
            let initial = stdout();
            assert!(catch_unwind(AssertUnwindSafe(|| {
                inner.as_mut().scope(|| unreachable!());
            }))
            .is_err());
            assert_eq!(stdout(), initial);
        });
        outer.as_mut().finish().unwrap();
        inner.as_mut().finish().unwrap();
    }

    #[test]
    fn teardown_reaps_a_pipe_child_that_does_not_exit_on_eof() {
        let mut session = StreamSession::new();
        let process = session.as_mut().scope(|| unsafe {
            let handle = popen(c"exec sleep 600".as_ptr(), c"w".as_ptr());
            assert!(!handle.is_null());
            let Backing::WritePipe(child, _) = &(*handle).backing else {
                panic!()
            };
            child.id()
        });
        session.as_mut().finish().unwrap();
        let mut status = 0;
        assert_eq!(
            unsafe { libc::waitpid(process as i32, &mut status, libc::WNOHANG) },
            -1
        );
        assert_eq!(
            io::Error::last_os_error().raw_os_error(),
            Some(libc::ECHILD)
        );
    }

    fn supervised_pipe_test(operation: impl FnOnce(std::sync::mpsc::Sender<Pid>) + Send + 'static) {
        use std::sync::mpsc::{channel, RecvTimeoutError};
        use std::time::Duration;

        let (group_sender, group_receiver) = channel();
        let (done_sender, done_receiver) = channel();
        let worker = std::thread::spawn(move || {
            operation(group_sender);
            let _ = done_sender.send(());
        });
        let group = group_receiver.recv_timeout(Duration::from_secs(10));
        if let Ok(group) = group {
            let completion = done_receiver.recv_timeout(Duration::from_secs(10));
            if matches!(completion, Err(RecvTimeoutError::Timeout)) {
                // A regression may be stuck flushing to a full pipe. Its
                // unreaped leader still reserves this group ID. Stop it so the
                // failing test does not leave a child or blocked worker behind.
                let _ = kill_process_group(group, Signal::KILL);
            }
            worker.join().unwrap();
            assert!(completion.is_ok(), "owned pipe teardown did not complete");
        } else {
            worker.join().unwrap();
            panic!("pipe worker did not register its child group");
        }
    }

    fn assert_pipe_child_reaped(process: u32) {
        let mut status = 0;
        assert_eq!(
            unsafe { libc::waitpid(process as i32, &mut status, libc::WNOHANG) },
            -1
        );
        assert_eq!(
            io::Error::last_os_error().raw_os_error(),
            Some(libc::ECHILD)
        );
    }

    fn pipe_process_state(process: Pid) -> String {
        let snapshot = Command::new("/bin/ps")
            .args(["-o", "stat=", "-p", &process.as_raw_pid().to_string()])
            .output()
            .unwrap();
        String::from_utf8(snapshot.stdout).unwrap()
    }

    fn assert_pipe_process_stopped(process: Pid) {
        use std::time::{Duration, Instant};

        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let state = pipe_process_state(process);
            if state.trim().is_empty() || state.trim().starts_with('Z') {
                return;
            }
            if Instant::now() >= deadline {
                // The test is responsible for its deliberately launched
                // descendant even when the production cleanup regresses.
                let _ = rustix::process::kill_process(process, Signal::KILL);
                panic!("abandoned pipe descendant is still running: {state}");
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    #[test]
    fn abandoned_full_write_pipe_skips_implicit_flush_and_releases_child() {
        supervised_pipe_test(|group_sender| {
            let mut session = StreamSession::new();
            let process = session.as_mut().scope(|| unsafe {
                let handle = popen(c"exec sleep 600".as_ptr(), c"w".as_ptr());
                assert!(!handle.is_null());
                let Backing::WritePipe(child, input) = &mut (*handle).backing else {
                    panic!()
                };
                let process = child.id();
                group_sender.send(child.group.unwrap()).unwrap();
                let descriptor = input.as_raw_fd();
                let flags = libc::fcntl(descriptor, libc::F_GETFL);
                assert!(flags >= 0);
                assert_eq!(
                    libc::fcntl(descriptor, libc::F_SETFL, flags | libc::O_NONBLOCK),
                    0
                );
                let mut full = false;
                for _ in 0..4096 {
                    match input.write(&[b'x'; 4096]) {
                        Ok(count) => assert!(count > 0),
                        Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                            full = true;
                            break;
                        }
                        Err(error) => panic!("cannot fill test pipe: {error}"),
                    }
                }
                assert!(full, "test pipe did not reach its bounded capacity");
                assert_eq!(libc::fcntl(descriptor, libc::F_SETFL, flags), 0);
                (*handle).output.extend_from_slice(b"pending pipe output");

                let file = tmpfile();
                assert_eq!(fwrite(c"ordinary".as_ptr().cast(), 1, 8, file), 8);
                assert_eq!(flush_for_exit(), 0);
                assert_eq!((*file).metadata().unwrap().len(), 8);
                assert_eq!((*handle).output, b"pending pipe output");
                process
            });
            session.as_mut().finish().unwrap();
            assert_pipe_child_reaped(process);
        });
    }

    #[test]
    fn abandoned_shell_pipe_stops_background_descendants_on_unwind() {
        supervised_pipe_test(|group_sender| {
            let mut process = 0;
            let mut descendant = None;
            let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut session = StreamSession::new();
                session.as_mut().scope(|| unsafe {
                    let handle = popen(
                        c"sleep 600 & printf '%s\\n' \"$!\"; wait".as_ptr(),
                        c"r".as_ptr(),
                    );
                    assert!(!handle.is_null());
                    let Backing::ReadPipe(child, _) = &(*handle).backing else {
                        panic!()
                    };
                    process = child.id();
                    group_sender.send(child.group.unwrap()).unwrap();
                    let mut line = [0 as c_char; 32];
                    assert!(!fgets(line.as_mut_ptr(), line.len() as c_int, handle).is_null());
                    let raw = CStr::from_ptr(line.as_ptr())
                        .to_str()
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap();
                    descendant = Pid::from_raw(raw);
                    assert!(descendant.is_some());
                    panic!("abandon the open pipe during engine unwinding");
                });
            }));
            assert!(failure.is_err());
            assert_pipe_child_reaped(process);
            assert_pipe_process_stopped(descendant.unwrap());
        });
    }

    #[test]
    fn explicit_pclose_preserves_background_commands_and_shell_exit_status() {
        supervised_pipe_test(|group_sender| {
            let mut session = StreamSession::new();
            let descendant = session.as_mut().scope(|| unsafe {
                let handle = popen(
                    c"sleep 600 & printf '%s\\n' \"$!\"; exit 7".as_ptr(),
                    c"r".as_ptr(),
                );
                assert!(!handle.is_null());
                let Backing::ReadPipe(child, _) = &(*handle).backing else {
                    panic!()
                };
                let process = child.id();
                group_sender.send(child.group.unwrap()).unwrap();
                let mut line = [0 as c_char; 32];
                assert!(!fgets(line.as_mut_ptr(), line.len() as c_int, handle).is_null());
                let raw = CStr::from_ptr(line.as_ptr())
                    .to_str()
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap();
                let descendant = Pid::from_raw(raw).unwrap();
                let result = pclose(handle);
                // Explicitly closed/background commands are caller-managed.
                // Stop this test's descendant before checking assertions.
                let state = pipe_process_state(descendant);
                let running = !state.trim().is_empty() && !state.trim().starts_with('Z');
                let _ = rustix::process::kill_process(descendant, Signal::KILL);
                assert_eq!(result, 7 << 8);
                assert!(running, "explicit pclose killed its background command");
                assert_pipe_child_reaped(process);
                descendant
            });
            session.as_mut().finish().unwrap();
            assert_pipe_process_stopped(descendant);
        });
    }

    #[test]
    #[ignore = "isolated worker launched by the supervised pipe policy test"]
    fn supervised_process_pipe_fixture() {
        assert_eq!(
            std::env::var("TEKAI_SUPERVISED_PIPE_FIXTURE").as_deref(),
            Ok("1")
        );
        // The test parent owns this worker's group. Killing it locally would
        // also kill this fixture, so test that cleanup reaches the assertion.
        let mut session = StreamSession::new_with_pipe_policy(PipeProcessPolicy::SupervisedProcess);
        let process = session.as_mut().scope(|| unsafe {
            let handle = popen(c"exec sleep 600".as_ptr(), c"w".as_ptr());
            assert!(!handle.is_null());
            let Backing::WritePipe(child, _) = &(*handle).backing else {
                panic!()
            };
            assert!(child.group.is_none());
            let process = Pid::from_raw(child.id() as i32).unwrap();
            assert_eq!(
                rustix::process::getpgid(Some(process)).unwrap(),
                rustix::process::getpgrp()
            );
            child.id()
        });
        session.as_mut().finish().unwrap();
        assert_pipe_child_reaped(process);
        assert_eq!(
            rustix::process::getpgrp().as_raw_pid(),
            std::process::id() as i32
        );
    }

    #[test]
    fn supervised_pipe_policy_never_signals_its_inherited_worker_group() {
        use rustix::process::{waitid, WaitId, WaitIdOptions};
        use std::time::{Duration, Instant};

        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--ignored",
                "--exact",
                "stdio::tests::supervised_process_pipe_fixture",
            ])
            .env("TEKAI_SUPERVISED_PIPE_FIXTURE", "1")
            .stdout(Stdio::null());
        let worker = PipeChild::spawn(
            &mut command,
            PipeProcessPolicy::InvocationOwned,
            InheritedIo::All,
        )
        .unwrap();
        let group = worker.group.unwrap();
        let deadline = Instant::now() + Duration::from_secs(10);
        let completed = loop {
            match waitid(
                WaitId::Pid(group),
                WaitIdOptions::EXITED | WaitIdOptions::NOWAIT | WaitIdOptions::NOHANG,
            ) {
                Ok(Some(_)) => break true,
                Ok(None) | Err(rustix::io::Errno::INTR) => {}
                Err(error) => panic!("cannot supervise pipe fixture: {error}"),
            }
            if Instant::now() >= deadline {
                break false;
            }
            std::thread::sleep(Duration::from_millis(10));
        };
        // Kill before reap, including timeout, so a failing fixture cannot
        // leave descendants and its reserved process-group ID cannot be reused.
        let status = worker.wait(true).unwrap();
        assert!(completed, "supervised pipe fixture did not finish");
        assert!(status.success(), "supervised pipe fixture failed: {status}");
    }

    // Native FILE objects are an oracle only. They never cross the production
    // Stream boundary, and each native descriptor remains owned by libc.
    struct NativeHandle(*mut libc::FILE);
    impl Drop for NativeHandle {
        fn drop(&mut self) {
            unsafe {
                libc::fclose(self.0);
            }
        }
    }
    fn native_temporary() -> NativeHandle {
        let handle = NativeHandle(unsafe { libc::tmpfile() });
        assert!(!handle.0.is_null());
        handle
    }

    #[test]
    fn standard_io_error_text_preserves_native_diagnostic_words() {
        for code in [libc::ENOENT, libc::EIO, libc::EBADF, libc::EINVAL] {
            let native = unsafe { CStr::from_ptr(libc::strerror(code)) }.to_string_lossy();
            assert_eq!(error_text(code), native);
        }
    }

    #[test]
    fn repeated_eof_and_growth_require_clearerr_like_native_stdio() {
        let rust = temporary();
        let native = native_temporary();
        unsafe {
            for payload in [b"abc".as_slice(), b"def".as_slice()] {
                assert_eq!(fwrite(payload.as_ptr().cast(), 1, payload.len(), rust.0), 3);
                assert_eq!(
                    libc::fwrite(payload.as_ptr().cast(), 1, payload.len(), native.0),
                    3
                );
            }
            rewind(rust.0);
            libc::rewind(native.0);
            for _ in 0..12 {
                assert_eq!(fgetc(rust.0), libc::fgetc(native.0));
                assert_eq!(feof(rust.0), (libc::feof(native.0) != 0) as i32);
                assert_eq!(ferror(rust.0), (libc::ferror(native.0) != 0) as i32);
            }
            // Append through a duplicate descriptor without touching either
            // logical stream or its sticky EOF state.
            let mut rust_file =
                File::from(std::os::fd::OwnedFd::from_raw_fd(libc::dup(fileno(rust.0))));
            let mut native_file = File::from(std::os::fd::OwnedFd::from_raw_fd(libc::dup(
                libc::fileno(native.0),
            )));
            rust_file.seek(SeekFrom::End(0)).unwrap();
            native_file.seek(SeekFrom::End(0)).unwrap();
            rust_file.write_all(b"g").unwrap();
            native_file.write_all(b"g").unwrap();
            assert_eq!(fgetc(rust.0), EOF);
            assert_eq!(libc::fgetc(native.0), EOF);
            clearerr(rust.0);
            libc::clearerr(native.0);
            // Shared descriptor offsets are now at the end after appending.
            rust_file.seek(SeekFrom::Start(6)).unwrap();
            native_file.seek(SeekFrom::Start(6)).unwrap();
            assert_eq!(fgetc(rust.0), b'g' as i32);
            assert_eq!(libc::fgetc(native.0), b'g' as i32);
            assert_eq!(fgetc(rust.0), libc::fgetc(native.0));
        }
    }

    #[test]
    fn read_error_after_success_never_replays_the_old_buffer() {
        let rust = temporary();
        let native = native_temporary();
        let write_only = OpenOptions::new().write(true).open("/dev/null").unwrap();
        unsafe {
            assert_eq!(fwrite(b"abc".as_ptr().cast(), 1, 3, rust.0), 3);
            assert_eq!(libc::fwrite(b"abc".as_ptr().cast(), 1, 3, native.0), 3);
            rewind(rust.0);
            libc::rewind(native.0);
            assert_eq!(fgetc(rust.0), b'a' as i32);
            assert_eq!(libc::fgetc(native.0), b'a' as i32);
            // Replacing, rather than closing, keeps both owned descriptors
            // valid for their eventual destructors. Buffered bytes still read
            // normally; the following refill gets EBADF on a write-only fd.
            assert_eq!(
                libc::dup2(write_only.as_raw_fd(), fileno(rust.0)),
                fileno(rust.0)
            );
            assert_eq!(
                libc::dup2(write_only.as_raw_fd(), libc::fileno(native.0)),
                libc::fileno(native.0)
            );
            for expected in [b'b' as i32, b'c' as i32, EOF, EOF, EOF] {
                assert_eq!(fgetc(rust.0), expected);
                assert_eq!(libc::fgetc(native.0), expected);
                assert_eq!(feof(rust.0), (libc::feof(native.0) != 0) as i32);
                assert_eq!(ferror(rust.0), (libc::ferror(native.0) != 0) as i32);
            }
            clearerr(rust.0);
            libc::clearerr(native.0);
            assert_eq!(fgetc(rust.0), libc::fgetc(native.0));
            assert_eq!(ferror(rust.0), 1);
        }
    }

    #[test]
    fn input_flush_and_update_stream_position_match_native_stdio() {
        let rust = temporary();
        let native = native_temporary();
        unsafe {
            assert_eq!(fwrite(b"abcdef".as_ptr().cast(), 1, 6, rust.0), 6);
            assert_eq!(libc::fwrite(b"abcdef".as_ptr().cast(), 1, 6, native.0), 6);
            rewind(rust.0);
            libc::rewind(native.0);
            assert_eq!(fgetc(rust.0), libc::fgetc(native.0));
            assert_eq!(fflush(rust.0), libc::fflush(native.0));
            assert_eq!(ftello(rust.0), libc::ftello(native.0));
            assert_eq!(
                libc::lseek(fileno(rust.0), 0, libc::SEEK_CUR),
                libc::lseek(libc::fileno(native.0), 0, libc::SEEK_CUR)
            );
            assert_eq!(fgetc(rust.0), libc::fgetc(native.0));
            assert_eq!(
                fseeko(rust.0, 0, libc::SEEK_CUR),
                libc::fseeko(native.0, 0, libc::SEEK_CUR)
            );
            assert_eq!(
                putc(b'Z' as i32, rust.0),
                libc::fputc(b'Z' as i32, native.0)
            );
            assert_eq!(fflush(rust.0), libc::fflush(native.0));
            rewind(rust.0);
            libc::rewind(native.0);
            for _ in 0..8 {
                assert_eq!(fgetc(rust.0), libc::fgetc(native.0));
            }
        }
    }

    #[test]
    fn input_flush_discards_mismatching_pushback_at_the_owned_logical_position() {
        let handle = temporary();
        unsafe {
            assert_eq!(fwrite(b"abcdef".as_ptr().cast(), 1, 6, handle.0), 6);
            rewind(handle.0);
            assert_eq!(fgetc(handle.0), b'a' as i32);
            assert_eq!(ungetc(b'A' as i32, handle.0), b'A' as i32);
            assert_eq!(ftello(handle.0), 0);
            assert_eq!(fflush(handle.0), 0);
            assert_eq!(ftello(handle.0), 0);
            assert_eq!(libc::lseek(fileno(handle.0), 0, libc::SEEK_CUR), 0);
            assert_eq!(fgetc(handle.0), b'a' as i32);
        }
    }

    #[derive(Default)]
    struct WriterState {
        bytes: Vec<u8>,
        calls: usize,
        flushes: usize,
        limit: Option<usize>,
        chunk: usize,
        interrupt: bool,
        flush_error: bool,
    }
    struct MeasuredWriter(Rc<RefCell<WriterState>>);
    impl Write for MeasuredWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            let mut state = self.0.borrow_mut();
            state.calls += 1;
            if std::mem::take(&mut state.interrupt) {
                return Err(io::ErrorKind::Interrupted.into());
            }
            let available = state
                .limit
                .map_or(bytes.len(), |limit| limit.saturating_sub(state.bytes.len()));
            if available == 0 {
                return Err(io_error(libc::EIO));
            }
            let count = bytes.len().min(available).min(state.chunk.max(1));
            state.bytes.extend_from_slice(&bytes[..count]);
            Ok(count)
        }
        fn flush(&mut self) -> io::Result<()> {
            let mut state = self.0.borrow_mut();
            state.flushes += 1;
            if state.flush_error {
                Err(io_error(libc::EIO))
            } else {
                Ok(())
            }
        }
    }
    fn measured_writer(state: Rc<RefCell<WriterState>>, size: usize) -> Stream {
        let mut stream = Stream::new(
            Backing::TestWriter(Box::new(MeasuredWriter(state))),
            false,
            true,
            false,
            Buffering::Full,
        );
        stream.buffer_size = size;
        stream
    }

    #[test]
    fn short_erroring_writes_preserve_pending_bytes_and_current_call_counts() {
        let state = Rc::new(RefCell::new(WriterState {
            limit: Some(10),
            chunk: 2,
            interrupt: true,
            ..Default::default()
        }));
        let mut stream = measured_writer(state.clone(), 8);
        assert_eq!(stream.write_bytes(b"old"), 3);
        assert_eq!(stream.write_bytes(b"0123456789abcdef"), 7);
        assert!(stream.error);
        assert_eq!(state.borrow().bytes, b"old0123456");
        assert_eq!(stream.output, b"789abc");
        for _ in 0..20 {
            assert_eq!(stream.write_bytes(b"XY"), 0);
            assert!(stream.output.len() <= 8);
            assert!(stream.output.capacity() <= MAX_BUFFER_SIZE);
        }
        state.borrow_mut().limit = None;
        stream.flush_output().unwrap();
        assert_eq!(state.borrow().bytes, b"old0123456789abcXY");
        assert!(stream.error); // Successful retry does not clear sticky error.
    }

    #[test]
    fn flush_failure_does_not_undo_already_written_byte_counts() {
        let state = Rc::new(RefCell::new(WriterState {
            chunk: usize::MAX,
            flush_error: true,
            ..Default::default()
        }));
        let mut stream = measured_writer(state.clone(), 4);
        stream.buffering = Buffering::None;
        assert_eq!(stream.write_bytes(b"abcd"), 4);
        assert_eq!(state.borrow().bytes, b"abcd");
        assert!(stream.output.is_empty());
        assert!(stream.error);
    }

    #[test]
    fn zero_progress_writer_reports_failure_without_growing_retained_data() {
        struct ZeroWriter;
        impl Write for ZeroWriter {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Ok(0)
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let mut stream = Stream::new(
            Backing::TestWriter(Box::new(ZeroWriter)),
            false,
            true,
            false,
            Buffering::Full,
        );
        stream.buffer_size = 4;
        unsafe {
            assert_eq!(fwrite(b"abcd".as_ptr().cast(), 2, 2, &mut stream), 0);
            assert_eq!(ferror(&mut stream), 1);
            for _ in 0..16 {
                assert_eq!(fwrite(b"xyz".as_ptr().cast(), 1, 3, &mut stream), 0);
            }
        }
        assert_eq!(stream.output, b"abcd");
        assert_eq!(stream.output.len(), 4);
    }

    #[test]
    fn byte_io_has_buffer_sized_backing_call_bounds() {
        struct MeasuredReader {
            bytes: io::Cursor<Vec<u8>>,
            calls: Rc<RefCell<usize>>,
        }
        impl Read for MeasuredReader {
            fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
                *self.calls.borrow_mut() += 1;
                self.bytes.read(bytes)
            }
        }
        let count = BUFFER_SIZE * 3 + 17;
        let input: Vec<u8> = (0..count).map(|index| (index % 251) as u8).collect();
        let read_calls = Rc::new(RefCell::new(0));
        let mut reader = Stream::new(
            Backing::TestReader(Box::new(MeasuredReader {
                bytes: io::Cursor::new(input.clone()),
                calls: read_calls.clone(),
            })),
            true,
            false,
            false,
            Buffering::Full,
        );
        for &byte in &input {
            assert_eq!(unsafe { fgetc(&mut reader) }, byte as i32);
        }
        assert_eq!(*read_calls.borrow(), count.div_ceil(BUFFER_SIZE));
        for _ in 0..20 {
            assert_eq!(unsafe { fgetc(&mut reader) }, EOF);
        }
        assert_eq!(*read_calls.borrow(), count.div_ceil(BUFFER_SIZE) + 1);
        let state = Rc::new(RefCell::new(WriterState {
            chunk: usize::MAX,
            ..Default::default()
        }));
        let mut writer = measured_writer(state.clone(), BUFFER_SIZE);
        for &byte in &input {
            assert_eq!(unsafe { putc(byte as i32, &mut writer) }, byte as i32);
        }
        writer.flush_output().unwrap();
        assert_eq!(state.borrow().bytes, input);
        assert_eq!(state.borrow().calls, count.div_ceil(BUFFER_SIZE));
        assert_eq!(state.borrow().flushes, 1);
    }

    #[test]
    fn line_and_unbuffered_modes_explicitly_flush_the_backing_writer() {
        let line_state = Rc::new(RefCell::new(WriterState {
            chunk: usize::MAX,
            ..Default::default()
        }));
        let mut line = measured_writer(line_state.clone(), 16);
        line.buffering = Buffering::Line;
        assert_eq!(line.write_bytes(b"first"), 5);
        assert_eq!(line_state.borrow().calls, 0);
        assert_eq!(line.write_bytes(b"line\nmore"), 9);
        assert_eq!(line_state.borrow().bytes, b"firstline\n");
        assert_eq!(line_state.borrow().flushes, 1);
        line.flush_output().unwrap();
        assert_eq!(line_state.borrow().bytes, b"firstline\nmore");
        assert_eq!(line_state.borrow().flushes, 2);
        let plain_state = Rc::new(RefCell::new(WriterState {
            chunk: usize::MAX,
            ..Default::default()
        }));
        let mut plain = measured_writer(plain_state.clone(), 16);
        plain.buffering = Buffering::None;
        for byte in b"abc" {
            assert_eq!(unsafe { putc(*byte as i32, &mut plain) }, *byte as i32);
        }
        assert_eq!(plain_state.borrow().bytes, b"abc");
        assert_eq!(
            (plain_state.borrow().calls, plain_state.borrow().flushes),
            (3, 3)
        );
    }

    #[test]
    fn tmpfile_permissions_and_buffer_cap_are_private_and_bounded() {
        use std::os::unix::fs::PermissionsExt;
        let handle = temporary();
        unsafe {
            assert_eq!(
                (*handle.0).metadata().unwrap().permissions().mode() & 0o777,
                0o600
            );
            assert_eq!(setvbuf(handle.0, ptr::null_mut(), 0, usize::MAX), 0);
            assert_eq!((*handle.0).buffer_size, MAX_BUFFER_SIZE);
            let payload = vec![1u8; MAX_BUFFER_SIZE * 3 + 1];
            assert_eq!(
                fwrite(payload.as_ptr().cast(), 1, payload.len(), handle.0),
                payload.len()
            );
            rewind(handle.0);
            let mut bytes = vec![0u8; payload.len()];
            assert_eq!(
                fread(bytes.as_mut_ptr().cast(), 1, bytes.len(), handle.0),
                bytes.len()
            );
            assert_eq!(bytes, payload);
            assert!((*handle.0).input.capacity() <= MAX_BUFFER_SIZE);
            assert!((*handle.0).output.capacity() <= MAX_BUFFER_SIZE);
        }
    }

    #[test]
    fn eof_pushback_and_error_flags_are_independent() {
        let handle = temporary();
        unsafe {
            assert_eq!(fgetc(handle.0), EOF);
            assert_eq!(feof(handle.0), 1);
            assert_eq!(ferror(handle.0), 0);
            assert_eq!(ungetc(255, handle.0), 255);
            assert_eq!(feof(handle.0), 0);
            assert_eq!(fgetc(handle.0), 255);
            assert_eq!(fgetc(handle.0), EOF);
            assert_eq!(ungetc(EOF, handle.0), EOF);
            assert_eq!(feof(handle.0), 1);
            assert_eq!(fseeko(handle.0, -1, libc::SEEK_SET), EOF);
            assert_eq!(ferror(handle.0), 1);
            clearerr(handle.0);
            assert_eq!(feof(handle.0), 0);
            assert_eq!(ferror(handle.0), 0);
            assert_eq!(fseeko(handle.0, 0, libc::SEEK_SET), 0);
        }
    }

    #[test]
    fn pushback_storage_is_bounded_and_failure_preserves_existing_bytes_and_flags() {
        let handle = temporary();
        unsafe {
            let input = [42u8; MAX_PUSHBACK];
            assert_eq!(
                fwrite(input.as_ptr().cast(), 1, input.len(), handle.0),
                input.len()
            );
            rewind(handle.0);
            for &byte in &input {
                assert_eq!(fgetc(handle.0), byte as i32);
            }
            assert_eq!(fgetc(handle.0), EOF);
            for byte in 0..MAX_PUSHBACK {
                assert_eq!(ungetc(byte as i32, handle.0), byte as i32);
            }
            assert_eq!(ftello(handle.0), 0);
            assert_eq!(ungetc(123, handle.0), EOF);
            assert_eq!(
                io::Error::last_os_error().raw_os_error(),
                Some(libc::ENOMEM)
            );
            assert_eq!(feof(handle.0), 0);
            assert_eq!(ferror(handle.0), 0);
            assert_eq!(ftello(handle.0), 0);
            assert_eq!((*handle.0).pushback.len(), MAX_PUSHBACK);
            assert!((*handle.0).pushback.capacity() <= MAX_PUSHBACK);
            for byte in (0..MAX_PUSHBACK).rev() {
                assert_eq!(fgetc(handle.0), byte as i32);
            }
            assert_eq!(ftello(handle.0), MAX_PUSHBACK as i64);
            assert_eq!(fgetc(handle.0), EOF);
        }
    }

    #[test]
    fn closing_an_unowned_handle_does_not_reopen_or_replay_buffered_input() {
        let mut stream = Stream::new(
            Backing::TestReader(Box::new(io::Cursor::new(b"abc".to_vec()))),
            true,
            false,
            false,
            Buffering::Full,
        );
        unsafe {
            assert_eq!(fgetc(&mut stream), b'a' as i32);
            assert_eq!(ungetc(b'A' as i32, &mut stream), b'A' as i32);
            assert_eq!(fclose(&mut stream), 0);
            assert_eq!(fgetc(&mut stream), EOF);
            assert_eq!(ferror(&mut stream), 1);
            assert_eq!(putc(b'X' as i32, &mut stream), EOF);
            assert_eq!(fileno(&mut stream), -1);
        }
        assert!(matches!(stream.backing, Backing::Closed));
        assert!(stream.pushback.is_empty());
    }

    #[test]
    fn element_counts_include_consumed_partial_bytes() {
        let handle = temporary();
        let input = b"abcde";
        let mut output = [0; 12];
        unsafe {
            assert_eq!(fwrite(input.as_ptr().cast(), 1, input.len(), handle.0), 5);
            assert_eq!(ftello(handle.0), 5);
            rewind(handle.0);
            assert_eq!(fread(output.as_mut_ptr().cast(), 3, 4, handle.0), 1);
            assert_eq!(&output[..5], input);
            assert_eq!(ftello(handle.0), 5);
            assert_eq!(feof(handle.0), 1);
            assert_eq!(fread(ptr::null_mut(), 0, usize::MAX, handle.0), 0);
            assert_eq!(feof(handle.0), 1);
            assert_eq!(
                fread(output.as_mut_ptr().cast(), usize::MAX, 2, handle.0),
                0
            );
            assert_eq!(ferror(handle.0), 1);
        }
    }

    #[test]
    fn interrupted_and_short_reads_are_retried_without_becoming_eof() {
        struct ShortReader {
            interrupted: bool,
            bytes: std::io::Cursor<Vec<u8>>,
        }
        impl Read for ShortReader {
            fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
                if !self.interrupted {
                    self.interrupted = true;
                    return Err(io::ErrorKind::Interrupted.into());
                }
                self.bytes.read(&mut buffer[..1])
            }
        }
        let reader = ShortReader {
            interrupted: false,
            bytes: std::io::Cursor::new(b"abc".to_vec()),
        };
        let mut stream = Stream::new(
            Backing::TestReader(Box::new(reader)),
            true,
            false,
            false,
            Buffering::Full,
        );
        let mut bytes = [0; 3];
        assert_eq!(stream.read_bytes(&mut bytes), 3);
        assert_eq!(&bytes, b"abc");
        assert!(!stream.eof);
        assert!(!stream.error);
        assert_eq!(stream.read_bytes(&mut bytes[..1]), 0);
        assert!(stream.eof);
    }

    #[test]
    fn gzip_transfers_the_existing_file_and_validates_the_footer() {
        let handle = temporary();
        let input = b"owned gzip stream";
        unsafe {
            let fd = fileno(handle.0);
            assert_eq!(gzip_open(handle.0, c"wb".as_ptr()), handle.0);
            assert_eq!(gzsetparams(handle.0, 1, 0), 0);
            assert_eq!(
                gzwrite(handle.0, input.as_ptr().cast(), input.len() as u32),
                input.len() as i32
            );
            assert_eq!(fileno(handle.0), fd);
            assert_eq!(fflush(handle.0), 0);
            let Backing::GzipEncoder(encoder) =
                std::mem::replace(&mut (*handle.0).backing, Backing::Closed)
            else {
                panic!()
            };
            let mut file = encoder.finish().unwrap();
            file.seek(SeekFrom::Start(0)).unwrap();
            let mut compressed = Vec::new();
            file.read_to_end(&mut compressed).unwrap();
            let mut decoder = MultiGzDecoder::new(compressed.as_slice());
            let mut decoded = Vec::new();
            decoder.read_to_end(&mut decoded).unwrap();
            assert_eq!(decoded, input);
            let last = compressed.len() - 8;
            compressed[last] ^= 1;
            let mut decoder = MultiGzDecoder::new(compressed.as_slice());
            assert!(decoder.read_to_end(&mut Vec::new()).is_err());
            (*handle.0).backing = Backing::File(file);
        }
    }

    #[test]
    fn full_gzip_buffers_do_not_sync_flush_each_compression_chunk() {
        let handle = temporary();
        let input: Vec<u8> = (0..BUFFER_SIZE * 40 + 17)
            .map(|index| (index % 251) as u8)
            .collect();
        let mut expected = GzEncoder::new(Vec::new(), Compression::default());
        for chunk in input.chunks(BUFFER_SIZE) {
            expected.write_all(chunk).unwrap();
        }
        let expected = expected.finish().unwrap();
        unsafe {
            assert_eq!(gzip_open(handle.0, c"wb".as_ptr()), handle.0);
            assert_eq!(
                gzwrite(handle.0, input.as_ptr().cast(), input.len() as u32),
                input.len() as i32
            );
            let fd = libc::dup(fileno(handle.0));
            assert!(fd >= 0);
            let mut file = File::from_raw_fd(fd);
            assert_eq!(gzclose(handle.0), 0);
            std::mem::forget(handle); // The stream's descriptor closed once.
            file.seek(SeekFrom::Start(0)).unwrap();
            let mut actual = Vec::new();
            file.read_to_end(&mut actual).unwrap();
            assert_eq!(actual.len(), expected.len());
            assert_eq!(actual, expected);
            let mut decoder = MultiGzDecoder::new(actual.as_slice());
            let mut decoded = Vec::new();
            decoder.read_to_end(&mut decoded).unwrap();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    fn gzip_reader_error_and_close_status_match_the_existing_rust_zlib_adapter() {
        let input = b"gzip footer checks";
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(input).unwrap();
        let compressed = encoder.finish().unwrap();
        for (corrupt, close_as_stdio) in [(false, false), (true, false), (true, true)] {
            let mut bytes = compressed.clone();
            if corrupt {
                let crc = bytes.len() - 8;
                bytes[crc] ^= 1;
            }
            let rust = temporary();
            let oracle_file = temporary();
            unsafe {
                assert_eq!(
                    fwrite(bytes.as_ptr().cast(), 1, bytes.len(), rust.0),
                    bytes.len()
                );
                assert_eq!(
                    fwrite(bytes.as_ptr().cast(), 1, bytes.len(), oracle_file.0),
                    bytes.len()
                );
                rewind(rust.0);
                rewind(oracle_file.0);
                let oracle_fd = libc::dup(fileno(oracle_file.0));
                assert!(oracle_fd >= 0);
                let oracle = crate::zlib::gzdopen(oracle_fd, c"rb".as_ptr());
                assert!(!oracle.is_null());
                assert_eq!(gzip_open(rust.0, c"rb".as_ptr()), rust.0);
                let mut actual = [0u8; 64];
                let mut expected = [0u8; 64];
                for _ in 0..3 {
                    let actual_count =
                        gzread(rust.0, actual.as_mut_ptr().cast(), actual.len() as u32);
                    let expected_count = crate::zlib::gzread(
                        oracle,
                        expected.as_mut_ptr().cast(),
                        expected.len() as u32,
                    );
                    assert_eq!(actual_count, expected_count, "corrupt={corrupt}");
                    if actual_count > 0 {
                        assert_eq!(
                            &actual[..actual_count as usize],
                            &expected[..expected_count as usize]
                        );
                    }
                }
                let closed = if close_as_stdio {
                    fclose(rust.0)
                } else {
                    gzclose(rust.0)
                };
                assert_eq!(closed, crate::zlib::gzclose(oracle));
                std::mem::forget(rust); // gzclose consumed the owned handle.
            }
        }
    }

    #[test]
    fn transparent_gzip_reader_preserves_descriptor_and_file_metadata() {
        let handle = temporary();
        unsafe {
            assert_eq!(fwrite(b"plain format".as_ptr().cast(), 1, 12, handle.0), 12);
            rewind(handle.0);
            let fd = fileno(handle.0);
            let before = (*handle.0).metadata().unwrap();
            assert_eq!(gzip_open(handle.0, c"rb".as_ptr()), handle.0);
            assert_eq!(fileno(handle.0), fd);
            let after = (*handle.0).metadata().unwrap();
            use std::os::unix::fs::MetadataExt;
            assert_eq!(
                (before.dev(), before.ino(), before.len()),
                (after.dev(), after.ino(), after.len())
            );
            let mut bytes = [0u8; 32];
            assert_eq!(
                gzread(handle.0, bytes.as_mut_ptr().cast(), bytes.len() as u32),
                12
            );
            assert_eq!(&bytes[..12], b"plain format");
        }
    }

    #[test]
    fn read_pipe_close_reports_wait_status() {
        let stream = unsafe { popen(c"printf 'pipe bytes'; exit 7".as_ptr(), c"r".as_ptr()) };
        assert!(!stream.is_null());
        let mut bytes = [0; 32];
        assert_eq!(
            unsafe { fread(bytes.as_mut_ptr().cast(), 1, bytes.len(), stream) },
            10
        );
        assert_eq!(&bytes[..10], b"pipe bytes");
        assert_eq!(unsafe { pclose(stream) }, 7 << 8);
    }

    proptest! {
        #[test]
        fn partial_writer_counts_never_include_pending_bytes_from_an_earlier_call(
            old in prop::collection::vec(any::<u8>(), 0..8),
            mut new in prop::collection::vec(any::<u8>(), 1..65),
            limit in 0usize..80,
            chunk in 1usize..12,
            element_size in 1usize..9,
        ) {
            let state = Rc::new(RefCell::new(WriterState { limit: Some(limit), chunk, ..Default::default() }));
            let mut stream = measured_writer(state.clone(), 8);
            prop_assert_eq!(stream.write_bytes(&old), old.len());
            new.truncate(new.len() / element_size * element_size);
            let accepted = unsafe { fwrite(new.as_ptr().cast(), element_size, new.len() / element_size, &mut stream) };
            let expected = if stream.error {
                state.borrow().bytes.len().saturating_sub(old.len()).min(new.len())
            } else { new.len() };
            prop_assert_eq!(accepted, expected / element_size);
            prop_assert!(stream.output.len() <= 8);
            state.borrow_mut().limit = None;
            stream.flush_output().unwrap();
            let written = state.borrow();
            let mut all = old;
            all.extend_from_slice(&new);
            prop_assert_eq!(&written.bytes, &all[..written.bytes.len()]);
        }

        #[test]
        fn buffered_reads_seeks_and_pushback_match_payloads(
            input in prop::collection::vec(any::<u8>(), 0..16385),
            operations in prop::collection::vec((0usize..20000, 1usize..65, 0usize..65, any::<bool>()), 1..32),
        ) {
            let handle = temporary();
            prop_assert_eq!(unsafe { fwrite(input.as_ptr().cast(), 1, input.len(), handle.0) }, input.len());
            for (position, size, count, push) in operations {
                prop_assert_eq!(unsafe { fseeko(handle.0, position as i64, libc::SEEK_SET) }, 0);
                let requested = size * count;
                let available = input.len().saturating_sub(position);
                let consumed = requested.min(available);
                let mut output = vec![0; requested];
                let elements = unsafe { fread(output.as_mut_ptr().cast(), size, count, handle.0) };
                prop_assert_eq!(elements, consumed / size);
                if consumed > 0 { prop_assert_eq!(&output[..consumed], &input[position..position + consumed]); }
                prop_assert_eq!(unsafe { ftello(handle.0) }, (position + consumed) as i64);
                prop_assert_eq!(unsafe { feof(handle.0) }, (requested > available) as i32);
                if push && consumed > 0 {
                    let byte = output[consumed - 1];
                    prop_assert_eq!(unsafe { ungetc(byte as i32, handle.0) }, byte as i32);
                    prop_assert_eq!(unsafe { feof(handle.0) }, 0);
                    prop_assert_eq!(unsafe { fgetc(handle.0) }, byte as i32);
                }
            }
        }
    }
}

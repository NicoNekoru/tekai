// Copyright 2016-2022 the Tectonic Project
// Licensed under the MIT License.

//! Rust-only I/O context for Tectonic's Rust BibTeX engine.
//!
//! This local adaptation retains the handle and driver APIs that BibTeX uses.
//! Each invocation owns its input and output handles. No C callbacks, global
//! raw state pointer, native archive, or setjmp/longjmp machinery is provided.
//! See the package README for exact upstream provenance.

use flate2::{read::GzDecoder, Compression, GzBuilder};
use std::{
    fmt::{Display, Error as FmtError, Formatter},
    io::{Read, Write},
    num::NonZeroUsize,
    path::PathBuf,
    result::Result as StdResult,
    sync::Mutex,
};
use tectonic_errors::prelude::*;
use tectonic_io_base::{
    digest::DigestData, normalize_tex_path, InputFeatures, InputHandle, IoProvider, OpenResult,
    OutputHandle,
};
use tectonic_status_base::{tt_warning, StatusBackend};

/// The ID of an InputHandle, used for Rust core state
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct InputId(NonZeroUsize);

impl InputId {
    fn new(idx: usize) -> Option<Self> {
        NonZeroUsize::new(idx).map(InputId)
    }

    fn idx(self) -> usize {
        self.0.get() - 1
    }
}

/// The ID of an OutputHandle, used for Rust core state
#[derive(Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct OutputId(NonZeroUsize);

impl OutputId {
    fn new(idx: usize) -> Option<Self> {
        NonZeroUsize::new(idx).map(OutputId)
    }

    fn idx(self) -> usize {
        self.0.get() - 1
    }
}

/// Possible failures for "system request" calls to the driver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemRequestError {
    /// The driver does not implement this system request.
    NotImplemented,

    /// The driver implements this system request but is not allowing it
    /// to be used in this circumstance.
    NotAllowed,

    /// The driver tried to execute this system request but it failed.
    Failed,
}

impl Display for SystemRequestError {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), FmtError> {
        write!(
            f,
            "{}",
            match self {
                SystemRequestError::NotImplemented => "not implemented by this driver",
                SystemRequestError::NotAllowed => "not allowed by this driver",
                SystemRequestError::Failed => "execution of the request failed",
            }
        )
    }
}

/// The DriverHooks trait allows engines to interact with the higher-level code
/// that is driving the TeX processing.
///
/// Drivers mainly manage interactions between the engines and the outside
/// world. The primary way they do this is by exposing an [`IoProvider`]
/// implementation.
///
/// Drivers can also implement handlers for additional events that help it track
/// the input and output access patterns of the engines. The CLI program needs
/// these to intelligently decide when to rerun the TeX engine, to choose which
/// files to actually save to disk, and to emit Makefile rules describing the
/// dependency of the outputs on the inputs. The relevant trait methods have
/// default implementations that do nothing.
pub trait DriverHooks {
    /// Get the main I/O implementations of this driver.
    fn io(&mut self) -> &mut dyn IoProvider;

    /// This function is called when an output file is closed. The "digest"
    /// argument specifies the cryptographic digest of the data that were
    /// written. Note that this function takes ownership of the name and
    /// digest.
    fn event_output_closed(&mut self, _name: String, _digest: DigestData) {}

    /// This function is called when an input file is closed. The "digest"
    /// argument specifies the cryptographic digest of the data that were
    /// read, if available. This digest is not always available, if the engine
    /// used seeks while reading the file. Note that this function takes
    /// ownership of the name and digest.
    fn event_input_closed(
        &mut self,
        _name: String,
        _digest: Option<DigestData>,
        _status: &mut dyn StatusBackend,
    ) {
    }

    /// The engine is requesting a "shell escape" evaluation.
    ///
    /// If the driver wishes to implement this request, it should run the
    /// specified command using the OS’s default shell. Relevant files should be
    /// available in the command's working directory, and if the command creates
    /// any files, they should be incorporated into the I/O environment. The
    /// shell-escape environment should persist across multiple invocations of
    /// this system request, because some packages run a series of commands that
    /// assume such persistence. Also note that the command text has to be
    /// evaluated through a shell, not just with `exec()`, since shell features
    /// such as redirections might be used. This is therefore a wildly insecure
    /// feature.
    ///
    /// Detailed error information should be logged or stored inside the hook.
    fn sysrq_shell_escape(
        &mut self,
        _command: &str,
        _status: &mut dyn StatusBackend,
    ) -> StdResult<(), SystemRequestError> {
        Err(SystemRequestError::NotImplemented)
    }
}

/// This type provides a minimal [`DriverHooks`] implementation.
#[derive(Clone, Debug, Default)]
pub struct MinimalDriver<T: IoProvider>(T);

impl<T: IoProvider> MinimalDriver<T> {
    /// Create a new minimal driver.
    pub fn new(io: T) -> Self {
        MinimalDriver(io)
    }
}

impl<T: IoProvider> DriverHooks for MinimalDriver<T> {
    fn io(&mut self) -> &mut dyn IoProvider {
        &mut self.0
    }
}

/// Create an invocation-scoped Rust I/O context for BibTeX.
pub struct CoreBridgeLauncher<'a> {
    hooks: &'a mut dyn DriverHooks,
    status: &'a mut dyn StatusBackend,
}

impl<'a> CoreBridgeLauncher<'a> {
    /// Create a launcher with the supplied I/O and status providers.
    pub fn new(hooks: &'a mut dyn DriverHooks, status: &'a mut dyn StatusBackend) -> Self {
        Self { hooks, status }
    }

    /// Run a Rust callback with a fresh I/O context and the upstream engine lock.
    ///
    /// Errors retain their original text and chain. A callback panic drops its
    /// state and releases the lock, and does not prevent a later invocation.
    pub fn with_global_lock<F, T>(&mut self, callback: F) -> Result<T>
    where
        F: FnOnce(&mut CoreBridgeState<'_>) -> Result<T>,
    {
        static ENGINE_LOCK: Mutex<()> = Mutex::new(());
        let _guard = ENGINE_LOCK
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        let mut state = CoreBridgeState {
            hooks: self.hooks,
            status: self.status,
            input_handles: Vec::new(),
            output_handles: Vec::new(),
        };
        callback(&mut state)
    }
}

/// Input and output handles belonging to one Rust engine invocation.
pub struct CoreBridgeState<'a> {
    hooks: &'a mut dyn DriverHooks,
    status: &'a mut dyn StatusBackend,
    input_handles: Vec<Option<InputHandle>>,
    output_handles: Vec<Option<OutputHandle>>,
}

impl CoreBridgeState<'_> {
    fn input_open_name_format(
        &mut self,
        name: &str,
        format: FileFormat,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        let io = self.hooks.io();

        if let FileFormat::Format = format {
            match io.input_open_format(name, self.status) {
                OpenResult::NotAvailable => {}
                OpenResult::Err(e) => return OpenResult::Err(e),
                OpenResult::Ok(h) => return OpenResult::Ok((h, None)),
            }
        } else {
            match io.input_open_name_with_abspath(name, self.status) {
                OpenResult::NotAvailable => {}
                r => return r,
            }
        }

        // It wasn't available under the immediately-given name. Try adding
        // extensions. Note that we always add a new extension here, even if the
        // filename already has one. E.g., lipsum in TeXLive 2020 asks for
        // `lipsum.ltd.tex` under the name `lipsum.ltd`.

        for e in format.extensions() {
            let ext = format!("{name}.{e}");

            if let FileFormat::Format = format {
                match io.input_open_format(&ext, self.status) {
                    OpenResult::NotAvailable => {}
                    OpenResult::Err(e) => return OpenResult::Err(e),
                    OpenResult::Ok(h) => return OpenResult::Ok((h, None)),
                }
            } else {
                match io.input_open_name_with_abspath(&ext, self.status) {
                    OpenResult::NotAvailable => {}
                    r => return r,
                }
            }
        }

        OpenResult::NotAvailable
    }

    fn input_open_name_format_gz(
        &mut self,
        name: &str,
        format: FileFormat,
        is_gz: bool,
    ) -> OpenResult<(InputHandle, Option<PathBuf>)> {
        let base = self.input_open_name_format(name, format);

        if !is_gz {
            return base;
        }

        match base {
            OpenResult::Ok((ih, path)) => {
                let origin = ih.origin();
                let dr = GzDecoder::new(ih.into_inner());

                OpenResult::Ok((InputHandle::new(name, dr, origin), path))
            }
            _ => base,
        }
    }

    /// Get a mutable reference to an open output.
    pub fn get_output(&mut self, id: OutputId) -> &mut OutputHandle {
        self.output_handles[id.idx()].as_mut().unwrap()
    }

    /// Open an output, optionally gzip-compressed.
    pub fn output_open(&mut self, name: &str, is_gz: bool) -> Option<OutputId> {
        let io = self.hooks.io();
        let name = normalize_tex_path(name);

        let mut oh = match io.output_open_name(&name) {
            OpenResult::Ok(oh) => oh,
            OpenResult::NotAvailable => return None,
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of output {} failed", name; e);
                return None;
            }
        };

        if is_gz {
            let name = oh.name().to_owned();
            oh = OutputHandle::new(
                name,
                GzBuilder::new().write(oh.into_inner(), Compression::default()),
            );
        }

        self.output_handles.push(Some(oh));
        OutputId::new(self.output_handles.len())
    }

    /// Open the provider's stdout output.
    pub fn output_open_stdout(&mut self) -> Option<OutputId> {
        let io = self.hooks.io();

        let oh = match io.output_open_stdout() {
            OpenResult::Ok(oh) => oh,
            OpenResult::NotAvailable => return None,
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of stdout failed"; e);
                return None;
            }
        };

        self.output_handles.push(Some(oh));
        OutputId::new(self.output_handles.len())
    }

    /// Close an output and notify the provider. Returns true on a flush error.
    pub fn output_close(&mut self, id: OutputId) -> bool {
        let mut rv = false;
        let mut oh = self.output_handles[id.idx()].take().unwrap();
        if let Err(e) = oh.flush() {
            tt_warning!(self.status, "error when closing output {}", oh.name(); e.into());
            rv = true;
        }
        let (name, digest) = oh.into_name_digest();
        self.hooks.event_output_closed(name, digest);
        rv
    }

    /// Get a mutable reference to an open input.
    pub fn get_input(&mut self, input: InputId) -> &mut InputHandle {
        self.input_handles[input.idx()].as_mut().unwrap()
    }

    /// Open an input using the provider and the format's fallback extensions.
    pub fn input_open(&mut self, name: &str, format: FileFormat, is_gz: bool) -> Option<InputId> {
        let name = normalize_tex_path(name);

        let (ih, _path) = match self.input_open_name_format_gz(&name, format, is_gz) {
            OpenResult::Ok(tup) => tup,
            OpenResult::NotAvailable => {
                return None;
            }
            OpenResult::Err(e) => {
                tt_warning!(self.status, "open of input {} failed", name; e);
                return None;
            }
        };

        self.input_handles.push(Some(ih));
        InputId::new(self.input_handles.len())
    }

    /// Read the size of an input. Returns zero and reports a warning on error.
    pub fn input_get_size(&mut self, handle: InputId) -> usize {
        let rhandle: &mut InputHandle = self.get_input(handle);

        match rhandle.get_size() {
            Ok(s) => s,
            Err(e) => {
                tt_warning!(self.status, "failed to get the size of an input"; e);
                0
            }
        }
    }

    /// Read exactly the requested bytes from an input.
    pub fn input_read(&mut self, handle: InputId, buf: &mut [u8]) -> Result<()> {
        let rhandle: &mut InputHandle = self.get_input(handle);
        rhandle.read_exact(buf).map_err(Error::from)
    }

    /// Close an input and notify the provider. Returns true on a read error.
    pub fn input_close(&mut self, id: InputId) -> bool {
        let mut ih = self.input_handles[id.idx()].take().unwrap();
        let mut rv = false;

        if let Err(e) = ih.scan_remainder() {
            tt_warning!(self.status, "error closing out input {}", ih.name(); e);
            rv = true;
        }

        let (name, digest_opt) = ih.into_name_digest();
        self.hooks.event_input_closed(name, digest_opt, self.status);
        rv
    }
}

/// Different types of files that can be opened by TeX engines
///
/// This enumeration is used to guess filename extensions to try when looking
/// for a file to open.
///
#[derive(Clone, Copy, Debug)]
pub enum FileFormat {
    /// An Adobe Font Metrics file.
    Afm = 4,

    /// A BibTeX bibliography data file.
    Bib = 6,

    /// A BibTeX style file.
    Bst = 7,

    /// A character map data file.
    Cmap = 45,

    /// A configuration file.
    Cnf = 8,

    /// An encoding data file.
    Enc = 44,

    /// A TeX "format" file.
    Format = 10,

    /// A font-map file.
    FontMap = 11,

    /// A miscellaneous font file.
    MiscFonts = 41,

    /// An OFM font metrics file.
    Ofm = 20,

    /// An OpenType font file.
    OpenType = 47,

    /// An OVF file.
    Ovf = 23,

    /// An image file.
    Pict = 25,

    /// A PK font file.
    Pk = 1,

    /// A general program data file.
    ProgramData = 39,

    /// An SFD file.
    Sfd = 46,

    /// The Tectonic primary input file.
    TectonicPrimary = 59,

    /// A TeX language file.
    Tex = 26,

    /// A TeX PostScript header file.
    TexPsHeader = 30,

    /// A TeX Font Metrics file.
    Tfm = 3,

    /// A TrueType font file.
    TrueType = 36,

    /// A Type1 font file.
    Type1 = 32,

    /// A Virtual Font file.
    Vf = 33,
}

impl FileFormat {
    fn extensions(&self) -> &[&str] {
        match *self {
            FileFormat::Afm => &["afm"],
            FileFormat::Bib => &["bib"],
            FileFormat::Bst => &["bst"],
            FileFormat::Cmap => &[],
            FileFormat::Cnf => &["cnf"],
            FileFormat::Enc => &["enc"],
            FileFormat::Format => &["fmt"],
            FileFormat::FontMap => &["map"],
            FileFormat::MiscFonts => &[],
            FileFormat::Ofm => &["ofm"],
            FileFormat::OpenType => &["otf", "OTF"],
            FileFormat::Ovf => &["ovf", "vf"],
            FileFormat::Pict => &["pdf", "jpg", "eps", "epsi"],
            FileFormat::Pk => &["pk"],
            FileFormat::ProgramData => &[],
            FileFormat::Sfd => &["sfd"],
            FileFormat::TectonicPrimary => &[],
            FileFormat::Tex => &["tex", "sty", "cls", "fd", "aux", "bbl", "def", "clo", "ldf"],
            FileFormat::TexPsHeader => &["pro"],
            FileFormat::Tfm => &["tfm"],
            FileFormat::TrueType => &["ttf", "ttc", "TTF", "TTC", "dfont"],
            FileFormat::Type1 => &["pfa", "pfb"],
            FileFormat::Vf => &["vf"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::{collections::HashMap, io::Cursor, sync::Arc};
    use tectonic_io_base::InputOrigin;
    use tectonic_status_base::NoopStatusBackend;

    #[derive(Default)]
    struct MemoryIo {
        files: HashMap<String, Vec<u8>>,
        attempts: Arc<Mutex<Vec<String>>>,
    }

    impl IoProvider for MemoryIo {
        fn input_open_name(
            &mut self,
            name: &str,
            _status: &mut dyn StatusBackend,
        ) -> OpenResult<InputHandle> {
            self.attempts.lock().unwrap().push(name.to_owned());
            match self.files.get(name) {
                Some(bytes) => OpenResult::Ok(InputHandle::new(
                    name,
                    Cursor::new(bytes.clone()),
                    InputOrigin::Other,
                )),
                None => OpenResult::NotAvailable,
            }
        }
    }

    #[test]
    fn callback_errors_keep_their_chain_and_next_run_has_fresh_handles() {
        let io = MemoryIo {
            files: HashMap::from([("db.bib".into(), b"database".to_vec())]),
            ..MemoryIo::default()
        };
        let mut driver = MinimalDriver::new(io);
        let mut status = NoopStatusBackend {};
        let mut launcher = CoreBridgeLauncher::new(&mut driver, &mut status);
        let result: Result<()> = launcher.with_global_lock(|state| {
            assert!(state.input_open("db", FileFormat::Bib, false).is_some());
            Err(anyhow!("original failure").context("callback context"))
        });
        assert_eq!(
            format!("{:#}", result.unwrap_err()),
            "callback context: original failure"
        );
        launcher
            .with_global_lock(|state| {
                assert!(state.input_handles.is_empty());
                assert!(state.output_handles.is_empty());
                let input = state.input_open("db", FileFormat::Bib, false).unwrap();
                assert!(!state.input_close(input));
                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn callback_panic_drops_state_and_does_not_disable_later_runs() {
        let mut driver = MinimalDriver::new(MemoryIo::default());
        let mut status = NoopStatusBackend {};
        let mut launcher = CoreBridgeLauncher::new(&mut driver, &mut status);
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _: Result<()> = launcher.with_global_lock(|_| panic!("callback panic"));
        }));
        assert!(panic.is_err());
        assert_eq!(launcher.with_global_lock(|_| Ok(17)).unwrap(), 17);
    }

    #[test]
    fn format_fallback_preserves_requested_name_and_extension_order() {
        let io = MemoryIo {
            files: HashMap::from([("style.bst".into(), b"style bytes".to_vec())]),
            ..MemoryIo::default()
        };
        let attempts = io.attempts.clone();
        let mut driver = MinimalDriver::new(io);
        let mut status = NoopStatusBackend {};
        CoreBridgeLauncher::new(&mut driver, &mut status)
            .with_global_lock(|state| {
                let input = state.input_open("style", FileFormat::Bst, false).unwrap();
                let mut bytes = Vec::new();
                state.get_input(input).read_to_end(&mut bytes)?;
                assert_eq!(bytes, b"style bytes");
                assert!(!state.input_close(input));
                Ok(())
            })
            .unwrap();
        assert_eq!(*attempts.lock().unwrap(), ["style", "style.bst"]);
    }

    #[test]
    fn input_provider_errors_do_not_try_lower_priority_extensions() {
        struct FailingIo;
        impl IoProvider for FailingIo {
            fn input_open_name(
                &mut self,
                name: &str,
                _status: &mut dyn StatusBackend,
            ) -> OpenResult<InputHandle> {
                assert_eq!(name, "blocked");
                OpenResult::Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied).into())
            }
        }
        let mut driver = MinimalDriver::new(FailingIo);
        let mut status = NoopStatusBackend {};
        CoreBridgeLauncher::new(&mut driver, &mut status)
            .with_global_lock(|state| {
                assert!(state
                    .input_open("blocked", FileFormat::Tex, false)
                    .is_none());
                Ok(())
            })
            .unwrap();
    }

    proptest! {
        #[test]
        fn rust_gzip_input_roundtrips_arbitrary_bytes(bytes in prop::collection::vec(any::<u8>(), 0..4096)) {
            let mut encoder = GzBuilder::new().write(Vec::new(), Compression::default());
            encoder.write_all(&bytes).unwrap();
            let compressed = encoder.finish().unwrap();
            let io = MemoryIo {
                files: HashMap::from([("data.bib".into(), compressed)]),
                ..MemoryIo::default()
            };
            let mut driver = MinimalDriver::new(io);
            let mut status = NoopStatusBackend {};
            let actual = CoreBridgeLauncher::new(&mut driver, &mut status)
                .with_global_lock(|state| {
                    let input = state.input_open("data", FileFormat::Bib, true).unwrap();
                    let mut actual = Vec::new();
                    state.get_input(input).read_to_end(&mut actual)?;
                    assert!(!state.input_close(input));
                    Ok(actual)
                }).unwrap();
            prop_assert_eq!(actual, bytes);
        }
    }
}

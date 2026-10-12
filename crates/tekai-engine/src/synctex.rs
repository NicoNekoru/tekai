//! Rust-owned SyncTeX writer for the embedded pdfTeX engine.
//!
//! The generated core already contains the standard pdfTeX SyncTeX hook
//! points and stores source tags and line numbers in shipped nodes. This
//! module implements those hooks without linking TeX Live's C SyncTeX
//! runtime.

use std::cell::{Cell, RefCell};
use std::ffi::{CString, OsStr};
use std::fs::{self, File};
use std::io::{self, Write as _};
use std::marker::{PhantomData, PhantomPinned};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::Rc;

use flate2::write::GzEncoder;
use flate2::Compression;

type Integer = core::ffi::c_int;
type Boolean = core::ffi::c_int;
type Halfword = core::ffi::c_int;

const SYNCTEX_NO_OPTION: Integer = Integer::MAX;
const BOX_TAG_OFFSET: Halfword = 7;
const BOX_LINE_OFFSET: Halfword = 8;
const MEDIUM_TAG_OFFSET: Halfword = 2;
const MEDIUM_LINE_OFFSET: Halfword = 3;
const RULE_TAG_OFFSET: Halfword = 4;
const RULE_LINE_OFFSET: Halfword = 5;
const RULE_NODE: Integer = 2;
const GLUE_NODE: Integer = 10;
const KERN_NODE: Integer = 11;

#[derive(Default)]
struct SyncTexContext {
    source: String,
    tag_counter: Integer,
    count: usize,
    bytes_since_anchor: usize,
    current_tag: Integer,
    current_line: Integer,
    content_ready: bool,
    off: bool,
    compress: bool,
    not_void: bool,
}

impl SyncTexContext {
    fn new() -> Self {
        let mut context = Self {
            compress: true,
            ..Self::default()
        };
        context.push_text("SyncTeX Version:1\n");
        context
    }

    fn push_text(&mut self, text: &str) {
        self.source.push_str(text);
        self.bytes_since_anchor += text.len();
    }

    fn push_record(&mut self, record: &str) {
        self.push_text(record);
        self.count += 1;
    }

    fn push_anchor(&mut self) {
        let anchor = format!("!{}\n", self.bytes_since_anchor);
        self.source.push_str(&anchor);
        self.bytes_since_anchor = anchor.len();
        self.count += 1;
    }

    fn add_input(&mut self, tag: Integer, name: String) {
        self.push_text(&format!("Input:{tag}:{name}\n"));
    }

    fn prepare_content(&mut self, magnification: Integer) {
        if self.content_ready {
            return;
        }
        self.compress = synctex_value() >= 0;
        let magnification = if magnification > 0 {
            magnification
        } else {
            1000
        };
        let settings = format!(
            "Output:pdf\nMagnification:{magnification}\nUnit:1\nX Offset:0\nY Offset:0\nContent:\n"
        );
        self.push_text(&settings);
        self.content_ready = true;
    }

    fn begin_sheet(&mut self, page: Integer) {
        self.push_anchor();
        self.push_record(&format!("{{{page}\n"));
    }

    fn end_sheet(&mut self, page: Integer) {
        self.push_anchor();
        self.push_record(&format!("}}{page}\n"));
    }

    fn finish(mut self) -> io::Result<Option<PathBuf>> {
        let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
            unsafe { crate::generated::pdftexextra::state() };

        if !self.content_ready || !self.not_void {
            return Ok(None);
        }

        self.push_anchor();
        self.push_text("Postamble:\n");
        self.push_text(&format!("Count:{}\n", self.count));
        self.push_anchor();
        self.push_text("Post scriptum:\n");

        let Some(path) = tex_path(unsafe { (*__tekai_state_generated_pdftexextra).texmflogname })
        else {
            return Ok(None);
        };
        let mut path = crate::working_directory::resolve(&path)?;
        if self.compress {
            path.set_extension("synctex.gz");
            let mut stale = path.clone();
            stale.set_extension("synctex");
            remove_if_exists(&stale)?;

            let file = File::create(&path)?;
            let mut encoder = GzEncoder::new(file, Compression::fast());
            encoder.write_all(self.source.as_bytes())?;
            encoder.finish()?;
        } else {
            path.set_extension("synctex");
            let mut stale = path.clone();
            stale.set_extension("synctex.gz");
            remove_if_exists(&stale)?;
            fs::write(&path, self.source.as_bytes())?;
        }
        record_output(&path);
        Ok(Some(path))
    }
}

pub(crate) struct SyncTexSession {
    context: RefCell<Option<SyncTexContext>>,
    _thread: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

thread_local! {
    static ACTIVE: Cell<Option<NonNull<SyncTexSession>>> = const { Cell::new(None) };
}

impl SyncTexSession {
    pub(crate) fn new() -> Pin<Box<Self>> {
        Box::pin(Self {
            context: RefCell::new(None),
            _thread: PhantomData,
            _pinned: PhantomPinned,
        })
    }

    pub(crate) fn scope<T>(session: Pin<&mut Self>, action: impl FnOnce() -> T) -> T {
        struct Unregister(NonNull<SyncTexSession>);
        impl Drop for Unregister {
            fn drop(&mut self) {
                let _ = ACTIVE.try_with(|active| {
                    if active.get() == Some(self.0) {
                        active.set(None);
                    }
                });
                // Discard unfinished text. Only synctexterminate writes the
                // postamble/sidecar, never this guard or the owner's Drop.
                unsafe { self.0.as_ref() }.context.borrow_mut().take();
            }
        }
        // SAFETY The owner stays pinned and exclusively borrowed through the
        // private guard. Dropping unfinished context discards text, not output.
        let pointer = NonNull::from(unsafe { session.get_unchecked_mut() });
        ACTIVE.with(|active| {
            assert!(active.get().is_none(), "nested SyncTeX scope");
            active.set(Some(pointer));
        });
        let _guard = Unregister(pointer);
        action()
    }
}

fn with_context(action: impl FnOnce(&mut SyncTexContext)) {
    ACTIVE.with(|active| {
        if let Some(pointer) = active.get() {
            // SAFETY Scope retains the owner; no context reference escapes.
            let mut context = unsafe { pointer.as_ref() }.context.borrow_mut();
            action(context.get_or_insert_with(SyncTexContext::new));
        }
    });
}

/// Reset per-run state before the generated engine parses command-line flags.
pub unsafe fn reset_for_run() {
    ACTIVE.with(|active| {
        if let Some(pointer) = active.get() {
            *unsafe { pointer.as_ref() }.context.borrow_mut() = Some(SyncTexContext::new());
        }
    });
}

fn remove_if_exists(path: &PathBuf) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn record_output(path: &PathBuf) {
    if let Ok(path) = CString::new(path.as_os_str().as_bytes()) {
        unsafe {
            crate::openclose::recorder_record_output(path.as_ptr());
        }
    }
}

fn tex_value<T>(number: Integer, convert: impl FnOnce(&[u8]) -> T) -> Option<T> {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        if (*__tekai_state_generated_pdftexextra).strstart.is_null()
            || (*__tekai_state_generated_pdftexextra).strpool.is_null()
            || number < 0
            || number >= (*__tekai_state_generated_pdftexextra).strptr
        {
            return None;
        }
        let start = *(*__tekai_state_generated_pdftexextra)
            .strstart
            .offset(number as isize);
        let end = *(*__tekai_state_generated_pdftexextra)
            .strstart
            .offset((number + 1) as isize);
        if start < 0 || end < start || end > (*__tekai_state_generated_pdftexextra).poolptr {
            return None;
        }
        let bytes = core::slice::from_raw_parts(
            (*__tekai_state_generated_pdftexextra)
                .strpool
                .offset(start as isize),
            (end - start) as usize,
        );
        Some(convert(bytes))
    }
}

fn tex_path(number: Integer) -> Option<PathBuf> {
    tex_value(number, |bytes| PathBuf::from(OsStr::from_bytes(bytes)))
}

fn tex_string(number: Integer) -> Option<String> {
    tex_value(number, |bytes| String::from_utf8_lossy(bytes).into_owned())
}

fn current_input_name() -> Option<String> {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        if (*__tekai_state_generated_pdftexextra)
            .fullsourcefilenamestack
            .is_null()
            || (*__tekai_state_generated_pdftexextra).inopen < 0
        {
            return None;
        }
        let name = tex_string(
            *(*__tekai_state_generated_pdftexextra)
                .fullsourcefilenamestack
                .offset((*__tekai_state_generated_pdftexextra).inopen as isize),
        )?;
        let path = crate::working_directory::resolve(&PathBuf::from(&name)).ok()?;
        Some(
            fs::canonicalize(&path)
                .unwrap_or(path)
                .to_string_lossy()
                .into_owned(),
        )
    }
}

fn synctex_value() -> Integer {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        if (*__tekai_state_generated_pdftexextra).zeqtb.is_null()
            || (*__tekai_state_generated_pdftexextra).synctexoffset <= 0
        {
            return 0;
        }
        (*(*__tekai_state_generated_pdftexextra)
            .zeqtb
            .offset((*__tekai_state_generated_pdftexextra).synctexoffset as isize))
        .u
        .CINT
    }
}

unsafe fn set_synctex_value(value: Integer) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        if !(*__tekai_state_generated_pdftexextra).zeqtb.is_null()
            && (*__tekai_state_generated_pdftexextra).synctexoffset > 0
        {
            (*(*__tekai_state_generated_pdftexextra)
                .zeqtb
                .offset((*__tekai_state_generated_pdftexextra).synctexoffset as isize))
            .u
            .CINT = value;
        }
    }
}

fn memory_int(index: Halfword) -> Integer {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        if (*__tekai_state_generated_pdftexextra).zmem.is_null()
            || index < (*__tekai_state_generated_pdftexextra).memmin
            || index > (*__tekai_state_generated_pdftexextra).memmax
        {
            return 0;
        }
        (*(*__tekai_state_generated_pdftexextra)
            .zmem
            .offset(index as isize))
        .u
        .CINT
    }
}

fn node_type(node: Halfword) -> Integer {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        if (*__tekai_state_generated_pdftexextra).zmem.is_null()
            || node < (*__tekai_state_generated_pdftexextra).memmin
            || node > (*__tekai_state_generated_pdftexextra).memmax
        {
            return -1;
        }
        (*(*__tekai_state_generated_pdftexextra)
            .zmem
            .offset(node as isize))
        .hh
        .u
        .B0 as Integer
    }
}

fn node_context(node: Halfword, tag_offset: Halfword, line_offset: Halfword) -> (Integer, Integer) {
    (
        memory_int(node.saturating_add(tag_offset)),
        memory_int(node.saturating_add(line_offset)),
    )
}

fn position() -> (Integer, Integer) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        (
            (*__tekai_state_generated_pdftexextra).curh,
            (*__tekai_state_generated_pdftexextra).curv,
        )
    }
}

fn record_box(node: Halfword, marker: char) {
    unsafe {
        with_context(|context| {
            if context.off || !context.content_ready || synctex_value() == 0 {
                return;
            }
            let (tag, line) = node_context(node, BOX_TAG_OFFSET, BOX_LINE_OFFSET);
            let (h, v) = position();
            let width = memory_int(node.saturating_add(1));
            let depth = memory_int(node.saturating_add(2));
            let height = memory_int(node.saturating_add(3));
            context.current_tag = tag;
            context.current_line = line;
            context.not_void = true;
            context.push_record(&format!(
                "{marker}{tag},{line}:{h},{v}:{width},{height},{depth}\n"
            ));
        });
    }
}

fn close_box(marker: char) {
    unsafe {
        with_context(|context| {
            if context.off || !context.content_ready || synctex_value() == 0 {
                return;
            }
            context.push_record(&format!("{marker}\n"));
        });
    }
}

fn record_medium_node(node: Halfword, marker: char, include_width: bool) {
    unsafe {
        with_context(|context| {
            if context.off || !context.content_ready || synctex_value() == 0 {
                return;
            }
            let (tag, line) = node_context(node, MEDIUM_TAG_OFFSET, MEDIUM_LINE_OFFSET);
            if tag <= 0 || line <= 0 {
                return;
            }
            let (h, v) = position();
            context.current_tag = tag;
            context.current_line = line;
            let width = include_width
                .then(|| format!(":{}", memory_int(node.saturating_add(1))))
                .unwrap_or_default();
            context.push_record(&format!("{marker}{tag},{line}:{h},{v}{width}\n"));
        });
    }
}

fn record_rule(node: Halfword) {
    unsafe {
        with_context(|context| {
            let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
                unsafe { crate::generated::pdftexextra::state() };

            if context.off || !context.content_ready || synctex_value() == 0 {
                return;
            }
            let (tag, line) = node_context(node, RULE_TAG_OFFSET, RULE_LINE_OFFSET);
            if tag <= 0 || line <= 0 {
                return;
            }
            let (h, v) = position();
            context.current_tag = tag;
            context.current_line = line;
            context.push_record(&format!(
                "r{tag},{line}:{h},{v}:{},{},{}\n",
                (*__tekai_state_generated_pdftexextra).rulewd,
                (*__tekai_state_generated_pdftexextra).ruleht,
                (*__tekai_state_generated_pdftexextra).ruledp
            ));
        });
    }
}

pub fn synctexinitcommand() {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    unsafe {
        let option = (*__tekai_state_generated_pdftexextra).synctexoption;
        with_context(|context| {
            if option == SYNCTEX_NO_OPTION {
                set_synctex_value(0);
            } else if option == 0 {
                context.off = true;
                set_synctex_value(0);
            } else {
                context.compress = option >= 0;
                set_synctex_value(option | 1);
            }
        });
    }
}

pub fn synctexabort(_log_opened: Boolean) {
    with_context(|context| {
        context.off = true;
        context.source.clear();
    });
}

pub fn synctexstartinput() {
    unsafe {
        with_context(|context| {
            let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
                unsafe { crate::generated::pdftexextra::state() };

            if context.off {
                (*__tekai_state_generated_pdftexextra)
                    .curinput
                    .synctextagfield = 0;
                return;
            }
            context.tag_counter = context.tag_counter.saturating_add(1);
            let tag = context.tag_counter;
            (*__tekai_state_generated_pdftexextra)
                .curinput
                .synctextagfield = tag;
            if let Some(name) = current_input_name() {
                context.add_input(tag, name);
            }
        });
    }
}

pub fn synctexterminate(log_opened: Boolean) {
    let context = ACTIVE.with(|active| {
        let pointer = active.get()?;
        // SAFETY Scope retains the owner; explicit termination consumes only
        // the context, never the registered session.
        unsafe { pointer.as_ref() }.context.borrow_mut().take()
    });
    if log_opened == 0 {
        return;
    }
    if let Some(context) = context {
        match context.finish() {
            Ok(Some(path)) => println!("\nSyncTeX written on {}.", path.display()),
            Ok(None) => {}
            Err(error) => eprintln!("SyncTeX: failed to write sidecar: {error}"),
        }
    }
}

pub fn synctexsheet(magnification: Integer) {
    unsafe {
        with_context(|context| {
            let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
                unsafe { crate::generated::pdftexextra::state() };

            if context.off || synctex_value() == 0 {
                return;
            }
            context.prepare_content(magnification);
            context.begin_sheet(
                (*__tekai_state_generated_pdftexextra)
                    .totalpages
                    .saturating_add(1),
            );
        });
    }
}

pub fn synctexteehs() {
    unsafe {
        with_context(|context| {
            let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
                unsafe { crate::generated::pdftexextra::state() };

            if context.off || !context.content_ready {
                return;
            }
            context.end_sheet((*__tekai_state_generated_pdftexextra).totalpages);
        });
    }
}

pub fn synctexpdfxform(_node: Halfword) {}

pub fn synctexmrofxfdp() {}

pub fn synctexpdfrefxform(_object_number: Integer) {}

pub fn synctexvlist(node: Halfword) {
    record_box(node, '[');
}

pub fn synctextsilv(_node: Halfword) {
    close_box(']');
}

pub fn synctexvoidvlist(node: Halfword, _parent: Halfword) {
    record_box(node, 'v');
}

pub fn synctexhlist(node: Halfword) {
    record_box(node, '(');
}

pub fn synctextsilh(_node: Halfword) {
    close_box(')');
}

pub fn synctexvoidhlist(node: Halfword, _parent: Halfword) {
    record_box(node, 'h');
}

pub fn synctexmath(node: Halfword, _parent: Halfword) {
    record_medium_node(node, '$', false);
}

pub fn synctexhorizontalruleorglue(node: Halfword, _parent: Halfword) {
    match node_type(node) {
        RULE_NODE => record_rule(node),
        GLUE_NODE => record_medium_node(node, 'g', false),
        KERN_NODE => record_medium_node(node, 'k', true),
        _ => {}
    }
}

pub fn synctexkern(node: Halfword, _parent: Halfword) {
    record_medium_node(node, 'k', true);
}

pub fn synctexchar(_node: Halfword) {}

pub fn synctexnode(_node: Halfword, _parent: Halfword) {}

pub fn synctexcurrent() {
    unsafe {
        with_context(|context| {
            if context.off
                || !context.content_ready
                || context.current_tag <= 0
                || context.current_line <= 0
            {
                return;
            }
            let (h, v) = position();
            context.push_record(&format!(
                "x{},{}:{h},{v}\n",
                context.current_tag, context.current_line
            ));
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn final_sidecar_path_preserves_pool_bytes_and_uses_owned_directory() {
        let fixture = std::env::temp_dir().join(format!(
            "tekai-byte-synctex-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&fixture).unwrap();
        let mut directory =
            crate::working_directory::WorkingDirectory::new(Some(fixture.clone())).unwrap();
        directory
            .as_mut()
            .scope(|| {
                for name in [b"output-\xc3\xa9.log".as_slice(), b"output-\xff.log"] {
                    let mut bytes = name.to_vec();
                    let mut starts = [0, bytes.len() as Integer];
                    crate::invocation_state::with_fixture(|| unsafe {
                        let state = crate::generated::pdftexextra::state();
                        (*state).strpool = bytes.as_mut_ptr();
                        (*state).strstart = starts.as_mut_ptr();
                        (*state).strptr = 1;
                        (*state).poolptr = bytes.len() as Integer;
                        (*state).texmflogname = 0;
                        assert_eq!(tex_path(0).unwrap().as_os_str().as_bytes(), name);
                        let mut context = SyncTexContext::new();
                        context.content_ready = true;
                        context.not_void = true;
                        context.compress = false;
                        let mut expected = fixture.join(OsStr::from_bytes(name));
                        expected.set_extension("synctex");
                        match context.finish() {
                            Ok(Some(path)) => {
                                assert_eq!(path, expected);
                                assert!(std::fs::read(path)
                                    .unwrap()
                                    .starts_with(b"SyncTeX Version:1\n"));
                            }
                            Err(error) if std::str::from_utf8(name).is_err() => {
                                #[cfg(target_os = "macos")]
                                assert_eq!(error.raw_os_error(), Some(libc::EILSEQ));
                                #[cfg(not(target_os = "macos"))]
                                panic!("cannot create byte-path sidecar fixture: {error}");
                            }
                            outcome => panic!("unexpected sidecar result: {outcome:?}"),
                        }
                        if std::str::from_utf8(name).is_err() {
                            let mut replacement =
                                fixture.join(String::from_utf8_lossy(name).as_ref());
                            replacement.set_extension("synctex");
                            assert!(!replacement.exists());
                        }
                    });
                }
            })
            .unwrap();
        std::fs::remove_dir_all(fixture).unwrap();
    }

    #[test]
    fn unfinished_sidecar_is_discarded_on_unwind_and_routing_is_cleared() {
        let mut owner = SyncTexSession::new();
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            SyncTexSession::scope(owner.as_mut(), || {
                with_context(|context| {
                    context.push_text("partial content\n");
                    context.content_ready = true;
                    context.not_void = true;
                });
                panic!("partial typesetting");
            });
        }));
        assert!(outcome.is_err());
        assert!(ACTIVE.with(Cell::get).is_none());
        assert!(owner.context.borrow().is_none());
        SyncTexSession::scope(owner.as_mut(), || {
            with_context(|context| assert_eq!(context.source, "SyncTeX Version:1\n"));
            synctexabort(0);
            with_context(|context| {
                assert!(context.off);
                assert!(context.source.is_empty());
            });
            synctexterminate(0);
        });
        assert!(owner.context.borrow().is_none());
    }

    #[test]
    fn context_borrow_failure_and_nested_scope_do_not_replace_routing() {
        let mut first = SyncTexSession::new();
        let mut second = SyncTexSession::new();
        SyncTexSession::scope(first.as_mut(), || {
            let pointer = ACTIVE.with(Cell::get);
            assert!(catch_unwind(AssertUnwindSafe(|| SyncTexSession::scope(
                second.as_mut(),
                || ()
            )))
            .is_err());
            assert_eq!(ACTIVE.with(Cell::get), pointer);
            assert!(
                catch_unwind(AssertUnwindSafe(|| with_context(|_| with_context(|_| ())))).is_err()
            );
            with_context(|context| assert!(!context.off));
        });
        assert!(ACTIVE.with(Cell::get).is_none());
    }
}

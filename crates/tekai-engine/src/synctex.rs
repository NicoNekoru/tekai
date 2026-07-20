//! Rust-owned SyncTeX writer for the embedded pdfTeX engine.
//!
//! The generated core already contains the standard pdfTeX SyncTeX hook
//! points and stores source tags and line numbers in shipped nodes. This
//! module implements those hooks without linking TeX Live's C SyncTeX
//! runtime.

use std::ffi::CString;
use std::fs::{self, File};
use std::io::{self, Write as _};
use std::path::PathBuf;

use flate2::write::GzEncoder;
use flate2::Compression;

use crate::generated::pdftexextra as engine;

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
        if !self.content_ready || !self.not_void {
            return Ok(None);
        }

        self.push_anchor();
        self.push_text("Postamble:\n");
        self.push_text(&format!("Count:{}\n", self.count));
        self.push_anchor();
        self.push_text("Post scriptum:\n");

        let Some(mut path) = tex_string(unsafe { engine::texmflogname }).map(PathBuf::from) else {
            return Ok(None);
        };
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

static mut CONTEXT: Option<SyncTexContext> = None;

/// Reset per-run state before the generated engine parses command-line flags.
pub unsafe fn reset_for_run() {
    unsafe {
        CONTEXT = Some(SyncTexContext::new());
    }
}

unsafe fn context_mut() -> &'static mut SyncTexContext {
    unsafe {
        if CONTEXT.is_none() {
            CONTEXT = Some(SyncTexContext::new());
        }
        CONTEXT.as_mut().expect("SyncTeX context initialized")
    }
}

fn remove_if_exists(path: &PathBuf) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn record_output(path: &PathBuf) {
    let bytes = path.to_string_lossy();
    if let Ok(path) = CString::new(bytes.as_bytes()) {
        unsafe {
            crate::openclose::recorder_record_output(path.as_ptr());
        }
    }
}

fn tex_string(number: Integer) -> Option<String> {
    unsafe {
        if engine::strstart.is_null()
            || engine::strpool.is_null()
            || number < 0
            || number >= engine::strptr
        {
            return None;
        }
        let start = *engine::strstart.offset(number as isize);
        let end = *engine::strstart.offset((number + 1) as isize);
        if start < 0 || end < start || end > engine::poolptr {
            return None;
        }
        let bytes = core::slice::from_raw_parts(
            engine::strpool.offset(start as isize),
            (end - start) as usize,
        );
        Some(String::from_utf8_lossy(bytes).into_owned())
    }
}

fn current_input_name() -> Option<String> {
    unsafe {
        if engine::fullsourcefilenamestack.is_null() || engine::inopen < 0 {
            return None;
        }
        let name = tex_string(*engine::fullsourcefilenamestack.offset(engine::inopen as isize))?;
        let path = PathBuf::from(&name);
        Some(
            fs::canonicalize(&path)
                .unwrap_or(path)
                .to_string_lossy()
                .into_owned(),
        )
    }
}

fn synctex_value() -> Integer {
    unsafe {
        if engine::zeqtb.is_null() || engine::synctexoffset <= 0 {
            return 0;
        }
        (*engine::zeqtb.offset(engine::synctexoffset as isize))
            .u
            .CINT
    }
}

unsafe fn set_synctex_value(value: Integer) {
    unsafe {
        if !engine::zeqtb.is_null() && engine::synctexoffset > 0 {
            (*engine::zeqtb.offset(engine::synctexoffset as isize))
                .u
                .CINT = value;
        }
    }
}

fn memory_int(index: Halfword) -> Integer {
    unsafe {
        if engine::zmem.is_null() || index < engine::memmin || index > engine::memmax {
            return 0;
        }
        (*engine::zmem.offset(index as isize)).u.CINT
    }
}

fn node_type(node: Halfword) -> Integer {
    unsafe {
        if engine::zmem.is_null() || node < engine::memmin || node > engine::memmax {
            return -1;
        }
        (*engine::zmem.offset(node as isize)).hh.u.B0 as Integer
    }
}

fn node_context(node: Halfword, tag_offset: Halfword, line_offset: Halfword) -> (Integer, Integer) {
    (
        memory_int(node.saturating_add(tag_offset)),
        memory_int(node.saturating_add(line_offset)),
    )
}

fn position() -> (Integer, Integer) {
    unsafe { (engine::curh, engine::curv) }
}

fn record_box(node: Halfword, marker: char) {
    unsafe {
        let context = context_mut();
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
    }
}

fn close_box(marker: char) {
    unsafe {
        let context = context_mut();
        if context.off || !context.content_ready || synctex_value() == 0 {
            return;
        }
        context.push_record(&format!("{marker}\n"));
    }
}

fn record_medium_node(node: Halfword, marker: char, include_width: bool) {
    unsafe {
        let context = context_mut();
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
    }
}

fn record_rule(node: Halfword) {
    unsafe {
        let context = context_mut();
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
            engine::rulewd,
            engine::ruleht,
            engine::ruledp
        ));
    }
}

#[no_mangle]
pub extern "C" fn synctexinitcommand() {
    unsafe {
        let option = engine::synctexoption;
        let context = context_mut();
        if option == SYNCTEX_NO_OPTION {
            set_synctex_value(0);
        } else if option == 0 {
            context.off = true;
            set_synctex_value(0);
        } else {
            context.compress = option >= 0;
            set_synctex_value(option | 1);
        }
    }
}

#[no_mangle]
pub extern "C" fn synctexabort(_log_opened: Boolean) {
    unsafe {
        if let Some(context) = CONTEXT.as_mut() {
            context.off = true;
            context.source.clear();
        }
    }
}

#[no_mangle]
pub extern "C" fn synctexstartinput() {
    unsafe {
        let context = context_mut();
        if context.off {
            engine::curinput.synctextagfield = 0;
            return;
        }
        context.tag_counter = context.tag_counter.saturating_add(1);
        let tag = context.tag_counter;
        engine::curinput.synctextagfield = tag;
        if let Some(name) = current_input_name() {
            context.add_input(tag, name);
        }
    }
}

#[no_mangle]
pub extern "C" fn synctexterminate(log_opened: Boolean) {
    let context = unsafe { CONTEXT.take() };
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

#[no_mangle]
pub extern "C" fn synctexsheet(magnification: Integer) {
    unsafe {
        let context = context_mut();
        if context.off || synctex_value() == 0 {
            return;
        }
        context.prepare_content(magnification);
        context.begin_sheet(engine::totalpages.saturating_add(1));
    }
}

#[no_mangle]
pub extern "C" fn synctexteehs() {
    unsafe {
        let context = context_mut();
        if context.off || !context.content_ready {
            return;
        }
        context.end_sheet(engine::totalpages);
    }
}

#[no_mangle]
pub extern "C" fn synctexpdfxform(_node: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexmrofxfdp() {}

#[no_mangle]
pub extern "C" fn synctexpdfrefxform(_object_number: Integer) {}

#[no_mangle]
pub extern "C" fn synctexvlist(node: Halfword) {
    record_box(node, '[');
}

#[no_mangle]
pub extern "C" fn synctextsilv(_node: Halfword) {
    close_box(']');
}

#[no_mangle]
pub extern "C" fn synctexvoidvlist(node: Halfword, _parent: Halfword) {
    record_box(node, 'v');
}

#[no_mangle]
pub extern "C" fn synctexhlist(node: Halfword) {
    record_box(node, '(');
}

#[no_mangle]
pub extern "C" fn synctextsilh(_node: Halfword) {
    close_box(')');
}

#[no_mangle]
pub extern "C" fn synctexvoidhlist(node: Halfword, _parent: Halfword) {
    record_box(node, 'h');
}

#[no_mangle]
pub extern "C" fn synctexmath(node: Halfword, _parent: Halfword) {
    record_medium_node(node, '$', false);
}

#[no_mangle]
pub extern "C" fn synctexhorizontalruleorglue(node: Halfword, _parent: Halfword) {
    match node_type(node) {
        RULE_NODE => record_rule(node),
        GLUE_NODE => record_medium_node(node, 'g', false),
        KERN_NODE => record_medium_node(node, 'k', true),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn synctexkern(node: Halfword, _parent: Halfword) {
    record_medium_node(node, 'k', true);
}

#[no_mangle]
pub extern "C" fn synctexchar(_node: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexnode(_node: Halfword, _parent: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexcurrent() {
    unsafe {
        let context = context_mut();
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
    }
}

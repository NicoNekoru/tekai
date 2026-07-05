//! Rust-owned SyncTeX boundary.
//!
//! SyncTeX is a source-mapping sidecar format, not part of the TeX-to-PDF
//! result. The fast native path keeps these exports as no-ops so the generated
//! pdfTeX core can call through its historical ABI without linking the C
//! sidecar writer or paying for synchronization bookkeeping.

use core::ffi::c_int;

type Integer = c_int;
type Boolean = c_int;
type Halfword = c_int;

#[no_mangle]
pub extern "C" fn synctexinitcommand() {}

#[no_mangle]
pub extern "C" fn synctexabort(_log_opened: Boolean) {}

#[no_mangle]
pub extern "C" fn synctexstartinput() {}

#[no_mangle]
pub extern "C" fn synctexterminate(_log_opened: Boolean) {}

#[no_mangle]
pub extern "C" fn synctexsheet(_mag: Integer) {}

#[no_mangle]
pub extern "C" fn synctexteehs() {}

#[no_mangle]
pub extern "C" fn synctexpdfxform(_p: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexmrofxfdp() {}

#[no_mangle]
pub extern "C" fn synctexpdfrefxform(_objnum: c_int) {}

#[no_mangle]
pub extern "C" fn synctexvlist(_this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctextsilv(_this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexvoidvlist(_p: Halfword, _this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexhlist(_this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctextsilh(_this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexvoidhlist(_p: Halfword, _this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexmath(_p: Halfword, _this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexhorizontalruleorglue(_p: Halfword, _this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexkern(_p: Halfword, _this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexchar(_p: Halfword, _this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexnode(_p: Halfword, _this_box: Halfword) {}

#[no_mangle]
pub extern "C" fn synctexcurrent() {}

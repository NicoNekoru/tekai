use libc::{c_char, c_double, c_float, c_int, c_long, c_void};
use std::ffi::{CStr, CString};
use std::ptr;

use crate::utils::{
    pdf_out, pdf_printf_args, pdf_write_bytes, pdftex_fail_args, pdftex_warn_args, tex_printf_args,
};
use crate::utils::{FdEntry, FmEntry, PrintfArg};
use crate::xpdf::*;

const MASK_SUPPRESS_PTEX_FILENAME: c_int = 0x02;
const MASK_SUPPRESS_PTEX_PAGENUMBER: c_int = 0x04;
const MASK_SUPPRESS_PTEX_INFODICT: c_int = 0x08;
const REPLACE_EMBEDDED_TYPE1_FONTS: bool = false;
const REPLACE_TYPE1C: bool = false;

#[derive(Clone, Copy, PartialEq, Eq)]
enum InObjType {
    Font,
    FontDesc,
    Other,
}

struct InObj {
    ref_: Ref,
    type_: InObjType,
    next: *mut InObj,
    num: c_int,
    fd: *mut FdEntry,
    enc_objnum: c_int,
    written: c_int,
}

struct UsedEncoding {
    enc_objnum: c_int,
    font: *mut GfxFont,
    next: *mut UsedEncoding,
}

#[repr(C)]
struct PdfDocument {
    file_name: *mut c_char,
    doc: *mut PDFDoc,
    xref: *mut XRef,
    in_obj_list: *mut InObj,
    occurrences: c_int,
    next: *mut PdfDocument,
}

struct Obj {
    ptr: *mut Object,
}

impl Obj {
    unsafe fn new() -> Self {
        Self {
            ptr: unsafe { xpdf_object_new() },
        }
    }
}

impl Drop for Obj {
    fn drop(&mut self) {
        unsafe {
            xpdf_object_delete(self.ptr);
        }
    }
}

extern "C" {
    static mut epdf_width: c_float;
    static mut epdf_height: c_float;
    static mut epdf_orig_x: c_float;
    static mut epdf_orig_y: c_float;
    static mut epdf_rotate: c_float;
    static mut epdf_selected_page: c_int;
    static mut epdf_num_pages: c_int;
    static mut epdf_page_box: c_int;
    static mut epdf_doc: *mut c_void;
    static mut epdf_has_page_group: c_int;

    static mut pdfpagegroupval: c_int;
    static mut fixedinclusioncopyfont: c_int;
    static mut pdflastbyte: u8;
    static mut pdfboxspecmedia: c_int;
    static mut pdfboxspeccrop: c_int;
    static mut pdfboxspecbleed: c_int;
    static mut pdfboxspectrim: c_int;
    static mut pdfboxspecart: c_int;
    static mut notdef: [c_char; 8];

    fn pdfnewobjnum() -> c_int;
    fn zpdfbeginobj(_: c_int, _: c_int);
    fn pdfendobj();
    fn pdfbeginstream();
    fn pdfendstream();

    fn getpdfsuppresswarningpagegroup() -> c_int;
    fn getpdfsuppressptexinfo() -> c_int;
    fn getptexuseunderscore() -> c_int;
    fn zround(_: c_double) -> c_int;

    fn lookup_fontmap(_: *mut c_char) -> *mut FmEntry;
    fn epdf_create_fontdescriptor(_: *mut FmEntry, _: c_int) -> *mut FdEntry;
    fn epdf_mark_glyphs(_: *mut FdEntry, _: *mut c_char);
    fn embed_whole_font(_: *mut FdEntry);
    fn get_fd_objnum(_: *mut FdEntry) -> c_int;
    fn get_fn_objnum(_: *mut FdEntry) -> c_int;
    fn is_subsetable(_: *mut FmEntry) -> c_int;
    fn epdf_write_enc(_: *mut *mut c_char, _: c_int);
}

static mut IN_OBJ_LIST: *mut InObj = ptr::null_mut();
static mut ENCODING_LIST: *mut UsedEncoding = ptr::null_mut();
static mut PDF_DOCUMENTS: *mut PdfDocument = ptr::null_mut();
static mut XREF: *mut XRef = ptr::null_mut();
static mut IS_INIT: c_int = 0;

unsafe fn c_bytes(ptr: *const c_char) -> &'static [u8] {
    if ptr.is_null() {
        b""
    } else {
        unsafe { CStr::from_ptr(ptr).to_bytes() }
    }
}

unsafe fn c_string_dup(ptr: *const c_char) -> *mut c_char {
    let bytes = unsafe { c_bytes(ptr) };
    let out = unsafe { libc::malloc(bytes.len() + 1) as *mut c_char };
    if out.is_null() {
        unsafe { pdftex_fail_args(c"out of memory".as_ptr(), &[]) };
    }
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), out as *mut u8, bytes.len());
        *out.add(bytes.len()) = 0;
    }
    out
}

unsafe fn find_add_document(file_name: *mut c_char) -> *mut PdfDocument {
    unsafe {
        let mut p = PDF_DOCUMENTS;
        while !p.is_null() && libc::strcmp((*p).file_name, file_name) != 0 {
            p = (*p).next;
        }
        if !p.is_null() {
            XREF = (*p).xref;
            (*p).occurrences += 1;
            return p;
        }
        let doc = xpdf_doc_new(file_name);
        if xpdf_doc_is_ok(doc) == 0 || xpdf_doc_ok_to_print(doc) == 0 {
            pdftex_fail_args(c"xpdf: reading PDF image failed".as_ptr(), &[]);
        }
        let p = Box::into_raw(Box::new(PdfDocument {
            file_name: c_string_dup(file_name),
            doc,
            xref: ptr::null_mut(),
            in_obj_list: ptr::null_mut(),
            occurrences: 0,
            next: PDF_DOCUMENTS,
        }));
        PDF_DOCUMENTS = p;
        XREF = ptr::null_mut();
        p
    }
}

unsafe fn free_inobj_list(mut list: *mut InObj) {
    unsafe {
        while !list.is_null() {
            let next = (*list).next;
            drop(Box::from_raw(list));
            list = next;
        }
    }
}

unsafe fn delete_document(pdf_doc: *mut PdfDocument) {
    unsafe {
        let mut p = &raw mut PDF_DOCUMENTS;
        while !(*p).is_null() && *p != pdf_doc {
            p = &raw mut (**p).next;
        }
        if (*p).is_null() {
            return;
        }
        *p = (*pdf_doc).next;
        free_inobj_list((*pdf_doc).in_obj_list);
        XREF = (*pdf_doc).xref;
        xpdf_doc_delete((*pdf_doc).doc);
        libc::free((*pdf_doc).file_name as *mut c_void);
        drop(Box::from_raw(pdf_doc));
    }
}

unsafe fn add_encoding(font: *mut GfxFont) -> c_int {
    unsafe {
        let n = Box::into_raw(Box::new(UsedEncoding {
            next: ENCODING_LIST,
            font,
            enc_objnum: pdfnewobjnum(),
        }));
        ENCODING_LIST = n;
        (*n).enc_objnum
    }
}

unsafe fn add_in_obj(type_: InObjType, ref_: Ref, fd: *mut FdEntry, enc: c_int) -> c_int {
    unsafe {
        if ref_.num == 0 {
            pdftex_fail_args(c"PDF inclusion: invalid reference".as_ptr(), &[]);
        }
        let n = Box::into_raw(Box::new(InObj {
            ref_,
            type_,
            next: ptr::null_mut(),
            fd,
            enc_objnum: enc,
            written: 0,
            num: 0,
        }));
        if IN_OBJ_LIST.is_null() {
            IN_OBJ_LIST = n;
        } else {
            let mut p = IN_OBJ_LIST;
            let mut q = IN_OBJ_LIST;
            while !p.is_null() {
                if (*p).ref_.num == ref_.num && (*p).ref_.gen == ref_.gen {
                    drop(Box::from_raw(n));
                    return (*p).num;
                }
                q = p;
                p = (*p).next;
            }
            (*q).next = n;
        }
        (*n).num = if type_ == InObjType::FontDesc {
            get_fd_objnum(fd)
        } else {
            pdfnewobjnum()
        };
        (*n).num
    }
}

unsafe fn copy_name(s: *mut c_char) {
    unsafe {
        crate::utils::pdf_puts(c"/".as_ptr());
        let mut p = s;
        while !p.is_null() && *p != 0 {
            let ch = *p as u8;
            if ch.is_ascii_digit()
                || ch.is_ascii_uppercase()
                || ch.is_ascii_lowercase()
                || matches!(ch, b'_' | b'.' | b'-' | b'+')
            {
                pdf_out(ch);
            } else {
                let frag = CString::new(format!("#{:02X}", ch)).unwrap();
                crate::utils::pdf_puts(frag.as_ptr());
            }
            p = p.add(1);
        }
    }
}

unsafe fn copy_dict_entry(obj: *mut Object, i: c_int) {
    unsafe {
        let obj1 = Obj::new();
        copy_name(xpdf_object_dict_get_key(obj, i));
        crate::utils::pdf_puts(c" ".as_ptr());
        xpdf_object_dict_get_val_nf(obj, i, obj1.ptr);
        copy_object(obj1.ptr);
        crate::utils::pdf_puts(c"\n".as_ptr());
    }
}

unsafe fn copy_dict(obj: *mut Object) {
    unsafe {
        if xpdf_object_is_dict(obj) == 0 {
            pdftex_fail_args(
                c"PDF inclusion: invalid dict type <%s>".as_ptr(),
                &[PrintfArg::from(xpdf_object_type_name(obj))],
            );
        }
        for i in 0..xpdf_object_dict_get_length(obj) {
            copy_dict_entry(obj, i);
        }
    }
}

unsafe fn copy_dict_from_dict(dict: *mut Dict) {
    unsafe {
        for i in 0..xpdf_dict_get_length(dict) {
            let obj1 = Obj::new();
            copy_name(xpdf_dict_get_key(dict, i));
            crate::utils::pdf_puts(c" ".as_ptr());
            xpdf_dict_get_val_nf(dict, i, obj1.ptr);
            copy_object(obj1.ptr);
            crate::utils::pdf_puts(c"\n".as_ptr());
        }
    }
}

unsafe fn copy_font_dict(obj: *mut Object, r: *mut InObj) {
    unsafe {
        if xpdf_object_is_dict(obj) == 0 {
            pdftex_fail_args(
                c"PDF inclusion: invalid dict type <%s>".as_ptr(),
                &[PrintfArg::from(xpdf_object_type_name(obj))],
            );
        }
        crate::utils::pdf_puts(c"<<\n".as_ptr());
        for i in 0..xpdf_object_dict_get_length(obj) {
            let key = xpdf_object_dict_get_key(obj, i);
            if libc::strncmp(
                key,
                c"FontDescriptor".as_ptr(),
                c"FontDescriptor".to_bytes().len(),
            ) == 0
                || libc::strncmp(key, c"BaseFont".as_ptr(), c"BaseFont".to_bytes().len()) == 0
                || libc::strncmp(key, c"Encoding".as_ptr(), c"Encoding".to_bytes().len()) == 0
            {
                continue;
            }
            copy_dict_entry(obj, i);
        }
        pdf_printf_args(
            c"/FontDescriptor %d 0 R\n".as_ptr(),
            &[PrintfArg::from(get_fd_objnum((*r).fd))],
        );
        pdf_printf_args(
            c"/BaseFont %d 0 R\n".as_ptr(),
            &[PrintfArg::from(get_fn_objnum((*r).fd))],
        );
        pdf_printf_args(
            c"/Encoding %d 0 R\n".as_ptr(),
            &[PrintfArg::from((*r).enc_objnum)],
        );
        crate::utils::pdf_puts(c">>".as_ptr());
    }
}

unsafe fn copy_stream(stream: *mut Stream) {
    unsafe {
        xpdf_stream_reset(stream);
        let mut len: libc::size_t = 0;
        let data = xpdf_stream_get_remaining_data(stream, &mut len);
        if data.is_null() || len == 0 {
            pdflastbyte = 0;
            return;
        }
        pdf_write_bytes(data, len);
    }
}

unsafe fn copy_proc_set(obj: *mut Object) {
    unsafe {
        if xpdf_object_is_array(obj) == 0 {
            pdftex_fail_args(
                c"PDF inclusion: invalid ProcSet array type <%s>".as_ptr(),
                &[PrintfArg::from(xpdf_object_type_name(obj))],
            );
        }
        crate::utils::pdf_puts(c"/ProcSet [ ".as_ptr());
        for i in 0..xpdf_object_array_get_length(obj) {
            let procset = Obj::new();
            xpdf_object_array_get_nf(obj, i, procset.ptr);
            if xpdf_object_is_name(procset.ptr) == 0 {
                pdftex_fail_args(
                    c"PDF inclusion: invalid ProcSet entry type <%s>".as_ptr(),
                    &[PrintfArg::from(xpdf_object_type_name(procset.ptr))],
                );
            }
            copy_name(xpdf_object_get_name(procset.ptr));
            crate::utils::pdf_puts(c" ".as_ptr());
        }
        crate::utils::pdf_puts(c"]\n".as_ptr());
    }
}

unsafe fn copy_font(tag: *mut c_char, font_ref: *mut Object) {
    unsafe {
        let ref_ = xpdf_object_get_ref(font_ref);
        let mut p = IN_OBJ_LIST;
        while !p.is_null() {
            if (*p).ref_.num == ref_.num && (*p).ref_.gen == ref_.gen {
                copy_name(tag);
                pdf_printf_args(c" %d 0 R ".as_ptr(), &[PrintfArg::from((*p).num)]);
                return;
            }
            p = (*p).next;
        }
        let fontdict = Obj::new();
        let subtype = Obj::new();
        let basefont = Obj::new();
        let fontdesc_ref = Obj::new();
        let fontdesc = Obj::new();
        let charset = Obj::new();
        let fontfile = Obj::new();
        let ffsubtype = Obj::new();
        let stemv = Obj::new();
        let fetched = xpdf_object_fetch(font_ref, XREF, fontdict.ptr);
        let mut fontmap: *mut FmEntry = ptr::null_mut();
        let replaceable = REPLACE_EMBEDDED_TYPE1_FONTS
            && fixedinclusioncopyfont == 0
            && xpdf_object_is_dict(fetched) != 0
            && {
                xpdf_object_dict_lookup(fontdict.ptr, c"Subtype".as_ptr(), subtype.ptr);
                xpdf_object_is_name(subtype.ptr) != 0
                    && libc::strcmp(xpdf_object_get_name(subtype.ptr), c"Type1".as_ptr()) == 0
            }
            && {
                xpdf_object_dict_lookup(fontdict.ptr, c"BaseFont".as_ptr(), basefont.ptr);
                xpdf_object_is_name(basefont.ptr) != 0
            }
            && {
                xpdf_object_dict_lookup_nf(
                    fontdict.ptr,
                    c"FontDescriptor".as_ptr(),
                    fontdesc_ref.ptr,
                );
                xpdf_object_is_ref(fontdesc_ref.ptr) != 0
            }
            && {
                xpdf_object_fetch(fontdesc_ref.ptr, XREF, fontdesc.ptr);
                xpdf_object_is_dict(fontdesc.ptr) != 0
            }
            && {
                xpdf_object_dict_lookup(fontdesc.ptr, c"FontFile".as_ptr(), fontfile.ptr);
                xpdf_object_is_stream(fontfile.ptr) != 0
                    || (REPLACE_TYPE1C && {
                        xpdf_object_free_contents(fontfile.ptr);
                        xpdf_object_dict_lookup(fontdesc.ptr, c"FontFile3".as_ptr(), fontfile.ptr);
                        xpdf_object_is_stream(fontfile.ptr) != 0 && {
                            xpdf_dict_lookup(
                                xpdf_object_stream_get_dict(fontfile.ptr),
                                c"Subtype".as_ptr(),
                                ffsubtype.ptr,
                            );
                            xpdf_object_is_name(ffsubtype.ptr) != 0
                                && libc::strcmp(
                                    xpdf_object_get_name(ffsubtype.ptr),
                                    c"Type1C".as_ptr(),
                                ) == 0
                        }
                    })
            }
            && {
                fontmap = lookup_fontmap(xpdf_object_get_name(basefont.ptr));
                !fontmap.is_null()
            };
        if replaceable {
            xpdf_object_dict_lookup(fontdesc.ptr, c"StemV".as_ptr(), stemv.ptr);
            let fd = epdf_create_fontdescriptor(fontmap, zround(xpdf_object_get_num(stemv.ptr)));
            xpdf_object_dict_lookup(fontdesc.ptr, c"CharSet".as_ptr(), charset.ptr);
            if xpdf_object_is_string(charset.ptr) != 0 && is_subsetable(fontmap) != 0 {
                epdf_mark_glyphs(
                    fd,
                    xpdf_gstring_cstr(xpdf_object_get_string(charset.ptr)) as *mut c_char,
                );
            } else {
                embed_whole_font(fd);
            }
            add_in_obj(
                InObjType::FontDesc,
                xpdf_object_get_ref(fontdesc_ref.ptr),
                fd,
                0,
            );
            copy_name(tag);
            let gfont = xpdf_gfx_font_make(
                XREF,
                tag,
                xpdf_object_get_ref(font_ref),
                xpdf_object_get_dict(fontdict.ptr),
            );
            let enc = add_encoding(gfont);
            let num = add_in_obj(InObjType::Font, xpdf_object_get_ref(font_ref), fd, enc);
            pdf_printf_args(c" %d 0 R ".as_ptr(), &[PrintfArg::from(num)]);
        } else {
            copy_name(tag);
            crate::utils::pdf_puts(c" ".as_ptr());
            copy_object(font_ref);
        }
    }
}

unsafe fn copy_font_resources(obj: *mut Object) {
    unsafe {
        if xpdf_object_is_dict(obj) == 0 {
            pdftex_fail_args(
                c"PDF inclusion: invalid font resources dict type <%s>".as_ptr(),
                &[PrintfArg::from(xpdf_object_type_name(obj))],
            );
        }
        crate::utils::pdf_puts(c"/Font << ".as_ptr());
        for i in 0..xpdf_object_dict_get_length(obj) {
            let font_ref = Obj::new();
            xpdf_object_dict_get_val_nf(obj, i, font_ref.ptr);
            if xpdf_object_is_ref(font_ref.ptr) != 0 {
                copy_font(xpdf_object_dict_get_key(obj, i), font_ref.ptr);
            } else if xpdf_object_is_dict(font_ref.ptr) != 0 {
                copy_name(xpdf_object_dict_get_key(obj, i));
                crate::utils::pdf_puts(c" ".as_ptr());
                copy_object(font_ref.ptr);
            } else {
                pdftex_fail_args(
                    c"PDF inclusion: invalid font in reference type <%s>".as_ptr(),
                    &[PrintfArg::from(xpdf_object_type_name(font_ref.ptr))],
                );
            }
        }
        crate::utils::pdf_puts(c">>\n".as_ptr());
    }
}

unsafe fn copy_other_resources(obj: *mut Object, key: *mut c_char) {
    unsafe {
        if libc::strcmp(c"Subtype".as_ptr(), key) == 0 {
            if xpdf_object_is_name(obj) == 0 {
                pdftex_warn_args(
                    c"PDF inclusion: Subtype in Resources dict is not a name (key '%s', type <%s>); ignored.".as_ptr(),
                    &[PrintfArg::from(key), PrintfArg::from(xpdf_object_type_name(obj))],
                );
                return;
            }
        } else if xpdf_object_is_dict(obj) == 0 {
            pdftex_warn_args(
                c"PDF inclusion: invalid other resource which is no dict (key '%s', type <%s>); ignored.".as_ptr(),
                &[PrintfArg::from(key), PrintfArg::from(xpdf_object_type_name(obj))],
            );
            return;
        }
        copy_name(key);
        crate::utils::pdf_puts(c" ".as_ptr());
        copy_object(obj);
    }
}

fn convert_num_to_pdf(mut n: f64) -> CString {
    const PRECISION: usize = 6;
    const FACT: f64 = 1_000_000.0;
    const EPSILON: f64 = 0.5e-6;
    if n.abs() < EPSILON {
        return CString::new("0").unwrap();
    }
    let mut out = String::new();
    if n < 0.0 {
        out.push('-');
        n = -n;
    }
    n += EPSILON;
    let ival = n.floor() as i32;
    n -= ival as f64;
    out.push_str(&ival.to_string());
    let mut fval = (n * FACT).floor() as i32;
    if fval != 0 {
        let mut digits = [b'0'; PRECISION];
        for idx in (0..PRECISION).rev() {
            digits[idx] = (fval % 10) as u8 + b'0';
            fval /= 10;
        }
        let end = digits
            .iter()
            .rposition(|&byte| byte != b'0')
            .map(|idx| idx + 1)
            .unwrap_or(0);
        if end > 0 {
            out.push('.');
            out.push_str(std::str::from_utf8(&digits[..end]).unwrap());
        }
    }
    CString::new(out).unwrap()
}

unsafe fn copy_object(obj: *mut Object) {
    unsafe {
        if xpdf_object_is_bool(obj) != 0 {
            pdf_printf_args(
                c"%s".as_ptr(),
                &[PrintfArg::from(if xpdf_object_get_bool(obj) != 0 {
                    c"true".as_ptr()
                } else {
                    c"false".as_ptr()
                })],
            );
        } else if xpdf_object_is_int(obj) != 0 {
            pdf_printf_args(c"%i".as_ptr(), &[PrintfArg::from(xpdf_object_get_int(obj))]);
        } else if xpdf_object_is_real(obj) != 0 {
            let s = convert_num_to_pdf(xpdf_object_get_real(obj));
            pdf_printf_args(c"%s".as_ptr(), &[PrintfArg::from(s.as_ptr())]);
        } else if xpdf_object_is_num(obj) != 0 {
            let s = convert_num_to_pdf(xpdf_object_get_num(obj));
            pdf_printf_args(c"%s".as_ptr(), &[PrintfArg::from(s.as_ptr())]);
        } else if xpdf_object_is_string(obj) != 0 {
            let gs = xpdf_object_get_string(obj);
            let p = xpdf_gstring_cstr(gs);
            let len = xpdf_gstring_len(gs);
            if libc::strlen(p) as c_int == len {
                crate::utils::pdf_puts(c"(".as_ptr());
                let mut q = p;
                while *q != 0 {
                    let c = *q as u8;
                    if matches!(c, b'(' | b')' | b'\\') {
                        pdf_printf_args(c"\\%c".as_ptr(), &[PrintfArg::from(c as c_int)]);
                    } else if c < 0x20 || c > 0x7f {
                        pdf_printf_args(c"\\%03o".as_ptr(), &[PrintfArg::from(c as c_int)]);
                    } else {
                        pdf_out(c);
                    }
                    q = q.add(1);
                }
                crate::utils::pdf_puts(c")".as_ptr());
            } else {
                crate::utils::pdf_puts(c"<".as_ptr());
                for i in 0..len {
                    pdf_printf_args(
                        c"%.2x".as_ptr(),
                        &[PrintfArg::from(xpdf_gstring_char(gs, i) & 0xff)],
                    );
                }
                crate::utils::pdf_puts(c">".as_ptr());
            }
        } else if xpdf_object_is_name(obj) != 0 {
            copy_name(xpdf_object_get_name(obj));
        } else if xpdf_object_is_null(obj) != 0 {
            crate::utils::pdf_puts(c"null".as_ptr());
        } else if xpdf_object_is_array(obj) != 0 {
            crate::utils::pdf_puts(c"[".as_ptr());
            for i in 0..xpdf_object_array_get_length(obj) {
                let obj1 = Obj::new();
                xpdf_object_array_get_nf(obj, i, obj1.ptr);
                if xpdf_object_is_name(obj1.ptr) == 0 {
                    crate::utils::pdf_puts(c" ".as_ptr());
                }
                copy_object(obj1.ptr);
            }
            crate::utils::pdf_puts(c"]".as_ptr());
        } else if xpdf_object_is_dict(obj) != 0 {
            crate::utils::pdf_puts(c"<<\n".as_ptr());
            copy_dict(obj);
            crate::utils::pdf_puts(c">>".as_ptr());
        } else if xpdf_object_is_stream(obj) != 0 {
            let obj1 = Obj::new();
            xpdf_object_init_dict_from_dict(obj1.ptr, XREF, xpdf_object_stream_get_dict(obj));
            crate::utils::pdf_puts(c"<<\n".as_ptr());
            copy_dict(obj1.ptr);
            crate::utils::pdf_puts(c">>\n".as_ptr());
            crate::utils::pdf_puts(c"stream\n".as_ptr());
            copy_stream(xpdf_object_stream_get_undecoded_stream(obj));
            crate::utils::pdf_puts(c"\nendstream".as_ptr());
        } else if xpdf_object_is_ref(obj) != 0 {
            let ref_ = xpdf_object_get_ref(obj);
            if ref_.num == 0 {
                pdftex_fail_args(
                    c"PDF inclusion: reference to invalid object (is the included pdf broken?)"
                        .as_ptr(),
                    &[],
                );
            }
            pdf_printf_args(
                c"%d 0 R".as_ptr(),
                &[PrintfArg::from(add_in_obj(
                    InObjType::Other,
                    ref_,
                    ptr::null_mut(),
                    0,
                ))],
            );
        } else {
            pdftex_fail_args(
                c"PDF inclusion: type <%s> cannot be copied".as_ptr(),
                &[PrintfArg::from(xpdf_object_type_name(obj))],
            );
        }
    }
}

unsafe fn write_refs() {
    unsafe {
        let mut r = IN_OBJ_LIST;
        while !r.is_null() {
            if (*r).written == 0 {
                let obj1 = Obj::new();
                (*r).written = 1;
                xpdf_xref_fetch(XREF, (*r).ref_.num, (*r).ref_.gen, obj1.ptr);
                if (*r).type_ == InObjType::Font {
                    zpdfbeginobj((*r).num, 2);
                    copy_font_dict(obj1.ptr, r);
                    crate::utils::pdf_puts(c"\n".as_ptr());
                    pdfendobj();
                } else if (*r).type_ != InObjType::FontDesc {
                    zpdfbeginobj(
                        (*r).num,
                        if xpdf_object_is_stream(obj1.ptr) != 0 {
                            0
                        } else {
                            2
                        },
                    );
                    copy_object(obj1.ptr);
                    crate::utils::pdf_puts(c"\n".as_ptr());
                    pdfendobj();
                }
            }
            r = (*r).next;
        }
    }
}

unsafe fn write_encodings() {
    unsafe {
        let mut r = ENCODING_LIST;
        while !r.is_null() {
            let mut glyph_names = [ptr::null_mut::<c_char>(); 256];
            for i in 0..256 {
                if xpdf_gfx_font_is_cid((*r).font) != 0 {
                    pdftex_fail_args(
                        c"PDF inclusion: CID fonts are not supported (try to disable font replacement to fix this)"
                            .as_ptr(),
                        &[],
                    );
                }
                let name = xpdf_gfx_8bit_font_char_name((*r).font, i as c_int);
                glyph_names[i] = if name.is_null() {
                    &raw mut notdef as *mut c_char
                } else {
                    name
                };
            }
            epdf_write_enc(glyph_names.as_mut_ptr(), (*r).enc_objnum);
            r = (*r).next;
        }
        r = ENCODING_LIST;
        while !r.is_null() {
            let next = (*r).next;
            xpdf_gfx_font_delete((*r).font);
            drop(Box::from_raw(r));
            r = next;
        }
        ENCODING_LIST = ptr::null_mut();
    }
}

unsafe fn get_pagebox(page: *mut Page, pagebox_spec: c_int) -> Rect {
    unsafe {
        if pagebox_spec == pdfboxspecmedia {
            xpdf_page_media_box(page)
        } else if pagebox_spec == pdfboxspeccrop {
            xpdf_page_crop_box(page)
        } else if pagebox_spec == pdfboxspecbleed {
            xpdf_page_bleed_box(page)
        } else if pagebox_spec == pdfboxspectrim {
            xpdf_page_trim_box(page)
        } else if pagebox_spec == pdfboxspecart {
            xpdf_page_art_box(page)
        } else {
            pdftex_fail_args(
                c"PDF inclusion: unknown value of pagebox spec (%i)".as_ptr(),
                &[PrintfArg::from(pagebox_spec)],
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn read_pdf_info(
    image_name: *mut c_char,
    page_name: *mut c_char,
    mut page_num: c_int,
    pagebox_spec: c_int,
    major_pdf_version_wanted: c_int,
    minor_pdf_version_wanted: c_int,
    pdf_inclusion_errorlevel: c_int,
) -> c_int {
    unsafe {
        if IS_INIT == 0 {
            xpdf_global_params_init();
            IS_INIT = 1;
        }
        let pdf_doc = find_add_document(image_name);
        epdf_doc = pdf_doc as *mut c_void;
        let found = xpdf_doc_pdf_version((*pdf_doc).doc);
        let wanted =
            major_pdf_version_wanted as c_float + minor_pdf_version_wanted as c_float * 0.1;
        if found > wanted + 0.01 {
            let msg =
                c"PDF inclusion: found PDF version <%.1f>, but at most version <%.1f> allowed";
            if pdf_inclusion_errorlevel > 0 {
                pdftex_fail_args(
                    msg.as_ptr(),
                    &[PrintfArg::from(found), PrintfArg::from(wanted)],
                );
            } else if pdf_inclusion_errorlevel == 0 {
                pdftex_warn_args(
                    msg.as_ptr(),
                    &[PrintfArg::from(found), PrintfArg::from(wanted)],
                );
            }
        }
        let catalog = xpdf_doc_catalog((*pdf_doc).doc);
        epdf_num_pages = xpdf_catalog_num_pages(catalog);
        if !page_name.is_null() {
            let link = xpdf_doc_find_dest((*pdf_doc).doc, page_name);
            if link.is_null() || xpdf_link_dest_is_ok(link) == 0 {
                pdftex_fail_args(
                    c"PDF inclusion: invalid destination <%s>".as_ptr(),
                    &[PrintfArg::from(page_name)],
                );
            }
            let ref_ = xpdf_link_dest_page_ref(link);
            page_num = xpdf_catalog_find_page(catalog, ref_.num, ref_.gen);
            if page_num == 0 {
                pdftex_fail_args(
                    c"PDF inclusion: destination is not a page <%s>".as_ptr(),
                    &[PrintfArg::from(page_name)],
                );
            }
            xpdf_link_dest_delete(link);
        } else if page_num <= 0 || page_num > epdf_num_pages {
            pdftex_fail_args(
                c"PDF inclusion: required page does not exist <%i>".as_ptr(),
                &[PrintfArg::from(epdf_num_pages)],
            );
        }
        let page = xpdf_catalog_get_page(catalog, page_num);
        let pagebox = get_pagebox(page, pagebox_spec);
        if pagebox.x2 > pagebox.x1 {
            epdf_orig_x = pagebox.x1 as c_float;
            epdf_width = (pagebox.x2 - pagebox.x1) as c_float;
        } else {
            epdf_orig_x = pagebox.x2 as c_float;
            epdf_width = (pagebox.x1 - pagebox.x2) as c_float;
        }
        if pagebox.y2 > pagebox.y1 {
            epdf_orig_y = pagebox.y1 as c_float;
            epdf_height = (pagebox.y2 - pagebox.y1) as c_float;
        } else {
            epdf_orig_y = pagebox.y2 as c_float;
            epdf_height = (pagebox.y1 - pagebox.y2) as c_float;
        }
        let mut rotate = xpdf_page_rotate(page) % 360;
        if rotate < 0 {
            rotate += 360;
        }
        epdf_rotate = rotate as c_float;
        epdf_has_page_group = (!xpdf_page_group(page).is_null()) as c_int;
        (*pdf_doc).xref = xpdf_doc_xref((*pdf_doc).doc);
        page_num
    }
}

fn stripped_pdf_line(prefix: &str, values: &[f64]) -> CString {
    let mut s = String::from(prefix);
    for (idx, value) in values.iter().enumerate() {
        if idx > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{value:.8}"));
    }
    s.push_str("]\n");
    let mut bytes = CString::new(s).unwrap().into_bytes_with_nul();
    unsafe {
        crate::utils::stripzeros(bytes.as_mut_ptr() as *mut c_char);
    }
    if let Some(nul) = bytes.iter().position(|&byte| byte == 0) {
        bytes.truncate(nul + 1);
    }
    CString::from_vec_with_nul(bytes).unwrap()
}

unsafe fn write_page_dict_key(page_dict: *mut Dict, key: &'static CStr) {
    unsafe {
        let dict_obj = Obj::new();
        xpdf_dict_lookup_nf(page_dict, key.as_ptr(), dict_obj.ptr);
        if xpdf_object_is_null(dict_obj.ptr) == 0 {
            crate::utils::pdf_newline();
            pdf_printf_args(c"/%s ".as_ptr(), &[PrintfArg::from(key.as_ptr())]);
            copy_object(dict_obj.ptr);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn write_epdf() {
    unsafe {
        let pdf_doc = epdf_doc as *mut PdfDocument;
        (*pdf_doc).occurrences -= 1;
        XREF = (*pdf_doc).xref;
        IN_OBJ_LIST = (*pdf_doc).in_obj_list;
        ENCODING_LIST = ptr::null_mut();
        let catalog = xpdf_doc_catalog((*pdf_doc).doc);
        let page = xpdf_catalog_get_page(catalog, epdf_selected_page);
        let page_ref = xpdf_catalog_get_page_ref(catalog, epdf_selected_page);
        let page_obj = Obj::new();
        xpdf_xref_fetch(XREF, page_ref.num, page_ref.gen, page_obj.ptr);
        let page_dict = xpdf_object_get_dict(page_obj.ptr);
        let rotate = xpdf_page_rotate(page);
        let suppress_ptex_info = getpdfsuppressptexinfo();
        let sep = if getptexuseunderscore() != 0 {
            c"_".as_ptr()
        } else {
            c".".as_ptr()
        };

        crate::utils::pdf_puts(c"/Type /XObject\n".as_ptr());
        crate::utils::pdf_puts(c"/Subtype /Form\n".as_ptr());
        crate::utils::pdf_puts(c"/FormType 1\n".as_ptr());
        if suppress_ptex_info & MASK_SUPPRESS_PTEX_FILENAME == 0 {
            pdf_printf_args(
                c"/PTEX%sFileName (%s)\n".as_ptr(),
                &[
                    PrintfArg::from(sep),
                    PrintfArg::from(crate::utils::convertStringToPDFString(
                        (*pdf_doc).file_name,
                        libc::strlen((*pdf_doc).file_name) as c_int,
                    )),
                ],
            );
        }
        if suppress_ptex_info & MASK_SUPPRESS_PTEX_PAGENUMBER == 0 {
            pdf_printf_args(
                c"/PTEX%sPageNumber %i\n".as_ptr(),
                &[PrintfArg::from(sep), PrintfArg::from(epdf_selected_page)],
            );
        }
        if suppress_ptex_info & MASK_SUPPRESS_PTEX_INFODICT == 0 {
            let info = Obj::new();
            xpdf_doc_get_info_nf((*pdf_doc).doc, info.ptr);
            if xpdf_object_is_ref(info.ptr) != 0 {
                pdf_printf_args(c"/PTEX%sInfoDict ".as_ptr(), &[PrintfArg::from(sep)]);
                pdf_printf_args(
                    c"%d 0 R\n".as_ptr(),
                    &[PrintfArg::from(add_in_obj(
                        InObjType::Other,
                        xpdf_object_get_ref(info.ptr),
                        ptr::null_mut(),
                        0,
                    ))],
                );
            }
        }

        let pagebox = get_pagebox(page, epdf_page_box);
        if rotate != 0 && rotate % 90 == 0 {
            let mut scale = [0.0f64; 6];
            let mut write_matrix = true;
            tex_printf_args(
                c", page is rotated %d degrees".as_ptr(),
                &[PrintfArg::from(rotate)],
            );
            match rotate {
                90 => {
                    scale[1] = -1.0;
                    scale[2] = 1.0;
                    scale[4] = pagebox.x1 - pagebox.y1;
                    scale[5] = pagebox.y1 + pagebox.x2;
                }
                180 => {
                    scale[0] = -1.0;
                    scale[3] = -1.0;
                    scale[4] = pagebox.x1 + pagebox.x2;
                    scale[5] = pagebox.y1 + pagebox.y2;
                }
                270 => {
                    scale[1] = 1.0;
                    scale[2] = -1.0;
                    scale[4] = pagebox.x1 + pagebox.y2;
                    scale[5] = pagebox.y1 - pagebox.x1;
                }
                _ => write_matrix = false,
            }
            if write_matrix {
                let s = stripped_pdf_line("/Matrix [", &scale);
                crate::utils::pdf_puts(s.as_ptr());
            }
        }
        let bbox = stripped_pdf_line("/BBox [", &[pagebox.x1, pagebox.y1, pagebox.x2, pagebox.y2]);
        crate::utils::pdf_puts(bbox.as_ptr());

        let dict_obj = Obj::new();
        xpdf_dict_lookup_nf(page_dict, c"Metadata".as_ptr(), dict_obj.ptr);
        if xpdf_object_is_null(dict_obj.ptr) == 0 && xpdf_object_is_ref(dict_obj.ptr) == 0 {
            pdftex_warn_args(
                c"PDF inclusion: /Metadata must be indirect object".as_ptr(),
                &[],
            );
        }
        write_page_dict_key(page_dict, c"LastModified");
        write_page_dict_key(page_dict, c"Metadata");
        write_page_dict_key(page_dict, c"PieceInfo");
        write_page_dict_key(page_dict, c"SeparationInfo");

        xpdf_object_free_contents(dict_obj.ptr);
        xpdf_dict_lookup_nf(page_dict, c"Group".as_ptr(), dict_obj.ptr);
        let mut write_sep_group = false;
        let group_dict = Obj::new();
        if xpdf_object_is_null(dict_obj.ptr) == 0 {
            if pdfpagegroupval == 0 {
                if getpdfsuppresswarningpagegroup() == 0 {
                    pdftex_warn_args(
                        c"PDF inclusion: multiple pdfs with page group included in a single page"
                            .as_ptr(),
                        &[],
                    );
                }
                crate::utils::pdf_newline();
                crate::utils::pdf_puts(c"/Group ".as_ptr());
                copy_object(dict_obj.ptr);
            } else {
                xpdf_object_free_contents(dict_obj.ptr);
                xpdf_dict_lookup(page_dict, c"Group".as_ptr(), dict_obj.ptr);
                if xpdf_object_is_dict(dict_obj.ptr) == 0 {
                    pdftex_fail_args(c"PDF inclusion: /Group dict missing".as_ptr(), &[]);
                }
                write_sep_group = true;
                xpdf_object_init_dict_from_dict(group_dict.ptr, XREF, xpdf_page_group(page));
                pdf_printf_args(
                    c"/Group %ld 0 R\n".as_ptr(),
                    &[PrintfArg::from(pdfpagegroupval as c_long)],
                );
            }
        }

        let res_dict = xpdf_page_resource_dict(page);
        if res_dict.is_null() {
            pdftex_warn_args(
                c"PDF inclusion: /Resources missing. 'This practice is not recommended' (PDF Ref)"
                    .as_ptr(),
                &[],
            );
        } else {
            let obj1 = Obj::new();
            let obj2 = Obj::new();
            xpdf_object_init_dict_from_dict(obj1.ptr, XREF, res_dict);
            if xpdf_object_is_dict(obj1.ptr) == 0 {
                pdftex_fail_args(
                    c"PDF inclusion: invalid resources dict type <%s>".as_ptr(),
                    &[PrintfArg::from(xpdf_object_type_name(obj1.ptr))],
                );
            }
            crate::utils::pdf_newline();
            crate::utils::pdf_puts(c"/Resources <<\n".as_ptr());
            for i in 0..xpdf_object_dict_get_length(obj1.ptr) {
                xpdf_object_free_contents(obj2.ptr);
                xpdf_object_dict_get_val(obj1.ptr, i, obj2.ptr);
                let key = xpdf_object_dict_get_key(obj1.ptr, i);
                if libc::strcmp(c"Font".as_ptr(), key) == 0 {
                    copy_font_resources(obj2.ptr);
                } else if libc::strcmp(c"ProcSet".as_ptr(), key) == 0 {
                    copy_proc_set(obj2.ptr);
                } else {
                    copy_other_resources(obj2.ptr, key);
                }
            }
            crate::utils::pdf_puts(c">>\n".as_ptr());
        }

        let contents = Obj::new();
        let obj1 = Obj::new();
        xpdf_page_get_contents(page, contents.ptr);
        if xpdf_object_is_stream(contents.ptr) != 0 {
            let stream_dict = xpdf_object_stream_get_dict(contents.ptr);
            xpdf_dict_lookup(stream_dict, c"F".as_ptr(), obj1.ptr);
            if xpdf_object_is_null(obj1.ptr) == 0 {
                pdftex_fail_args(c"PDF inclusion: Unsupported external stream".as_ptr(), &[]);
            }
            xpdf_object_free_contents(obj1.ptr);
            xpdf_dict_lookup(stream_dict, c"Length".as_ptr(), obj1.ptr);
            crate::utils::pdf_puts(c"/Length ".as_ptr());
            copy_object(obj1.ptr);
            crate::utils::pdf_puts(c"\n".as_ptr());
            xpdf_object_free_contents(obj1.ptr);
            xpdf_dict_lookup(stream_dict, c"Filter".as_ptr(), obj1.ptr);
            if xpdf_object_is_null(obj1.ptr) == 0 {
                crate::utils::pdf_puts(c"/Filter ".as_ptr());
                copy_object(obj1.ptr);
                crate::utils::pdf_puts(c"\n".as_ptr());
                xpdf_object_free_contents(obj1.ptr);
                xpdf_dict_lookup(stream_dict, c"DecodeParms".as_ptr(), obj1.ptr);
                if xpdf_object_is_null(obj1.ptr) == 0 {
                    crate::utils::pdf_puts(c"/DecodeParms ".as_ptr());
                    copy_object(obj1.ptr);
                    crate::utils::pdf_puts(c"\n".as_ptr());
                }
            }
            crate::utils::pdf_puts(c">>\nstream\n".as_ptr());
            copy_stream(xpdf_object_stream_get_undecoded_stream(contents.ptr));
            pdfendstream();
        } else if xpdf_object_is_array(contents.ptr) != 0 {
            pdfbeginstream();
            for i in 0..xpdf_object_array_get_length(contents.ptr) {
                let contents_obj = Obj::new();
                xpdf_object_array_get(contents.ptr, i, contents_obj.ptr);
                copy_stream(xpdf_object_get_stream(contents_obj.ptr));
                if i < xpdf_object_array_get_length(contents.ptr) - 1 {
                    crate::utils::pdf_newline();
                }
            }
            pdfendstream();
        } else {
            pdfbeginstream();
            pdfendstream();
        }

        write_encodings();
        if write_sep_group {
            zpdfbeginobj(pdfpagegroupval, 2);
            copy_object(group_dict.ptr);
            crate::utils::pdf_puts(c"\n".as_ptr());
            pdfendobj();
            pdfpagegroupval = 0;
        }
        write_refs();
        (*pdf_doc).in_obj_list = IN_OBJ_LIST;
        (*pdf_doc).xref = XREF;
    }
}

#[no_mangle]
pub unsafe extern "C" fn epdf_delete() {
    unsafe {
        let pdf_doc = epdf_doc as *mut PdfDocument;
        XREF = (*pdf_doc).xref;
        if (*pdf_doc).occurrences < 0 {
            delete_document(pdf_doc);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn epdf_check_mem() {
    unsafe {
        if IS_INIT != 0 {
            let mut p = PDF_DOCUMENTS;
            while !p.is_null() {
                let next = (*p).next;
                delete_document(p);
                p = next;
            }
            xpdf_global_params_free();
            IS_INIT = 0;
        }
    }
}

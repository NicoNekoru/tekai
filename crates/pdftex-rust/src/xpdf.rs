use libc::{c_char, c_double, c_float, c_int};

#[repr(C)]
pub struct Object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dict {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GString {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PDFDoc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Catalog {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct XRef {
    _private: [u8; 0],
}

#[repr(C)]
pub struct LinkDest {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GfxFont {
    _private: [u8; 0],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Ref {
    pub num: c_int,
    pub gen: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rect {
    pub x1: c_double,
    pub y1: c_double,
    pub x2: c_double,
    pub y2: c_double,
}

extern "C" {
    pub fn xpdf_global_params_init();
    pub fn xpdf_global_params_free();

    pub fn xpdf_object_new() -> *mut Object;
    pub fn xpdf_object_delete(_: *mut Object);
    pub fn xpdf_object_free_contents(_: *mut Object);
    pub fn xpdf_object_type_name(_: *mut Object) -> *const c_char;
    pub fn xpdf_object_is_bool(_: *mut Object) -> c_int;
    pub fn xpdf_object_is_int(_: *mut Object) -> c_int;
    pub fn xpdf_object_is_real(_: *mut Object) -> c_int;
    pub fn xpdf_object_is_num(_: *mut Object) -> c_int;
    pub fn xpdf_object_is_string(_: *mut Object) -> c_int;
    pub fn xpdf_object_is_name(_: *mut Object) -> c_int;
    pub fn xpdf_object_is_null(_: *mut Object) -> c_int;
    pub fn xpdf_object_is_array(_: *mut Object) -> c_int;
    pub fn xpdf_object_is_dict(_: *mut Object) -> c_int;
    pub fn xpdf_object_is_stream(_: *mut Object) -> c_int;
    pub fn xpdf_object_is_ref(_: *mut Object) -> c_int;
    pub fn xpdf_object_get_bool(_: *mut Object) -> c_int;
    pub fn xpdf_object_get_int(_: *mut Object) -> c_int;
    pub fn xpdf_object_get_real(_: *mut Object) -> c_double;
    pub fn xpdf_object_get_num(_: *mut Object) -> c_double;
    pub fn xpdf_object_get_string(_: *mut Object) -> *mut GString;
    pub fn xpdf_object_get_name(_: *mut Object) -> *mut c_char;
    pub fn xpdf_object_get_dict(_: *mut Object) -> *mut Dict;
    pub fn xpdf_object_get_stream(_: *mut Object) -> *mut Stream;
    pub fn xpdf_object_get_ref(_: *mut Object) -> Ref;
    pub fn xpdf_object_fetch(_: *mut Object, _: *mut XRef, _: *mut Object) -> *mut Object;
    pub fn xpdf_object_array_get_length(_: *mut Object) -> c_int;
    pub fn xpdf_object_array_get(_: *mut Object, _: c_int, _: *mut Object) -> *mut Object;
    pub fn xpdf_object_array_get_nf(_: *mut Object, _: c_int, _: *mut Object) -> *mut Object;
    pub fn xpdf_object_dict_get_length(_: *mut Object) -> c_int;
    pub fn xpdf_object_dict_get_key(_: *mut Object, _: c_int) -> *mut c_char;
    pub fn xpdf_object_dict_get_val(_: *mut Object, _: c_int, _: *mut Object) -> *mut Object;
    pub fn xpdf_object_dict_get_val_nf(_: *mut Object, _: c_int, _: *mut Object) -> *mut Object;
    pub fn xpdf_object_dict_lookup(
        _: *mut Object,
        _: *const c_char,
        _: *mut Object,
    ) -> *mut Object;
    pub fn xpdf_object_dict_lookup_nf(
        _: *mut Object,
        _: *const c_char,
        _: *mut Object,
    ) -> *mut Object;
    pub fn xpdf_object_stream_get_dict(_: *mut Object) -> *mut Dict;
    pub fn xpdf_object_stream_get_undecoded_stream(_: *mut Object) -> *mut Stream;
    pub fn xpdf_object_init_dict_from_dict(_: *mut Object, _: *mut XRef, _: *mut Dict);

    pub fn xpdf_dict_get_length(_: *mut Dict) -> c_int;
    pub fn xpdf_dict_get_key(_: *mut Dict, _: c_int) -> *mut c_char;
    pub fn xpdf_dict_get_val(_: *mut Dict, _: c_int, _: *mut Object) -> *mut Object;
    pub fn xpdf_dict_get_val_nf(_: *mut Dict, _: c_int, _: *mut Object) -> *mut Object;
    pub fn xpdf_dict_lookup(_: *mut Dict, _: *const c_char, _: *mut Object) -> *mut Object;
    pub fn xpdf_dict_lookup_nf(_: *mut Dict, _: *const c_char, _: *mut Object) -> *mut Object;

    pub fn xpdf_gstring_cstr(_: *mut GString) -> *const c_char;
    pub fn xpdf_gstring_len(_: *mut GString) -> c_int;
    pub fn xpdf_gstring_char(_: *mut GString, _: c_int) -> c_int;

    pub fn xpdf_stream_reset(_: *mut Stream);
    pub fn xpdf_stream_get_char(_: *mut Stream) -> c_int;
    pub fn xpdf_stream_get_dict(_: *mut Stream) -> *mut Dict;
    pub fn xpdf_stream_get_undecoded_stream(_: *mut Stream) -> *mut Stream;

    pub fn xpdf_doc_new(_: *const c_char) -> *mut PDFDoc;
    pub fn xpdf_doc_delete(_: *mut PDFDoc);
    pub fn xpdf_doc_is_ok(_: *mut PDFDoc) -> c_int;
    pub fn xpdf_doc_ok_to_print(_: *mut PDFDoc) -> c_int;
    pub fn xpdf_doc_pdf_version(_: *mut PDFDoc) -> c_float;
    pub fn xpdf_doc_catalog(_: *mut PDFDoc) -> *mut Catalog;
    pub fn xpdf_doc_xref(_: *mut PDFDoc) -> *mut XRef;
    pub fn xpdf_doc_find_dest(_: *mut PDFDoc, _: *const c_char) -> *mut LinkDest;
    pub fn xpdf_doc_get_info_nf(_: *mut PDFDoc, _: *mut Object);

    pub fn xpdf_link_dest_delete(_: *mut LinkDest);
    pub fn xpdf_link_dest_is_ok(_: *mut LinkDest) -> c_int;
    pub fn xpdf_link_dest_page_ref(_: *mut LinkDest) -> Ref;

    pub fn xpdf_catalog_num_pages(_: *mut Catalog) -> c_int;
    pub fn xpdf_catalog_get_page(_: *mut Catalog, _: c_int) -> *mut Page;
    pub fn xpdf_catalog_get_page_ref(_: *mut Catalog, _: c_int) -> Ref;
    pub fn xpdf_catalog_find_page(_: *mut Catalog, _: c_int, _: c_int) -> c_int;
    pub fn xpdf_xref_fetch(_: *mut XRef, _: c_int, _: c_int, _: *mut Object);

    pub fn xpdf_page_media_box(_: *mut Page) -> Rect;
    pub fn xpdf_page_crop_box(_: *mut Page) -> Rect;
    pub fn xpdf_page_bleed_box(_: *mut Page) -> Rect;
    pub fn xpdf_page_trim_box(_: *mut Page) -> Rect;
    pub fn xpdf_page_art_box(_: *mut Page) -> Rect;
    pub fn xpdf_page_rotate(_: *mut Page) -> c_int;
    pub fn xpdf_page_group(_: *mut Page) -> *mut Dict;
    pub fn xpdf_page_resource_dict(_: *mut Page) -> *mut Dict;
    pub fn xpdf_page_get_contents(_: *mut Page, _: *mut Object);

    pub fn xpdf_gfx_font_make(
        _: *mut XRef,
        _: *const c_char,
        _: Ref,
        _: *mut Dict,
    ) -> *mut GfxFont;
    pub fn xpdf_gfx_font_delete(_: *mut GfxFont);
    pub fn xpdf_gfx_font_is_cid(_: *mut GfxFont) -> c_int;
    pub fn xpdf_gfx_8bit_font_char_name(_: *mut GfxFont, _: c_int) -> *mut c_char;
}

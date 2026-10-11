use crate::allocation::free;
use crate::backend_grammar::{numeric_glyph_name, NumericGlyphName};
use crate::openclose::open_input;
use crate::openclose::{xmalloc, xstrdup};
use crate::support::{xfclose, xfseek, xftell};
use crate::ttf_name::{self, NameError, NameTable};
use crate::ttf_post::{self, OutputGlyphName, PostError, PostNames};
use crate::utils::xgetc;
use std::ffi::CStr;
use std::io::Read;

extern "C" {
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcasecmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    static mut nameoffile: *mut ASCIIcode;
    static mut pdfptr: integer;
    static mut pdfgone: longinteger;
    static mut pdfsaveoffset: longinteger;
    fn zpackfilename(n: strnumber, a: strnumber, e: strnumber);
    fn pdfflush();
    fn getnullstr() -> strnumber;
    fn avl_create(
        _: Option<avl_comparison_func>,
        _: *mut ::core::ffi::c_void,
        _: *mut libavl_allocator,
    ) -> *mut avl_table;
    fn avl_destroy(_: *mut avl_table, _: Option<avl_item_func>);
    fn avl_probe(_: *mut avl_table, _: *mut ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    fn avl_find(_: *const avl_table, _: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn avl_t_init(_: *mut avl_traverser, _: *mut avl_table);
    fn avl_t_first(_: *mut avl_traverser, _: *mut avl_table) -> *mut ::core::ffi::c_void;
    fn avl_t_next(_: *mut avl_traverser) -> *mut ::core::ffi::c_void;
    static mut avl_xallocator: libavl_allocator;
    static mut cur_file_name: *mut ::core::ffi::c_char;
    static mut fb_array: *mut ::core::ffi::c_char;
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn pdftex_warn(_: *const ::core::ffi::c_char, ...);
    fn tex_printf(_: *const ::core::ffi::c_char, ...);
    fn maketexstring(_: *const ::core::ffi::c_char) -> strnumber;
    fn fb_offset() -> integer;
    fn fb_putchar(b: eightbits);
    fn fb_seek(_: integer);
    fn make_subset_tag(_: *mut fd_entry);
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type int32_t = i32;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
pub use crate::stdio::Stream as FILE;
pub type off_t = __darwin_off_t;
pub type uint32_t = u32;
pub type boolean = ::core::ffi::c_int;
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const kpse_last_format: C2RustUnnamed = 59;
pub const kpse_bltxml_format: C2RustUnnamed = 58;
pub const kpse_ris_format: C2RustUnnamed = 57;
pub const kpse_clua_format: C2RustUnnamed = 56;
pub const kpse_mlbst_format: C2RustUnnamed = 55;
pub const kpse_mlbib_format: C2RustUnnamed = 54;
pub const kpse_cid_format: C2RustUnnamed = 53;
pub const kpse_fea_format: C2RustUnnamed = 52;
pub const kpse_lua_format: C2RustUnnamed = 51;
pub const kpse_texmfscripts_format: C2RustUnnamed = 50;
pub const kpse_lig_format: C2RustUnnamed = 49;
pub const kpse_pdftex_config_format: C2RustUnnamed = 48;
pub const kpse_opentype_format: C2RustUnnamed = 47;
pub const kpse_sfd_format: C2RustUnnamed = 46;
pub const kpse_cmap_format: C2RustUnnamed = 45;
pub const kpse_enc_format: C2RustUnnamed = 44;
pub const kpse_cweb_format: C2RustUnnamed = 43;
pub const kpse_web_format: C2RustUnnamed = 42;
pub const kpse_miscfonts_format: C2RustUnnamed = 41;
pub const kpse_program_binary_format: C2RustUnnamed = 40;
pub const kpse_program_text_format: C2RustUnnamed = 39;
pub const kpse_web2c_format: C2RustUnnamed = 38;
pub const kpse_type42_format: C2RustUnnamed = 37;
pub const kpse_truetype_format: C2RustUnnamed = 36;
pub const kpse_ist_format: C2RustUnnamed = 35;
pub const kpse_dvips_config_format: C2RustUnnamed = 34;
pub const kpse_vf_format: C2RustUnnamed = 33;
pub const kpse_type1_format: C2RustUnnamed = 32;
pub const kpse_troff_font_format: C2RustUnnamed = 31;
pub const kpse_tex_ps_header_format: C2RustUnnamed = 30;
pub const kpse_texsource_format: C2RustUnnamed = 29;
pub const kpse_texpool_format: C2RustUnnamed = 28;
pub const kpse_texdoc_format: C2RustUnnamed = 27;
pub const kpse_tex_format: C2RustUnnamed = 26;
pub const kpse_pict_format: C2RustUnnamed = 25;
pub const kpse_ovp_format: C2RustUnnamed = 24;
pub const kpse_ovf_format: C2RustUnnamed = 23;
pub const kpse_otp_format: C2RustUnnamed = 22;
pub const kpse_opl_format: C2RustUnnamed = 21;
pub const kpse_ofm_format: C2RustUnnamed = 20;
pub const kpse_ocp_format: C2RustUnnamed = 19;
pub const kpse_mpsupport_format: C2RustUnnamed = 18;
pub const kpse_mppool_format: C2RustUnnamed = 17;
pub const kpse_mp_format: C2RustUnnamed = 16;
pub const kpse_mft_format: C2RustUnnamed = 15;
pub const kpse_mfpool_format: C2RustUnnamed = 14;
pub const kpse_mf_format: C2RustUnnamed = 13;
pub const kpse_mem_format: C2RustUnnamed = 12;
pub const kpse_fontmap_format: C2RustUnnamed = 11;
pub const kpse_fmt_format: C2RustUnnamed = 10;
pub const kpse_db_format: C2RustUnnamed = 9;
pub const kpse_cnf_format: C2RustUnnamed = 8;
pub const kpse_bst_format: C2RustUnnamed = 7;
pub const kpse_bib_format: C2RustUnnamed = 6;
pub const kpse_base_format: C2RustUnnamed = 5;
pub const kpse_afm_format: C2RustUnnamed = 4;
pub const kpse_tfm_format: C2RustUnnamed = 3;
pub const kpse_any_glyph_format: C2RustUnnamed = 2;
pub const kpse_pk_format: C2RustUnnamed = 1;
pub const kpse_gf_format: C2RustUnnamed = 0;
pub type integer = ::core::ffi::c_int;
pub type longinteger = off_t;

pub type ASCIIcode = ::core::ffi::c_uchar;
pub type eightbits = ::core::ffi::c_uchar;
pub type strnumber = integer;
pub type avl_comparison_func = unsafe extern "C" fn(
    *const ::core::ffi::c_void,
    *const ::core::ffi::c_void,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub type avl_item_func =
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libavl_allocator {
    pub libavl_malloc:
        Option<unsafe extern "C" fn(*mut libavl_allocator, size_t) -> *mut ::core::ffi::c_void>,
    pub libavl_free:
        Option<unsafe extern "C" fn(*mut libavl_allocator, *mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avl_table {
    pub avl_root: *mut avl_node,
    pub avl_compare: Option<avl_comparison_func>,
    pub avl_param: *mut ::core::ffi::c_void,
    pub avl_alloc: *mut libavl_allocator,
    pub avl_count: size_t,
    pub avl_generation: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avl_node {
    pub avl_link: [*mut avl_node; 2],
    pub avl_data: *mut ::core::ffi::c_void,
    pub avl_balance: ::core::ffi::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avl_traverser {
    pub avl_table: *mut avl_table,
    pub avl_node: *mut avl_node,
    pub avl_stack: [*mut avl_node; 32],
    pub avl_height: size_t,
    pub avl_generation: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _subfont_entry {
    pub infix: *mut ::core::ffi::c_char,
    pub charcodes: [::core::ffi::c_long; 256],
    pub next: *mut subfont_entry,
}
pub type subfont_entry = _subfont_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fe_entry {
    pub fe_objnum: integer,
    pub name: *mut ::core::ffi::c_char,
    pub glyph_names: *mut *mut ::core::ffi::c_char,
    pub tx_tree: *mut avl_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fm_entry {
    pub tfm_name: *mut ::core::ffi::c_char,
    pub sfd_name: *mut ::core::ffi::c_char,
    pub ps_name: *mut ::core::ffi::c_char,
    pub fd_flags: integer,
    pub slant: integer,
    pub extend: integer,
    pub encname: *mut ::core::ffi::c_char,
    pub ff_name: *mut ::core::ffi::c_char,
    pub type_0: ::core::ffi::c_ushort,
    pub pid: ::core::ffi::c_short,
    pub eid: ::core::ffi::c_short,
    pub subfont: *mut subfont_entry,
    pub links: ::core::ffi::c_ushort,
    pub in_use: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intparm {
    pub val: ::core::ffi::c_int,
    pub set: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_entry_ {
    pub fd_objnum: integer,
    pub fontname: *mut ::core::ffi::c_char,
    pub subset_tag: *mut ::core::ffi::c_char,
    pub ff_found: boolean,
    pub ff_objnum: integer,
    pub fn_objnum: integer,
    pub all_glyphs: boolean,
    pub write_ttf_glyph_names: boolean,
    pub font_dim: [intparm; 11],
    pub fe: *mut fe_entry,
    pub builtin_glyph_names: *mut *mut ::core::ffi::c_char,
    pub fm: *mut fm_entry,
    pub tx_tree: *mut avl_table,
    pub gl_tree: *mut avl_table,
}
pub type fd_entry = fd_entry_;
pub type TTF_USHORT = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
// Runtime-only metadata, never serialized or passed across FFI.
pub struct glyph_entry {
    pub offset: TTF_LONG,
    pub newoffset: TTF_LONG,
    pub advWidth: TTF_UFWORD,
    pub lsb: TTF_FWORD,
    pub name: *const ::core::ffi::c_char,
    pub newindex: Option<TTF_USHORT>,
    pub name_index: TTF_USHORT,
}
pub type TTF_SHORT = ::core::ffi::c_short;
pub type TTF_FWORD = ::core::ffi::c_short;
pub type TTF_UFWORD = ::core::ffi::c_ushort;
pub type TTF_LONG = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirtab_entry {
    pub tag: [::core::ffi::c_char; 4],
    pub checksum: TTF_ULONG,
    pub offset: TTF_ULONG,
    pub length: TTF_ULONG,
}
pub type TTF_ULONG = uint32_t;
pub type TTF_CHAR = ::core::ffi::c_schar;
pub type TTF_BYTE = ::core::ffi::c_uchar;
pub type TTF_FIXED = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttfenc_entry {
    pub name: *const ::core::ffi::c_char,
    pub code: ::core::ffi::c_long,
    pub newindex: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmap_entry {
    pub platform_id: TTF_USHORT,
    pub encoding_id: TTF_USHORT,
    pub offset: TTF_ULONG,
    pub format: TTF_USHORT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttf_cmap_entry {
    pub ttf_name: *mut ::core::ffi::c_char,
    pub pid: TTF_USHORT,
    pub eid: TTF_USHORT,
    pub table: *mut ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seg_entry {
    pub endCode: TTF_USHORT,
    pub startCode: TTF_USHORT,
    pub idDelta: TTF_USHORT,
    pub idRangeOffset: TTF_USHORT,
}
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ASCENT_CODE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CAPHEIGHT_CODE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DESCENT_CODE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ITALIC_ANGLE_CODE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const XHEIGHT_CODE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const FONTBBOX1_CODE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const FONTBBOX2_CODE: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const FONTBBOX3_CODE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const FONTBBOX4_CODE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const FONTNAME_CODE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const F_SUBSETTED: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const F_SUBFONT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;

pub const TTF_CHAR_SIZE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TTF_BYTE_SIZE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TTF_SHORT_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TTF_USHORT_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TTF_ULONG_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TTF_FIXED_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TTF_FWORD_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TTF_UFWORD_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TTF_F2DOT14_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ARG_1_AND_2_ARE_WORDS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const WE_HAVE_A_SCALE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const MORE_COMPONENTS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const WE_HAVE_AN_X_AND_Y_SCALE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const WE_HAVE_A_TWO_BY_TWO: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
pub const WE_HAVE_INSTRUCTIONS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const NMACGLYPHS: ::core::ffi::c_int = 258 as ::core::ffi::c_int;
pub const TABDIR_OFF: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const GLYPH_PREFIX_UNICODE: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"uni\0") };
pub const DEFAULT_NTABS: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const NEW_CMAP_SIZE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

// Format-4 cmaps allocate one entry for every unsigned 16-bit character code.
const TTF_CMAP_ENTRIES: usize = 0x10000;

fn checked_cmap_code(code: ::core::ffi::c_long) -> Option<usize> {
    let code = usize::try_from(code).ok()?;
    (code < TTF_CMAP_ENTRIES).then_some(code)
}

// glyph_tab has an additional loca-only entry at glyph_count, never a glyph ID.
fn checked_glyph_index(index: ::core::ffi::c_long, glyph_count: TTF_USHORT) -> Option<usize> {
    let index = usize::try_from(index).ok()?;
    (index < usize::from(glyph_count)).then_some(index)
}

fn checked_post_name_count(count: ::core::ffi::c_long, glyph_count: TTF_USHORT) -> Option<usize> {
    let count = usize::try_from(count).ok()?;
    (count <= usize::from(glyph_count)).then_some(count)
}

// Name-selected glyphs must exclude the reserved .notdef glyph and loca sentinel.
// Invalid numeric names, including index0, warn and keep output index zero.
fn checked_selected_glyph_index(
    index: ::core::ffi::c_long,
    glyph_count: TTF_USHORT,
) -> Option<usize> {
    checked_glyph_index(index, glyph_count).filter(|&index| index > 0)
}

fn initial_glyph_newindex(index: usize) -> Option<TTF_USHORT> {
    match index {
        0 => Some(0),
        1 => Some(1),
        _ => None,
    }
}

// A subset cannot contain more glyphs than its source. This also keeps every
// insertion inside glyph_index's allocation of glyph_count + 1 slots.
fn checked_output_glyph_index(
    index: ::core::ffi::c_long,
    glyph_count: TTF_USHORT,
) -> Option<usize> {
    checked_glyph_index(index, glyph_count)
}

unsafe fn ttf_append_glyph(source_index: ::core::ffi::c_long) -> TTF_USHORT {
    let index = checked_glyph_index(source_index, glyphs_count).unwrap_or_else(|| {
        crate::utils::pdftex_fail_args(
            b"glyph index %li out of range [0..%i)\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(source_index),
                crate::utils::PrintfArg::from(glyphs_count as ::core::ffi::c_int),
            ],
        )
    });
    let output = checked_output_glyph_index(new_glyphs_count as ::core::ffi::c_long, glyphs_count)
        .unwrap_or_else(|| {
            crate::utils::pdftex_fail_args(
                b"TrueType subset output index %i exceeds source glyph count %i\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(new_glyphs_count as ::core::ffi::c_int),
                    crate::utils::PrintfArg::from(glyphs_count as ::core::ffi::c_int),
                ],
            )
        });
    let next_count = new_glyphs_count.checked_add(1).unwrap_or_else(|| {
        crate::utils::pdftex_fail_args(
            b"TrueType subset glyph count exceeds unsigned 16-bit capacity\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        )
    });
    *glyph_index.add(output) = source_index;
    (*glyph_tab.add(index)).newindex = Some(new_glyphs_count);
    let output_index = new_glyphs_count;
    new_glyphs_count = next_count;
    output_index
}

static mut ntabs: TTF_USHORT = 0;
static mut upem: TTF_USHORT = 0;
static mut post_format: TTF_FIXED = 0;
static mut loca_format: TTF_SHORT = 0;
static mut last_glyf_offset: TTF_ULONG = 0;
static mut glyphs_count: TTF_USHORT = 0;
static mut new_glyphs_count: TTF_USHORT = 0;
static mut nhmtxs: TTF_USHORT = 0;
static mut new_ntabs: TTF_USHORT = 0;
static mut glyph_tab: *mut glyph_entry = ::core::ptr::null::<glyph_entry>() as *mut glyph_entry;
static mut glyph_index: *mut ::core::ffi::c_long =
    ::core::ptr::null::<::core::ffi::c_long>() as *mut ::core::ffi::c_long;
static mut new_cmap_tab: [cmap_entry; 2] = [cmap_entry {
    platform_id: 0,
    encoding_id: 0,
    offset: 0,
    format: 0,
}; 2];
static mut dir_tab: *mut dirtab_entry = ::core::ptr::null::<dirtab_entry>() as *mut dirtab_entry;
static mut checksum: TTF_ULONG = 0;
static mut tab_length: TTF_ULONG = 0;
static mut tmp_ulong: TTF_ULONG = 0;
static mut checkSumAdjustment_offset: TTF_ULONG = 0;
static mut ttf_file: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
static mut ttfenc_tab: [ttfenc_entry; 256] = [ttfenc_entry {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    code: 0,
    newindex: 0,
}; 256];
static mut fd_cur: *mut fd_entry = ::core::ptr::null::<fd_entry>() as *mut fd_entry;
static mut ttf_cmap_tree: *mut avl_table = ::core::ptr::null::<avl_table>() as *mut avl_table;
#[no_mangle]
pub static mut ttf_length: integer = 0;
#[no_mangle]
pub static mut notdef: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b".notdef\0") };
#[no_mangle]
pub static mut mac_glyph_names: [*const ::core::ffi::c_char; 258] = unsafe {
    [
        &raw const notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char,
        b".null\0" as *const u8 as *const ::core::ffi::c_char,
        b"CR\0" as *const u8 as *const ::core::ffi::c_char,
        b"space\0" as *const u8 as *const ::core::ffi::c_char,
        b"exclam\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotedbl\0" as *const u8 as *const ::core::ffi::c_char,
        b"numbersign\0" as *const u8 as *const ::core::ffi::c_char,
        b"dollar\0" as *const u8 as *const ::core::ffi::c_char,
        b"percent\0" as *const u8 as *const ::core::ffi::c_char,
        b"ampersand\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotesingle\0" as *const u8 as *const ::core::ffi::c_char,
        b"parenleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"parenright\0" as *const u8 as *const ::core::ffi::c_char,
        b"asterisk\0" as *const u8 as *const ::core::ffi::c_char,
        b"plus\0" as *const u8 as *const ::core::ffi::c_char,
        b"comma\0" as *const u8 as *const ::core::ffi::c_char,
        b"hyphen\0" as *const u8 as *const ::core::ffi::c_char,
        b"period\0" as *const u8 as *const ::core::ffi::c_char,
        b"slash\0" as *const u8 as *const ::core::ffi::c_char,
        b"zero\0" as *const u8 as *const ::core::ffi::c_char,
        b"one\0" as *const u8 as *const ::core::ffi::c_char,
        b"two\0" as *const u8 as *const ::core::ffi::c_char,
        b"three\0" as *const u8 as *const ::core::ffi::c_char,
        b"four\0" as *const u8 as *const ::core::ffi::c_char,
        b"five\0" as *const u8 as *const ::core::ffi::c_char,
        b"six\0" as *const u8 as *const ::core::ffi::c_char,
        b"seven\0" as *const u8 as *const ::core::ffi::c_char,
        b"eight\0" as *const u8 as *const ::core::ffi::c_char,
        b"nine\0" as *const u8 as *const ::core::ffi::c_char,
        b"colon\0" as *const u8 as *const ::core::ffi::c_char,
        b"semicolon\0" as *const u8 as *const ::core::ffi::c_char,
        b"less\0" as *const u8 as *const ::core::ffi::c_char,
        b"equal\0" as *const u8 as *const ::core::ffi::c_char,
        b"greater\0" as *const u8 as *const ::core::ffi::c_char,
        b"question\0" as *const u8 as *const ::core::ffi::c_char,
        b"at\0" as *const u8 as *const ::core::ffi::c_char,
        b"A\0" as *const u8 as *const ::core::ffi::c_char,
        b"B\0" as *const u8 as *const ::core::ffi::c_char,
        b"C\0" as *const u8 as *const ::core::ffi::c_char,
        b"D\0" as *const u8 as *const ::core::ffi::c_char,
        b"E\0" as *const u8 as *const ::core::ffi::c_char,
        b"F\0" as *const u8 as *const ::core::ffi::c_char,
        b"G\0" as *const u8 as *const ::core::ffi::c_char,
        b"H\0" as *const u8 as *const ::core::ffi::c_char,
        b"I\0" as *const u8 as *const ::core::ffi::c_char,
        b"J\0" as *const u8 as *const ::core::ffi::c_char,
        b"K\0" as *const u8 as *const ::core::ffi::c_char,
        b"L\0" as *const u8 as *const ::core::ffi::c_char,
        b"M\0" as *const u8 as *const ::core::ffi::c_char,
        b"N\0" as *const u8 as *const ::core::ffi::c_char,
        b"O\0" as *const u8 as *const ::core::ffi::c_char,
        b"P\0" as *const u8 as *const ::core::ffi::c_char,
        b"Q\0" as *const u8 as *const ::core::ffi::c_char,
        b"R\0" as *const u8 as *const ::core::ffi::c_char,
        b"S\0" as *const u8 as *const ::core::ffi::c_char,
        b"T\0" as *const u8 as *const ::core::ffi::c_char,
        b"U\0" as *const u8 as *const ::core::ffi::c_char,
        b"V\0" as *const u8 as *const ::core::ffi::c_char,
        b"W\0" as *const u8 as *const ::core::ffi::c_char,
        b"X\0" as *const u8 as *const ::core::ffi::c_char,
        b"Y\0" as *const u8 as *const ::core::ffi::c_char,
        b"Z\0" as *const u8 as *const ::core::ffi::c_char,
        b"bracketleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"backslash\0" as *const u8 as *const ::core::ffi::c_char,
        b"bracketright\0" as *const u8 as *const ::core::ffi::c_char,
        b"asciicircum\0" as *const u8 as *const ::core::ffi::c_char,
        b"underscore\0" as *const u8 as *const ::core::ffi::c_char,
        b"grave\0" as *const u8 as *const ::core::ffi::c_char,
        b"a\0" as *const u8 as *const ::core::ffi::c_char,
        b"b\0" as *const u8 as *const ::core::ffi::c_char,
        b"c\0" as *const u8 as *const ::core::ffi::c_char,
        b"d\0" as *const u8 as *const ::core::ffi::c_char,
        b"e\0" as *const u8 as *const ::core::ffi::c_char,
        b"f\0" as *const u8 as *const ::core::ffi::c_char,
        b"g\0" as *const u8 as *const ::core::ffi::c_char,
        b"h\0" as *const u8 as *const ::core::ffi::c_char,
        b"i\0" as *const u8 as *const ::core::ffi::c_char,
        b"j\0" as *const u8 as *const ::core::ffi::c_char,
        b"k\0" as *const u8 as *const ::core::ffi::c_char,
        b"l\0" as *const u8 as *const ::core::ffi::c_char,
        b"m\0" as *const u8 as *const ::core::ffi::c_char,
        b"n\0" as *const u8 as *const ::core::ffi::c_char,
        b"o\0" as *const u8 as *const ::core::ffi::c_char,
        b"p\0" as *const u8 as *const ::core::ffi::c_char,
        b"q\0" as *const u8 as *const ::core::ffi::c_char,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
        b"s\0" as *const u8 as *const ::core::ffi::c_char,
        b"t\0" as *const u8 as *const ::core::ffi::c_char,
        b"u\0" as *const u8 as *const ::core::ffi::c_char,
        b"v\0" as *const u8 as *const ::core::ffi::c_char,
        b"w\0" as *const u8 as *const ::core::ffi::c_char,
        b"x\0" as *const u8 as *const ::core::ffi::c_char,
        b"y\0" as *const u8 as *const ::core::ffi::c_char,
        b"z\0" as *const u8 as *const ::core::ffi::c_char,
        b"braceleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"bar\0" as *const u8 as *const ::core::ffi::c_char,
        b"braceright\0" as *const u8 as *const ::core::ffi::c_char,
        b"asciitilde\0" as *const u8 as *const ::core::ffi::c_char,
        b"Adieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"Aring\0" as *const u8 as *const ::core::ffi::c_char,
        b"Ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
        b"Eacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"Ntilde\0" as *const u8 as *const ::core::ffi::c_char,
        b"Odieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"Udieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"aacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"agrave\0" as *const u8 as *const ::core::ffi::c_char,
        b"acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"adieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"atilde\0" as *const u8 as *const ::core::ffi::c_char,
        b"aring\0" as *const u8 as *const ::core::ffi::c_char,
        b"ccedilla\0" as *const u8 as *const ::core::ffi::c_char,
        b"eacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"egrave\0" as *const u8 as *const ::core::ffi::c_char,
        b"ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"edieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"iacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"igrave\0" as *const u8 as *const ::core::ffi::c_char,
        b"icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"idieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"ntilde\0" as *const u8 as *const ::core::ffi::c_char,
        b"oacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"ograve\0" as *const u8 as *const ::core::ffi::c_char,
        b"ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"odieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"otilde\0" as *const u8 as *const ::core::ffi::c_char,
        b"uacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"ugrave\0" as *const u8 as *const ::core::ffi::c_char,
        b"ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"udieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"dagger\0" as *const u8 as *const ::core::ffi::c_char,
        b"degree\0" as *const u8 as *const ::core::ffi::c_char,
        b"cent\0" as *const u8 as *const ::core::ffi::c_char,
        b"sterling\0" as *const u8 as *const ::core::ffi::c_char,
        b"section\0" as *const u8 as *const ::core::ffi::c_char,
        b"bullet\0" as *const u8 as *const ::core::ffi::c_char,
        b"paragraph\0" as *const u8 as *const ::core::ffi::c_char,
        b"germandbls\0" as *const u8 as *const ::core::ffi::c_char,
        b"registered\0" as *const u8 as *const ::core::ffi::c_char,
        b"copyright\0" as *const u8 as *const ::core::ffi::c_char,
        b"trademark\0" as *const u8 as *const ::core::ffi::c_char,
        b"acute\0" as *const u8 as *const ::core::ffi::c_char,
        b"dieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"notequal\0" as *const u8 as *const ::core::ffi::c_char,
        b"AE\0" as *const u8 as *const ::core::ffi::c_char,
        b"Oslash\0" as *const u8 as *const ::core::ffi::c_char,
        b"infinity\0" as *const u8 as *const ::core::ffi::c_char,
        b"plusminus\0" as *const u8 as *const ::core::ffi::c_char,
        b"lessequal\0" as *const u8 as *const ::core::ffi::c_char,
        b"greaterequal\0" as *const u8 as *const ::core::ffi::c_char,
        b"yen\0" as *const u8 as *const ::core::ffi::c_char,
        b"mu\0" as *const u8 as *const ::core::ffi::c_char,
        b"partialdiff\0" as *const u8 as *const ::core::ffi::c_char,
        b"Sigma\0" as *const u8 as *const ::core::ffi::c_char,
        b"Pi\0" as *const u8 as *const ::core::ffi::c_char,
        b"pi\0" as *const u8 as *const ::core::ffi::c_char,
        b"integral\0" as *const u8 as *const ::core::ffi::c_char,
        b"ordfeminine\0" as *const u8 as *const ::core::ffi::c_char,
        b"ordmasculine\0" as *const u8 as *const ::core::ffi::c_char,
        b"Omega\0" as *const u8 as *const ::core::ffi::c_char,
        b"ae\0" as *const u8 as *const ::core::ffi::c_char,
        b"oslash\0" as *const u8 as *const ::core::ffi::c_char,
        b"questiondown\0" as *const u8 as *const ::core::ffi::c_char,
        b"exclamdown\0" as *const u8 as *const ::core::ffi::c_char,
        b"logicalnot\0" as *const u8 as *const ::core::ffi::c_char,
        b"radical\0" as *const u8 as *const ::core::ffi::c_char,
        b"florin\0" as *const u8 as *const ::core::ffi::c_char,
        b"approxequal\0" as *const u8 as *const ::core::ffi::c_char,
        b"Delta\0" as *const u8 as *const ::core::ffi::c_char,
        b"guillemotleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"guillemotright\0" as *const u8 as *const ::core::ffi::c_char,
        b"ellipsis\0" as *const u8 as *const ::core::ffi::c_char,
        b"nbspace\0" as *const u8 as *const ::core::ffi::c_char,
        b"Agrave\0" as *const u8 as *const ::core::ffi::c_char,
        b"Atilde\0" as *const u8 as *const ::core::ffi::c_char,
        b"Otilde\0" as *const u8 as *const ::core::ffi::c_char,
        b"OE\0" as *const u8 as *const ::core::ffi::c_char,
        b"oe\0" as *const u8 as *const ::core::ffi::c_char,
        b"endash\0" as *const u8 as *const ::core::ffi::c_char,
        b"emdash\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotedblleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotedblright\0" as *const u8 as *const ::core::ffi::c_char,
        b"quoteleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"quoteright\0" as *const u8 as *const ::core::ffi::c_char,
        b"divide\0" as *const u8 as *const ::core::ffi::c_char,
        b"lozenge\0" as *const u8 as *const ::core::ffi::c_char,
        b"ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"Ydieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"fraction\0" as *const u8 as *const ::core::ffi::c_char,
        b"currency\0" as *const u8 as *const ::core::ffi::c_char,
        b"guilsinglleft\0" as *const u8 as *const ::core::ffi::c_char,
        b"guilsinglright\0" as *const u8 as *const ::core::ffi::c_char,
        b"fi\0" as *const u8 as *const ::core::ffi::c_char,
        b"fl\0" as *const u8 as *const ::core::ffi::c_char,
        b"daggerdbl\0" as *const u8 as *const ::core::ffi::c_char,
        b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotesinglbase\0" as *const u8 as *const ::core::ffi::c_char,
        b"quotedblbase\0" as *const u8 as *const ::core::ffi::c_char,
        b"perthousand\0" as *const u8 as *const ::core::ffi::c_char,
        b"Acircumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"Ecircumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"Aacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"Edieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"Egrave\0" as *const u8 as *const ::core::ffi::c_char,
        b"Iacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"Icircumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"Idieresis\0" as *const u8 as *const ::core::ffi::c_char,
        b"Igrave\0" as *const u8 as *const ::core::ffi::c_char,
        b"Oacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"Ocircumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"applelogo\0" as *const u8 as *const ::core::ffi::c_char,
        b"Ograve\0" as *const u8 as *const ::core::ffi::c_char,
        b"Uacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"Ucircumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"Ugrave\0" as *const u8 as *const ::core::ffi::c_char,
        b"dotlessi\0" as *const u8 as *const ::core::ffi::c_char,
        b"circumflex\0" as *const u8 as *const ::core::ffi::c_char,
        b"tilde\0" as *const u8 as *const ::core::ffi::c_char,
        b"macron\0" as *const u8 as *const ::core::ffi::c_char,
        b"breve\0" as *const u8 as *const ::core::ffi::c_char,
        b"dotaccent\0" as *const u8 as *const ::core::ffi::c_char,
        b"ring\0" as *const u8 as *const ::core::ffi::c_char,
        b"cedilla\0" as *const u8 as *const ::core::ffi::c_char,
        b"hungarumlaut\0" as *const u8 as *const ::core::ffi::c_char,
        b"ogonek\0" as *const u8 as *const ::core::ffi::c_char,
        b"caron\0" as *const u8 as *const ::core::ffi::c_char,
        b"Lslash\0" as *const u8 as *const ::core::ffi::c_char,
        b"lslash\0" as *const u8 as *const ::core::ffi::c_char,
        b"Scaron\0" as *const u8 as *const ::core::ffi::c_char,
        b"scaron\0" as *const u8 as *const ::core::ffi::c_char,
        b"Zcaron\0" as *const u8 as *const ::core::ffi::c_char,
        b"zcaron\0" as *const u8 as *const ::core::ffi::c_char,
        b"brokenbar\0" as *const u8 as *const ::core::ffi::c_char,
        b"Eth\0" as *const u8 as *const ::core::ffi::c_char,
        b"eth\0" as *const u8 as *const ::core::ffi::c_char,
        b"Yacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"yacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"Thorn\0" as *const u8 as *const ::core::ffi::c_char,
        b"thorn\0" as *const u8 as *const ::core::ffi::c_char,
        b"minus\0" as *const u8 as *const ::core::ffi::c_char,
        b"multiply\0" as *const u8 as *const ::core::ffi::c_char,
        b"onesuperior\0" as *const u8 as *const ::core::ffi::c_char,
        b"twosuperior\0" as *const u8 as *const ::core::ffi::c_char,
        b"threesuperior\0" as *const u8 as *const ::core::ffi::c_char,
        b"onehalf\0" as *const u8 as *const ::core::ffi::c_char,
        b"onequarter\0" as *const u8 as *const ::core::ffi::c_char,
        b"threequarters\0" as *const u8 as *const ::core::ffi::c_char,
        b"franc\0" as *const u8 as *const ::core::ffi::c_char,
        b"Gbreve\0" as *const u8 as *const ::core::ffi::c_char,
        b"gbreve\0" as *const u8 as *const ::core::ffi::c_char,
        b"Idot\0" as *const u8 as *const ::core::ffi::c_char,
        b"Scedilla\0" as *const u8 as *const ::core::ffi::c_char,
        b"scedilla\0" as *const u8 as *const ::core::ffi::c_char,
        b"Cacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"cacute\0" as *const u8 as *const ::core::ffi::c_char,
        b"Ccaron\0" as *const u8 as *const ::core::ffi::c_char,
        b"ccaron\0" as *const u8 as *const ::core::ffi::c_char,
        b"dmacron\0" as *const u8 as *const ::core::ffi::c_char,
    ]
};
#[no_mangle]
pub static mut ambiguous_names: [*const ::core::ffi::c_char; 8] = [
    b"Delta\0" as *const u8 as *const ::core::ffi::c_char,
    b"Omega\0" as *const u8 as *const ::core::ffi::c_char,
    b"Pi\0" as *const u8 as *const ::core::ffi::c_char,
    b"Sigma\0" as *const u8 as *const ::core::ffi::c_char,
    b"dmacron\0" as *const u8 as *const ::core::ffi::c_char,
    b"macron\0" as *const u8 as *const ::core::ffi::c_char,
    b"periodcentered\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut newtabnames: [*const ::core::ffi::c_char; 14] = [
    b"OS/2\0" as *const u8 as *const ::core::ffi::c_char,
    b"PCLT\0" as *const u8 as *const ::core::ffi::c_char,
    b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
    b"cvt \0" as *const u8 as *const ::core::ffi::c_char,
    b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
    b"glyf\0" as *const u8 as *const ::core::ffi::c_char,
    b"head\0" as *const u8 as *const ::core::ffi::c_char,
    b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
    b"hmtx\0" as *const u8 as *const ::core::ffi::c_char,
    b"loca\0" as *const u8 as *const ::core::ffi::c_char,
    b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
    b"name\0" as *const u8 as *const ::core::ffi::c_char,
    b"post\0" as *const u8 as *const ::core::ffi::c_char,
    b"prep\0" as *const u8 as *const ::core::ffi::c_char,
];
unsafe extern "C" fn new_ttf_cmap_entry() -> *mut ttf_cmap_entry {
    let mut e: *mut ttf_cmap_entry = ::core::ptr::null_mut::<ttf_cmap_entry>();
    e = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<ttf_cmap_entry>() as size_t))
        as *mut ttf_cmap_entry;
    (*e).ttf_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*e).table = ::core::ptr::null_mut::<::core::ffi::c_long>();
    return e;
}
unsafe extern "C" fn destroy_ttf_cmap_entry(
    mut pa: *mut ::core::ffi::c_void,
    mut pb: *mut ::core::ffi::c_void,
) {
    let mut p: *mut ttf_cmap_entry = ::core::ptr::null_mut::<ttf_cmap_entry>();
    p = pa as *mut ttf_cmap_entry;
    if !(*p).ttf_name.is_null() {
        free((*p).ttf_name as *mut ::core::ffi::c_void);
    }
    (*p).ttf_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*p).table.is_null() {
        free((*p).table as *mut ::core::ffi::c_void);
    }
    (*p).table = ::core::ptr::null_mut::<::core::ffi::c_long>();
    if !p.is_null() {
        free(p as *mut ::core::ffi::c_void);
    }
    p = ::core::ptr::null_mut::<ttf_cmap_entry>();
}
#[no_mangle]
pub unsafe extern "C" fn ttf_free() {
    if !ttf_cmap_tree.is_null() {
        avl_destroy(
            ttf_cmap_tree,
            Some(
                destroy_ttf_cmap_entry
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
        );
    }
}
unsafe extern "C" fn comp_ttf_cmap_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p1: *const ttf_cmap_entry = pa as *const ttf_cmap_entry;
    let mut p2: *const ttf_cmap_entry = pb as *const ttf_cmap_entry;
    let mut i: ::core::ffi::c_int = 0;
    if !(!(*p1).ttf_name.is_null() && !(*p2).ttf_name.is_null()) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"comp_ttf_cmap_entry\0" as *const u8 as *const ::core::ffi::c_char,
            b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
            153 as ::core::ffi::c_int,
            b"p1->ttf_name != NULL && p2->ttf_name != NULL\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    i = strcmp((*p1).ttf_name, (*p2).ttf_name);
    if i != 0 as ::core::ffi::c_int {
        return i;
    }
    if (*p1).pid as ::core::ffi::c_int > (*p2).pid as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if ((*p1).pid as ::core::ffi::c_int) < (*p2).pid as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if (*p1).eid as ::core::ffi::c_int > (*p2).eid as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if ((*p1).eid as ::core::ffi::c_int) < (*p2).eid as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ttf_addchksm(mut b: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar {
    tmp_ulong = (tmp_ulong << 8 as ::core::ffi::c_int).wrapping_add(b as TTF_ULONG);
    tab_length = tab_length.wrapping_add(1);
    if tab_length.wrapping_rem(4 as TTF_ULONG) == 0 as TTF_ULONG {
        checksum = checksum.wrapping_add(tmp_ulong);
        tmp_ulong = 0 as TTF_ULONG;
    }
    return b;
}
unsafe extern "C" fn ttf_getchksm() -> TTF_ULONG {
    while tab_length.wrapping_rem(4 as TTF_ULONG) != 0 as TTF_ULONG {
        fb_putchar(ttf_addchksm(0 as ::core::ffi::c_uchar) as eightbits);
    }
    return checksum;
}
unsafe extern "C" fn ttf_putnum(
    mut s: ::core::ffi::c_int,
    mut n: ::core::ffi::c_long,
) -> ::core::ffi::c_long {
    let mut i: ::core::ffi::c_long = n;
    let mut buf: [::core::ffi::c_char; 5] = [0; 5];
    let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
    loop {
        let fresh1 = s;
        s = s - 1;
        if !(fresh1 > 0 as ::core::ffi::c_int) {
            break;
        }
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = (i & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
        i >>= 8 as ::core::ffi::c_int;
    }
    p = p.offset(-1);
    while p >= &raw mut buf as *mut ::core::ffi::c_char {
        let fresh3 = p;
        p = p.offset(-1);
        fb_putchar(ttf_addchksm(*fresh3 as ::core::ffi::c_uchar) as eightbits);
    }
    return n;
}
unsafe extern "C" fn ttf_getnum(mut s: ::core::ffi::c_int) -> ::core::ffi::c_long {
    let mut i: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    let mut c: ::core::ffi::c_int = 0;
    while s > 0 as ::core::ffi::c_int {
        c = xgetc(ttf_file);
        if c < 0 as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"unexpected EOF\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
        i = (i << 8 as ::core::ffi::c_int) + c as ::core::ffi::c_long;
        s -= 1;
    }
    return i;
}
unsafe extern "C" fn ttf_funit(mut n: ::core::ffi::c_long) -> ::core::ffi::c_long {
    if n < 0 as ::core::ffi::c_long {
        return -(-n / upem as ::core::ffi::c_long * 1000 as ::core::ffi::c_long
            + -n % upem as ::core::ffi::c_long * 1000 as ::core::ffi::c_long
                / upem as ::core::ffi::c_long);
    } else {
        return n / upem as ::core::ffi::c_long * 1000 as ::core::ffi::c_long
            + n % upem as ::core::ffi::c_long * 1000 as ::core::ffi::c_long
                / upem as ::core::ffi::c_long;
    };
}
unsafe extern "C" fn ttf_ncopy(mut n: ::core::ffi::c_int) {
    loop {
        let fresh4 = n;
        n = n - 1;
        if !(fresh4 > 0 as ::core::ffi::c_int) {
            break;
        }
        ttf_putnum(
            TTF_BYTE_SIZE,
            ttf_getnum(1 as ::core::ffi::c_int) as TTF_BYTE as ::core::ffi::c_long,
        );
    }
}
unsafe extern "C" fn ttf_name_lookup(
    mut s: *const ::core::ffi::c_char,
    mut required: boolean,
) -> *mut dirtab_entry {
    let mut tab: *mut dirtab_entry = ::core::ptr::null_mut::<dirtab_entry>();
    tab = dir_tab;
    while (tab.offset_from(dir_tab) as ::core::ffi::c_long) < ntabs as ::core::ffi::c_long {
        if strncmp(
            &raw mut (*tab).tag as *mut ::core::ffi::c_char,
            s,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            break;
        }
        tab = tab.offset(1);
    }
    if tab.offset_from(dir_tab) as ::core::ffi::c_long == ntabs as ::core::ffi::c_long {
        if required != 0 {
            crate::utils::pdftex_fail_args(
                b"can't find table `%s'\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(s)],
            );
        } else {
            tab = ::core::ptr::null_mut::<dirtab_entry>();
        }
    }
    return tab;
}
unsafe extern "C" fn ttf_seek_tab(
    mut name: *const ::core::ffi::c_char,
    mut offset: TTF_LONG,
) -> *mut dirtab_entry {
    let mut tab: *mut dirtab_entry = ttf_name_lookup(name, true_0);
    xfseek(
        ttf_file,
        (*tab).offset.wrapping_add(offset as TTF_ULONG) as ::core::ffi::c_long,
        SEEK_SET,
        cur_file_name as const_string,
    );
    return tab;
}
unsafe extern "C" fn ttf_seek_off(mut offset: TTF_LONG) {
    xfseek(
        ttf_file,
        offset as ::core::ffi::c_long,
        SEEK_SET,
        cur_file_name as const_string,
    );
}
unsafe extern "C" fn ttf_copy_encoding() {
    let mut i: ::core::ffi::c_int = 0;
    let mut q: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut glyph_names: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut charcodes: *mut ::core::ffi::c_long = ::core::ptr::null_mut::<::core::ffi::c_long>();
    static mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    let mut e: *mut ttfenc_entry = &raw mut ttfenc_tab as *mut ttfenc_entry;
    if (*fd_cur).tx_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"ttf_copy_encoding\0" as *const u8 as *const ::core::ffi::c_char,
            b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
            257 as ::core::ffi::c_int,
            b"fd_cur->tx_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !(*fd_cur).fe.is_null() {
        glyph_names = (*(*fd_cur).fe).glyph_names;
        if glyph_names.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"ttf_copy_encoding\0" as *const u8 as *const ::core::ffi::c_char,
                b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                261 as ::core::ffi::c_int,
                b"glyph_names != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            ttfenc_tab[i as usize].name = &raw mut notdef as *mut ::core::ffi::c_char;
            i += 1;
        }
        if strcmp(
            *glyph_names.offset(97 as ::core::ffi::c_int as isize),
            b"a\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            q = xmalloc(
                (1 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
            ) as *mut ::core::ffi::c_int;
            *q = 'a' as i32;
            aa = avl_probe((*fd_cur).tx_tree, q as *mut ::core::ffi::c_void);
            if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"ttf_copy_encoding\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                    271 as ::core::ffi::c_int,
                    b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
        }
        avl_t_init(&raw mut t, (*fd_cur).tx_tree);
        q = avl_t_first(&raw mut t, (*fd_cur).tx_tree) as *mut ::core::ffi::c_int;
        while !q.is_null() {
            if !(*q >= 0 as ::core::ffi::c_int && *q < 256 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"ttf_copy_encoding\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                    279 as ::core::ffi::c_int,
                    b"*q >= 0 && *q < 256\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            ttfenc_tab[*q as usize].name = *glyph_names.offset(*q as isize);
            q = avl_t_next(&raw mut t) as *mut ::core::ffi::c_int;
        }
        make_subset_tag(fd_cur);
    } else if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBFONT != 0 as ::core::ffi::c_int {
        charcodes = &raw mut (*(*(*fd_cur).fm).subfont).charcodes as *mut ::core::ffi::c_long;
        if charcodes.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"ttf_copy_encoding\0" as *const u8 as *const ::core::ffi::c_char,
                b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                285 as ::core::ffi::c_int,
                b"charcodes != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        i = 0 as ::core::ffi::c_int;
        while i < 256 as ::core::ffi::c_int {
            ttfenc_tab[i as usize].code = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
            i += 1;
        }
        avl_t_init(&raw mut t, (*fd_cur).tx_tree);
        q = avl_t_first(&raw mut t, (*fd_cur).tx_tree) as *mut ::core::ffi::c_int;
        while !q.is_null() {
            if !(*q >= 0 as ::core::ffi::c_int && *q < 256 as ::core::ffi::c_int)
                as ::core::ffi::c_int as ::core::ffi::c_long
                != 0
            {
                __assert_rtn(
                    b"ttf_copy_encoding\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                    294 as ::core::ffi::c_int,
                    b"*q >= 0 && *q < 256\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            e = (&raw mut ttfenc_tab as *mut ttfenc_entry).offset(*q as isize);
            (*e).code = *charcodes.offset(*q as isize);
            if checked_cmap_code((*e).code).is_none() {
                (*e).code = -1;
                crate::utils::pdftex_warn_args(
                    b"character %i in subfont %s is not mapped to any charcode\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(*q),
                        crate::utils::PrintfArg::from((*(*fd_cur).fm).tfm_name),
                    ],
                );
            } else {
                crate::utils::sprintf_args(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    b"/c%4.4X\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(
                        (*e).code as ::core::ffi::c_int,
                    )],
                );
                aa = avl_probe(
                    (*fd_cur).gl_tree,
                    xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
                        as *mut ::core::ffi::c_void,
                );
                if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                    __assert_rtn(
                        b"ttf_copy_encoding\0" as *const u8 as *const ::core::ffi::c_char,
                        b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                        305 as ::core::ffi::c_int,
                        b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                } else {
                };
            }
            q = avl_t_next(&raw mut t) as *mut ::core::ffi::c_int;
        }
        make_subset_tag(fd_cur);
    } else {
        if (0 as ::core::ffi::c_int == 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"ttf_copy_encoding\0" as *const u8 as *const ::core::ffi::c_char,
                b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                310 as ::core::ffi::c_int,
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    };
}
unsafe fn name_failure(error: NameError) -> ! {
    crate::utils::pdftex_fail_args(error.message().as_ptr(), &[])
}

unsafe fn ttf_read_name() -> NameTable {
    let tab = ttf_name_lookup(c"name".as_ptr(), true_0);
    xfseek(ttf_file, 0, libc::SEEK_END, cur_file_name as const_string);
    let file_length = xftell(ttf_file, cur_file_name as const_string) as u64;
    let extent = ttf_name::checked_table_extent((*tab).offset, (*tab).length, file_length)
        .unwrap_or_else(|error| name_failure(error));
    xfseek(
        ttf_file,
        extent.start as ::core::ffi::c_long,
        SEEK_SET,
        cur_file_name as const_string,
    );
    let names = ttf_name::read_table(PostReader(ttf_file), extent.end - extent.start)
        .unwrap_or_else(|error| name_failure(error));
    let font_name = names
        .font_name((*fd_cur).font_dim[FONTNAME_CODE as usize].set == 0)
        .unwrap_or_else(|error| name_failure(error));
    if let Some(font_name) = font_name {
        if !(*fd_cur).fontname.is_null() {
            free((*fd_cur).fontname.cast());
        }
        // fd_cur owns this legacy allocation beyond the stack-owned table.
        (*fd_cur).fontname = xstrdup(font_name.as_ptr());
        (*fd_cur).font_dim[FONTNAME_CODE as usize].set = true_0;
    }
    names
}
unsafe extern "C" fn ttf_read_mapx() {
    let mut glyph: *mut glyph_entry = ::core::ptr::null_mut::<glyph_entry>();
    ttf_seek_tab(
        b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
        TTF_FIXED_SIZE as TTF_LONG,
    );
    glyphs_count = ttf_getnum(2 as ::core::ffi::c_int) as TTF_USHORT;
    if checked_glyph_index(1, glyphs_count).is_none() {
        crate::utils::pdftex_fail_args(
            b"unsupported TrueType font: glyph count %i is below required minimum 2\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                glyphs_count as ::core::ffi::c_int,
            )],
        );
    }
    glyph_tab = xmalloc(
        ((1 as ::core::ffi::c_int + glyphs_count as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<glyph_entry>() as size_t),
    ) as *mut glyph_entry;
    glyph = glyph_tab;
    while (glyph.offset_from(glyph_tab) as ::core::ffi::c_long)
        < glyphs_count as ::core::ffi::c_long
    {
        // The prefilled output slots own source glyphs 0 and 1 from the start.
        (*glyph).newindex = initial_glyph_newindex(glyph.offset_from(glyph_tab) as usize);
        (*glyph).newoffset = 0 as ::core::ffi::c_int as TTF_LONG;
        (*glyph).name_index = 0 as TTF_USHORT;
        (*glyph).name = &raw mut notdef as *mut ::core::ffi::c_char;
        glyph = glyph.offset(1);
    }
    (*glyph).newindex = None;
    glyph_index = xmalloc(
        ((glyphs_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as size_t),
    ) as *mut ::core::ffi::c_long;
    *glyph_index.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_long;
    *glyph_index.offset(1 as ::core::ffi::c_int as isize) = 1 as ::core::ffi::c_long;
}
unsafe extern "C" fn ttf_read_head() {
    ttf_seek_tab(
        b"head\0" as *const u8 as *const ::core::ffi::c_char,
        2 as TTF_LONG * TTF_FIXED_SIZE as TTF_LONG
            + 2 as TTF_LONG * TTF_ULONG_SIZE as TTF_LONG
            + TTF_USHORT_SIZE as TTF_LONG,
    );
    upem = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
    ttf_getnum(16 as ::core::ffi::c_int);
    (*fd_cur).font_dim[FONTBBOX1_CODE as usize].val =
        ttf_funit(ttf_getnum(TTF_FWORD_SIZE) as TTF_FWORD as ::core::ffi::c_long)
            as ::core::ffi::c_int;
    (*fd_cur).font_dim[FONTBBOX2_CODE as usize].val =
        ttf_funit(ttf_getnum(TTF_FWORD_SIZE) as TTF_FWORD as ::core::ffi::c_long)
            as ::core::ffi::c_int;
    (*fd_cur).font_dim[FONTBBOX3_CODE as usize].val =
        ttf_funit(ttf_getnum(TTF_FWORD_SIZE) as TTF_FWORD as ::core::ffi::c_long)
            as ::core::ffi::c_int;
    (*fd_cur).font_dim[FONTBBOX4_CODE as usize].val =
        ttf_funit(ttf_getnum(TTF_FWORD_SIZE) as TTF_FWORD as ::core::ffi::c_long)
            as ::core::ffi::c_int;
    (*fd_cur).font_dim[FONTBBOX1_CODE as usize].set = true_0 as boolean;
    (*fd_cur).font_dim[FONTBBOX2_CODE as usize].set = true_0 as boolean;
    (*fd_cur).font_dim[FONTBBOX3_CODE as usize].set = true_0 as boolean;
    (*fd_cur).font_dim[FONTBBOX4_CODE as usize].set = true_0 as boolean;
    ttf_getnum(2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int);
    loca_format = ttf_getnum(TTF_SHORT_SIZE) as TTF_SHORT;
}
unsafe extern "C" fn ttf_read_hhea() {
    ttf_seek_tab(
        b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        TTF_FIXED_SIZE as TTF_LONG,
    );
    (*fd_cur).font_dim[ASCENT_CODE as usize].val =
        ttf_funit(ttf_getnum(TTF_FWORD_SIZE) as TTF_FWORD as ::core::ffi::c_long)
            as ::core::ffi::c_int;
    (*fd_cur).font_dim[DESCENT_CODE as usize].val =
        ttf_funit(ttf_getnum(TTF_FWORD_SIZE) as TTF_FWORD as ::core::ffi::c_long)
            as ::core::ffi::c_int;
    (*fd_cur).font_dim[ASCENT_CODE as usize].set = true_0 as boolean;
    (*fd_cur).font_dim[DESCENT_CODE as usize].set = true_0 as boolean;
    ttf_getnum(
        2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
    );
    nhmtxs = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
}
unsafe extern "C" fn ttf_read_pclt() {
    if ttf_name_lookup(
        b"PCLT\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        return;
    }
    ttf_seek_tab(
        b"PCLT\0" as *const u8 as *const ::core::ffi::c_char,
        TTF_FIXED_SIZE as TTF_LONG + TTF_ULONG_SIZE as TTF_LONG + TTF_USHORT_SIZE as TTF_LONG,
    );
    (*fd_cur).font_dim[XHEIGHT_CODE as usize].val =
        ttf_funit(ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT as ::core::ffi::c_long)
            as ::core::ffi::c_int;
    ttf_getnum(2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int);
    (*fd_cur).font_dim[CAPHEIGHT_CODE as usize].val =
        ttf_funit(ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT as ::core::ffi::c_long)
            as ::core::ffi::c_int;
    (*fd_cur).font_dim[XHEIGHT_CODE as usize].set = true_0 as boolean;
    (*fd_cur).font_dim[CAPHEIGHT_CODE as usize].set = true_0 as boolean;
}
unsafe extern "C" fn ttf_read_hmtx() {
    let mut glyph: *mut glyph_entry = ::core::ptr::null_mut::<glyph_entry>();
    let mut last_advWidth: TTF_UFWORD = 0;
    if checked_glyph_index(nhmtxs as ::core::ffi::c_long - 1, glyphs_count).is_none() {
        crate::utils::pdftex_fail_args(
            b"hmtx: metric count %i out of range [1..%i]\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(nhmtxs as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(glyphs_count as ::core::ffi::c_int),
            ],
        );
    }
    ttf_seek_tab(
        b"hmtx\0" as *const u8 as *const ::core::ffi::c_char,
        0 as TTF_LONG,
    );
    glyph = glyph_tab;
    while (glyph.offset_from(glyph_tab) as ::core::ffi::c_long) < nhmtxs as ::core::ffi::c_long {
        (*glyph).advWidth = ttf_getnum(TTF_UFWORD_SIZE) as TTF_UFWORD;
        (*glyph).lsb = ttf_getnum(TTF_UFWORD_SIZE) as TTF_UFWORD as TTF_FWORD;
        glyph = glyph.offset(1);
    }
    if (nhmtxs as ::core::ffi::c_int) < glyphs_count as ::core::ffi::c_int {
        last_advWidth = (*glyph.offset(-(1 as ::core::ffi::c_int) as isize)).advWidth;
        while (glyph.offset_from(glyph_tab) as ::core::ffi::c_long)
            < glyphs_count as ::core::ffi::c_long
        {
            (*glyph).advWidth = last_advWidth;
            (*glyph).lsb = ttf_getnum(TTF_UFWORD_SIZE) as TTF_UFWORD as TTF_FWORD;
            glyph = glyph.offset(1);
        }
    }
}
struct PostReader(*mut FILE);

impl Read for PostReader {
    fn read(&mut self, bytes: &mut [u8]) -> std::io::Result<usize> {
        let count =
            unsafe { crate::stdio::fread(bytes.as_mut_ptr().cast(), 1, bytes.len(), self.0) };
        if count == 0 && unsafe { crate::stdio::ferror(self.0) } != 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(count)
        }
    }
}

unsafe fn post_failure(error: PostError) -> ! {
    crate::utils::pdftex_fail_args(error.message().as_ptr(), &[])
}

unsafe fn ttf_read_post() -> Option<PostNames> {
    let mut int_part: ::core::ffi::c_long = 0;
    let mut frac_part: ::core::ffi::c_long = 0;
    let mut sign: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut italic_angle: TTF_FIXED = 0;
    let tab = ttf_name_lookup(b"post\0" as *const u8 as *const ::core::ffi::c_char, true_0);
    xfseek(ttf_file, 0, libc::SEEK_END, cur_file_name as const_string);
    let file_length = xftell(ttf_file, cur_file_name as const_string) as u64;
    let extent = ttf_post::checked_table_extent((*tab).offset, (*tab).length, file_length)
        .unwrap_or_else(|error| post_failure(error));
    xfseek(
        ttf_file,
        extent.start as ::core::ffi::c_long,
        SEEK_SET,
        cur_file_name as const_string,
    );
    post_format = ttf_getnum(TTF_FIXED_SIZE) as TTF_FIXED;
    italic_angle = ttf_getnum(TTF_FIXED_SIZE) as TTF_FIXED;
    int_part = (italic_angle >> 16 as ::core::ffi::c_int) as ::core::ffi::c_long;
    if int_part > 0x7fff as ::core::ffi::c_long {
        int_part = 0x10000 as ::core::ffi::c_long - int_part;
        sign = -(1 as ::core::ffi::c_int);
    }
    frac_part = italic_angle.wrapping_rem(0x10000 as TTF_FIXED) as ::core::ffi::c_long;
    (*fd_cur).font_dim[ITALIC_ANGLE_CODE as usize].val = (sign as ::core::ffi::c_double
        * (int_part as ::core::ffi::c_double
            + frac_part as ::core::ffi::c_double * 1.0f64
                / 0x10000 as ::core::ffi::c_int as ::core::ffi::c_double))
        as ::core::ffi::c_int;
    (*fd_cur).font_dim[ITALIC_ANGLE_CODE as usize].set = true_0 as boolean;
    if glyph_tab.is_null() {
        return None;
    }
    ttf_getnum(
        2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
    );
    match post_format {
        65536 => {
            if checked_post_name_count(NMACGLYPHS as ::core::ffi::c_long, glyphs_count).is_none() {
                crate::utils::pdftex_fail_args(
                    b"post: glyph name count %i exceeds font glyph count %i\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(NMACGLYPHS),
                        crate::utils::PrintfArg::from(glyphs_count as ::core::ffi::c_int),
                    ],
                );
            }
            for index in 0..ttf_post::STANDARD_NAMES {
                let glyph = glyph_tab.add(index);
                (*glyph).name = mac_glyph_names[index];
                (*glyph).name_index = index as TTF_USHORT;
            }
        }
        131072 => {
            let names = ttf_post::read_format2(
                PostReader(ttf_file),
                u64::from((*tab).length) - 32,
                glyphs_count,
            )
            .unwrap_or_else(|error| post_failure(error));
            for (index, &name_index) in names.indices().iter().enumerate() {
                let glyph = glyph_tab.add(index);
                (*glyph).name_index = name_index;
                (*glyph).name = if usize::from(name_index) < ttf_post::STANDARD_NAMES {
                    mac_glyph_names[usize::from(name_index)]
                } else {
                    names
                        .custom(name_index)
                        .unwrap_or_else(|| post_failure(PostError::NameIndex))
                        .as_ptr()
                };
            }
            return Some(names);
        }
        _ => {
            if post_format != 0x30000 {
                crate::utils::pdftex_warn_args(
                    b"unsupported format (%.8X) of `post' table, assuming 3.0\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(
                        post_format as ::core::ffi::c_uint,
                    )],
                );
            }
            for index in 0..ttf_post::STANDARD_NAMES.min(usize::from(glyphs_count)) {
                (*glyph_tab.add(index)).name_index = index as TTF_USHORT;
            }
        }
    }
    None
}
unsafe extern "C" fn ttf_read_loca() {
    let mut glyph: *mut glyph_entry = ::core::ptr::null_mut::<glyph_entry>();
    ttf_seek_tab(
        b"loca\0" as *const u8 as *const ::core::ffi::c_char,
        0 as TTF_LONG,
    );
    if loca_format as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        glyph = glyph_tab;
        while (glyph.offset_from(glyph_tab) as ::core::ffi::c_long)
            < (glyphs_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            (*glyph).offset = ttf_getnum(TTF_ULONG_SIZE) as TTF_ULONG as TTF_LONG;
            glyph = glyph.offset(1);
        }
    } else {
        glyph = glyph_tab;
        while (glyph.offset_from(glyph_tab) as ::core::ffi::c_long)
            < (glyphs_count as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            (*glyph).offset = ((ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) as TTF_LONG;
            glyph = glyph.offset(1);
        }
    };
}
unsafe extern "C" fn ttf_read_tabdir() {
    let mut i: ::core::ffi::c_int = 0;
    let mut tab: *mut dirtab_entry = ::core::ptr::null_mut::<dirtab_entry>();
    ttf_getnum(4 as ::core::ffi::c_int);
    ntabs = ttf_getnum(2 as ::core::ffi::c_int) as TTF_USHORT;
    dir_tab =
        xmalloc((ntabs as size_t).wrapping_mul(::core::mem::size_of::<dirtab_entry>() as size_t))
            as *mut dirtab_entry;
    ttf_getnum(3 as ::core::ffi::c_int * 2 as ::core::ffi::c_int);
    tab = dir_tab;
    while (tab.offset_from(dir_tab) as ::core::ffi::c_long) < ntabs as ::core::ffi::c_long {
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            (*tab).tag[i as usize] = ttf_getnum(TTF_CHAR_SIZE) as TTF_CHAR as ::core::ffi::c_char;
            i += 1;
        }
        (*tab).checksum = ttf_getnum(TTF_ULONG_SIZE) as TTF_ULONG;
        (*tab).offset = ttf_getnum(TTF_ULONG_SIZE) as TTF_ULONG;
        (*tab).length = ttf_getnum(TTF_ULONG_SIZE) as TTF_ULONG;
        tab = tab.offset(1);
    }
}
unsafe extern "C" fn ttf_read_cmap(
    mut ttf_name: *mut ::core::ffi::c_char,
    mut pid: ::core::ffi::c_int,
    mut eid: ::core::ffi::c_int,
    mut warn: boolean,
) -> *mut ttf_cmap_entry {
    let mut current_block: u64;
    let mut seg_tab: *mut seg_entry = ::core::ptr::null_mut::<seg_entry>();
    let mut s: *mut seg_entry = ::core::ptr::null_mut::<seg_entry>();
    let mut glyphId: *mut TTF_USHORT = ::core::ptr::null_mut::<TTF_USHORT>();
    let mut format: TTF_USHORT = 0;
    let mut segCount: TTF_USHORT = 0;
    let mut ncmapsubtabs: TTF_USHORT = 0;
    let mut tmp_pid: TTF_USHORT = 0;
    let mut tmp_eid: TTF_USHORT = 0;
    let mut cmap_offset: TTF_ULONG = 0;
    let mut tmp_offset: TTF_ULONG = 0;
    let mut n: ::core::ffi::c_long = 0;
    let mut i: ::core::ffi::c_long = 0;
    let mut k_0: ::core::ffi::c_long = 0;
    let mut length: ::core::ffi::c_long = 0;
    let mut index: ::core::ffi::c_long = 0;
    let mut tmp_e: ttf_cmap_entry = ttf_cmap_entry {
        ttf_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pid: 0,
        eid: 0,
        table: ::core::ptr::null_mut::<::core::ffi::c_long>(),
    };
    let mut p: *mut ttf_cmap_entry = ::core::ptr::null_mut::<ttf_cmap_entry>();
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    tmp_e.ttf_name = ttf_name;
    tmp_e.pid = pid as TTF_USHORT;
    tmp_e.eid = eid as TTF_USHORT;
    if ttf_cmap_tree.is_null() {
        ttf_cmap_tree = avl_create(
            Some(
                comp_ttf_cmap_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if ttf_cmap_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"ttf_read_cmap\0" as *const u8 as *const ::core::ffi::c_char,
                b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                575 as ::core::ffi::c_int,
                b"ttf_cmap_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    p = avl_find(ttf_cmap_tree, &raw mut tmp_e as *const ::core::ffi::c_void)
        as *mut ttf_cmap_entry;
    if !p.is_null() {
        return p;
    }
    ttf_seek_tab(
        b"cmap\0" as *const u8 as *const ::core::ffi::c_char,
        TTF_USHORT_SIZE as TTF_LONG,
    );
    ncmapsubtabs = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
    cmap_offset = (xftell(ttf_file, cur_file_name as const_string)
        - (2 as ::core::ffi::c_int * TTF_USHORT_SIZE) as ::core::ffi::c_long)
        as TTF_ULONG;
    i = 0 as ::core::ffi::c_long;
    loop {
        if !(i < ncmapsubtabs as ::core::ffi::c_long) {
            current_block = 14576567515993809846;
            break;
        }
        tmp_pid = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
        tmp_eid = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
        tmp_offset = ttf_getnum(TTF_ULONG_SIZE) as TTF_ULONG;
        if tmp_pid as ::core::ffi::c_int == pid && tmp_eid as ::core::ffi::c_int == eid {
            ttf_seek_off(cmap_offset.wrapping_add(tmp_offset) as TTF_LONG);
            format = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
            if format as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
                current_block = 16671468535566315319;
                break;
            }
            if warn != 0 {
                crate::utils::pdftex_warn_args(
                    b"cmap format %i unsupported\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(format as ::core::ffi::c_int)],
                );
            }
            return ::core::ptr::null_mut::<ttf_cmap_entry>();
        } else {
            i += 1;
        }
    }
    match current_block {
        14576567515993809846 => {
            if warn != 0 {
                crate::utils::pdftex_warn_args(
                    b"cannot find cmap subtable for (pid,eid) = (%i, %i)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from(pid),
                        crate::utils::PrintfArg::from(eid),
                    ],
                );
            }
            return ::core::ptr::null_mut::<ttf_cmap_entry>();
        }
        _ => {
            p = new_ttf_cmap_entry();
            (*p).ttf_name = xstrdup(ttf_name as const_string) as *mut ::core::ffi::c_char;
            (*p).pid = pid as TTF_USHORT;
            (*p).eid = eid as TTF_USHORT;
            (*p).table = xmalloc(
                (TTF_CMAP_ENTRIES as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_long>() as size_t),
            ) as *mut ::core::ffi::c_long;
            i = 0 as ::core::ffi::c_long;
            while i < TTF_CMAP_ENTRIES as ::core::ffi::c_long {
                *(*p).table.offset(i as isize) = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
                i += 1;
            }
            length = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT as ::core::ffi::c_long;
            ttf_getnum(TTF_USHORT_SIZE);
            segCount = (ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT as ::core::ffi::c_int
                / 2 as ::core::ffi::c_int) as TTF_USHORT;
            ttf_getnum(TTF_USHORT_SIZE);
            ttf_getnum(TTF_USHORT_SIZE);
            ttf_getnum(TTF_USHORT_SIZE);
            seg_tab = xmalloc(
                (segCount as size_t).wrapping_mul(::core::mem::size_of::<seg_entry>() as size_t),
            ) as *mut seg_entry;
            s = seg_tab;
            while (s.offset_from(seg_tab) as ::core::ffi::c_long) < segCount as ::core::ffi::c_long
            {
                (*s).endCode = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
                s = s.offset(1);
            }
            ttf_getnum(TTF_USHORT_SIZE);
            s = seg_tab;
            while (s.offset_from(seg_tab) as ::core::ffi::c_long) < segCount as ::core::ffi::c_long
            {
                (*s).startCode = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
                s = s.offset(1);
            }
            s = seg_tab;
            while (s.offset_from(seg_tab) as ::core::ffi::c_long) < segCount as ::core::ffi::c_long
            {
                (*s).idDelta = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
                s = s.offset(1);
            }
            s = seg_tab;
            while (s.offset_from(seg_tab) as ::core::ffi::c_long) < segCount as ::core::ffi::c_long
            {
                (*s).idRangeOffset = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
                s = s.offset(1);
            }
            length -= (8 as ::core::ffi::c_int * TTF_USHORT_SIZE
                + 4 as ::core::ffi::c_int * segCount as ::core::ffi::c_int * TTF_USHORT_SIZE)
                as ::core::ffi::c_long;
            n = length / TTF_USHORT_SIZE as ::core::ffi::c_long;
            glyphId =
                xmalloc((n as size_t).wrapping_mul(::core::mem::size_of::<TTF_USHORT>() as size_t))
                    as *mut TTF_USHORT;
            i = 0 as ::core::ffi::c_long;
            while i < n {
                *glyphId.offset(i as isize) = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
                i += 1;
            }
            s = seg_tab;
            while (s.offset_from(seg_tab) as ::core::ffi::c_long) < segCount as ::core::ffi::c_long
            {
                i = (*s).startCode as ::core::ffi::c_long;
                while i <= (*s).endCode as ::core::ffi::c_long {
                    if i == 0xffff as ::core::ffi::c_long {
                        break;
                    }
                    if (*s).idRangeOffset as ::core::ffi::c_int != 0xffff as ::core::ffi::c_int {
                        if (*s).idRangeOffset as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            index = (*s).idDelta as ::core::ffi::c_long + i
                                & 0xffff as ::core::ffi::c_long;
                        } else {
                            k_0 = i - (*s).startCode as ::core::ffi::c_long
                                + ((*s).idRangeOffset as ::core::ffi::c_int
                                    / 2 as ::core::ffi::c_int)
                                    as ::core::ffi::c_long
                                + s.offset_from(seg_tab) as ::core::ffi::c_long
                                - segCount as ::core::ffi::c_long;
                            if !(k_0 >= 0 as ::core::ffi::c_long && k_0 < n) as ::core::ffi::c_int
                                as ::core::ffi::c_long
                                != 0
                            {
                                __assert_rtn(
                                    b"ttf_read_cmap\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                                    648 as ::core::ffi::c_int,
                                    b"k >= 0 && k < n\0" as *const u8 as *const ::core::ffi::c_char,
                                );
                            } else {
                            };
                            index = *glyphId.offset(k_0 as isize) as ::core::ffi::c_long;
                            if index != 0 as ::core::ffi::c_long {
                                index = index + (*s).idDelta as ::core::ffi::c_long
                                    & 0xffff as ::core::ffi::c_long;
                            }
                        }
                        if checked_glyph_index(index, glyphs_count).is_none() {
                            crate::utils::pdftex_fail_args(
                                b"cmap: glyph index %li out of range [0..%i)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[
                                    crate::utils::PrintfArg::from(index),
                                    crate::utils::PrintfArg::from(
                                        glyphs_count as ::core::ffi::c_int,
                                    ),
                                ],
                            );
                        }
                        if *(*p).table.offset(i as isize)
                            != -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
                        {
                            crate::utils::pdftex_warn_args(b"cmap: multiple glyphs are mapped to unicode %.4lX, only %li will be used (glyph %li being ignored)\0"
                                    as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(i), crate::utils::PrintfArg::from(*(*p).table.offset(i as isize)), crate::utils::PrintfArg::from(index)]);
                        } else {
                            *(*p).table.offset(i as isize) = index;
                        }
                    }
                    i += 1;
                }
                s = s.offset(1);
            }
            if !seg_tab.is_null() {
                free(seg_tab as *mut ::core::ffi::c_void);
            }
            seg_tab = ::core::ptr::null_mut::<seg_entry>();
            if !glyphId.is_null() {
                free(glyphId as *mut ::core::ffi::c_void);
            }
            glyphId = ::core::ptr::null_mut::<TTF_USHORT>();
            aa = avl_probe(ttf_cmap_tree, p as *mut ::core::ffi::c_void);
            if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                __assert_rtn(
                    b"ttf_read_cmap\0" as *const u8 as *const ::core::ffi::c_char,
                    b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                    669 as ::core::ffi::c_int,
                    b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
            };
            return p;
        }
    };
}
unsafe fn ttf_read_font() -> (Option<PostNames>, NameTable) {
    ttf_read_tabdir();
    if ttf_name_lookup(
        b"PCLT\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        new_ntabs = new_ntabs.wrapping_sub(1);
    }
    if ttf_name_lookup(
        b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        new_ntabs = new_ntabs.wrapping_sub(1);
    }
    if ttf_name_lookup(
        b"cvt \0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        new_ntabs = new_ntabs.wrapping_sub(1);
    }
    if ttf_name_lookup(
        b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        new_ntabs = new_ntabs.wrapping_sub(1);
    }
    ttf_read_mapx();
    ttf_read_head();
    ttf_read_hhea();
    ttf_read_pclt();
    ttf_read_hmtx();
    let glyph_names = ttf_read_post();
    ttf_read_loca();
    let names = ttf_read_name();
    (glyph_names, names)
}
unsafe extern "C" fn ttf_reset_chksm(mut tab: *mut dirtab_entry) {
    checksum = 0 as TTF_ULONG;
    tab_length = 0 as TTF_ULONG;
    tmp_ulong = 0 as TTF_ULONG;
    (*tab).offset = fb_offset() as TTF_ULONG;
    if (*tab).offset.wrapping_rem(4 as TTF_ULONG) != 0 as TTF_ULONG {
        crate::utils::pdftex_warn_args(
            b"offset of `%4.4s' is not a multiple of 4\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                &raw mut (*tab).tag as *mut ::core::ffi::c_char,
            )],
        );
    }
}
unsafe extern "C" fn ttf_set_chksm(mut tab: *mut dirtab_entry) {
    (*tab).length = (fb_offset() as TTF_ULONG).wrapping_sub((*tab).offset);
    (*tab).checksum = ttf_getchksm();
}
unsafe extern "C" fn ttf_copytab(mut name: *const ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_long = 0;
    let mut tab: *mut dirtab_entry = ttf_seek_tab(name, 0 as TTF_LONG);
    ttf_reset_chksm(tab);
    i = (*tab).length as ::core::ffi::c_long;
    while i > 0 as ::core::ffi::c_long {
        ttf_putnum(
            TTF_CHAR_SIZE,
            ttf_getnum(1 as ::core::ffi::c_int) as TTF_CHAR as ::core::ffi::c_long,
        );
        i -= 1;
    }
    ttf_set_chksm(tab);
}
pub const BYTE_ENCODING_LENGTH: ::core::ffi::c_int =
    256 as ::core::ffi::c_int * TTF_BYTE_SIZE + 3 as ::core::ffi::c_int * TTF_USHORT_SIZE;
unsafe extern "C" fn ttf_byte_encoding() {
    let mut e: *mut ttfenc_entry = ::core::ptr::null_mut::<ttfenc_entry>();
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    ttf_putnum(
        TTF_USHORT_SIZE,
        (256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long,
    );
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    e = &raw mut ttfenc_tab as *mut ttfenc_entry;
    while (e.offset_from(&raw mut ttfenc_tab as *mut ttfenc_entry) as ::core::ffi::c_long)
        < 256 as ::core::ffi::c_long
    {
        if (*e).newindex < 256 as ::core::ffi::c_long {
            ttf_putnum(TTF_BYTE_SIZE, (*e).newindex);
        } else {
            if (*e).name
                != &raw mut notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
            {
                crate::utils::pdftex_warn_args(
                    b"glyph `%s' has been mapped to `%s' in `ttf_byte_encoding' cmap table\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    &[
                        crate::utils::PrintfArg::from((*e).name),
                        crate::utils::PrintfArg::from(&raw mut notdef as *mut ::core::ffi::c_char),
                    ],
                );
            }
            ttf_putnum(TTF_BYTE_SIZE, 0 as ::core::ffi::c_long);
        }
        e = e.offset(1);
    }
}
pub const TRIMMED_TABLE_MAP_LENGTH: ::core::ffi::c_int =
    TTF_USHORT_SIZE * (5 as ::core::ffi::c_int + 256 as ::core::ffi::c_int);
unsafe extern "C" fn ttf_trimmed_table_map() {
    let mut e: *mut ttfenc_entry = ::core::ptr::null_mut::<ttfenc_entry>();
    ttf_putnum(TTF_USHORT_SIZE, 6 as ::core::ffi::c_long);
    ttf_putnum(
        TTF_USHORT_SIZE,
        (2 as ::core::ffi::c_int * (5 as ::core::ffi::c_int + 256 as ::core::ffi::c_int))
            as ::core::ffi::c_long,
    );
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 256 as ::core::ffi::c_long);
    e = &raw mut ttfenc_tab as *mut ttfenc_entry;
    while (e.offset_from(&raw mut ttfenc_tab as *mut ttfenc_entry) as ::core::ffi::c_long)
        < 256 as ::core::ffi::c_long
    {
        ttf_putnum(TTF_USHORT_SIZE, (*e).newindex);
        e = e.offset(1);
    }
}
pub const SEG_MAP_DELTA_LENGTH: ::core::ffi::c_int =
    (16 as ::core::ffi::c_int + 256 as ::core::ffi::c_int) * TTF_USHORT_SIZE;
unsafe extern "C" fn ttf_seg_map_delta() {
    let mut e: *mut ttfenc_entry = ::core::ptr::null_mut::<ttfenc_entry>();
    ttf_putnum(TTF_USHORT_SIZE, 4 as ::core::ffi::c_long);
    ttf_putnum(
        TTF_USHORT_SIZE,
        ((16 as ::core::ffi::c_int + 256 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int)
            as ::core::ffi::c_long,
    );
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 4 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 4 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 1 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 0xf0ff as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 0xffff as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 0xf000 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 0xffff as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 1 as ::core::ffi::c_long);
    ttf_putnum(
        TTF_USHORT_SIZE,
        (2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as ::core::ffi::c_long,
    );
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    e = &raw mut ttfenc_tab as *mut ttfenc_entry;
    while (e.offset_from(&raw mut ttfenc_tab as *mut ttfenc_entry) as ::core::ffi::c_long)
        < 256 as ::core::ffi::c_long
    {
        ttf_putnum(TTF_USHORT_SIZE, (*e).newindex);
        e = e.offset(1);
    }
}
pub const CMAP_ENTRY_LENGTH: ::core::ffi::c_int =
    2 as ::core::ffi::c_int * TTF_USHORT_SIZE + TTF_ULONG_SIZE;
unsafe extern "C" fn ttf_select_cmap() {
    if !(::core::mem::size_of::<[cmap_entry; 2]>() as usize
        <= (2 as usize).wrapping_mul(::core::mem::size_of::<cmap_entry>() as usize))
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"ttf_select_cmap\0" as *const u8 as *const ::core::ffi::c_char,
            b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
            785 as ::core::ffi::c_int,
            b"sizeof(new_cmap_tab) <= NEW_CMAP_SIZE * sizeof(cmap_entry)\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    } else {
    };
    new_cmap_tab[0 as ::core::ffi::c_int as usize].platform_id = 1 as TTF_USHORT;
    new_cmap_tab[0 as ::core::ffi::c_int as usize].encoding_id = 0 as TTF_USHORT;
    new_cmap_tab[0 as ::core::ffi::c_int as usize].format =
        (if (new_glyphs_count as ::core::ffi::c_int) < 256 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            6 as ::core::ffi::c_int
        }) as TTF_USHORT;
    new_cmap_tab[1 as ::core::ffi::c_int as usize].platform_id = 3 as TTF_USHORT;
    new_cmap_tab[1 as ::core::ffi::c_int as usize].encoding_id = 0 as TTF_USHORT;
    new_cmap_tab[1 as ::core::ffi::c_int as usize].format = 4 as TTF_USHORT;
}
unsafe extern "C" fn ttf_write_cmap() {
    let mut ce: *mut cmap_entry = ::core::ptr::null_mut::<cmap_entry>();
    let mut offset: ::core::ffi::c_long = 0;
    let mut tab: *mut dirtab_entry =
        ttf_name_lookup(b"cmap\0" as *const u8 as *const ::core::ffi::c_char, true_0);
    ttf_select_cmap();
    ttf_reset_chksm(tab);
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 2 as ::core::ffi::c_long);
    offset = (2 as ::core::ffi::c_int * TTF_USHORT_SIZE + NEW_CMAP_SIZE * CMAP_ENTRY_LENGTH)
        as ::core::ffi::c_long;
    ce = &raw mut new_cmap_tab as *mut cmap_entry;
    while (ce.offset_from(&raw mut new_cmap_tab as *mut cmap_entry) as ::core::ffi::c_long)
        < NEW_CMAP_SIZE as ::core::ffi::c_long
    {
        (*ce).offset = offset as TTF_ULONG;
        match (*ce).format as ::core::ffi::c_int {
            0 => {
                offset += BYTE_ENCODING_LENGTH as ::core::ffi::c_long;
            }
            4 => {
                offset += SEG_MAP_DELTA_LENGTH as ::core::ffi::c_long;
            }
            6 => {
                offset += TRIMMED_TABLE_MAP_LENGTH as ::core::ffi::c_long;
            }
            _ => {
                crate::utils::pdftex_fail_args(
                    b"invalid format (it should not have happened)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[],
                );
            }
        }
        ttf_putnum(TTF_USHORT_SIZE, (*ce).platform_id as ::core::ffi::c_long);
        ttf_putnum(TTF_USHORT_SIZE, (*ce).encoding_id as ::core::ffi::c_long);
        ttf_putnum(TTF_ULONG_SIZE, (*ce).offset as ::core::ffi::c_long);
        ce = ce.offset(1);
    }
    ce = &raw mut new_cmap_tab as *mut cmap_entry;
    while (ce.offset_from(&raw mut new_cmap_tab as *mut cmap_entry) as ::core::ffi::c_long)
        < NEW_CMAP_SIZE as ::core::ffi::c_long
    {
        match (*ce).format as ::core::ffi::c_int {
            0 => {
                ttf_byte_encoding();
            }
            4 => {
                ttf_seg_map_delta();
            }
            6 => {
                ttf_trimmed_table_map();
            }
            _ => {}
        }
        ce = ce.offset(1);
    }
    ttf_set_chksm(tab);
}
unsafe fn ttf_write_name(names: &NameTable) {
    let tab = ttf_name_lookup(c"name".as_ptr(), true_0);
    let subset_tag = if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 {
        if (*fd_cur).subset_tag.is_null() {
            crate::utils::pdftex_fail_args(c"name: missing subset tag".as_ptr(), &[]);
        }
        let bytes = CStr::from_ptr((*fd_cur).subset_tag).to_bytes();
        Some(<[u8; 6]>::try_from(bytes).unwrap_or_else(|_| {
            crate::utils::pdftex_fail_args(c"name: subset tag must contain six bytes".as_ptr(), &[])
        }))
    } else {
        None
    };
    // Plan and validate every u16 field before mutating the PDF font stream.
    let output = names
        .output(subset_tag)
        .unwrap_or_else(|error| name_failure(error));
    ttf_reset_chksm(tab);
    output.emit(|byte| {
        ttf_putnum(TTF_CHAR_SIZE, ::core::ffi::c_long::from(byte));
    });
    ttf_set_chksm(tab);
}
unsafe extern "C" fn ttf_write_dirtab() {
    let mut tab: *mut dirtab_entry = ::core::ptr::null_mut::<dirtab_entry>();
    let mut i: TTF_ULONG = 0;
    let mut k_0: TTF_ULONG = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let save_offset: uint32_t = fb_offset() as uint32_t;
    fb_seek(TABDIR_OFF);
    if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int {
        i = 0 as TTF_ULONG;
        while i < DEFAULT_NTABS as TTF_ULONG {
            tab = ttf_name_lookup(newtabnames[i as usize], false_0);
            if !tab.is_null() {
                k_0 = 0 as TTF_ULONG;
                while k_0 < 4 as TTF_ULONG {
                    ttf_putnum(
                        TTF_CHAR_SIZE,
                        (*tab).tag[k_0 as usize] as ::core::ffi::c_long,
                    );
                    k_0 = k_0.wrapping_add(1);
                }
                ttf_putnum(TTF_ULONG_SIZE, (*tab).checksum as ::core::ffi::c_long);
                ttf_putnum(TTF_ULONG_SIZE, (*tab).offset as ::core::ffi::c_long);
                ttf_putnum(TTF_ULONG_SIZE, (*tab).length as ::core::ffi::c_long);
            }
            i = i.wrapping_add(1);
        }
    } else {
        tab = dir_tab;
        while (tab.offset_from(dir_tab) as ::core::ffi::c_long) < ntabs as ::core::ffi::c_long {
            k_0 = 0 as TTF_ULONG;
            while k_0 < 4 as TTF_ULONG {
                ttf_putnum(
                    TTF_CHAR_SIZE,
                    (*tab).tag[k_0 as usize] as ::core::ffi::c_long,
                );
                k_0 = k_0.wrapping_add(1);
            }
            ttf_putnum(TTF_ULONG_SIZE, (*tab).checksum as ::core::ffi::c_long);
            ttf_putnum(TTF_ULONG_SIZE, (*tab).offset as ::core::ffi::c_long);
            ttf_putnum(TTF_ULONG_SIZE, (*tab).length as ::core::ffi::c_long);
            tab = tab.offset(1);
        }
    }
    tmp_ulong = 0 as TTF_ULONG;
    checksum = 0 as TTF_ULONG;
    p = fb_array;
    i = 0 as TTF_ULONG;
    while i < save_offset {
        let fresh0 = p;
        p = p.offset(1);
        tmp_ulong = (tmp_ulong << 8 as ::core::ffi::c_int).wrapping_add(*fresh0 as TTF_ULONG);
        i = i.wrapping_add(1);
        if i.wrapping_rem(4 as TTF_ULONG) == 0 as TTF_ULONG {
            checksum = checksum.wrapping_add(tmp_ulong);
            tmp_ulong = 0 as TTF_ULONG;
        }
    }
    if i.wrapping_rem(4 as TTF_ULONG) != 0 as TTF_ULONG {
        crate::utils::pdftex_warn_args(
            b"font length is not a multiple of 4 (%d)\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(i)],
        );
        checksum <<= (8 as TTF_ULONG)
            .wrapping_mul((4 as TTF_ULONG).wrapping_sub(i.wrapping_rem(4 as TTF_ULONG)));
    }
    k_0 = (0xb1b0afba as TTF_ULONG).wrapping_sub(checksum);
    fb_seek(checkSumAdjustment_offset as integer);
    ttf_putnum(TTF_ULONG_SIZE, k_0 as ::core::ffi::c_long);
    fb_seek(save_offset as integer);
}
unsafe extern "C" fn ttf_write_glyf() {
    let mut id: *mut ::core::ffi::c_long = ::core::ptr::null_mut::<::core::ffi::c_long>();
    let mut k_0: ::core::ffi::c_long = 0;
    let mut idx: TTF_USHORT = 0;
    let mut flags: TTF_USHORT = 0;
    let mut tab: *mut dirtab_entry =
        ttf_name_lookup(b"glyf\0" as *const u8 as *const ::core::ffi::c_char, true_0);
    let glyf_offset: ::core::ffi::c_long = (*tab).offset as ::core::ffi::c_long;
    let new_glyf_offset: ::core::ffi::c_long = fb_offset() as ::core::ffi::c_long;
    ttf_reset_chksm(tab);
    id = glyph_index;
    while (id.offset_from(glyph_index) as ::core::ffi::c_long)
        < new_glyphs_count as ::core::ffi::c_long
    {
        (*glyph_tab.offset(*id as isize)).newoffset =
            (fb_offset() as ::core::ffi::c_long - new_glyf_offset) as TTF_LONG;
        if (*glyph_tab.offset(*id as isize)).offset
            != (*glyph_tab.offset((*id + 1 as ::core::ffi::c_long) as isize)).offset
        {
            ttf_seek_off(
                (glyf_offset + (*glyph_tab.offset(*id as isize)).offset as ::core::ffi::c_long)
                    as TTF_LONG,
            );
            k_0 = ttf_putnum(
                TTF_SHORT_SIZE,
                ttf_getnum(2 as ::core::ffi::c_int) as TTF_SHORT as ::core::ffi::c_long,
            ) as TTF_SHORT as ::core::ffi::c_long;
            ttf_ncopy(4 as ::core::ffi::c_int * TTF_FWORD_SIZE);
            if k_0 < 0 as ::core::ffi::c_long {
                loop {
                    flags = ttf_putnum(
                        TTF_USHORT_SIZE,
                        ttf_getnum(2 as ::core::ffi::c_int) as TTF_USHORT as ::core::ffi::c_long,
                    ) as TTF_USHORT;
                    idx = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
                    let index = checked_glyph_index(idx as ::core::ffi::c_long, glyphs_count)
                        .unwrap_or_else(|| {
                            crate::utils::pdftex_fail_args(
                                b"glyf: glyph index %li out of range [0..%i)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[
                                    crate::utils::PrintfArg::from(idx as ::core::ffi::c_long),
                                    crate::utils::PrintfArg::from(
                                        glyphs_count as ::core::ffi::c_int,
                                    ),
                                ],
                            )
                        });
                    let glyph = glyph_tab.add(index);
                    let output_index = match (*glyph).newindex {
                        Some(index) => index,
                        None => ttf_append_glyph(idx as ::core::ffi::c_long),
                    };
                    ttf_putnum(TTF_USHORT_SIZE, output_index as ::core::ffi::c_long);
                    if flags as ::core::ffi::c_int & ARG_1_AND_2_ARE_WORDS != 0 {
                        ttf_ncopy(2 as ::core::ffi::c_int * TTF_SHORT_SIZE);
                    } else {
                        ttf_ncopy(TTF_USHORT_SIZE);
                    }
                    if flags as ::core::ffi::c_int & WE_HAVE_A_SCALE != 0 {
                        ttf_ncopy(TTF_F2DOT14_SIZE);
                    } else if flags as ::core::ffi::c_int & WE_HAVE_AN_X_AND_Y_SCALE != 0 {
                        ttf_ncopy(2 as ::core::ffi::c_int * TTF_F2DOT14_SIZE);
                    } else if flags as ::core::ffi::c_int & WE_HAVE_A_TWO_BY_TWO != 0 {
                        ttf_ncopy(4 as ::core::ffi::c_int * TTF_F2DOT14_SIZE);
                    }
                    if !(flags as ::core::ffi::c_int & MORE_COMPONENTS != 0) {
                        break;
                    }
                }
                if flags as ::core::ffi::c_int & WE_HAVE_INSTRUCTIONS != 0 {
                    ttf_ncopy(ttf_putnum(
                        TTF_USHORT_SIZE,
                        ttf_getnum(2 as ::core::ffi::c_int) as TTF_USHORT as ::core::ffi::c_long,
                    ) as TTF_USHORT as ::core::ffi::c_int);
                }
            } else {
                ttf_ncopy(
                    (*glyph_tab.offset((*id + 1 as ::core::ffi::c_long) as isize)).offset
                        as ::core::ffi::c_int
                        - (*glyph_tab.offset(*id as isize)).offset as ::core::ffi::c_int
                        - TTF_USHORT_SIZE
                        - 4 as ::core::ffi::c_int * TTF_FWORD_SIZE,
                );
            }
        }
        id = id.offset(1);
    }
    last_glyf_offset = (fb_offset() as ::core::ffi::c_long - new_glyf_offset) as TTF_ULONG;
    ttf_set_chksm(tab);
}
unsafe extern "C" fn ttf_reindex_glyphs() {
    let mut current_block: u64;
    let mut e: *mut ttfenc_entry = ::core::ptr::null_mut::<ttfenc_entry>();
    let mut glyph: *mut glyph_entry = ::core::ptr::null_mut::<glyph_entry>();
    let mut index: ::core::ffi::c_int = 0;
    let mut t: *mut ::core::ffi::c_long = ::core::ptr::null_mut::<::core::ffi::c_long>();
    let mut cmap: *mut ttf_cmap_entry = ::core::ptr::null_mut::<ttf_cmap_entry>();
    let mut cmap_not_found: boolean = false_0;
    e = &raw mut ttfenc_tab as *mut ttfenc_entry;
    while (e.offset_from(&raw mut ttfenc_tab as *mut ttfenc_entry) as ::core::ffi::c_long)
        < 256 as ::core::ffi::c_long
    {
        (*e).newindex = 0 as ::core::ffi::c_long;
        if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBFONT != 0 as ::core::ffi::c_int {
            if (*e).code == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long {
                current_block = 16658872821858055392;
            } else {
                if !((*(*fd_cur).fm).pid as ::core::ffi::c_int != -(1 as ::core::ffi::c_int)
                    && (*(*fd_cur).fm).eid as ::core::ffi::c_int != -(1 as ::core::ffi::c_int))
                    as ::core::ffi::c_int as ::core::ffi::c_long
                    != 0
                {
                    __assert_rtn(
                        b"ttf_reindex_glyphs\0" as *const u8 as *const ::core::ffi::c_char,
                        b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                        1040 as ::core::ffi::c_int,
                        b"fd_cur->fm->pid != -1 && fd_cur->fm->eid != -1\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                } else {
                };
                if cmap.is_null() && cmap_not_found == 0 {
                    cmap = ttf_read_cmap(
                        (*(*fd_cur).fm).ff_name,
                        (*(*fd_cur).fm).pid as ::core::ffi::c_int,
                        (*(*fd_cur).fm).eid as ::core::ffi::c_int,
                        true_0,
                    );
                    if cmap.is_null() {
                        cmap_not_found = true_0 as boolean;
                    }
                }
                if cmap.is_null() {
                    current_block = 16658872821858055392;
                } else {
                    t = (*cmap).table;
                    if t.is_null() {
                        __assert_rtn(
                            b"ttf_reindex_glyphs\0" as *const u8 as *const ::core::ffi::c_char,
                            b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                            1051 as ::core::ffi::c_int,
                            b"t != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                    };
                    let selected = checked_cmap_code((*e).code)
                        .and_then(|code| checked_selected_glyph_index(*t.add(code), glyphs_count));
                    if let Some(index) = selected {
                        glyph = glyph_tab.add(index);
                        current_block = 9772907965827425793;
                    } else {
                        crate::utils::pdftex_warn_args(
                            b"subfont %s: wrong mapping: character %li --> 0x%4.4lX --> .notdef\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            &[
                                crate::utils::PrintfArg::from((*(*fd_cur).fm).tfm_name),
                                crate::utils::PrintfArg::from(
                                    e.offset_from(&raw mut ttfenc_tab as *mut ttfenc_entry)
                                        as ::core::ffi::c_long,
                                ),
                                crate::utils::PrintfArg::from((*e).code),
                            ],
                        );
                        current_block = 16658872821858055392;
                    }
                }
            }
        } else if (*e).name
            == &raw mut notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
        {
            current_block = 16658872821858055392;
        } else {
            glyph = glyph_tab.add(1);
            loop {
                if !((glyph.offset_from(glyph_tab) as ::core::ffi::c_long)
                    < glyphs_count as ::core::ffi::c_long)
                {
                    current_block = 4495394744059808450;
                    break;
                }
                if (*glyph).name
                    != &raw mut notdef as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
                    && strcmp((*glyph).name, (*e).name) == 0 as ::core::ffi::c_int
                {
                    current_block = 9772907965827425793;
                    break;
                }
                glyph = glyph.offset(1);
            }
            match current_block {
                9772907965827425793 => {}
                _ => {
                    let numeric_name =
                        match numeric_glyph_name(CStr::from_ptr((*e).name).to_bytes()) {
                            Ok(name) => name,
                            Err(_) => {
                                crate::utils::pdftex_warn_args(
                                    b"numeric glyph `%s' out of range\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[crate::utils::PrintfArg::from((*e).name)],
                                );
                                e = e.offset(1);
                                continue;
                            }
                        };
                    if let Some(NumericGlyphName::Unicode(value)) = numeric_name {
                        index = value as ::core::ffi::c_int;
                        if cmap.is_null() && cmap_not_found == 0 {
                            cmap = ttf_read_cmap(
                                (*(*fd_cur).fm).ff_name,
                                3 as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                                false_0,
                            );
                            if cmap.is_null() {
                                cmap = ttf_read_cmap(
                                    (*(*fd_cur).fm).ff_name,
                                    0 as ::core::ffi::c_int,
                                    3 as ::core::ffi::c_int,
                                    false_0,
                                );
                            }
                            if cmap.is_null() {
                                crate::utils::pdftex_warn_args(b"no unicode mapping found, all `uniXXXX' names will be ignored\0"
                                        as *const u8 as *const ::core::ffi::c_char, &[]);
                                cmap_not_found = true_0 as boolean;
                            }
                        }
                        if cmap.is_null() {
                            current_block = 16658872821858055392;
                        } else {
                            t = (*cmap).table;
                            if t.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                                __assert_rtn(
                                    b"ttf_reindex_glyphs\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                                    1090 as ::core::ffi::c_int,
                                    b"t != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                                );
                            } else {
                            };
                            if let Some(code) = checked_cmap_code(index as ::core::ffi::c_long) {
                                let mapped = *t.add(code);
                                if let Some(index) =
                                    checked_selected_glyph_index(mapped, glyphs_count)
                                {
                                    glyph = glyph_tab.add(index);
                                    current_block = 9772907965827425793;
                                } else {
                                    if mapped == -1 {
                                        crate::utils::pdftex_warn_args(
                                            b"`unicode %s%.4X' is not mapped to any glyph\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                            &[
                                                crate::utils::PrintfArg::from(
                                                    GLYPH_PREFIX_UNICODE.as_ptr(),
                                                ),
                                                crate::utils::PrintfArg::from(index),
                                            ],
                                        );
                                    } else {
                                        crate::utils::pdftex_warn_args(b"`%s' is mapped to index %li which is out of valid range [1..%i)\0"
                                                as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*e).name), crate::utils::PrintfArg::from(mapped), crate::utils::PrintfArg::from(glyphs_count as ::core::ffi::c_int)]);
                                    }
                                    current_block = 16658872821858055392;
                                }
                            } else {
                                crate::utils::pdftex_warn_args(
                                    b"`%s' out of valid unicode range [0..65536)\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[crate::utils::PrintfArg::from((*e).name)],
                                );
                                current_block = 16658872821858055392;
                            }
                        }
                    } else {
                        if let Some(NumericGlyphName::Index(value)) = numeric_name {
                            index = value;
                            if let Some(index) = checked_selected_glyph_index(
                                index as ::core::ffi::c_long,
                                glyphs_count,
                            ) {
                                glyph = glyph_tab.add(index);
                                current_block = 9772907965827425793;
                            } else {
                                crate::utils::pdftex_warn_args(
                                    b"`%s' out of valid range [1..%i)\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    &[
                                        crate::utils::PrintfArg::from((*e).name),
                                        crate::utils::PrintfArg::from(
                                            glyphs_count as ::core::ffi::c_int,
                                        ),
                                    ],
                                );
                                current_block = 16658872821858055392;
                            }
                        } else {
                            crate::utils::pdftex_warn_args(
                                b"glyph `%s' not found\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                &[crate::utils::PrintfArg::from((*e).name)],
                            );
                            current_block = 16658872821858055392;
                        }
                    }
                }
            }
        }
        match current_block {
            9772907965827425793 => {
                if !(glyph > glyph_tab
                    && (glyph.offset_from(glyph_tab) as ::core::ffi::c_long)
                        < glyphs_count as ::core::ffi::c_long)
                    as ::core::ffi::c_int as ::core::ffi::c_long
                    != 0
                {
                    __assert_rtn(
                        b"ttf_reindex_glyphs\0" as *const u8 as *const ::core::ffi::c_char,
                        b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
                        1125 as ::core::ffi::c_int,
                        b"glyph > glyph_tab && glyph - glyph_tab < glyphs_count\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                } else {
                };
                (*e).newindex = match (*glyph).newindex {
                    Some(index) => index,
                    None => ttf_append_glyph(glyph.offset_from(glyph_tab) as ::core::ffi::c_long),
                } as ::core::ffi::c_long;
            }
            _ => {}
        }
        e = e.offset(1);
    }
}
unsafe extern "C" fn ttf_write_head() {
    let mut tab: *mut dirtab_entry = ::core::ptr::null_mut::<dirtab_entry>();
    tab = ttf_seek_tab(
        b"head\0" as *const u8 as *const ::core::ffi::c_char,
        0 as TTF_LONG,
    );
    ttf_reset_chksm(tab);
    ttf_ncopy(2 as ::core::ffi::c_int * TTF_FIXED_SIZE);
    checkSumAdjustment_offset = fb_offset() as TTF_ULONG;
    ttf_putnum(TTF_ULONG_SIZE, 0 as ::core::ffi::c_long);
    ttf_getnum(4 as ::core::ffi::c_int);
    ttf_ncopy(
        TTF_ULONG_SIZE
            + 2 as ::core::ffi::c_int * TTF_USHORT_SIZE
            + 16 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * TTF_FWORD_SIZE
            + 2 as ::core::ffi::c_int * TTF_USHORT_SIZE
            + TTF_SHORT_SIZE,
    );
    if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int {
        ttf_putnum(TTF_SHORT_SIZE, loca_format as ::core::ffi::c_long);
        ttf_putnum(TTF_SHORT_SIZE, 0 as ::core::ffi::c_long);
    } else {
        ttf_ncopy(2 as ::core::ffi::c_int * TTF_SHORT_SIZE);
    }
    ttf_set_chksm(tab);
}
unsafe extern "C" fn ttf_write_hhea() {
    let mut tab: *mut dirtab_entry = ::core::ptr::null_mut::<dirtab_entry>();
    tab = ttf_seek_tab(
        b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        0 as TTF_LONG,
    );
    ttf_reset_chksm(tab);
    ttf_ncopy(
        TTF_FIXED_SIZE
            + 3 as ::core::ffi::c_int * TTF_FWORD_SIZE
            + TTF_UFWORD_SIZE
            + 3 as ::core::ffi::c_int * TTF_FWORD_SIZE
            + 8 as ::core::ffi::c_int * TTF_SHORT_SIZE,
    );
    ttf_putnum(TTF_USHORT_SIZE, new_glyphs_count as ::core::ffi::c_long);
    ttf_set_chksm(tab);
}
unsafe extern "C" fn ttf_write_htmx() {
    let mut id: *mut ::core::ffi::c_long = ::core::ptr::null_mut::<::core::ffi::c_long>();
    let mut tab: *mut dirtab_entry = ttf_seek_tab(
        b"hmtx\0" as *const u8 as *const ::core::ffi::c_char,
        0 as TTF_LONG,
    );
    ttf_reset_chksm(tab);
    id = glyph_index;
    while (id.offset_from(glyph_index) as ::core::ffi::c_long)
        < new_glyphs_count as ::core::ffi::c_long
    {
        ttf_putnum(
            TTF_UFWORD_SIZE,
            (*glyph_tab.offset(*id as isize)).advWidth as ::core::ffi::c_long,
        );
        ttf_putnum(
            TTF_UFWORD_SIZE,
            (*glyph_tab.offset(*id as isize)).lsb as ::core::ffi::c_long,
        );
        id = id.offset(1);
    }
    ttf_set_chksm(tab);
}
unsafe extern "C" fn ttf_write_loca() {
    let mut id: *mut ::core::ffi::c_long = ::core::ptr::null_mut::<::core::ffi::c_long>();
    let mut tab: *mut dirtab_entry = ttf_seek_tab(
        b"loca\0" as *const u8 as *const ::core::ffi::c_char,
        0 as TTF_LONG,
    );
    ttf_reset_chksm(tab);
    loca_format = 0 as TTF_SHORT;
    if last_glyf_offset >= 0x20000 as TTF_ULONG || last_glyf_offset & 1 as TTF_ULONG != 0 {
        loca_format = 1 as TTF_SHORT;
    } else {
        id = glyph_index;
        while (id.offset_from(glyph_index) as ::core::ffi::c_long)
            < new_glyphs_count as ::core::ffi::c_long
        {
            if (*glyph_tab.offset(*id as isize)).newoffset & 1 as TTF_LONG != 0 {
                loca_format = 1 as TTF_SHORT;
                break;
            } else {
                id = id.offset(1);
            }
        }
    }
    if loca_format as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        id = glyph_index;
        while (id.offset_from(glyph_index) as ::core::ffi::c_long)
            < new_glyphs_count as ::core::ffi::c_long
        {
            ttf_putnum(
                TTF_ULONG_SIZE,
                (*glyph_tab.offset(*id as isize)).newoffset as ::core::ffi::c_long,
            );
            id = id.offset(1);
        }
        ttf_putnum(TTF_ULONG_SIZE, last_glyf_offset as ::core::ffi::c_long);
    } else {
        id = glyph_index;
        while (id.offset_from(glyph_index) as ::core::ffi::c_long)
            < new_glyphs_count as ::core::ffi::c_long
        {
            ttf_putnum(
                TTF_USHORT_SIZE,
                ((*glyph_tab.offset(*id as isize)).newoffset / 2 as TTF_LONG)
                    as ::core::ffi::c_long,
            );
            id = id.offset(1);
        }
        ttf_putnum(
            TTF_USHORT_SIZE,
            last_glyf_offset.wrapping_div(2 as TTF_ULONG) as ::core::ffi::c_long,
        );
    }
    ttf_set_chksm(tab);
}
unsafe extern "C" fn ttf_write_mapx() {
    let mut tab: *mut dirtab_entry = ttf_seek_tab(
        b"maxp\0" as *const u8 as *const ::core::ffi::c_char,
        TTF_FIXED_SIZE as TTF_LONG + TTF_USHORT_SIZE as TTF_LONG,
    );
    ttf_reset_chksm(tab);
    ttf_putnum(TTF_FIXED_SIZE, 0x10000 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, new_glyphs_count as ::core::ffi::c_long);
    ttf_ncopy(13 as ::core::ffi::c_int * TTF_USHORT_SIZE);
    ttf_set_chksm(tab);
}
unsafe extern "C" fn ttf_write_OS2() {
    let mut tab: *mut dirtab_entry = ttf_seek_tab(
        b"OS/2\0" as *const u8 as *const ::core::ffi::c_char,
        0 as TTF_LONG,
    );
    let mut version: TTF_USHORT = 0;
    ttf_reset_chksm(tab);
    version = ttf_getnum(TTF_USHORT_SIZE) as TTF_USHORT;
    if version as ::core::ffi::c_int > 5 as ::core::ffi::c_int {
        crate::utils::pdftex_warn_args(
            b"unknown version of OS/2 table (%.4X)\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(version as ::core::ffi::c_int)],
        );
    }
    ttf_putnum(TTF_USHORT_SIZE, 0x1 as ::core::ffi::c_long);
    ttf_ncopy(
        2 as ::core::ffi::c_int * TTF_USHORT_SIZE
            + 13 as ::core::ffi::c_int * TTF_SHORT_SIZE
            + 10 as ::core::ffi::c_int * TTF_BYTE_SIZE,
    );
    ttf_getnum(4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int);
    ttf_putnum(TTF_ULONG_SIZE, 0x3 as ::core::ffi::c_long);
    ttf_putnum(TTF_ULONG_SIZE, 0x10000000 as ::core::ffi::c_long);
    ttf_putnum(TTF_ULONG_SIZE, 0 as ::core::ffi::c_long);
    ttf_putnum(TTF_ULONG_SIZE, 0 as ::core::ffi::c_long);
    ttf_ncopy(4 as ::core::ffi::c_int * TTF_CHAR_SIZE + TTF_USHORT_SIZE);
    ttf_getnum(2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int);
    ttf_putnum(TTF_USHORT_SIZE, 0 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, 0xf0ff as ::core::ffi::c_long);
    ttf_ncopy(5 as ::core::ffi::c_int * TTF_USHORT_SIZE);
    ttf_putnum(
        TTF_ULONG_SIZE,
        0x80000000 as ::core::ffi::c_uint as ::core::ffi::c_long,
    );
    ttf_putnum(TTF_ULONG_SIZE, 0 as ::core::ffi::c_long);
    ttf_set_chksm(tab);
}
unsafe extern "C" fn unsafe_name(mut s: *const ::core::ffi::c_char) -> boolean {
    let mut p: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    p = &raw mut ambiguous_names as *mut *const ::core::ffi::c_char;
    while !(*p).is_null() {
        if strcmp(s, *p) == 0 as ::core::ffi::c_int {
            return true_0;
        }
        p = p.offset(1);
    }
    return false_0;
}
unsafe extern "C" fn ttf_write_post() {
    let mut tab: *mut dirtab_entry = ttf_seek_tab(
        b"post\0" as *const u8 as *const ::core::ffi::c_char,
        TTF_FIXED_SIZE as TTF_LONG,
    );
    ttf_reset_chksm(tab);
    if (*fd_cur).write_ttf_glyph_names == 0 || post_format == 0x30000 as TTF_FIXED {
        ttf_putnum(TTF_FIXED_SIZE, 0x30000 as ::core::ffi::c_long);
        ttf_ncopy(
            TTF_FIXED_SIZE
                + 2 as ::core::ffi::c_int * TTF_FWORD_SIZE
                + 5 as ::core::ffi::c_int * TTF_ULONG_SIZE,
        );
    } else {
        ttf_putnum(TTF_FIXED_SIZE, 0x20000 as ::core::ffi::c_long);
        ttf_ncopy(
            TTF_FIXED_SIZE
                + 2 as ::core::ffi::c_int * TTF_FWORD_SIZE
                + 5 as ::core::ffi::c_int * TTF_ULONG_SIZE,
        );
        ttf_putnum(TTF_USHORT_SIZE, new_glyphs_count as ::core::ffi::c_long);
        let mut glyphs = Vec::new();
        glyphs
            .try_reserve_exact(usize::from(new_glyphs_count))
            .unwrap_or_else(|_| post_failure(PostError::Allocation));
        for output_index in 0..usize::from(new_glyphs_count) {
            let source_index = checked_glyph_index(*glyph_index.add(output_index), glyphs_count)
                .unwrap_or_else(|| post_failure(PostError::GlyphCount));
            let glyph = &*glyph_tab.add(source_index);
            glyphs.push(OutputGlyphName {
                index: glyph.name_index,
                name: CStr::from_ptr(glyph.name),
                ambiguous: unsafe_name(glyph.name) != 0,
            });
        }
        let output = ttf_post::output_names(&glyphs).unwrap_or_else(|error| post_failure(error));
        for index in output.indices {
            ttf_putnum(TTF_USHORT_SIZE, ::core::ffi::c_long::from(index));
        }
        for name in output.custom {
            let bytes = name.to_bytes();
            ttf_putnum(TTF_BYTE_SIZE, bytes.len() as ::core::ffi::c_long);
            for &byte in bytes {
                ttf_putnum(TTF_CHAR_SIZE, ::core::ffi::c_long::from(byte));
            }
        }
    }
    ttf_set_chksm(tab);
}
unsafe extern "C" fn ttf_init_font(mut n: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k_0: ::core::ffi::c_int = 0;
    i = 1 as ::core::ffi::c_int;
    k_0 = 0 as ::core::ffi::c_int;
    while i <= n {
        i <<= 1 as ::core::ffi::c_int;
        k_0 += 1;
    }
    ttf_putnum(TTF_FIXED_SIZE, 0x10000 as ::core::ffi::c_long);
    ttf_putnum(TTF_USHORT_SIZE, n as ::core::ffi::c_long);
    ttf_putnum(
        TTF_USHORT_SIZE,
        (i << 3 as ::core::ffi::c_int) as ::core::ffi::c_long,
    );
    ttf_putnum(
        TTF_USHORT_SIZE,
        (k_0 - 1 as ::core::ffi::c_int) as ::core::ffi::c_long,
    );
    ttf_putnum(
        TTF_USHORT_SIZE,
        ((n << 4 as ::core::ffi::c_int) - (i << 3 as ::core::ffi::c_int)) as ::core::ffi::c_long,
    );
    fb_seek(TABDIR_OFF + n as integer * 4 as integer * TTF_ULONG_SIZE);
}
unsafe fn ttf_subset_font(names: &NameTable) {
    ttf_init_font(new_ntabs as ::core::ffi::c_int);
    if !ttf_name_lookup(
        b"PCLT\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        ttf_copytab(b"PCLT\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !ttf_name_lookup(
        b"fpgm\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        ttf_copytab(b"fpgm\0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !ttf_name_lookup(
        b"cvt \0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        ttf_copytab(b"cvt \0" as *const u8 as *const ::core::ffi::c_char);
    }
    if !ttf_name_lookup(
        b"prep\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        ttf_copytab(b"prep\0" as *const u8 as *const ::core::ffi::c_char);
    }
    ttf_reindex_glyphs();
    ttf_write_glyf();
    ttf_write_loca();
    ttf_write_OS2();
    ttf_write_head();
    ttf_write_hhea();
    ttf_write_htmx();
    ttf_write_mapx();
    ttf_write_name(names);
    ttf_write_post();
    ttf_write_cmap();
    ttf_write_dirtab();
}
unsafe extern "C" fn ttf_copy_font() {
    let mut tab: *mut dirtab_entry = ::core::ptr::null_mut::<dirtab_entry>();
    ttf_init_font(ntabs as ::core::ffi::c_int);
    tab = dir_tab;
    while (tab.offset_from(dir_tab) as ::core::ffi::c_long) < ntabs as ::core::ffi::c_long {
        if strncmp(
            &raw mut (*tab).tag as *mut ::core::ffi::c_char,
            b"head\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            ttf_write_head();
        } else {
            ttf_copytab(&raw mut (*tab).tag as *mut ::core::ffi::c_char);
        }
        tab = tab.offset(1);
    }
    ttf_write_dirtab();
}
#[no_mangle]
pub unsafe extern "C" fn writettf(mut fd: *mut fd_entry) {
    fd_cur = fd;
    if (*fd_cur).fm.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"writettf\0" as *const u8 as *const ::core::ffi::c_char,
            b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
            1337 as ::core::ffi::c_int,
            b"fd_cur->fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"writettf\0" as *const u8 as *const ::core::ffi::c_char,
            b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
            1338 as ::core::ffi::c_int,
            b"is_truetype(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"writettf\0" as *const u8 as *const ::core::ffi::c_char,
            b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
            1339 as ::core::ffi::c_int,
            b"is_included(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    cur_file_name = (*(*fd_cur).fm).ff_name;
    zpackfilename(maketexstring(cur_file_name), getnullstr(), getnullstr());
    if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int
        && (*fd_cur).fe.is_null()
        && !((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBFONT != 0 as ::core::ffi::c_int)
    {
        crate::utils::pdftex_fail_args(
            b"Subset TrueType must be a reencoded or a subfont\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    if open_input(
        &raw mut ttf_file,
        kpse_truetype_format as ::core::ffi::c_int,
        FOPEN_RBIN_MODE.as_ptr(),
    ) == 0
    {
        crate::utils::pdftex_fail_args(
            b"cannot open TrueType font file for reading\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    cur_file_name =
        (nameoffile as *mut ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize);
    if strcasecmp(
        strchr(cur_file_name, 0 as ::core::ffi::c_int).offset(-(4 as ::core::ffi::c_int as isize)),
        b".ttc\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        if ttf_getnum(TTF_ULONG_SIZE) as TTF_ULONG != 0x74746366 as TTF_ULONG {
            xfseek(
                ttf_file,
                0 as ::core::ffi::c_long,
                SEEK_SET,
                cur_file_name as const_string,
            );
        } else {
            ttf_getnum(4 as ::core::ffi::c_int);
            ttf_getnum(4 as ::core::ffi::c_int);
            xfseek(
                ttf_file,
                ttf_getnum(TTF_ULONG_SIZE) as TTF_ULONG as ::core::ffi::c_long,
                SEEK_SET,
                cur_file_name as const_string,
            );
        }
    }
    if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int {
        crate::utils::tex_printf_args(
            b"<%s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(cur_file_name)],
        );
    } else {
        crate::utils::tex_printf_args(
            b"<<%s\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(cur_file_name)],
        );
    }
    (*fd_cur).ff_found = true_0 as boolean;
    new_glyphs_count = 2 as TTF_USHORT;
    new_ntabs = DEFAULT_NTABS as TTF_USHORT;
    dir_tab = ::core::ptr::null_mut::<dirtab_entry>();
    glyph_tab = ::core::ptr::null_mut::<glyph_entry>();
    glyph_index = ::core::ptr::null_mut::<::core::ffi::c_long>();
    // Custom glyph pointers borrow this stack-owned name table. Keep it alive
    // through encoding, subset emission and every other font consumer.
    let (glyph_names, names) = ttf_read_font();
    pdfsaveoffset = pdfgone + pdfptr as longinteger;
    pdfflush();
    if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int {
        ttf_copy_encoding();
        ttf_subset_font(&names);
    } else {
        ttf_copy_font();
    }
    ttf_length = fb_offset();
    if !dir_tab.is_null() {
        free(dir_tab as *mut ::core::ffi::c_void);
    }
    dir_tab = ::core::ptr::null_mut::<dirtab_entry>();
    if !glyph_tab.is_null() {
        free(glyph_tab as *mut ::core::ffi::c_void);
    }
    glyph_tab = ::core::ptr::null_mut::<glyph_entry>();
    if !glyph_index.is_null() {
        free(glyph_index as *mut ::core::ffi::c_void);
    }
    glyph_index = ::core::ptr::null_mut::<::core::ffi::c_long>();
    // Drop Rust-owned strings only after glyph_tab and all font consumers.
    drop(glyph_names);
    xfclose(ttf_file, cur_file_name as const_string);
    if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int {
        crate::utils::tex_printf_args(b">\0" as *const u8 as *const ::core::ffi::c_char, &[]);
    } else {
        crate::utils::tex_printf_args(b">>\0" as *const u8 as *const ::core::ffi::c_char, &[]);
    }
    cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn writeotf(mut fd: *mut fd_entry) {
    let mut tab: *mut dirtab_entry = ::core::ptr::null_mut::<dirtab_entry>();
    let mut i: ::core::ffi::c_long = 0;
    fd_cur = fd;
    if (*fd_cur).fm.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"writeotf\0" as *const u8 as *const ::core::ffi::c_char,
            b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
            1408 as ::core::ffi::c_int,
            b"fd_cur->fm != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & 0x40 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"writeotf\0" as *const u8 as *const ::core::ffi::c_char,
            b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
            1409 as ::core::ffi::c_int,
            b"is_opentype(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*(*fd_cur).fm).type_0 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"writeotf\0" as *const u8 as *const ::core::ffi::c_char,
            b"writettf.c\0" as *const u8 as *const ::core::ffi::c_char,
            1410 as ::core::ffi::c_int,
            b"is_included(fd_cur->fm)\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    cur_file_name = (*(*fd_cur).fm).ff_name;
    zpackfilename(maketexstring(cur_file_name), getnullstr(), getnullstr());
    if (*(*fd_cur).fm).type_0 as ::core::ffi::c_int & F_SUBSETTED != 0 as ::core::ffi::c_int {
        crate::utils::pdftex_fail_args(
            b"OTF fonts must be included entirely\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    if open_input(
        &raw mut ttf_file,
        kpse_opentype_format as ::core::ffi::c_int,
        FOPEN_RBIN_MODE.as_ptr(),
    ) == 0
    {
        crate::utils::pdftex_fail_args(
            b"cannot open OpenType font file for reading\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    cur_file_name =
        (nameoffile as *mut ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize);
    crate::utils::tex_printf_args(
        b"<<%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(cur_file_name)],
    );
    (*fd_cur).ff_found = true_0 as boolean;
    dir_tab = ::core::ptr::null_mut::<dirtab_entry>();
    glyph_tab = ::core::ptr::null_mut::<glyph_entry>();
    ttf_read_tabdir();
    if !ttf_name_lookup(
        b"head\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        ttf_read_head();
    }
    if !ttf_name_lookup(
        b"hhea\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        ttf_read_hhea();
    }
    if !ttf_name_lookup(
        b"PCLT\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        ttf_read_pclt();
    }
    if !ttf_name_lookup(
        b"post\0" as *const u8 as *const ::core::ffi::c_char,
        false_0,
    )
    .is_null()
    {
        let _ = ttf_read_post();
    }
    tab = ttf_seek_tab(
        b"CFF \0" as *const u8 as *const ::core::ffi::c_char,
        0 as TTF_LONG,
    );
    i = (*tab).length as ::core::ffi::c_long;
    while i > 0 as ::core::ffi::c_long {
        ttf_putnum(
            TTF_CHAR_SIZE,
            ttf_getnum(1 as ::core::ffi::c_int) as TTF_CHAR as ::core::ffi::c_long,
        );
        i -= 1;
    }
    if !dir_tab.is_null() {
        free(dir_tab as *mut ::core::ffi::c_void);
    }
    dir_tab = ::core::ptr::null_mut::<dirtab_entry>();
    xfclose(ttf_file, cur_file_name as const_string);
    crate::utils::tex_printf_args(b">>\0" as *const u8 as *const ::core::ffi::c_char, &[]);
    cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

#[cfg(test)]
mod glyph_index_tests {
    use super::{
        checked_cmap_code, checked_glyph_index, checked_output_glyph_index,
        checked_post_name_count, checked_selected_glyph_index, initial_glyph_newindex,
        TTF_CMAP_ENTRIES,
    };
    use core::ffi::c_long;
    use proptest::prelude::*;

    #[test]
    fn cmap_code_boundaries_reject_signed_and_oversized_indices() {
        for code in [c_long::MIN, -65536, -1, 65536, 65537, c_long::MAX] {
            assert_eq!(checked_cmap_code(code), None, "code {code}");
        }
        assert_eq!(checked_cmap_code(0), Some(0));
        assert_eq!(checked_cmap_code(1), Some(1));
        assert_eq!(checked_cmap_code(65535), Some(TTF_CMAP_ENTRIES - 1));
    }

    #[test]
    fn glyph_boundaries_exclude_the_loca_sentinel_and_selected_zero() {
        for count in [0_u16, 1, 2, 258, u16::MAX] {
            for index in [c_long::MIN, -1, c_long::from(count), c_long::MAX] {
                assert_eq!(
                    checked_glyph_index(index, count),
                    None,
                    "index {index}, count {count}"
                );
                assert_eq!(
                    checked_selected_glyph_index(index, count),
                    None,
                    "index {index}, count {count}"
                );
            }
            assert_eq!(checked_glyph_index(0, count), (count > 0).then_some(0));
            assert_eq!(checked_selected_glyph_index(0, count), None);
            if count > 1 {
                let last = usize::from(count) - 1;
                assert_eq!(checked_glyph_index(last as c_long, count), Some(last));
                assert_eq!(
                    checked_selected_glyph_index(last as c_long, count),
                    Some(last)
                );
            }
        }
    }

    #[test]
    fn reserved_glyphs_own_the_prefilled_output_slots() {
        assert_eq!(checked_glyph_index(1, 0), None);
        assert_eq!(checked_glyph_index(1, 1), None);
        assert_eq!(checked_glyph_index(1, 2), Some(1));
        assert_eq!(initial_glyph_newindex(0), Some(0));
        assert_eq!(initial_glyph_newindex(1), Some(1));
        for index in [2, 258, 32768, 65534, usize::MAX] {
            assert_eq!(initial_glyph_newindex(index), None);
        }
    }

    #[test]
    fn output_slots_support_unsigned_indices_without_using_extra_allocation() {
        for count in [0_u16, 1, 2, 258, 32768, u16::MAX] {
            for index in [c_long::MIN, -1, c_long::from(count), c_long::MAX] {
                assert_eq!(checked_output_glyph_index(index, count), None);
            }
        }
        assert_eq!(checked_output_glyph_index(32768, u16::MAX), Some(32768));
        assert_eq!(checked_output_glyph_index(65534, u16::MAX), Some(65534));
        assert_eq!(65534_u16.checked_add(1), Some(u16::MAX));
        assert_eq!(u16::MAX.checked_add(1), None);
    }

    #[test]
    fn post_name_counts_cannot_include_the_loca_only_entry() {
        for count in [0_u16, 1, 2, 4, 258, u16::MAX] {
            assert_eq!(checked_post_name_count(-1, count), None);
            assert_eq!(checked_post_name_count(0, count), Some(0));
            assert_eq!(
                checked_post_name_count(c_long::from(count), count),
                Some(usize::from(count))
            );
            assert_eq!(
                checked_post_name_count(c_long::from(count) + 1, count),
                None
            );
        }
        assert_eq!(checked_post_name_count(258, 4), None);
        assert_eq!(checked_post_name_count(258, 258), Some(258));
    }

    proptest! {
        #[test]
        fn cmap_codes_match_the_allocated_domain(
            code in prop_oneof![any::<c_long>(), -2 as c_long..=65537 as c_long],
        ) {
            let expected = if (0..65536).contains(&code) { Some(code as usize) } else { None };
            prop_assert_eq!(checked_cmap_code(code), expected);
        }

        #[test]
        fn composite_ids_never_select_the_loca_only_entry(
            index in prop_oneof![any::<c_long>(), -2 as c_long..=65537 as c_long],
            count in any::<u16>(),
        ) {
            let expected = if index >= 0 && index < c_long::from(count) {
                Some(index as usize)
            } else { None };
            prop_assert_eq!(checked_glyph_index(index, count), expected);
        }

        #[test]
        fn numeric_names_and_cmap_values_require_a_nonzero_glyph(
            index in prop_oneof![any::<c_long>(), -2 as c_long..=65537 as c_long],
            count in any::<u16>(),
        ) {
            let expected = if index > 0 && index < c_long::from(count) {
                Some(index as usize)
            } else { None };
            prop_assert_eq!(checked_selected_glyph_index(index, count), expected);
        }

        #[test]
        fn output_indices_fit_the_allocation_and_next_unsigned_count(
            index in prop_oneof![any::<c_long>(), -2 as c_long..=65537 as c_long],
            count in any::<u16>(),
        ) {
            let expected = if index >= 0 && index < c_long::from(count) {
                Some(index as usize)
            } else { None };
            let actual = checked_output_glyph_index(index, count);
            prop_assert_eq!(actual, expected);
            if let Some(index) = actual {
                prop_assert!(index < usize::from(count) + 1);
                prop_assert!((index as u16).checked_add(1).is_some());
            }
        }

        #[test]
        fn post_name_prefixes_fit_the_initialized_glyph_domain(
            names in prop_oneof![any::<c_long>(), -2 as c_long..=65537 as c_long],
            count in any::<u16>(),
        ) {
            let expected = if names >= 0 && names <= c_long::from(count) {
                Some(names as usize)
            } else { None };
            prop_assert_eq!(checked_post_name_count(names, count), expected);
        }

        #[test]
        fn metric_counts_require_a_nonempty_initialized_glyph_prefix(
            metrics in any::<u16>(),
            count in any::<u16>(),
        ) {
            let valid = checked_glyph_index(c_long::from(metrics) - 1, count).is_some();
            prop_assert_eq!(valid, metrics > 0 && metrics <= count);
        }

        #[test]
        fn reserved_ownership_keeps_repeated_selections_within_capacity(
            count in 2_u16..512,
            selections in prop::collection::vec(any::<u16>(), 0..512),
        ) {
            let mut ownership = (0..usize::from(count)).map(initial_glyph_newindex).collect::<Vec<_>>();
            let mut output = vec![0_usize, 1];
            for selection in selections {
                let source = usize::from(selection % count);
                let index = match ownership[source] {
                    Some(index) => usize::from(index),
                    None => {
                        let index = checked_output_glyph_index(output.len() as c_long, count)
                            .expect("each source glyph is appended at most once");
                        ownership[source] = Some(index as u16);
                        output.push(source);
                        index
                    }
                };
                prop_assert_eq!(output[index], source);
            }
            prop_assert!(output.len() <= usize::from(count));
            let unique = output.iter().copied().collect::<std::collections::HashSet<_>>();
            prop_assert_eq!(unique.len(), output.len());
        }
    }
}

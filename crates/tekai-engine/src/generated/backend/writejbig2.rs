pub use crate::generated::pdftex0::{
    pdfendstream, pdfflush, zpdfbegindict, zpdfcreateobj, zpdfosgetosbuf,
};
pub use crate::utils::pdf_puts;

pub use crate::generated::backend::avl::{
    avl_create, avl_find, avl_node, avl_probe, avl_t_first, avl_t_init, avl_t_next, avl_table,
    avl_traverser,
};

use crate::byte_string::strcmp;

use crate::allocation::free;
use crate::openclose::{xmalloc, xstrdup};
use crate::support::{xfclose, xfopen, xfseeko, xftello};

pub use crate::pngshim::png_struct_def;

pub use crate::pngshim::png_info_def;

use crate::termination::__assert_rtn;

#[inline(always)]
unsafe fn fast_fgetc(stream: *mut FILE) -> ::core::ffi::c_int {
    unsafe { crate::stdio::fgetc(stream) }
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
pub use crate::stdio::Stream as FILE;
pub type off_t = __darwin_off_t;
pub type boolean = ::core::ffi::c_int;
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub type eightbits = ::core::ffi::c_uchar;
pub type FILEINFO = _FILEINFO;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _FILEINFO {
    pub file: *mut FILE,
    pub filename: *mut ::core::ffi::c_char,
    pub filesize: off_t,
    pub pages: LIST,
    pub page0: LIST,
    pub filehdrflags: ::core::ffi::c_uint,
    pub sequentialaccess: boolean,
    pub numofpages: ::core::ffi::c_ulong,
    pub streamstart: off_t,
    pub pdfpage0objnum: ::core::ffi::c_ulong,
    pub phase: PHASE,
}
pub type PHASE = ::core::ffi::c_uint;
pub const WRITEPDF: PHASE = 2;
pub const HAVEINFO: PHASE = 1;
pub const INITIAL: PHASE = 0;
pub type LIST = _LIST;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LIST {
    pub first: *mut LITEM,
    pub last: *mut LITEM,
    pub tree: *mut avl_table,
}
pub type LITEM = _LITEM;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _LITEM {
    pub prev: *mut _LITEM,
    pub next: *mut _LITEM,
    pub d: *mut ::core::ffi::c_void,
}
pub type SEGINFO = _SEGINFO;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SEGINFO {
    pub segnum: ::core::ffi::c_ulong,
    pub isrefered: boolean,
    pub refers: boolean,
    pub seghdrflags: ::core::ffi::c_uint,
    pub pageassocsizeflag: boolean,
    pub reftosegcount: ::core::ffi::c_uint,
    pub countofrefered: ::core::ffi::c_uint,
    pub fieldlen: ::core::ffi::c_uint,
    pub segnumwidth: ::core::ffi::c_uint,
    pub segpage: ::core::ffi::c_long,
    pub segdatalen: ::core::ffi::c_ulong,
    pub hdrstart: off_t,
    pub hdrend: off_t,
    pub datastart: off_t,
    pub dataend: off_t,
    pub endofstripeflag: boolean,
    pub endofpageflag: boolean,
    pub pageinfoflag: boolean,
    pub endoffileflag: boolean,
}
pub type PAGEINFO = _PAGEINFO;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PAGEINFO {
    pub segments: LIST,
    pub pagenum: ::core::ffi::c_ulong,
    pub width: ::core::ffi::c_uint,
    pub height: ::core::ffi::c_uint,
    pub xres: ::core::ffi::c_uint,
    pub yres: ::core::ffi::c_uint,
    pub pagesegmentflags: ::core::ffi::c_uint,
    pub stripinginfo: ::core::ffi::c_uint,
    pub stripedheight: ::core::ffi::c_uint,
}
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
#[cfg(test)]
pub use crate::backend_types::image_entry;
#[cfg(test)]
pub use crate::backend_types::pdf_image_struct;
#[cfg(test)]
pub use crate::backend_types::png_image_struct;
#[cfg(test)]
pub use crate::backend_types::ImageData as C2RustUnnamed;
#[cfg(test)]
pub use crate::backend_types::JBIG2_IMAGE_INFO;
#[cfg(test)]
pub use crate::backend_types::JPG_IMAGE_INFO;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const M_SymbolDictionary: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
pub const M_IntermediateTextRegion: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const M_ImmediateTextRegion: ::core::ffi::c_uint = 6 as ::core::ffi::c_uint;
pub const M_ImmediateLosslessTextRegion: ::core::ffi::c_uint = 7 as ::core::ffi::c_uint;
pub const M_PatternDictionary: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const M_IntermediateHalftoneRegion: ::core::ffi::c_uint = 20 as ::core::ffi::c_uint;
pub const M_ImmediateHalftoneRegion: ::core::ffi::c_uint = 22 as ::core::ffi::c_uint;
pub const M_ImmediateLosslessHalftoneRegion: ::core::ffi::c_uint = 23 as ::core::ffi::c_uint;
pub const M_IntermediateGenericRegion: ::core::ffi::c_uint = 36 as ::core::ffi::c_uint;
pub const M_ImmediateGenericRegion: ::core::ffi::c_uint = 38 as ::core::ffi::c_uint;
pub const M_ImmediateLosslessGenericRegion: ::core::ffi::c_uint = 39 as ::core::ffi::c_uint;
pub const M_IntermediateGenericRefinementRegion: ::core::ffi::c_uint = 40 as ::core::ffi::c_uint;
pub const M_ImmediateGenericRefinementRegion: ::core::ffi::c_uint = 42 as ::core::ffi::c_uint;
pub const M_ImmediateLosslessGenericRefinementRegion: ::core::ffi::c_uint =
    43 as ::core::ffi::c_uint;
pub const M_PageInformation: ::core::ffi::c_uint = 48 as ::core::ffi::c_uint;
pub const M_EndOfPage: ::core::ffi::c_uint = 49 as ::core::ffi::c_uint;
pub const M_EndOfStripe: ::core::ffi::c_uint = 50 as ::core::ffi::c_uint;
pub const M_EndOfFile: ::core::ffi::c_uint = 51 as ::core::ffi::c_uint;
pub const M_Profiles: ::core::ffi::c_uint = 52 as ::core::ffi::c_uint;
pub const M_Tables: ::core::ffi::c_uint = 53 as ::core::ffi::c_uint;
pub const M_Extension: ::core::ffi::c_uint = 62 as ::core::ffi::c_uint;

unsafe fn comp_file_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        (*(pa as *const FILEINFO)).filename,
        (*(pb as *const FILEINFO)).filename,
    );
}
unsafe fn comp_page_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(pa as *const PAGEINFO))
        .pagenum
        .wrapping_sub((*(pb as *const PAGEINFO)).pagenum) as ::core::ffi::c_int;
}
unsafe fn comp_segment_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(pa as *const SEGINFO))
        .segnum
        .wrapping_sub((*(pb as *const SEGINFO)).segnum) as ::core::ffi::c_int;
}
unsafe fn ygetc(mut stream: *mut FILE) -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = fast_fgetc(stream);
    if c < 0 as ::core::ffi::c_int {
        if c == EOF {
            crate::utils::pdftex_fail_args(
                b"fgetc() failed; premature end of JBIG2 image file\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        } else {
            crate::utils::pdftex_fail_args(
                b"fgetc() failed (can't happen)\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        }
    }
    return c;
}
unsafe fn new_fileinfo() -> *mut FILEINFO {
    let mut fip: *mut FILEINFO = ::core::ptr::null_mut::<FILEINFO>();
    fip = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<FILEINFO>() as size_t))
        as *mut FILEINFO;
    (*fip).file = ::core::ptr::null_mut::<FILE>();
    (*fip).filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fip).filesize = 0 as off_t;
    initlinkedlist(&raw mut (*fip).pages);
    initlinkedlist(&raw mut (*fip).page0);
    (*fip).filehdrflags = 0 as ::core::ffi::c_uint;
    (*fip).sequentialaccess = false_0 as boolean;
    (*fip).numofpages = 0 as ::core::ffi::c_ulong;
    (*fip).streamstart = 0 as off_t;
    (*fip).pdfpage0objnum = 0 as ::core::ffi::c_ulong;
    (*fip).phase = INITIAL;
    return fip;
}
unsafe fn new_pageinfo() -> *mut PAGEINFO {
    let mut pip: *mut PAGEINFO = ::core::ptr::null_mut::<PAGEINFO>();
    pip = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<PAGEINFO>() as size_t))
        as *mut PAGEINFO;
    initlinkedlist(&raw mut (*pip).segments);
    (*pip).pagenum = 0 as ::core::ffi::c_ulong;
    (*pip).width = 0 as ::core::ffi::c_uint;
    (*pip).height = 0 as ::core::ffi::c_uint;
    (*pip).xres = 0 as ::core::ffi::c_uint;
    (*pip).yres = 0 as ::core::ffi::c_uint;
    (*pip).pagesegmentflags = 0 as ::core::ffi::c_uint;
    (*pip).stripinginfo = 0 as ::core::ffi::c_uint;
    (*pip).stripedheight = 0 as ::core::ffi::c_uint;
    return pip;
}
unsafe fn init_seginfo(mut sip: *mut SEGINFO) {
    (*sip).segnum = 0 as ::core::ffi::c_ulong;
    (*sip).isrefered = false_0 as boolean;
    (*sip).refers = false_0 as boolean;
    (*sip).seghdrflags = 0 as ::core::ffi::c_uint;
    (*sip).pageassocsizeflag = false_0 as boolean;
    (*sip).reftosegcount = 0 as ::core::ffi::c_uint;
    (*sip).countofrefered = 0 as ::core::ffi::c_uint;
    (*sip).fieldlen = 0 as ::core::ffi::c_uint;
    (*sip).segnumwidth = 0 as ::core::ffi::c_uint;
    (*sip).segpage = 0 as ::core::ffi::c_long;
    (*sip).segdatalen = 0 as ::core::ffi::c_ulong;
    (*sip).hdrstart = 0 as off_t;
    (*sip).hdrend = 0 as off_t;
    (*sip).datastart = 0 as off_t;
    (*sip).dataend = 0 as off_t;
    (*sip).endofstripeflag = false_0 as boolean;
    (*sip).endofpageflag = false_0 as boolean;
    (*sip).pageinfoflag = false_0 as boolean;
    (*sip).endoffileflag = false_0 as boolean;
}
unsafe fn initlinkedlist(mut lp: *mut LIST) {
    (*lp).first = ::core::ptr::null_mut::<LITEM>();
    (*lp).last = ::core::ptr::null_mut::<LITEM>();
    (*lp).tree = ::core::ptr::null_mut::<avl_table>();
}
unsafe fn litem_append(mut lp: *mut LIST) -> *mut LIST {
    let mut ip: *mut LITEM = ::core::ptr::null_mut::<LITEM>();
    ip = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<LITEM>() as size_t))
        as *mut LITEM;
    if (*lp).first.is_null() {
        (*lp).first = ip;
        (*ip).prev = ::core::ptr::null_mut::<_LITEM>();
    } else {
        (*(*lp).last).next = ip as *mut _LITEM;
        (*ip).prev = (*lp).last as *mut _LITEM;
    }
    (*lp).last = ip;
    (*ip).next = ::core::ptr::null_mut::<_LITEM>();
    (*ip).d = NULL;
    return lp;
}
unsafe fn pages_maketree(mut plp: *mut LIST) {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };

    let mut ip: *mut LITEM = ::core::ptr::null_mut::<LITEM>();
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if !(*plp).tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"pages_maketree\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            304 as ::core::ffi::c_int,
            b"plp->tree == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*plp).tree = avl_create(
        Some(
            comp_page_entry
                as unsafe fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
        &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
    );
    if (*plp).tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"pages_maketree\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            306 as ::core::ffi::c_int,
            b"plp->tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    ip = (*plp).first;
    while !ip.is_null() {
        aa = avl_probe(
            (*plp).tree,
            (*ip).d as *mut PAGEINFO as *mut ::core::ffi::c_void,
        );
        if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"pages_maketree\0" as *const u8 as *const ::core::ffi::c_char,
                b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
                309 as ::core::ffi::c_int,
                b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        ip = (*ip).next as *mut LITEM;
    }
}
unsafe fn segments_maketree(mut slp: *mut LIST) {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };

    let mut ip: *mut LITEM = ::core::ptr::null_mut::<LITEM>();
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if !(*slp).tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"segments_maketree\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            317 as ::core::ffi::c_int,
            b"slp->tree == NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    (*slp).tree = avl_create(
        Some(
            comp_segment_entry
                as unsafe fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        NULL,
        &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
    );
    if (*slp).tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"segments_maketree\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            319 as ::core::ffi::c_int,
            b"slp->tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    ip = (*slp).first;
    while !ip.is_null() {
        aa = avl_probe(
            (*slp).tree,
            (*ip).d as *mut SEGINFO as *mut ::core::ffi::c_void,
        );
        if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"segments_maketree\0" as *const u8 as *const ::core::ffi::c_char,
                b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
                322 as ::core::ffi::c_int,
                b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        ip = (*ip).next as *mut LITEM;
    }
}
unsafe fn find_pageinfo(mut plp: *mut LIST, mut pagenum: ::core::ffi::c_ulong) -> *mut PAGEINFO {
    let mut tmp: PAGEINFO = _PAGEINFO {
        segments: _LIST {
            first: ::core::ptr::null_mut::<LITEM>(),
            last: ::core::ptr::null_mut::<LITEM>(),
            tree: ::core::ptr::null_mut::<avl_table>(),
        },
        pagenum: 0,
        width: 0,
        height: 0,
        xres: 0,
        yres: 0,
        pagesegmentflags: 0,
        stripinginfo: 0,
        stripedheight: 0,
    };
    tmp.pagenum = pagenum;
    if (*plp).tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"find_pageinfo\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            332 as ::core::ffi::c_int,
            b"plp->tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return avl_find((*plp).tree, &raw mut tmp as *const ::core::ffi::c_void) as *mut PAGEINFO;
}
unsafe fn find_seginfo(mut slp: *mut LIST, mut segnum: ::core::ffi::c_ulong) -> *mut SEGINFO {
    let mut tmp: SEGINFO = _SEGINFO {
        segnum: 0,
        isrefered: 0,
        refers: 0,
        seghdrflags: 0,
        pageassocsizeflag: 0,
        reftosegcount: 0,
        countofrefered: 0,
        fieldlen: 0,
        segnumwidth: 0,
        segpage: 0,
        segdatalen: 0,
        hdrstart: 0,
        hdrend: 0,
        datastart: 0,
        dataend: 0,
        endofstripeflag: 0,
        endofpageflag: 0,
        pageinfoflag: 0,
        endoffileflag: 0,
    };
    tmp.segnum = segnum;
    if (*slp).tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"find_seginfo\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            340 as ::core::ffi::c_int,
            b"slp->tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return avl_find((*slp).tree, &raw mut tmp as *const ::core::ffi::c_void) as *mut SEGINFO;
}
unsafe fn read2bytes(mut f: *mut FILE) -> ::core::ffi::c_uint {
    let mut c: ::core::ffi::c_uint = ygetc(f) as ::core::ffi::c_uint;
    return (c << 8 as ::core::ffi::c_int).wrapping_add(ygetc(f) as ::core::ffi::c_uint);
}
unsafe fn read4bytes(mut f: *mut FILE) -> ::core::ffi::c_ulong {
    let mut l: ::core::ffi::c_uint = read2bytes(f);
    return (l << 16 as ::core::ffi::c_int).wrapping_add(read2bytes(f)) as ::core::ffi::c_ulong;
}
unsafe fn getstreamlen(mut slip: *mut LITEM, mut refer: boolean) -> off_t {
    let mut sip: *mut SEGINFO = ::core::ptr::null_mut::<SEGINFO>();
    let mut len: off_t = 0 as off_t;
    while !slip.is_null() {
        sip = (*slip).d as *mut SEGINFO;
        if refer != 0 || (*sip).isrefered != 0 {
            len += (*sip).hdrend - (*sip).hdrstart + ((*sip).dataend - (*sip).datastart);
        }
        slip = (*slip).next as *mut LITEM;
    }
    return len;
}
unsafe fn readfilehdr(mut fip: *mut FILEINFO) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut jbig2_id: [::core::ffi::c_uchar; 8] = [
        0x97 as ::core::ffi::c_int as ::core::ffi::c_uchar,
        'J' as i32 as ::core::ffi::c_uchar,
        'B' as i32 as ::core::ffi::c_uchar,
        '2' as i32 as ::core::ffi::c_uchar,
        0xd as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0x1a as ::core::ffi::c_int as ::core::ffi::c_uchar,
        0xa as ::core::ffi::c_int as ::core::ffi::c_uchar,
    ];
    xfseeko(
        (*fip).file,
        0 as ::core::ffi::c_int as off_t,
        SEEK_SET,
        (*fip).filename as const_string,
    );
    i = 0 as ::core::ffi::c_uint;
    while i < 8 as ::core::ffi::c_uint {
        if ygetc((*fip).file) != jbig2_id[i as usize] as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"readfilehdr(): reading JBIG2 image file failed: ID string missing\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        i = i.wrapping_add(1);
    }
    (*fip).filehdrflags = ygetc((*fip).file) as ::core::ffi::c_uint;
    (*fip).sequentialaccess = (if (*fip).filehdrflags & 0x1 as ::core::ffi::c_uint != 0 {
        true_0
    } else {
        false_0
    }) as boolean;
    if (*fip).sequentialaccess != 0 {
        xfseeko(
            (*fip).file,
            0 as ::core::ffi::c_int as off_t,
            SEEK_END,
            (*fip).filename as const_string,
        );
        (*fip).filesize = xftello((*fip).file, (*fip).filename as const_string);
        xfseeko(
            (*fip).file,
            9 as ::core::ffi::c_int as off_t,
            SEEK_SET,
            (*fip).filename as const_string,
        );
    }
    if ((*fip).filehdrflags >> 1 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
        & 0x1 as ::core::ffi::c_int
        != 0
    {
        (*fip).numofpages = read4bytes((*fip).file);
    }
}
unsafe fn readseghdr(mut fip: *mut FILEINFO, mut sip: *mut SEGINFO) -> boolean {
    let mut i: ::core::ffi::c_uint = 0;
    (*sip).hdrstart = xftello((*fip).file, (*fip).filename as const_string);
    if (*fip).sequentialaccess != 0 && (*sip).hdrstart == (*fip).filesize {
        return false_0;
    }
    (*sip).segnum = read4bytes((*fip).file);
    (*sip).seghdrflags = ygetc((*fip).file) as ::core::ffi::c_uint;
    checkseghdrflags(sip);
    if (*fip).sequentialaccess != 0 && (*sip).endoffileflag != 0 {
        return true_0;
    }
    (*sip).pageassocsizeflag =
        (if (*sip).seghdrflags >> 6 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_uint != 0 {
            true_0
        } else {
            false_0
        }) as boolean;
    (*sip).reftosegcount = ygetc((*fip).file) as ::core::ffi::c_uint;
    (*sip).countofrefered = (*sip).reftosegcount >> 5 as ::core::ffi::c_int;
    if (*sip).countofrefered < 5 as ::core::ffi::c_uint {
        (*sip).fieldlen = 1 as ::core::ffi::c_uint;
    } else {
        (*sip).fieldlen = (5 as ::core::ffi::c_uint)
            .wrapping_add((*sip).countofrefered.wrapping_div(8 as ::core::ffi::c_uint));
        xfseeko(
            (*fip).file,
            (*sip).fieldlen as off_t - 1 as off_t,
            SEEK_CUR,
            (*fip).filename as const_string,
        );
    }
    if (*sip).segnum <= 256 as ::core::ffi::c_ulong {
        (*sip).segnumwidth = 1 as ::core::ffi::c_uint;
    } else if (*sip).segnum <= 65536 as ::core::ffi::c_ulong {
        (*sip).segnumwidth = 2 as ::core::ffi::c_uint;
    } else {
        (*sip).segnumwidth = 4 as ::core::ffi::c_uint;
    }
    i = 0 as ::core::ffi::c_uint;
    while i < (*sip).countofrefered {
        match (*sip).segnumwidth {
            1 => {
                ygetc((*fip).file);
            }
            2 => {
                read2bytes((*fip).file);
            }
            4 => {
                read4bytes((*fip).file);
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if (*sip).pageassocsizeflag != 0 {
        (*sip).segpage = read4bytes((*fip).file) as ::core::ffi::c_long;
    } else {
        (*sip).segpage = ygetc((*fip).file) as ::core::ffi::c_long;
    }
    (*sip).segdatalen = read4bytes((*fip).file);
    (*sip).hdrend = xftello((*fip).file, (*fip).filename as const_string);
    return true_0;
}
unsafe fn writeseghdr(mut fip: *mut FILEINFO, mut sip: *mut SEGINFO) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut i: ::core::ffi::c_uint = 0;
    let mut referedseg: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    i = 0 as ::core::ffi::c_uint;
    while i < (5 as ::core::ffi::c_uint).wrapping_add((*sip).fieldlen) {
        if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr) as ::core::ffi::c_uint
            > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
        {
            if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                zpdfosgetosbuf(1 as ::core::ffi::c_int);
            } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
            {
                crate::utils::pdftex_fail_args(
                    b"PDF output buffer overflowed\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            } else {
                pdfflush();
            }
        }
        let fresh1 = (*__tekai_state_generated_pdftexextra).pdfptr;
        (*__tekai_state_generated_pdftexextra).pdfptr =
            (*__tekai_state_generated_pdftexextra).pdfptr + 1;
        *(*__tekai_state_generated_pdftexextra)
            .pdfbuf
            .offset(fresh1 as isize) = ygetc((*fip).file) as eightbits;
        i = i.wrapping_add(1);
    }
    i = 0 as ::core::ffi::c_uint;
    while i < (*sip).countofrefered {
        match (*sip).segnumwidth {
            1 => {
                referedseg = ygetc((*fip).file) as ::core::ffi::c_ulong;
                if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(1 as ::core::ffi::c_int);
                    } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                let fresh2 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh2 as isize) = referedseg as eightbits;
            }
            2 => {
                referedseg = read2bytes((*fip).file) as ::core::ffi::c_ulong;
                if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(1 as ::core::ffi::c_int);
                    } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                let fresh3 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh3 as isize) = (referedseg >> 8 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_ulong)
                    as eightbits;
                if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(1 as ::core::ffi::c_int);
                    } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                let fresh4 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh4 as isize) =
                    (referedseg & 0xff as ::core::ffi::c_ulong) as eightbits;
            }
            4 => {
                referedseg = read4bytes((*fip).file);
                if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(1 as ::core::ffi::c_int);
                    } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                let fresh5 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh5 as isize) = (referedseg >> 24 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_ulong)
                    as eightbits;
                if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(1 as ::core::ffi::c_int);
                    } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                let fresh6 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh6 as isize) = (referedseg >> 16 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_ulong)
                    as eightbits;
                if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(1 as ::core::ffi::c_int);
                    } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                let fresh7 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh7 as isize) = (referedseg >> 8 as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_ulong)
                    as eightbits;
                if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(1 as ::core::ffi::c_int);
                    } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                let fresh8 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh8 as isize) =
                    (referedseg & 0xff as ::core::ffi::c_ulong) as eightbits;
            }
            _ => {}
        }
        if !(*fip).page0.last.is_null() && (*sip).refers == 0 {
            markpage0seg(fip, referedseg);
        }
        i = i.wrapping_add(1);
    }
    if (*sip).countofrefered > 0 as ::core::ffi::c_uint {
        (*sip).refers = true_0 as boolean;
    }
    if (*sip).pageassocsizeflag != 0 {
        i = 0 as ::core::ffi::c_uint;
        while i < 3 as ::core::ffi::c_uint {
            ygetc((*fip).file);
            if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr) as ::core::ffi::c_uint
                > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
            {
                if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                    zpdfosgetosbuf(1 as ::core::ffi::c_int);
                } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"PDF output buffer overflowed\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
                } else {
                    pdfflush();
                }
            }
            let fresh9 = (*__tekai_state_generated_pdftexextra).pdfptr;
            (*__tekai_state_generated_pdftexextra).pdfptr =
                (*__tekai_state_generated_pdftexextra).pdfptr + 1;
            *(*__tekai_state_generated_pdftexextra)
                .pdfbuf
                .offset(fresh9 as isize) = 0 as eightbits;
            i = i.wrapping_add(1);
        }
    }
    ygetc((*fip).file);
    if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr) as ::core::ffi::c_uint
        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
    {
        if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
            zpdfosgetosbuf(1 as ::core::ffi::c_int);
        } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
            > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"PDF output buffer overflowed\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
        } else {
            pdfflush();
        }
    }
    let fresh10 = (*__tekai_state_generated_pdftexextra).pdfptr;
    (*__tekai_state_generated_pdftexextra).pdfptr =
        (*__tekai_state_generated_pdftexextra).pdfptr + 1;
    *(*__tekai_state_generated_pdftexextra)
        .pdfbuf
        .offset(fresh10 as isize) = (if (*sip).segpage > 0 as ::core::ffi::c_long {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    }) as eightbits;
    i = 0 as ::core::ffi::c_uint;
    while i < 4 as ::core::ffi::c_uint {
        if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr) as ::core::ffi::c_uint
            > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
        {
            if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                zpdfosgetosbuf(1 as ::core::ffi::c_int);
            } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
            {
                crate::utils::pdftex_fail_args(
                    b"PDF output buffer overflowed\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            } else {
                pdfflush();
            }
        }
        let fresh11 = (*__tekai_state_generated_pdftexextra).pdfptr;
        (*__tekai_state_generated_pdftexextra).pdfptr =
            (*__tekai_state_generated_pdftexextra).pdfptr + 1;
        *(*__tekai_state_generated_pdftexextra)
            .pdfbuf
            .offset(fresh11 as isize) = ygetc((*fip).file) as eightbits;
        i = i.wrapping_add(1);
    }
}
unsafe fn checkseghdr(mut fip: *mut FILEINFO, mut sip: *mut SEGINFO) {
    let mut i: ::core::ffi::c_uint = 0;
    let mut referedseg: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    xfseeko(
        (*fip).file,
        (*sip).fieldlen as off_t + 5 as off_t,
        SEEK_CUR,
        (*fip).filename as const_string,
    );
    i = 0 as ::core::ffi::c_uint;
    while i < (*sip).countofrefered {
        match (*sip).segnumwidth {
            1 => {
                referedseg = ygetc((*fip).file) as ::core::ffi::c_ulong;
            }
            2 => {
                referedseg = read2bytes((*fip).file) as ::core::ffi::c_ulong;
            }
            4 => {
                referedseg = read4bytes((*fip).file);
            }
            _ => {}
        }
        if (*sip).refers == 0 {
            markpage0seg(fip, referedseg);
        }
        i = i.wrapping_add(1);
    }
    if (*sip).countofrefered > 0 as ::core::ffi::c_uint {
        (*sip).refers = true_0 as boolean;
    }
    if (*sip).pageassocsizeflag != 0 {
        xfseeko(
            (*fip).file,
            8 as off_t,
            SEEK_CUR,
            (*fip).filename as const_string,
        );
    } else {
        xfseeko(
            (*fip).file,
            5 as off_t,
            SEEK_CUR,
            (*fip).filename as const_string,
        );
    };
}
unsafe fn checkseghdrflags(mut sip: *mut SEGINFO) {
    (*sip).endofstripeflag = false_0 as boolean;
    (*sip).endofpageflag = false_0 as boolean;
    (*sip).pageinfoflag = false_0 as boolean;
    (*sip).endoffileflag = false_0 as boolean;
    match (*sip).seghdrflags & 0x3f as ::core::ffi::c_uint {
        48 => {
            (*sip).pageinfoflag = true_0 as boolean;
        }
        49 => {
            (*sip).endofpageflag = true_0 as boolean;
        }
        50 => {
            (*sip).endofstripeflag = true_0 as boolean;
        }
        51 => {
            (*sip).endoffileflag = true_0 as boolean;
        }
        0 | 4 | 6 | 7 | 16 | 20 | 22 | 23 | 36 | 38 | 39 | 40 | 42 | 43 | 52 | 53 | 62 => {}
        _ => {
            crate::utils::pdftex_fail_args(
                b"checkseghdrflags(): unknown segment type in JBIG2 image file\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
    };
}
unsafe fn markpage0seg(mut fip: *mut FILEINFO, mut referedseg: ::core::ffi::c_ulong) {
    let mut pip: *mut PAGEINFO = ::core::ptr::null_mut::<PAGEINFO>();
    let mut sip: *mut SEGINFO = ::core::ptr::null_mut::<SEGINFO>();
    pip = (*(*fip).page0.first).d as *mut PAGEINFO;
    sip = find_seginfo(&raw mut (*pip).segments, referedseg);
    if !sip.is_null() {
        if (*sip).refers == 0 && (*sip).countofrefered > 0 as ::core::ffi::c_uint {
            checkseghdr(fip, sip);
        }
        (*sip).isrefered = true_0 as boolean;
    }
}
unsafe fn findstreamstart(mut fip: *mut FILEINFO) -> off_t {
    let mut tmp: SEGINFO = _SEGINFO {
        segnum: 0,
        isrefered: 0,
        refers: 0,
        seghdrflags: 0,
        pageassocsizeflag: 0,
        reftosegcount: 0,
        countofrefered: 0,
        fieldlen: 0,
        segnumwidth: 0,
        segpage: 0,
        segdatalen: 0,
        hdrstart: 0,
        hdrend: 0,
        datastart: 0,
        dataend: 0,
        endofstripeflag: 0,
        endofpageflag: 0,
        pageinfoflag: 0,
        endoffileflag: 0,
    };
    if ((*fip).sequentialaccess != 0) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"findstreamstart\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            614 as ::core::ffi::c_int,
            b"!fip->sequentialaccess\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    loop {
        readseghdr(fip, &raw mut tmp);
        if !(tmp.endoffileflag == 0) {
            break;
        }
    }
    (*fip).streamstart = tmp.hdrend;
    readfilehdr(fip);
    return (*fip).streamstart;
}
unsafe fn rd_jbig2_info(mut fip: *mut FILEINFO) {
    let mut seekdist: off_t = 0 as off_t;
    let mut streampos: off_t = 0 as off_t;
    let mut currentpage: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut sipavail: boolean = false_0;
    let mut pip: *mut PAGEINFO = ::core::ptr::null_mut::<PAGEINFO>();
    let mut sip: *mut SEGINFO = ::core::ptr::null_mut::<SEGINFO>();
    let mut plp: *mut LIST = ::core::ptr::null_mut::<LIST>();
    let mut slp: *mut LIST = ::core::ptr::null_mut::<LIST>();
    (*fip).file = xfopen((*fip).filename as const_string, FOPEN_RBIN_MODE.as_ptr());
    readfilehdr(fip);
    if (*fip).sequentialaccess == 0 {
        streampos = findstreamstart(fip);
    }
    loop {
        if sipavail == 0 {
            sip = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<SEGINFO>() as size_t))
                as *mut SEGINFO;
            sipavail = true_0 as boolean;
        }
        init_seginfo(sip);
        if readseghdr(fip, sip) == 0 || (*sip).endoffileflag != 0 {
            break;
        }
        if (*sip).segpage > 0 as ::core::ffi::c_long {
            if (*sip).segpage as ::core::ffi::c_ulong > currentpage {
                plp = litem_append(&raw mut (*fip).pages);
                (*(*plp).last).d = new_pageinfo() as *mut ::core::ffi::c_void;
                currentpage = (*sip).segpage as ::core::ffi::c_ulong;
            }
            pip = (*(*fip).pages.last).d as *mut PAGEINFO;
        } else {
            if (*fip).page0.last.is_null() {
                plp = litem_append(&raw mut (*fip).page0);
                (*(*plp).last).d = new_pageinfo() as *mut ::core::ffi::c_void;
            }
            pip = (*(*fip).page0.last).d as *mut PAGEINFO;
        }
        if (*sip).endofpageflag == 0 {
            slp = litem_append(&raw mut (*pip).segments);
            (*(*slp).last).d = sip as *mut ::core::ffi::c_void;
            sipavail = false_0 as boolean;
        }
        if (*fip).sequentialaccess == 0 {
            (*sip).datastart = streampos;
        } else {
            (*sip).datastart = (*sip).hdrend;
        }
        (*sip).dataend = ((*sip).datastart as ::core::ffi::c_ulonglong)
            .wrapping_add((*sip).segdatalen as ::core::ffi::c_ulonglong)
            as off_t;
        if (*fip).sequentialaccess == 0 && ((*sip).pageinfoflag != 0 || (*sip).endofstripeflag != 0)
        {
            xfseeko(
                (*fip).file,
                (*sip).datastart,
                SEEK_SET,
                (*fip).filename as const_string,
            );
        }
        seekdist = (*sip).segdatalen as off_t;
        if (*sip).pageinfoflag != 0 {
            (*pip).pagenum = (*sip).segpage as ::core::ffi::c_ulong;
            (*pip).width = read4bytes((*fip).file) as ::core::ffi::c_uint;
            (*pip).height = read4bytes((*fip).file) as ::core::ffi::c_uint;
            (*pip).xres = read4bytes((*fip).file) as ::core::ffi::c_uint;
            (*pip).yres = read4bytes((*fip).file) as ::core::ffi::c_uint;
            (*pip).pagesegmentflags = ygetc((*fip).file) as ::core::ffi::c_uint;
            (*pip).stripinginfo = read2bytes((*fip).file);
            seekdist -= 19 as off_t;
        }
        if (*sip).endofstripeflag != 0 {
            (*pip).stripedheight = read4bytes((*fip).file) as ::core::ffi::c_uint;
            seekdist -= 4 as off_t;
        }
        if (*fip).sequentialaccess == 0 && ((*sip).pageinfoflag != 0 || (*sip).endofstripeflag != 0)
        {
            xfseeko(
                (*fip).file,
                (*sip).hdrend,
                SEEK_SET,
                (*fip).filename as const_string,
            );
        }
        if (*fip).sequentialaccess == 0 {
            streampos = (streampos as ::core::ffi::c_ulonglong)
                .wrapping_add((*sip).segdatalen as ::core::ffi::c_ulonglong)
                as off_t as off_t;
        }
        if (*fip).sequentialaccess != 0 {
            xfseeko(
                (*fip).file,
                seekdist,
                SEEK_CUR,
                (*fip).filename as const_string,
            );
        }
        if (*sip).endofpageflag != 0
            && currentpage != 0
            && (*pip).stripinginfo >> 15 as ::core::ffi::c_int != 0
        {
            (*pip).height = (*pip).stripedheight;
        }
    }
    (*fip).phase = HAVEINFO;
    if sipavail != 0 {
        if !sip.is_null() {
            free(sip as *mut ::core::ffi::c_void);
        }
        sip = ::core::ptr::null_mut::<SEGINFO>();
    }
    xfclose((*fip).file, (*fip).filename as const_string);
}
unsafe fn wr_jbig2(mut fip: *mut FILEINFO, mut page: ::core::ffi::c_ulong) {
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut slip: *mut LITEM = ::core::ptr::null_mut::<LITEM>();
    let mut pip: *mut PAGEINFO = ::core::ptr::null_mut::<PAGEINFO>();
    let mut sip: *mut SEGINFO = ::core::ptr::null_mut::<SEGINFO>();
    let mut i: off_t = 0;
    if page > 0 as ::core::ffi::c_ulong {
        pip = find_pageinfo(&raw mut (*fip).pages, page);
        if pip.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"wr_jbig2\0" as *const u8 as *const ::core::ffi::c_char,
                b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
                716 as ::core::ffi::c_int,
                b"pip != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        pdf_puts(b"/Type /XObject\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdf_puts(b"/Subtype /Image\n\0" as *const u8 as *const ::core::ffi::c_char);
        crate::utils::pdf_printf_args(
            b"/Width %i\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from((*pip).width)],
        );
        crate::utils::pdf_printf_args(
            b"/Height %i\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from((*pip).height)],
        );
        pdf_puts(b"/ColorSpace /DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdf_puts(b"/BitsPerComponent 1\n\0" as *const u8 as *const ::core::ffi::c_char);
        crate::utils::pdf_printf_args(
            b"/Length %ld\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                getstreamlen((*pip).segments.first, true_0) as ::core::ffi::c_long,
            )],
        );
        pdf_puts(b"/Filter [/JBIG2Decode]\n\0" as *const u8 as *const ::core::ffi::c_char);
        if !(*fip).page0.last.is_null() {
            if (*fip).pdfpage0objnum == 0 as ::core::ffi::c_ulong {
                zpdfcreateobj(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
                (*fip).pdfpage0objnum =
                    (*__tekai_state_generated_pdftexextra).objptr as ::core::ffi::c_ulong;
            }
            crate::utils::pdf_printf_args(
                b"/DecodeParms [<< /JBIG2Globals %lu 0 R >>]\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from((*fip).pdfpage0objnum)],
            );
        }
    } else {
        pip = find_pageinfo(&raw mut (*fip).page0, page);
        if pip.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"wr_jbig2\0" as *const u8 as *const ::core::ffi::c_char,
                b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
                736 as ::core::ffi::c_int,
                b"pip != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        zpdfbegindict((*fip).pdfpage0objnum as integer, 0 as ::core::ffi::c_int);
        crate::utils::pdf_printf_args(
            b"/Length %ld\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                getstreamlen((*pip).segments.first, false_0) as ::core::ffi::c_long,
            )],
        );
    }
    pdf_puts(b">>\n\0" as *const u8 as *const ::core::ffi::c_char);
    pdf_puts(b"stream\n\0" as *const u8 as *const ::core::ffi::c_char);
    (*fip).file = xfopen((*fip).filename as const_string, FOPEN_RBIN_MODE.as_ptr());
    slip = (*pip).segments.first;
    while !slip.is_null() {
        sip = (*slip).d as *mut SEGINFO;
        if (*sip).isrefered != 0 || page > 0 as ::core::ffi::c_ulong {
            xfseeko(
                (*fip).file,
                (*sip).hdrstart,
                SEEK_SET,
                (*fip).filename as const_string,
            );
            writeseghdr(fip, sip);
            xfseeko(
                (*fip).file,
                (*sip).datastart,
                SEEK_SET,
                (*fip).filename as const_string,
            );
            i = (*sip).datastart;
            while i < (*sip).dataend {
                if (1 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(1 as ::core::ffi::c_int);
                    } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        crate::utils::pdftex_fail_args(
                            b"PDF output buffer overflowed\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[],
                        );
                    } else {
                        pdfflush();
                    }
                }
                let fresh0 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh0 as isize) = ygetc((*fip).file) as eightbits;
                i += 1;
            }
        }
        slip = (*slip).next as *mut LITEM;
    }
    pdfendstream();
    xfclose((*fip).file, (*fip).filename as const_string);
}
pub unsafe fn read_jbig2_info(mut img: integer) {
    let __tekai_state_generated_backend_avlstuff: *mut crate::generated::backend::avlstuff::State =
        unsafe { crate::generated::backend::avlstuff::state() };
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };
    let __tekai_state_generated_backend_writejbig2: *mut crate::generated::backend::writejbig2::State = unsafe { crate::generated::backend::writejbig2::state() };

    let mut fip: *mut FILEINFO = ::core::ptr::null_mut::<FILEINFO>();
    let mut tmp: FILEINFO = _FILEINFO {
        file: ::core::ptr::null_mut::<FILE>(),
        filename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        filesize: 0,
        pages: _LIST {
            first: ::core::ptr::null_mut::<LITEM>(),
            last: ::core::ptr::null_mut::<LITEM>(),
            tree: ::core::ptr::null_mut::<avl_table>(),
        },
        page0: _LIST {
            first: ::core::ptr::null_mut::<LITEM>(),
            last: ::core::ptr::null_mut::<LITEM>(),
            tree: ::core::ptr::null_mut::<avl_table>(),
        },
        filehdrflags: 0,
        sequentialaccess: 0,
        numofpages: 0,
        streamstart: 0,
        pdfpage0objnum: 0,
        phase: INITIAL,
    };
    let mut pip: *mut PAGEINFO = ::core::ptr::null_mut::<PAGEINFO>();
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    if (*(*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .image_struct
    .jbig2)
        .selected_page
        < 1 as ::core::ffi::c_int
    {
        crate::utils::pdftex_fail_args(
            b"read_jbig2_info(): page %d not in JBIG2 image file; page must be > 0\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*(*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .jbig2)
                    .selected_page,
            )],
        );
    }
    if (*__tekai_state_generated_backend_writejbig2)
        .file_tree
        .is_null()
    {
        (*__tekai_state_generated_backend_writejbig2).file_tree = avl_create(
            Some(
                comp_file_entry
                    as unsafe fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut (*__tekai_state_generated_backend_avlstuff).avl_xallocator,
        );
        if (*__tekai_state_generated_backend_writejbig2)
            .file_tree
            .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            __assert_rtn(
                b"read_jbig2_info\0" as *const u8 as *const ::core::ffi::c_char,
                b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
                772 as ::core::ffi::c_int,
                b"file_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    tmp.filename = (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .image_name;
    fip = avl_find(
        (*__tekai_state_generated_backend_writejbig2).file_tree,
        &raw mut tmp as *const ::core::ffi::c_void,
    ) as *mut FILEINFO;
    if fip.is_null() {
        fip = new_fileinfo();
        (*fip).filename = xstrdup(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_name as const_string,
        ) as *mut ::core::ffi::c_char;
        aa = avl_probe(
            (*__tekai_state_generated_backend_writejbig2).file_tree,
            fip as *mut ::core::ffi::c_void,
        );
        if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"read_jbig2_info\0" as *const u8 as *const ::core::ffi::c_char,
                b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
                780 as ::core::ffi::c_int,
                b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    if (*fip).phase as ::core::ffi::c_uint == INITIAL as ::core::ffi::c_int as ::core::ffi::c_uint {
        rd_jbig2_info(fip);
        pages_maketree(&raw mut (*fip).pages);
        if !(*fip).page0.last.is_null() {
            pages_maketree(&raw mut (*fip).page0);
            pip = (*(*fip).page0.first).d as *mut PAGEINFO;
            segments_maketree(&raw mut (*pip).segments);
        }
    }
    pip = find_pageinfo(
        &raw mut (*fip).pages,
        (*(*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .jbig2)
            .selected_page as ::core::ffi::c_ulong,
    );
    if pip.is_null() {
        crate::utils::pdftex_fail_args(
            b"read_jbig2_info(): page %d not found in JBIG2 image file\0" as *const u8
                as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*(*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .jbig2)
                    .selected_page,
            )],
        );
    }
    (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .num_pages = (*fip).numofpages as integer;
    (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .width = (*pip).width as integer;
    (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .height = (*pip).height as integer;
    (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .x_res = ((*pip).xres as ::core::ffi::c_double * 0.0254f64 + 0.5f64) as ::core::ffi::c_int
        as integer;
    (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .y_res = ((*pip).yres as ::core::ffi::c_double * 0.0254f64 + 0.5f64) as ::core::ffi::c_int
        as integer;
}
pub unsafe fn write_jbig2(mut img: integer) {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };
    let __tekai_state_generated_backend_writejbig2: *mut crate::generated::backend::writejbig2::State = unsafe { crate::generated::backend::writejbig2::state() };

    let mut fip: *mut FILEINFO = ::core::ptr::null_mut::<FILEINFO>();
    let mut tmp: FILEINFO = _FILEINFO {
        file: ::core::ptr::null_mut::<FILE>(),
        filename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        filesize: 0,
        pages: _LIST {
            first: ::core::ptr::null_mut::<LITEM>(),
            last: ::core::ptr::null_mut::<LITEM>(),
            tree: ::core::ptr::null_mut::<avl_table>(),
        },
        page0: _LIST {
            first: ::core::ptr::null_mut::<LITEM>(),
            last: ::core::ptr::null_mut::<LITEM>(),
            tree: ::core::ptr::null_mut::<avl_table>(),
        },
        filehdrflags: 0,
        sequentialaccess: 0,
        numofpages: 0,
        streamstart: 0,
        pdfpage0objnum: 0,
        phase: INITIAL,
    };
    let mut pip: *mut PAGEINFO = ::core::ptr::null_mut::<PAGEINFO>();
    if (*__tekai_state_generated_backend_writejbig2)
        .file_tree
        .is_null() as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"write_jbig2\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            808 as ::core::ffi::c_int,
            b"file_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    tmp.filename = (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .image_name;
    fip = avl_find(
        (*__tekai_state_generated_backend_writejbig2).file_tree,
        &raw mut tmp as *const ::core::ffi::c_void,
    ) as *mut FILEINFO;
    if fip.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_jbig2\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            811 as ::core::ffi::c_int,
            b"fip != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if !((*fip).phase as ::core::ffi::c_uint
        == HAVEINFO as ::core::ffi::c_int as ::core::ffi::c_uint) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __assert_rtn(
            b"write_jbig2\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            812 as ::core::ffi::c_int,
            b"fip->phase == HAVEINFO\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    pip = find_pageinfo(
        &raw mut (*fip).pages,
        (*(*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .jbig2)
            .selected_page as ::core::ffi::c_ulong,
    );
    if pip.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"write_jbig2\0" as *const u8 as *const ::core::ffi::c_char,
            b"writejbig2.c\0" as *const u8 as *const ::core::ffi::c_char,
            814 as ::core::ffi::c_int,
            b"pip != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    wr_jbig2(fip, (*pip).pagenum);
}
pub unsafe fn flushjbig2page0objects() {
    let __tekai_state_generated_backend_writejbig2: *mut crate::generated::backend::writejbig2::State = unsafe { crate::generated::backend::writejbig2::state() };

    let mut fip: *mut FILEINFO = ::core::ptr::null_mut::<FILEINFO>();
    let mut t: avl_traverser = avl_traverser {
        avl_table: ::core::ptr::null_mut::<avl_table>(),
        avl_node: ::core::ptr::null_mut::<avl_node>(),
        avl_stack: [::core::ptr::null_mut::<avl_node>(); 32],
        avl_height: 0,
        avl_generation: 0,
    };
    if !(*__tekai_state_generated_backend_writejbig2)
        .file_tree
        .is_null()
    {
        avl_t_init(
            &raw mut t,
            (*__tekai_state_generated_backend_writejbig2).file_tree,
        );
        fip = avl_t_first(
            &raw mut t,
            (*__tekai_state_generated_backend_writejbig2).file_tree,
        ) as *mut FILEINFO;
        while !fip.is_null() {
            if !(*fip).page0.last.is_null() {
                wr_jbig2(fip, 0 as ::core::ffi::c_ulong);
            }
            fip = avl_t_next(&raw mut t) as *mut FILEINFO;
        }
    }
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

pub(crate) struct State {
    file_tree: *mut avl_table,
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 1],
}
#[inline]
unsafe fn drop_initialized<T>(pointer: *mut T) {
    if core::mem::needs_drop::<T>() {
        unsafe { core::ptr::drop_in_place(pointer) };
    }
}

impl Drop for StateInitialization {
    fn drop(&mut self) {
        unsafe {
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!((*self.destination).file_tree));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 1],
        }
    }
    pub(crate) fn disarm(&mut self) {
        self.initialized.fill(false);
    }
    pub(crate) unsafe fn initialize_fields(
        &mut self,
        invocation: *const crate::invocation_state::Storage,
    ) {
        let destination = self.destination;
        let _ = invocation;
        const {
            assert!(core::mem::size_of::<*mut avl_table>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).file_tree)
            .write(::core::ptr::null::<avl_table>() as *mut avl_table);
        self.initialized[0] = true;
    }
    pub(crate) unsafe fn initialize_fixups(
        &mut self,
        invocation: *const crate::invocation_state::Storage,
    ) {
        let destination = self.destination;
        let _ = invocation;
    }
}
pub(crate) unsafe fn state() -> *mut State {
    core::cell::UnsafeCell::raw_get(core::ptr::addr_of!(
        (*crate::invocation_state::current()).generated_backend_writejbig2
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}

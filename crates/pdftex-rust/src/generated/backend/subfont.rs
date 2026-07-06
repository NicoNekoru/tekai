#[repr(C)]
pub struct __sFILEX {
    _unused: [u8; 0],
}

extern "C" {
    fn feof(_: *mut FILE) -> ::core::ffi::c_int;
    fn fgetc(_: *mut FILE) -> ::core::ffi::c_int;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn free(_: *mut ::core::ffi::c_void);
    fn strcat(
        __s1: *mut ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    fn xstrdup(s: const_string) -> string;
    fn xfclose(fp: *mut FILE, filename: const_string);
    fn xmalloc(size: size_t) -> address;
    fn open_input(_: *mut *mut FILE, _: ::core::ffi::c_int, fopen_mode: const_string) -> boolean;
    fn zpackfilename(n: strnumber, a: strnumber, e: strnumber);
    fn getnullstr() -> strnumber;
    fn avl_create(
        _: Option<avl_comparison_func>,
        _: *mut ::core::ffi::c_void,
        _: *mut libavl_allocator,
    ) -> *mut avl_table;
    fn avl_destroy(_: *mut avl_table, _: Option<avl_item_func>);
    fn avl_probe(_: *mut avl_table, _: *mut ::core::ffi::c_void) -> *mut *mut ::core::ffi::c_void;
    fn avl_find(_: *const avl_table, _: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    static mut avl_xallocator: libavl_allocator;
    static mut cur_file_name: *mut ::core::ffi::c_char;
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn pdftex_warn(_: *const ::core::ffi::c_char, ...);
    fn tex_printf(_: *const ::core::ffi::c_char, ...);
    fn new_fm_entry() -> *mut fm_entry;
    fn delete_fm_entry(_: *mut fm_entry);
    fn avl_do_entry(_: *mut fm_entry, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn maketexstring(_: *const ::core::ffi::c_char) -> strnumber;
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut ::core::ffi::c_uchar,
    pub _size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut ::core::ffi::c_uchar,
    pub _r: ::core::ffi::c_int,
    pub _w: ::core::ffi::c_int,
    pub _flags: ::core::ffi::c_short,
    pub _file: ::core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::core::ffi::c_int,
    pub _cookie: *mut ::core::ffi::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, fpos_t, ::core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::core::ffi::c_int,
    pub _ubuf: [::core::ffi::c_uchar; 3],
    pub _nbuf: [::core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: ::core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
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
pub struct _subfont_entry {
    pub infix: *mut ::core::ffi::c_char,
    pub charcodes: [::core::ffi::c_long; 256],
    pub next: *mut subfont_entry,
}
pub type subfont_entry = _subfont_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfd_entry {
    pub name: *mut ::core::ffi::c_char,
    pub subfont: *mut subfont_entry,
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
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_SUBFONT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
static mut sfd_tree: *mut avl_table = ::core::ptr::null::<avl_table>() as *mut avl_table;
static mut sfd_file: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
static mut sfd_line: [::core::ffi::c_char; 256] = [0; 256];
unsafe extern "C" fn new_subfont_entry() -> *mut subfont_entry {
    let mut i: ::core::ffi::c_int = 0;
    let mut subfont: *mut subfont_entry = ::core::ptr::null_mut::<subfont_entry>();
    subfont = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<subfont_entry>() as size_t))
        as *mut subfont_entry;
    (*subfont).infix = ::core::ptr::null_mut::<::core::ffi::c_char>();
    i = 0 as ::core::ffi::c_int;
    while i < 256 as ::core::ffi::c_int {
        (*subfont).charcodes[i as usize] = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
        i += 1;
    }
    (*subfont).next = ::core::ptr::null_mut::<subfont_entry>();
    return subfont;
}
unsafe extern "C" fn new_sfd_entry() -> *mut sfd_entry {
    let mut sfd: *mut sfd_entry = ::core::ptr::null_mut::<sfd_entry>();
    sfd = xmalloc((1 as size_t).wrapping_mul(::core::mem::size_of::<sfd_entry>() as size_t))
        as *mut sfd_entry;
    (*sfd).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*sfd).subfont = ::core::ptr::null_mut::<subfont_entry>();
    return sfd;
}
unsafe extern "C" fn destroy_sfd_entry(
    mut pa: *mut ::core::ffi::c_void,
    mut pb: *mut ::core::ffi::c_void,
) {
    let mut p: *mut subfont_entry = ::core::ptr::null_mut::<subfont_entry>();
    let mut q: *mut subfont_entry = ::core::ptr::null_mut::<subfont_entry>();
    let mut sfd: *mut sfd_entry = ::core::ptr::null_mut::<sfd_entry>();
    sfd = pa as *mut sfd_entry;
    p = (*sfd).subfont;
    while !p.is_null() {
        q = (*p).next;
        if !(*p).infix.is_null() {
            free((*p).infix as *mut ::core::ffi::c_void);
        }
        (*p).infix = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if !p.is_null() {
            free(p as *mut ::core::ffi::c_void);
        }
        p = ::core::ptr::null_mut::<subfont_entry>();
        p = q;
    }
    if !(*sfd).name.is_null() {
        free((*sfd).name as *mut ::core::ffi::c_void);
    }
    (*sfd).name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn comp_sfd_entry(
    mut pa: *const ::core::ffi::c_void,
    mut pb: *const ::core::ffi::c_void,
    mut p: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return strcmp(
        (*(pa as *const sfd_entry)).name,
        (*(pb as *const sfd_entry)).name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sfd_free() {
    if !sfd_tree.is_null() {
        avl_destroy(
            sfd_tree,
            Some(
                destroy_sfd_entry
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> (),
            ),
        );
    }
}
unsafe extern "C" fn sfd_getline(mut expect_eof: boolean) {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c: ::core::ffi::c_int = 0;
    loop {
        if feof(sfd_file) != 0 {
            if expect_eof != 0 {
                return;
            } else {
                crate::utils::pdftex_fail_args(
                    b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
        }
        p = &raw mut sfd_line as *mut ::core::ffi::c_char;
        loop {
            c = fgetc(sfd_file);
            if c == 9 as ::core::ffi::c_int {
                c = 32 as ::core::ffi::c_int;
            }
            if c == 13 as ::core::ffi::c_int || c == EOF {
                c = 10 as ::core::ffi::c_int;
            }
            if c != ' ' as i32
                || p > &raw mut sfd_line as *mut ::core::ffi::c_char
                    && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        != 32 as ::core::ffi::c_int
            {
                if (p.offset_from(&raw mut sfd_line as *mut ::core::ffi::c_char)
                    as ::core::ffi::c_long
                    + 1 as ::core::ffi::c_long) as ::core::ffi::c_uint
                    > 256 as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    crate::utils::pdftex_fail_args(
                        b"buffer overflow at file %s, line %d\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(
                                b"pdftex-rust/generated/backend/subfont.rs\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                            crate::utils::PrintfArg::from(95 as ::core::ffi::c_int),
                        ],
                    );
                }
                let fresh3 = p;
                p = p.offset(1);
                *fresh3 = c as ::core::ffi::c_char;
            }
            if !(c != 10 as ::core::ffi::c_int) {
                break;
            }
        }
        if (p.offset_from(&raw mut sfd_line as *mut ::core::ffi::c_char) as ::core::ffi::c_long
            + 2 as ::core::ffi::c_long) as ::core::ffi::c_uint
            > 256 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(
                b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
                &[
                    crate::utils::PrintfArg::from(
                        b"pdftex-rust/generated/backend/subfont.rs\0" as *const u8
                            as *const ::core::ffi::c_char,
                    ),
                    crate::utils::PrintfArg::from(97 as ::core::ffi::c_int),
                ],
            );
        }
        if p.offset_from(&raw mut sfd_line as *mut ::core::ffi::c_char) as ::core::ffi::c_long
            > 1 as ::core::ffi::c_long
            && *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                != 10 as ::core::ffi::c_int
        {
            let fresh4 = p;
            p = p.offset(1);
            *fresh4 = 10 as ::core::ffi::c_char;
        }
        if p.offset_from(&raw mut sfd_line as *mut ::core::ffi::c_char) as ::core::ffi::c_long
            > 2 as ::core::ffi::c_long
            && *p.offset(-(2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                == 32 as ::core::ffi::c_int
        {
            *p.offset(-(2 as ::core::ffi::c_int) as isize) = 10 as ::core::ffi::c_char;
            p = p.offset(-1);
        }
        *p = 0 as ::core::ffi::c_char;
        if !((p.offset_from(&raw mut sfd_line as *mut ::core::ffi::c_char) as ::core::ffi::c_long)
            < 2 as ::core::ffi::c_long
            || *(&raw mut sfd_line as *mut ::core::ffi::c_char) as ::core::ffi::c_int == '#' as i32)
        {
            break;
        }
    }
}
unsafe extern "C" fn read_sfd(mut sfd_name: *mut ::core::ffi::c_char) -> *mut sfd_entry {
    let mut aa: *mut *mut ::core::ffi::c_void = ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
    let mut sfd: *mut sfd_entry = ::core::ptr::null_mut::<sfd_entry>();
    let mut tmp_sfd: sfd_entry = sfd_entry {
        name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        subfont: ::core::ptr::null_mut::<subfont_entry>(),
    };
    let mut sf: *mut subfont_entry = ::core::ptr::null_mut::<subfont_entry>();
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_long = 0;
    let mut j: ::core::ffi::c_long = 0;
    let mut k: ::core::ffi::c_long = 0;
    let mut n: ::core::ffi::c_int = 0;
    tmp_sfd.name = sfd_name;
    if sfd_tree.is_null() {
        sfd_tree = avl_create(
            Some(
                comp_sfd_entry
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
            &raw mut avl_xallocator,
        );
        if sfd_tree.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"read_sfd\0" as *const u8 as *const ::core::ffi::c_char,
                b"subfont.c\0" as *const u8 as *const ::core::ffi::c_char,
                114 as ::core::ffi::c_int,
                b"sfd_tree != NULL\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
    }
    sfd = avl_find(sfd_tree, &raw mut tmp_sfd as *const ::core::ffi::c_void) as *mut sfd_entry;
    if !sfd.is_null() {
        return sfd;
    }
    cur_file_name = sfd_name;
    zpackfilename(maketexstring(cur_file_name), getnullstr(), getnullstr());
    if open_input(
        &raw mut sfd_file,
        kpse_sfd_format as ::core::ffi::c_int,
        FOPEN_RBIN_MODE.as_ptr(),
    ) == 0
    {
        crate::utils::pdftex_warn_args(
            b"cannot open SFD file for reading\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        cur_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return ::core::ptr::null_mut::<sfd_entry>();
    }
    crate::utils::tex_printf_args(b"{\0" as *const u8 as *const ::core::ffi::c_char, &[]);
    crate::utils::tex_printf_args(
        b"%s\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(cur_file_name)],
    );
    sfd = new_sfd_entry();
    (*sfd).name = xstrdup(sfd_name as const_string) as *mut ::core::ffi::c_char;
    while feof(sfd_file) == 0 {
        sfd_getline(true_0);
        if *(&raw mut sfd_line as *mut ::core::ffi::c_char) as ::core::ffi::c_int
            == 10 as ::core::ffi::c_int
        {
            break;
        }
        sf = new_subfont_entry();
        (*sf).next = (*sfd).subfont;
        (*sfd).subfont = sf;
        sscanf(
            &raw mut sfd_line as *mut ::core::ffi::c_char,
            b"%s %n\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
            &raw mut n,
        );
        (*sf).infix = xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
            as *mut ::core::ffi::c_char;
        p = (&raw mut sfd_line as *mut ::core::ffi::c_char).offset(n as isize);
        k = 0 as ::core::ffi::c_long;
        loop {
            if *p as ::core::ffi::c_int == '\\' as i32 {
                sfd_getline(false_0);
                p = &raw mut sfd_line as *mut ::core::ffi::c_char;
            } else {
                if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    break;
                }
                if sscanf(
                    p,
                    b" %li %n\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut i,
                    &raw mut n,
                ) == 0 as ::core::ffi::c_int
                {
                    crate::utils::pdftex_fail_args(
                        b"invalid token:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                        &[crate::utils::PrintfArg::from(p)],
                    );
                }
                p = p.offset(n as isize);
                if *p as ::core::ffi::c_int == ':' as i32 {
                    k = i;
                    p = p.offset(1);
                } else if *p as ::core::ffi::c_int == '_' as i32 {
                    if sscanf(
                        p.offset(1 as ::core::ffi::c_int as isize),
                        b" %li %n\0" as *const u8 as *const ::core::ffi::c_char,
                        &raw mut j,
                        &raw mut n,
                    ) == 0 as ::core::ffi::c_int
                    {
                        crate::utils::pdftex_fail_args(
                            b"invalid token:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from(p)],
                        );
                    }
                    if i > j || k + (j - i) > 255 as ::core::ffi::c_long {
                        crate::utils::pdftex_fail_args(
                            b"invalid range:\n%s\0" as *const u8 as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from(p)],
                        );
                    }
                    while i <= j {
                        let fresh0 = i;
                        i = i + 1;
                        let fresh1 = k;
                        k = k + 1;
                        (*sf).charcodes[fresh1 as usize] = fresh0;
                    }
                    p = p.offset((n + 1 as ::core::ffi::c_int) as isize);
                } else {
                    let fresh2 = k;
                    k = k + 1;
                    (*sf).charcodes[fresh2 as usize] = i;
                }
            }
        }
    }
    xfclose(sfd_file, cur_file_name as const_string);
    crate::utils::tex_printf_args(b"}\0" as *const u8 as *const ::core::ffi::c_char, &[]);
    aa = avl_probe(sfd_tree, sfd as *mut ::core::ffi::c_void);
    if aa.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"read_sfd\0" as *const u8 as *const ::core::ffi::c_char,
            b"subfont.c\0" as *const u8 as *const ::core::ffi::c_char,
            169 as ::core::ffi::c_int,
            b"aa != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    return sfd;
}
#[no_mangle]
pub unsafe extern "C" fn handle_subfont_fm(
    mut fm: *mut fm_entry,
    mut mode: ::core::ffi::c_int,
) -> boolean {
    let mut l: size_t = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sfd: *mut sfd_entry = ::core::ptr::null_mut::<sfd_entry>();
    let mut sf: *mut subfont_entry = ::core::ptr::null_mut::<subfont_entry>();
    let mut fm2: *mut fm_entry = ::core::ptr::null_mut::<fm_entry>();
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    if (*fm).tfm_name.is_null() as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"handle_subfont_fm\0" as *const u8 as *const ::core::ffi::c_char,
            b"subfont.c\0" as *const u8 as *const ::core::ffi::c_char,
            181 as ::core::ffi::c_int,
            b"fm->tfm_name != NULL\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    p = (*fm).tfm_name;
    q = strchr(p, '@' as i32);
    if q.is_null() {
        return false_0;
    }
    r = strchr(q.offset(1 as ::core::ffi::c_int as isize), '@' as i32);
    if r.is_null() {
        return false_0;
    }
    if q <= p
        || r <= q.offset(1 as ::core::ffi::c_int as isize)
        || r.offset_from(p) as ::core::ffi::c_long as size_t != strlen(p).wrapping_sub(1 as size_t)
    {
        return false_0;
    }
    l = r.offset_from(q.offset(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_long as size_t;
    strncpy(
        &raw mut buf as *mut ::core::ffi::c_char,
        q.offset(1 as ::core::ffi::c_int as isize),
        l,
    );
    buf[l as usize] = 0 as ::core::ffi::c_char;
    if strlen(&raw mut buf as *mut ::core::ffi::c_char).wrapping_add(4 as size_t)
        as ::core::ffi::c_uint
        > 256 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        crate::utils::pdftex_fail_args(
            b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(
                    b"pdftex-rust/generated/backend/subfont.rs\0" as *const u8
                        as *const ::core::ffi::c_char,
                ),
                crate::utils::PrintfArg::from(196 as ::core::ffi::c_int),
            ],
        );
    }
    strcat(
        &raw mut buf as *mut ::core::ffi::c_char,
        b".sfd\0" as *const u8 as *const ::core::ffi::c_char,
    );
    sfd = read_sfd(&raw mut buf as *mut ::core::ffi::c_char);
    if sfd.is_null() {
        return false_0;
    }
    (*fm).type_0 = ((*fm).type_0 as ::core::ffi::c_int | F_SUBFONT) as ::core::ffi::c_ushort;
    if !(*fm).ps_name.is_null() {
        free((*fm).ps_name as *mut ::core::ffi::c_void);
    }
    (*fm).ps_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*fm).ps_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if (*fm).pid as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
        (*fm).pid = 3 as ::core::ffi::c_short;
        (*fm).eid = 1 as ::core::ffi::c_short;
    }
    l = q.offset_from(p) as ::core::ffi::c_long as size_t;
    sf = (*sfd).subfont;
    while !sf.is_null() {
        strncpy(&raw mut buf as *mut ::core::ffi::c_char, p, l);
        buf[l as usize] = 0 as ::core::ffi::c_char;
        strcat(&raw mut buf as *mut ::core::ffi::c_char, (*sf).infix);
        fm2 = new_fm_entry();
        (*fm2).tfm_name = xstrdup(&raw mut buf as *mut ::core::ffi::c_char as const_string)
            as *mut ::core::ffi::c_char;
        (*fm2).ff_name = xstrdup((*fm).ff_name as const_string) as *mut ::core::ffi::c_char;
        (*fm2).type_0 = (*fm).type_0;
        (*fm2).pid = (*fm).pid;
        (*fm2).eid = (*fm).eid;
        (*fm2).subfont = sf;
        if avl_do_entry(fm2, mode) != 0 as ::core::ffi::c_int {
            delete_fm_entry(fm2);
        }
        sf = (*sf).next;
    }
    delete_fm_entry(fm);
    return true_0;
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

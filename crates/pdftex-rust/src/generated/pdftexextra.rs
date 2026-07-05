#[repr(C)]
pub struct __sFILEX {
    _unused: [u8; 0],
}

extern "C" {
    static mut __stderrp: *mut FILE;
    fn fclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(_: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __mode: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputs(_: *const ::core::ffi::c_char, _: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __nitems: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(_: *mut FILE, _: ::core::ffi::c_long, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fgetc(_: *mut FILE) -> ::core::ffi::c_int;
    fn setvbuf(
        _: *mut FILE,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        __size: size_t,
    ) -> ::core::ffi::c_int;
    fn sprintf(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn ungetc(_: ::core::ffi::c_int, _: *mut FILE) -> ::core::ffi::c_int;
    fn pclose(_: *mut FILE) -> ::core::ffi::c_int;
    fn popen(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char) -> *mut FILE;
    fn snprintf(
        __str: *mut ::core::ffi::c_char,
        __size: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn access(_: *const ::core::ffi::c_char, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn close(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sleep(_: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn write(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __nbyte: size_t,
    ) -> ssize_t;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn signal(
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    ) -> Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
    fn free(_: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn atoi(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn exit(_: ::core::ffi::c_int) -> !;
    fn getenv(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strtol(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strtoull(
        __str: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulonglong;
    fn system(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn memcpy(
        __dst: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    fn strcpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strncpy(
        __dst: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strtok(
        __str: *mut ::core::ffi::c_char,
        __sep: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn __error() -> *mut ::core::ffi::c_int;
    fn __assert_rtn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> !;
    static mut kpse_def_inst: kpathsea_instance;
    static mut kpse_def: kpathsea;
    fn concat(s1: const_string, s2: const_string) -> string;
    fn concat3(_: const_string, _: const_string, _: const_string) -> string;
    fn xstrdup(s: const_string) -> string;
    fn find_suffix(name: const_string) -> const_string;
    fn xputenv(var: const_string, value: const_string);
    fn xgetcwd() -> string;
    fn dir_p(fn_0: string) -> boolean;
    fn xfopen(filename: const_string, mode: const_string) -> *mut FILE;
    fn xfclose(fp: *mut FILE, filename: const_string);
    fn xmalloc(size: size_t) -> address;
    fn xrealloc(old_address: address, new_size: size_t) -> address;
    fn kpse_set_program_name(argv0: const_string, progname: const_string);
    fn fcntl(_: ::core::ffi::c_int, _: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn uexit(status: ::core::ffi::c_int) -> !;
    fn usagehelp(message: *mut const_string, bug_email: const_string);
    fn getopt_long_only(
        argc_0: ::core::ffi::c_int,
        argv_0: *const *mut ::core::ffi::c_char,
        shortopts: *const ::core::ffi::c_char,
        longopts: *const option,
        longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn kpse_set_program_enabled(fmt: kpse_file_format_type, value: boolean, level: kpse_src_type);
    fn kpse_maketex_option(fmtname: const_string, value: boolean);
    fn kpse_find_file(
        name: const_string,
        format: kpse_file_format_type,
        must_exist: boolean,
    ) -> string;
    fn kpse_in_name_ok(fname: const_string) -> boolean;
    fn kpse_reset_program_name(progname: const_string);
    fn kpse_var_value(var: const_string) -> string;
    fn mainbody();
    fn open_input(_: *mut *mut FILE, _: ::core::ffi::c_int, fopen_mode: const_string) -> boolean;
    fn open_output(_: *mut *mut FILE, fopen_mode: const_string) -> boolean;
    fn close_file(_: *mut FILE);
    fn recorder_record_input(_: const_string);
    fn recorder_record_output(_: const_string);
    static mut fullnameoffile: string;
    static mut recorder_enabled: boolean;
    static mut output_directory: string;
    fn printversionandexit(_: const_string, _: const_string, _: const_string, _: const_string);
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __maskrune(_: __darwin_ct_rune_t, _: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    fn gzread(file: gzFile, buf: voidp, len: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn gzwrite(file: gzFile, buf: voidpc, len: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn maketexstring(_: *const ::core::ffi::c_char) -> strnumber;
    fn initversionstring(versions: *mut *mut ::core::ffi::c_char);
    fn kpathsea_cnf_line_env_progname(kpse: kpathsea, l_0: string);
    fn read_line(f_0: *mut FILE) -> string;
    fn kpse_readable_file(name: string) -> string;
    fn kpse_absolute_p(filename: const_string, relative_ok: boolean) -> boolean;
    fn gmtime(_: *const time_t) -> *mut tm;
    fn localtime(_: *const time_t) -> *mut tm;
    fn strftime(
        _: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        _: *const ::core::ffi::c_char,
        _: *const tm,
    ) -> size_t;
    fn time(_: *mut time_t) -> time_t;
    fn gettimeofday(_: *mut timeval, _: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn connect(_: ::core::ffi::c_int, _: *const sockaddr, _: socklen_t) -> ::core::ffi::c_int;
    fn socket(
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn stat(_: *const ::core::ffi::c_char, _: *mut stat) -> ::core::ffi::c_int;
    fn md5_init(pms: *mut md5_state_t);
    fn md5_append(pms: *mut md5_state_t, data: *const md5_byte_t, nbytes: ::core::ffi::c_int);
    fn md5_finish(pms: *mut md5_state_t, digest: *mut md5_byte_t);
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __darwin_ct_rune_t = ::core::ffi::c_int;
pub type __darwin_size_t = usize;
pub type __darwin_wchar_t = ::libc::wchar_t;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = isize;
pub type __darwin_time_t = ::core::ffi::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
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
pub type off_t = __darwin_off_t;
pub type ssize_t = __darwin_ssize_t;
pub type dev_t = __darwin_dev_t;
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type mode_t = __darwin_mode_t;
pub type nlink_t = __uint16_t;
pub type uid_t = __darwin_uid_t;
pub type time_t = __darwin_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}
pub type boolean = ::core::ffi::c_int;
pub type string = *mut ::core::ffi::c_char;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type p_record_input = Option<unsafe extern "C" fn(const_string) -> ()>;
pub type p_record_output = Option<unsafe extern "C" fn(const_string) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct str_llist_elt {
    pub str_0: string,
    pub moved: boolean,
    pub next: *mut str_llist_elt,
}
pub type str_llist_type = *mut str_llist_elt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_entry {
    pub key: const_string,
    pub value: *mut str_llist_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expansion_type {
    pub var: const_string,
    pub expanding: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_element_struct {
    pub key: const_string,
    pub value: const_string,
    pub next: *mut hash_element_struct,
}
pub type hash_element_type = hash_element_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table_type {
    pub buckets: *mut *mut hash_element_type,
    pub size: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct str_list_type {
    pub length: ::core::ffi::c_uint,
    pub list: *mut string,
}
pub type kpse_file_format_type = ::core::ffi::c_uint;
pub const kpse_last_format: kpse_file_format_type = 59;
pub const kpse_bltxml_format: kpse_file_format_type = 58;
pub const kpse_ris_format: kpse_file_format_type = 57;
pub const kpse_clua_format: kpse_file_format_type = 56;
pub const kpse_mlbst_format: kpse_file_format_type = 55;
pub const kpse_mlbib_format: kpse_file_format_type = 54;
pub const kpse_cid_format: kpse_file_format_type = 53;
pub const kpse_fea_format: kpse_file_format_type = 52;
pub const kpse_lua_format: kpse_file_format_type = 51;
pub const kpse_texmfscripts_format: kpse_file_format_type = 50;
pub const kpse_lig_format: kpse_file_format_type = 49;
pub const kpse_pdftex_config_format: kpse_file_format_type = 48;
pub const kpse_opentype_format: kpse_file_format_type = 47;
pub const kpse_sfd_format: kpse_file_format_type = 46;
pub const kpse_cmap_format: kpse_file_format_type = 45;
pub const kpse_enc_format: kpse_file_format_type = 44;
pub const kpse_cweb_format: kpse_file_format_type = 43;
pub const kpse_web_format: kpse_file_format_type = 42;
pub const kpse_miscfonts_format: kpse_file_format_type = 41;
pub const kpse_program_binary_format: kpse_file_format_type = 40;
pub const kpse_program_text_format: kpse_file_format_type = 39;
pub const kpse_web2c_format: kpse_file_format_type = 38;
pub const kpse_type42_format: kpse_file_format_type = 37;
pub const kpse_truetype_format: kpse_file_format_type = 36;
pub const kpse_ist_format: kpse_file_format_type = 35;
pub const kpse_dvips_config_format: kpse_file_format_type = 34;
pub const kpse_vf_format: kpse_file_format_type = 33;
pub const kpse_type1_format: kpse_file_format_type = 32;
pub const kpse_troff_font_format: kpse_file_format_type = 31;
pub const kpse_tex_ps_header_format: kpse_file_format_type = 30;
pub const kpse_texsource_format: kpse_file_format_type = 29;
pub const kpse_texpool_format: kpse_file_format_type = 28;
pub const kpse_texdoc_format: kpse_file_format_type = 27;
pub const kpse_tex_format: kpse_file_format_type = 26;
pub const kpse_pict_format: kpse_file_format_type = 25;
pub const kpse_ovp_format: kpse_file_format_type = 24;
pub const kpse_ovf_format: kpse_file_format_type = 23;
pub const kpse_otp_format: kpse_file_format_type = 22;
pub const kpse_opl_format: kpse_file_format_type = 21;
pub const kpse_ofm_format: kpse_file_format_type = 20;
pub const kpse_ocp_format: kpse_file_format_type = 19;
pub const kpse_mpsupport_format: kpse_file_format_type = 18;
pub const kpse_mppool_format: kpse_file_format_type = 17;
pub const kpse_mp_format: kpse_file_format_type = 16;
pub const kpse_mft_format: kpse_file_format_type = 15;
pub const kpse_mfpool_format: kpse_file_format_type = 14;
pub const kpse_mf_format: kpse_file_format_type = 13;
pub const kpse_mem_format: kpse_file_format_type = 12;
pub const kpse_fontmap_format: kpse_file_format_type = 11;
pub const kpse_fmt_format: kpse_file_format_type = 10;
pub const kpse_db_format: kpse_file_format_type = 9;
pub const kpse_cnf_format: kpse_file_format_type = 8;
pub const kpse_bst_format: kpse_file_format_type = 7;
pub const kpse_bib_format: kpse_file_format_type = 6;
pub const kpse_base_format: kpse_file_format_type = 5;
pub const kpse_afm_format: kpse_file_format_type = 4;
pub const kpse_tfm_format: kpse_file_format_type = 3;
pub const kpse_any_glyph_format: kpse_file_format_type = 2;
pub const kpse_pk_format: kpse_file_format_type = 1;
pub const kpse_gf_format: kpse_file_format_type = 0;
pub type kpse_src_type = ::core::ffi::c_uint;
pub const kpse_src_cmdline: kpse_src_type = 6;
pub const kpse_src_x: kpse_src_type = 5;
pub const kpse_src_env: kpse_src_type = 4;
pub const kpse_src_client_cnf: kpse_src_type = 3;
pub const kpse_src_texmf_cnf: kpse_src_type = 2;
pub const kpse_src_compile: kpse_src_type = 1;
pub const kpse_src_implicit: kpse_src_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kpse_format_info_type {
    pub type_0: const_string,
    pub path: string,
    pub raw_path: const_string,
    pub path_source: const_string,
    pub override_path: const_string,
    pub client_path: const_string,
    pub cnf_path: const_string,
    pub default_path: const_string,
    pub suffix: *mut const_string,
    pub alt_suffix: *mut const_string,
    pub suffix_search_only: boolean,
    pub program: const_string,
    pub argc: ::core::ffi::c_int,
    pub argv: *mut const_string,
    pub program_enabled_p: boolean,
    pub program_enable_level: kpse_src_type,
    pub binmode: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kpathsea_instance {
    pub record_input: p_record_input,
    pub record_output: p_record_output,
    pub cnf_hash: hash_table_type,
    pub doing_cnf_init: boolean,
    pub db: hash_table_type,
    pub alias_db: hash_table_type,
    pub db_dir_list: str_list_type,
    pub debug: ::core::ffi::c_uint,
    pub link_table: hash_table_type,
    pub the_cache: *mut cache_entry,
    pub cache_length: ::core::ffi::c_uint,
    pub map: hash_table_type,
    pub map_path: const_string,
    pub debug_hash_lookup_int: boolean,
    pub elt: string,
    pub elt_alloc: ::core::ffi::c_uint,
    pub path: const_string,
    pub followup_search: boolean,
    pub log_file: *mut FILE,
    pub log_opened: boolean,
    pub invocation_name: string,
    pub invocation_short_name: string,
    pub program_name: string,
    pub ll_verbose: ::core::ffi::c_int,
    pub fallback_font: const_string,
    pub fallback_resolutions_string: const_string,
    pub fallback_resolutions: *mut ::core::ffi::c_uint,
    pub format_info: [kpse_format_info_type; 59],
    pub make_tex_discard_errors: boolean,
    pub missfont: *mut FILE,
    pub expansions: *mut expansion_type,
    pub expansion_len: ::core::ffi::c_uint,
    pub saved_env: *mut *mut ::core::ffi::c_char,
    pub saved_count: ::core::ffi::c_int,
}
pub type kpathsea = *mut kpathsea_instance;
pub type schar = ::core::ffi::c_schar;
pub type integer = ::core::ffi::c_int;
pub type longinteger = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type real = ::core::ffi::c_double;
pub type text = *mut FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: ::core::ffi::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [::core::ffi::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [::core::ffi::c_char; 8],
    pub __encoding: [::core::ffi::c_char; 32],
    pub __sgetrune: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_char,
            __darwin_size_t,
            *mut *const ::core::ffi::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut ::core::ffi::c_char,
            __darwin_size_t,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut ::core::ffi::c_void,
    pub __variable_len: ::core::ffi::c_int,
    pub __ncharclasses: ::core::ffi::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *mut ::core::ffi::c_char,
}
pub type packedASCIIcode = ::core::ffi::c_uchar;
pub type poolpointer = integer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
pub type strnumber = integer;
pub type md5_byte_t = ::core::ffi::c_uchar;
pub type md5_state_t = md5_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_state_s {
    pub count: [md5_word_t; 2],
    pub abcd: [md5_word_t; 4],
    pub buf: [md5_byte_t; 64],
}
pub type md5_word_t = ::core::ffi::c_uint;
pub type ASCIIcode = ::core::ffi::c_uchar;
pub type socklen_t = __darwin_socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_len: __uint8_t,
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
pub type sa_family_t = __uint8_t;
pub type glueratio = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub msg: msg,
    pub more_data: [::core::ffi::c_char; 1024],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msg {
    pub namelength: ::core::ffi::c_int,
    pub eof: ::core::ffi::c_int,
}
pub type voidpc = *const ::core::ffi::c_void;
pub type voidp = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: ::core::ffi::c_uint,
    pub next: *mut ::core::ffi::c_uchar,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
pub type eightbits = ::core::ffi::c_uchar;
pub type alphafile = text;
pub type bytefile = text;
pub type scaled = integer;
pub type smallnumber = ::core::ffi::c_uchar;
pub type quarterword = ::core::ffi::c_uchar;
pub type halfword = integer;
#[derive(Copy, Clone)]
#[repr(C)]
pub union twohalves {
    pub v: C2RustUnnamed_1,
    pub u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub B1: ::core::ffi::c_short,
    pub B0: ::core::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub LH: halfword,
    pub RH: halfword,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fourquarters {
    pub u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub B3: quarterword,
    pub B2: quarterword,
    pub B1: quarterword,
    pub B0: quarterword,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union memoryword {
    pub gr: glueratio,
    pub hh: twohalves,
    pub u: C2RustUnnamed_4,
    pub v: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub junk: halfword,
    pub QQQQ: fourquarters,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub junk: halfword,
    pub CINT: integer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union fmemoryword {
    pub u: C2RustUnnamed_6,
    pub v: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub QQQQ: fourquarters,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub CINT: integer,
}
pub type wordfile = gzFile;
pub type glueord = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct liststaterecord {
    pub modefield: ::core::ffi::c_short,
    pub headfield: halfword,
    pub tailfield: halfword,
    pub eTeXauxfield: halfword,
    pub pgfield: integer,
    pub mlfield: integer,
    pub auxfield: memoryword,
}
pub type groupcode = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instaterecord {
    pub statefield: quarterword,
    pub indexfield: quarterword,
    pub startfield: halfword,
    pub locfield: halfword,
    pub limitfield: halfword,
    pub namefield: halfword,
    pub synctextagfield: integer,
}
pub type internalfontnumber = integer;
pub type fontindex = integer;
pub type ninebits = ::core::ffi::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct objentry {
    pub int0: integer,
    pub int1: integer,
    pub int2: longinteger,
    pub int3: integer,
    pub int4: integer,
}
pub type charusedarray = [eightbits; 32];
pub type fmentryptr = *mut integer;
pub type vfstackindex = integer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vfstackrecord {
    pub stackh: scaled,
    pub stackv: scaled,
    pub stackw: scaled,
    pub stackx: scaled,
    pub stacky: scaled,
    pub stackz: scaled,
}
pub type triepointer = integer;
pub type trieopcode = ::core::ffi::c_ushort;
pub type hyphpointer = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct destnameentry {
    pub objname: strnumber,
    pub objnum: integer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdflinkstackrecord {
    pub nestinglevel: integer,
    pub linknode: halfword,
    pub reflinknode: halfword,
}
pub type savepointer = integer;
pub const MAKE_TEX_FMT_BY_DEFAULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MAKE_TEX_TEX_BY_DEFAULT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MAKE_TEX_TFM_BY_DEFAULT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const _IONBF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const R_OK: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIG_DFL: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()> = None;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EDITOR: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"vi +%d '%s'\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FOPEN_R_MODE: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"r\0") };
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const O_NONBLOCK: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const DIR_SEP: ::core::ffi::c_int = '/' as i32;
pub const DIR_SEP_STRING: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0") };
pub const _CTYPE_S: ::core::ffi::c_long = 0x4000 as ::core::ffi::c_long;
unsafe fn isascii(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (_c & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe fn __istype(
    mut _c: __darwin_ct_rune_t,
    mut _f: ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    return if isascii(_c as ::core::ffi::c_int) != 0 {
        (_DefaultRuneLocale.__runetype[_c as usize] as ::core::ffi::c_ulong & _f != 0)
            as ::core::ffi::c_int
    } else {
        (__maskrune(_c, _f) != 0) as ::core::ffi::c_int
    };
}
unsafe fn isspace(mut _c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return __istype(_c as __darwin_ct_rune_t, _CTYPE_S as ::core::ffi::c_ulong);
}
pub const TEXMFENGINENAME: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"pdftex\0") };
#[no_mangle]
pub static mut bad: integer = 0;
#[no_mangle]
pub static mut xord: [ASCIIcode; 256] = [0; 256];
#[no_mangle]
pub static mut xchr: [ASCIIcode; 256] = [0; 256];
#[no_mangle]
pub static mut xprn: [ASCIIcode; 256] = [0; 256];
#[no_mangle]
pub static mut mubyteread: [halfword; 256] = [0; 256];
#[no_mangle]
pub static mut mubytewrite: [strnumber; 256] = [0; 256];
#[no_mangle]
pub static mut mubytecswrite: [halfword; 128] = [0; 128];
#[no_mangle]
pub static mut mubyteskip: integer = 0;
#[no_mangle]
pub static mut mubytekeep: integer = 0;
#[no_mangle]
pub static mut mubyteskeep: integer = 0;
#[no_mangle]
pub static mut mubyteprefix: integer = 0;
#[no_mangle]
pub static mut mubytetablein: boolean = 0;
#[no_mangle]
pub static mut mubytetableout: boolean = 0;
#[no_mangle]
pub static mut mubyterelax: boolean = 0;
#[no_mangle]
pub static mut mubytestart: boolean = 0;
#[no_mangle]
pub static mut mubytesstart: boolean = 0;
#[no_mangle]
pub static mut mubytetoken: halfword = 0;
#[no_mangle]
pub static mut mubytestoken: halfword = 0;
#[no_mangle]
pub static mut mubytesout: integer = 0;
#[no_mangle]
pub static mut mubyteslog: integer = 0;
#[no_mangle]
pub static mut specsout: integer = 0;
#[no_mangle]
pub static mut noconvert: boolean = 0;
#[no_mangle]
pub static mut activenoconvert: boolean = 0;
#[no_mangle]
pub static mut writenoexpanding: boolean = 0;
#[no_mangle]
pub static mut csconverting: boolean = 0;
#[no_mangle]
pub static mut specialprinting: boolean = 0;
#[no_mangle]
pub static mut messageprinting: boolean = 0;
#[no_mangle]
pub static mut nameoffile: *mut ASCIIcode = ::core::ptr::null::<ASCIIcode>() as *mut ASCIIcode;
#[no_mangle]
pub static mut namelength: integer = 0;
#[no_mangle]
pub static mut buffer: *mut ASCIIcode = ::core::ptr::null::<ASCIIcode>() as *mut ASCIIcode;
#[no_mangle]
pub static mut first: integer = 0;
#[no_mangle]
pub static mut last: integer = 0;
#[no_mangle]
pub static mut maxbufstack: integer = 0;
#[no_mangle]
pub static mut iniversion: boolean = 0;
#[no_mangle]
pub static mut dumpoption: boolean = 0;
#[no_mangle]
pub static mut dumpline: boolean = 0;
#[no_mangle]
pub static mut dump_name: const_string = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub static mut bounddefault: integer = 0;
#[no_mangle]
pub static mut boundname: const_string = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub static mut membot: integer = 0;
#[no_mangle]
pub static mut mainmemory: integer = 0;
#[no_mangle]
pub static mut extramembot: integer = 0;
#[no_mangle]
pub static mut memmin: integer = 0;
#[no_mangle]
pub static mut memtop: integer = 0;
#[no_mangle]
pub static mut extramemtop: integer = 0;
#[no_mangle]
pub static mut memmax: integer = 0;
#[no_mangle]
pub static mut errorline: integer = 0;
#[no_mangle]
pub static mut halferrorline: integer = 0;
#[no_mangle]
pub static mut maxprintline: integer = 0;
#[no_mangle]
pub static mut maxstrings: integer = 0;
#[no_mangle]
pub static mut stringsfree: integer = 0;
#[no_mangle]
pub static mut stringvacancies: integer = 0;
#[no_mangle]
pub static mut poolsize: integer = 0;
#[no_mangle]
pub static mut poolfree: integer = 0;
#[no_mangle]
pub static mut fontmemsize: integer = 0;
#[no_mangle]
pub static mut fontmax: integer = 0;
#[no_mangle]
pub static mut fontk: integer = 0;
#[no_mangle]
pub static mut hyphsize: integer = 0;
#[no_mangle]
pub static mut triesize: integer = 0;
#[no_mangle]
pub static mut bufsize: integer = 0;
#[no_mangle]
pub static mut stacksize: integer = 0;
#[no_mangle]
pub static mut maxinopen: integer = 0;
#[no_mangle]
pub static mut paramsize: integer = 0;
#[no_mangle]
pub static mut nestsize: integer = 0;
#[no_mangle]
pub static mut savesize: integer = 0;
#[no_mangle]
pub static mut dvibufsize: integer = 0;
#[no_mangle]
pub static mut expanddepth: integer = 0;
#[no_mangle]
pub static mut parsefirstlinep: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut filelineerrorstylep: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut eightbitp: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut haltonerrorp: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut haltingonerrorp: boolean = 0;
#[no_mangle]
pub static mut quotedfilename: boolean = 0;
#[no_mangle]
pub static mut srcspecialsp: boolean = 0;
#[no_mangle]
pub static mut insertsrcspecialauto: boolean = 0;
#[no_mangle]
pub static mut insertsrcspecialeverypar: boolean = 0;
#[no_mangle]
pub static mut insertsrcspecialeveryparend: boolean = 0;
#[no_mangle]
pub static mut insertsrcspecialeverycr: boolean = 0;
#[no_mangle]
pub static mut insertsrcspecialeverymath: boolean = 0;
#[no_mangle]
pub static mut insertsrcspecialeveryhbox: boolean = 0;
#[no_mangle]
pub static mut insertsrcspecialeveryvbox: boolean = 0;
#[no_mangle]
pub static mut insertsrcspecialeverydisplay: boolean = 0;
#[no_mangle]
pub static mut strpool: *mut packedASCIIcode =
    ::core::ptr::null::<packedASCIIcode>() as *mut packedASCIIcode;
#[no_mangle]
pub static mut strstart: *mut poolpointer = ::core::ptr::null::<poolpointer>() as *mut poolpointer;
#[no_mangle]
pub static mut poolptr: poolpointer = 0;
#[no_mangle]
pub static mut strptr: strnumber = 0;
#[no_mangle]
pub static mut initpoolptr: poolpointer = 0;
#[no_mangle]
pub static mut initstrptr: strnumber = 0;
#[no_mangle]
pub static mut poolfile: alphafile = ::core::ptr::null::<FILE>() as *mut FILE;
#[no_mangle]
pub static mut logfile: alphafile = ::core::ptr::null::<FILE>() as *mut FILE;
#[no_mangle]
pub static mut selector: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut dig: [::core::ffi::c_uchar; 23] = [0; 23];
#[no_mangle]
pub static mut tally: integer = 0;
#[no_mangle]
pub static mut termoffset: integer = 0;
#[no_mangle]
pub static mut fileoffset: integer = 0;
#[no_mangle]
pub static mut trickbuf: [ASCIIcode; 256] = [0; 256];
#[no_mangle]
pub static mut trickcount: integer = 0;
#[no_mangle]
pub static mut firstcount: integer = 0;
#[no_mangle]
pub static mut interaction: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut interactionoption: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut deletionsallowed: boolean = 0;
#[no_mangle]
pub static mut setboxallowed: boolean = 0;
#[no_mangle]
pub static mut history: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut errorcount: schar = 0;
#[no_mangle]
pub static mut helpline: [strnumber; 6] = [0; 6];
#[no_mangle]
pub static mut helpptr: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut useerrhelp: boolean = 0;
#[no_mangle]
pub static mut interrupt: integer = 0;
#[no_mangle]
pub static mut OKtointerrupt: boolean = 0;
#[no_mangle]
pub static mut savearitherror: boolean = 0;
#[no_mangle]
pub static mut aritherror: boolean = 0;
#[no_mangle]
pub static mut texremainder: scaled = 0;
#[no_mangle]
pub static mut randoms: [integer; 55] = [0; 55];
#[no_mangle]
pub static mut jrandom: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut randomseed: scaled = 0;
#[no_mangle]
pub static mut twotothe: [integer; 31] = [0; 31];
#[no_mangle]
pub static mut speclog: [integer; 29] = [0; 29];
#[no_mangle]
pub static mut tempptr: halfword = 0;
#[no_mangle]
pub static mut yzmem: *mut memoryword = ::core::ptr::null::<memoryword>() as *mut memoryword;
#[no_mangle]
pub static mut zmem: *mut memoryword = ::core::ptr::null::<memoryword>() as *mut memoryword;
#[no_mangle]
pub static mut lomemmax: halfword = 0;
#[no_mangle]
pub static mut himemmin: halfword = 0;
#[no_mangle]
pub static mut varused: integer = 0;
#[no_mangle]
pub static mut dynused: integer = 0;
#[no_mangle]
pub static mut avail: halfword = 0;
#[no_mangle]
pub static mut memend: halfword = 0;
#[no_mangle]
pub static mut rover: halfword = 0;
#[no_mangle]
pub static mut fontinshortdisplay: integer = 0;
#[no_mangle]
pub static mut depththreshold: integer = 0;
#[no_mangle]
pub static mut breadthmax: integer = 0;
#[no_mangle]
pub static mut nest: *mut liststaterecord =
    ::core::ptr::null::<liststaterecord>() as *mut liststaterecord;
#[no_mangle]
pub static mut nestptr: integer = 0;
#[no_mangle]
pub static mut maxneststack: integer = 0;
#[no_mangle]
pub static mut curlist: liststaterecord = liststaterecord {
    modefield: 0,
    headfield: 0,
    tailfield: 0,
    eTeXauxfield: 0,
    pgfield: 0,
    mlfield: 0,
    auxfield: memoryword { gr: 0. },
};
#[no_mangle]
pub static mut shownmode: ::core::ffi::c_short = 0;
#[no_mangle]
pub static mut savetail: halfword = 0;
#[no_mangle]
pub static mut prevtail: halfword = 0;
#[no_mangle]
pub static mut oldsetting: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut oldselectorignorederr: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut systime: integer = 0;
#[no_mangle]
pub static mut sysday: integer = 0;
#[no_mangle]
pub static mut sysmonth: integer = 0;
#[no_mangle]
pub static mut sysyear: integer = 0;
#[no_mangle]
pub static mut zeqtb: *mut memoryword = ::core::ptr::null::<memoryword>() as *mut memoryword;
#[no_mangle]
pub static mut zzzaa: [quarterword; 916] = [0; 916];
#[no_mangle]
pub static mut hash: *mut twohalves = ::core::ptr::null::<twohalves>() as *mut twohalves;
#[no_mangle]
pub static mut yhash: *mut twohalves = ::core::ptr::null::<twohalves>() as *mut twohalves;
#[no_mangle]
pub static mut hashused: halfword = 0;
#[no_mangle]
pub static mut hashextra: halfword = 0;
#[no_mangle]
pub static mut hashtop: halfword = 0;
#[no_mangle]
pub static mut eqtbtop: halfword = 0;
#[no_mangle]
pub static mut hashhigh: halfword = 0;
#[no_mangle]
pub static mut nonewcontrolsequence: boolean = 0;
#[no_mangle]
pub static mut cscount: integer = 0;
#[no_mangle]
pub static mut prim: [twohalves; 2101] = [twohalves {
    v: C2RustUnnamed_1 { LH: 0, RH: 0 },
}; 2101];
#[no_mangle]
pub static mut primused: halfword = 0;
#[no_mangle]
pub static mut savestack: *mut memoryword = ::core::ptr::null::<memoryword>() as *mut memoryword;
#[no_mangle]
pub static mut saveptr: integer = 0;
#[no_mangle]
pub static mut maxsavestack: integer = 0;
#[no_mangle]
pub static mut curlevel: quarterword = 0;
#[no_mangle]
pub static mut curgroup: groupcode = 0;
#[no_mangle]
pub static mut curboundary: integer = 0;
#[no_mangle]
pub static mut magset: integer = 0;
#[no_mangle]
pub static mut curcmd: eightbits = 0;
#[no_mangle]
pub static mut curchr: halfword = 0;
#[no_mangle]
pub static mut curcs: halfword = 0;
#[no_mangle]
pub static mut curtok: halfword = 0;
#[no_mangle]
pub static mut inputstack: *mut instaterecord =
    ::core::ptr::null::<instaterecord>() as *mut instaterecord;
#[no_mangle]
pub static mut inputptr: integer = 0;
#[no_mangle]
pub static mut maxinstack: integer = 0;
#[no_mangle]
pub static mut curinput: instaterecord = instaterecord {
    statefield: 0,
    indexfield: 0,
    startfield: 0,
    locfield: 0,
    limitfield: 0,
    namefield: 0,
    synctextagfield: 0,
};
#[no_mangle]
pub static mut inopen: integer = 0;
#[no_mangle]
pub static mut openparens: integer = 0;
#[no_mangle]
pub static mut inputfile: *mut alphafile = ::core::ptr::null::<alphafile>() as *mut alphafile;
#[no_mangle]
pub static mut line: integer = 0;
#[no_mangle]
pub static mut linestack: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut sourcefilenamestack: *mut strnumber =
    ::core::ptr::null::<strnumber>() as *mut strnumber;
#[no_mangle]
pub static mut fullsourcefilenamestack: *mut strnumber =
    ::core::ptr::null::<strnumber>() as *mut strnumber;
#[no_mangle]
pub static mut scannerstatus: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut warningindex: halfword = 0;
#[no_mangle]
pub static mut defref: halfword = 0;
#[no_mangle]
pub static mut paramstack: *mut halfword = ::core::ptr::null::<halfword>() as *mut halfword;
#[no_mangle]
pub static mut paramptr: integer = 0;
#[no_mangle]
pub static mut maxparamstack: integer = 0;
#[no_mangle]
pub static mut alignstate: integer = 0;
#[no_mangle]
pub static mut baseptr: integer = 0;
#[no_mangle]
pub static mut parloc: halfword = 0;
#[no_mangle]
pub static mut partoken: halfword = 0;
#[no_mangle]
pub static mut forceeof: boolean = 0;
#[no_mangle]
pub static mut isincsname: boolean = 0;
#[no_mangle]
pub static mut curmark: [halfword; 5] = [0; 5];
#[no_mangle]
pub static mut longstate: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut pstack: [halfword; 9] = [0; 9];
#[no_mangle]
pub static mut curval: integer = 0;
#[no_mangle]
pub static mut curvallevel: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut radix: smallnumber = 0;
#[no_mangle]
pub static mut curorder: glueord = 0;
#[no_mangle]
pub static mut readfile: [alphafile; 16] = [::core::ptr::null::<FILE>() as *mut FILE; 16];
#[no_mangle]
pub static mut readopen: [::core::ffi::c_uchar; 17] = [0; 17];
#[no_mangle]
pub static mut condptr: halfword = 0;
#[no_mangle]
pub static mut iflimit: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut curif: smallnumber = 0;
#[no_mangle]
pub static mut ifline: integer = 0;
#[no_mangle]
pub static mut skipline: integer = 0;
#[no_mangle]
pub static mut curname: strnumber = 0;
#[no_mangle]
pub static mut curarea: strnumber = 0;
#[no_mangle]
pub static mut curext: strnumber = 0;
#[no_mangle]
pub static mut areadelimiter: poolpointer = 0;
#[no_mangle]
pub static mut extdelimiter: poolpointer = 0;
#[no_mangle]
pub static mut formatdefaultlength: integer = 0;
#[no_mangle]
pub static mut TEXformatdefault: string =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut nameinprogress: boolean = 0;
#[no_mangle]
pub static mut jobname: strnumber = 0;
#[no_mangle]
pub static mut logopened: boolean = 0;
#[no_mangle]
pub static mut dvifile: bytefile = ::core::ptr::null::<FILE>() as *mut FILE;
#[no_mangle]
pub static mut outputfilename: strnumber = 0;
#[no_mangle]
pub static mut texmflogname: strnumber = 0;
#[no_mangle]
pub static mut tfmfile: bytefile = ::core::ptr::null::<FILE>() as *mut FILE;
#[no_mangle]
pub static mut fontinfo: *mut fmemoryword = ::core::ptr::null::<fmemoryword>() as *mut fmemoryword;
#[no_mangle]
pub static mut fmemptr: fontindex = 0;
#[no_mangle]
pub static mut fontptr: internalfontnumber = 0;
#[no_mangle]
pub static mut fontcheck: *mut fourquarters =
    ::core::ptr::null::<fourquarters>() as *mut fourquarters;
#[no_mangle]
pub static mut fontsize: *mut scaled = ::core::ptr::null::<scaled>() as *mut scaled;
#[no_mangle]
pub static mut fontdsize: *mut scaled = ::core::ptr::null::<scaled>() as *mut scaled;
#[no_mangle]
pub static mut fontparams: *mut fontindex = ::core::ptr::null::<fontindex>() as *mut fontindex;
#[no_mangle]
pub static mut fontname: *mut strnumber = ::core::ptr::null::<strnumber>() as *mut strnumber;
#[no_mangle]
pub static mut fontarea: *mut strnumber = ::core::ptr::null::<strnumber>() as *mut strnumber;
#[no_mangle]
pub static mut fontbc: *mut eightbits = ::core::ptr::null::<eightbits>() as *mut eightbits;
#[no_mangle]
pub static mut fontec: *mut eightbits = ::core::ptr::null::<eightbits>() as *mut eightbits;
#[no_mangle]
pub static mut fontglue: *mut halfword = ::core::ptr::null::<halfword>() as *mut halfword;
#[no_mangle]
pub static mut fontused: *mut boolean = ::core::ptr::null::<boolean>() as *mut boolean;
#[no_mangle]
pub static mut hyphenchar: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut skewchar: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut bcharlabel: *mut fontindex = ::core::ptr::null::<fontindex>() as *mut fontindex;
#[no_mangle]
pub static mut fontbchar: *mut ninebits = ::core::ptr::null::<ninebits>() as *mut ninebits;
#[no_mangle]
pub static mut fontfalsebchar: *mut ninebits = ::core::ptr::null::<ninebits>() as *mut ninebits;
#[no_mangle]
pub static mut charbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut widthbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut heightbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut depthbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut italicbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut ligkernbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut kernbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut extenbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut parambase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut nullcharacter: fourquarters = fourquarters {
    u: C2RustUnnamed_2 {
        B3: 0,
        B2: 0,
        B1: 0,
        B0: 0,
    },
};
#[no_mangle]
pub static mut totalpages: integer = 0;
#[no_mangle]
pub static mut maxv: scaled = 0;
#[no_mangle]
pub static mut maxh: scaled = 0;
#[no_mangle]
pub static mut maxpush: integer = 0;
#[no_mangle]
pub static mut lastbop: integer = 0;
#[no_mangle]
pub static mut deadcycles: integer = 0;
#[no_mangle]
pub static mut doingleaders: boolean = 0;
#[no_mangle]
pub static mut c: quarterword = 0;
#[no_mangle]
pub static mut f: internalfontnumber = 0;
#[no_mangle]
pub static mut ruleht: scaled = 0;
#[no_mangle]
pub static mut ruledp: scaled = 0;
#[no_mangle]
pub static mut rulewd: scaled = 0;
#[no_mangle]
pub static mut g: halfword = 0;
#[no_mangle]
pub static mut lq: integer = 0;
#[no_mangle]
pub static mut lr: integer = 0;
#[no_mangle]
pub static mut dvibuf: *mut eightbits = ::core::ptr::null::<eightbits>() as *mut eightbits;
#[no_mangle]
pub static mut halfbuf: integer = 0;
#[no_mangle]
pub static mut dvilimit: integer = 0;
#[no_mangle]
pub static mut dviptr: integer = 0;
#[no_mangle]
pub static mut dvioffset: integer = 0;
#[no_mangle]
pub static mut dvigone: integer = 0;
#[no_mangle]
pub static mut downptr: halfword = 0;
#[no_mangle]
pub static mut rightptr: halfword = 0;
#[no_mangle]
pub static mut dvih: scaled = 0;
#[no_mangle]
pub static mut dviv: scaled = 0;
#[no_mangle]
pub static mut curh: scaled = 0;
#[no_mangle]
pub static mut curv: scaled = 0;
#[no_mangle]
pub static mut dvif: internalfontnumber = 0;
#[no_mangle]
pub static mut curs: integer = 0;
#[no_mangle]
pub static mut pdfmemsize: integer = 0;
#[no_mangle]
pub static mut pdfmem: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdfmemptr: integer = 0;
#[no_mangle]
pub static mut pdffile: bytefile = ::core::ptr::null::<FILE>() as *mut FILE;
#[no_mangle]
pub static mut pdfbuf: *mut eightbits = ::core::ptr::null::<eightbits>() as *mut eightbits;
#[no_mangle]
pub static mut pdfbufsize: integer = 0;
#[no_mangle]
pub static mut pdfptr: integer = 0;
#[no_mangle]
pub static mut pdfopbuf: *mut eightbits = ::core::ptr::null::<eightbits>() as *mut eightbits;
#[no_mangle]
pub static mut pdfosbuf: *mut eightbits = ::core::ptr::null::<eightbits>() as *mut eightbits;
#[no_mangle]
pub static mut pdfosbufsize: integer = 0;
#[no_mangle]
pub static mut pdfosobjnum: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdfosobjoff: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdfosobjidx: halfword = 0;
#[no_mangle]
pub static mut pdfoscntr: integer = 0;
#[no_mangle]
pub static mut pdfopptr: integer = 0;
#[no_mangle]
pub static mut pdfosptr: integer = 0;
#[no_mangle]
pub static mut pdfosmode: boolean = 0;
#[no_mangle]
pub static mut pdfosenable: boolean = 0;
#[no_mangle]
pub static mut pdfoscurobjnum: integer = 0;
#[no_mangle]
pub static mut pdfgone: longinteger = 0;
#[no_mangle]
pub static mut pdfsaveoffset: longinteger = 0;
#[no_mangle]
pub static mut zipwritestate: integer = 0;
#[no_mangle]
pub static mut fixedpdfmajorversion: integer = 0;
#[no_mangle]
pub static mut fixedpdfminorversion: integer = 0;
#[no_mangle]
pub static mut fixedpdfobjcompresslevel: integer = 0;
#[no_mangle]
pub static mut pdfversionwritten: boolean = 0;
#[no_mangle]
pub static mut fixedpdfoutput: integer = 0;
#[no_mangle]
pub static mut fixedpdfoutputset: boolean = 0;
#[no_mangle]
pub static mut fixedgamma: integer = 0;
#[no_mangle]
pub static mut fixedimagegamma: integer = 0;
#[no_mangle]
pub static mut fixedimagehicolor: boolean = 0;
#[no_mangle]
pub static mut fixedimageapplygamma: integer = 0;
#[no_mangle]
pub static mut epochseconds: integer = 0;
#[no_mangle]
pub static mut microseconds: integer = 0;
#[no_mangle]
pub static mut fixedpdfdraftmode: integer = 0;
#[no_mangle]
pub static mut fixedpdfdraftmodeset: boolean = 0;
#[no_mangle]
pub static mut pdfpagegroupval: integer = 0;
#[no_mangle]
pub static mut onebp: scaled = 0;
#[no_mangle]
pub static mut onehundredbp: scaled = 0;
#[no_mangle]
pub static mut onehundredinch: scaled = 0;
#[no_mangle]
pub static mut oneinch: integer = 0;
#[no_mangle]
pub static mut tenpow: [integer; 10] = [0; 10];
#[no_mangle]
pub static mut scaledout: integer = 0;
#[no_mangle]
pub static mut initpdfoutput: boolean = 0;
#[no_mangle]
pub static mut advcharwidths: integer = 0;
#[no_mangle]
pub static mut advcharwidthsout: scaled = 0;
#[no_mangle]
pub static mut pdff: internalfontnumber = 0;
#[no_mangle]
pub static mut pdfh: scaled = 0;
#[no_mangle]
pub static mut pdfv: scaled = 0;
#[no_mangle]
pub static mut pdftjstarth: scaled = 0;
#[no_mangle]
pub static mut curdeltah: scaled = 0;
#[no_mangle]
pub static mut pdfdeltah: scaled = 0;
#[no_mangle]
pub static mut pdforiginh: scaled = 0;
#[no_mangle]
pub static mut pdforiginv: scaled = 0;
#[no_mangle]
pub static mut pdfdoingstring: boolean = 0;
#[no_mangle]
pub static mut pdfdoingtext: boolean = 0;
#[no_mangle]
pub static mut minbpval: scaled = 0;
#[no_mangle]
pub static mut minfontval: scaled = 0;
#[no_mangle]
pub static mut fixedpkresolution: integer = 0;
#[no_mangle]
pub static mut fixeddecimaldigits: integer = 0;
#[no_mangle]
pub static mut fixedgentounicode: integer = 0;
#[no_mangle]
pub static mut fixedinclusioncopyfont: integer = 0;
#[no_mangle]
pub static mut pkscalefactor: integer = 0;
#[no_mangle]
pub static mut pdfoutputoption: integer = 0;
#[no_mangle]
pub static mut pdfoutputvalue: integer = 0;
#[no_mangle]
pub static mut pdfdraftmodeoption: integer = 0;
#[no_mangle]
pub static mut pdfdraftmodevalue: integer = 0;
#[no_mangle]
pub static mut pdfcurTma: integer = 0;
#[no_mangle]
pub static mut pdflastf: internalfontnumber = 0;
#[no_mangle]
pub static mut pdflastfs: internalfontnumber = 0;
#[no_mangle]
pub static mut pdfdummyfont: internalfontnumber = 0;
#[no_mangle]
pub static mut objtabsize: integer = 0;
#[no_mangle]
pub static mut objtab: *mut objentry = ::core::ptr::null::<objentry>() as *mut objentry;
#[no_mangle]
pub static mut headtab: [integer; 11] = [0; 11];
#[no_mangle]
pub static mut pagestail: integer = 0;
#[no_mangle]
pub static mut objptr: integer = 0;
#[no_mangle]
pub static mut sysobjptr: integer = 0;
#[no_mangle]
pub static mut pdflastpages: integer = 0;
#[no_mangle]
pub static mut pdflastpage: integer = 0;
#[no_mangle]
pub static mut pdflaststream: integer = 0;
#[no_mangle]
pub static mut pdfstreamlength: longinteger = 0;
#[no_mangle]
pub static mut pdfstreamlengthoffset: longinteger = 0;
#[no_mangle]
pub static mut pdfseekwritelength: boolean = 0;
#[no_mangle]
pub static mut pdflastbyte: eightbits = 0;
#[no_mangle]
pub static mut pdfappendlistarg: integer = 0;
#[no_mangle]
pub static mut ff: integer = 0;
#[no_mangle]
pub static mut pdfboxspecmedia: integer = 0;
#[no_mangle]
pub static mut pdfboxspeccrop: integer = 0;
#[no_mangle]
pub static mut pdfboxspecbleed: integer = 0;
#[no_mangle]
pub static mut pdfboxspectrim: integer = 0;
#[no_mangle]
pub static mut pdfboxspecart: integer = 0;
#[no_mangle]
pub static mut pdfimageprocset: integer = 0;
#[no_mangle]
pub static mut pdftextprocset: boolean = 0;
#[no_mangle]
pub static mut pdffonttype: *mut eightbits = ::core::ptr::null::<eightbits>() as *mut eightbits;
#[no_mangle]
pub static mut pdffontattr: *mut strnumber = ::core::ptr::null::<strnumber>() as *mut strnumber;
#[no_mangle]
pub static mut pdffontnobuiltintounicode: *mut boolean =
    ::core::ptr::null::<boolean>() as *mut boolean;
#[no_mangle]
pub static mut pdfcharused: *mut charusedarray =
    ::core::ptr::null::<charusedarray>() as *mut charusedarray;
#[no_mangle]
pub static mut pdffontsize: *mut scaled = ::core::ptr::null::<scaled>() as *mut scaled;
#[no_mangle]
pub static mut pdffontnum: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontmap: *mut fmentryptr = ::core::ptr::null::<fmentryptr>() as *mut fmentryptr;
#[no_mangle]
pub static mut pdffontlist: halfword = 0;
#[no_mangle]
pub static mut pdfresnameprefix: strnumber = 0;
#[no_mangle]
pub static mut lasttokensstring: strnumber = 0;
#[no_mangle]
pub static mut vfpacketbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut vfdefaultfont: *mut internalfontnumber =
    ::core::ptr::null::<internalfontnumber>() as *mut internalfontnumber;
#[no_mangle]
pub static mut vflocalfontnum: *mut internalfontnumber =
    ::core::ptr::null::<internalfontnumber>() as *mut internalfontnumber;
#[no_mangle]
pub static mut vfpacketlength: integer = 0;
#[no_mangle]
pub static mut vffile: bytefile = ::core::ptr::null::<FILE>() as *mut FILE;
#[no_mangle]
pub static mut vfnf: internalfontnumber = 0;
#[no_mangle]
pub static mut vfefnts: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut vfifnts: *mut internalfontnumber =
    ::core::ptr::null::<internalfontnumber>() as *mut internalfontnumber;
#[no_mangle]
pub static mut tmpw: memoryword = memoryword { gr: 0. };
#[no_mangle]
pub static mut vfcurs: integer = 0;
#[no_mangle]
pub static mut vfstack: [vfstackrecord; 101] = [vfstackrecord {
    stackh: 0,
    stackv: 0,
    stackw: 0,
    stackx: 0,
    stacky: 0,
    stackz: 0,
}; 101];
#[no_mangle]
pub static mut vfstackptr: vfstackindex = 0;
#[no_mangle]
pub static mut savedpdfcurform: integer = 0;
#[no_mangle]
pub static mut pdftexbanner: strnumber = 0;
#[no_mangle]
pub static mut totalstretch: [scaled; 4] = [0; 4];
#[no_mangle]
pub static mut totalshrink: [scaled; 4] = [0; 4];
#[no_mangle]
pub static mut lastbadness: integer = 0;
#[no_mangle]
pub static mut adjusttail: halfword = 0;
#[no_mangle]
pub static mut pdffontblink: *mut internalfontnumber =
    ::core::ptr::null::<internalfontnumber>() as *mut internalfontnumber;
#[no_mangle]
pub static mut pdffontelink: *mut internalfontnumber =
    ::core::ptr::null::<internalfontnumber>() as *mut internalfontnumber;
#[no_mangle]
pub static mut pdffonthasspacechar: *mut boolean = ::core::ptr::null::<boolean>() as *mut boolean;
#[no_mangle]
pub static mut pdffontstretch: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontshrink: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontstep: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontexpandratio: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontautoexpand: *mut boolean = ::core::ptr::null::<boolean>() as *mut boolean;
#[no_mangle]
pub static mut pdffontlpbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontrpbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontefbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontknbsbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontstbsbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontshbsbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontknbcbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut pdffontknacbase: *mut integer = ::core::ptr::null::<integer>() as *mut integer;
#[no_mangle]
pub static mut fontexpandratio: integer = 0;
#[no_mangle]
pub static mut lastleftmostchar: halfword = 0;
#[no_mangle]
pub static mut lastrightmostchar: halfword = 0;
#[no_mangle]
pub static mut hliststack: [halfword; 513] = [0; 513];
#[no_mangle]
pub static mut hliststacklevel: ::core::ffi::c_short = 0;
#[no_mangle]
pub static mut preadjusttail: halfword = 0;
#[no_mangle]
pub static mut packbeginline: integer = 0;
#[no_mangle]
pub static mut emptyfield: twohalves = twohalves {
    v: C2RustUnnamed_1 { LH: 0, RH: 0 },
};
#[no_mangle]
pub static mut nulldelimiter: fourquarters = fourquarters {
    u: C2RustUnnamed_2 {
        B3: 0,
        B2: 0,
        B1: 0,
        B0: 0,
    },
};
#[no_mangle]
pub static mut curmlist: halfword = 0;
#[no_mangle]
pub static mut curstyle: smallnumber = 0;
#[no_mangle]
pub static mut cursize: smallnumber = 0;
#[no_mangle]
pub static mut curmu: scaled = 0;
#[no_mangle]
pub static mut mlistpenalties: boolean = 0;
#[no_mangle]
pub static mut curf: internalfontnumber = 0;
#[no_mangle]
pub static mut curc: quarterword = 0;
#[no_mangle]
pub static mut curi: fourquarters = fourquarters {
    u: C2RustUnnamed_2 {
        B3: 0,
        B2: 0,
        B1: 0,
        B0: 0,
    },
};
#[no_mangle]
pub static mut magicoffset: integer = 0;
#[no_mangle]
pub static mut curalign: halfword = 0;
#[no_mangle]
pub static mut curspan: halfword = 0;
#[no_mangle]
pub static mut curloop: halfword = 0;
#[no_mangle]
pub static mut alignptr: halfword = 0;
#[no_mangle]
pub static mut curhead: halfword = 0;
#[no_mangle]
pub static mut curtail: halfword = 0;
#[no_mangle]
pub static mut curprehead: halfword = 0;
#[no_mangle]
pub static mut curpretail: halfword = 0;
#[no_mangle]
pub static mut justbox: halfword = 0;
#[no_mangle]
pub static mut passive: halfword = 0;
#[no_mangle]
pub static mut printednode: halfword = 0;
#[no_mangle]
pub static mut passnumber: halfword = 0;
#[no_mangle]
pub static mut activewidth: [scaled; 9] = [0; 9];
#[no_mangle]
pub static mut curactivewidth: [scaled; 9] = [0; 9];
#[no_mangle]
pub static mut background: [scaled; 9] = [0; 9];
#[no_mangle]
pub static mut breakwidth: [scaled; 9] = [0; 9];
#[no_mangle]
pub static mut autobreaking: boolean = 0;
#[no_mangle]
pub static mut prevp: halfword = 0;
#[no_mangle]
pub static mut firstp: halfword = 0;
#[no_mangle]
pub static mut prevcharp: halfword = 0;
#[no_mangle]
pub static mut nextcharp: halfword = 0;
#[no_mangle]
pub static mut tryprevbreak: boolean = 0;
#[no_mangle]
pub static mut prevlegal: halfword = 0;
#[no_mangle]
pub static mut prevprevlegal: halfword = 0;
#[no_mangle]
pub static mut prevautobreaking: boolean = 0;
#[no_mangle]
pub static mut prevactivewidth: [scaled; 9] = [0; 9];
#[no_mangle]
pub static mut rejectedcurp: halfword = 0;
#[no_mangle]
pub static mut beforerejectedcurp: boolean = 0;
#[no_mangle]
pub static mut maxstretchratio: integer = 0;
#[no_mangle]
pub static mut maxshrinkratio: integer = 0;
#[no_mangle]
pub static mut curfontstep: integer = 0;
#[no_mangle]
pub static mut noshrinkerroryet: boolean = 0;
#[no_mangle]
pub static mut curp: halfword = 0;
#[no_mangle]
pub static mut secondpass: boolean = 0;
#[no_mangle]
pub static mut finalpass: boolean = 0;
#[no_mangle]
pub static mut threshold: integer = 0;
#[no_mangle]
pub static mut minimaldemerits: [integer; 4] = [0; 4];
#[no_mangle]
pub static mut minimumdemerits: integer = 0;
#[no_mangle]
pub static mut bestplace: [halfword; 4] = [0; 4];
#[no_mangle]
pub static mut bestplline: [halfword; 4] = [0; 4];
#[no_mangle]
pub static mut discwidth: [scaled; 9] = [0; 9];
#[no_mangle]
pub static mut easyline: halfword = 0;
#[no_mangle]
pub static mut lastspecialline: halfword = 0;
#[no_mangle]
pub static mut firstwidth: scaled = 0;
#[no_mangle]
pub static mut secondwidth: scaled = 0;
#[no_mangle]
pub static mut firstindent: scaled = 0;
#[no_mangle]
pub static mut secondindent: scaled = 0;
#[no_mangle]
pub static mut bestbet: halfword = 0;
#[no_mangle]
pub static mut fewestdemerits: integer = 0;
#[no_mangle]
pub static mut bestline: halfword = 0;
#[no_mangle]
pub static mut actuallooseness: integer = 0;
#[no_mangle]
pub static mut linediff: integer = 0;
#[no_mangle]
pub static mut hc: [::core::ffi::c_short; 66] = [0; 66];
#[no_mangle]
pub static mut hn: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut ha: halfword = 0;
#[no_mangle]
pub static mut hb: halfword = 0;
#[no_mangle]
pub static mut hf: internalfontnumber = 0;
#[no_mangle]
pub static mut hu: [::core::ffi::c_short; 64] = [0; 64];
#[no_mangle]
pub static mut hyfchar: integer = 0;
#[no_mangle]
pub static mut initcurlang: ASCIIcode = 0;
#[no_mangle]
pub static mut curlang: ASCIIcode = 0;
#[no_mangle]
pub static mut lhyf: integer = 0;
#[no_mangle]
pub static mut rhyf: integer = 0;
#[no_mangle]
pub static mut initlhyf: integer = 0;
#[no_mangle]
pub static mut initrhyf: integer = 0;
#[no_mangle]
pub static mut hyfbchar: halfword = 0;
#[no_mangle]
pub static mut hyf: [::core::ffi::c_uchar; 65] = [0; 65];
#[no_mangle]
pub static mut initlist: halfword = 0;
#[no_mangle]
pub static mut initlig: boolean = 0;
#[no_mangle]
pub static mut initlft: boolean = 0;
#[no_mangle]
pub static mut hyphenpassed: smallnumber = 0;
#[no_mangle]
pub static mut curl: halfword = 0;
#[no_mangle]
pub static mut curr: halfword = 0;
#[no_mangle]
pub static mut curq: halfword = 0;
#[no_mangle]
pub static mut ligstack: halfword = 0;
#[no_mangle]
pub static mut ligaturepresent: boolean = 0;
#[no_mangle]
pub static mut lfthit: boolean = 0;
#[no_mangle]
pub static mut rthit: boolean = 0;
#[no_mangle]
pub static mut trietrl: *mut triepointer = ::core::ptr::null::<triepointer>() as *mut triepointer;
#[no_mangle]
pub static mut trietro: *mut triepointer = ::core::ptr::null::<triepointer>() as *mut triepointer;
#[no_mangle]
pub static mut trietrc: *mut quarterword = ::core::ptr::null::<quarterword>() as *mut quarterword;
#[no_mangle]
pub static mut hyfdistance: [smallnumber; 35112] = [0; 35112];
#[no_mangle]
pub static mut hyfnum: [smallnumber; 35112] = [0; 35112];
#[no_mangle]
pub static mut hyfnext: [trieopcode; 35112] = [0; 35112];
#[no_mangle]
pub static mut opstart: [integer; 256] = [0; 256];
#[no_mangle]
pub static mut hyphword: *mut strnumber = ::core::ptr::null::<strnumber>() as *mut strnumber;
#[no_mangle]
pub static mut hyphlist: *mut halfword = ::core::ptr::null::<halfword>() as *mut halfword;
#[no_mangle]
pub static mut hyphlink: *mut hyphpointer = ::core::ptr::null::<hyphpointer>() as *mut hyphpointer;
#[no_mangle]
pub static mut hyphcount: integer = 0;
#[no_mangle]
pub static mut hyphnext: integer = 0;
#[no_mangle]
pub static mut zzzab: [integer; 70223] = [0; 70223];
#[no_mangle]
pub static mut trieused: [trieopcode; 256] = [0; 256];
#[no_mangle]
pub static mut trieoplang: [ASCIIcode; 35112] = [0; 35112];
#[no_mangle]
pub static mut trieopval: [trieopcode; 35112] = [0; 35112];
#[no_mangle]
pub static mut trieopptr: integer = 0;
#[no_mangle]
pub static mut maxopused: trieopcode = 0;
#[no_mangle]
pub static mut smallop: boolean = 0;
#[no_mangle]
pub static mut triec: *mut packedASCIIcode =
    ::core::ptr::null::<packedASCIIcode>() as *mut packedASCIIcode;
#[no_mangle]
pub static mut trieo: *mut trieopcode = ::core::ptr::null::<trieopcode>() as *mut trieopcode;
#[no_mangle]
pub static mut triel: *mut triepointer = ::core::ptr::null::<triepointer>() as *mut triepointer;
#[no_mangle]
pub static mut trier: *mut triepointer = ::core::ptr::null::<triepointer>() as *mut triepointer;
#[no_mangle]
pub static mut trieptr: triepointer = 0;
#[no_mangle]
pub static mut triehash: *mut triepointer = ::core::ptr::null::<triepointer>() as *mut triepointer;
#[no_mangle]
pub static mut trietaken: *mut boolean = ::core::ptr::null::<boolean>() as *mut boolean;
#[no_mangle]
pub static mut triemin: [triepointer; 256] = [0; 256];
#[no_mangle]
pub static mut triemax: triepointer = 0;
#[no_mangle]
pub static mut trienotready: boolean = 0;
#[no_mangle]
pub static mut bestheightplusdepth: scaled = 0;
#[no_mangle]
pub static mut pagetail: halfword = 0;
#[no_mangle]
pub static mut pagecontents: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut pagemaxdepth: scaled = 0;
#[no_mangle]
pub static mut bestpagebreak: halfword = 0;
#[no_mangle]
pub static mut leastpagecost: integer = 0;
#[no_mangle]
pub static mut bestsize: scaled = 0;
#[no_mangle]
pub static mut pagesofar: [scaled; 8] = [0; 8];
#[no_mangle]
pub static mut lastglue: halfword = 0;
#[no_mangle]
pub static mut lastpenalty: integer = 0;
#[no_mangle]
pub static mut lastkern: scaled = 0;
#[no_mangle]
pub static mut lastnodetype: integer = 0;
#[no_mangle]
pub static mut insertpenalties: integer = 0;
#[no_mangle]
pub static mut outputactive: boolean = 0;
#[no_mangle]
pub static mut outputcanend: boolean = 0;
#[no_mangle]
pub static mut mainf: internalfontnumber = 0;
#[no_mangle]
pub static mut maini: fourquarters = fourquarters {
    u: C2RustUnnamed_2 {
        B3: 0,
        B2: 0,
        B1: 0,
        B0: 0,
    },
};
#[no_mangle]
pub static mut mainj: fourquarters = fourquarters {
    u: C2RustUnnamed_2 {
        B3: 0,
        B2: 0,
        B1: 0,
        B0: 0,
    },
};
#[no_mangle]
pub static mut maink: fontindex = 0;
#[no_mangle]
pub static mut mainp: halfword = 0;
#[no_mangle]
pub static mut mains: integer = 0;
#[no_mangle]
pub static mut bchar: halfword = 0;
#[no_mangle]
pub static mut falsebchar: halfword = 0;
#[no_mangle]
pub static mut cancelboundary: boolean = 0;
#[no_mangle]
pub static mut insdisc: boolean = 0;
#[no_mangle]
pub static mut curbox: halfword = 0;
#[no_mangle]
pub static mut aftertoken: halfword = 0;
#[no_mangle]
pub static mut longhelpseen: boolean = 0;
#[no_mangle]
pub static mut formatident: strnumber = 0;
#[no_mangle]
pub static mut fmtfile: wordfile = ::core::ptr::null::<gzFile_s>() as *mut gzFile_s;
#[no_mangle]
pub static mut readyalready: integer = 0;
#[no_mangle]
pub static mut writefile: [alphafile; 16] = [::core::ptr::null::<FILE>() as *mut FILE; 16];
#[no_mangle]
pub static mut writeopen: [boolean; 18] = [0; 18];
#[no_mangle]
pub static mut writeloc: halfword = 0;
#[no_mangle]
pub static mut pdflastobj: integer = 0;
#[no_mangle]
pub static mut pdflastxform: integer = 0;
#[no_mangle]
pub static mut pdflastximage: integer = 0;
#[no_mangle]
pub static mut pdflastximagepages: integer = 0;
#[no_mangle]
pub static mut pdflastximagecolordepth: integer = 0;
#[no_mangle]
pub static mut altrule: halfword = 0;
#[no_mangle]
pub static mut warnpdfpagebox: boolean = 0;
#[no_mangle]
pub static mut pdflastannot: integer = 0;
#[no_mangle]
pub static mut pdflastlink: integer = 0;
#[no_mangle]
pub static mut pdflastxpos: integer = 0;
#[no_mangle]
pub static mut pdflastypos: integer = 0;
#[no_mangle]
pub static mut pdfsnapxrefpos: integer = 0;
#[no_mangle]
pub static mut pdfsnapyrefpos: integer = 0;
#[no_mangle]
pub static mut countdosnapy: integer = 0;
#[no_mangle]
pub static mut pdfretval: integer = 0;
#[no_mangle]
pub static mut curpagewidth: scaled = 0;
#[no_mangle]
pub static mut curpageheight: scaled = 0;
#[no_mangle]
pub static mut curhoffset: scaled = 0;
#[no_mangle]
pub static mut curvoffset: scaled = 0;
#[no_mangle]
pub static mut pdfobjlist: halfword = 0;
#[no_mangle]
pub static mut pdfxformlist: halfword = 0;
#[no_mangle]
pub static mut pdfximagelist: halfword = 0;
#[no_mangle]
pub static mut lastthread: halfword = 0;
#[no_mangle]
pub static mut pdfthreadht: scaled = 0;
#[no_mangle]
pub static mut pdfthreaddp: scaled = 0;
#[no_mangle]
pub static mut pdfthreadwd: scaled = 0;
#[no_mangle]
pub static mut pdflastthreadid: halfword = 0;
#[no_mangle]
pub static mut pdflastthreadnamedid: boolean = 0;
#[no_mangle]
pub static mut pdfthreadlevel: integer = 0;
#[no_mangle]
pub static mut pdfannotlist: halfword = 0;
#[no_mangle]
pub static mut pdflinklist: halfword = 0;
#[no_mangle]
pub static mut pdfdestlist: halfword = 0;
#[no_mangle]
pub static mut pdfbeadlist: halfword = 0;
#[no_mangle]
pub static mut pdfobjcount: integer = 0;
#[no_mangle]
pub static mut pdfxformcount: integer = 0;
#[no_mangle]
pub static mut pdfximagecount: integer = 0;
#[no_mangle]
pub static mut pdfcurform: integer = 0;
#[no_mangle]
pub static mut pdflastoutline: integer = 0;
#[no_mangle]
pub static mut pdffirstoutline: integer = 0;
#[no_mangle]
pub static mut pdfparentoutline: integer = 0;
#[no_mangle]
pub static mut pdfxformwidth: scaled = 0;
#[no_mangle]
pub static mut pdfxformdepth: scaled = 0;
#[no_mangle]
pub static mut pdfxformheight: scaled = 0;
#[no_mangle]
pub static mut pdfinfotoks: halfword = 0;
#[no_mangle]
pub static mut pdfcatalogtoks: halfword = 0;
#[no_mangle]
pub static mut pdfcatalogopenaction: integer = 0;
#[no_mangle]
pub static mut pdfnamestoks: halfword = 0;
#[no_mangle]
pub static mut pdfdestnamesptr: integer = 0;
#[no_mangle]
pub static mut destnamessize: integer = 0;
#[no_mangle]
pub static mut destnames: *mut destnameentry =
    ::core::ptr::null::<destnameentry>() as *mut destnameentry;
#[no_mangle]
pub static mut pkdpi: integer = 0;
#[no_mangle]
pub static mut imageorigy: integer = 0;
#[no_mangle]
pub static mut imageorigx: integer = 0;
#[no_mangle]
pub static mut pdftrailertoks: halfword = 0;
#[no_mangle]
pub static mut pdftraileridtoks: halfword = 0;
#[no_mangle]
pub static mut genfakedinterwordspace: boolean = 0;
#[no_mangle]
pub static mut genrunninglink: boolean = 0;
#[no_mangle]
pub static mut pdfspacefontname: strnumber = 0;
#[no_mangle]
pub static mut pdflinkstack: [pdflinkstackrecord; 11] = [pdflinkstackrecord {
    nestinglevel: 0,
    linknode: 0,
    reflinknode: 0,
}; 11];
#[no_mangle]
pub static mut pdflinkstackptr: smallnumber = 0;
#[no_mangle]
pub static mut isshippingpage: boolean = 0;
#[no_mangle]
pub static mut eTeXmode: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut etexp: boolean = 0;
#[no_mangle]
pub static mut eofseen: *mut boolean = ::core::ptr::null::<boolean>() as *mut boolean;
#[no_mangle]
pub static mut LRptr: halfword = 0;
#[no_mangle]
pub static mut LRproblems: integer = 0;
#[no_mangle]
pub static mut curdir: smallnumber = 0;
#[no_mangle]
pub static mut pseudofiles: halfword = 0;
#[no_mangle]
pub static mut grpstack: *mut savepointer = ::core::ptr::null::<savepointer>() as *mut savepointer;
#[no_mangle]
pub static mut ifstack: *mut halfword = ::core::ptr::null::<halfword>() as *mut halfword;
#[no_mangle]
pub static mut maxregnum: halfword = 0;
#[no_mangle]
pub static mut maxreghelpline: strnumber = 0;
#[no_mangle]
pub static mut saroot: [halfword; 7] = [0; 7];
#[no_mangle]
pub static mut curptr: halfword = 0;
#[no_mangle]
pub static mut sanull: memoryword = memoryword { gr: 0. };
#[no_mangle]
pub static mut sachain: halfword = 0;
#[no_mangle]
pub static mut salevel: quarterword = 0;
#[no_mangle]
pub static mut lastlinefill: halfword = 0;
#[no_mangle]
pub static mut dolastlinefit: boolean = 0;
#[no_mangle]
pub static mut activenodesize: smallnumber = 0;
#[no_mangle]
pub static mut fillwidth: [scaled; 3] = [0; 3];
#[no_mangle]
pub static mut bestplshort: [scaled; 4] = [0; 4];
#[no_mangle]
pub static mut bestplglue: [scaled; 4] = [0; 4];
#[no_mangle]
pub static mut hyphstart: triepointer = 0;
#[no_mangle]
pub static mut hyphindex: triepointer = 0;
#[no_mangle]
pub static mut discptr: [halfword; 4] = [0; 4];
#[no_mangle]
pub static mut editnamestart: poolpointer = 0;
#[no_mangle]
pub static mut editline: integer = 0;
#[no_mangle]
pub static mut editnamelength: integer = 0;
#[no_mangle]
pub static mut ipcon: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut stopatspace: boolean = 0;
#[no_mangle]
pub static mut savestrptr: strnumber = 0;
#[no_mangle]
pub static mut savepoolptr: poolpointer = 0;
#[no_mangle]
pub static mut shellenabledp: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut restrictedshell: ::core::ffi::c_int = 0;
#[no_mangle]
pub static mut outputcomment: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut k: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut l: ::core::ffi::c_uchar = 0;
#[no_mangle]
pub static mut debugformatfile: boolean = 0;
#[no_mangle]
pub static mut expanddepthcount: integer = 0;
#[no_mangle]
pub static mut mltexp: boolean = 0;
#[no_mangle]
pub static mut mltexenabledp: boolean = 0;
#[no_mangle]
pub static mut accentc: integer = 0;
#[no_mangle]
pub static mut basec: integer = 0;
#[no_mangle]
pub static mut replacec: integer = 0;
#[no_mangle]
pub static mut iac: fourquarters = fourquarters {
    u: C2RustUnnamed_2 {
        B3: 0,
        B2: 0,
        B1: 0,
        B0: 0,
    },
};
#[no_mangle]
pub static mut ibc: fourquarters = fourquarters {
    u: C2RustUnnamed_2 {
        B3: 0,
        B2: 0,
        B1: 0,
        B0: 0,
    },
};
#[no_mangle]
pub static mut baseslant: real = 0.;
#[no_mangle]
pub static mut accentslant: real = 0.;
#[no_mangle]
pub static mut basexheight: scaled = 0;
#[no_mangle]
pub static mut basewidth: scaled = 0;
#[no_mangle]
pub static mut baseheight: scaled = 0;
#[no_mangle]
pub static mut accentwidth: scaled = 0;
#[no_mangle]
pub static mut accentheight: scaled = 0;
#[no_mangle]
pub static mut delta: scaled = 0;
#[no_mangle]
pub static mut enctexp: boolean = 0;
#[no_mangle]
pub static mut enctexenabledp: boolean = 0;
#[no_mangle]
pub static mut synctexoption: integer = 0;
#[no_mangle]
pub static mut synctexoffset: integer = 0;
pub const COPYRIGHT_HOLDER: [::core::ffi::c_char; 29] = unsafe {
    ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
        *b"Han The Thanh (pdfTeX) et al\0",
    )
};
pub const AUTHOR: *mut ::core::ffi::c_void = NULL;
pub const BUG_ADDRESS: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"pdftex@tug.org\0") };
pub const DUMP_OPTION: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"fmt\0") };
pub const DUMP_EXT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b".fmt\0") };
#[no_mangle]
pub static mut PDFTEXHELP: [const_string; 57] = [
    b"Usage: pdftex [OPTION]... [TEXNAME[.tex]] [COMMANDS]\0" as *const u8
        as *const ::core::ffi::c_char,
    b"   or: pdftex [OPTION]... \\FIRST-LINE\0" as *const u8 as *const ::core::ffi::c_char,
    b"   or: pdftex [OPTION]... &FMT ARGS\0" as *const u8 as *const ::core::ffi::c_char,
    b"  Run pdfTeX on TEXNAME, usually creating TEXNAME.pdf.\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  Any remaining COMMANDS are processed as pdfTeX input, after TEXNAME is read.\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  If the first line of TEXNAME is %&FMT, and FMT is an existing .fmt file,\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  use it.  Else use `NAME.fmt', where NAME is the program invocation name,\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  most commonly `pdftex'.\0" as *const u8 as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"  Alternatively, if the first non-option argument begins with a backslash,\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  interpret all non-option arguments as a line of pdfTeX input.\0" as *const u8
        as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"  Alternatively, if the first non-option argument begins with a &, the\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  next word is taken as the FMT to read, overriding all else.  Any\0" as *const u8
        as *const ::core::ffi::c_char,
    b"  remaining arguments are processed as above.\0" as *const u8 as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"  If no arguments or options are specified, prompt for input.\0" as *const u8
        as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"-cnf-line=STRING        parse STRING as a configuration file line\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-draftmode              switch on draft mode (generates no output PDF)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-enc                    enable encTeX extensions such as \\mubyte\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-etex                   enable e-TeX extensions\0" as *const u8 as *const ::core::ffi::c_char,
    b"[-no]-file-line-error   disable/enable file:line:error style messages\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-fmt=FMTNAME            use FMTNAME instead of program name or a %& line\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-halt-on-error          stop processing at the first error\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-ini                    be pdfinitex, for dumping formats; this is implicitly\0" as *const u8
        as *const ::core::ffi::c_char,
    b"                          true if the program name is `pdfinitex'\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-interaction=STRING     set interaction mode (STRING=batchmode/nonstopmode/\0" as *const u8
        as *const ::core::ffi::c_char,
    b"                          scrollmode/errorstopmode)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-ipc                    send DVI output to a socket as well as the usual\0" as *const u8
        as *const ::core::ffi::c_char,
    b"                          output file\0" as *const u8 as *const ::core::ffi::c_char,
    b"-ipc-start              as -ipc, and also start the server at the other end\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-jobname=STRING         set the job name to STRING\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-kpathsea-debug=NUMBER  set path searching debugging flags according to\0" as *const u8
        as *const ::core::ffi::c_char,
    b"                          the bits of NUMBER\0" as *const u8 as *const ::core::ffi::c_char,
    b"[-no]-mktex=FMT         disable/enable mktexFMT generation (FMT=tex/tfm/pk)\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-mltex                  enable MLTeX extensions such as \\charsubdef\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-output-comment=STRING  use STRING for DVI file comment instead of date\0" as *const u8
        as *const ::core::ffi::c_char,
    b"                          (no effect for PDF)\0" as *const u8 as *const ::core::ffi::c_char,
    b"-output-directory=DIR   use existing DIR as the directory to write files in\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-output-format=FORMAT   use FORMAT for job output; FORMAT is `dvi' or `pdf'\0" as *const u8
        as *const ::core::ffi::c_char,
    b"[-no]-parse-first-line  disable/enable parsing of first line of input file\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-progname=STRING        set program (and fmt) name to STRING\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-recorder               enable filename recorder\0" as *const u8
        as *const ::core::ffi::c_char,
    b"[-no]-shell-escape      disable/enable \\write18{SHELL COMMAND}\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-shell-restricted       enable restricted \\write18\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-src-specials           insert source specials into the DVI file\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-src-specials=WHERE     insert source specials in certain places of\0" as *const u8
        as *const ::core::ffi::c_char,
    b"                          the DVI file. WHERE is a comma-separated value\0" as *const u8
        as *const ::core::ffi::c_char,
    b"                          list: cr display hbox math par parend vbox\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-translate-file=TCXNAME use the TCX file TCXNAME\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-8bit                   make all characters printable by default\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-help                   display this help and exit\0" as *const u8
        as *const ::core::ffi::c_char,
    b"-version                output version information and exit\0" as *const u8
        as *const ::core::ffi::c_char,
    b"\0" as *const u8 as *const ::core::ffi::c_char,
    b"pdfTeX home page: <https://pdftex.org>\0" as *const u8 as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
pub const edit_var: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"TEXEDIT\0") };
unsafe extern "C" fn Isspace(mut c_0: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return (c_0 as ::core::ffi::c_int == ' ' as i32 || c_0 as ::core::ffi::c_int == '\t' as i32)
        as ::core::ffi::c_int;
}
static mut cmdlist: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char;
unsafe extern "C" fn mk_shellcmdlist(mut v: *mut ::core::ffi::c_char) {
    let mut p: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: size_t = 0;
    q = v;
    n = 1 as size_t;
    loop {
        r = strchr(q, ',' as i32);
        if r.is_null() {
            break;
        }
        n = n.wrapping_add(1);
        q = r.offset(1 as ::core::ffi::c_int as isize);
    }
    if *q != 0 {
        n = n.wrapping_add(1);
    }
    cmdlist = xmalloc(n.wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t))
        as *mut *mut ::core::ffi::c_char;
    p = cmdlist;
    q = v;
    loop {
        r = strchr(q, ',' as i32);
        if r.is_null() {
            break;
        }
        *r = '\0' as i32 as ::core::ffi::c_char;
        let fresh20 = p;
        p = p.offset(1);
        *fresh20 = xstrdup(q as const_string) as *mut ::core::ffi::c_char;
        q = r.offset(1 as ::core::ffi::c_int as isize);
    }
    if *q != 0 {
        let fresh21 = p;
        p = p.offset(1);
        *fresh21 = xstrdup(q as const_string) as *mut ::core::ffi::c_char;
    }
    *p = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn init_shell_escape() {
    if shellenabledp < 0 as ::core::ffi::c_int {
        shellenabledp = 0 as ::core::ffi::c_int;
    } else {
        if shellenabledp == 0 as ::core::ffi::c_int {
            let mut v1: *mut ::core::ffi::c_char =
                kpse_var_value(b"shell_escape\0" as *const u8 as const_string)
                    as *mut ::core::ffi::c_char;
            if !v1.is_null() {
                if *v1 as ::core::ffi::c_int == 't' as i32
                    || *v1 as ::core::ffi::c_int == 'y' as i32
                    || *v1 as ::core::ffi::c_int == '1' as i32
                {
                    shellenabledp = 1 as ::core::ffi::c_int;
                } else if *v1 as ::core::ffi::c_int == 'p' as i32 {
                    shellenabledp = 1 as ::core::ffi::c_int;
                    restrictedshell = 1 as ::core::ffi::c_int;
                }
                free(v1 as *mut ::core::ffi::c_void);
            }
        }
        if shellenabledp != 0 && restrictedshell == 1 as ::core::ffi::c_int {
            let mut v2: *mut ::core::ffi::c_char =
                kpse_var_value(b"shell_escape_commands\0" as *const u8 as const_string)
                    as *mut ::core::ffi::c_char;
            if !v2.is_null() {
                mk_shellcmdlist(v2);
                free(v2 as *mut ::core::ffi::c_void);
            }
        }
    };
}
pub const QUOTE: ::core::ffi::c_int = '\'' as i32;
unsafe extern "C" fn shell_cmd_is_allowed(
    mut cmd: *const ::core::ffi::c_char,
    mut safecmd: *mut *mut ::core::ffi::c_char,
    mut cmdname: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut d: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut pre: ::core::ffi::c_int = 0;
    let mut spaces: ::core::ffi::c_int = 0;
    let mut allow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    buf = xmalloc(strlen(cmd).wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    strcpy(buf, cmd);
    c_0 = buf;
    while Isspace(*c_0) != 0 {
        c_0 = c_0.offset(1);
    }
    d = c_0;
    while Isspace(*d) == 0 && *d as ::core::ffi::c_int != 0 {
        d = d.offset(1);
    }
    *d = '\0' as i32 as ::core::ffi::c_char;
    *cmdname = xstrdup(c_0 as const_string) as *mut ::core::ffi::c_char;
    free(buf as *mut ::core::ffi::c_void);
    p = cmdlist;
    if !p.is_null() {
        while !(*p).is_null() {
            if strcmp(*p, *cmdname) == 0 as ::core::ffi::c_int {
                allow = 2 as ::core::ffi::c_int;
                break;
            } else {
                p = p.offset(1);
            }
        }
    }
    if allow == 2 as ::core::ffi::c_int {
        spaces = 0 as ::core::ffi::c_int;
        s = cmd;
        while *s != 0 {
            if Isspace(*s) != 0 {
                spaces += 1;
            }
            s = s.offset(1);
        }
        *safecmd = xmalloc(
            strlen(cmd)
                .wrapping_add(3 as size_t)
                .wrapping_add((2 as ::core::ffi::c_int * spaces) as size_t),
        ) as *mut ::core::ffi::c_char;
        s = cmd;
        while Isspace(*s) != 0 {
            s = s.offset(1);
        }
        d = *safecmd;
        while Isspace(*s) == 0 && *s as ::core::ffi::c_int != 0 {
            let fresh5 = s;
            s = s.offset(1);
            let fresh6 = d;
            d = d.offset(1);
            *fresh6 = *fresh5;
        }
        pre = 1 as ::core::ffi::c_int;
        while *s != 0 {
            if *s as ::core::ffi::c_int == '\'' as i32 {
                return -(1 as ::core::ffi::c_int);
            }
            if *s as ::core::ffi::c_int == '"' as i32 {
                if pre == 0 as ::core::ffi::c_int {
                    let fresh7 = d;
                    d = d.offset(1);
                    *fresh7 = QUOTE as ::core::ffi::c_char;
                }
                pre = 0 as ::core::ffi::c_int;
                let fresh8 = d;
                d = d.offset(1);
                *fresh8 = QUOTE as ::core::ffi::c_char;
                s = s.offset(1);
                while *s as ::core::ffi::c_int != '"' as i32 {
                    if *s as ::core::ffi::c_int == '\'' as i32
                        || *s as ::core::ffi::c_int == '\0' as i32
                    {
                        return -(1 as ::core::ffi::c_int);
                    }
                    let fresh9 = s;
                    s = s.offset(1);
                    let fresh10 = d;
                    d = d.offset(1);
                    *fresh10 = *fresh9;
                }
                s = s.offset(1);
                if Isspace(*s) == 0 && *s as ::core::ffi::c_int != 0 {
                    return -(1 as ::core::ffi::c_int);
                }
            } else if pre == 1 as ::core::ffi::c_int && Isspace(*s) == 0 {
                pre = 0 as ::core::ffi::c_int;
                let fresh11 = d;
                d = d.offset(1);
                *fresh11 = QUOTE as ::core::ffi::c_char;
                let fresh12 = s;
                s = s.offset(1);
                let fresh13 = d;
                d = d.offset(1);
                *fresh13 = *fresh12;
            } else if pre == 0 as ::core::ffi::c_int && Isspace(*s) != 0 {
                pre = 1 as ::core::ffi::c_int;
                let fresh14 = d;
                d = d.offset(1);
                *fresh14 = QUOTE as ::core::ffi::c_char;
                let fresh15 = s;
                s = s.offset(1);
                let fresh16 = d;
                d = d.offset(1);
                *fresh16 = *fresh15;
            } else {
                let fresh17 = s;
                s = s.offset(1);
                let fresh18 = d;
                d = d.offset(1);
                *fresh18 = *fresh17;
            }
        }
        if pre == 0 as ::core::ffi::c_int {
            let fresh19 = d;
            d = d.offset(1);
            *fresh19 = QUOTE as ::core::ffi::c_char;
        }
        *d = '\0' as i32 as ::core::ffi::c_char;
    }
    return allow;
}
#[no_mangle]
pub unsafe extern "C" fn runsystem(mut cmd: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut allow: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut safecmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cmdname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if shellenabledp <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if restrictedshell == 0 as ::core::ffi::c_int {
        allow = 1 as ::core::ffi::c_int;
    } else {
        allow = shell_cmd_is_allowed(cmd, &raw mut safecmd, &raw mut cmdname);
    }
    if allow == 1 as ::core::ffi::c_int {
        status = system(cmd);
    } else if allow == 2 as ::core::ffi::c_int {
        let mut k_0: size_t = 0;
        k_0 = 0 as size_t;
        while k_0 < strlen(safecmd) {
            if *safecmd.offset(k_0 as isize) as ::core::ffi::c_int == '|' as i32 {
                return 0 as ::core::ffi::c_int;
            }
            k_0 = k_0.wrapping_add(1);
        }
        status = system(safecmd);
    }
    if status != 0 as ::core::ffi::c_int {
        fprintf(
            __stderrp,
            b"system returned with code %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            status,
        );
    }
    if !safecmd.is_null() {
        free(safecmd as *mut ::core::ffi::c_void);
    }
    if !cmdname.is_null() {
        free(cmdname as *mut ::core::ffi::c_void);
    }
    return allow;
}
unsafe extern "C" fn runpopen(
    mut cmd: *mut ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> *mut FILE {
    let mut f_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut safecmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cmdname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut allow: ::core::ffi::c_int = 0;
    if restrictedshell == 0 as ::core::ffi::c_int {
        allow = 1 as ::core::ffi::c_int;
    } else {
        allow = shell_cmd_is_allowed(cmd, &raw mut safecmd, &raw mut cmdname);
    }
    if allow == 1 as ::core::ffi::c_int {
        f_0 = popen(cmd, mode);
    } else if allow == 2 as ::core::ffi::c_int {
        f_0 = popen(safecmd, mode);
    } else if allow == -(1 as ::core::ffi::c_int) {
        fprintf(
            __stderrp,
            b"\nrunpopen quotation error in command line: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            cmd,
        );
    } else {
        fprintf(
            __stderrp,
            b"\nrunpopen command not allowed: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            cmdname,
        );
    }
    if !safecmd.is_null() {
        free(safecmd as *mut ::core::ffi::c_void);
    }
    if !cmdname.is_null() {
        free(cmdname as *mut ::core::ffi::c_void);
    }
    return f_0;
}
#[no_mangle]
pub static mut argv: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut argc: ::core::ffi::c_int = 0;
static mut user_progname: const_string = ::core::ptr::null::<::core::ffi::c_char>();
static mut user_cnf_lines: *mut string = ::core::ptr::null::<string>() as *mut string;
static mut user_cnf_nlines: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
static mut c_job_name: const_string = ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub static mut translate_filename: string =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut default_translate_filename: string =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut last_source_name: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut last_lineno: ::core::ffi::c_int = 0;
static mut srcspecialsoption: boolean = false_0;
#[no_mangle]
pub unsafe extern "C" fn texmf_yesno(mut var: const_string) -> boolean {
    let mut value: string = kpse_var_value(var);
    return (!value.is_null()
        && (*value as ::core::ffi::c_int == 't' as i32
            || *value as ::core::ffi::c_int == 'y' as i32
            || *value as ::core::ffi::c_int == '1' as i32)) as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut ptexbanner: *const ::core::ffi::c_char =
    b"This is pdfTeX, Version 3.141592653-2.6-1.40.29\0" as *const u8 as *const ::core::ffi::c_char;
#[no_mangle]
pub unsafe extern "C" fn maininit(mut ac: ::core::ffi::c_int, mut av: *mut string) {
    let mut main_input_file: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    argc = ac;
    argv = av as *mut *mut ::core::ffi::c_char;
    interactionoption = 4 as ::core::ffi::c_uchar;
    kpse_def_inst.record_input =
        Some(recorder_record_input as unsafe extern "C" fn(const_string) -> ()) as p_record_input;
    kpse_def_inst.record_output =
        Some(recorder_record_output as unsafe extern "C" fn(const_string) -> ()) as p_record_output;
    parse_options(ac, av);
    if user_progname.is_null() {
        user_progname = dump_name;
    }
    kpse_set_program_name(
        *argv.offset(0 as ::core::ffi::c_int as isize) as const_string,
        user_progname,
    );
    xputenv(
        b"engine\0" as *const u8 as const_string,
        TEXMFENGINENAME.as_ptr(),
    );
    if !user_cnf_lines.is_null() {
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < user_cnf_nlines {
            kpathsea_cnf_line_env_progname(kpse_def, *user_cnf_lines.offset(i as isize));
            free(*user_cnf_lines.offset(i as isize) as *mut ::core::ffi::c_void);
            i = i.wrapping_add(1);
        }
    }
    main_input_file = get_input_file_name();
    if filelineerrorstylep < 0 as ::core::ffi::c_int {
        filelineerrorstylep = 0 as ::core::ffi::c_int;
    } else if filelineerrorstylep == 0 {
        filelineerrorstylep = texmf_yesno(b"file_line_error_style\0" as *const u8 as const_string)
            as ::core::ffi::c_int;
    }
    if parsefirstlinep < 0 as ::core::ffi::c_int {
        parsefirstlinep = 0 as ::core::ffi::c_int;
    } else if parsefirstlinep == 0 {
        parsefirstlinep =
            texmf_yesno(b"parse_first_line\0" as *const u8 as const_string) as ::core::ffi::c_int;
    }
    if parsefirstlinep != 0 && (dump_name.is_null() || translate_filename.is_null()) {
        parse_first_line(main_input_file as const_string);
    }
    if translate_filename.is_null() {
        translate_filename = default_translate_filename;
    }
    if readyalready != 314159 as ::core::ffi::c_int {
        let mut virversion: boolean = false_0;
        if !kpse_def_inst.program_name.is_null()
            && !(b"pdfinitex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                kpse_def_inst.program_name as *const ::core::ffi::c_char,
                b"pdfinitex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            iniversion = true_0 as boolean;
        } else if !kpse_def_inst.program_name.is_null()
            && !(b"pdfvirtex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                kpse_def_inst.program_name as *const ::core::ffi::c_char,
                b"pdfvirtex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            virversion = true_0 as boolean;
        } else if !kpse_def_inst.program_name.is_null()
            && !(b"initex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                kpse_def_inst.program_name as *const ::core::ffi::c_char,
                b"initex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            iniversion = true_0 as boolean;
        } else if !kpse_def_inst.program_name.is_null()
            && !(b"virtex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                kpse_def_inst.program_name as *const ::core::ffi::c_char,
                b"virtex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            virversion = true_0 as boolean;
        } else if !kpse_def_inst.program_name.is_null()
            && !(b"mltex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                kpse_def_inst.program_name as *const ::core::ffi::c_char,
                b"mltex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            mltexp = true_0 as boolean;
        }
        if main_input_file.is_null() {
            if !(*argv.offset(1 as ::core::ffi::c_int as isize)).is_null()
                && **argv.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '&' as i32
            {
                dump_name = (*argv.offset(1 as ::core::ffi::c_int as isize))
                    .offset(1 as ::core::ffi::c_int as isize)
                    as const_string;
            }
        }
        if dump_name.is_null() {
            dump_name = (if virversion != 0 {
                b"plain\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                kpse_def_inst.program_name as *const ::core::ffi::c_char
            }) as const_string;
        }
    }
    if iniversion == 0 {
        if mltexp != 0 {
            fprintf(
                __stderrp,
                b"-mltex only works with -ini\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if enctexp != 0 {
            fprintf(
                __stderrp,
                b"-enc only works with -ini\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        if etexp != 0 {
            fprintf(
                __stderrp,
                b"-etex only works with -ini\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    if !dump_name.is_null() {
        let mut with_ext: const_string = ::core::ptr::null::<::core::ffi::c_char>();
        let mut name_len: ::core::ffi::c_uint =
            strlen(dump_name as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
        let mut ext_len: ::core::ffi::c_uint = strlen(DUMP_EXT.as_ptr()) as ::core::ffi::c_uint;
        if name_len > ext_len
            && (!dump_name
                .offset(name_len as isize)
                .offset(-(ext_len as isize))
                .is_null()
                && !(b".fmt\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                && strcmp(
                    dump_name
                        .offset(name_len as isize)
                        .offset(-(ext_len as isize)),
                    b".fmt\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
        {
            with_ext = dump_name;
        } else {
            with_ext = concat(dump_name, DUMP_EXT.as_ptr()) as const_string;
        }
        TEXformatdefault = concat(b" \0" as *const u8 as const_string, with_ext);
        formatdefaultlength =
            strlen(TEXformatdefault.offset(1 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char) as integer;
    } else {
        abort();
    }
    kpse_set_program_enabled(kpse_tfm_format, MAKE_TEX_TFM_BY_DEFAULT, kpse_src_compile);
    kpse_set_program_enabled(kpse_tex_format, MAKE_TEX_TEX_BY_DEFAULT, kpse_src_compile);
    kpse_set_program_enabled(kpse_fmt_format, MAKE_TEX_FMT_BY_DEFAULT, kpse_src_compile);
    init_shell_escape();
    if outputcomment.is_null() {
        outputcomment = kpse_var_value(b"output_comment\0" as *const u8 as const_string)
            as *mut ::core::ffi::c_char;
    }
}
unsafe fn main_0(mut ac: ::core::ffi::c_int, mut av: *mut string) -> ::core::ffi::c_int {
    maininit(ac, av);
    mainbody();
    return EXIT_SUCCESS;
}
#[no_mangle]
pub unsafe extern "C" fn topenin() {
    let mut i: ::core::ffi::c_int = 0;
    *buffer.offset(first as isize) = 0 as ASCIIcode;
    if optind < argc {
        let mut k_0: ::core::ffi::c_int = first as ::core::ffi::c_int;
        i = optind;
        while i < argc {
            let mut ptr: *mut ::core::ffi::c_char = (*argv.offset(i as isize))
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_char;
            while *ptr != 0 {
                let fresh30 = ptr;
                ptr = ptr.offset(1);
                let fresh31 = k_0;
                k_0 = k_0 + 1;
                *buffer.offset(fresh31 as isize) = *fresh30 as ASCIIcode;
            }
            let fresh32 = k_0;
            k_0 = k_0 + 1;
            *buffer.offset(fresh32 as isize) = ' ' as i32 as ASCIIcode;
            i += 1;
        }
        argc = 0 as ::core::ffi::c_int;
        *buffer.offset(k_0 as isize) = 0 as ASCIIcode;
    }
    last = first;
    while *buffer.offset(last as isize) != 0 {
        last += 1;
    }
    last -= 1;
    while last >= first
        && (*buffer.offset(last as isize) as ::core::ffi::c_int == ' ' as i32
            || *buffer.offset(last as isize) as ::core::ffi::c_int == '\r' as i32
            || *buffer.offset(last as isize) as ::core::ffi::c_int == '\n' as i32)
    {
        last -= 1;
    }
    last += 1;
    i = first as ::core::ffi::c_int;
    while i < last {
        *buffer.offset(i as isize) = xord[*buffer.offset(i as isize) as usize];
        i += 1;
    }
}
pub const SOCK_STREAM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const AF_UNIX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IPC_AF: ::core::ffi::c_int = AF_UNIX;
pub const IPC_PIPE_NAME: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"/.TeXview_Pipe\0") };
pub const IPC_SERVER_CMD: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"open `which TeXview`\0")
};
static mut ipc_addr: *mut sockaddr = ::core::ptr::null::<sockaddr>() as *mut sockaddr;
static mut ipc_addr_len: ::core::ffi::c_int = 0;
unsafe extern "C" fn ipc_make_name() -> ::core::ffi::c_int {
    if ipc_addr_len == 0 as ::core::ffi::c_int {
        let mut s: string = getenv(b"HOME\0" as *const u8 as *const ::core::ffi::c_char) as string;
        if !s.is_null() {
            let mut ipc_name: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            ipc_addr = xmalloc(strlen(s as *const ::core::ffi::c_char).wrapping_add(40 as size_t))
                as *mut sockaddr;
            (*ipc_addr).sa_family = 0 as sa_family_t;
            ipc_name = &raw mut (*ipc_addr).sa_data as *mut ::core::ffi::c_char;
            strcpy(ipc_name, s as *const ::core::ffi::c_char);
            strcat(ipc_name, IPC_PIPE_NAME.as_ptr());
            ipc_addr_len = strlen(ipc_name).wrapping_add(3 as size_t) as ::core::ffi::c_int;
        }
    }
    return ipc_addr_len;
}
static mut sock: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
unsafe extern "C" fn ipc_is_open() -> ::core::ffi::c_int {
    return (sock != -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int;
}
unsafe extern "C" fn ipc_open_out() {
    if sock != -(1 as ::core::ffi::c_int) {
        return;
    }
    if ipc_make_name() <= 0 as ::core::ffi::c_int {
        return;
    }
    sock = socket(IPC_AF, SOCK_STREAM, 0 as ::core::ffi::c_int);
    if sock != -(1 as ::core::ffi::c_int) {
        if connect(sock, ipc_addr, ipc_addr_len as socklen_t) != 0 as ::core::ffi::c_int
            || fcntl(sock, F_SETFL, O_NONBLOCK) < 0 as ::core::ffi::c_int
        {
            close(sock);
            sock = -(1 as ::core::ffi::c_int);
            return;
        }
    }
}
unsafe extern "C" fn ipc_close_out() {
    if ipc_is_open() != 0 {
        close(sock);
        sock = -(1 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn ipc_snd(
    mut n: ::core::ffi::c_int,
    mut is_eof: ::core::ffi::c_int,
    mut data: *mut ::core::ffi::c_char,
) {
    let mut ourmsg: C2RustUnnamed = C2RustUnnamed {
        msg: msg {
            namelength: 0,
            eof: 0,
        },
        more_data: [0; 1024],
    };
    if ipc_is_open() == 0 {
        return;
    }
    ourmsg.msg.namelength = n;
    ourmsg.msg.eof = is_eof;
    if n != 0 {
        strcpy(&raw mut ourmsg.more_data as *mut ::core::ffi::c_char, data);
    }
    n = (n as ::core::ffi::c_ulong)
        .wrapping_add(::core::mem::size_of::<msg>() as usize as ::core::ffi::c_ulong)
        as ::core::ffi::c_int as ::core::ffi::c_int;
    if write(
        sock,
        &raw mut ourmsg as *const ::core::ffi::c_void,
        n as size_t,
    ) != n as ssize_t
    {
        ipc_close_out();
    }
}
#[no_mangle]
pub unsafe extern "C" fn ipcpage(mut is_eof: ::core::ffi::c_int) {
    static mut begun: boolean = false_0;
    let mut len: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut p: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if begun == 0 {
        let mut name: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut cwd: string = xgetcwd();
        ipc_open_out();
        len = (*strstart
            .offset((outputfilename as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            - *strstart.offset(outputfilename as isize)) as ::core::ffi::c_uint;
        name = xmalloc(len.wrapping_add(1 as ::core::ffi::c_uint) as size_t) as string;
        strncpy(
            name as *mut ::core::ffi::c_char,
            strpool.offset(*strstart.offset(outputfilename as isize) as isize)
                as *mut packedASCIIcode as string as *const ::core::ffi::c_char,
            len as size_t,
        );
        *name.offset(len as isize) = 0 as ::core::ffi::c_char;
        p = concat3(
            cwd as const_string,
            DIR_SEP_STRING.as_ptr(),
            name as const_string,
        );
        free(cwd as *mut ::core::ffi::c_void);
        free(name as *mut ::core::ffi::c_void);
        len = strlen(p as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
        begun = true_0 as boolean;
    }
    ipc_snd(
        len as ::core::ffi::c_int,
        is_eof,
        p as *mut ::core::ffi::c_char,
    );
    if !p.is_null() {
        free(p as *mut ::core::ffi::c_void);
    }
}
unsafe extern "C" fn tcx_get_num(
    mut upb: ::core::ffi::c_int,
    mut line_count: ::core::ffi::c_uint,
    mut start: string,
    mut post: *mut string,
) -> ::core::ffi::c_int {
    let mut num: ::core::ffi::c_int = strtol(
        start as *const ::core::ffi::c_char,
        post as *mut *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if !(!post.is_null() && !(*post).is_null()) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        __assert_rtn(
            b"tcx_get_num\0" as *const u8 as *const ::core::ffi::c_char,
            b"texmfmp.c\0" as *const u8 as *const ::core::ffi::c_char,
            1570 as ::core::ffi::c_int,
            b"post && *post\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
    };
    if *post == start {
        let mut p: string = start;
        while *p as ::core::ffi::c_int != 0
            && (*p as ::core::ffi::c_uint <= 127 as ::core::ffi::c_uint
                && isspace(*p as ::core::ffi::c_uchar as ::core::ffi::c_int) != 0)
        {
            p = p.offset(1);
        }
        if *p as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            fprintf(
                __stderrp,
                b"%s:%d: Expected numeric constant, not `%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                translate_filename,
                line_count,
                start,
            );
        }
        num = -(1 as ::core::ffi::c_int);
    } else if num < 0 as ::core::ffi::c_int || num > upb {
        fprintf(
            __stderrp,
            b"%s:%d: Destination charcode %d <0 or >%d.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            translate_filename,
            line_count,
            num,
            upb,
        );
        num = -(1 as ::core::ffi::c_int);
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn readtcxfile() {
    let mut orig_filename: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if find_suffix(translate_filename as const_string).is_null() {
        translate_filename = concat(
            translate_filename as const_string,
            b".tcx\0" as *const u8 as const_string,
        );
    }
    orig_filename = translate_filename;
    translate_filename = kpse_find_file(
        translate_filename as const_string,
        kpse_web2c_format,
        true_0,
    );
    if !translate_filename.is_null() {
        let mut line_0: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut line_count: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut translate_file: *mut FILE =
            xfopen(translate_filename as const_string, FOPEN_R_MODE.as_ptr());
        loop {
            line_0 = read_line(translate_file);
            if line_0.is_null() {
                break;
            }
            let mut first_0: ::core::ffi::c_int = 0;
            let mut start2: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut comment_loc: string =
                strchr(line_0 as *const ::core::ffi::c_char, '%' as i32) as string;
            if !comment_loc.is_null() {
                *comment_loc = 0 as ::core::ffi::c_char;
            }
            line_count = line_count.wrapping_add(1);
            first_0 = tcx_get_num(
                255 as ::core::ffi::c_int,
                line_count,
                line_0,
                &raw mut start2,
            );
            if first_0 >= 0 as ::core::ffi::c_int {
                let mut start3: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut second: ::core::ffi::c_int = 0;
                let mut printable: ::core::ffi::c_int = 0;
                second = tcx_get_num(
                    255 as ::core::ffi::c_int,
                    line_count,
                    start2,
                    &raw mut start3,
                );
                if second >= 0 as ::core::ffi::c_int {
                    let mut extra: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    xord[first_0 as usize] = second as ASCIIcode;
                    xchr[second as usize] = first_0 as ASCIIcode;
                    printable =
                        tcx_get_num(1 as ::core::ffi::c_int, line_count, start3, &raw mut extra);
                    if printable == -(1 as ::core::ffi::c_int) {
                        printable = 1 as ::core::ffi::c_int;
                    }
                    if 32 as ::core::ffi::c_int <= second && second <= 126 as ::core::ffi::c_int {
                        printable = 1 as ::core::ffi::c_int;
                    }
                } else {
                    second = first_0;
                    printable = 1 as ::core::ffi::c_int;
                }
                xprn[second as usize] = printable as ASCIIcode;
            }
            free(line_0 as *mut ::core::ffi::c_void);
        }
        xfclose(translate_file, translate_filename as const_string);
    } else {
        fputs(
            b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
            __stderrp,
        );
        fprintf(
            __stderrp,
            b"Could not open char translation file `%s'\0" as *const u8
                as *const ::core::ffi::c_char,
            orig_filename,
        );
        fputs(
            b".\n\0" as *const u8 as *const ::core::ffi::c_char,
            __stderrp,
        );
        fflush(__stderrp);
    };
}
unsafe extern "C" fn normalize_quotes(mut name: const_string, mut mesg: const_string) -> string {
    let mut quoted: boolean = false_0;
    let mut must_quote: boolean = (strchr(name as *const ::core::ffi::c_char, ' ' as i32)
        != NULL as *mut ::core::ffi::c_char)
        as ::core::ffi::c_int;
    let mut ret: string =
        xmalloc(strlen(name as *const ::core::ffi::c_char).wrapping_add(3 as size_t)) as string;
    let mut p: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: const_string = ::core::ptr::null::<::core::ffi::c_char>();
    p = ret;
    if must_quote != 0 {
        let fresh25 = p;
        p = p.offset(1);
        *fresh25 = '"' as i32 as ::core::ffi::c_char;
    }
    q = name;
    while *q != 0 {
        if *q as ::core::ffi::c_int == '"' as i32 {
            quoted = (quoted == 0) as ::core::ffi::c_int as boolean;
        } else {
            let fresh26 = p;
            p = p.offset(1);
            *fresh26 = *q;
        }
        q = q.offset(1);
    }
    if must_quote != 0 {
        let fresh27 = p;
        p = p.offset(1);
        *fresh27 = '"' as i32 as ::core::ffi::c_char;
    }
    *p = '\0' as i32 as ::core::ffi::c_char;
    if quoted != 0 {
        fprintf(
            __stderrp,
            b"! Unbalanced quotes in %s %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            mesg,
            name,
        );
        uexit(1 as ::core::ffi::c_int);
    }
    return ret;
}
unsafe extern "C" fn get_input_file_name() -> string {
    let mut input_file_name: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*argv.offset(optind as isize)).is_null()
        && *(*argv.offset(optind as isize)).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            != '&' as i32
        && *(*argv.offset(optind as isize)).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            != '\\' as i32
    {
        let mut name: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut quoted: boolean = 0;
        name = normalize_quotes(
            *argv.offset(optind as isize) as const_string,
            b"argument\0" as *const u8 as const_string,
        );
        quoted = (*name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '"' as i32) as ::core::ffi::c_int as boolean;
        if quoted != 0 {
            *name.offset(
                strlen(name as *const ::core::ffi::c_char).wrapping_sub(1 as size_t) as isize,
            ) = '\0' as i32 as ::core::ffi::c_char;
            name = name.offset(1);
        }
        input_file_name = kpse_find_file(name as const_string, kpse_tex_format, false_0);
        if quoted != 0 {
            *name.offset(strlen(name as *const ::core::ffi::c_char) as isize) =
                '"' as i32 as ::core::ffi::c_char;
            name = name.offset(-1);
        }
        let ref mut fresh24 = *argv.offset(optind as isize);
        *fresh24 = name as *mut ::core::ffi::c_char;
    }
    return input_file_name;
}
static mut long_options: [option; 40] = unsafe {
    [
        option {
            name: DUMP_OPTION.as_ptr(),
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"efmt\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"cnf-line\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"help\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"ini\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const iniversion as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"interaction\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"halt-on-error\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const haltonerrorp as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"kpathsea-debug\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"progname\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"recorder\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const recorder_enabled as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"version\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"ipc\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const ipcon as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"ipc-start\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const ipcon as *mut ::core::ffi::c_int,
            val: 2 as ::core::ffi::c_int,
        },
        option {
            name: b"mltex\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const mltexp as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"enc\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const enctexp as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"etex\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const etexp as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"output-comment\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"draftmode\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"output-format\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"shell-escape\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const shellenabledp as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"no-shell-escape\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const shellenabledp as *mut ::core::ffi::c_int,
            val: -(1 as ::core::ffi::c_int),
        },
        option {
            name: b"enable-write18\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const shellenabledp as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"disable-write18\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const shellenabledp as *mut ::core::ffi::c_int,
            val: -(1 as ::core::ffi::c_int),
        },
        option {
            name: b"shell-restricted\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"debug-format\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const debugformatfile as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"src-specials\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 2 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"file-line-error-style\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const filelineerrorstylep as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"no-file-line-error-style\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const filelineerrorstylep as *mut ::core::ffi::c_int,
            val: -(1 as ::core::ffi::c_int),
        },
        option {
            name: b"file-line-error\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const filelineerrorstylep as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"no-file-line-error\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const filelineerrorstylep as *mut ::core::ffi::c_int,
            val: -(1 as ::core::ffi::c_int),
        },
        option {
            name: b"jobname\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"output-directory\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"parse-first-line\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const parsefirstlinep as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"no-parse-first-line\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const parsefirstlinep as *mut ::core::ffi::c_int,
            val: -(1 as ::core::ffi::c_int),
        },
        option {
            name: b"translate-file\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"default-translate-file\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"8bit\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const eightbitp as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"mktex\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: b"no-mktex\0" as *const u8 as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null::<::core::ffi::c_int>() as *mut ::core::ffi::c_int,
            val: 0 as ::core::ffi::c_int,
        },
    ]
};
unsafe extern "C" fn parse_options(mut argc_0: ::core::ffi::c_int, mut argv_0: *mut string) {
    let mut g_0: ::core::ffi::c_int = 0;
    let mut option_index: ::core::ffi::c_int = 0;
    loop {
        g_0 = getopt_long_only(
            argc_0,
            argv_0,
            b"+\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut long_options as *mut option,
            &raw mut option_index,
        );
        if g_0 == -(1 as ::core::ffi::c_int) {
            break;
        }
        if g_0 == '?' as i32 {
            continue;
        }
        if !(g_0 == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            __assert_rtn(
                b"parse_options\0" as *const u8 as *const ::core::ffi::c_char,
                b"texmfmp.c\0" as *const u8 as *const ::core::ffi::c_char,
                1884 as ::core::ffi::c_int,
                b"g == 0\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
        };
        if !long_options[option_index as usize].name.is_null()
            && !(b"kpathsea-debug\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"kpathsea-debug\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            kpse_def_inst.debug |= atoi(optarg) as ::core::ffi::c_uint;
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"progname\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"progname\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            user_progname = optarg as const_string;
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"cnf-line\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"cnf-line\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            if user_cnf_lines.is_null() {
                user_cnf_nlines = 1 as ::core::ffi::c_uint;
                user_cnf_lines =
                    xmalloc(::core::mem::size_of::<const_string>() as size_t) as *mut string;
            } else {
                user_cnf_nlines = user_cnf_nlines.wrapping_add(1);
                user_cnf_lines = xrealloc(
                    user_cnf_lines as address,
                    (user_cnf_nlines as size_t)
                        .wrapping_mul(::core::mem::size_of::<const_string>() as size_t),
                ) as *mut string;
            }
            let ref mut fresh28 = *user_cnf_lines
                .offset(user_cnf_nlines.wrapping_sub(1 as ::core::ffi::c_uint) as isize);
            *fresh28 = xstrdup(optarg as const_string);
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"jobname\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"jobname\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            c_job_name = normalize_quotes(
                optarg as const_string,
                b"jobname\0" as *const u8 as const_string,
            ) as const_string;
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"fmt\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"fmt\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            dump_name = optarg as const_string;
            dumpoption = true_0 as boolean;
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"efmt\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"efmt\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            dump_name = optarg as const_string;
            dumpoption = true_0 as boolean;
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"output-directory\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"output-directory\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            output_directory = optarg as string;
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"output-comment\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"output-comment\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            let mut len: ::core::ffi::c_uint = strlen(optarg) as ::core::ffi::c_uint;
            if len < 256 as ::core::ffi::c_uint {
                outputcomment = optarg;
            } else {
                fputs(
                    b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                    __stderrp,
                );
                fprintf(
                    __stderrp,
                    b"Comment truncated to 255 characters from %d. (%s)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    len,
                    optarg,
                );
                fputs(
                    b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                    __stderrp,
                );
                fflush(__stderrp);
                outputcomment = xmalloc(256 as size_t) as *mut ::core::ffi::c_char;
                strncpy(outputcomment, optarg, 255 as size_t);
                *outputcomment.offset(255 as ::core::ffi::c_int as isize) =
                    0 as ::core::ffi::c_char;
            }
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"ipc-start\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"ipc-start\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            ipc_open_out();
            if ipc_is_open() == 0 {
                if system(IPC_SERVER_CMD.as_ptr()) == 0 as ::core::ffi::c_int {
                    let mut i: ::core::ffi::c_uint = 0;
                    i = 0 as ::core::ffi::c_uint;
                    while i < 20 as ::core::ffi::c_uint && ipc_is_open() == 0 {
                        sleep(2 as ::core::ffi::c_uint);
                        ipc_open_out();
                        i = i.wrapping_add(1);
                    }
                }
            }
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"shell-restricted\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"shell-restricted\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            shellenabledp = 1 as ::core::ffi::c_int;
            restrictedshell = 1 as ::core::ffi::c_int;
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"src-specials\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"src-specials\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            last_source_name =
                xstrdup(b"\0" as *const u8 as const_string) as *mut ::core::ffi::c_char;
            if optarg.is_null() {
                insertsrcspecialeverypar = true_0 as boolean;
                insertsrcspecialauto = true_0 as boolean;
                srcspecialsoption = true_0 as boolean;
                srcspecialsp = true_0 as boolean;
            } else {
                parse_src_specials_option(optarg as const_string);
            }
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"output-format\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"output-format\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            pdfoutputoption = 1 as ::core::ffi::c_int as integer;
            if strcmp(optarg, b"dvi\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                pdfoutputvalue = 0 as ::core::ffi::c_int as integer;
            } else if strcmp(optarg, b"pdf\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                pdfoutputvalue = 2 as ::core::ffi::c_int as integer;
            } else {
                fputs(
                    b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                    __stderrp,
                );
                fprintf(
                    __stderrp,
                    b"Ignoring unknown value `%s' for --output-format\0" as *const u8
                        as *const ::core::ffi::c_char,
                    optarg,
                );
                fputs(
                    b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                    __stderrp,
                );
                fflush(__stderrp);
                pdfoutputoption = 0 as ::core::ffi::c_int as integer;
            }
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"draftmode\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"draftmode\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            pdfdraftmodeoption = 1 as ::core::ffi::c_int as integer;
            pdfdraftmodevalue = 1 as ::core::ffi::c_int as integer;
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"translate-file\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"translate-file\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            translate_filename = optarg as string;
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"default-translate-file\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"default-translate-file\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            default_translate_filename = optarg as string;
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"mktex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"mktex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            kpse_maketex_option(optarg as const_string, true_0);
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"no-mktex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"no-mktex\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            kpse_maketex_option(optarg as const_string, false_0);
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"interaction\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"interaction\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            if !optarg.is_null()
                && !(b"batchmode\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                && strcmp(
                    optarg,
                    b"batchmode\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                interactionoption = 0 as ::core::ffi::c_uchar;
            } else if !optarg.is_null()
                && !(b"nonstopmode\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                && strcmp(
                    optarg,
                    b"nonstopmode\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                interactionoption = 1 as ::core::ffi::c_uchar;
            } else if !optarg.is_null()
                && !(b"scrollmode\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                && strcmp(
                    optarg,
                    b"scrollmode\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                interactionoption = 2 as ::core::ffi::c_uchar;
            } else if !optarg.is_null()
                && !(b"errorstopmode\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                && strcmp(
                    optarg,
                    b"errorstopmode\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                interactionoption = 3 as ::core::ffi::c_uchar;
            } else {
                fputs(
                    b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                    __stderrp,
                );
                fprintf(
                    __stderrp,
                    b"Ignoring unknown argument `%s' to --interaction\0" as *const u8
                        as *const ::core::ffi::c_char,
                    optarg,
                );
                fputs(
                    b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                    __stderrp,
                );
                fflush(__stderrp);
            }
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"help\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"help\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            usagehelp(
                &raw mut PDFTEXHELP as *mut const_string,
                BUG_ADDRESS.as_ptr(),
            );
        } else if !long_options[option_index as usize].name.is_null()
            && !(b"version\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                long_options[option_index as usize].name,
                b"version\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            let mut versions: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            initversionstring(&raw mut versions);
            printversionandexit(
                b"This is pdfTeX, Version 3.141592653-2.6-1.40.29\0" as *const u8 as const_string,
                COPYRIGHT_HOLDER.as_ptr(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                versions as const_string,
            );
        }
    }
    if !output_directory.is_null() {
        xputenv(
            b"TEXMF_OUTPUT_DIRECTORY\0" as *const u8 as const_string,
            output_directory as const_string,
        );
    } else if !getenv(b"TEXMF_OUTPUT_DIRECTORY\0" as *const u8 as *const ::core::ffi::c_char)
        .is_null()
    {
        output_directory =
            getenv(b"TEXMF_OUTPUT_DIRECTORY\0" as *const u8 as *const ::core::ffi::c_char)
                as string;
    }
}
unsafe extern "C" fn parse_src_specials_option(mut opt_list: const_string) {
    let mut toklist: *mut ::core::ffi::c_char = xstrdup(opt_list) as *mut ::core::ffi::c_char;
    let mut tok: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    insertsrcspecialauto = false_0 as boolean;
    tok = strtok(toklist, b", \0" as *const u8 as *const ::core::ffi::c_char);
    while !tok.is_null() {
        if strcmp(
            tok,
            b"everypar\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(tok, b"par\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            || strcmp(tok, b"auto\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            insertsrcspecialauto = true_0 as boolean;
            insertsrcspecialeverypar = true_0 as boolean;
        } else if strcmp(
            tok,
            b"everyparend\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(tok, b"parend\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            insertsrcspecialeveryparend = true_0 as boolean;
        } else if strcmp(tok, b"everycr\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(tok, b"cr\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            insertsrcspecialeverycr = true_0 as boolean;
        } else if strcmp(
            tok,
            b"everymath\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(tok, b"math\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            insertsrcspecialeverymath = true_0 as boolean;
        } else if strcmp(
            tok,
            b"everyhbox\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(tok, b"hbox\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            insertsrcspecialeveryhbox = true_0 as boolean;
        } else if strcmp(
            tok,
            b"everyvbox\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(tok, b"vbox\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            insertsrcspecialeveryvbox = true_0 as boolean;
        } else if strcmp(
            tok,
            b"everydisplay\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(tok, b"display\0" as *const u8 as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            insertsrcspecialeverydisplay = true_0 as boolean;
        } else if strcmp(tok, b"none\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            insertsrcspecialeverydisplay = false_0 as boolean;
            insertsrcspecialeveryvbox = insertsrcspecialeverydisplay;
            insertsrcspecialeveryhbox = insertsrcspecialeveryvbox;
            insertsrcspecialeverymath = insertsrcspecialeveryhbox;
            insertsrcspecialeverycr = insertsrcspecialeverymath;
            insertsrcspecialeveryparend = insertsrcspecialeverycr;
            insertsrcspecialeverypar = insertsrcspecialeveryparend;
            insertsrcspecialauto = insertsrcspecialeverypar;
        } else {
            fputs(
                b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                __stderrp,
            );
            fprintf(
                __stderrp,
                b"Ignoring unknown argument `%s' to --src-specials\0" as *const u8
                    as *const ::core::ffi::c_char,
                tok,
            );
            fputs(
                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                __stderrp,
            );
            fflush(__stderrp);
        }
        tok = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b", \0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    free(toklist as *mut ::core::ffi::c_void);
    srcspecialsp = insertsrcspecialauto
        | insertsrcspecialeverypar
        | insertsrcspecialeveryparend
        | insertsrcspecialeverycr
        | insertsrcspecialeverymath
        | insertsrcspecialeveryhbox
        | insertsrcspecialeveryvbox
        | insertsrcspecialeverydisplay;
    srcspecialsoption = true_0 as boolean;
}
unsafe extern "C" fn parse_first_line(mut filename: const_string) {
    let mut f_0: *mut FILE = if !filename.is_null() {
        fopen(
            filename as *const ::core::ffi::c_char,
            FOPEN_R_MODE.as_ptr(),
        ) as *mut FILE
    } else {
        ::core::ptr::null_mut::<FILE>()
    };
    if !f_0.is_null() {
        let mut first_line: string = read_line(f_0);
        xfclose(f_0, filename);
        if !first_line.is_null()
            && *first_line.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '%' as i32
            && *first_line.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '&' as i32
        {
            let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut part: [*mut ::core::ffi::c_char; 4] =
                [::core::ptr::null_mut::<::core::ffi::c_char>(); 4];
            let mut npart: ::core::ffi::c_int = 0;
            let mut parse: *mut *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
            s = first_line.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char;
            while *s as ::core::ffi::c_uint <= 127 as ::core::ffi::c_uint
                && (*s as ::core::ffi::c_uchar as ::core::ffi::c_int == ' ' as i32
                    || *s as ::core::ffi::c_uchar as ::core::ffi::c_int == '\t' as i32)
            {
                s = s.offset(1);
            }
            npart = 0 as ::core::ffi::c_int;
            while *s as ::core::ffi::c_int != 0 && npart != 3 as ::core::ffi::c_int {
                let fresh22 = npart;
                npart = npart + 1;
                part[fresh22 as usize] = s;
                while *s as ::core::ffi::c_int != 0 && *s as ::core::ffi::c_int != ' ' as i32 {
                    s = s.offset(1);
                }
                while *s as ::core::ffi::c_int == ' ' as i32 {
                    let fresh23 = s;
                    s = s.offset(1);
                    *fresh23 = '\0' as i32 as ::core::ffi::c_char;
                }
            }
            part[npart as usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
            parse = &raw mut part as *mut *mut ::core::ffi::c_char;
            if !(*parse).is_null() && **parse as ::core::ffi::c_int != '-' as i32 {
                if dump_name.is_null() {
                    let mut f_name: string = concat(
                        part[0 as ::core::ffi::c_int as usize] as const_string,
                        DUMP_EXT.as_ptr(),
                    );
                    let mut d_name: string =
                        kpse_find_file(f_name as const_string, kpse_fmt_format, false_0);
                    if !d_name.is_null() && !kpse_readable_file(d_name).is_null() {
                        dump_name = xstrdup(part[0 as ::core::ffi::c_int as usize] as const_string)
                            as const_string;
                        kpse_reset_program_name(dump_name);
                        dumpline = true_0 as boolean;
                    }
                    free(f_name as *mut ::core::ffi::c_void);
                }
                parse = parse.offset(1);
            }
            if !(*parse).is_null() {
                s = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if translate_filename.is_null() {
                    if !(*parse).is_null()
                        && !(b"--translate-file\0" as *const u8 as *const ::core::ffi::c_char)
                            .is_null()
                        && strcmp(
                            *parse,
                            b"--translate-file\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        s = *parse.offset(1 as ::core::ffi::c_int as isize);
                    } else if !(*parse).is_null()
                        && !(b"-translate-file\0" as *const u8 as *const ::core::ffi::c_char)
                            .is_null()
                        && strcmp(
                            *parse,
                            b"-translate-file\0" as *const u8 as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        s = *parse.offset(1 as ::core::ffi::c_int as isize);
                    } else if !(*parse).is_null()
                        && !(b"--translate-file=\0" as *const u8 as *const ::core::ffi::c_char)
                            .is_null()
                        && strncmp(
                            *parse,
                            b"--translate-file=\0" as *const u8 as *const ::core::ffi::c_char,
                            17 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        s = (*parse).offset(17 as ::core::ffi::c_int as isize);
                    } else if !(*parse).is_null()
                        && !(b"-translate-file=\0" as *const u8 as *const ::core::ffi::c_char)
                            .is_null()
                        && strncmp(
                            *parse,
                            b"-translate-file=\0" as *const u8 as *const ::core::ffi::c_char,
                            16 as size_t,
                        ) == 0 as ::core::ffi::c_int
                    {
                        s = (*parse).offset(16 as ::core::ffi::c_int as isize);
                    }
                }
                if !s.is_null() && *s as ::core::ffi::c_int != 0 {
                    translate_filename = xstrdup(s as const_string);
                }
            }
        }
        if !first_line.is_null() {
            free(first_line as *mut ::core::ffi::c_void);
        }
    }
}
pub const NUM_PIPES: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
static mut pipes: [*mut FILE; 16] = [::core::ptr::null::<FILE>() as *mut FILE; 16];
#[no_mangle]
pub unsafe extern "C" fn open_in_or_pipe(
    mut f_ptr: *mut *mut FILE,
    mut filefmt: ::core::ffi::c_int,
    mut fopen_mode: const_string,
) -> boolean {
    let mut fname: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if shellenabledp != 0
        && *nameoffile.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '|' as i32
    {
        *f_ptr = ::core::ptr::null_mut::<FILE>();
        fname = xmalloc(
            strlen(
                nameoffile.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            )
            .wrapping_add(1 as size_t),
        ) as string;
        strcpy(
            fname as *mut ::core::ffi::c_char,
            nameoffile.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
        );
        if !fullnameoffile.is_null() {
            free(fullnameoffile as *mut ::core::ffi::c_void);
        }
        fullnameoffile = xstrdup(fname as const_string);
        recorder_record_input(fname.offset(1 as ::core::ffi::c_int as isize) as const_string);
        *f_ptr = runpopen(
            fname.offset(1 as ::core::ffi::c_int as isize),
            b"r\0" as *const u8 as *const ::core::ffi::c_char,
        );
        free(fname as *mut ::core::ffi::c_void);
        i = 0 as ::core::ffi::c_int;
        while i < NUM_PIPES {
            if pipes[i as usize].is_null() {
                pipes[i as usize] = *f_ptr;
                break;
            } else {
                i += 1;
            }
        }
        if !(*f_ptr).is_null() {
            setvbuf(
                *f_ptr,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                _IONBF,
                0 as size_t,
            );
        }
        return (*f_ptr != NULL as *mut FILE) as ::core::ffi::c_int;
    }
    return open_input(f_ptr, filefmt, fopen_mode);
}
#[no_mangle]
pub unsafe extern "C" fn open_out_or_pipe(
    mut f_ptr: *mut *mut FILE,
    mut fopen_mode: const_string,
) -> boolean {
    let mut fname: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if shellenabledp != 0
        && *nameoffile.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '|' as i32
    {
        fname = xmalloc(
            strlen(
                nameoffile.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
            )
            .wrapping_add(1 as size_t),
        ) as string;
        strcpy(
            fname as *mut ::core::ffi::c_char,
            nameoffile.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char,
        );
        if strchr(fname as *const ::core::ffi::c_char, ' ' as i32).is_null()
            && strchr(fname as *const ::core::ffi::c_char, '>' as i32).is_null()
        {
            if !fname
                .offset(strlen(fname as *const ::core::ffi::c_char) as isize)
                .offset(-(4 as ::core::ffi::c_int as isize))
                .is_null()
                && !(b".tex\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                && strcmp(
                    fname
                        .offset(strlen(fname as *const ::core::ffi::c_char) as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize))
                        as *const ::core::ffi::c_char,
                    b".tex\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
            {
                *fname
                    .offset(strlen(fname as *const ::core::ffi::c_char) as isize)
                    .offset(-(4 as ::core::ffi::c_int as isize)) = 0 as ::core::ffi::c_char;
            }
            *f_ptr = runpopen(
                fname.offset(1 as ::core::ffi::c_int as isize),
                b"w\0" as *const u8 as *const ::core::ffi::c_char,
            );
            *fname.offset(strlen(fname as *const ::core::ffi::c_char) as isize) =
                '.' as i32 as ::core::ffi::c_char;
        } else {
            *f_ptr = runpopen(
                fname.offset(1 as ::core::ffi::c_int as isize),
                b"w\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        recorder_record_output(fname.offset(1 as ::core::ffi::c_int as isize) as const_string);
        free(fname as *mut ::core::ffi::c_void);
        i = 0 as ::core::ffi::c_int;
        while i < NUM_PIPES {
            if pipes[i as usize].is_null() {
                pipes[i as usize] = *f_ptr;
                break;
            } else {
                i += 1;
            }
        }
        if !(*f_ptr).is_null() {
            setvbuf(
                *f_ptr,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                _IONBF,
                0 as size_t,
            );
        }
        return (*f_ptr != NULL as *mut FILE) as ::core::ffi::c_int;
    }
    return open_output(f_ptr, fopen_mode);
}
#[no_mangle]
pub unsafe extern "C" fn close_file_or_pipe(mut f_0: *mut FILE) {
    let mut i: ::core::ffi::c_int = 0;
    if shellenabledp != 0 {
        i = 0 as ::core::ffi::c_int;
        while i < NUM_PIPES {
            if pipes[i as usize] == f_0 {
                if !f_0.is_null() {
                    pclose(f_0);
                }
                pipes[i as usize] = ::core::ptr::null_mut::<FILE>();
                return;
            }
            i += 1;
        }
    }
    close_file(f_0);
}
unsafe extern "C" fn catch_interrupt(mut arg: ::core::ffi::c_int) {
    interrupt = 1 as ::core::ffi::c_int as integer;
    signal(
        SIGINT,
        Some(catch_interrupt as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
}
static mut start_time_set: boolean = false_0;
static mut start_time: time_t = 0 as time_t;
static mut SOURCE_DATE_EPOCH_set: boolean = false_0;
static mut FORCE_SOURCE_DATE_set: boolean = false_0;
#[no_mangle]
pub unsafe extern "C" fn init_start_time() {
    let mut source_date_epoch: *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut epoch: ::core::ffi::c_ulonglong = 0;
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if start_time_set == 0 {
        start_time_set = true_0 as boolean;
        source_date_epoch =
            getenv(b"SOURCE_DATE_EPOCH\0" as *const u8 as *const ::core::ffi::c_char);
        if !source_date_epoch.is_null() {
            *__error() = 0 as ::core::ffi::c_int;
            epoch = strtoull(source_date_epoch, &raw mut endptr, 10 as ::core::ffi::c_int);
            if *endptr as ::core::ffi::c_int != '\0' as i32 || *__error() != 0 as ::core::ffi::c_int
            {
                fprintf(
                    __stderrp,
                    b"%s: fatal: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                    (*kpse_def).invocation_name,
                );
                fprintf(
                    __stderrp,
                    b"invalid epoch-seconds-timezone value for environment variable $SOURCE_DATE_EPOCH: %s\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    source_date_epoch,
                );
                fputs(
                    b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                    __stderrp,
                );
                exit(1 as ::core::ffi::c_int);
            }
            start_time = epoch as time_t;
            SOURCE_DATE_EPOCH_set = true_0 as boolean;
        } else {
            start_time = time(NULL as *mut time_t);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_date_and_time(
    mut minutes: *mut integer,
    mut day: *mut integer,
    mut month: *mut integer,
    mut year: *mut integer,
) {
    let mut tmptr: *mut tm = ::core::ptr::null_mut::<tm>();
    let mut sde_texprim: string =
        getenv(b"FORCE_SOURCE_DATE\0" as *const u8 as *const ::core::ffi::c_char) as string;
    if !sde_texprim.is_null()
        && (!sde_texprim.is_null()
            && !(b"1\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                sde_texprim as *const ::core::ffi::c_char,
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int)
    {
        init_start_time();
        tmptr = gmtime(&raw mut start_time);
        FORCE_SOURCE_DATE_set = true_0 as boolean;
    } else {
        let mut myclock: time_t = time(::core::ptr::null_mut::<time_t>());
        tmptr = localtime(&raw mut myclock);
        if !sde_texprim.is_null()
            && strlen(sde_texprim as *const ::core::ffi::c_char) > 0 as size_t
            && !(!sde_texprim.is_null()
                && !(b"0\0" as *const u8 as *const ::core::ffi::c_char).is_null()
                && strcmp(
                    sde_texprim as *const ::core::ffi::c_char,
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int)
        {
            fputs(
                b"warning: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                __stderrp,
            );
            fprintf(
                __stderrp,
                b"invalid value (expected 0 or 1) for environment variable $FORCE_SOURCE_DATE: %s\0"
                    as *const u8 as *const ::core::ffi::c_char,
                sde_texprim,
            );
            fputs(
                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                __stderrp,
            );
            fflush(__stderrp);
        }
    }
    *minutes = ((*tmptr).tm_hour * 60 as ::core::ffi::c_int + (*tmptr).tm_min) as integer;
    *day = (*tmptr).tm_mday as integer;
    *month = ((*tmptr).tm_mon + 1 as ::core::ffi::c_int) as integer;
    *year = ((*tmptr).tm_year + 1900 as ::core::ffi::c_int) as integer;
    let mut old_handler: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()> = None;
    old_handler = signal(
        SIGINT,
        Some(catch_interrupt as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    if old_handler.is_some() {
        signal(SIGINT, old_handler);
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_seconds_and_micros(
    mut seconds: *mut integer,
    mut micros: *mut integer,
) {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&raw mut tv, NULL);
    *seconds = tv.tv_sec as integer;
    *micros = tv.tv_usec as integer;
}
#[no_mangle]
pub unsafe extern "C" fn input_line(mut f_0: *mut FILE) -> boolean {
    let mut i: ::core::ffi::c_int = EOF;
    last = first;
    loop {
        *__error() = 0 as ::core::ffi::c_int;
        while last < bufsize
            && {
                i = fgetc(f_0);
                i != EOF
            }
            && i != '\n' as i32
            && i != '\r' as i32
        {
            let fresh29 = last;
            last = last + 1;
            *buffer.offset(fresh29 as isize) = i as ASCIIcode;
        }
        if !(i == EOF && *__error() == EINTR) {
            break;
        }
    }
    if i == EOF && last == first {
        return false_0;
    }
    if i != EOF && i != '\n' as i32 && i != '\r' as i32 {
        fprintf(
            __stderrp,
            b"! Unable to read an entire line---bufsize=%u.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            bufsize as ::core::ffi::c_uint,
        );
        fputs(
            b"Please increase buf_size in texmf.cnf.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            __stderrp,
        );
        uexit(1 as ::core::ffi::c_int);
    }
    *buffer.offset(last as isize) = ' ' as i32 as ASCIIcode;
    if last >= maxbufstack {
        maxbufstack = last;
    }
    if i == '\r' as i32 {
        loop {
            i = fgetc(f_0);
            if !(i == EOF && *__error() == EINTR) {
                break;
            }
        }
        if i != '\n' as i32 {
            ungetc(i, f_0);
        }
    }
    while last > first
        && *buffer.offset((last as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            == ' ' as i32
    {
        last -= 1;
    }
    i = first as ::core::ffi::c_int;
    while i <= last {
        *buffer.offset(i as isize) = xord[*buffer.offset(i as isize) as usize];
        i += 1;
    }
    return true_0;
}
static mut edit_value: const_string = EDITOR.as_ptr();
#[no_mangle]
pub unsafe extern "C" fn calledit(
    mut filename: *mut packedASCIIcode,
    mut fnstart: poolpointer,
    mut fnlength: integer,
    mut linenumber: integer,
) {
    let mut temp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut command: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fullcmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut c_0: ::core::ffi::c_char = 0;
    let mut sdone: ::core::ffi::c_int = 0;
    let mut ddone: ::core::ffi::c_int = 0;
    ddone = 0 as ::core::ffi::c_int;
    sdone = ddone;
    filename = filename.offset(fnstart as isize);
    let mut is_ptr: ::core::ffi::c_int = 0;
    is_ptr = 0 as ::core::ffi::c_int;
    while is_ptr < inputptr {
        if !((*inputstack.offset(is_ptr as isize)).statefield as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || (*inputstack.offset(is_ptr as isize)).namefield <= 255 as ::core::ffi::c_int)
        {
            let mut f_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
            let mut if_ptr: ::core::ffi::c_int =
                (*inputstack.offset(is_ptr as isize)).indexfield as ::core::ffi::c_int;
            if if_ptr < 1 as ::core::ffi::c_int || if_ptr > inopen {
                fprintf(
                    __stderrp,
                    b"%s:calledit: unexpected if_ptr=%d not in range 1..%d,\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                    if_ptr,
                    inopen,
                );
                fprintf(
                    __stderrp,
                    b"from input_stack[%d].namefield=%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    is_ptr,
                    (*inputstack.offset(is_ptr as isize)).namefield,
                );
                exit(1 as ::core::ffi::c_int);
            }
            f_0 = *inputfile.offset(if_ptr as isize) as *mut FILE;
            if !f_0.is_null() {
                xfclose(f_0, b"inputfile\0" as *const u8 as const_string);
            } else {
                fprintf(
                    __stderrp,
                    b"%s:calledit: not closing unexpected zero\0" as *const u8
                        as *const ::core::ffi::c_char,
                    *argv.offset(0 as ::core::ffi::c_int as isize),
                );
                fprintf(
                    __stderrp,
                    b" input_file[%d] from input_stack[%d].namefield=%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    if_ptr,
                    is_ptr,
                    (*inputstack.offset(is_ptr as isize)).namefield,
                );
            }
        }
        is_ptr += 1;
    }
    temp = kpse_var_value(edit_var.as_ptr()) as *mut ::core::ffi::c_char;
    if !temp.is_null() {
        edit_value = temp as const_string;
    }
    command = xmalloc(
        strlen(edit_value as *const ::core::ffi::c_char)
            .wrapping_add(fnlength as size_t)
            .wrapping_add(11 as size_t),
    ) as *mut ::core::ffi::c_char;
    temp = command;
    loop {
        let fresh37 = edit_value;
        edit_value = edit_value.offset(1);
        c_0 = *fresh37;
        if !(c_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
        if c_0 as ::core::ffi::c_int == '%' as i32 {
            let mut i: ::core::ffi::c_int = 0;
            let fresh38 = edit_value;
            edit_value = edit_value.offset(1);
            c_0 = *fresh38;
            match c_0 as ::core::ffi::c_int {
                100 => {
                    if ddone != 0 {
                        fprintf(
                            __stderrp,
                            b"%s: fatal: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                            (*kpse_def).invocation_name,
                        );
                        fputs(
                            b"call_edit: `%%d' appears twice in editor command\0" as *const u8
                                as *const ::core::ffi::c_char,
                            __stderrp,
                        );
                        fputs(
                            b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                            __stderrp,
                        );
                        exit(1 as ::core::ffi::c_int);
                    }
                    sprintf(
                        temp,
                        b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
                        linenumber as ::core::ffi::c_long,
                    );
                    while *temp as ::core::ffi::c_int != '\0' as i32 {
                        temp = temp.offset(1);
                    }
                    ddone = 1 as ::core::ffi::c_int;
                }
                115 => {
                    if sdone != 0 {
                        fprintf(
                            __stderrp,
                            b"%s: fatal: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                            (*kpse_def).invocation_name,
                        );
                        fputs(
                            b"call_edit: `%%s' appears twice in editor command\0" as *const u8
                                as *const ::core::ffi::c_char,
                            __stderrp,
                        );
                        fputs(
                            b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                            __stderrp,
                        );
                        exit(1 as ::core::ffi::c_int);
                    }
                    i = 0 as ::core::ffi::c_int;
                    while i < fnlength {
                        let fresh39 = temp;
                        temp = temp.offset(1);
                        *fresh39 =
                            xchr[*filename.offset(i as isize) as usize] as ::core::ffi::c_char;
                        i += 1;
                    }
                    sdone = 1 as ::core::ffi::c_int;
                }
                0 => {
                    let fresh40 = temp;
                    temp = temp.offset(1);
                    *fresh40 = '%' as i32 as ::core::ffi::c_char;
                    edit_value = edit_value.offset(-1);
                }
                _ => {
                    let fresh41 = temp;
                    temp = temp.offset(1);
                    *fresh41 = '%' as i32 as ::core::ffi::c_char;
                    let fresh42 = temp;
                    temp = temp.offset(1);
                    *fresh42 = c_0;
                }
            }
        } else {
            let fresh43 = temp;
            temp = temp.offset(1);
            *fresh43 = c_0;
        }
    }
    *temp = 0 as ::core::ffi::c_char;
    fullcmd = command;
    if system(fullcmd) != 0 as ::core::ffi::c_int {
        fprintf(
            __stderrp,
            b"! Trouble executing `%s'.\n\0" as *const u8 as *const ::core::ffi::c_char,
            command,
        );
    }
    uexit(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn swap_items(
    mut p: *mut ::core::ffi::c_char,
    mut nitems: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) {
    let mut temp: ::core::ffi::c_char = 0;
    match size {
        16 => loop {
            let fresh33 = nitems;
            nitems = nitems - 1;
            if !(fresh33 != 0) {
                break;
            }
            temp = *p.offset(0 as ::core::ffi::c_int as isize);
            *p.offset(0 as ::core::ffi::c_int as isize) =
                *p.offset(15 as ::core::ffi::c_int as isize);
            *p.offset(15 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(1 as ::core::ffi::c_int as isize);
            *p.offset(1 as ::core::ffi::c_int as isize) =
                *p.offset(14 as ::core::ffi::c_int as isize);
            *p.offset(14 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(2 as ::core::ffi::c_int as isize);
            *p.offset(2 as ::core::ffi::c_int as isize) =
                *p.offset(13 as ::core::ffi::c_int as isize);
            *p.offset(13 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(3 as ::core::ffi::c_int as isize);
            *p.offset(3 as ::core::ffi::c_int as isize) =
                *p.offset(12 as ::core::ffi::c_int as isize);
            *p.offset(12 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(4 as ::core::ffi::c_int as isize);
            *p.offset(4 as ::core::ffi::c_int as isize) =
                *p.offset(11 as ::core::ffi::c_int as isize);
            *p.offset(11 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(5 as ::core::ffi::c_int as isize);
            *p.offset(5 as ::core::ffi::c_int as isize) =
                *p.offset(10 as ::core::ffi::c_int as isize);
            *p.offset(10 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(6 as ::core::ffi::c_int as isize);
            *p.offset(6 as ::core::ffi::c_int as isize) =
                *p.offset(9 as ::core::ffi::c_int as isize);
            *p.offset(9 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(7 as ::core::ffi::c_int as isize);
            *p.offset(7 as ::core::ffi::c_int as isize) =
                *p.offset(8 as ::core::ffi::c_int as isize);
            *p.offset(8 as ::core::ffi::c_int as isize) = temp;
            p = p.offset(size as isize);
        },
        8 => loop {
            let fresh34 = nitems;
            nitems = nitems - 1;
            if !(fresh34 != 0) {
                break;
            }
            temp = *p.offset(0 as ::core::ffi::c_int as isize);
            *p.offset(0 as ::core::ffi::c_int as isize) =
                *p.offset(7 as ::core::ffi::c_int as isize);
            *p.offset(7 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(1 as ::core::ffi::c_int as isize);
            *p.offset(1 as ::core::ffi::c_int as isize) =
                *p.offset(6 as ::core::ffi::c_int as isize);
            *p.offset(6 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(2 as ::core::ffi::c_int as isize);
            *p.offset(2 as ::core::ffi::c_int as isize) =
                *p.offset(5 as ::core::ffi::c_int as isize);
            *p.offset(5 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(3 as ::core::ffi::c_int as isize);
            *p.offset(3 as ::core::ffi::c_int as isize) =
                *p.offset(4 as ::core::ffi::c_int as isize);
            *p.offset(4 as ::core::ffi::c_int as isize) = temp;
            p = p.offset(size as isize);
        },
        4 => loop {
            let fresh35 = nitems;
            nitems = nitems - 1;
            if !(fresh35 != 0) {
                break;
            }
            temp = *p.offset(0 as ::core::ffi::c_int as isize);
            *p.offset(0 as ::core::ffi::c_int as isize) =
                *p.offset(3 as ::core::ffi::c_int as isize);
            *p.offset(3 as ::core::ffi::c_int as isize) = temp;
            temp = *p.offset(1 as ::core::ffi::c_int as isize);
            *p.offset(1 as ::core::ffi::c_int as isize) =
                *p.offset(2 as ::core::ffi::c_int as isize);
            *p.offset(2 as ::core::ffi::c_int as isize) = temp;
            p = p.offset(size as isize);
        },
        2 => loop {
            let fresh36 = nitems;
            nitems = nitems - 1;
            if !(fresh36 != 0) {
                break;
            }
            temp = *p.offset(0 as ::core::ffi::c_int as isize);
            *p.offset(0 as ::core::ffi::c_int as isize) =
                *p.offset(1 as ::core::ffi::c_int as isize);
            *p.offset(1 as ::core::ffi::c_int as isize) = temp;
            p = p.offset(size as isize);
        },
        1 => {}
        _ => {
            fprintf(
                __stderrp,
                b"%s: fatal: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
                (*kpse_def).invocation_name,
            );
            fprintf(
                __stderrp,
                b"Can't swap a %d-byte item for (un)dumping\0" as *const u8
                    as *const ::core::ffi::c_char,
                size,
            );
            fputs(
                b".\n\0" as *const u8 as *const ::core::ffi::c_char,
                __stderrp,
            );
            exit(1 as ::core::ffi::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn do_dump(
    mut p: *mut ::core::ffi::c_char,
    mut item_size: ::core::ffi::c_int,
    mut nitems: ::core::ffi::c_int,
    mut out_file: gzFile,
) {
    swap_items(p, nitems, item_size);
    if gzwrite(
        out_file,
        p as voidpc,
        (item_size * nitems) as ::core::ffi::c_uint,
    ) != item_size * nitems
    {
        fprintf(
            __stderrp,
            b"! Could not write %d %d-byte item(s) to %s.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            nitems,
            item_size,
            nameoffile.offset(1 as ::core::ffi::c_int as isize),
        );
        uexit(1 as ::core::ffi::c_int);
    }
    swap_items(p, nitems, item_size);
}
#[no_mangle]
pub unsafe extern "C" fn do_undump(
    mut p: *mut ::core::ffi::c_char,
    mut item_size: ::core::ffi::c_int,
    mut nitems: ::core::ffi::c_int,
    mut in_file: gzFile,
) {
    if gzread(
        in_file,
        p as voidp,
        (item_size * nitems) as ::core::ffi::c_uint,
    ) != item_size * nitems
    {
        fprintf(
            __stderrp,
            b"%s: fatal: (kpathsea) \0" as *const u8 as *const ::core::ffi::c_char,
            (*kpse_def).invocation_name,
        );
        fprintf(
            __stderrp,
            b"Could not undump %d %d-byte item(s) from %s\0" as *const u8
                as *const ::core::ffi::c_char,
            nitems,
            item_size,
            nameoffile.offset(1 as ::core::ffi::c_int as isize),
        );
        fputs(
            b".\n\0" as *const u8 as *const ::core::ffi::c_char,
            __stderrp,
        );
        exit(1 as ::core::ffi::c_int);
    }
    swap_items(p, nitems, item_size);
}
#[no_mangle]
pub unsafe extern "C" fn makefullnamestring() -> strnumber {
    return maketexstring(fullnameoffile as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn getjobname(mut name: strnumber) -> strnumber {
    let mut ret: strnumber = name;
    let mut i: ::core::ffi::c_int = 0;
    let mut l_0: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    if !c_job_name.is_null() {
        ret = maketexstring(c_job_name as *const ::core::ffi::c_char);
    }
    return ret;
}
unsafe extern "C" fn compare_paths(
    mut p1: const_string,
    mut p2: const_string,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    loop {
        ret = *p1 as ::core::ffi::c_int - *p2 as ::core::ffi::c_int;
        if !(ret == 0 as ::core::ffi::c_int && *p2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            || *p1 as ::core::ffi::c_int == DIR_SEP && *p2 as ::core::ffi::c_int == DIR_SEP)
        {
            break;
        }
        p1 = p1.offset(1);
        p2 = p2.offset(1);
    }
    ret = if ret < 0 as ::core::ffi::c_int {
        -(1 as ::core::ffi::c_int)
    } else if ret > 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn gettexstring(mut s: strnumber) -> string {
    let mut len: poolpointer = 0;
    let mut name: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    len = *strstart.offset((s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        - *strstart.offset(s as isize);
    name = xmalloc((len as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t) as string;
    strncpy(
        name as *mut ::core::ffi::c_char,
        strpool.offset(*strstart.offset(s as isize) as isize) as *mut packedASCIIcode as string
            as *const ::core::ffi::c_char,
        len as size_t,
    );
    *name.offset(len as isize) = 0 as ::core::ffi::c_char;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn isnewsource(
    mut srcfilename: strnumber,
    mut lineno: ::core::ffi::c_int,
) -> boolean {
    let mut name: *mut ::core::ffi::c_char = gettexstring(srcfilename) as *mut ::core::ffi::c_char;
    return (compare_paths(name as const_string, last_source_name as const_string)
        != 0 as ::core::ffi::c_int
        || lineno != last_lineno) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn remembersourceinfo(
    mut srcfilename: strnumber,
    mut lineno: ::core::ffi::c_int,
) {
    if !last_source_name.is_null() {
        free(last_source_name as *mut ::core::ffi::c_void);
    }
    last_source_name = gettexstring(srcfilename) as *mut ::core::ffi::c_char;
    last_lineno = lineno;
}
#[no_mangle]
pub unsafe extern "C" fn makesrcspecial(
    mut srcfilename: strnumber,
    mut lineno: ::core::ffi::c_int,
) -> poolpointer {
    let mut oldpoolptr: poolpointer = poolptr as poolpointer;
    let mut filename: *mut ::core::ffi::c_char =
        gettexstring(srcfilename) as *mut ::core::ffi::c_char;
    let mut buf: [::core::ffi::c_char; 40] = [0; 40];
    let mut s: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"src:%d \0" as *const u8 as *const ::core::ffi::c_char,
        lineno,
    );
    if (poolptr as size_t)
        .wrapping_add(strlen(&raw mut buf as *mut ::core::ffi::c_char))
        .wrapping_add(strlen(filename))
        >= poolsize as size_t
    {
        fprintf(
            __stderrp,
            b"\nstring pool overflow\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    s = &raw mut buf as *mut ::core::ffi::c_char;
    while *s != 0 {
        let fresh44 = s;
        s = s.offset(1);
        let fresh45 = poolptr;
        poolptr = poolptr + 1;
        *strpool.offset(fresh45 as isize) = *fresh44 as packedASCIIcode;
    }
    s = filename;
    while *s != 0 {
        let fresh46 = s;
        s = s.offset(1);
        let fresh47 = poolptr;
        poolptr = poolptr + 1;
        *strpool.offset(fresh47 as isize) = *fresh46 as packedASCIIcode;
    }
    return oldpoolptr;
}
pub const MAX_CSTRING_LEN: ::core::ffi::c_int =
    1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
pub const TIME_STR_SIZE: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
#[no_mangle]
pub static mut start_time_str: [::core::ffi::c_char; 30] = [0; 30];
static mut time_str: [::core::ffi::c_char; 30] = [0; 30];
unsafe extern "C" fn makepdftime(
    mut t: time_t,
    mut time_str_0: *mut ::core::ffi::c_char,
    mut utc: boolean,
) {
    let mut lt: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut gmt: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut size: size_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut off: ::core::ffi::c_int = 0;
    let mut off_hours: ::core::ffi::c_int = 0;
    let mut off_mins: ::core::ffi::c_int = 0;
    if utc != 0 {
        lt = *gmtime(&raw mut t);
    } else {
        lt = *localtime(&raw mut t);
    }
    size = strftime(
        time_str_0,
        TIME_STR_SIZE as size_t,
        b"D:%Y%m%d%H%M%S\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut lt,
    );
    if size == 0 as size_t {
        *time_str_0.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        return;
    }
    if *time_str_0.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '6' as i32 {
        *time_str_0.offset(14 as ::core::ffi::c_int as isize) = '5' as i32 as ::core::ffi::c_char;
        *time_str_0.offset(15 as ::core::ffi::c_int as isize) = '9' as i32 as ::core::ffi::c_char;
        *time_str_0.offset(16 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    gmt = *gmtime(&raw mut t);
    off = 60 as ::core::ffi::c_int * (lt.tm_hour - gmt.tm_hour) + lt.tm_min - gmt.tm_min;
    if lt.tm_year != gmt.tm_year {
        off += if lt.tm_year > gmt.tm_year {
            1440 as ::core::ffi::c_int
        } else {
            -(1440 as ::core::ffi::c_int)
        };
    } else if lt.tm_yday != gmt.tm_yday {
        off += if lt.tm_yday > gmt.tm_yday {
            1440 as ::core::ffi::c_int
        } else {
            -(1440 as ::core::ffi::c_int)
        };
    }
    if off == 0 as ::core::ffi::c_int {
        let fresh0 = size;
        size = size.wrapping_add(1);
        *time_str_0.offset(fresh0 as isize) = 'Z' as i32 as ::core::ffi::c_char;
        *time_str_0.offset(size as isize) = 0 as ::core::ffi::c_char;
    } else {
        off_hours = off / 60 as ::core::ffi::c_int;
        off_mins = (if off - off_hours * 60 as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            off - off_hours * 60 as ::core::ffi::c_int
        } else {
            -(off - off_hours * 60 as ::core::ffi::c_int)
        }) as ::core::ffi::c_int;
        i = snprintf(
            time_str_0.offset(size as isize) as *mut ::core::ffi::c_char,
            9 as size_t,
            b"%+03d'%02d'\0" as *const u8 as *const ::core::ffi::c_char,
            off_hours,
            off_mins,
        );
        if i as ::core::ffi::c_uint >= 9 as ::core::ffi::c_int as ::core::ffi::c_uint {
            crate::utils::pdftex_fail_args(b"snprintf failed: file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(b"pdftex-rust/generated/texmfmp.rs\0"
                    as *const u8 as *const ::core::ffi::c_char), crate::utils::PrintfArg::from(3413 as ::core::ffi::c_int)]);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn initstarttime() {
    if start_time_set == 0 {
        init_start_time();
        if !getenv(b"SOURCE_DATE_EPOCH\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            makepdftime(
                start_time,
                &raw mut start_time_str as *mut ::core::ffi::c_char,
                true_0,
            );
        } else {
            makepdftime(
                start_time,
                &raw mut start_time_str as *mut ::core::ffi::c_char,
                false_0,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn find_input_file(mut s: integer) -> string {
    let mut filename: string = ::core::ptr::null_mut::<::core::ffi::c_char>();
    filename = makecfilename(s) as string;
    if !output_directory.is_null() && kpse_absolute_p(filename as const_string, false_0) == 0 {
        let mut pathname: string = concat3(
            output_directory as const_string,
            DIR_SEP_STRING.as_ptr(),
            filename as const_string,
        );
        if access(pathname as *const ::core::ffi::c_char, R_OK) == 0 as ::core::ffi::c_int
            && dir_p(pathname) == 0
        {
            return pathname;
        }
        if !pathname.is_null() {
            free(pathname as *mut ::core::ffi::c_void);
        }
        pathname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if kpse_in_name_ok(filename as const_string) == 0 {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return kpse_find_file(filename as const_string, kpse_tex_format, true_0);
}
#[no_mangle]
pub unsafe extern "C" fn makecstring(mut s: integer) -> *mut ::core::ffi::c_char {
    static mut cstrbuf: *mut ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut allocsize: ::core::ffi::c_int = 0;
    let mut allocgrow: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut l_0: ::core::ffi::c_int = *strstart
        .offset((s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        - *strstart.offset(s as isize) as ::core::ffi::c_int;
    if (l_0 + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
        > (1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        crate::utils::pdftex_fail_args(b"buffer overflow at file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(b"pdftex-rust/generated/texmfmp.rs\0"
                as *const u8 as *const ::core::ffi::c_char), crate::utils::PrintfArg::from(3494 as ::core::ffi::c_int)]);
    }
    if cstrbuf.is_null() {
        allocsize = l_0 + 1 as ::core::ffi::c_int;
        cstrbuf = xmalloc(
            ((allocsize + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        ) as *mut ::core::ffi::c_char;
    } else if l_0 + 1 as ::core::ffi::c_int > allocsize {
        allocgrow = (allocsize as ::core::ffi::c_double * 0.2f64) as ::core::ffi::c_int;
        if l_0 + 1 as ::core::ffi::c_int - allocgrow > allocsize {
            allocsize = l_0 + 1 as ::core::ffi::c_int;
        } else if allocsize < MAX_CSTRING_LEN - allocgrow {
            allocsize += allocgrow;
        } else {
            allocsize = MAX_CSTRING_LEN;
        }
        cstrbuf = xrealloc(
            cstrbuf as address,
            ((allocsize + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
        ) as *mut ::core::ffi::c_char;
    }
    p = cstrbuf;
    i = 0 as ::core::ffi::c_int;
    while i < l_0 {
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = *strpool.offset((i as poolpointer + *strstart.offset(s as isize)) as isize)
            as ::core::ffi::c_char;
        i += 1;
    }
    *p = 0 as ::core::ffi::c_char;
    return cstrbuf;
}
#[no_mangle]
pub unsafe extern "C" fn makecfilename(mut s: integer) -> *mut ::core::ffi::c_char {
    let mut name: *mut ::core::ffi::c_char = makecstring(s);
    let mut p: *mut ::core::ffi::c_char = name;
    let mut q: *mut ::core::ffi::c_char = name;
    while *p != 0 {
        if *p as ::core::ffi::c_int != '"' as i32 {
            let fresh1 = q;
            q = q.offset(1);
            *fresh1 = *p;
        }
        p = p.offset(1);
    }
    *q = '\0' as i32 as ::core::ffi::c_char;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn getcreationdate() {
    let mut len: size_t = 0;
    initstarttime();
    len = strlen(&raw mut start_time_str as *mut ::core::ffi::c_char);
    if (poolptr as size_t).wrapping_add(len) as ::core::ffi::c_uint
        >= poolsize as ::core::ffi::c_uint
    {
        poolptr = poolsize;
        return;
    }
    memcpy(
        strpool.offset(poolptr as isize) as *mut packedASCIIcode as *mut ::core::ffi::c_void,
        &raw mut start_time_str as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len,
    );
    poolptr = (poolptr as size_t).wrapping_add(len) as integer as integer;
}
#[no_mangle]
pub unsafe extern "C" fn getfilemoddate(mut s: integer) {
    let mut file_data: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    let mut file_name: *mut ::core::ffi::c_char = find_input_file(s) as *mut ::core::ffi::c_char;
    if file_name.is_null() {
        return;
    }
    recorder_record_input(file_name as const_string);
    if stat(file_name, &raw mut file_data) == 0 as ::core::ffi::c_int {
        let mut len: size_t = 0;
        let mut use_utc: boolean =
            (FORCE_SOURCE_DATE_set != 0 && SOURCE_DATE_EPOCH_set != 0) as ::core::ffi::c_int;
        makepdftime(
            file_data.st_mtimespec.tv_sec as time_t,
            &raw mut time_str as *mut ::core::ffi::c_char,
            use_utc,
        );
        len = strlen(&raw mut time_str as *mut ::core::ffi::c_char);
        if (poolptr as size_t).wrapping_add(len) as ::core::ffi::c_uint
            >= poolsize as ::core::ffi::c_uint
        {
            poolptr = poolsize;
        } else {
            memcpy(
                strpool.offset(poolptr as isize) as *mut packedASCIIcode
                    as *mut ::core::ffi::c_void,
                &raw mut time_str as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                len,
            );
            poolptr = (poolptr as size_t).wrapping_add(len) as integer as integer;
        }
    }
    if !file_name.is_null() {
        free(file_name as *mut ::core::ffi::c_void);
    }
    file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn getfilesize(mut s: integer) {
    let mut file_data: stat = stat {
        st_dev: 0,
        st_mode: 0,
        st_nlink: 0,
        st_ino: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        st_atimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_birthtimespec: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_size: 0,
        st_blocks: 0,
        st_blksize: 0,
        st_flags: 0,
        st_gen: 0,
        st_lspare: 0,
        st_qspare: [0; 2],
    };
    let mut i: ::core::ffi::c_int = 0;
    let mut file_name: *mut ::core::ffi::c_char = find_input_file(s) as *mut ::core::ffi::c_char;
    if file_name.is_null() {
        return;
    }
    recorder_record_input(file_name as const_string);
    if stat(file_name, &raw mut file_data) == 0 as ::core::ffi::c_int {
        let mut len: size_t = 0;
        let mut buf: [::core::ffi::c_char; 20] = [0; 20];
        i = snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 20]>() as size_t,
            b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
            file_data.st_size as ::core::ffi::c_ulong,
        );
        if i as ::core::ffi::c_uint
            >= ::core::mem::size_of::<[::core::ffi::c_char; 20]>() as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(b"snprintf failed: file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(b"pdftex-rust/generated/texmfmp.rs\0"
                    as *const u8 as *const ::core::ffi::c_char), crate::utils::PrintfArg::from(3637 as ::core::ffi::c_int)]);
        }
        len = strlen(&raw mut buf as *mut ::core::ffi::c_char);
        if (poolptr as size_t).wrapping_add(len) as ::core::ffi::c_uint
            >= poolsize as ::core::ffi::c_uint
        {
            poolptr = poolsize;
        } else {
            memcpy(
                strpool.offset(poolptr as isize) as *mut packedASCIIcode
                    as *mut ::core::ffi::c_void,
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                len,
            );
            poolptr = (poolptr as size_t).wrapping_add(len) as integer as integer;
        }
    }
    if !file_name.is_null() {
        free(file_name as *mut ::core::ffi::c_void);
    }
    file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn getfiledump(
    mut s: integer,
    mut offset: ::core::ffi::c_int,
    mut length: ::core::ffi::c_int,
) {
    let mut f_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut read: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut data_ptr: poolpointer = 0;
    let mut data_end: poolpointer = 0;
    let mut file_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if length == 0 as ::core::ffi::c_int {
        return;
    }
    if poolptr as ::core::ffi::c_int + 2 as ::core::ffi::c_int * length + 1 as ::core::ffi::c_int
        >= poolsize
    {
        poolptr = poolsize;
        return;
    }
    file_name = find_input_file(s) as *mut ::core::ffi::c_char;
    if file_name.is_null() {
        return;
    }
    f_0 = fopen(file_name, FOPEN_RBIN_MODE.as_ptr()) as *mut FILE;
    if f_0.is_null() {
        if !file_name.is_null() {
            free(file_name as *mut ::core::ffi::c_void);
        }
        file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return;
    }
    recorder_record_input(file_name as const_string);
    if fseek(f_0, offset as ::core::ffi::c_long, SEEK_SET) != 0 as ::core::ffi::c_int {
        if !file_name.is_null() {
            free(file_name as *mut ::core::ffi::c_void);
        }
        file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return;
    }
    data_ptr = (poolptr as ::core::ffi::c_int + length) as poolpointer;
    read = fread(
        strpool.offset(data_ptr as isize) as *mut packedASCIIcode as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
        length as size_t,
        f_0,
    ) as ::core::ffi::c_int;
    fclose(f_0);
    data_end = (data_ptr as ::core::ffi::c_int + read) as poolpointer;
    while data_ptr < data_end {
        i = snprintf(
            strpool.offset(poolptr as isize) as *mut packedASCIIcode as *mut ::core::ffi::c_char,
            3 as size_t,
            b"%.2X\0" as *const u8 as *const ::core::ffi::c_char,
            *strpool.offset(data_ptr as isize) as ::core::ffi::c_uint,
        );
        if i as ::core::ffi::c_uint >= 3 as ::core::ffi::c_int as ::core::ffi::c_uint {
            crate::utils::pdftex_fail_args(b"snprintf failed: file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(b"pdftex-rust/generated/texmfmp.rs\0"
                    as *const u8 as *const ::core::ffi::c_char), crate::utils::PrintfArg::from(3725 as ::core::ffi::c_int)]);
        }
        poolptr += i;
        data_ptr += 1;
    }
    if !file_name.is_null() {
        free(file_name as *mut ::core::ffi::c_void);
    }
    file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn convertStringToHexString(
    mut in_0: *const ::core::ffi::c_char,
    mut out: *mut ::core::ffi::c_char,
    mut lin: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k_0: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 3] = [0; 3];
    j = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < lin {
        k_0 = snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 3]>() as size_t,
            b"%02X\0" as *const u8 as *const ::core::ffi::c_char,
            *in_0.offset(i as isize) as ::core::ffi::c_uchar as ::core::ffi::c_uint,
        );
        if k_0 as ::core::ffi::c_uint
            >= ::core::mem::size_of::<[::core::ffi::c_char; 3]>() as ::core::ffi::c_uint
        {
            crate::utils::pdftex_fail_args(b"snprintf failed: file %s, line %d\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(b"pdftex-rust/generated/texmfmp.rs\0"
                    as *const u8 as *const ::core::ffi::c_char), crate::utils::PrintfArg::from(3745 as ::core::ffi::c_int)]);
        }
        let fresh3 = j;
        j = j + 1;
        *out.offset(fresh3 as isize) = buf[0 as ::core::ffi::c_int as usize];
        let fresh4 = j;
        j = j + 1;
        *out.offset(fresh4 as isize) = buf[1 as ::core::ffi::c_int as usize];
        i += 1;
    }
    *out.offset(j as isize) = '\0' as i32 as ::core::ffi::c_char;
}
pub const DIGEST_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const FILE_BUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn getmd5sum(mut s: strnumber, mut file: boolean) {
    let mut state: md5_state_t = md5_state_s {
        count: [0; 2],
        abcd: [0; 4],
        buf: [0; 64],
    };
    let mut digest: [md5_byte_t; 16] = [0; 16];
    let mut outbuf: [::core::ffi::c_char; 33] = [0; 33];
    let mut len: ::core::ffi::c_int = 2 as ::core::ffi::c_int * DIGEST_SIZE;
    if file != 0 {
        let mut file_buf: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut read: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut f_0: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut file_name: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        file_name = find_input_file(s as integer) as *mut ::core::ffi::c_char;
        if file_name.is_null() {
            return;
        }
        f_0 = fopen(file_name, FOPEN_RBIN_MODE.as_ptr()) as *mut FILE;
        if f_0.is_null() {
            if !file_name.is_null() {
                free(file_name as *mut ::core::ffi::c_void);
            }
            file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return;
        }
        recorder_record_input(file_name as const_string);
        md5_init(&raw mut state);
        loop {
            read = fread(
                &raw mut file_buf as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
                FILE_BUF_SIZE as size_t,
                f_0,
            ) as ::core::ffi::c_int;
            if !(read > 0 as ::core::ffi::c_int) {
                break;
            }
            md5_append(
                &raw mut state,
                &raw mut file_buf as *mut ::core::ffi::c_char as *const md5_byte_t,
                read,
            );
        }
        md5_finish(&raw mut state, &raw mut digest as *mut md5_byte_t);
        fclose(f_0);
        if !file_name.is_null() {
            free(file_name as *mut ::core::ffi::c_void);
        }
        file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        md5_init(&raw mut state);
        md5_append(
            &raw mut state,
            strpool.offset(*strstart.offset(s as isize) as isize) as *mut packedASCIIcode
                as *mut md5_byte_t,
            *strstart.offset((s as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - *strstart.offset(s as isize) as ::core::ffi::c_int,
        );
        md5_finish(&raw mut state, &raw mut digest as *mut md5_byte_t);
    }
    if poolptr as ::core::ffi::c_int + len >= poolsize {
        return;
    }
    convertStringToHexString(
        &raw mut digest as *mut md5_byte_t as *mut ::core::ffi::c_char,
        &raw mut outbuf as *mut ::core::ffi::c_char,
        DIGEST_SIZE,
    );
    memcpy(
        strpool.offset(poolptr as isize) as *mut packedASCIIcode as *mut ::core::ffi::c_void,
        &raw mut outbuf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
    );
    poolptr += len;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut string,
        ) as i32)
    }
}

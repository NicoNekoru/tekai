#[repr(C)]
pub struct __sFILEX {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct png_struct_def {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct png_info_def {
    _unused: [u8; 0],
}

extern "C" {
    fn feof(_: *mut FILE) -> ::core::ffi::c_int;
    fn fgetc(_: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __nitems: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn free(_: *mut ::core::ffi::c_void);
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn xfopen(filename: const_string, mode: const_string) -> *mut FILE;
    fn xfseek(
        fp: *mut FILE,
        offset: ::core::ffi::c_long,
        wherefrom: ::core::ffi::c_int,
        filename: const_string,
    );
    fn xftell(fp: *mut FILE, filename: const_string) -> ::core::ffi::c_long;
    fn xmalloc(size: size_t) -> address;
    static mut pdfbuf: *mut eightbits;
    static mut pdfbufsize: integer;
    static mut pdfptr: integer;
    static mut pdfosmode: boolean;
    static mut fixedpdfmajorversion: integer;
    static mut fixedpdfminorversion: integer;
    fn zpdfosgetosbuf(s: integer);
    fn pdfflush();
    fn pdfendstream();
    static mut cur_file_name: *mut ::core::ffi::c_char;
    fn pdf_printf(_: *const ::core::ffi::c_char, ...);
    fn pdf_puts(_: *const ::core::ffi::c_char);
    fn pdftex_fail(_: *const ::core::ffi::c_char, ...) -> !;
    fn xgetc(_: *mut FILE) -> ::core::ffi::c_int;
    static mut image_array: *mut image_entry;
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
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub type eightbits = ::core::ffi::c_uchar;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JPG_IMAGE_INFO {
    pub color_space: ::core::ffi::c_int,
    pub bits_per_component: ::core::ffi::c_uchar,
    pub length: ::core::ffi::c_ulong,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_image_struct {
    pub png_ptr: png_structp,
    pub info_ptr: png_infop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_image_struct {
    pub orig_x: integer,
    pub orig_y: integer,
    pub selected_page: integer,
    pub page_box: integer,
    pub doc: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JBIG2_IMAGE_INFO {
    pub selected_page: integer,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_entry {
    pub image_name: *mut ::core::ffi::c_char,
    pub image_type: ::core::ffi::c_int,
    pub color_type: ::core::ffi::c_int,
    pub width: integer,
    pub height: integer,
    pub rotate: integer,
    pub x_res: integer,
    pub y_res: integer,
    pub num_pages: integer,
    pub colorspace_ref: integer,
    pub group_ref: integer,
    pub image_struct: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub pdf: *mut pdf_image_struct,
    pub png: png_image_struct,
    pub jpg: *mut JPG_IMAGE_INFO,
    pub jbig2: *mut JBIG2_IMAGE_INFO,
}
pub const M_RST7: C2RustUnnamed_0 = 215;
pub const M_RST6: C2RustUnnamed_0 = 214;
pub const M_RST5: C2RustUnnamed_0 = 213;
pub const M_RST4: C2RustUnnamed_0 = 212;
pub const M_RST3: C2RustUnnamed_0 = 211;
pub const M_RST2: C2RustUnnamed_0 = 210;
pub const M_RST1: C2RustUnnamed_0 = 209;
pub const M_RST0: C2RustUnnamed_0 = 208;
pub const M_TEM: C2RustUnnamed_0 = 1;
pub const M_EOI: C2RustUnnamed_0 = 217;
pub const M_SOI: C2RustUnnamed_0 = 216;
pub const M_SOF3: C2RustUnnamed_0 = 195;
pub const M_SOF1: C2RustUnnamed_0 = 193;
pub const M_SOF0: C2RustUnnamed_0 = 192;
pub const M_SOF2: C2RustUnnamed_0 = 194;
pub const M_SOF15: C2RustUnnamed_0 = 207;
pub const M_SOF14: C2RustUnnamed_0 = 206;
pub const M_SOF13: C2RustUnnamed_0 = 205;
pub const M_SOF11: C2RustUnnamed_0 = 203;
pub const M_SOF10: C2RustUnnamed_0 = 202;
pub const M_SOF9: C2RustUnnamed_0 = 201;
pub const M_SOF7: C2RustUnnamed_0 = 199;
pub const M_SOF6: C2RustUnnamed_0 = 198;
pub const M_SOF5: C2RustUnnamed_0 = 197;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const M_ERROR: C2RustUnnamed_0 = 256;
pub const M_COM: C2RustUnnamed_0 = 254;
pub const M_JPG13: C2RustUnnamed_0 = 253;
pub const M_JPG0: C2RustUnnamed_0 = 240;
pub const M_APP15: C2RustUnnamed_0 = 239;
pub const M_APP14: C2RustUnnamed_0 = 238;
pub const M_APP13: C2RustUnnamed_0 = 237;
pub const M_APP12: C2RustUnnamed_0 = 236;
pub const M_APP11: C2RustUnnamed_0 = 235;
pub const M_APP10: C2RustUnnamed_0 = 234;
pub const M_APP9: C2RustUnnamed_0 = 233;
pub const M_APP8: C2RustUnnamed_0 = 232;
pub const M_APP7: C2RustUnnamed_0 = 231;
pub const M_APP6: C2RustUnnamed_0 = 230;
pub const M_APP5: C2RustUnnamed_0 = 229;
pub const M_APP4: C2RustUnnamed_0 = 228;
pub const M_APP3: C2RustUnnamed_0 = 227;
pub const M_APP2: C2RustUnnamed_0 = 226;
pub const M_APP1: C2RustUnnamed_0 = 225;
pub const M_APP0: C2RustUnnamed_0 = 224;
pub const M_EXP: C2RustUnnamed_0 = 223;
pub const M_DHP: C2RustUnnamed_0 = 222;
pub const M_DRI: C2RustUnnamed_0 = 221;
pub const M_DNL: C2RustUnnamed_0 = 220;
pub const M_DQT: C2RustUnnamed_0 = 219;
pub const M_SOS: C2RustUnnamed_0 = 218;
pub const M_DAC: C2RustUnnamed_0 = 204;
pub const M_DHT: C2RustUnnamed_0 = 196;
pub const M_JPG: C2RustUnnamed_0 = 200;
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const IMAGE_COLOR_B: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IMAGE_COLOR_C: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const JPG_GRAY: ::core::ffi::c_int = 1;
pub const JPG_RGB: ::core::ffi::c_int = 3;
pub const JPG_CMYK: ::core::ffi::c_int = 4;
unsafe extern "C" fn read2bytes(mut f: *mut FILE) -> ::core::ffi::c_uint {
    let mut c: ::core::ffi::c_int = xgetc(f);
    return ((c << 8 as ::core::ffi::c_int) + xgetc(f)) as ::core::ffi::c_uint;
}
unsafe extern "C" fn get_unsigned_byte(mut file: *mut FILE) -> ::core::ffi::c_uchar {
    let mut ch: ::core::ffi::c_int = 0;
    ch = fgetc(file);
    return ch as ::core::ffi::c_uchar;
}
unsafe extern "C" fn get_unsigned_pair(mut file: *mut FILE) -> ::core::ffi::c_ushort {
    let mut pair: ::core::ffi::c_ushort = get_unsigned_byte(file) as ::core::ffi::c_ushort;
    pair = ((pair as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | get_unsigned_byte(file) as ::core::ffi::c_int) as ::core::ffi::c_ushort;
    return pair;
}
unsafe extern "C" fn read_exif_bytes(
    mut p: *mut *mut ::core::ffi::c_uchar,
    mut n: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut rval: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut pp: *mut ::core::ffi::c_uchar = *p;
    if b != 0 {
        let mut current_block_6: u64;
        match n {
            4 => {
                let fresh8 = pp;
                pp = pp.offset(1);
                rval = rval.wrapping_add(*fresh8 as ::core::ffi::c_uint);
                rval <<= 8 as ::core::ffi::c_int;
                let fresh9 = pp;
                pp = pp.offset(1);
                rval = rval.wrapping_add(*fresh9 as ::core::ffi::c_uint);
                rval <<= 8 as ::core::ffi::c_int;
                current_block_6 = 2035738867698081325;
            }
            2 => {
                current_block_6 = 2035738867698081325;
            }
            _ => {
                current_block_6 = 7815301370352969686;
            }
        }
        match current_block_6 {
            2035738867698081325 => {
                let fresh10 = pp;
                pp = pp.offset(1);
                rval = rval.wrapping_add(*fresh10 as ::core::ffi::c_uint);
                rval <<= 8 as ::core::ffi::c_int;
                rval = rval.wrapping_add(*pp as ::core::ffi::c_uint);
            }
            _ => {}
        }
    } else {
        pp = pp.offset(n as isize);
        let mut current_block_16: u64;
        match n {
            4 => {
                pp = pp.offset(-1);
                rval = rval.wrapping_add(*pp as ::core::ffi::c_uint);
                rval <<= 8 as ::core::ffi::c_int;
                pp = pp.offset(-1);
                rval = rval.wrapping_add(*pp as ::core::ffi::c_uint);
                rval <<= 8 as ::core::ffi::c_int;
                current_block_16 = 10256748123573275224;
            }
            2 => {
                current_block_16 = 10256748123573275224;
            }
            _ => {
                current_block_16 = 17407779659766490442;
            }
        }
        match current_block_16 {
            10256748123573275224 => {
                pp = pp.offset(-1);
                rval = rval.wrapping_add(*pp as ::core::ffi::c_uint);
                rval <<= 8 as ::core::ffi::c_int;
                pp = pp.offset(-1);
                rval = rval.wrapping_add(*pp as ::core::ffi::c_uint);
            }
            _ => {}
        }
    }
    *p = (*p).offset(n as isize);
    return rval;
}
unsafe extern "C" fn read_APP1_Exif(
    mut fp: *mut FILE,
    mut length: ::core::ffi::c_ushort,
    mut xx: *mut ::core::ffi::c_int,
    mut yy: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut buffer: *mut ::core::ffi::c_uchar = xmalloc(
        (length as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_uchar>() as size_t),
    ) as *mut ::core::ffi::c_uchar;
    let mut p: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut rp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut tiff_header: *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut bigendian: ::core::ffi::c_char = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut num_fields: ::core::ffi::c_int = 0;
    let mut tag: ::core::ffi::c_int = 0;
    let mut type_0: ::core::ffi::c_int = 0;
    let mut value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut den: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut xres: ::core::ffi::c_double = 72.0f64;
    let mut yres: ::core::ffi::c_double = 72.0f64;
    let mut res_unit: ::core::ffi::c_double = 1.0f64;
    fread(
        buffer as *mut ::core::ffi::c_void,
        length as size_t,
        1 as size_t,
        fp,
    );
    p = buffer;
    while p < buffer.offset(length as ::core::ffi::c_int as isize)
        && *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        p = p.offset(1);
    }
    tiff_header = p;
    if *p as ::core::ffi::c_int == 'M' as i32
        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'M' as i32
    {
        bigendian = 1 as ::core::ffi::c_char;
        current_block = 11650488183268122163;
    } else if *p as ::core::ffi::c_int == 'I' as i32
        && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'I' as i32
    {
        bigendian = 0 as ::core::ffi::c_char;
        current_block = 11650488183268122163;
    } else {
        current_block = 5902651919372197310;
    }
    match current_block {
        11650488183268122163 => {
            p = p.offset(2 as ::core::ffi::c_int as isize);
            i = read_exif_bytes(
                &raw mut p,
                2 as ::core::ffi::c_int,
                bigendian as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            if !(i != 42 as ::core::ffi::c_int) {
                i = read_exif_bytes(
                    &raw mut p,
                    4 as ::core::ffi::c_int,
                    bigendian as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
                p = tiff_header.offset(i as isize);
                num_fields = read_exif_bytes(
                    &raw mut p,
                    2 as ::core::ffi::c_int,
                    bigendian as ::core::ffi::c_int,
                ) as ::core::ffi::c_int;
                loop {
                    let fresh5 = num_fields;
                    num_fields = num_fields - 1;
                    if !(fresh5 > 0 as ::core::ffi::c_int) {
                        break;
                    }
                    tag = read_exif_bytes(
                        &raw mut p,
                        2 as ::core::ffi::c_int,
                        bigendian as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    type_0 = read_exif_bytes(
                        &raw mut p,
                        2 as ::core::ffi::c_int,
                        bigendian as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    read_exif_bytes(
                        &raw mut p,
                        4 as ::core::ffi::c_int,
                        bigendian as ::core::ffi::c_int,
                    );
                    match type_0 {
                        1 => {
                            let fresh6 = p;
                            p = p.offset(1);
                            value = *fresh6 as ::core::ffi::c_int;
                            p = p.offset(3 as ::core::ffi::c_int as isize);
                        }
                        3 => {
                            value = read_exif_bytes(
                                &raw mut p,
                                2 as ::core::ffi::c_int,
                                bigendian as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int;
                            p = p.offset(2 as ::core::ffi::c_int as isize);
                        }
                        4 | 9 => {
                            value = read_exif_bytes(
                                &raw mut p,
                                4 as ::core::ffi::c_int,
                                bigendian as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int;
                        }
                        5 | 10 => {
                            value = read_exif_bytes(
                                &raw mut p,
                                4 as ::core::ffi::c_int,
                                bigendian as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int;
                            rp = tiff_header.offset(value as isize);
                            num = read_exif_bytes(
                                &raw mut rp,
                                4 as ::core::ffi::c_int,
                                bigendian as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int;
                            den = read_exif_bytes(
                                &raw mut rp,
                                4 as ::core::ffi::c_int,
                                bigendian as ::core::ffi::c_int,
                            ) as ::core::ffi::c_int;
                        }
                        7 => {
                            let fresh7 = p;
                            p = p.offset(1);
                            value = *fresh7 as ::core::ffi::c_int;
                            p = p.offset(3 as ::core::ffi::c_int as isize);
                        }
                        2 | _ => {
                            p = p.offset(4 as ::core::ffi::c_int as isize);
                        }
                    }
                    match tag {
                        282 => {
                            if den != 0 as ::core::ffi::c_int {
                                xres = (num / den) as ::core::ffi::c_double;
                            }
                        }
                        283 => {
                            if den != 0 as ::core::ffi::c_int {
                                yres = (num / den) as ::core::ffi::c_double;
                            }
                        }
                        296 => match value {
                            2 => {
                                res_unit = 1.0f64;
                            }
                            3 => {
                                res_unit = 2.54f64;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
                *xx = (xres * res_unit) as ::core::ffi::c_int;
                *yy = (yres * res_unit) as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    free(buffer as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn read_jpg_info(mut img: integer) {
    let mut i: ::core::ffi::c_int = 0;
    let mut units: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut appmk: ::core::ffi::c_ushort = 0;
    let mut length: ::core::ffi::c_ushort = 0;
    let mut jpg_id: [::core::ffi::c_uchar; 5] =
        ::core::mem::transmute::<[u8; 5], [::core::ffi::c_uchar; 5]>(*b"JFIF\0");
    let ref mut fresh0 = (*image_array.offset(img as isize)).y_res;
    *fresh0 = 0 as ::core::ffi::c_int as integer;
    (*image_array.offset(img as isize)).x_res = *fresh0;
    let ref mut fresh1 = (*(*image_array.offset(img as isize)).image_struct.jpg).file;
    *fresh1 = xfopen(
        (*image_array.offset(img as isize)).image_name as const_string,
        FOPEN_RBIN_MODE.as_ptr(),
    );
    xfseek(
        (*(*image_array.offset(img as isize)).image_struct.jpg).file,
        0 as ::core::ffi::c_long,
        SEEK_END,
        cur_file_name as const_string,
    );
    (*(*image_array.offset(img as isize)).image_struct.jpg).length = xftell(
        (*(*image_array.offset(img as isize)).image_struct.jpg).file,
        cur_file_name as const_string,
    ) as ::core::ffi::c_ulong;
    xfseek(
        (*(*image_array.offset(img as isize)).image_struct.jpg).file,
        0 as ::core::ffi::c_long,
        SEEK_SET,
        cur_file_name as const_string,
    );
    if read2bytes((*(*image_array.offset(img as isize)).image_struct.jpg).file)
        != 0xffd8 as ::core::ffi::c_uint
    {
        crate::utils::pdftex_fail_args(b"reading JPEG image failed (no JPEG header found)\0" as *const u8
                as *const ::core::ffi::c_char, &[]);
    }
    appmk = read2bytes((*(*image_array.offset(img as isize)).image_struct.jpg).file)
        as ::core::ffi::c_ushort;
    if appmk as ::core::ffi::c_int == 0xffe0 as ::core::ffi::c_int {
        read2bytes((*(*image_array.offset(img as isize)).image_struct.jpg).file);
        i = 0 as ::core::ffi::c_int;
        while i < 5 as ::core::ffi::c_int {
            if xgetc((*(*image_array.offset(img as isize)).image_struct.jpg).file)
                != jpg_id[i as usize] as ::core::ffi::c_int
            {
                break;
            }
            i += 1;
        }
        if i == 5 as ::core::ffi::c_int {
            read2bytes((*(*image_array.offset(img as isize)).image_struct.jpg).file);
            units = xgetc((*(*image_array.offset(img as isize)).image_struct.jpg).file);
            (*image_array.offset(img as isize)).x_res =
                read2bytes((*(*image_array.offset(img as isize)).image_struct.jpg).file) as integer;
            (*image_array.offset(img as isize)).y_res =
                read2bytes((*(*image_array.offset(img as isize)).image_struct.jpg).file) as integer;
            match units {
                1 => {}
                2 => {
                    let ref mut fresh2 = (*image_array.offset(img as isize)).x_res;
                    *fresh2 = (*fresh2 as ::core::ffi::c_double * 2.54f64) as integer;
                    let ref mut fresh3 = (*image_array.offset(img as isize)).y_res;
                    *fresh3 = (*fresh3 as ::core::ffi::c_double * 2.54f64) as integer;
                }
                _ => {
                    let ref mut fresh4 = (*image_array.offset(img as isize)).y_res;
                    *fresh4 = 0 as ::core::ffi::c_int as integer;
                    (*image_array.offset(img as isize)).x_res = *fresh4;
                }
            }
        }
        if (*image_array.offset(img as isize)).x_res == 0 as ::core::ffi::c_int
            && (*image_array.offset(img as isize)).y_res != 0 as ::core::ffi::c_int
        {
            (*image_array.offset(img as isize)).x_res = (*image_array.offset(img as isize)).y_res;
        }
        if (*image_array.offset(img as isize)).y_res == 0 as ::core::ffi::c_int
            && (*image_array.offset(img as isize)).x_res != 0 as ::core::ffi::c_int
        {
            (*image_array.offset(img as isize)).y_res = (*image_array.offset(img as isize)).x_res;
        }
    } else if appmk as ::core::ffi::c_int == 0xffe1 as ::core::ffi::c_int {
        let mut fp: *mut FILE = (*(*image_array.offset(img as isize)).image_struct.jpg).file;
        let mut xxres: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut yyres: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut app_sig: [::core::ffi::c_char; 32] = [0; 32];
        length = (get_unsigned_pair(fp) as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
            as ::core::ffi::c_ushort;
        if length as ::core::ffi::c_int > 5 as ::core::ffi::c_int {
            if fread(
                &raw mut app_sig as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_char>() as size_t,
                5 as size_t,
                fp,
            ) != 5 as ::core::ffi::c_ulong
            {
                return;
            }
            length =
                (length as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as ::core::ffi::c_ushort;
            if memcmp(
                &raw mut app_sig as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                b"Exif\0\0" as *const u8 as *const ::core::ffi::c_char
                    as *const ::core::ffi::c_void,
                5 as size_t,
            ) == 0
            {
                read_APP1_Exif(fp, length, &raw mut xxres, &raw mut yyres);
            }
        }
        (*image_array.offset(img as isize)).x_res = xxres as integer;
        (*image_array.offset(img as isize)).y_res = yyres as integer;
    }
    xfseek(
        (*(*image_array.offset(img as isize)).image_struct.jpg).file,
        0 as ::core::ffi::c_long,
        SEEK_SET,
        cur_file_name as const_string,
    );
    loop {
        if feof((*(*image_array.offset(img as isize)).image_struct.jpg).file) != 0 {
            crate::utils::pdftex_fail_args(b"reading JPEG image failed (premature file end)\0" as *const u8
                    as *const ::core::ffi::c_char, &[]);
        }
        if fgetc((*(*image_array.offset(img as isize)).image_struct.jpg).file)
            != 0xff as ::core::ffi::c_int
        {
            crate::utils::pdftex_fail_args(b"reading JPEG image failed (no marker found)\0" as *const u8
                    as *const ::core::ffi::c_char, &[]);
        }
        let mut current_block_62: u64;
        match xgetc((*(*image_array.offset(img as isize)).image_struct.jpg).file) {
            197 | 198 | 199 | 201 | 202 | 203 | 205 | 206 | 207 => {
                crate::utils::pdftex_fail_args(b"unsupported type of compression\0" as *const u8 as *const ::core::ffi::c_char, &[]);
            }
            194 => {
                if fixedpdfmajorversion == 1 as ::core::ffi::c_int
                    && fixedpdfminorversion <= 2 as ::core::ffi::c_int
                {
                    crate::utils::pdftex_fail_args(b"cannot use progressive DCT with PDF-1.2\0" as *const u8
                            as *const ::core::ffi::c_char, &[]);
                }
                current_block_62 = 3812095161210007121;
            }
            192 | 193 | 195 => {
                current_block_62 = 3812095161210007121;
            }
            216 | 217 | 1 | 208 | 209 | 210 | 211 | 212 | 213 | 214 | 215 => {
                current_block_62 = 12930649117290160518;
            }
            _ => {
                xfseek(
                    (*(*image_array.offset(img as isize)).image_struct.jpg).file,
                    read2bytes((*(*image_array.offset(img as isize)).image_struct.jpg).file)
                        .wrapping_sub(2 as ::core::ffi::c_uint)
                        as ::core::ffi::c_long,
                    SEEK_CUR,
                    cur_file_name as const_string,
                );
                current_block_62 = 12930649117290160518;
            }
        }
        match current_block_62 {
            12930649117290160518 => {}
            _ => {
                read2bytes((*(*image_array.offset(img as isize)).image_struct.jpg).file);
                (*(*image_array.offset(img as isize)).image_struct.jpg).bits_per_component =
                    xgetc((*(*image_array.offset(img as isize)).image_struct.jpg).file)
                        as ::core::ffi::c_uchar;
                (*image_array.offset(img as isize)).height =
                    read2bytes((*(*image_array.offset(img as isize)).image_struct.jpg).file)
                        as integer;
                (*image_array.offset(img as isize)).width =
                    read2bytes((*(*image_array.offset(img as isize)).image_struct.jpg).file)
                        as integer;
                (*(*image_array.offset(img as isize)).image_struct.jpg).color_space =
                    xgetc((*(*image_array.offset(img as isize)).image_struct.jpg).file);
                xfseek(
                    (*(*image_array.offset(img as isize)).image_struct.jpg).file,
                    0 as ::core::ffi::c_long,
                    SEEK_SET,
                    cur_file_name as const_string,
                );
                match (*(*image_array.offset(img as isize)).image_struct.jpg).color_space {
                    JPG_GRAY => {
                        (*image_array.offset(img as isize)).color_type = IMAGE_COLOR_B;
                    }
                    JPG_RGB => {
                        (*image_array.offset(img as isize)).color_type = IMAGE_COLOR_C;
                    }
                    JPG_CMYK => {
                        (*image_array.offset(img as isize)).color_type = IMAGE_COLOR_C;
                    }
                    _ => {
                        crate::utils::pdftex_fail_args(b"Unsupported color space %i\0" as *const u8
                                as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*(*image_array.offset(img as isize)).image_struct.jpg).color_space)]);
                    }
                }
                return;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn write_jpg(mut img: integer) {
    let mut l: ::core::ffi::c_ulong = 0;
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    pdf_puts(b"/Type /XObject\n/Subtype /Image\n\0" as *const u8 as *const ::core::ffi::c_char);
    crate::utils::pdf_printf_args(b"/Width %i\n/Height %i\n/BitsPerComponent %i\n/Length %i\n\0" as *const u8
            as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*image_array.offset(img as isize)).width), crate::utils::PrintfArg::from((*image_array.offset(img as isize)).height), crate::utils::PrintfArg::from((*(*image_array.offset(img as isize)).image_struct.jpg).bits_per_component
            as ::core::ffi::c_int), crate::utils::PrintfArg::from((*(*image_array.offset(img as isize)).image_struct.jpg).length as ::core::ffi::c_int)]);
    pdf_puts(b"/ColorSpace \0" as *const u8 as *const ::core::ffi::c_char);
    if (*image_array.offset(img as isize)).colorspace_ref != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*image_array.offset(img as isize)).colorspace_ref)]);
    } else {
        match (*(*image_array.offset(img as isize)).image_struct.jpg).color_space {
            JPG_GRAY => {
                pdf_puts(b"/DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
            }
            JPG_RGB => {
                pdf_puts(b"/DeviceRGB\n\0" as *const u8 as *const ::core::ffi::c_char);
            }
            JPG_CMYK => {
                pdf_puts(
                    b"/DeviceCMYK\n/Decode [1 0 1 0 1 0 1 0]\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            _ => {
                crate::utils::pdftex_fail_args(b"Unsupported color space %i\0" as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from((*(*image_array.offset(img as isize)).image_struct.jpg).color_space)]);
            }
        }
    }
    pdf_puts(b"/Filter /DCTDecode\n>>\nstream\n\0" as *const u8 as *const ::core::ffi::c_char);
    l = (*(*image_array.offset(img as isize)).image_struct.jpg).length;
    f = (*(*image_array.offset(img as isize)).image_struct.jpg).file;
    while l > 0 as ::core::ffi::c_ulong {
        if (1 as integer + pdfptr) as ::core::ffi::c_uint > pdfbufsize as ::core::ffi::c_uint {
            if pdfosmode != 0 {
                zpdfosgetosbuf(1 as ::core::ffi::c_int);
            } else if 1 as ::core::ffi::c_int as ::core::ffi::c_uint
                > pdfbufsize as ::core::ffi::c_uint
            {
                crate::utils::pdftex_fail_args(b"PDF output buffer overflowed\0" as *const u8 as *const ::core::ffi::c_char, &[]);
            } else {
                pdfflush();
            }
        }
        let fresh11 = pdfptr;
        pdfptr = pdfptr + 1;
        *pdfbuf.offset(fresh11 as isize) = xgetc(f) as eightbits;
        l = l.wrapping_sub(1);
    }
    pdfendstream();
}
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;

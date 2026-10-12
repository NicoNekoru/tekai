pub use crate::generated::pdftex0::{pdfendstream, pdfflush, zpdfosgetosbuf};
pub use crate::utils::pdf_puts;

use crate::stdio::{feof, fread};
use crate::support::{xfopen, xfseek, xftell};
use crate::utils::xgetc;

pub use crate::pngshim::png_struct_def;

pub use crate::pngshim::png_info_def;

extern "C" {
    static mut pdfbuf: *mut eightbits;
    static mut pdfbufsize: integer;
    static mut pdfptr: integer;
    static mut pdfosmode: boolean;
    static mut fixedpdfmajorversion: integer;
    static mut fixedpdfminorversion: integer;
    static mut cur_file_name: *mut ::core::ffi::c_char;
    static mut image_array: *mut image_entry;
}
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
pub type boolean = ::core::ffi::c_int;
pub type const_string = *const ::core::ffi::c_char;
pub type address = *mut ::core::ffi::c_void;
pub type integer = ::core::ffi::c_int;
pub type eightbits = ::core::ffi::c_uchar;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub use crate::backend_types::image_entry;
pub use crate::backend_types::pdf_image_struct;
pub use crate::backend_types::png_image_struct;
pub use crate::backend_types::ImageData as C2RustUnnamed;
pub use crate::backend_types::JBIG2_IMAGE_INFO;
pub use crate::backend_types::JPG_IMAGE_INFO;
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
unsafe fn read2bytes(mut f: *mut FILE) -> ::core::ffi::c_uint {
    let high = xgetc(f);
    let low = xgetc(f);
    if high == libc::EOF || low == libc::EOF {
        crate::utils::pdftex_fail_args(
            b"reading JPEG image failed (premature file end)\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
    }
    ((high as ::core::ffi::c_uint) << 8) | low as ::core::ffi::c_uint
}
pub unsafe fn read_jpg_info(mut img: integer) {
    let mut i: ::core::ffi::c_int = 0;
    let mut units: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut appmk: ::core::ffi::c_ushort = 0;
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
        crate::utils::pdftex_fail_args(
            b"reading JPEG image failed (no JPEG header found)\0" as *const u8
                as *const ::core::ffi::c_char,
            &[],
        );
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
        let fp = (*(*image_array.offset(img as isize)).image_struct.jpg).file;
        let segment_length = read2bytes(fp) as u16;
        let Some(payload_length) = crate::jpeg_exif::app1_payload_length(segment_length) else {
            crate::utils::pdftex_fail_args(
                b"reading JPEG image failed (invalid APP1 segment length)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        };
        // The declared u16 extent bounds both allocation and TIFF traversal.
        let mut payload = vec![0u8; payload_length];
        if fread(
            payload.as_mut_ptr() as *mut ::core::ffi::c_void,
            1 as size_t,
            payload_length as size_t,
            fp,
        ) != payload_length as usize
        {
            crate::utils::pdftex_fail_args(
                b"reading JPEG image failed (premature file end)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        if let Some((xres, yres)) = crate::jpeg_exif::parse_exif_resolution(&payload) {
            (*image_array.offset(img as isize)).x_res = xres as integer;
            (*image_array.offset(img as isize)).y_res = yres as integer;
        }
    }
    xfseek(
        (*(*image_array.offset(img as isize)).image_struct.jpg).file,
        0 as ::core::ffi::c_long,
        SEEK_SET,
        cur_file_name as const_string,
    );
    loop {
        if feof((*(*image_array.offset(img as isize)).image_struct.jpg).file) != 0 {
            crate::utils::pdftex_fail_args(
                b"reading JPEG image failed (premature file end)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        let marker_prefix =
            fast_fgetc((*(*image_array.offset(img as isize)).image_struct.jpg).file);
        if marker_prefix == libc::EOF {
            crate::utils::pdftex_fail_args(
                b"reading JPEG image failed (premature file end)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        if marker_prefix != 0xff as ::core::ffi::c_int {
            crate::utils::pdftex_fail_args(
                b"reading JPEG image failed (no marker found)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        let marker = xgetc((*(*image_array.offset(img as isize)).image_struct.jpg).file);
        if marker == libc::EOF {
            crate::utils::pdftex_fail_args(
                b"reading JPEG image failed (premature file end)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &[],
            );
        }
        let mut current_block_62: u64;
        match marker {
            197 | 198 | 199 | 201 | 202 | 203 | 205 | 206 | 207 => {
                crate::utils::pdftex_fail_args(
                    b"unsupported type of compression\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            194 => {
                if fixedpdfmajorversion == 1 as ::core::ffi::c_int
                    && fixedpdfminorversion <= 2 as ::core::ffi::c_int
                {
                    crate::utils::pdftex_fail_args(
                        b"cannot use progressive DCT with PDF-1.2\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[],
                    );
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
                        crate::utils::pdftex_fail_args(
                            b"Unsupported color space %i\0" as *const u8
                                as *const ::core::ffi::c_char,
                            &[crate::utils::PrintfArg::from(
                                (*(*image_array.offset(img as isize)).image_struct.jpg).color_space,
                            )],
                        );
                    }
                }
                return;
            }
        }
    }
}
pub unsafe fn write_jpg(mut img: integer) {
    let mut l: ::core::ffi::c_ulong = 0;
    let mut f: *mut FILE = ::core::ptr::null_mut::<FILE>();
    pdf_puts(b"/Type /XObject\n/Subtype /Image\n\0" as *const u8 as *const ::core::ffi::c_char);
    crate::utils::pdf_printf_args(
        b"/Width %i\n/Height %i\n/BitsPerComponent %i\n/Length %i\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from((*image_array.offset(img as isize)).width),
            crate::utils::PrintfArg::from((*image_array.offset(img as isize)).height),
            crate::utils::PrintfArg::from(
                (*(*image_array.offset(img as isize)).image_struct.jpg).bits_per_component
                    as ::core::ffi::c_int,
            ),
            crate::utils::PrintfArg::from(
                (*(*image_array.offset(img as isize)).image_struct.jpg).length
                    as ::core::ffi::c_int,
            ),
        ],
    );
    pdf_puts(b"/ColorSpace \0" as *const u8 as *const ::core::ffi::c_char);
    if (*image_array.offset(img as isize)).colorspace_ref != 0 as ::core::ffi::c_int {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*image_array.offset(img as isize)).colorspace_ref,
            )],
        );
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
                crate::utils::pdftex_fail_args(
                    b"Unsupported color space %i\0" as *const u8 as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(
                        (*(*image_array.offset(img as isize)).image_struct.jpg).color_space,
                    )],
                );
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
                crate::utils::pdftex_fail_args(
                    b"PDF output buffer overflowed\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
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

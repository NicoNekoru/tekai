pub use crate::generated::pdftex0::{
    getpdfcompresslevel, pdfbeginstream, pdfendobj, pdfendstream, pdfflush, pdfnewobjnum,
    zpdfbegindict, zpdfbeginobj, zpdfcreateobj, zpdfosgetosbuf,
};
pub use crate::pngshim::{
    png_create_info_struct, png_create_read_struct, png_get_PLTE, png_get_bit_depth,
    png_get_color_type, png_get_gAMA, png_get_gAMA_fixed, png_get_image_height,
    png_get_image_width, png_get_interlace_type, png_get_io_ptr, png_get_rowbytes, png_get_valid,
    png_get_x_pixels_per_meter, png_get_y_pixels_per_meter, png_read_image, png_read_info,
    png_read_row, png_read_update_info, png_set_gamma, png_set_interlace_handling, png_set_option,
    png_set_strip_16, png_set_strip_alpha, png_set_tRNS_to_alpha,
};
pub use crate::utils::pdf_puts;

use crate::byte_string::strcmp;
use crate::environment::getenv;

use crate::stdio::fread;
use crate::support::xfopen;
use crate::web2c::zround;

use crate::allocation::free;
use crate::openclose::xmalloc;

pub use crate::pngshim::png_struct_def;

pub use crate::pngshim::png_info_def;

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
pub type png_byte = ::core::ffi::c_uchar;
pub type png_int_32 = ::core::ffi::c_int;
pub type png_uint_32 = ::core::ffi::c_uint;
pub type png_fixed_point = png_int_32;
pub type png_voidp = *mut ::core::ffi::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const ::core::ffi::c_char;
pub type png_bytepp = *mut *mut png_byte;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_structrp = *mut png_struct;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
pub use crate::pngshim::png_color_struct;
pub type png_color = png_color_struct;
pub type png_colorp = *mut png_color;
pub type png_error_ptr = Option<unsafe fn(png_structp, png_const_charp) -> ()>;
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
pub const FOPEN_RBIN_MODE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_LIBPNG_VER_STRING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"1.6.58\0") };
pub const PNG_FP_1: ::core::ffi::c_int = 100000 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_PALETTE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_COLOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_COLOR_MASK_ALPHA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PNG_COLOR_TYPE_GRAY: ::core::ffi::c_int = 0;
pub const PNG_COLOR_TYPE_PALETTE: ::core::ffi::c_int = 3;
pub const PNG_COLOR_TYPE_RGB: ::core::ffi::c_int = 2;
pub const PNG_COLOR_TYPE_RGB_ALPHA: ::core::ffi::c_int =
    PNG_COLOR_MASK_COLOR | PNG_COLOR_MASK_ALPHA;
pub const PNG_COLOR_TYPE_GRAY_ALPHA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PNG_INTERLACE_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PNG_INFO_gAMA: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
pub const PNG_INFO_sBIT: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const PNG_INFO_cHRM: ::core::ffi::c_uint = 0x4 as ::core::ffi::c_uint;
pub const PNG_INFO_tRNS: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
pub const PNG_INFO_bKGD: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
pub const PNG_INFO_hIST: ::core::ffi::c_uint = 0x40 as ::core::ffi::c_uint;
pub const PNG_INFO_pHYs: ::core::ffi::c_uint = 0x80 as ::core::ffi::c_uint;
pub const PNG_INFO_sRGB: ::core::ffi::c_uint = 0x800 as ::core::ffi::c_uint;
pub const PNG_INFO_iCCP: ::core::ffi::c_uint = 0x1000 as ::core::ffi::c_uint;
pub const PNG_INFO_sPLT: ::core::ffi::c_uint = 0x2000 as ::core::ffi::c_uint;
pub const PNG_MAXIMUM_INFLATE_WINDOW: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PNG_OPTION_ON: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IMAGE_COLOR_B: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IMAGE_COLOR_C: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IMAGE_COLOR_I: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

unsafe fn warn(mut png_ptr: png_structp, mut msg: png_const_charp) {}
pub unsafe fn read_png_info(mut img: integer) {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };
    let __tekai_state_generated_backend_writepng: *mut crate::generated::backend::writepng::State =
        unsafe { crate::generated::backend::writepng::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut png_file: *mut FILE = xfopen(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_name as const_string,
        FOPEN_RBIN_MODE.as_ptr(),
    );
    let ref mut fresh0 = (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .image_struct
    .png
    .png_ptr;
    *fresh0 = png_create_read_struct(
        PNG_LIBPNG_VER_STRING.as_ptr(),
        NULL,
        None,
        Some(warn as unsafe fn(png_structp, png_const_charp) -> ()),
    );
    if (*fresh0).is_null() {
        crate::utils::pdftex_fail_args(
            b"libpng: png_create_read_struct() failed\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    let ref mut fresh1 = (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .image_struct
    .png
    .info_ptr;
    *fresh1 = png_create_info_struct(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
    );
    if (*fresh1).is_null() {
        crate::utils::pdftex_fail_args(
            b"libpng: png_create_info_struct() failed\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
    }
    // The Rust decoder reports errors directly through pdftex_fail_args.
    // It never performs a non-local jump through a libpng callback.
    png_set_option(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_structrp,
        PNG_MAXIMUM_INFLATE_WINDOW,
        PNG_OPTION_ON,
    );
    crate::pngshim::png_init_io(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr
        .cast(),
        png_file,
    );
    png_read_info(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_inforp,
    );
    (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .width = png_get_image_width(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as integer;
    (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .height = png_get_image_height(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as integer;
    if png_get_valid(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
        PNG_INFO_pHYs,
    ) != 0
    {
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .x_res = zround(
            0.0254f64
                * png_get_x_pixels_per_meter(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_double,
        );
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .y_res = zround(
            0.0254f64
                * png_get_y_pixels_per_meter(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_double,
        );
    }
    match png_get_color_type(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
    {
        PNG_COLOR_TYPE_PALETTE => {
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .color_type = IMAGE_COLOR_C | IMAGE_COLOR_I;
        }
        PNG_COLOR_TYPE_GRAY | PNG_COLOR_TYPE_GRAY_ALPHA => {
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .color_type = IMAGE_COLOR_B;
        }
        PNG_COLOR_TYPE_RGB | PNG_COLOR_TYPE_RGB_ALPHA => {
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .color_type = IMAGE_COLOR_C;
        }
        _ => {
            crate::utils::pdftex_fail_args(
                b"unsupported type of color_type <%i>\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(png_get_color_type(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                )
                    as ::core::ffi::c_int)],
            );
        }
    }
    if ((*__tekai_state_generated_pdftexextra).fixedpdfmajorversion > 1 as ::core::ffi::c_int
        || (*__tekai_state_generated_pdftexextra).fixedpdfminorversion >= 4 as ::core::ffi::c_int)
        && (png_get_color_type(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == PNG_COLOR_TYPE_GRAY_ALPHA
            || png_get_color_type(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                == PNG_COLOR_TYPE_RGB_ALPHA)
    {
        if (*__tekai_state_generated_backend_writepng).transparent_page_group
            == 0 as ::core::ffi::c_int
        {
            (*__tekai_state_generated_backend_writepng).transparent_page_group =
                pdfnewobjnum() as ::core::ffi::c_int;
        }
        if (*__tekai_state_generated_pdftexextra).pdfpagegroupval == 0 as ::core::ffi::c_int {
            (*__tekai_state_generated_pdftexextra).pdfpagegroupval =
                (*__tekai_state_generated_backend_writepng).transparent_page_group as integer;
        }
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .group_ref = (*__tekai_state_generated_pdftexextra).pdfpagegroupval;
    }
}
unsafe fn write_png_palette(mut img: integer) {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut r: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: *mut png_bytep = ::core::ptr::null_mut::<png_bytep>();
    let mut palette_objnum: integer = 0 as integer;
    let mut palette: png_colorp = ::core::ptr::null_mut::<png_color>();
    let mut num_palette: ::core::ffi::c_int = 0;
    png_get_PLTE(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_inforp,
        &raw mut palette,
        &raw mut num_palette,
    );
    zpdfcreateobj(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    palette_objnum = (*__tekai_state_generated_pdftexextra).objptr;
    if (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .colorspace_ref
        != 0 as ::core::ffi::c_int
    {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .colorspace_ref,
            )],
        );
    } else {
        crate::utils::pdf_printf_args(
            b"[/Indexed /DeviceRGB %i %i 0 R]\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(num_palette - 1 as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(palette_objnum),
            ],
        );
    }
    pdfbeginstream();
    if png_get_interlace_type(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == PNG_INTERLACE_NONE
    {
        row = xmalloc(
            png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            )
            .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            png_read_row(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_structrp,
                row,
                ::core::ptr::null_mut::<png_byte>(),
            );
            r = row;
            k = png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                    (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint
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
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh57 = r;
                    r = r.offset(1);
                    let fresh58 = (*__tekai_state_generated_pdftexextra).pdfptr;
                    (*__tekai_state_generated_pdftexextra).pdfptr =
                        (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                    *(*__tekai_state_generated_pdftexextra)
                        .pdfbuf
                        .offset(fresh58 as isize) = *fresh57 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            i += 1;
        }
        if !row.is_null() {
            free(row as *mut ::core::ffi::c_void);
        }
        row = ::core::ptr::null_mut::<png_byte>();
    } else {
        if (png_get_image_height(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as size_t)
            .wrapping_mul(png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ))
            >= 10240000 as ::core::ffi::c_long as size_t
        {
            crate::utils::pdftex_warn_args(b"large interlaced PNG might cause out of memory (use non-interlaced PNG to fix this)\0"
                    as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        rows = xmalloc(
            (png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as size_t)
                .wrapping_mul(::core::mem::size_of::<png_bytep>() as size_t),
        ) as *mut png_bytep;
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint)
            < png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            )
        {
            let ref mut fresh59 = *rows.offset(i as isize);
            *fresh59 = xmalloc(
                png_get_rowbytes(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                )
                .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
            ) as *mut png_byte as png_bytep;
            i += 1;
        }
        png_read_image(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_structrp,
            rows as png_bytepp,
        );
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            row = *rows.offset(i as isize);
            k = png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                    (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint
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
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh60 = row;
                    row = row.offset(1);
                    let fresh61 = (*__tekai_state_generated_pdftexextra).pdfptr;
                    (*__tekai_state_generated_pdftexextra).pdfptr =
                        (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                    *(*__tekai_state_generated_pdftexextra)
                        .pdfbuf
                        .offset(fresh61 as isize) = *fresh60 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            if !(*rows.offset(i as isize)).is_null() {
                free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
            }
            let ref mut fresh62 = *rows.offset(i as isize);
            *fresh62 = ::core::ptr::null_mut::<png_byte>();
            i += 1;
        }
        if !rows.is_null() {
            free(rows as *mut ::core::ffi::c_void);
        }
        rows = ::core::ptr::null_mut::<png_bytep>();
    }
    pdfendstream();
    if palette_objnum > 0 as ::core::ffi::c_int {
        zpdfbegindict(palette_objnum, 0 as ::core::ffi::c_int);
        pdfbeginstream();
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint) < num_palette as ::core::ffi::c_uint {
            if (3 as integer + (*__tekai_state_generated_pdftexextra).pdfptr) as ::core::ffi::c_uint
                > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
            {
                if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                    zpdfosgetosbuf(3 as ::core::ffi::c_int);
                } else if 3 as ::core::ffi::c_int as ::core::ffi::c_uint
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
            let fresh63 = (*__tekai_state_generated_pdftexextra).pdfptr;
            (*__tekai_state_generated_pdftexextra).pdfptr =
                (*__tekai_state_generated_pdftexextra).pdfptr + 1;
            *(*__tekai_state_generated_pdftexextra)
                .pdfbuf
                .offset(fresh63 as isize) = (*palette.offset(i as isize)).red as eightbits;
            let fresh64 = (*__tekai_state_generated_pdftexextra).pdfptr;
            (*__tekai_state_generated_pdftexextra).pdfptr =
                (*__tekai_state_generated_pdftexextra).pdfptr + 1;
            *(*__tekai_state_generated_pdftexextra)
                .pdfbuf
                .offset(fresh64 as isize) = (*palette.offset(i as isize)).green as eightbits;
            let fresh65 = (*__tekai_state_generated_pdftexextra).pdfptr;
            (*__tekai_state_generated_pdftexextra).pdfptr =
                (*__tekai_state_generated_pdftexextra).pdfptr + 1;
            *(*__tekai_state_generated_pdftexextra)
                .pdfbuf
                .offset(fresh65 as isize) = (*palette.offset(i as isize)).blue as eightbits;
            i += 1;
        }
        pdfendstream();
    }
}
unsafe fn write_png_gray(mut img: integer) {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut r: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: *mut png_bytep = ::core::ptr::null_mut::<png_bytep>();
    if (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .colorspace_ref
        != 0 as ::core::ffi::c_int
    {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .colorspace_ref,
            )],
        );
    } else {
        pdf_puts(b"/DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pdfbeginstream();
    if png_get_interlace_type(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == PNG_INTERLACE_NONE
    {
        row = xmalloc(
            png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            )
            .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            png_read_row(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_structrp,
                row,
                ::core::ptr::null_mut::<png_byte>(),
            );
            r = row;
            k = png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                    (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint
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
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh31 = r;
                    r = r.offset(1);
                    let fresh32 = (*__tekai_state_generated_pdftexextra).pdfptr;
                    (*__tekai_state_generated_pdftexextra).pdfptr =
                        (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                    *(*__tekai_state_generated_pdftexextra)
                        .pdfbuf
                        .offset(fresh32 as isize) = *fresh31 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            i += 1;
        }
        if !row.is_null() {
            free(row as *mut ::core::ffi::c_void);
        }
        row = ::core::ptr::null_mut::<png_byte>();
    } else {
        if (png_get_image_height(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as size_t)
            .wrapping_mul(png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ))
            >= 10240000 as ::core::ffi::c_long as size_t
        {
            crate::utils::pdftex_warn_args(b"large interlaced PNG might cause out of memory (use non-interlaced PNG to fix this)\0"
                    as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        rows = xmalloc(
            (png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as size_t)
                .wrapping_mul(::core::mem::size_of::<png_bytep>() as size_t),
        ) as *mut png_bytep;
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint)
            < png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            )
        {
            let ref mut fresh33 = *rows.offset(i as isize);
            *fresh33 = xmalloc(
                png_get_rowbytes(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                )
                .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
            ) as *mut png_byte as png_bytep;
            i += 1;
        }
        png_read_image(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_structrp,
            rows as png_bytepp,
        );
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            row = *rows.offset(i as isize);
            k = png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                    (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint
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
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh34 = row;
                    row = row.offset(1);
                    let fresh35 = (*__tekai_state_generated_pdftexextra).pdfptr;
                    (*__tekai_state_generated_pdftexextra).pdfptr =
                        (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                    *(*__tekai_state_generated_pdftexextra)
                        .pdfbuf
                        .offset(fresh35 as isize) = *fresh34 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            if !(*rows.offset(i as isize)).is_null() {
                free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
            }
            let ref mut fresh36 = *rows.offset(i as isize);
            *fresh36 = ::core::ptr::null_mut::<png_byte>();
            i += 1;
        }
        if !rows.is_null() {
            free(rows as *mut ::core::ffi::c_void);
        }
        rows = ::core::ptr::null_mut::<png_bytep>();
    }
    pdfendstream();
}
unsafe fn write_png_gray_alpha(mut img: integer) {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut r: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: *mut png_bytep = ::core::ptr::null_mut::<png_bytep>();
    let mut smask_objnum: integer = 0 as integer;
    let mut smask: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut smask_ptr: integer = 0 as integer;
    let mut smask_size: integer = 0 as integer;
    let mut bitdepth: ::core::ffi::c_int = 0;
    if (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .colorspace_ref
        != 0 as ::core::ffi::c_int
    {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .colorspace_ref,
            )],
        );
    } else {
        pdf_puts(b"/DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    zpdfcreateobj(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    smask_objnum = (*__tekai_state_generated_pdftexextra).objptr;
    crate::utils::pdf_printf_args(
        b"/SMask %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(smask_objnum)],
    );
    smask_size = png_get_rowbytes(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    )
    .wrapping_div(2 as size_t)
    .wrapping_mul(png_get_image_height(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as size_t) as integer;
    smask =
        xmalloc((smask_size as size_t).wrapping_mul(::core::mem::size_of::<png_byte>() as size_t))
            as *mut png_byte as png_bytep;
    pdfbeginstream();
    if png_get_interlace_type(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == PNG_INTERLACE_NONE
    {
        row = xmalloc(
            png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            )
            .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        if png_get_bit_depth(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == 16 as ::core::ffi::c_int
            && (*__tekai_state_generated_pdftexextra).fixedimagehicolor != 0
        {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                png_read_row(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_structrp,
                    row,
                    ::core::ptr::null_mut::<png_byte>(),
                );
                r = row;
                k = png_get_rowbytes(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                        (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                        as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
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
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if j % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            || j % 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                        {
                            let fresh37 = r;
                            r = r.offset(1);
                            let fresh38 = (*__tekai_state_generated_pdftexextra).pdfptr;
                            (*__tekai_state_generated_pdftexextra).pdfptr =
                                (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                            *(*__tekai_state_generated_pdftexextra)
                                .pdfbuf
                                .offset(fresh38 as isize) = *fresh37 as eightbits;
                        } else {
                            let fresh39 = r;
                            r = r.offset(1);
                            let fresh40 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh40 as isize) = *fresh39;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                i += 1;
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                png_read_row(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_structrp,
                    row,
                    ::core::ptr::null_mut::<png_byte>(),
                );
                r = row;
                k = png_get_rowbytes(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                        (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                        as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
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
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if j % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            let fresh41 = r;
                            r = r.offset(1);
                            let fresh42 = (*__tekai_state_generated_pdftexextra).pdfptr;
                            (*__tekai_state_generated_pdftexextra).pdfptr =
                                (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                            *(*__tekai_state_generated_pdftexextra)
                                .pdfbuf
                                .offset(fresh42 as isize) = *fresh41 as eightbits;
                        } else {
                            let fresh43 = r;
                            r = r.offset(1);
                            let fresh44 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh44 as isize) = *fresh43;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                i += 1;
            }
        }
        if !row.is_null() {
            free(row as *mut ::core::ffi::c_void);
        }
        row = ::core::ptr::null_mut::<png_byte>();
    } else {
        if (png_get_image_height(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as size_t)
            .wrapping_mul(png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ))
            >= 10240000 as ::core::ffi::c_long as size_t
        {
            crate::utils::pdftex_warn_args(b"large interlaced PNG might cause out of memory (use non-interlaced PNG to fix this)\0"
                    as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        rows = xmalloc(
            (png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as size_t)
                .wrapping_mul(::core::mem::size_of::<png_bytep>() as size_t),
        ) as *mut png_bytep;
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint)
            < png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            )
        {
            let ref mut fresh45 = *rows.offset(i as isize);
            *fresh45 = xmalloc(
                png_get_rowbytes(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                )
                .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
            ) as *mut png_byte as png_bytep;
            i += 1;
        }
        png_read_image(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_structrp,
            rows as png_bytepp,
        );
        if png_get_bit_depth(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == 16 as ::core::ffi::c_int
            && (*__tekai_state_generated_pdftexextra).fixedimagehicolor != 0
        {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                row = *rows.offset(i as isize);
                k = png_get_rowbytes(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                        (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                        as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
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
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if j % 4 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            || j % 4 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
                        {
                            let fresh46 = row;
                            row = row.offset(1);
                            let fresh47 = (*__tekai_state_generated_pdftexextra).pdfptr;
                            (*__tekai_state_generated_pdftexextra).pdfptr =
                                (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                            *(*__tekai_state_generated_pdftexextra)
                                .pdfbuf
                                .offset(fresh47 as isize) = *fresh46 as eightbits;
                        } else {
                            let fresh48 = row;
                            row = row.offset(1);
                            let fresh49 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh49 as isize) = *fresh48;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                if !(*rows.offset(i as isize)).is_null() {
                    free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
                }
                let ref mut fresh50 = *rows.offset(i as isize);
                *fresh50 = ::core::ptr::null_mut::<png_byte>();
                i += 1;
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                row = *rows.offset(i as isize);
                k = png_get_rowbytes(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int;
                while k > 0 as ::core::ffi::c_int {
                    l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                        (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                    } else {
                        k
                    };
                    if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                        as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                            zpdfosgetosbuf(l);
                        } else if l as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
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
                    j = 0 as ::core::ffi::c_int;
                    while j < l {
                        if j % 2 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                            let fresh51 = row;
                            row = row.offset(1);
                            let fresh52 = (*__tekai_state_generated_pdftexextra).pdfptr;
                            (*__tekai_state_generated_pdftexextra).pdfptr =
                                (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                            *(*__tekai_state_generated_pdftexextra)
                                .pdfbuf
                                .offset(fresh52 as isize) = *fresh51 as eightbits;
                        } else {
                            let fresh53 = row;
                            row = row.offset(1);
                            let fresh54 = smask_ptr;
                            smask_ptr = smask_ptr + 1;
                            *smask.offset(fresh54 as isize) = *fresh53;
                        }
                        j += 1;
                    }
                    k -= l;
                }
                if !(*rows.offset(i as isize)).is_null() {
                    free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
                }
                let ref mut fresh55 = *rows.offset(i as isize);
                *fresh55 = ::core::ptr::null_mut::<png_byte>();
                i += 1;
            }
        }
        if !rows.is_null() {
            free(rows as *mut ::core::ffi::c_void);
        }
        rows = ::core::ptr::null_mut::<png_bytep>();
    }
    pdfendstream();
    pdfflush();
    if smask_objnum > 0 as ::core::ffi::c_int {
        bitdepth = png_get_bit_depth(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int;
        zpdfbegindict(smask_objnum, 0 as ::core::ffi::c_int);
        pdf_puts(b"/Type /XObject\n/Subtype /Image\n\0" as *const u8 as *const ::core::ffi::c_char);
        crate::utils::pdf_printf_args(
            b"/Width %i\n/Height %i\n/BitsPerComponent %i\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(png_get_image_width(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(png_get_image_height(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(if bitdepth == 16 as ::core::ffi::c_int {
                    8 as ::core::ffi::c_int
                } else {
                    bitdepth
                }),
            ],
        );
        pdf_puts(b"/ColorSpace /DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdfbeginstream();
        if bitdepth == 8 as ::core::ffi::c_int {
            crate::utils::pdf_write_bytes(smask, smask_size as usize);
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < smask_size {
                if i % 8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if (8 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                        as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                            zpdfosgetosbuf(8 as ::core::ffi::c_int);
                        } else if 8 as ::core::ffi::c_int as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
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
                }
                let fresh56 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh56 as isize) = *smask.offset(i as isize) as eightbits;
                if bitdepth == 16 as ::core::ffi::c_int {
                    i += 1;
                }
                i += 1;
            }
        }
        if !smask.is_null() {
            free(smask as *mut ::core::ffi::c_void);
        }
        smask = ::core::ptr::null_mut::<png_byte>();
        pdfendstream();
    }
}
unsafe fn write_png_rgb(mut img: integer) {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut r: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: *mut png_bytep = ::core::ptr::null_mut::<png_bytep>();
    if (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .colorspace_ref
        != 0 as ::core::ffi::c_int
    {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .colorspace_ref,
            )],
        );
    } else {
        pdf_puts(b"/DeviceRGB\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    pdfbeginstream();
    if png_get_interlace_type(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == PNG_INTERLACE_NONE
    {
        row = xmalloc(
            png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            )
            .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            png_read_row(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_structrp,
                row,
                ::core::ptr::null_mut::<png_byte>(),
            );
            r = row;
            k = png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                    (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint
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
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh5 = r;
                    r = r.offset(1);
                    let fresh6 = (*__tekai_state_generated_pdftexextra).pdfptr;
                    (*__tekai_state_generated_pdftexextra).pdfptr =
                        (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                    *(*__tekai_state_generated_pdftexextra)
                        .pdfbuf
                        .offset(fresh6 as isize) = *fresh5 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            i += 1;
        }
        if !row.is_null() {
            free(row as *mut ::core::ffi::c_void);
        }
        row = ::core::ptr::null_mut::<png_byte>();
    } else {
        if (png_get_image_height(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as size_t)
            .wrapping_mul(png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ))
            >= 10240000 as ::core::ffi::c_long as size_t
        {
            crate::utils::pdftex_warn_args(b"large interlaced PNG might cause out of memory (use non-interlaced PNG to fix this)\0"
                    as *const u8 as *const ::core::ffi::c_char, &[]);
        }
        rows = xmalloc(
            (png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as size_t)
                .wrapping_mul(::core::mem::size_of::<png_bytep>() as size_t),
        ) as *mut png_bytep;
        i = 0 as ::core::ffi::c_int;
        while (i as ::core::ffi::c_uint)
            < png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            )
        {
            let ref mut fresh7 = *rows.offset(i as isize);
            *fresh7 = xmalloc(
                png_get_rowbytes(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                )
                .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
            ) as *mut png_byte as png_bytep;
            i += 1;
        }
        png_read_image(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_structrp,
            rows as png_bytepp,
        );
        i = 0 as ::core::ffi::c_int;
        while i < png_get_image_height(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            row = *rows.offset(i as isize);
            k = png_get_rowbytes(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int;
            while k > 0 as ::core::ffi::c_int {
                l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                    (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                } else {
                    k
                };
                if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(l);
                    } else if l as ::core::ffi::c_uint
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
                j = 0 as ::core::ffi::c_int;
                while j < l {
                    let fresh8 = row;
                    row = row.offset(1);
                    let fresh9 = (*__tekai_state_generated_pdftexextra).pdfptr;
                    (*__tekai_state_generated_pdftexextra).pdfptr =
                        (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                    *(*__tekai_state_generated_pdftexextra)
                        .pdfbuf
                        .offset(fresh9 as isize) = *fresh8 as eightbits;
                    j += 1;
                }
                k -= l;
            }
            if !(*rows.offset(i as isize)).is_null() {
                free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
            }
            let ref mut fresh10 = *rows.offset(i as isize);
            *fresh10 = ::core::ptr::null_mut::<png_byte>();
            i += 1;
        }
        if !rows.is_null() {
            free(rows as *mut ::core::ffi::c_void);
        }
        rows = ::core::ptr::null_mut::<png_bytep>();
    }
    pdfendstream();
}
unsafe fn try_write_png_rgb_alpha_8_fast(
    mut img: integer,
    mut smask: png_bytep,
    mut smask_ptr: *mut integer,
) -> boolean {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };

    if smask.is_null() || smask_ptr.is_null() {
        return false_0 as boolean;
    }
    let width = png_get_image_width(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as usize;
    let height = png_get_image_height(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as usize;
    let Some(source_rowbytes) = width.checked_mul(4) else {
        return false_0 as boolean;
    };
    let Some(rgb_rowbytes) = width.checked_mul(3) else {
        return false_0 as boolean;
    };
    let Some(expected_len) = source_rowbytes.checked_mul(height) else {
        return false_0 as boolean;
    };

    let mut decoded_rowbytes: size_t = 0;
    let mut decoded_len: size_t = 0;
    let decoded = crate::pngshim::png_decoded_data(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as *mut ::core::ffi::c_void,
        &mut decoded_rowbytes,
        &mut decoded_len,
    );
    if decoded.is_null() || decoded_rowbytes != source_rowbytes || decoded_len < expected_len {
        return false_0 as boolean;
    }

    let rgb_row = xmalloc(rgb_rowbytes as size_t) as *mut u8;
    if rgb_row.is_null() {
        return false_0 as boolean;
    }

    let mut mask_offset = *smask_ptr as usize;
    for y in 0..height {
        let source = decoded.add(y * decoded_rowbytes);
        let mut source_offset = 0usize;
        let mut rgb_offset = 0usize;
        for _ in 0..width {
            *rgb_row.add(rgb_offset) = *source.add(source_offset);
            *rgb_row.add(rgb_offset + 1) = *source.add(source_offset + 1);
            *rgb_row.add(rgb_offset + 2) = *source.add(source_offset + 2);
            *smask.add(mask_offset) = *source.add(source_offset + 3);
            source_offset += 4;
            rgb_offset += 3;
            mask_offset += 1;
        }
        crate::utils::pdf_write_bytes(rgb_row, rgb_rowbytes);
    }

    free(rgb_row as *mut ::core::ffi::c_void);
    *smask_ptr = mask_offset as integer;
    true_0 as boolean
}
unsafe fn try_write_opaque_png_rgb_alpha_8_fast(mut img: integer) -> boolean {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };

    if png_get_bit_depth(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        != 8 as ::core::ffi::c_int
        || png_get_interlace_type(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            != PNG_INTERLACE_NONE
    {
        return false_0 as boolean;
    }

    let width = png_get_image_width(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as usize;
    let height = png_get_image_height(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as usize;
    let Some(source_rowbytes) = width.checked_mul(4) else {
        return false_0 as boolean;
    };
    let Some(rgb_rowbytes) = width.checked_mul(3) else {
        return false_0 as boolean;
    };
    let Some(expected_len) = source_rowbytes.checked_mul(height) else {
        return false_0 as boolean;
    };

    let mut decoded_rowbytes: size_t = 0;
    let mut decoded_len: size_t = 0;
    let decoded = crate::pngshim::png_decoded_data(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as *mut ::core::ffi::c_void,
        &mut decoded_rowbytes,
        &mut decoded_len,
    );
    if decoded.is_null() || decoded_rowbytes != source_rowbytes || decoded_len < expected_len {
        return false_0 as boolean;
    }

    for y in 0..height {
        let source = decoded.add(y * decoded_rowbytes);
        let mut alpha_offset = 3usize;
        while alpha_offset < source_rowbytes {
            if *source.add(alpha_offset) != 255 {
                return false_0 as boolean;
            }
            alpha_offset += 4;
        }
    }

    let rgb_row = xmalloc(rgb_rowbytes as size_t) as *mut u8;
    if rgb_row.is_null() {
        return false_0 as boolean;
    }

    pdfbeginstream();
    for y in 0..height {
        let source = decoded.add(y * decoded_rowbytes);
        let mut source_offset = 0usize;
        let mut rgb_offset = 0usize;
        for _ in 0..width {
            *rgb_row.add(rgb_offset) = *source.add(source_offset);
            *rgb_row.add(rgb_offset + 1) = *source.add(source_offset + 1);
            *rgb_row.add(rgb_offset + 2) = *source.add(source_offset + 2);
            source_offset += 4;
            rgb_offset += 3;
        }
        crate::utils::pdf_write_bytes(rgb_row, rgb_rowbytes);
    }
    pdfendstream();

    free(rgb_row as *mut ::core::ffi::c_void);
    true_0 as boolean
}
unsafe fn write_opaque_smask_bytes(mut len: usize) {
    let buf = [255u8; 8192];
    while len > 0 {
        let chunk = if len > buf.len() { buf.len() } else { len };
        crate::utils::pdf_write_bytes(buf.as_ptr(), chunk);
        len -= chunk;
    }
}

unsafe fn write_png_rgb_alpha(mut img: integer) {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut row: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut r: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut rows: *mut png_bytep = ::core::ptr::null_mut::<png_bytep>();
    let mut smask_objnum: integer = 0 as integer;
    let mut smask: png_bytep = ::core::ptr::null_mut::<png_byte>();
    let mut smask_ptr: integer = 0 as integer;
    let mut smask_size: integer = 0 as integer;
    let mut bitdepth: ::core::ffi::c_int = 0;
    let mut opaque_rgb_alpha_fast: boolean = false_0 as boolean;
    if (*(*__tekai_state_generated_backend_writeimg)
        .image_array
        .offset(img as isize))
    .colorspace_ref
        != 0 as ::core::ffi::c_int
    {
        crate::utils::pdf_printf_args(
            b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
            &[crate::utils::PrintfArg::from(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .colorspace_ref,
            )],
        );
    } else {
        pdf_puts(b"/DeviceRGB\n\0" as *const u8 as *const ::core::ffi::c_char);
    }
    zpdfcreateobj(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    smask_objnum = (*__tekai_state_generated_pdftexextra).objptr;
    crate::utils::pdf_printf_args(
        b"/SMask %i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
        &[crate::utils::PrintfArg::from(smask_objnum)],
    );
    smask_size = png_get_rowbytes(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    )
    .wrapping_div(4 as size_t)
    .wrapping_mul(png_get_image_height(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as size_t) as integer;
    opaque_rgb_alpha_fast = try_write_opaque_png_rgb_alpha_8_fast(img);
    if opaque_rgb_alpha_fast == 0 {
        smask = xmalloc(
            (smask_size as size_t).wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
        ) as *mut png_byte as png_bytep;
        pdfbeginstream();
        let fast_rgb_alpha = png_get_bit_depth(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == 8 as ::core::ffi::c_int
            && png_get_interlace_type(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                == PNG_INTERLACE_NONE
            && try_write_png_rgb_alpha_8_fast(img, smask, &mut smask_ptr) != 0;
        if fast_rgb_alpha {
        } else if png_get_interlace_type(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == PNG_INTERLACE_NONE
        {
            row = xmalloc(
                png_get_rowbytes(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                )
                .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
            ) as *mut png_byte as png_bytep;
            if png_get_bit_depth(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                == 16 as ::core::ffi::c_int
                && (*__tekai_state_generated_pdftexextra).fixedimagehicolor != 0
            {
                i = 0 as ::core::ffi::c_int;
                while i < png_get_image_height(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int
                {
                    png_read_row(
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .png_ptr as png_structrp,
                        row,
                        ::core::ptr::null_mut::<png_byte>(),
                    );
                    r = row;
                    k = png_get_rowbytes(
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .png_ptr as png_const_structrp,
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                    ) as ::core::ffi::c_int;
                    while k > 0 as ::core::ffi::c_int {
                        l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                            (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                        } else {
                            k
                        };
                        if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                            as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
                        {
                            if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                                zpdfosgetosbuf(l);
                            } else if l as ::core::ffi::c_uint
                                > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                    as ::core::ffi::c_uint
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
                        j = 0 as ::core::ffi::c_int;
                        while j < l {
                            if !(j % 8 as ::core::ffi::c_int == 6 as ::core::ffi::c_int
                                || j % 8 as ::core::ffi::c_int == 7 as ::core::ffi::c_int)
                            {
                                let fresh11 = r;
                                r = r.offset(1);
                                let fresh12 = (*__tekai_state_generated_pdftexextra).pdfptr;
                                (*__tekai_state_generated_pdftexextra).pdfptr =
                                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                                *(*__tekai_state_generated_pdftexextra)
                                    .pdfbuf
                                    .offset(fresh12 as isize) = *fresh11 as eightbits;
                            } else {
                                let fresh13 = r;
                                r = r.offset(1);
                                let fresh14 = smask_ptr;
                                smask_ptr = smask_ptr + 1;
                                *smask.offset(fresh14 as isize) = *fresh13;
                            }
                            j += 1;
                        }
                        k -= l;
                    }
                    i += 1;
                }
            } else {
                i = 0 as ::core::ffi::c_int;
                while i < png_get_image_height(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int
                {
                    png_read_row(
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .png_ptr as png_structrp,
                        row,
                        ::core::ptr::null_mut::<png_byte>(),
                    );
                    r = row;
                    k = png_get_rowbytes(
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .png_ptr as png_const_structrp,
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                    ) as ::core::ffi::c_int;
                    while k > 0 as ::core::ffi::c_int {
                        l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                            (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                        } else {
                            k
                        };
                        if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                            as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
                        {
                            if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                                zpdfosgetosbuf(l);
                            } else if l as ::core::ffi::c_uint
                                > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                    as ::core::ffi::c_uint
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
                        j = 0 as ::core::ffi::c_int;
                        while j < l {
                            if j % 4 as ::core::ffi::c_int != 3 as ::core::ffi::c_int {
                                let fresh15 = r;
                                r = r.offset(1);
                                let fresh16 = (*__tekai_state_generated_pdftexextra).pdfptr;
                                (*__tekai_state_generated_pdftexextra).pdfptr =
                                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                                *(*__tekai_state_generated_pdftexextra)
                                    .pdfbuf
                                    .offset(fresh16 as isize) = *fresh15 as eightbits;
                            } else {
                                let fresh17 = r;
                                r = r.offset(1);
                                let fresh18 = smask_ptr;
                                smask_ptr = smask_ptr + 1;
                                *smask.offset(fresh18 as isize) = *fresh17;
                            }
                            j += 1;
                        }
                        k -= l;
                    }
                    i += 1;
                }
            }
            if !row.is_null() {
                free(row as *mut ::core::ffi::c_void);
            }
            row = ::core::ptr::null_mut::<png_byte>();
        } else {
            if (png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as size_t)
                .wrapping_mul(png_get_rowbytes(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ))
                >= 10240000 as ::core::ffi::c_long as size_t
            {
                crate::utils::pdftex_warn_args(b"large interlaced PNG might cause out of memory (use non-interlaced PNG to fix this)\0"
                    as *const u8 as *const ::core::ffi::c_char, &[]);
            }
            rows = xmalloc(
                (png_get_image_height(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as size_t)
                    .wrapping_mul(::core::mem::size_of::<png_bytep>() as size_t),
            ) as *mut png_bytep;
            i = 0 as ::core::ffi::c_int;
            while (i as ::core::ffi::c_uint)
                < png_get_image_height(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                )
            {
                let ref mut fresh19 = *rows.offset(i as isize);
                *fresh19 = xmalloc(
                    png_get_rowbytes(
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .png_ptr as png_const_structrp,
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                    )
                    .wrapping_mul(::core::mem::size_of::<png_byte>() as size_t),
                ) as *mut png_byte as png_bytep;
                i += 1;
            }
            png_read_image(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_structrp,
                rows as png_bytepp,
            );
            if png_get_bit_depth(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                == 16 as ::core::ffi::c_int
                && (*__tekai_state_generated_pdftexextra).fixedimagehicolor != 0
            {
                i = 0 as ::core::ffi::c_int;
                while i < png_get_image_height(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int
                {
                    row = *rows.offset(i as isize);
                    k = png_get_rowbytes(
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .png_ptr as png_const_structrp,
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                    ) as ::core::ffi::c_int;
                    while k > 0 as ::core::ffi::c_int {
                        l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                            (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                        } else {
                            k
                        };
                        if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                            as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
                        {
                            if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                                zpdfosgetosbuf(l);
                            } else if l as ::core::ffi::c_uint
                                > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                    as ::core::ffi::c_uint
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
                        j = 0 as ::core::ffi::c_int;
                        while j < l {
                            if !(j % 8 as ::core::ffi::c_int == 6 as ::core::ffi::c_int
                                || j % 8 as ::core::ffi::c_int == 7 as ::core::ffi::c_int)
                            {
                                let fresh20 = row;
                                row = row.offset(1);
                                let fresh21 = (*__tekai_state_generated_pdftexextra).pdfptr;
                                (*__tekai_state_generated_pdftexextra).pdfptr =
                                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                                *(*__tekai_state_generated_pdftexextra)
                                    .pdfbuf
                                    .offset(fresh21 as isize) = *fresh20 as eightbits;
                            } else {
                                let fresh22 = row;
                                row = row.offset(1);
                                let fresh23 = smask_ptr;
                                smask_ptr = smask_ptr + 1;
                                *smask.offset(fresh23 as isize) = *fresh22;
                            }
                            j += 1;
                        }
                        k -= l;
                    }
                    if !(*rows.offset(i as isize)).is_null() {
                        free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
                    }
                    let ref mut fresh24 = *rows.offset(i as isize);
                    *fresh24 = ::core::ptr::null_mut::<png_byte>();
                    i += 1;
                }
            } else {
                i = 0 as ::core::ffi::c_int;
                while i < png_get_image_height(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int
                {
                    row = *rows.offset(i as isize);
                    k = png_get_rowbytes(
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .png_ptr as png_const_structrp,
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                    ) as ::core::ffi::c_int;
                    while k > 0 as ::core::ffi::c_int {
                        l = if k > (*__tekai_state_generated_pdftexextra).pdfbufsize {
                            (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_int
                        } else {
                            k
                        };
                        if (l as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                            as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
                        {
                            if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                                zpdfosgetosbuf(l);
                            } else if l as ::core::ffi::c_uint
                                > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                    as ::core::ffi::c_uint
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
                        j = 0 as ::core::ffi::c_int;
                        while j < l {
                            if j % 4 as ::core::ffi::c_int != 3 as ::core::ffi::c_int {
                                let fresh25 = row;
                                row = row.offset(1);
                                let fresh26 = (*__tekai_state_generated_pdftexextra).pdfptr;
                                (*__tekai_state_generated_pdftexextra).pdfptr =
                                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                                *(*__tekai_state_generated_pdftexextra)
                                    .pdfbuf
                                    .offset(fresh26 as isize) = *fresh25 as eightbits;
                            } else {
                                let fresh27 = row;
                                row = row.offset(1);
                                let fresh28 = smask_ptr;
                                smask_ptr = smask_ptr + 1;
                                *smask.offset(fresh28 as isize) = *fresh27;
                            }
                            j += 1;
                        }
                        k -= l;
                    }
                    if !(*rows.offset(i as isize)).is_null() {
                        free(*rows.offset(i as isize) as *mut ::core::ffi::c_void);
                    }
                    let ref mut fresh29 = *rows.offset(i as isize);
                    *fresh29 = ::core::ptr::null_mut::<png_byte>();
                    i += 1;
                }
            }
            if !rows.is_null() {
                free(rows as *mut ::core::ffi::c_void);
            }
            rows = ::core::ptr::null_mut::<png_bytep>();
        }
        pdfendstream();
    }
    pdfflush();
    if smask_objnum > 0 as ::core::ffi::c_int {
        bitdepth = png_get_bit_depth(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int;
        zpdfbegindict(smask_objnum, 0 as ::core::ffi::c_int);
        pdf_puts(b"/Type /XObject\n/Subtype /Image\n\0" as *const u8 as *const ::core::ffi::c_char);
        crate::utils::pdf_printf_args(
            b"/Width %i\n/Height %i\n/BitsPerComponent %i\n\0" as *const u8
                as *const ::core::ffi::c_char,
            &[
                crate::utils::PrintfArg::from(png_get_image_width(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(png_get_image_height(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int),
                crate::utils::PrintfArg::from(if bitdepth == 16 as ::core::ffi::c_int {
                    8 as ::core::ffi::c_int
                } else {
                    bitdepth
                }),
            ],
        );
        pdf_puts(b"/ColorSpace /DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
        pdfbeginstream();
        if bitdepth == 8 as ::core::ffi::c_int {
            if opaque_rgb_alpha_fast != 0 {
                write_opaque_smask_bytes(smask_size as usize);
            } else {
                crate::utils::pdf_write_bytes(smask, smask_size as usize);
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < smask_size {
                if i % 8 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    if (8 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                        as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                            zpdfosgetosbuf(8 as ::core::ffi::c_int);
                        } else if 8 as ::core::ffi::c_int as ::core::ffi::c_uint
                            > (*__tekai_state_generated_pdftexextra).pdfbufsize
                                as ::core::ffi::c_uint
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
                }
                let fresh30 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh30 as isize) = *smask.offset(i as isize) as eightbits;
                if bitdepth == 16 as ::core::ffi::c_int {
                    i += 1;
                }
                i += 1;
            }
        }
        if !smask.is_null() {
            free(smask as *mut ::core::ffi::c_void);
        }
        smask = ::core::ptr::null_mut::<png_byte>();
        pdfendstream();
    }
}
pub const SPNG_CHUNK_IDAT: ::core::ffi::c_int = 1229209940;
pub const SPNG_CHUNK_IEND: ::core::ffi::c_int = 1229278788;

unsafe fn png_copy_fail(message: &'static str) -> ! {
    let message = std::ffi::CString::new(message).expect("static PNG error contains no NUL");
    crate::utils::pdftex_fail_args(
        c"writepng: %s".as_ptr(),
        &[crate::utils::PrintfArg::from(message.as_ptr())],
    );
}

unsafe fn read_copy_png_chunk(
    fp: *mut FILE,
    header_offset: u64,
    file_extent: u64,
) -> (usize, u32, u64) {
    let mut header = [0u8; 8];
    if fread(header.as_mut_ptr().cast(), 1, header.len(), fp) != header.len() {
        png_copy_fail("short PNG read");
    }
    let raw_length = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
    let length = crate::pngshim::checked_png_chunk_length(raw_length)
        .unwrap_or_else(|error| png_copy_fail(error));
    let chunk_type = u32::from_be_bytes([header[4], header[5], header[6], header[7]]);
    if chunk_type == SPNG_CHUNK_IEND as u32 && length != 0 {
        png_copy_fail("invalid PNG IEND length");
    }
    let next_offset = crate::pngshim::checked_png_chunk_end(header_offset, length, file_extent)
        .unwrap_or_else(|error| png_copy_fail(error));
    (length, chunk_type, next_offset)
}

unsafe fn copy_png(mut img: integer) {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let fp = png_get_io_ptr(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
    ) as *mut FILE;
    let file_extent =
        crate::pngshim::png_file_extent(fp.cast()).unwrap_or_else(|error| png_copy_fail(error));
    let mut streamlength = 0u64;
    let mut header_offset = 8u64;
    crate::pngshim::seek_png_absolute(fp.cast(), header_offset)
        .unwrap_or_else(|error| png_copy_fail(error));
    loop {
        let (length, chunk_type, next_offset) = read_copy_png_chunk(fp, header_offset, file_extent);
        if chunk_type == SPNG_CHUNK_IEND as u32 {
            break;
        }
        if chunk_type == SPNG_CHUNK_IDAT as u32 {
            streamlength = crate::pngshim::checked_png_idat_total(streamlength, length)
                .unwrap_or_else(|error| png_copy_fail(error));
        }
        // Forward, checked extents include CRC bytes without validating their checksum.
        crate::pngshim::seek_png_absolute(fp.cast(), next_offset)
            .unwrap_or_else(|error| png_copy_fail(error));
        header_offset = next_offset;
    }
    crate::utils::pdf_printf_args(b"/Length %llu\n/Filter/FlateDecode\n/DecodeParms<</Colors %d/Columns %d/BitsPerComponent %i/Predictor 10>>\n>>\nstream\n\0"
            as *const u8 as *const ::core::ffi::c_char, &[crate::utils::PrintfArg::from(streamlength), crate::utils::PrintfArg::from(if png_get_color_type(
            (*(*__tekai_state_generated_backend_writeimg).image_array.offset(img as isize)).image_struct.png.png_ptr
                as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg).image_array.offset(img as isize)).image_struct.png.info_ptr
                as png_const_inforp,
        ) as ::core::ffi::c_int == 2 as ::core::ffi::c_int
        {
            3 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }), crate::utils::PrintfArg::from(png_get_image_width(
            (*(*__tekai_state_generated_backend_writeimg).image_array.offset(img as isize)).image_struct.png.png_ptr
                as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg).image_array.offset(img as isize)).image_struct.png.info_ptr
                as png_const_inforp,
        ) as ::core::ffi::c_int), crate::utils::PrintfArg::from(png_get_bit_depth(
            (*(*__tekai_state_generated_backend_writeimg).image_array.offset(img as isize)).image_struct.png.png_ptr
                as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg).image_array.offset(img as isize)).image_struct.png.info_ptr
                as png_const_inforp,
        ) as ::core::ffi::c_int)]);

    let mut idat = 0;
    let mut copied_length = 0u64;
    header_offset = 8;
    crate::pngshim::seek_png_absolute(fp.cast(), header_offset)
        .unwrap_or_else(|error| png_copy_fail(error));
    loop {
        let (length, chunk_type, next_offset) = read_copy_png_chunk(fp, header_offset, file_extent);
        if chunk_type == SPNG_CHUNK_IDAT as u32 {
            if idat == 2 {
                png_copy_fail("IDAT chunk sequence broken");
            }
            idat = 1;
            copied_length = crate::pngshim::checked_png_idat_total(copied_length, length)
                .unwrap_or_else(|error| png_copy_fail(error));
            let mut remaining = length;
            while remaining > 0 {
                let count = remaining
                    .min((*__tekai_state_generated_pdftexextra).pdfbufsize as usize)
                    as integer;
                if (count + (*__tekai_state_generated_pdftexextra).pdfptr) as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(count);
                    } else if count as ::core::ffi::c_uint
                        > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                    {
                        png_copy_fail("PDF output buffer overflowed");
                    } else {
                        pdfflush();
                    }
                }
                if fread(
                    (*__tekai_state_generated_pdftexextra)
                        .pdfbuf
                        .offset((*__tekai_state_generated_pdftexextra).pdfptr as isize)
                        .cast(),
                    1,
                    count as size_t,
                    fp,
                ) != count as usize
                {
                    png_copy_fail("short PNG read");
                }
                (*__tekai_state_generated_pdftexextra).pdfptr += count;
                remaining -= count as usize;
            }
        } else if chunk_type == SPNG_CHUNK_IEND as u32 {
            if copied_length != streamlength {
                png_copy_fail("PNG IDAT length changed while copying");
            }
            pdfendstream();
            break;
        } else if idat == 1 {
            idat = 2;
        }
        crate::pngshim::seek_png_absolute(fp.cast(), next_offset)
            .unwrap_or_else(|error| png_copy_fail(error));
        header_offset = next_offset;
    }
}

unsafe fn write_additional_png_objects() {
    let __tekai_state_generated_backend_writepng: *mut crate::generated::backend::writepng::State =
        unsafe { crate::generated::backend::writepng::state() };

    if (*__tekai_state_generated_backend_writepng).last_png_needs_page_group != 0 {
        if (*__tekai_state_generated_backend_writepng).transparent_page_group_was_written == 0
            && (*__tekai_state_generated_backend_writepng).transparent_page_group
                > 0 as ::core::ffi::c_int
        {
            (*__tekai_state_generated_backend_writepng).transparent_page_group_was_written =
                true_0 as boolean;
            zpdfbeginobj(
                (*__tekai_state_generated_backend_writepng).transparent_page_group,
                2 as ::core::ffi::c_int,
            );
            if getpdfcompresslevel() == 0 as ::core::ffi::c_int {
                pdf_puts(
                    b"%PTEX Group needed for transparent pngs\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            pdf_puts(
                b"<</Type/Group /S/Transparency /CS/DeviceRGB /I true>>\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            pdfendobj();
        }
    }
}
pub unsafe fn write_png(mut img: integer) {
    let __tekai_state_generated_backend_writeimg: *mut crate::generated::backend::writeimg::State =
        unsafe { crate::generated::backend::writeimg::state() };
    let __tekai_state_generated_backend_writepng: *mut crate::generated::backend::writepng::State =
        unsafe { crate::generated::backend::writepng::state() };
    let __tekai_state_generated_pdftexextra: *mut crate::generated::pdftexextra::State =
        unsafe { crate::generated::pdftexextra::state() };

    let mut png_copy: boolean = true_0;
    let mut gamma: ::core::ffi::c_double = 0.0f64;
    let mut int_file_gamma: png_fixed_point = 0 as png_fixed_point;
    let mut i: ::core::ffi::c_int = 0;
    let mut palette_objnum: integer = 0 as integer;
    let mut palette: png_colorp = ::core::ptr::null_mut::<png_color>();
    let mut num_palette: ::core::ffi::c_int = 0;
    (*__tekai_state_generated_backend_writepng).last_png_needs_page_group = false_0 as boolean;
    png_get_PLTE(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_inforp,
        &raw mut palette,
        &raw mut num_palette,
    );
    if (*__tekai_state_generated_pdftexextra).fixedpdfmajorversion == 1 as ::core::ffi::c_int
        && (*__tekai_state_generated_pdftexextra).fixedpdfminorversion < 5 as ::core::ffi::c_int
    {
        (*__tekai_state_generated_pdftexextra).fixedimagehicolor =
            0 as ::core::ffi::c_int as boolean;
    }
    pdf_puts(b"/Type /XObject\n/Subtype /Image\n\0" as *const u8 as *const ::core::ffi::c_char);
    if png_get_valid(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
        PNG_INFO_tRNS,
    ) != 0
    {
        png_set_tRNS_to_alpha(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_structrp,
        );
        png_copy = false_0 as boolean;
    }
    if (*__tekai_state_generated_pdftexextra).fixedpdfmajorversion == 1 as ::core::ffi::c_int
        && (*__tekai_state_generated_pdftexextra).fixedpdfminorversion < 4 as ::core::ffi::c_int
        && png_get_color_type(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            & PNG_COLOR_MASK_ALPHA
            != 0
    {
        png_set_strip_alpha(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_structrp,
        );
        png_copy = false_0 as boolean;
    }
    if (*__tekai_state_generated_pdftexextra).fixedpdfmajorversion == 1 as ::core::ffi::c_int
        && (*__tekai_state_generated_pdftexextra).fixedpdfminorversion < 5 as ::core::ffi::c_int
    {
        (*__tekai_state_generated_pdftexextra).fixedimagehicolor =
            0 as ::core::ffi::c_int as boolean;
    }
    if png_get_bit_depth(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
    ) as ::core::ffi::c_int
        == 16 as ::core::ffi::c_int
        && (*__tekai_state_generated_pdftexextra).fixedimagehicolor == 0 as ::core::ffi::c_int
    {
        png_set_strip_16(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_structrp,
        );
        png_copy = false_0 as boolean;
    }
    if png_get_valid(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_const_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_const_inforp,
        PNG_INFO_gAMA,
    ) != 0
    {
        png_get_gAMA(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
            &raw mut gamma,
        );
        png_get_gAMA_fixed(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
            &raw mut int_file_gamma,
        );
    }
    if (*__tekai_state_generated_pdftexextra).fixedimageapplygamma != 0 {
        if png_get_valid(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
            PNG_INFO_gAMA,
        ) != 0
        {
            png_set_gamma(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_structrp,
                (*__tekai_state_generated_pdftexextra).fixedgamma as ::core::ffi::c_double
                    / 1000.0f64,
                gamma,
            );
        } else {
            png_set_gamma(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_structrp,
                (*__tekai_state_generated_pdftexextra).fixedgamma as ::core::ffi::c_double
                    / 1000.0f64,
                1000.0f64
                    / (*__tekai_state_generated_pdftexextra).fixedimagegamma
                        as ::core::ffi::c_double,
            );
        }
        png_copy = false_0 as boolean;
    }
    png_set_interlace_handling(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_structrp,
    );
    png_read_update_info(
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .png_ptr as png_structrp,
        (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .image_struct
        .png
        .info_ptr as png_inforp,
    );
    crate::utils::pdf_printf_args(
        b"/Width %i\n/Height %i\n/BitsPerComponent %i\n\0" as *const u8
            as *const ::core::ffi::c_char,
        &[
            crate::utils::PrintfArg::from(png_get_image_width(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int),
            crate::utils::PrintfArg::from(png_get_image_height(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int),
            crate::utils::PrintfArg::from(png_get_bit_depth(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int),
        ],
    );
    pdf_puts(b"/ColorSpace \0" as *const u8 as *const ::core::ffi::c_char);
    if png_copy != 0
        && ((*__tekai_state_generated_pdftexextra).fixedpdfmajorversion > 1 as ::core::ffi::c_int
            || (*__tekai_state_generated_pdftexextra).fixedpdfminorversion
                > 1 as ::core::ffi::c_int)
        && png_get_interlace_type(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == PNG_INTERLACE_NONE
        && (png_get_color_type(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
            == PNG_COLOR_TYPE_GRAY
            || png_get_color_type(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                == PNG_COLOR_TYPE_RGB)
        && (*__tekai_state_generated_pdftexextra).fixedimageapplygamma == 0
        && (png_get_valid(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
            PNG_INFO_gAMA,
        ) == 0
            || int_file_gamma == PNG_FP_1)
        && png_get_valid(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
            PNG_INFO_cHRM
                | PNG_INFO_iCCP
                | PNG_INFO_sBIT
                | PNG_INFO_sRGB
                | PNG_INFO_bKGD
                | PNG_INFO_hIST
                | PNG_INFO_tRNS
                | PNG_INFO_sPLT,
        ) == 0
    {
        if (*(*__tekai_state_generated_backend_writeimg)
            .image_array
            .offset(img as isize))
        .colorspace_ref
            != 0 as ::core::ffi::c_int
        {
            crate::utils::pdf_printf_args(
                b"%i 0 R\n\0" as *const u8 as *const ::core::ffi::c_char,
                &[crate::utils::PrintfArg::from(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .colorspace_ref,
                )],
            );
        } else {
            match png_get_color_type(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
            {
                PNG_COLOR_TYPE_PALETTE => {
                    zpdfcreateobj(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
                    palette_objnum = (*__tekai_state_generated_pdftexextra).objptr;
                    crate::utils::pdf_printf_args(
                        b"[/Indexed /DeviceRGB %i %i 0 R]\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        &[
                            crate::utils::PrintfArg::from(num_palette - 1 as ::core::ffi::c_int),
                            crate::utils::PrintfArg::from(palette_objnum),
                        ],
                    );
                }
                PNG_COLOR_TYPE_GRAY => {
                    pdf_puts(b"/DeviceGray\n\0" as *const u8 as *const ::core::ffi::c_char);
                }
                _ => {
                    pdf_puts(b"/DeviceRGB\n\0" as *const u8 as *const ::core::ffi::c_char);
                }
            }
        }
        crate::utils::tex_printf_args(
            b" (PNG copy)\0" as *const u8 as *const ::core::ffi::c_char,
            &[],
        );
        copy_png(img);
        if palette_objnum > 0 as ::core::ffi::c_int {
            zpdfbegindict(palette_objnum, 0 as ::core::ffi::c_int);
            pdfbeginstream();
            i = 0 as ::core::ffi::c_int;
            while i < num_palette {
                if (3 as integer + (*__tekai_state_generated_pdftexextra).pdfptr)
                    as ::core::ffi::c_uint
                    > (*__tekai_state_generated_pdftexextra).pdfbufsize as ::core::ffi::c_uint
                {
                    if (*__tekai_state_generated_pdftexextra).pdfosmode != 0 {
                        zpdfosgetosbuf(3 as ::core::ffi::c_int);
                    } else if 3 as ::core::ffi::c_int as ::core::ffi::c_uint
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
                    .offset(fresh2 as isize) = (*palette.offset(i as isize)).red as eightbits;
                let fresh3 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh3 as isize) = (*palette.offset(i as isize)).green as eightbits;
                let fresh4 = (*__tekai_state_generated_pdftexextra).pdfptr;
                (*__tekai_state_generated_pdftexextra).pdfptr =
                    (*__tekai_state_generated_pdftexextra).pdfptr + 1;
                *(*__tekai_state_generated_pdftexextra)
                    .pdfbuf
                    .offset(fresh4 as isize) = (*palette.offset(i as isize)).blue as eightbits;
                i += 1;
            }
            pdfendstream();
        }
    } else {
        if !getenv(b"TEXMF_DEBUG_PNG_COPY\0" as *const u8 as *const ::core::ffi::c_char).is_null()
            && strcmp(
                getenv(b"TEXMF_DEBUG_PNG_COPY\0" as *const u8 as *const ::core::ffi::c_char),
                b"1\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            crate::utils::tex_printf_args(
                b" *** PNG copy skipped because:\0" as *const u8 as *const ::core::ffi::c_char,
                &[],
            );
            if png_copy == 0 {
                crate::utils::tex_printf_args(
                    b" !png_copy\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if (*__tekai_state_generated_pdftexextra).fixedpdfmajorversion
                == 1 as ::core::ffi::c_int
                && (*__tekai_state_generated_pdftexextra).fixedpdfminorversion
                    <= 1 as ::core::ffi::c_int
            {
                crate::utils::tex_printf_args(
                    b" minorversion=%d (and majorversion=1)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(
                        (*__tekai_state_generated_pdftexextra).fixedpdfminorversion,
                    )],
                );
            }
            if png_get_interlace_type(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                != PNG_INTERLACE_NONE
            {
                crate::utils::tex_printf_args(
                    b" interlaced\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if !(png_get_color_type(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
            ) as ::core::ffi::c_int
                == PNG_COLOR_TYPE_GRAY
                || png_get_color_type(
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .png_ptr as png_const_structrp,
                    (*(*__tekai_state_generated_backend_writeimg)
                        .image_array
                        .offset(img as isize))
                    .image_struct
                    .png
                    .info_ptr as png_const_inforp,
                ) as ::core::ffi::c_int
                    == PNG_COLOR_TYPE_RGB)
            {
                crate::utils::tex_printf_args(
                    b" colortype\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if (*__tekai_state_generated_pdftexextra).fixedimageapplygamma != 0 {
                crate::utils::tex_printf_args(
                    b" apply gamma\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if !(png_get_valid(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
                PNG_INFO_gAMA,
            ) == 0
                || int_file_gamma == PNG_FP_1)
            {
                crate::utils::tex_printf_args(
                    b" gamma\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
                PNG_INFO_cHRM,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" cHRM\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
                PNG_INFO_iCCP,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" iCCP\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
                PNG_INFO_sBIT,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" sBIT\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
                PNG_INFO_sRGB,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" sRGB\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
                PNG_INFO_bKGD,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" bKGD\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
                PNG_INFO_hIST,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" hIST\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
                PNG_INFO_tRNS,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" tRNS\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
            if png_get_valid(
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .png_ptr as png_const_structrp,
                (*(*__tekai_state_generated_backend_writeimg)
                    .image_array
                    .offset(img as isize))
                .image_struct
                .png
                .info_ptr as png_const_inforp,
                PNG_INFO_sPLT,
            ) != 0
            {
                crate::utils::tex_printf_args(
                    b" sPLT\0" as *const u8 as *const ::core::ffi::c_char,
                    &[],
                );
            }
        }
        match png_get_color_type(
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .png_ptr as png_const_structrp,
            (*(*__tekai_state_generated_backend_writeimg)
                .image_array
                .offset(img as isize))
            .image_struct
            .png
            .info_ptr as png_const_inforp,
        ) as ::core::ffi::c_int
        {
            PNG_COLOR_TYPE_PALETTE => {
                write_png_palette(img);
            }
            PNG_COLOR_TYPE_GRAY => {
                write_png_gray(img);
            }
            PNG_COLOR_TYPE_GRAY_ALPHA => {
                if (*__tekai_state_generated_pdftexextra).fixedpdfminorversion
                    >= 4 as ::core::ffi::c_int
                    || (*__tekai_state_generated_pdftexextra).fixedpdfmajorversion
                        > 1 as ::core::ffi::c_int
                {
                    write_png_gray_alpha(img);
                    (*__tekai_state_generated_backend_writepng).last_png_needs_page_group =
                        true_0 as boolean;
                } else {
                    write_png_gray(img);
                }
            }
            PNG_COLOR_TYPE_RGB => {
                write_png_rgb(img);
            }
            PNG_COLOR_TYPE_RGB_ALPHA => {
                if (*__tekai_state_generated_pdftexextra).fixedpdfminorversion
                    >= 4 as ::core::ffi::c_int
                    || (*__tekai_state_generated_pdftexextra).fixedpdfmajorversion
                        > 1 as ::core::ffi::c_int
                {
                    write_png_rgb_alpha(img);
                    (*__tekai_state_generated_backend_writepng).last_png_needs_page_group =
                        true_0 as boolean;
                } else {
                    write_png_rgb(img);
                }
            }
            _ => {
                crate::utils::pdftex_fail_args(
                    b"unsupported type of color_type <%i>\0" as *const u8
                        as *const ::core::ffi::c_char,
                    &[crate::utils::PrintfArg::from(png_get_color_type(
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .png_ptr as png_const_structrp,
                        (*(*__tekai_state_generated_backend_writeimg)
                            .image_array
                            .offset(img as isize))
                        .image_struct
                        .png
                        .info_ptr as png_const_inforp,
                    )
                        as ::core::ffi::c_int)],
                );
            }
        }
    }
    pdfflush();
    write_additional_png_objects();
}
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;

pub(crate) struct State {
    transparent_page_group: ::core::ffi::c_int,
    last_png_needs_page_group: boolean,
    transparent_page_group_was_written: boolean,
}

#[inline(always)]
fn __tekai_require_copy<T: Copy>() {}

pub(crate) struct StateInitialization {
    destination: *mut State,
    initialized: [bool; 3],
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
            if self.initialized[2] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).transparent_page_group_was_written
                ));
            }
            if self.initialized[1] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).last_png_needs_page_group
                ));
            }
            if self.initialized[0] {
                drop_initialized(core::ptr::addr_of_mut!(
                    (*self.destination).transparent_page_group
                ));
            }
        }
    }
}
impl StateInitialization {
    pub(crate) unsafe fn begin(destination: *mut State) -> Self {
        Self {
            destination,
            initialized: [false; 3],
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
            assert!(core::mem::size_of::<::core::ffi::c_int>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).transparent_page_group)
            .write(0 as ::core::ffi::c_int);
        self.initialized[0] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).last_png_needs_page_group).write(0);
        self.initialized[1] = true;
        const {
            assert!(core::mem::size_of::<boolean>() <= 16 * 1024);
        }
        core::ptr::addr_of_mut!((*destination).transparent_page_group_was_written).write(false_0);
        self.initialized[2] = true;
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
        (*crate::invocation_state::current()).generated_backend_writepng
    ))
}
#[cfg(test)]
pub(crate) unsafe fn check_state_construction(phase: u8) {
    let invocation = crate::invocation_state::current();
    let destination = state();
    assert!(!destination.is_null());
    let _ = (invocation, phase);
}

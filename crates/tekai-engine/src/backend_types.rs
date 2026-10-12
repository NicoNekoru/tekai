//! Canonical backend records shared by generated and handwritten Rust callers.
//!
//! These are invocation data, not owners. Their C layouts remain explicit for
//! raw image unions and the backend's first-field pointer comparisons.

use crate::generated::backend::avl::avl_table;
use crate::pngshim::{png_info_def, png_struct_def};
use crate::stdio::Stream as FILE;
use core::ffi::c_int;

type integer = c_int;
type boolean = c_int;
type internalfontnumber = c_int;
type halfword = c_int;
type png_infop = *mut png_info_def;
type png_structp = *mut png_struct_def;
pub type subfont_entry = _subfont_entry;
pub type fd_entry = fd_entry_;
pub type cw_entry = cw_entry_;
pub type fo_entry = fo_entry_;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct key_entry {
    pub pdfname: *const ::core::ffi::c_char,
    pub t1name: *const ::core::ffi::c_char,
    pub value: ::core::ffi::c_float,
    pub valid: boolean,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _subfont_entry {
    pub infix: *mut ::core::ffi::c_char,
    pub charcodes: [::core::ffi::c_long; 256],
    pub next: *mut subfont_entry,
}

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cw_entry_ {
    pub cw_objnum: integer,
    pub width: *mut integer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct fo_entry_ {
    pub fo_objnum: integer,
    pub tex_font: internalfontnumber,
    pub fm: *mut fm_entry,
    pub fd: *mut fd_entry,
    pub fe: *mut fe_entry,
    pub cw: *mut cw_entry,
    pub first_char: integer,
    pub last_char: integer,
    pub tounicode_objnum: integer,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ff_entry {
    pub ff_name: *mut ::core::ffi::c_char,
    pub ff_path: *mut ::core::ffi::c_char,
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
    pub image_struct: ImageData,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ImageData {
    pub pdf: *mut pdf_image_struct,
    pub png: png_image_struct,
    pub jpg: *mut JPG_IMAGE_INFO,
    pub jbig2: *mut JBIG2_IMAGE_INFO,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct JBIG2_IMAGE_INFO {
    pub selected_page: integer,
    pub file: *mut FILE,
}

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
pub struct chardesc {
    pub charcode: integer,
    pub cwidth: integer,
    pub cheight: integer,
    pub xoff: integer,
    pub yoff: integer,
    pub xescape: integer,
    pub rastersize: integer,
    pub raster: *mut halfword,
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, offset_of, size_of};

    // Assignment, not size equality or casts, proves the exact Rust type identity.
    macro_rules! same_type {
        ($left:ty, $right:ty) => {{
            let left: Option<$left> = None;
            let _: Option<$right> = left;
        }};
    }

    macro_rules! layout {
        ($type:ty, $size:expr, $align:expr, {$($field:ident: $offset:expr,)*}) => {{
            assert_eq!(size_of::<$type>(), $size, stringify!($type));
            assert_eq!(align_of::<$type>(), $align, stringify!($type));
            $(assert_eq!(offset_of!($type, $field), $offset,
                concat!(stringify!($type), "::", stringify!($field)));)*
        }};
    }

    #[test]
    fn generated_aliases_are_identical_rust_types() {
        same_type!(chardesc, crate::generated::backend::pkin::chardesc);
        same_type!(chardesc, crate::generated::backend::writet3::chardesc);
        same_type!(key_entry, crate::generated::backend::writefont::key_entry);
        same_type!(
            _subfont_entry,
            crate::generated::backend::writefont::_subfont_entry
        );
        same_type!(fe_entry, crate::generated::backend::writefont::fe_entry);
        same_type!(fm_entry, crate::generated::backend::writefont::fm_entry);
        same_type!(intparm, crate::generated::backend::writefont::intparm);
        same_type!(fd_entry_, crate::generated::backend::writefont::fd_entry_);
        same_type!(cw_entry_, crate::generated::backend::writefont::cw_entry_);
        same_type!(fo_entry_, crate::generated::backend::writefont::fo_entry_);
        same_type!(key_entry, crate::generated::backend::writet1::key_entry);
        same_type!(
            _subfont_entry,
            crate::generated::backend::writet1::_subfont_entry
        );
        same_type!(fe_entry, crate::generated::backend::writet1::fe_entry);
        same_type!(fm_entry, crate::generated::backend::writet1::fm_entry);
        same_type!(intparm, crate::generated::backend::writet1::intparm);
        same_type!(fd_entry_, crate::generated::backend::writet1::fd_entry_);
        same_type!(ff_entry, crate::generated::backend::writet1::ff_entry);
        same_type!(key_entry, crate::generated::backend::writet3::key_entry);
        same_type!(
            _subfont_entry,
            crate::generated::backend::writet3::_subfont_entry
        );
        same_type!(fe_entry, crate::generated::backend::writet3::fe_entry);
        same_type!(fm_entry, crate::generated::backend::writet3::fm_entry);
        same_type!(
            _subfont_entry,
            crate::generated::backend::writettf::_subfont_entry
        );
        same_type!(fe_entry, crate::generated::backend::writettf::fe_entry);
        same_type!(fm_entry, crate::generated::backend::writettf::fm_entry);
        same_type!(intparm, crate::generated::backend::writettf::intparm);
        same_type!(fd_entry_, crate::generated::backend::writettf::fd_entry_);
        same_type!(
            _subfont_entry,
            crate::generated::backend::mapfile::_subfont_entry
        );
        same_type!(fe_entry, crate::generated::backend::mapfile::fe_entry);
        same_type!(fm_entry, crate::generated::backend::mapfile::fm_entry);
        same_type!(ff_entry, crate::generated::backend::mapfile::ff_entry);
        same_type!(fe_entry, crate::generated::backend::writeenc::fe_entry);
        same_type!(
            _subfont_entry,
            crate::generated::backend::subfont::_subfont_entry
        );
        same_type!(fm_entry, crate::generated::backend::subfont::fm_entry);
        same_type!(
            image_entry,
            crate::generated::backend::writeimg::image_entry
        );
        same_type!(
            ImageData,
            crate::generated::backend::writeimg::C2RustUnnamed
        );
        same_type!(
            JBIG2_IMAGE_INFO,
            crate::generated::backend::writeimg::JBIG2_IMAGE_INFO
        );
        same_type!(
            JPG_IMAGE_INFO,
            crate::generated::backend::writeimg::JPG_IMAGE_INFO
        );
        same_type!(
            png_image_struct,
            crate::generated::backend::writeimg::png_image_struct
        );
        same_type!(
            pdf_image_struct,
            crate::generated::backend::writeimg::pdf_image_struct
        );
        same_type!(
            crate::pngshim::png_struct_def,
            crate::generated::backend::writeimg::png_struct_def
        );
        same_type!(
            crate::pngshim::png_info_def,
            crate::generated::backend::writeimg::png_info_def
        );
        same_type!(
            JPG_IMAGE_INFO,
            crate::generated::backend::writepng::JPG_IMAGE_INFO
        );
        same_type!(
            png_image_struct,
            crate::generated::backend::writepng::png_image_struct
        );
        same_type!(
            pdf_image_struct,
            crate::generated::backend::writepng::pdf_image_struct
        );
        same_type!(
            JBIG2_IMAGE_INFO,
            crate::generated::backend::writepng::JBIG2_IMAGE_INFO
        );
        same_type!(
            image_entry,
            crate::generated::backend::writepng::image_entry
        );
        same_type!(
            ImageData,
            crate::generated::backend::writepng::C2RustUnnamed
        );
        same_type!(
            crate::pngshim::png_struct_def,
            crate::generated::backend::writepng::png_struct_def
        );
        same_type!(
            crate::pngshim::png_info_def,
            crate::generated::backend::writepng::png_info_def
        );
        same_type!(
            crate::pngshim::png_color_struct,
            crate::generated::backend::writepng::png_color_struct
        );
        same_type!(
            JPG_IMAGE_INFO,
            crate::generated::backend::writejpg::JPG_IMAGE_INFO
        );
        same_type!(
            png_image_struct,
            crate::generated::backend::writejpg::png_image_struct
        );
        same_type!(
            pdf_image_struct,
            crate::generated::backend::writejpg::pdf_image_struct
        );
        same_type!(
            JBIG2_IMAGE_INFO,
            crate::generated::backend::writejpg::JBIG2_IMAGE_INFO
        );
        same_type!(
            image_entry,
            crate::generated::backend::writejpg::image_entry
        );
        same_type!(
            ImageData,
            crate::generated::backend::writejpg::C2RustUnnamed
        );
        same_type!(
            crate::pngshim::png_struct_def,
            crate::generated::backend::writejpg::png_struct_def
        );
        same_type!(
            crate::pngshim::png_info_def,
            crate::generated::backend::writejpg::png_info_def
        );
        same_type!(
            JPG_IMAGE_INFO,
            crate::generated::backend::writejbig2::JPG_IMAGE_INFO
        );
        same_type!(
            png_image_struct,
            crate::generated::backend::writejbig2::png_image_struct
        );
        same_type!(
            pdf_image_struct,
            crate::generated::backend::writejbig2::pdf_image_struct
        );
        same_type!(
            JBIG2_IMAGE_INFO,
            crate::generated::backend::writejbig2::JBIG2_IMAGE_INFO
        );
        same_type!(
            image_entry,
            crate::generated::backend::writejbig2::image_entry
        );
        same_type!(
            ImageData,
            crate::generated::backend::writejbig2::C2RustUnnamed
        );
        same_type!(
            crate::pngshim::png_struct_def,
            crate::generated::backend::writejbig2::png_struct_def
        );
        same_type!(
            crate::pngshim::png_info_def,
            crate::generated::backend::writejbig2::png_info_def
        );
        same_type!(
            _subfont_entry,
            crate::generated::backend::epdf::_subfont_entry
        );
        same_type!(fe_entry, crate::generated::backend::epdf::fe_entry);
        same_type!(fm_entry, crate::generated::backend::epdf::fm_entry);
        same_type!(intparm, crate::generated::backend::epdf::intparm);
        same_type!(fd_entry_, crate::generated::backend::epdf::fd_entry_);
        same_type!(fd_entry, crate::utils::FdEntry);
        same_type!(fe_entry, crate::utils::FeEntry);
        same_type!(fm_entry, crate::utils::FmEntry);
        same_type!(intparm, crate::utils::IntParm);
    }

    // Frozen field-for-field fixtures from the prior repr(C) definitions, for
    // the two supported 64-bit macOS architectures. No native library is needed.
    #[test]
    #[cfg(all(target_os = "macos", target_pointer_width = "64"))]
    fn backend_layouts_preserve_every_field() {
        layout!(chardesc, 40, 8, {
            charcode: 0,
            cwidth: 4,
            cheight: 8,
            xoff: 12,
            yoff: 16,
            xescape: 20,
            rastersize: 24,
            raster: 32,
        });
        layout!(key_entry, 24, 8, {
            pdfname: 0,
            t1name: 8,
            value: 16,
            valid: 20,
        });
        layout!(_subfont_entry, 2064, 8, {
            infix: 0,
            charcodes: 8,
            next: 2056,
        });
        layout!(fe_entry, 32, 8, {
            fe_objnum: 0,
            name: 8,
            glyph_names: 16,
            tx_tree: 24,
        });
        layout!(fm_entry, 80, 8, {
            tfm_name: 0,
            sfd_name: 8,
            ps_name: 16,
            fd_flags: 24,
            slant: 28,
            extend: 32,
            encname: 40,
            ff_name: 48,
            type_0: 56,
            pid: 58,
            eid: 60,
            subfont: 64,
            links: 72,
            in_use: 76,
        });
        layout!(intparm, 8, 4, {
            val: 0,
            set: 4,
        });
        layout!(fd_entry_, 176, 8, {
            fd_objnum: 0,
            fontname: 8,
            subset_tag: 16,
            ff_found: 24,
            ff_objnum: 28,
            fn_objnum: 32,
            all_glyphs: 36,
            write_ttf_glyph_names: 40,
            font_dim: 44,
            fe: 136,
            builtin_glyph_names: 144,
            fm: 152,
            tx_tree: 160,
            gl_tree: 168,
        });
        layout!(cw_entry_, 16, 8, {
            cw_objnum: 0,
            width: 8,
        });
        layout!(fo_entry_, 56, 8, {
            fo_objnum: 0,
            tex_font: 4,
            fm: 8,
            fd: 16,
            fe: 24,
            cw: 32,
            first_char: 40,
            last_char: 44,
            tounicode_objnum: 48,
        });
        layout!(ff_entry, 16, 8, {
            ff_name: 0,
            ff_path: 8,
        });
        layout!(png_image_struct, 16, 8, {
            png_ptr: 0,
            info_ptr: 8,
        });
        layout!(ImageData, 16, 8, {
            pdf: 0,
            png: 0,
            jpg: 0,
            jbig2: 0,
        });
        layout!(image_entry, 64, 8, {
            image_name: 0,
            image_type: 8,
            color_type: 12,
            width: 16,
            height: 20,
            rotate: 24,
            x_res: 28,
            y_res: 32,
            num_pages: 36,
            colorspace_ref: 40,
            group_ref: 44,
            image_struct: 48,
        });
        layout!(JBIG2_IMAGE_INFO, 16, 8, {
            selected_page: 0,
            file: 8,
        });
        layout!(JPG_IMAGE_INFO, 24, 8, {
            color_space: 0,
            bits_per_component: 4,
            length: 8,
            file: 16,
        });
        layout!(pdf_image_struct, 24, 8, {
            orig_x: 0,
            orig_y: 4,
            selected_page: 8,
            page_box: 12,
            doc: 16,
        });
    }
}

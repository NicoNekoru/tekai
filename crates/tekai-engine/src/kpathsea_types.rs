// Upstream-derived records with subsequent Rust modifications for Tekai.
// Upstream TeX Live source revision 52dac44dde13a8310bbb8231e81c58be28cebe20.
// Origin texk/kpathsea/types.h and related boundary-record headers.
// LGPL-2.1-or-later upstream records; GPL-2.0-or-later combined crate.
// Modified for Tekai. Licensing and provenance notice restored 2026-10-13.
// Original copyrights and license terms are retained in the crate's
// NOTICE.md and licenses/upstream-notices.txt.

//! Canonical kpathsea boundary records.
//!
//! All generated modules and the Rust implementation share these exact types.
//! The callback ABI is retained here until its callers migrate together.

use crate::stdio::Stream as FILE;
use core::ffi::{c_char, c_int, c_uint};

type boolean = c_int;
pub type string = *mut c_char;
pub type const_string = *const c_char;
pub type p_record_input = Option<unsafe fn(const_string)>;
pub type p_record_output = Option<unsafe fn(const_string)>;
pub type str_llist_type = *mut str_llist_elt;
pub type hash_element_type = hash_element_struct;
pub type kpse_file_format_type = c_uint;
pub type kpse_src_type = c_uint;
pub type kpathsea = *mut kpathsea_instance;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct str_llist_elt {
    pub str_0: string,
    pub moved: boolean,
    pub next: *mut str_llist_elt,
}

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct KpseGlyphFileType {
    pub name: const_string,
    pub dpi: ::core::ffi::c_uint,
    pub format: kpse_file_format_type,
    pub source: c_uint,
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
        same_type!(str_llist_elt, crate::generated::pdftexextra::str_llist_elt);
        same_type!(cache_entry, crate::generated::pdftexextra::cache_entry);
        same_type!(
            expansion_type,
            crate::generated::pdftexextra::expansion_type
        );
        same_type!(
            hash_element_struct,
            crate::generated::pdftexextra::hash_element_struct
        );
        same_type!(
            hash_table_type,
            crate::generated::pdftexextra::hash_table_type
        );
        same_type!(str_list_type, crate::generated::pdftexextra::str_list_type);
        same_type!(
            kpse_format_info_type,
            crate::generated::pdftexextra::kpse_format_info_type
        );
        same_type!(
            kpathsea_instance,
            crate::generated::pdftexextra::kpathsea_instance
        );
        same_type!(str_llist_elt, crate::generated::pdftex0::str_llist_elt);
        same_type!(cache_entry, crate::generated::pdftex0::cache_entry);
        same_type!(expansion_type, crate::generated::pdftex0::expansion_type);
        same_type!(
            hash_element_struct,
            crate::generated::pdftex0::hash_element_struct
        );
        same_type!(hash_table_type, crate::generated::pdftex0::hash_table_type);
        same_type!(str_list_type, crate::generated::pdftex0::str_list_type);
        same_type!(
            kpse_format_info_type,
            crate::generated::pdftex0::kpse_format_info_type
        );
        same_type!(
            kpathsea_instance,
            crate::generated::pdftex0::kpathsea_instance
        );
        same_type!(str_llist_elt, crate::generated::pdftexini::str_llist_elt);
        same_type!(cache_entry, crate::generated::pdftexini::cache_entry);
        same_type!(expansion_type, crate::generated::pdftexini::expansion_type);
        same_type!(
            hash_element_struct,
            crate::generated::pdftexini::hash_element_struct
        );
        same_type!(
            hash_table_type,
            crate::generated::pdftexini::hash_table_type
        );
        same_type!(str_list_type, crate::generated::pdftexini::str_list_type);
        same_type!(
            kpse_format_info_type,
            crate::generated::pdftexini::kpse_format_info_type
        );
        same_type!(
            kpathsea_instance,
            crate::generated::pdftexini::kpathsea_instance
        );
        same_type!(KpseGlyphFileType, crate::kpathsea::KpseGlyphFileType);
        same_type!(
            KpseGlyphFileType,
            crate::generated::backend::writet3::kpse_glyph_file_type
        );
    }

    // Frozen field-for-field fixtures from the prior repr(C) definitions, for
    // the two supported 64-bit macOS architectures. No native library is needed.
    #[test]
    #[cfg(all(target_os = "macos", target_pointer_width = "64"))]
    fn kpathsea_layouts_preserve_every_field() {
        layout!(str_llist_elt, 24, 8, {
            str_0: 0,
            moved: 8,
            next: 16,
        });
        layout!(cache_entry, 16, 8, {
            key: 0,
            value: 8,
        });
        layout!(expansion_type, 16, 8, {
            var: 0,
            expanding: 8,
        });
        layout!(hash_element_struct, 24, 8, {
            key: 0,
            value: 8,
            next: 16,
        });
        layout!(hash_table_type, 16, 8, {
            buckets: 0,
            size: 8,
        });
        layout!(str_list_type, 16, 8, {
            length: 0,
            list: 8,
        });
        layout!(kpse_format_info_type, 128, 8, {
            type_0: 0,
            path: 8,
            raw_path: 16,
            path_source: 24,
            override_path: 32,
            client_path: 40,
            cnf_path: 48,
            default_path: 56,
            suffix: 64,
            alt_suffix: 72,
            suffix_search_only: 80,
            program: 88,
            argc: 96,
            argv: 104,
            program_enabled_p: 112,
            program_enable_level: 116,
            binmode: 120,
        });
        layout!(kpathsea_instance, 7864, 8, {
            record_input: 0,
            record_output: 8,
            cnf_hash: 16,
            doing_cnf_init: 32,
            db: 40,
            alias_db: 56,
            db_dir_list: 72,
            debug: 88,
            link_table: 96,
            the_cache: 112,
            cache_length: 120,
            map: 128,
            map_path: 144,
            debug_hash_lookup_int: 152,
            elt: 160,
            elt_alloc: 168,
            path: 176,
            followup_search: 184,
            log_file: 192,
            log_opened: 200,
            invocation_name: 208,
            invocation_short_name: 216,
            program_name: 224,
            ll_verbose: 232,
            fallback_font: 240,
            fallback_resolutions_string: 248,
            fallback_resolutions: 256,
            format_info: 264,
            make_tex_discard_errors: 7816,
            missfont: 7824,
            expansions: 7832,
            expansion_len: 7840,
            saved_env: 7848,
            saved_count: 7856,
        });
        layout!(KpseGlyphFileType, 24, 8, {
            name: 0,
            dpi: 8,
            format: 12,
            source: 16,
        });
    }
}

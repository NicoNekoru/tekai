//! Keep large immutable assets in object data, not Rust compiler metadata.
//! The assembler supplies read-only symbols and each asset's actual byte length.

include!(concat!(env!("OUT_DIR"), "/embedded_data.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn embedded_bytes_match_checked_in_assets() {
        for (bytes, relative) in [
            (super::texmf_bytes(), "runtime/texmf.tar.gz"),
            (super::font_outlines_bytes(), "runtime/font-outlines.tar.gz"),
            (super::pdflatex_format_bytes(), "formats/pdflatex.fmt"),
        ] {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join(relative);
            let original = std::fs::read(path).unwrap();
            assert!(bytes == original, "embedded bytes changed for {relative}");
        }
    }
}

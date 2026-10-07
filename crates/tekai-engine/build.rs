use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let object_embed =
        matches!(os.as_str(), "macos" | "linux") && matches!(arch.as_str(), "aarch64" | "x86_64");
    let mut source = String::new();
    let mut assembly = String::new();
    if object_embed {
        assembly.push_str(if os == "macos" {
            ".section __TEXT,__const\n"
        } else {
            ".section .rodata.tekai_embedded,\"a\"\n"
        });
    }
    for (name, relative) in [
        ("texmf", "runtime/texmf.tar.gz"),
        ("font_outlines", "runtime/font-outlines.tar.gz"),
        ("pdflatex_format", "formats/pdflatex.fmt"),
    ] {
        let path = manifest
            .join("../..")
            .join(relative)
            .canonicalize()
            .unwrap();
        println!("cargo:rerun-if-changed={}", path.display());
        let path = path.to_str().expect("embedded asset paths must be UTF-8");
        if object_embed {
            let symbol = format!("tekai_embedded_{name}");
            let label = if os == "macos" {
                format!("_{symbol}")
            } else {
                symbol.clone()
            };
            let visibility = if os == "macos" {
                ".private_extern"
            } else {
                ".hidden"
            };
            let local = if os == "macos" {
                format!("L{symbol}")
            } else {
                format!(".L{symbol}")
            };
            // .incbin strings are assembler strings, not Rust literals.
            let quoted_path = path.replace('\\', "\\\\").replace('"', "\\\"");
            assembly.push_str(&format!(
                ".balign 8\n.globl {label}\n{visibility} {label}\n{label}:\n{local}_begin:\n\
                 .incbin \"{quoted_path}\"\n{local}_end:\n.balign 8\n\
                 .globl {label}_len\n{visibility} {label}_len\n{label}_len:\n\
                 .quad {local}_end - {local}_begin\n"
            ));
            source.push_str(&format!(
                "unsafe extern \"C\" {{\n    #[link_name = {symbol:?}]\n    static {name}: u8;\n\
                 #[link_name = {len_symbol:?}]\n    static {name}_len: usize;\n}}\n\
                 pub(super) fn {name}_bytes() -> &'static [u8] {{\n\
                 // SAFETY: the assembler stores the actual length of this immutable,\n\
                 // contiguous region in an aligned usize (both supported targets are 64-bit).\n\
                 // No separately measured file length or one-past-the-end reference is used.\n\
                 unsafe {{\n\
                 let start = core::ptr::addr_of!({name});\n\
                 core::slice::from_raw_parts(start, {name}_len)\n\
                 }}\n}}\n",
                len_symbol = format!("{symbol}_len"),
            ));
        } else {
            // Preserve portability where the inline assembler/object format is
            // not supported. A static slice is still smaller than a const slice.
            source.push_str(&format!(
                "static {name}: &[u8] = include_bytes!({path:?});\n\
                 pub(super) fn {name}_bytes() -> &'static [u8] {{ {name} }}\n"
            ));
        }
    }
    if object_embed {
        source.push_str(&format!(
            "core::arch::global_asm!({assembly:?}, options(raw));\n"
        ));
    }
    fs::write(output.join("embedded_data.rs"), source).unwrap();
}

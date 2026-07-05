use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest_dir.parent().unwrap().parent().unwrap();
    let texlive = root.join("third_party/texlive-source");
    let build = root.join("target/pdftex-port/texlive-build");

    println!("cargo:rerun-if-changed=cpp/xpdf_bridge.cc");

    let mut bridge = cc::Build::new();
    bridge
        .cpp(true)
        .file("cpp/xpdf_bridge.cc")
        .flag_if_supported("-Wno-write-strings")
        .flag_if_supported("-Wno-unused-parameter")
        .include(build.join("libs/xpdf"))
        .include(build.join("libs/xpdf/xpdf"))
        .include(texlive.join("libs/xpdf/xpdf-src/goo"))
        .include(texlive.join("libs/xpdf/xpdf-src/fofi"))
        .include(texlive.join("libs/xpdf/xpdf-src/xpdf"));
    bridge.compile("pdftex_xpdf_bridge");
}

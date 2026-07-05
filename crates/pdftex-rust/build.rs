use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest_dir.parent().unwrap().parent().unwrap();
    let texlive = root.join("third_party/texlive-source");
    let build = root.join("target/pdftex-port/texlive-build");
    let web2c = build.join("texk/web2c");

    println!("cargo:rerun-if-changed=cpp/xpdf_bridge.cc");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_RUST_BINARY");

    let xpdf_config = build.join("libs/xpdf/aconf.h");
    if !xpdf_config.exists() {
        panic!(
            "missing {}; run `scripts/pdftex_port.py build-upstream` before building pdftex-rust",
            xpdf_config.display()
        );
    }

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

    if std::env::var_os("CARGO_FEATURE_RUST_BINARY").is_some() {
        let required = [
            web2c.join("pdftexd.h"),
            build.join("libs/xpdf/libxpdf.a"),
        ];
        for path in required {
            if !path.exists() {
                panic!(
                    "missing {}; run `scripts/pdftex_port.py build-upstream` before building the Cargo pdfTeX binary",
                    path.display()
                );
            }
        }

        println!(
            "cargo:rustc-link-search=native={}",
            build.join("libs/xpdf").display()
        );
        println!("cargo:rustc-link-lib=static=xpdf");
    }
}

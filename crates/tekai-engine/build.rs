fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_RUST_BINARY");
    if std::env::var_os("CARGO_FEATURE_RUST_BINARY").is_some()
        && std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos")
    {
        println!("cargo:rustc-link-arg-bin=tekai-engine=-Wl,-dead_strip_dylibs");
    }
}

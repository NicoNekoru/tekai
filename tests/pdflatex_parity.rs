use std::process::Command;

#[test]
#[ignore = "downloads/templates local papers and runs a full TeX toolchain"]
fn parity_suite_matches_direct_pdflatex() {
    let status = Command::new("scripts/verify_pdflatex_parity.sh")
        .status()
        .expect("failed to launch pdflatex parity script");
    assert!(status.success(), "pdflatex parity script failed");
}

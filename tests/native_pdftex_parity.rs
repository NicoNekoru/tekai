use std::fs;
use std::process::Command;

#[test]
fn native_pdftex_parity_reads_native_trace_caption_pages() {
    let workdir = std::env::temp_dir().join(format!(
        "texpilot-native-pdftex-trace-captions-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&workdir);
    fs::create_dir_all(&workdir).expect("failed to create temp directory");
    let trace = workdir.join("main.texpilot-pdftex.trace");
    fs::write(
        &trace,
        concat!(
            "engine\ttexpilot-pdftex-native\n",
            "layout_caption_entries\t1\n",
            "layout_caption\tpage=42 slot=66 line=2580 kind=figure text=Figure 17. Depiction of optimized beta values\n",
        ),
    )
    .expect("failed to write trace");

    let code = r#"
import importlib.util
import os
import sys
from pathlib import Path

spec = importlib.util.spec_from_file_location("native_pdftex_parity", "scripts/native_pdftex_parity.py")
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)
hint = module.SourceCaptionHint("Figure", 17, module.source_caption_tokens("Depiction of optimized beta values"))
captions = module.trace_caption_occurrences(Path(os.environ["TRACE_PATH"]), {("Figure", 17): hint})
assert len(captions) == 1, captions
assert captions[0].kind == "Figure", captions
assert captions[0].number == 17, captions
assert captions[0].page == 42, captions
"#;
    let status = Command::new("python3")
        .arg("-c")
        .arg(code)
        .env("TRACE_PATH", &trace)
        .status()
        .expect("failed to run Python trace-caption parser check");

    let _ = fs::remove_dir_all(&workdir);
    assert!(status.success(), "trace-caption parser check failed");
}

#[test]
fn native_pdftex_parity_summarizes_caption_drift_for_gates() {
    let code = r#"
import importlib.util
import sys
from pathlib import Path
from types import SimpleNamespace

spec = importlib.util.spec_from_file_location("native_pdftex_parity", "scripts/native_pdftex_parity.py")
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)

drifts = [
    module.CaptionDrift("Figure", 1, 3, 1, -2, "baseline", "native"),
    module.CaptionDrift("Table", 2, 4, 7, 3, "baseline", "native"),
    module.CaptionDrift("Figure", 3, 8, 8, 0, "baseline", "native"),
]
summary = module.caption_drift_summary(drifts)
assert summary.count == 3, summary
assert summary.sum_abs_pages == 5, summary
assert abs(summary.mean_abs_pages - (5 / 3)) < 1e-9, summary
assert summary.max_abs_pages == 3, summary

args = SimpleNamespace(
    require_page_count_match=False,
    max_mean_rmse=None,
    max_page_rmse=None,
    max_different_pixel_ratio=None,
    max_caption_drift_sum=4,
    max_caption_drift_mean=None,
    max_caption_drift_page=3,
    max_two_column_graphic_float_fallbacks=None,
    max_two_column_wide_graphic_float_fallbacks=None,
    max_two_column_graphic_float_fallback_native_slots=None,
    max_two_column_wide_graphic_float_fallback_native_slots=None,
    fail_on_warn=False,
)
page = module.PageMetrics(1, "1x1", "1x1", True, True, 0.0, 0.0, 0.0, 0, None)
native = module.BuildResult("native.pdf", 0.0, True, pages=1)
status, failures, warnings = module.assess_status([page], 1, 1, summary, native, args)
assert status == "FAIL", (status, failures, warnings)
assert any("caption drift sum 5 > 4" in failure for failure in failures), failures

coverage_args = SimpleNamespace(
    require_page_count_match=False,
    max_mean_rmse=None,
    max_page_rmse=None,
    max_different_pixel_ratio=None,
    max_caption_drift_sum=None,
    max_caption_drift_mean=None,
    max_caption_drift_page=None,
    max_two_column_graphic_float_fallbacks=0,
    max_two_column_wide_graphic_float_fallbacks=0,
    max_two_column_graphic_float_fallback_native_slots=20,
    max_two_column_wide_graphic_float_fallback_native_slots=10,
    fail_on_warn=False,
)
coverage_native = module.BuildResult(
    "native.pdf",
    0.0,
    True,
    pages=1,
    two_column_graphic_float_fallbacks=2,
    two_column_wide_graphic_float_fallbacks=1,
    two_column_graphic_float_fallback_estimated_native_slots=21,
    two_column_wide_graphic_float_fallback_estimated_native_slots=11,
)
empty_summary = module.CaptionDriftSummary(0, 0, 0.0, 0)
status, failures, warnings = module.assess_status(
    [page], 1, 1, empty_summary, coverage_native, coverage_args
)
assert status == "FAIL", (status, failures, warnings)
assert any("two-column graphic float fallbacks 2 > 0" in failure for failure in failures), failures
assert any("two-column wide graphic float fallbacks 1 > 0" in failure for failure in failures), failures
assert any("two-column graphic float fallback native slots 21 > 20" in failure for failure in failures), failures
assert any("two-column wide graphic float fallback native slots 11 > 10" in failure for failure in failures), failures
"#;
    let status = Command::new("python3")
        .arg("-c")
        .arg(code)
        .status()
        .expect("failed to run Python caption-drift summary check");

    assert!(status.success(), "caption-drift summary check failed");
}

#[test]
#[ignore = "runs pdflatex, pdftoppm, and the experimental native pdftex backend"]
fn native_pdftex_rendered_parity_smoke_reports_metrics() {
    let workdir = std::env::temp_dir().join(format!(
        "texpilot-native-pdftex-parity-smoke-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&workdir);

    let status = Command::new("scripts/native_pdftex_parity.py")
        .arg("--smoke")
        .arg("--profile")
        .arg("debug")
        .arg("--resolution")
        .arg("72")
        .arg("--workdir")
        .arg(&workdir)
        .status()
        .expect("failed to launch native pdftex parity script");

    let _ = fs::remove_dir_all(&workdir);
    assert!(status.success(), "native pdftex parity smoke failed");
}

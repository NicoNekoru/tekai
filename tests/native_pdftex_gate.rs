use std::process::Command;

#[test]
fn native_pdftex_gate_summarizes_coverage_failures() {
    let code = r#"
import importlib.util
import sys
from types import SimpleNamespace

spec = importlib.util.spec_from_file_location("native_pdftex_gate", "scripts/native_pdftex_gate.py")
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)

summary = {
    "two_column_graphic_float_fallbacks": 11,
    "two_column_wide_graphic_float_fallbacks": 6,
    "two_column_graphic_float_fallback_estimated_native_slots": 258,
    "two_column_wide_graphic_float_fallback_estimated_native_slots": 132,
}
args = SimpleNamespace(
    max_two_column_graphic_float_fallbacks=10,
    max_two_column_wide_graphic_float_fallbacks=5,
    max_two_column_graphic_float_fallback_native_slots=257,
    max_two_column_wide_graphic_float_fallback_native_slots=131,
)
failures = module.coverage_failures(summary, args)
assert "two-column graphic float fallbacks 11 > 10" in failures, failures
assert "two-column wide graphic float fallbacks 6 > 5" in failures, failures
assert "two-column graphic float fallback native slots 258 > 257" in failures, failures
assert "two-column wide graphic float fallback native slots 132 > 131" in failures, failures
"#;
    let status = Command::new("python3")
        .arg("-c")
        .arg(code)
        .status()
        .expect("failed to run Python native gate coverage check");

    assert!(status.success(), "native gate coverage check failed");
}

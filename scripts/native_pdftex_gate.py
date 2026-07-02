#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import shutil
import statistics
import subprocess
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path


DEFAULT_PAPERS = [
    "examples/arXiv-2605.26379v1/main.tex",
    "examples/arXiv-2511.08544v3/main.tex",
]


@dataclass(frozen=True)
class Measurement:
    wall_seconds: float
    report_elapsed_seconds: float
    pages: int
    two_column_graphic_float_fallbacks: int
    two_column_wide_graphic_float_fallbacks: int
    two_column_graphic_float_fallback_estimated_native_slots: int
    two_column_wide_graphic_float_fallback_estimated_native_slots: int


def env_int(name: str, default: int) -> int:
    value = os.environ.get(name)
    return default if value is None else int(value)


def env_int_or_none(name: str) -> int | None:
    value = os.environ.get(name)
    return None if value is None or value == "" else int(value)


def env_float(name: str, default: float) -> float:
    value = os.environ.get(name)
    return default if value is None else float(value)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Gate the experimental native texpilot-pdftex backend on the bundled "
            "large-paper clean-build target."
        )
    )
    parser.add_argument(
        "papers",
        nargs="*",
        default=os.environ.get("TEXPILOT_NATIVE_PDFTEX_PAPERS", "").split()
        or DEFAULT_PAPERS,
        help="TeX roots to build; defaults to the two bundled large examples.",
    )
    parser.add_argument(
        "--runs",
        type=int,
        default=env_int("TEXPILOT_NATIVE_PDFTEX_RUNS", 3),
        help="clean native builds per paper",
    )
    parser.add_argument(
        "--max-seconds",
        type=float,
        default=env_float("TEXPILOT_NATIVE_PDFTEX_MAX_SECONDS", 1.0),
        help="maximum allowed median wall-clock seconds per paper",
    )
    parser.add_argument(
        "--max-two-column-graphic-float-fallbacks",
        type=int,
        default=env_int_or_none(
            "TEXPILOT_NATIVE_PDFTEX_MAX_TWO_COLUMN_GRAPHIC_FLOAT_FALLBACKS"
        ),
        help=(
            "optional maximum allowed two-column graphic float bodies still "
            "using the approximate native path"
        ),
    )
    parser.add_argument(
        "--max-two-column-wide-graphic-float-fallbacks",
        type=int,
        default=env_int_or_none(
            "TEXPILOT_NATIVE_PDFTEX_MAX_TWO_COLUMN_WIDE_GRAPHIC_FLOAT_FALLBACKS"
        ),
        help=(
            "optional maximum allowed starred/wide two-column graphic float "
            "bodies still using the approximate native path"
        ),
    )
    parser.add_argument(
        "--max-two-column-graphic-float-fallback-native-slots",
        type=int,
        default=env_int_or_none(
            "TEXPILOT_NATIVE_PDFTEX_MAX_TWO_COLUMN_GRAPHIC_FLOAT_FALLBACK_NATIVE_SLOTS"
        ),
        help=(
            "optional maximum allowed estimated native slots still behind "
            "two-column graphic float fallbacks"
        ),
    )
    parser.add_argument(
        "--max-two-column-wide-graphic-float-fallback-native-slots",
        type=int,
        default=env_int_or_none(
            "TEXPILOT_NATIVE_PDFTEX_MAX_TWO_COLUMN_WIDE_GRAPHIC_FLOAT_FALLBACK_NATIVE_SLOTS"
        ),
        help=(
            "optional maximum allowed estimated native slots still behind "
            "starred/wide two-column graphic float fallbacks"
        ),
    )
    parser.add_argument(
        "--profile",
        choices=("release", "debug"),
        default=os.environ.get("TEXPILOT_NATIVE_PDFTEX_PROFILE", "release"),
        help="Cargo profile used when --texpilot is not supplied",
    )
    parser.add_argument(
        "--texpilot",
        type=Path,
        default=Path(os.environ["TEXPILOT_BIN"])
        if os.environ.get("TEXPILOT_BIN")
        else None,
        help="prebuilt texpilot binary to gate",
    )
    parser.add_argument(
        "--workdir",
        type=Path,
        default=Path(os.environ["TEXPILOT_NATIVE_PDFTEX_WORKDIR"])
        if os.environ.get("TEXPILOT_NATIVE_PDFTEX_WORKDIR")
        else None,
        help="directory for clean build outputs; defaults to a fresh /tmp directory",
    )
    parser.add_argument(
        "--keep-workdir",
        action="store_true",
        help="preserve generated build outputs after the gate finishes",
    )
    parser.add_argument("--json", action="store_true", help="print machine-readable results")
    args = parser.parse_args()
    if (
        args.max_two_column_graphic_float_fallbacks is not None
        and args.max_two_column_graphic_float_fallbacks < 0
    ):
        raise ValueError("--max-two-column-graphic-float-fallbacks must be non-negative")
    if (
        args.max_two_column_wide_graphic_float_fallbacks is not None
        and args.max_two_column_wide_graphic_float_fallbacks < 0
    ):
        raise ValueError("--max-two-column-wide-graphic-float-fallbacks must be non-negative")
    if (
        args.max_two_column_graphic_float_fallback_native_slots is not None
        and args.max_two_column_graphic_float_fallback_native_slots < 0
    ):
        raise ValueError(
            "--max-two-column-graphic-float-fallback-native-slots must be non-negative"
        )
    if (
        args.max_two_column_wide_graphic_float_fallback_native_slots is not None
        and args.max_two_column_wide_graphic_float_fallback_native_slots < 0
    ):
        raise ValueError(
            "--max-two-column-wide-graphic-float-fallback-native-slots must be non-negative"
        )
    return args


def build_texpilot(root: Path, profile: str) -> Path:
    command = ["cargo", "build", "--locked", "--quiet"]
    if profile == "release":
        command.append("--release")
    subprocess.run(command, cwd=root, check=True)
    return root / "target" / profile / "texpilot"


def trace_optional_int(trace: str, key: str) -> int | None:
    prefix = f"{key}\t"
    for line in trace.splitlines():
        if line.startswith(prefix):
            value = line.split("\t", 1)[1].strip()
            try:
                return int(value)
            except ValueError as exc:
                raise RuntimeError(f"trace key {key} is not an integer: {value}") from exc
    return None


def trace_int(trace: str, key: str, default: int = 0) -> int:
    value = trace_optional_int(trace, key)
    return default if value is None else value


def check_trace(trace_path: Path) -> tuple[int, int, int, int, int]:
    trace = trace_path.read_text(encoding="utf-8")
    if "engine\ttexpilot-pdftex-native" not in trace:
        raise RuntimeError(f"{trace_path} did not record the native backend")
    if "unsupported\t" in trace:
        raise RuntimeError(f"{trace_path} recorded native fallback:\n{trace}")
    if "layout_caption_entries\t" not in trace:
        raise RuntimeError(f"{trace_path} did not record native caption-placement diagnostics")
    pages = trace_optional_int(trace, "pages")
    if pages is None:
        raise RuntimeError(f"{trace_path} did not record a page count")
    return (
        pages,
        trace_int(trace, "layout_two_column_graphic_float_fallbacks"),
        trace_int(trace, "layout_two_column_wide_graphic_float_fallbacks"),
        trace_int(trace, "layout_two_column_graphic_float_fallback_estimated_native_slots"),
        trace_int(trace, "layout_two_column_wide_graphic_float_fallback_estimated_native_slots"),
    )


def run_native_build(root: Path, texpilot: Path, paper: Path, out_dir: Path) -> Measurement:
    shutil.rmtree(out_dir, ignore_errors=True)
    out_dir.mkdir(parents=True, exist_ok=True)
    command = [
        str(texpilot),
        "build",
        str(paper),
        "--engine",
        "texpilot-pdftex",
        "--out-dir",
        str(out_dir),
        "--force",
        "--quiet",
        "--report-json",
    ]
    started = time.perf_counter()
    completed = subprocess.run(
        command,
        cwd=root,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    )
    wall_seconds = time.perf_counter() - started
    report = json.loads(completed.stdout)
    pdf_path = Path(report["pdf_path"])
    if not pdf_path.exists() or pdf_path.stat().st_size == 0:
        raise RuntimeError(f"native build did not write a non-empty PDF: {pdf_path}")
    if report.get("tex_runs") != 1 or report.get("pdf_tex_runs") != 1:
        raise RuntimeError(f"expected one native PDF run, got report: {report}")
    for key in ("bibliography_runs", "index_runs", "external_runs"):
        if report.get(key) != 0:
            raise RuntimeError(f"expected no {key}, got report: {report}")
    trace_path = out_dir / f"{paper.stem}.texpilot-pdftex.trace"
    (
        pages,
        two_column_fallbacks,
        two_column_wide_fallbacks,
        two_column_fallback_slots,
        two_column_wide_fallback_slots,
    ) = check_trace(trace_path)
    return Measurement(
        wall_seconds=wall_seconds,
        report_elapsed_seconds=report["elapsed_ms"] / 1000.0,
        pages=pages,
        two_column_graphic_float_fallbacks=two_column_fallbacks,
        two_column_wide_graphic_float_fallbacks=two_column_wide_fallbacks,
        two_column_graphic_float_fallback_estimated_native_slots=two_column_fallback_slots,
        two_column_wide_graphic_float_fallback_estimated_native_slots=(
            two_column_wide_fallback_slots
        ),
    )


def summarize(measurements: list[Measurement]) -> dict[str, float | int]:
    walls = [measurement.wall_seconds for measurement in measurements]
    reports = [measurement.report_elapsed_seconds for measurement in measurements]
    return {
        "min": min(walls),
        "median": statistics.median(walls),
        "mean": statistics.fmean(walls),
        "max": max(walls),
        "report_median": statistics.median(reports),
        "pages": measurements[0].pages,
        "two_column_graphic_float_fallbacks": measurements[0].two_column_graphic_float_fallbacks,
        "two_column_wide_graphic_float_fallbacks": (
            measurements[0].two_column_wide_graphic_float_fallbacks
        ),
        "two_column_graphic_float_fallback_estimated_native_slots": (
            measurements[0].two_column_graphic_float_fallback_estimated_native_slots
        ),
        "two_column_wide_graphic_float_fallback_estimated_native_slots": (
            measurements[0].two_column_wide_graphic_float_fallback_estimated_native_slots
        ),
    }


def coverage_failures(summary: dict[str, float | int], args: argparse.Namespace) -> list[str]:
    failures: list[str] = []
    total = int(summary["two_column_graphic_float_fallbacks"])
    wide = int(summary["two_column_wide_graphic_float_fallbacks"])
    total_slots = int(summary["two_column_graphic_float_fallback_estimated_native_slots"])
    wide_slots = int(summary["two_column_wide_graphic_float_fallback_estimated_native_slots"])
    if (
        args.max_two_column_graphic_float_fallbacks is not None
        and total > args.max_two_column_graphic_float_fallbacks
    ):
        failures.append(
            "two-column graphic float fallbacks "
            f"{total} > {args.max_two_column_graphic_float_fallbacks}"
        )
    if (
        args.max_two_column_wide_graphic_float_fallbacks is not None
        and wide > args.max_two_column_wide_graphic_float_fallbacks
    ):
        failures.append(
            "two-column wide graphic float fallbacks "
            f"{wide} > {args.max_two_column_wide_graphic_float_fallbacks}"
        )
    if (
        args.max_two_column_graphic_float_fallback_native_slots is not None
        and total_slots > args.max_two_column_graphic_float_fallback_native_slots
    ):
        failures.append(
            "two-column graphic float fallback native slots "
            f"{total_slots} > {args.max_two_column_graphic_float_fallback_native_slots}"
        )
    if (
        args.max_two_column_wide_graphic_float_fallback_native_slots is not None
        and wide_slots > args.max_two_column_wide_graphic_float_fallback_native_slots
    ):
        failures.append(
            "two-column wide graphic float fallback native slots "
            f"{wide_slots} > {args.max_two_column_wide_graphic_float_fallback_native_slots}"
        )
    return failures


def main() -> int:
    args = parse_args()
    if args.runs <= 0:
        raise ValueError("--runs must be positive")
    if args.max_seconds <= 0:
        raise ValueError("--max-seconds must be positive")

    root = Path(__file__).resolve().parents[1]
    texpilot = args.texpilot.resolve() if args.texpilot else build_texpilot(root, args.profile)
    if not texpilot.exists():
        raise FileNotFoundError(f"texpilot binary not found: {texpilot}")

    workdir = (
        args.workdir.resolve()
        if args.workdir
        else Path(tempfile.mkdtemp(prefix="texpilot-native-pdftex-gate-")).resolve()
    )
    workdir.mkdir(parents=True, exist_ok=True)

    results: dict[str, dict[str, float | int | bool]] = {}
    print(
        f"native pdftex gate: median wall time <= {args.max_seconds:.3f}s "
        f"({args.runs} clean build{'s' if args.runs != 1 else ''} per paper)"
    )
    print(f"texpilot: {texpilot}")
    print()
    print(
        f"{'status':<6} {'paper':<42} {'median':>8} {'min':>8} "
        f"{'mean':>8} {'max':>8} {'pages':>6} {'2col':>6} {'wide':>6} "
        f"{'slots':>7} {'wslots':>7}"
    )

    for paper_arg in args.papers:
        paper = (root / paper_arg).resolve()
        if not paper.exists():
            raise FileNotFoundError(f"paper not found: {paper}")
        paper_key = str(paper.relative_to(root))
        measurements = [
            run_native_build(
                root,
                texpilot,
                paper,
                workdir / f"{paper.parent.name}-{run_index:02d}",
            )
            for run_index in range(1, args.runs + 1)
        ]
        summary = summarize(measurements)
        failures = coverage_failures(summary, args)
        if summary["median"] > args.max_seconds:
            failures.append(f"median {summary['median']:.3f}s > {args.max_seconds:.3f}s")
        passed = not failures
        results[paper_key] = {**summary, "passed": passed}
        status = "PASS" if passed else "FAIL"
        print(
            f"{status:<6} {paper_key:<42} {summary['median']:8.3f} "
            f"{summary['min']:8.3f} {summary['mean']:8.3f} "
            f"{summary['max']:8.3f} {summary['pages']:6d} "
            f"{summary['two_column_graphic_float_fallbacks']:6d} "
            f"{summary['two_column_wide_graphic_float_fallbacks']:6d} "
            f"{summary['two_column_graphic_float_fallback_estimated_native_slots']:7d} "
            f"{summary['two_column_wide_graphic_float_fallback_estimated_native_slots']:7d}"
        )
        for failure in failures:
            print(f"  failure: {failure}")

    if args.json:
        print()
        print(json.dumps({"texpilot": str(texpilot), "results": results}, indent=2))

    if not args.keep_workdir and not args.workdir:
        shutil.rmtree(workdir, ignore_errors=True)
    return 1 if any(not result["passed"] for result in results.values()) else 0


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
import re
import shutil
import statistics
import subprocess
import tempfile
import time
from dataclasses import asdict, dataclass
from pathlib import Path


DEFAULT_PAPERS = [
    "examples/arXiv-2605.26379v1/main.tex",
    "examples/arXiv-2511.08544v3/main.tex",
]

SMOKE_SOURCE = r"""\documentclass{article}
\title{Native PDF parity smoke}
\author{Texpilot}
\date{2026}
\begin{document}
\maketitle
\section{A small section}
This is a focused rendered-output comparison with an inline equation $a+b=c$.
\end{document}
"""


@dataclass(frozen=True)
class BuildResult:
    pdf: str
    elapsed_seconds: float
    native: bool
    pages: int | None = None
    trace_path: str | None = None
    two_column_graphic_float_fallbacks: int = 0
    two_column_wide_graphic_float_fallbacks: int = 0
    two_column_graphic_float_fallback_estimated_native_slots: int = 0
    two_column_wide_graphic_float_fallback_estimated_native_slots: int = 0


@dataclass(frozen=True)
class PageMetrics:
    page: int
    expected_size: str
    actual_size: str
    exact_sha_match: bool
    dimensions_match: bool
    mean_abs_channel_delta: float
    rmse: float
    different_pixel_ratio: float
    max_abs_channel_delta: int
    missing: str | None = None


@dataclass(frozen=True)
class CaptionOccurrence:
    kind: str
    number: int
    page: int
    text: str


@dataclass(frozen=True)
class SourceCaptionHint:
    kind: str
    number: int
    tokens: tuple[str, ...]


@dataclass(frozen=True)
class CaptionDrift:
    kind: str
    number: int
    baseline_page: int
    native_page: int
    delta_pages: int
    baseline_text: str
    native_text: str


@dataclass(frozen=True)
class CaptionDriftSummary:
    count: int
    sum_abs_pages: int
    mean_abs_pages: float
    max_abs_pages: int


@dataclass(frozen=True)
class CaseResult:
    paper: str
    status: str
    baseline: BuildResult
    native: BuildResult
    page_count_match: bool
    exact_render_match: bool
    mean_rmse: float
    max_page_rmse: float
    max_different_pixel_ratio: float
    dimension_mismatches: int
    failures: list[str]
    warnings: list[str]
    pages: list[PageMetrics]
    caption_drifts: list[CaptionDrift]
    caption_drift_summary: CaptionDriftSummary
    comparison_dir: str | None = None


@dataclass(frozen=True)
class RenderedPage:
    path: Path
    width: int
    height: int
    pixels: bytes
    sha256: str


def env_int(name: str, default: int) -> int:
    value = os.environ.get(name)
    return default if value is None else int(value)


def env_int_or_none(name: str) -> int | None:
    value = os.environ.get(name)
    return None if value is None or value == "" else int(value)


def env_float_or_none(name: str) -> float | None:
    value = os.environ.get(name)
    return None if value is None or value == "" else float(value)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Compare experimental native texpilot-pdftex output against a "
            "converged external pdfTeX baseline using rendered-page metrics."
        )
    )
    parser.add_argument(
        "papers",
        nargs="*",
        help=(
            "TeX roots to compare; defaults to the two bundled large examples "
            "unless --smoke is used."
        ),
    )
    parser.add_argument(
        "--smoke",
        action="store_true",
        help="compare a generated one-page smoke fixture instead of the large examples",
    )
    parser.add_argument(
        "--resolution",
        type=int,
        default=env_int("TEXPILOT_NATIVE_PDFTEX_PARITY_DPI", 96),
        help="pdftoppm rasterization DPI",
    )
    parser.add_argument(
        "--pixel-tolerance",
        type=int,
        default=env_int("TEXPILOT_NATIVE_PDFTEX_PIXEL_TOLERANCE", 2),
        help="per-channel tolerance for counting a pixel as different",
    )
    parser.add_argument(
        "--max-mean-rmse",
        type=float,
        default=env_float_or_none("TEXPILOT_NATIVE_PDFTEX_MAX_MEAN_RMSE"),
        help="optional failing threshold for mean page RMSE",
    )
    parser.add_argument(
        "--max-page-rmse",
        type=float,
        default=env_float_or_none("TEXPILOT_NATIVE_PDFTEX_MAX_PAGE_RMSE"),
        help="optional failing threshold for the worst page RMSE",
    )
    parser.add_argument(
        "--max-different-pixel-ratio",
        type=float,
        default=env_float_or_none("TEXPILOT_NATIVE_PDFTEX_MAX_DIFF_RATIO"),
        help="optional failing threshold for the worst page different-pixel ratio",
    )
    parser.add_argument(
        "--max-caption-drift-sum",
        type=int,
        default=env_int_or_none("TEXPILOT_NATIVE_PDFTEX_MAX_CAPTION_DRIFT_SUM"),
        help="optional failing threshold for total absolute caption page drift",
    )
    parser.add_argument(
        "--max-caption-drift-mean",
        type=float,
        default=env_float_or_none("TEXPILOT_NATIVE_PDFTEX_MAX_CAPTION_DRIFT_MEAN"),
        help="optional failing threshold for mean absolute caption page drift",
    )
    parser.add_argument(
        "--max-caption-drift-page",
        type=int,
        default=env_int_or_none("TEXPILOT_NATIVE_PDFTEX_MAX_CAPTION_DRIFT_PAGE"),
        help="optional failing threshold for the largest single caption page drift",
    )
    parser.add_argument(
        "--max-two-column-graphic-float-fallbacks",
        type=int,
        default=env_int_or_none(
            "TEXPILOT_NATIVE_PDFTEX_MAX_TWO_COLUMN_GRAPHIC_FLOAT_FALLBACKS"
        ),
        help=(
            "optional failing threshold for two-column graphic float bodies "
            "that still use the approximate native path"
        ),
    )
    parser.add_argument(
        "--max-two-column-wide-graphic-float-fallbacks",
        type=int,
        default=env_int_or_none(
            "TEXPILOT_NATIVE_PDFTEX_MAX_TWO_COLUMN_WIDE_GRAPHIC_FLOAT_FALLBACKS"
        ),
        help=(
            "optional failing threshold for starred/wide two-column graphic "
            "float bodies that still use the approximate native path"
        ),
    )
    parser.add_argument(
        "--max-two-column-graphic-float-fallback-native-slots",
        type=int,
        default=env_int_or_none(
            "TEXPILOT_NATIVE_PDFTEX_MAX_TWO_COLUMN_GRAPHIC_FLOAT_FALLBACK_NATIVE_SLOTS"
        ),
        help=(
            "optional failing threshold for estimated native slots still behind "
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
            "optional failing threshold for estimated native slots still behind "
            "starred/wide two-column graphic float fallbacks"
        ),
    )
    parser.add_argument(
        "--require-page-count-match",
        action="store_true",
        help="fail when native and external PDFs have different page counts",
    )
    parser.add_argument(
        "--fail-on-warn",
        action="store_true",
        help="turn warning-class regressions, such as page-count drift, into failures",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="enable near-identical default thresholds suitable for a future gate",
    )
    parser.add_argument(
        "--allow-fallback",
        action="store_true",
        help="allow texpilot-pdftex fallback to external pdflatex in the native run",
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
        help="prebuilt texpilot binary to compare",
    )
    parser.add_argument(
        "--pdftoppm",
        default=os.environ.get("PDFTOPPM", "pdftoppm"),
        help="pdftoppm executable",
    )
    parser.add_argument(
        "--pdftotext",
        default=os.environ.get("PDFTOTEXT", "pdftotext"),
        help="pdftotext executable for optional caption-flow diagnostics",
    )
    parser.add_argument(
        "--workdir",
        type=Path,
        default=Path(os.environ["TEXPILOT_NATIVE_PDFTEX_PARITY_WORKDIR"])
        if os.environ.get("TEXPILOT_NATIVE_PDFTEX_PARITY_WORKDIR")
        else None,
        help="directory for copied sources and rendered pages",
    )
    parser.add_argument(
        "--keep-workdir",
        action="store_true",
        help="preserve generated sources, PDFs, page renders, and metrics",
    )
    parser.add_argument("--json", action="store_true", help="print machine-readable results")
    parser.add_argument(
        "--json-pages",
        action="store_true",
        help="include per-page metrics in --json output",
    )
    parser.add_argument(
        "--top-pages",
        type=int,
        default=env_int("TEXPILOT_NATIVE_PDFTEX_TOP_PAGES", 0),
        help="print the N worst rendered pages by RMSE",
    )
    parser.add_argument(
        "--write-comparison-pages",
        action="store_true",
        help="write side-by-side PPM comparisons for the worst rendered pages",
    )
    parser.add_argument(
        "--caption-drift",
        type=int,
        default=env_int("TEXPILOT_NATIVE_PDFTEX_CAPTION_DRIFT", 0),
        help=(
            "print the N figure/table/listing captions with the largest native-vs-baseline "
            "page drift; requires pdftotext"
        ),
    )
    args = parser.parse_args()

    if args.strict:
        args.require_page_count_match = True
        if args.max_mean_rmse is None:
            args.max_mean_rmse = 1.5
        if args.max_page_rmse is None:
            args.max_page_rmse = 3.0
        if args.max_different_pixel_ratio is None:
            args.max_different_pixel_ratio = 0.01

    if args.resolution <= 0:
        raise ValueError("--resolution must be positive")
    if not 0 <= args.pixel_tolerance <= 255:
        raise ValueError("--pixel-tolerance must be between 0 and 255")
    if args.top_pages < 0:
        raise ValueError("--top-pages must be non-negative")
    if args.caption_drift < 0:
        raise ValueError("--caption-drift must be non-negative")
    if args.max_caption_drift_sum is not None and args.max_caption_drift_sum < 0:
        raise ValueError("--max-caption-drift-sum must be non-negative")
    if args.max_caption_drift_mean is not None and args.max_caption_drift_mean < 0:
        raise ValueError("--max-caption-drift-mean must be non-negative")
    if args.max_caption_drift_page is not None and args.max_caption_drift_page < 0:
        raise ValueError("--max-caption-drift-page must be non-negative")
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


def command_available(command: str) -> bool:
    return shutil.which(command) is not None


def require_command(command: str) -> None:
    if not command_available(command):
        raise FileNotFoundError(f"missing required command: {command}")


def run(command: list[str], cwd: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        command,
        cwd=cwd,
        text=True,
        encoding="utf-8",
        errors="replace",
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=True,
    )


def build_texpilot(root: Path, profile: str) -> Path:
    command = ["cargo", "build", "--locked", "--quiet"]
    if profile == "release":
        command.append("--release")
    subprocess.run(command, cwd=root, check=True)
    return root / "target" / profile / "texpilot"


def job_name_for(main: Path) -> str:
    return main.with_suffix("").name


def line_requests_rerun(line: str) -> bool:
    lower = line.lower()
    return (
        "rerun to get" in lower
        or "rerun latex" in lower
        or "rerun lualatex" in lower
        or "rerun xelatex" in lower
        or "label(s) may have changed" in lower
        or "reference(s) may have changed" in lower
        or "citation(s) may have changed" in lower
        or ("file `" in lower and "' has changed" in lower)
        or ('file "' in lower and '" has changed' in lower)
    )


def needs_rerun(log_path: Path) -> bool:
    if not log_path.exists():
        return False
    with log_path.open(encoding="utf-8", errors="replace") as handle:
        return any(line_requests_rerun(line) for line in handle)


def run_baseline_bibliography_tools(cwd: Path, job: str) -> None:
    if (cwd / f"{job}.bcf").exists():
        require_command("biber")
        run(["biber", job], cwd)

    for aux in sorted(cwd.rglob("*.aux")):
        if "\\bibdata" not in aux.read_text(encoding="utf-8", errors="replace"):
            continue
        relative_job = aux.relative_to(cwd).with_suffix("")
        run(["bibtex", str(relative_job)], cwd)


def run_pdflatex_baseline(source_dir: Path, main_name: str) -> BuildResult:
    require_command("pdflatex")
    require_command("bibtex")
    job = job_name_for(Path(main_name))
    started = time.perf_counter()

    run(["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "-file-line-error", main_name], source_dir)
    run_baseline_bibliography_tools(source_dir, job)
    for _ in range(8):
        run(
            ["pdflatex", "-interaction=nonstopmode", "-halt-on-error", "-file-line-error", main_name],
            source_dir,
        )
        if not needs_rerun(source_dir / f"{job}.log"):
            pdf = source_dir / f"{job}.pdf"
            if not pdf.exists() or pdf.stat().st_size == 0:
                raise RuntimeError(f"pdflatex baseline did not produce a non-empty PDF: {pdf}")
            return BuildResult(str(pdf), time.perf_counter() - started, native=False)

    raise RuntimeError(f"pdflatex baseline did not converge for {source_dir / main_name}")


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


def check_native_trace(
    trace_path: Path, allow_fallback: bool
) -> tuple[int | None, int, int, int, int]:
    trace = trace_path.read_text(encoding="utf-8")
    if "engine\ttexpilot-pdftex-native" not in trace:
        raise RuntimeError(f"{trace_path} did not record the native backend")
    if not allow_fallback and "unsupported\t" in trace:
        raise RuntimeError(f"{trace_path} recorded native fallback:\n{trace}")
    if "unsupported\t" not in trace and "layout_caption_entries\t" not in trace:
        raise RuntimeError(f"{trace_path} did not record native caption-placement diagnostics")
    return (
        trace_optional_int(trace, "pages"),
        trace_int(trace, "layout_two_column_graphic_float_fallbacks"),
        trace_int(trace, "layout_two_column_wide_graphic_float_fallbacks"),
        trace_int(trace, "layout_two_column_graphic_float_fallback_estimated_native_slots"),
        trace_int(trace, "layout_two_column_wide_graphic_float_fallback_estimated_native_slots"),
    )


def run_native_build(
    root: Path,
    texpilot: Path,
    source_dir: Path,
    main_name: str,
    out_dir: Path,
    allow_fallback: bool,
) -> BuildResult:
    shutil.rmtree(out_dir, ignore_errors=True)
    out_dir.mkdir(parents=True, exist_ok=True)
    command = [
        str(texpilot),
        "build",
        str(source_dir / main_name),
        "--engine",
        "texpilot-pdftex",
        "--out-dir",
        str(out_dir),
        "--force",
        "--quiet",
        "--report-json",
    ]
    started = time.perf_counter()
    completed = run(command, root)
    elapsed = time.perf_counter() - started
    report = json.loads(completed.stdout)
    pdf = Path(report["pdf_path"])
    if not pdf.exists() or pdf.stat().st_size == 0:
        raise RuntimeError(f"native build did not write a non-empty PDF: {pdf}")
    trace_path = out_dir / f"{Path(main_name).stem}.texpilot-pdftex.trace"
    (
        pages,
        two_column_fallbacks,
        two_column_wide_fallbacks,
        two_column_fallback_slots,
        two_column_wide_fallback_slots,
    ) = check_native_trace(trace_path, allow_fallback)
    return BuildResult(
        str(pdf),
        elapsed,
        native=True,
        pages=pages,
        trace_path=str(trace_path),
        two_column_graphic_float_fallbacks=two_column_fallbacks,
        two_column_wide_graphic_float_fallbacks=two_column_wide_fallbacks,
        two_column_graphic_float_fallback_estimated_native_slots=two_column_fallback_slots,
        two_column_wide_graphic_float_fallback_estimated_native_slots=(
            two_column_wide_fallback_slots
        ),
    )


def copy_source_tree(source_dir: Path, dest: Path) -> None:
    if dest.exists():
        shutil.rmtree(dest)
    shutil.copytree(source_dir, dest, symlinks=True)


def materialize_smoke_source(workdir: Path) -> Path:
    source_dir = workdir / "smoke-source"
    source_dir.mkdir(parents=True, exist_ok=True)
    (source_dir / "main.tex").write_text(SMOKE_SOURCE, encoding="utf-8")
    return source_dir / "main.tex"


def selected_papers(root: Path, workdir: Path, args: argparse.Namespace) -> list[Path]:
    if args.smoke:
        return [materialize_smoke_source(workdir)]
    if args.papers:
        return [(root / paper).resolve() for paper in args.papers]
    env_papers = os.environ.get("TEXPILOT_NATIVE_PDFTEX_PARITY_PAPERS", "").split()
    return [(root / paper).resolve() for paper in (env_papers or DEFAULT_PAPERS)]


def read_token(data: bytes, offset: int) -> tuple[bytes, int]:
    length = len(data)
    while offset < length:
        byte = data[offset]
        if byte == ord("#"):
            while offset < length and data[offset] not in b"\r\n":
                offset += 1
        elif byte in b" \t\r\n":
            offset += 1
        else:
            break
    start = offset
    while offset < length and data[offset] not in b" \t\r\n":
        offset += 1
    if start == offset:
        raise ValueError("unexpected end of PPM header")
    return data[start:offset], offset


def read_ppm(path: Path) -> RenderedPage:
    data = path.read_bytes()
    magic, offset = read_token(data, 0)
    if magic != b"P6":
        raise ValueError(f"{path} is not a binary PPM file")
    width_token, offset = read_token(data, offset)
    height_token, offset = read_token(data, offset)
    maxval_token, offset = read_token(data, offset)
    width = int(width_token)
    height = int(height_token)
    maxval = int(maxval_token)
    if maxval != 255:
        raise ValueError(f"{path} uses unsupported PPM maxval {maxval}")
    if offset >= len(data) or data[offset] not in b" \t\r\n":
        raise ValueError(f"{path} has a malformed PPM raster separator")
    offset += 1
    expected_len = width * height * 3
    pixels = data[offset : offset + expected_len]
    if len(pixels) != expected_len:
        raise ValueError(f"{path} has truncated PPM raster data")
    return RenderedPage(path, width, height, pixels, hashlib.sha256(pixels).hexdigest())


def render_pdf(pdftoppm: str, resolution: int, pdf: Path, out_dir: Path) -> list[RenderedPage]:
    shutil.rmtree(out_dir, ignore_errors=True)
    out_dir.mkdir(parents=True, exist_ok=True)
    run([pdftoppm, "-r", str(resolution), str(pdf), str(out_dir / "page")], out_dir)
    pages = sorted(out_dir.glob("page-*.ppm"))
    if not pages:
        raise RuntimeError(f"pdftoppm did not render pages for {pdf}")
    return [read_ppm(page) for page in pages]


def page_pixel(page: RenderedPage, x: int, y: int, channel: int) -> int:
    if x >= page.width or y >= page.height:
        return 255
    return page.pixels[((y * page.width + x) * 3) + channel]


def compare_same_size(expected: RenderedPage, actual: RenderedPage, pixel_tolerance: int) -> PageMetrics:
    sum_abs = 0
    sum_sq = 0
    max_abs = 0
    diff_pixels = 0
    pixels = expected.width * expected.height
    expected_data = expected.pixels
    actual_data = actual.pixels

    for offset in range(0, len(expected_data), 3):
        dr = abs(expected_data[offset] - actual_data[offset])
        dg = abs(expected_data[offset + 1] - actual_data[offset + 1])
        db = abs(expected_data[offset + 2] - actual_data[offset + 2])
        if dr > pixel_tolerance or dg > pixel_tolerance or db > pixel_tolerance:
            diff_pixels += 1
        for delta in (dr, dg, db):
            sum_abs += delta
            sum_sq += delta * delta
            max_abs = max(max_abs, delta)

    channels = pixels * 3
    return PageMetrics(
        page=0,
        expected_size=f"{expected.width}x{expected.height}",
        actual_size=f"{actual.width}x{actual.height}",
        exact_sha_match=expected.sha256 == actual.sha256,
        dimensions_match=True,
        mean_abs_channel_delta=sum_abs / channels,
        rmse=math.sqrt(sum_sq / channels),
        different_pixel_ratio=diff_pixels / pixels,
        max_abs_channel_delta=max_abs,
    )


def compare_with_padding(expected: RenderedPage, actual: RenderedPage, pixel_tolerance: int) -> PageMetrics:
    width = max(expected.width, actual.width)
    height = max(expected.height, actual.height)
    sum_abs = 0
    sum_sq = 0
    max_abs = 0
    diff_pixels = 0
    pixels = width * height

    for y in range(height):
        for x in range(width):
            changed = False
            for channel in range(3):
                delta = abs(page_pixel(expected, x, y, channel) - page_pixel(actual, x, y, channel))
                if delta > pixel_tolerance:
                    changed = True
                sum_abs += delta
                sum_sq += delta * delta
                max_abs = max(max_abs, delta)
            if changed:
                diff_pixels += 1

    channels = pixels * 3
    return PageMetrics(
        page=0,
        expected_size=f"{expected.width}x{expected.height}",
        actual_size=f"{actual.width}x{actual.height}",
        exact_sha_match=False,
        dimensions_match=False,
        mean_abs_channel_delta=sum_abs / channels,
        rmse=math.sqrt(sum_sq / channels),
        different_pixel_ratio=diff_pixels / pixels,
        max_abs_channel_delta=max_abs,
    )


def missing_page_metrics(page_index: int, missing: str, peer: RenderedPage) -> PageMetrics:
    peer_size = f"{peer.width}x{peer.height}"
    return PageMetrics(
        page=page_index,
        expected_size="missing" if missing == "expected" else peer_size,
        actual_size="missing" if missing == "actual" else peer_size,
        exact_sha_match=False,
        dimensions_match=False,
        mean_abs_channel_delta=255.0,
        rmse=255.0,
        different_pixel_ratio=1.0,
        max_abs_channel_delta=255,
        missing=missing,
    )


def compare_pages(
    expected_pages: list[RenderedPage],
    actual_pages: list[RenderedPage],
    pixel_tolerance: int,
) -> list[PageMetrics]:
    metrics: list[PageMetrics] = []
    for page_index in range(1, max(len(expected_pages), len(actual_pages)) + 1):
        expected = expected_pages[page_index - 1] if page_index <= len(expected_pages) else None
        actual = actual_pages[page_index - 1] if page_index <= len(actual_pages) else None
        if expected is None and actual is None:
            continue
        if expected is None:
            assert actual is not None
            metrics.append(missing_page_metrics(page_index, "expected", actual))
            continue
        if actual is None:
            metrics.append(missing_page_metrics(page_index, "actual", expected))
            continue
        if expected.width == actual.width and expected.height == actual.height:
            page_metrics = compare_same_size(expected, actual, pixel_tolerance)
        else:
            page_metrics = compare_with_padding(expected, actual, pixel_tolerance)
        metrics.append(
            PageMetrics(
                page=page_index,
                expected_size=page_metrics.expected_size,
                actual_size=page_metrics.actual_size,
                exact_sha_match=page_metrics.exact_sha_match,
                dimensions_match=page_metrics.dimensions_match,
                mean_abs_channel_delta=page_metrics.mean_abs_channel_delta,
                rmse=page_metrics.rmse,
                different_pixel_ratio=page_metrics.different_pixel_ratio,
                max_abs_channel_delta=page_metrics.max_abs_channel_delta,
                missing=page_metrics.missing,
            )
        )
    return metrics


def worst_pages(pages: list[PageMetrics], count: int) -> list[PageMetrics]:
    if count <= 0:
        return []
    return sorted(
        pages,
        key=lambda page: (page.rmse, page.different_pixel_ratio, page.mean_abs_channel_delta),
        reverse=True,
    )[:count]


def write_ppm(path: Path, width: int, height: int, pixels: bytes) -> None:
    path.write_bytes(f"P6\n{width} {height}\n255\n".encode("ascii") + pixels)


def white_pixels(width: int, height: int) -> bytes:
    return b"\xff" * width * height * 3


def padded_pixels(page: RenderedPage | None, width: int, height: int) -> bytes:
    if page is None:
        return white_pixels(width, height)
    if page.width == width and page.height == height:
        return page.pixels
    row_bytes = width * 3
    page_row_bytes = page.width * 3
    out = bytearray(white_pixels(width, height))
    for row in range(min(height, page.height)):
        source_start = row * page_row_bytes
        source_end = source_start + page_row_bytes
        out_start = row * row_bytes
        out[out_start : out_start + page_row_bytes] = page.pixels[source_start:source_end]
    return bytes(out)


def side_by_side_pixels(left: RenderedPage | None, right: RenderedPage | None) -> tuple[int, int, bytes]:
    if left is None and right is None:
        raise ValueError("cannot compare two missing pages")
    width = max(page.width for page in (left, right) if page is not None)
    height = max(page.height for page in (left, right) if page is not None)
    left_pixels = padded_pixels(left, width, height)
    right_pixels = padded_pixels(right, width, height)
    row_bytes = width * 3
    combined = bytearray(width * 2 * height * 3)
    for row in range(height):
        left_start = row * row_bytes
        left_end = left_start + row_bytes
        out_start = row * row_bytes * 2
        combined[out_start : out_start + row_bytes] = left_pixels[left_start:left_end]
        combined[out_start + row_bytes : out_start + row_bytes * 2] = right_pixels[left_start:left_end]
    return width * 2, height, bytes(combined)


def write_page_comparisons(
    out_dir: Path,
    expected_pages: list[RenderedPage],
    actual_pages: list[RenderedPage],
    pages: list[PageMetrics],
    count: int,
) -> Path | None:
    top = worst_pages(pages, count)
    if not top:
        return None
    shutil.rmtree(out_dir, ignore_errors=True)
    out_dir.mkdir(parents=True, exist_ok=True)
    for page in top:
        expected = expected_pages[page.page - 1] if page.page <= len(expected_pages) else None
        actual = actual_pages[page.page - 1] if page.page <= len(actual_pages) else None
        width, height, pixels = side_by_side_pixels(expected, actual)
        filename = f"page-{page.page:03d}-rmse-{page.rmse:07.3f}.ppm"
        write_ppm(out_dir / filename, width, height, pixels)
    return out_dir


CAPTION_LINE_RE = re.compile(r"^\s*(Figure|Table|Listing|Algorithm)\s+(\d+)\s*[:.]\s*(.+)")
LAYOUT_CAPTION_TRACE_RE = re.compile(
    r"^layout_caption\tpage=(\d+)\s+slot=(\d+)\s+line=(\d+)\s+kind=([a-z]+)\s+text=(.*)$"
)
INPUT_COMMAND_RE = re.compile(r"\\(?:input|include)\b\s*(?:\{([^{}]+)\}|([^\s{}]+))")
SOURCE_CAPTION_STOP_TOKENS = {
    "bf",
    "bfseries",
    "em",
    "emph",
    "footnotesize",
    "it",
    "itshape",
    "large",
    "normalsize",
    "rm",
    "sc",
    "scriptsize",
    "small",
    "textbf",
    "textit",
    "textsc",
}


def pdf_page_text(pdftotext: str, pdf: Path, page: int) -> str:
    completed = run(
        [pdftotext, "-layout", "-f", str(page), "-l", str(page), str(pdf), "-"],
        pdf.parent,
    )
    return completed.stdout


def caption_occurrences(
    pdftotext: str,
    pdf: Path,
    pages: int,
    source_hints: dict[tuple[str, int], SourceCaptionHint],
) -> list[CaptionOccurrence]:
    captions: list[CaptionOccurrence] = []
    seen: set[tuple[str, int, int]] = set()
    for page in range(1, pages + 1):
        text = pdf_page_text(pdftotext, pdf, page)
        for line in text.splitlines():
            match = CAPTION_LINE_RE.match(line)
            if not match:
                continue
            kind, number, caption_text = match.groups()
            number_value = int(number)
            if not source_caption_matches(kind, number_value, caption_text, source_hints):
                continue
            key = (kind, number_value, page)
            if key in seen:
                continue
            seen.add(key)
            captions.append(
                CaptionOccurrence(
                    kind=kind,
                    number=number_value,
                    page=page,
                    text=compact_text(f"{kind} {number}: {caption_text}"),
                )
            )
    return captions


def trace_caption_occurrences(
    trace_path: Path,
    source_hints: dict[tuple[str, int], SourceCaptionHint],
) -> list[CaptionOccurrence]:
    captions: list[CaptionOccurrence] = []
    seen: set[tuple[str, int, int]] = set()
    for line in trace_path.read_text(encoding="utf-8").splitlines():
        match = LAYOUT_CAPTION_TRACE_RE.match(line)
        if not match:
            continue
        page, _, _, kind_raw, text = match.groups()
        caption_match = CAPTION_LINE_RE.match(text)
        if not caption_match:
            continue
        kind, number, caption_text = caption_match.groups()
        if kind.lower() != kind_raw:
            continue
        number_value = int(number)
        if not source_caption_matches(kind, number_value, caption_text, source_hints):
            continue
        key = (kind, number_value, int(page))
        if key in seen:
            continue
        seen.add(key)
        captions.append(
            CaptionOccurrence(
                kind=kind,
                number=number_value,
                page=int(page),
                text=compact_text(text),
            )
        )
    return captions


def strip_tex_comments(source: str) -> str:
    lines: list[str] = []
    for line in source.splitlines():
        escaped = False
        kept: list[str] = []
        for ch in line:
            if ch == "%" and not escaped:
                break
            kept.append(ch)
            escaped = ch == "\\" and not escaped
            if ch != "\\":
                escaped = False
        lines.append("".join(kept))
    return "\n".join(lines)


def resolve_tex_input(base_dir: Path, name: str) -> Path:
    name = name.strip()
    path = (base_dir / name).resolve()
    if path.suffix:
        return path
    return path.with_suffix(".tex")


def expanded_tex_source(path: Path, seen: set[Path] | None = None) -> str:
    seen = set() if seen is None else seen
    path = path.resolve()
    if path in seen:
        return ""
    seen.add(path)
    source = strip_tex_comments(path.read_text(encoding="utf-8", errors="replace"))

    def replace_input(match: re.Match[str]) -> str:
        input_name = match.group(1) or match.group(2)
        if not input_name:
            return ""
        input_path = resolve_tex_input(path.parent, input_name)
        if not input_path.exists():
            return ""
        return expanded_tex_source(input_path, seen)

    return INPUT_COMMAND_RE.sub(replace_input, source)


def take_balanced(source: str, start: int, open_ch: str, close_ch: str) -> tuple[str, int] | None:
    if start >= len(source) or source[start] != open_ch:
        return None
    depth = 0
    escaped = False
    payload: list[str] = []
    for index in range(start, len(source)):
        ch = source[index]
        if escaped:
            if depth > 0:
                payload.append(ch)
            escaped = False
            continue
        if ch == "\\":
            escaped = True
            if depth > 0:
                payload.append(ch)
            continue
        if ch == open_ch:
            depth += 1
            if depth > 1:
                payload.append(ch)
            continue
        if ch == close_ch:
            depth -= 1
            if depth == 0:
                return ("".join(payload), index + 1)
            payload.append(ch)
            continue
        if depth > 0:
            payload.append(ch)
    return None


def skip_space(source: str, index: int) -> int:
    while index < len(source) and source[index].isspace():
        index += 1
    return index


def take_source_braced(source: str, index: int) -> tuple[str, int] | None:
    return take_balanced(source, skip_space(source, index), "{", "}")


def take_source_optional(source: str, index: int) -> tuple[str | None, int]:
    index = skip_space(source, index)
    bracketed = take_balanced(source, index, "[", "]")
    if bracketed is None:
        return None, index
    return bracketed


def source_caption_tokens(text: str) -> tuple[str, ...]:
    text = text.replace("~", " ")
    text = re.sub(r"\\(?:[a-zA-Z@]+|.)", " ", text)
    text = text.replace("{", " ").replace("}", " ")
    tokens = re.findall(r"[a-zA-Z0-9]+", text.lower())
    return tuple(token for token in tokens if token not in SOURCE_CAPTION_STOP_TOKENS)


def source_caption_matches(
    kind: str,
    number: int,
    caption_text: str,
    source_hints: dict[tuple[str, int], SourceCaptionHint],
) -> bool:
    hint = source_hints.get((kind, number))
    if hint is None or not hint.tokens:
        return True
    pdf_tokens = source_caption_tokens(caption_text)
    if not pdf_tokens:
        return False
    expected = hint.tokens[: min(4, len(hint.tokens))]
    search_window = pdf_tokens[: max(10, len(expected) + 2)]
    cursor = 0
    for token in expected:
        try:
            offset = search_window[cursor:].index(token)
        except ValueError:
            return False
        cursor += offset + 1
    return True


def caption_option_value_source(options: str) -> str | None:
    index = options.find("caption")
    while index != -1:
        after = index + len("caption")
        cursor = skip_space(options, after)
        if cursor < len(options) and options[cursor] == "=":
            cursor = skip_space(options, cursor + 1)
            if cursor < len(options) and options[cursor] == "{":
                payload = take_balanced(options, cursor, "{", "}")
                return None if payload is None else payload[0]
            end = cursor
            while end < len(options) and options[end] not in ",]":
                end += 1
            return options[cursor:end].strip()
        index = options.find("caption", after)
    return None


def source_caption_hints(root_tex: Path) -> dict[tuple[str, int], SourceCaptionHint]:
    source = expanded_tex_source(root_tex)
    cursor = 0
    current_float: list[str] = []
    counters = {"Figure": 0, "Table": 0, "Listing": 0}
    hints: dict[tuple[str, int], SourceCaptionHint] = {}
    markers = ("\\begin", "\\end", "\\captionof", "\\captionsetup", "\\caption")
    while cursor < len(source):
        next_match = min(
            (
                (index, marker)
                for marker in markers
                if (index := source.find(marker, cursor)) != -1
            ),
            key=lambda item: (item[0], -len(item[1])),
            default=None,
        )
        if next_match is None:
            break
        index, marker = next_match
        cursor = index + len(marker)
        if marker == "\\begin":
            parsed = take_source_braced(source, cursor)
            if parsed is None:
                continue
            env, cursor = parsed
            env_base = env.strip().rstrip("*")
            options, cursor = take_source_optional(source, cursor)
            if env_base == "figure":
                current_float.append("Figure")
            elif env_base == "table":
                current_float.append("Table")
            elif env_base == "lstlisting" and options:
                caption = caption_option_value_source(options)
                if caption:
                    counters["Listing"] += 1
                    number = counters["Listing"]
                    hints[("Listing", number)] = SourceCaptionHint(
                        "Listing", number, source_caption_tokens(caption)
                    )
            continue
        if marker == "\\end":
            parsed = take_source_braced(source, cursor)
            if parsed is None:
                continue
            env, cursor = parsed
            env_base = env.strip().rstrip("*")
            if env_base in {"figure", "table"} and current_float:
                current_float.pop()
            continue
        if marker == "\\captionof":
            parsed_kind = take_source_braced(source, cursor)
            if parsed_kind is None:
                continue
            kind_raw, cursor = parsed_kind
            _, cursor = take_source_optional(source, cursor)
            parsed_caption = take_source_braced(source, cursor)
            if parsed_caption is None:
                continue
            caption, cursor = parsed_caption
            kind = "Table" if kind_raw.strip().startswith("table") else "Figure"
            counters[kind] += 1
            number = counters[kind]
            hints[(kind, number)] = SourceCaptionHint(kind, number, source_caption_tokens(caption))
            continue
        if marker == "\\captionsetup":
            _, cursor = take_source_optional(source, cursor)
            parsed = take_source_braced(source, cursor)
            if parsed is not None:
                _, cursor = parsed
            continue
        if marker == "\\caption":
            _, cursor = take_source_optional(source, cursor)
            parsed_caption = take_source_braced(source, cursor)
            if parsed_caption is None:
                continue
            caption, cursor = parsed_caption
            kind = current_float[-1] if current_float else "Figure"
            counters[kind] += 1
            number = counters[kind]
            hints[(kind, number)] = SourceCaptionHint(kind, number, source_caption_tokens(caption))
    return hints


def compact_text(text: str, limit: int = 140) -> str:
    compact = " ".join(text.split())
    if len(compact) <= limit:
        return compact
    return compact[: limit - 1].rstrip() + "..."


def first_caption_pages(captions: list[CaptionOccurrence]) -> dict[tuple[str, int], CaptionOccurrence]:
    first: dict[tuple[str, int], CaptionOccurrence] = {}
    for caption in captions:
        key = (caption.kind, caption.number)
        if key not in first or caption.page < first[key].page:
            first[key] = caption
    return first


def caption_page_drifts(
    pdftotext: str,
    baseline_pdf: Path,
    native_pdf: Path,
    baseline_pages: int,
    native_pages: int,
    source_hints: dict[tuple[str, int], SourceCaptionHint],
    native_trace_path: Path | None,
) -> list[CaptionDrift]:
    baseline = first_caption_pages(
        caption_occurrences(pdftotext, baseline_pdf, baseline_pages, source_hints)
    )
    if native_trace_path is not None and native_trace_path.exists():
        native = first_caption_pages(trace_caption_occurrences(native_trace_path, source_hints))
    else:
        native = first_caption_pages(
            caption_occurrences(pdftotext, native_pdf, native_pages, source_hints)
        )
    drifts: list[CaptionDrift] = []
    for key in sorted(set(baseline) & set(native), key=lambda item: (item[0], item[1])):
        expected = baseline[key]
        actual = native[key]
        delta = actual.page - expected.page
        drifts.append(
            CaptionDrift(
                kind=expected.kind,
                number=expected.number,
                baseline_page=expected.page,
                native_page=actual.page,
                delta_pages=delta,
                baseline_text=expected.text,
                native_text=actual.text,
            )
        )
    return sorted(
        drifts,
        key=lambda drift: (abs(drift.delta_pages), drift.kind, drift.number),
        reverse=True,
    )


def caption_drift_summary(drifts: list[CaptionDrift]) -> CaptionDriftSummary:
    if not drifts:
        return CaptionDriftSummary(
            count=0,
            sum_abs_pages=0,
            mean_abs_pages=0.0,
            max_abs_pages=0,
        )
    abs_pages = [abs(drift.delta_pages) for drift in drifts]
    return CaptionDriftSummary(
        count=len(drifts),
        sum_abs_pages=sum(abs_pages),
        mean_abs_pages=statistics.fmean(abs_pages),
        max_abs_pages=max(abs_pages),
    )


def caption_drift_threshold_requested(args: argparse.Namespace) -> bool:
    return (
        args.max_caption_drift_sum is not None
        or args.max_caption_drift_mean is not None
        or args.max_caption_drift_page is not None
    )


def native_coverage_threshold_requested(args: argparse.Namespace) -> bool:
    return (
        args.max_two_column_graphic_float_fallbacks is not None
        or args.max_two_column_wide_graphic_float_fallbacks is not None
        or args.max_two_column_graphic_float_fallback_native_slots is not None
        or args.max_two_column_wide_graphic_float_fallback_native_slots is not None
    )


def assess_status(
    pages: list[PageMetrics],
    expected_count: int,
    actual_count: int,
    caption_summary: CaptionDriftSummary,
    native: BuildResult,
    args: argparse.Namespace,
) -> tuple[str, list[str], list[str]]:
    failures: list[str] = []
    warnings: list[str] = []
    page_count_match = expected_count == actual_count
    if not page_count_match:
        message = f"page count differs: external={expected_count}, native={actual_count}"
        if args.require_page_count_match:
            failures.append(message)
        else:
            warnings.append(message)

    mean_rmse = statistics.fmean(page.rmse for page in pages)
    max_page_rmse = max(page.rmse for page in pages)
    max_diff_ratio = max(page.different_pixel_ratio for page in pages)

    if args.max_mean_rmse is not None and mean_rmse > args.max_mean_rmse:
        failures.append(f"mean RMSE {mean_rmse:.3f} > {args.max_mean_rmse:.3f}")
    if args.max_page_rmse is not None and max_page_rmse > args.max_page_rmse:
        failures.append(f"max page RMSE {max_page_rmse:.3f} > {args.max_page_rmse:.3f}")
    if (
        args.max_different_pixel_ratio is not None
        and max_diff_ratio > args.max_different_pixel_ratio
    ):
        failures.append(
            f"max different-pixel ratio {max_diff_ratio:.4f} "
            f"> {args.max_different_pixel_ratio:.4f}"
        )
    if caption_drift_threshold_requested(args) and caption_summary.count == 0:
        failures.append("caption drift metrics unavailable")
    if (
        args.max_caption_drift_sum is not None
        and caption_summary.sum_abs_pages > args.max_caption_drift_sum
    ):
        failures.append(
            f"caption drift sum {caption_summary.sum_abs_pages} "
            f"> {args.max_caption_drift_sum}"
        )
    if (
        args.max_caption_drift_mean is not None
        and caption_summary.mean_abs_pages > args.max_caption_drift_mean
    ):
        failures.append(
            f"caption drift mean {caption_summary.mean_abs_pages:.3f} "
            f"> {args.max_caption_drift_mean:.3f}"
        )
    if (
        args.max_caption_drift_page is not None
        and caption_summary.max_abs_pages > args.max_caption_drift_page
    ):
        failures.append(
            f"caption drift max {caption_summary.max_abs_pages} "
            f"> {args.max_caption_drift_page}"
        )
    if (
        args.max_two_column_graphic_float_fallbacks is not None
        and native.two_column_graphic_float_fallbacks
        > args.max_two_column_graphic_float_fallbacks
    ):
        failures.append(
            "two-column graphic float fallbacks "
            f"{native.two_column_graphic_float_fallbacks} "
            f"> {args.max_two_column_graphic_float_fallbacks}"
        )
    if (
        args.max_two_column_wide_graphic_float_fallbacks is not None
        and native.two_column_wide_graphic_float_fallbacks
        > args.max_two_column_wide_graphic_float_fallbacks
    ):
        failures.append(
            "two-column wide graphic float fallbacks "
            f"{native.two_column_wide_graphic_float_fallbacks} "
            f"> {args.max_two_column_wide_graphic_float_fallbacks}"
        )
    if (
        args.max_two_column_graphic_float_fallback_native_slots is not None
        and native.two_column_graphic_float_fallback_estimated_native_slots
        > args.max_two_column_graphic_float_fallback_native_slots
    ):
        failures.append(
            "two-column graphic float fallback native slots "
            f"{native.two_column_graphic_float_fallback_estimated_native_slots} "
            f"> {args.max_two_column_graphic_float_fallback_native_slots}"
        )
    if (
        args.max_two_column_wide_graphic_float_fallback_native_slots is not None
        and native.two_column_wide_graphic_float_fallback_estimated_native_slots
        > args.max_two_column_wide_graphic_float_fallback_native_slots
    ):
        failures.append(
            "two-column wide graphic float fallback native slots "
            f"{native.two_column_wide_graphic_float_fallback_estimated_native_slots} "
            f"> {args.max_two_column_wide_graphic_float_fallback_native_slots}"
        )

    if args.fail_on_warn and warnings:
        failures.extend(warnings)
        warnings = []

    if failures:
        return "FAIL", failures, warnings
    if (
        args.max_mean_rmse is None
        and args.max_page_rmse is None
        and args.max_different_pixel_ratio is None
        and not caption_drift_threshold_requested(args)
        and not native_coverage_threshold_requested(args)
    ):
        return ("WARN" if warnings else "MEASURE"), failures, warnings
    return ("WARN" if warnings else "PASS"), failures, warnings


def compare_case(
    root: Path,
    texpilot: Path,
    paper: Path,
    case_dir: Path,
    args: argparse.Namespace,
) -> CaseResult:
    if not paper.exists():
        raise FileNotFoundError(f"paper not found: {paper}")

    baseline_source = case_dir / "baseline-source"
    native_source = case_dir / "native-source"
    copy_source_tree(paper.parent, baseline_source)
    copy_source_tree(paper.parent, native_source)

    main_name = paper.name
    baseline = run_pdflatex_baseline(baseline_source, main_name)
    native = run_native_build(
        root,
        texpilot,
        native_source,
        main_name,
        case_dir / "native-out",
        allow_fallback=args.allow_fallback,
    )
    expected_pages = render_pdf(
        args.pdftoppm,
        args.resolution,
        Path(baseline.pdf),
        case_dir / "baseline-pages",
    )
    actual_pages = render_pdf(
        args.pdftoppm,
        args.resolution,
        Path(native.pdf),
        case_dir / "native-pages",
    )
    pages = compare_pages(expected_pages, actual_pages, args.pixel_tolerance)
    comparison_dir = None
    if args.write_comparison_pages:
        count = args.top_pages or 5
        comparison_dir = write_page_comparisons(
            case_dir / "page-comparisons",
            expected_pages,
            actual_pages,
            pages,
            count,
        )
    compute_caption_drifts = args.caption_drift or caption_drift_threshold_requested(args)
    caption_drifts = (
        caption_page_drifts(
            args.pdftotext,
            Path(baseline.pdf),
            Path(native.pdf),
            len(expected_pages),
            len(actual_pages),
            source_caption_hints(baseline_source / main_name),
            Path(native.trace_path) if native.trace_path else None,
        )
        if compute_caption_drifts
        else []
    )
    caption_summary = caption_drift_summary(caption_drifts)
    status, failures, warnings = assess_status(
        pages,
        len(expected_pages),
        len(actual_pages),
        caption_summary,
        native,
        args,
    )
    exact_render_match = (
        len(expected_pages) == len(actual_pages)
        and all(page.exact_sha_match for page in pages)
    )

    return CaseResult(
        paper=str(paper),
        status=status,
        baseline=BuildResult(baseline.pdf, baseline.elapsed_seconds, native=False, pages=len(expected_pages)),
        native=BuildResult(
            native.pdf,
            native.elapsed_seconds,
            native=True,
            pages=len(actual_pages),
            trace_path=native.trace_path,
            two_column_graphic_float_fallbacks=native.two_column_graphic_float_fallbacks,
            two_column_wide_graphic_float_fallbacks=(
                native.two_column_wide_graphic_float_fallbacks
            ),
            two_column_graphic_float_fallback_estimated_native_slots=(
                native.two_column_graphic_float_fallback_estimated_native_slots
            ),
            two_column_wide_graphic_float_fallback_estimated_native_slots=(
                native.two_column_wide_graphic_float_fallback_estimated_native_slots
            ),
        ),
        page_count_match=len(expected_pages) == len(actual_pages),
        exact_render_match=exact_render_match,
        mean_rmse=statistics.fmean(page.rmse for page in pages),
        max_page_rmse=max(page.rmse for page in pages),
        max_different_pixel_ratio=max(page.different_pixel_ratio for page in pages),
        dimension_mismatches=sum(1 for page in pages if not page.dimensions_match),
        failures=failures,
        warnings=warnings,
        pages=pages,
        caption_drifts=caption_drifts,
        caption_drift_summary=caption_summary,
        comparison_dir=str(comparison_dir) if comparison_dir else None,
    )


def print_table(
    results: list[CaseResult],
    workdir: Path,
    thresholded: bool,
    top_pages: int,
    caption_drift: int,
    show_native_coverage: bool,
) -> None:
    mode = "gate" if thresholded else "measurement"
    print(f"native pdftex rendered parity {mode}")
    print(f"workdir: {workdir}")
    print()
    print(
        f"{'status':<8} {'paper':<42} {'pages':>11} {'exact':>7} "
        f"{'mean_rmse':>10} {'max_rmse':>9} {'max_diff':>9} {'dims':>6}"
    )
    for result in results:
        paper = Path(result.paper).name
        if "examples" in Path(result.paper).parts:
            try:
                paper = str(Path(result.paper).relative_to(Path.cwd()))
            except ValueError:
                paper = result.paper
        pages = f"{result.baseline.pages}/{result.native.pages}"
        exact = "yes" if result.exact_render_match else "no"
        print(
            f"{result.status:<8} {paper:<42} {pages:>11} {exact:>7} "
            f"{result.mean_rmse:10.3f} {result.max_page_rmse:9.3f} "
            f"{result.max_different_pixel_ratio:9.4f} {result.dimension_mismatches:6d}"
        )
        for warning in result.warnings:
            print(f"  warning: {warning}")
        for failure in result.failures:
            print(f"  failure: {failure}")
        if result.caption_drift_summary.count:
            summary = result.caption_drift_summary
            print(
                "  caption drift summary: "
                f"count={summary.count} "
                f"sum_abs={summary.sum_abs_pages} "
                f"mean_abs={summary.mean_abs_pages:.3f} "
                f"max_abs={summary.max_abs_pages}"
            )
        total_float_fallbacks = result.native.two_column_graphic_float_fallbacks
        wide_float_fallbacks = result.native.two_column_wide_graphic_float_fallbacks
        total_float_fallback_slots = (
            result.native.two_column_graphic_float_fallback_estimated_native_slots
        )
        wide_float_fallback_slots = (
            result.native.two_column_wide_graphic_float_fallback_estimated_native_slots
        )
        if (
            show_native_coverage
            or total_float_fallbacks
            or wide_float_fallbacks
            or total_float_fallback_slots
            or wide_float_fallback_slots
        ):
            print(
                "  native coverage gaps: "
                f"two_column_graphic_float_fallbacks={total_float_fallbacks} "
                f"two_column_wide_graphic_float_fallbacks={wide_float_fallbacks} "
                f"two_column_graphic_float_fallback_native_slots={total_float_fallback_slots} "
                f"two_column_wide_graphic_float_fallback_native_slots={wide_float_fallback_slots}"
            )
        top = worst_pages(result.pages, top_pages)
        if top:
            formatted = ", ".join(
                f"p{page.page} rmse={page.rmse:.3f} diff={page.different_pixel_ratio:.4f}"
                for page in top
            )
            print(f"  worst pages: {formatted}")
        if caption_drift and result.caption_drifts:
            print("  caption drift:")
            for drift in result.caption_drifts[:caption_drift]:
                direction = "late" if drift.delta_pages > 0 else "early" if drift.delta_pages < 0 else "same"
                print(
                    f"    {drift.kind} {drift.number}: baseline p{drift.baseline_page}, "
                    f"native p{drift.native_page} ({drift.delta_pages:+d}, {direction})"
                )
                print(f"      baseline: {drift.baseline_text}")
                print(f"      native:   {drift.native_text}")
        if result.comparison_dir:
            print(f"  comparisons: {result.comparison_dir}")


def result_for_json(result: CaseResult, include_pages: bool) -> dict[str, object]:
    payload = asdict(result)
    if not include_pages:
        payload.pop("pages", None)
    return payload


def main() -> int:
    args = parse_args()
    for command in ("cargo", "pdflatex", "bibtex", args.pdftoppm):
        require_command(command)
    if args.caption_drift or caption_drift_threshold_requested(args):
        require_command(args.pdftotext)

    root = Path(__file__).resolve().parents[1]
    workdir = (
        args.workdir.resolve()
        if args.workdir
        else Path(tempfile.mkdtemp(prefix="texpilot-native-pdftex-parity-")).resolve()
    )
    workdir.mkdir(parents=True, exist_ok=True)

    texpilot = args.texpilot.resolve() if args.texpilot else build_texpilot(root, args.profile)
    if not texpilot.exists():
        raise FileNotFoundError(f"texpilot binary not found: {texpilot}")

    papers = selected_papers(root, workdir, args)
    thresholded = (
        args.require_page_count_match
        or args.fail_on_warn
        or args.max_mean_rmse is not None
        or args.max_page_rmse is not None
        or args.max_different_pixel_ratio is not None
        or caption_drift_threshold_requested(args)
        or native_coverage_threshold_requested(args)
    )
    results = [
        compare_case(root, texpilot, paper, workdir / f"case-{index:02d}", args)
        for index, paper in enumerate(papers, start=1)
    ]
    print_table(
        results,
        workdir,
        thresholded,
        args.top_pages,
        args.caption_drift,
        native_coverage_threshold_requested(args),
    )
    if args.json:
        print()
        print(
            json.dumps(
                {
                    "texpilot": str(texpilot),
                    "results": [result_for_json(result, args.json_pages) for result in results],
                },
                indent=2,
            )
        )

    failed = any(result.status == "FAIL" for result in results)
    if not args.keep_workdir and not args.workdir:
        shutil.rmtree(workdir, ignore_errors=True)
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())

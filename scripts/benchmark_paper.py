#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import shutil
import statistics
import subprocess
import tempfile
import time
import re
from dataclasses import dataclass
from pathlib import Path


FULL_LINE_COMMENT_PREFIX = "% texpilot benchmark seeded full-line comment"
SCENARIO_ALIASES = {
    "clean": "clean",
    "warm": "warm-edits",
    "warm-edit": "warm-edits",
    "warm-edits": "warm-edits",
}


def parse_scenario(value: str) -> str:
    try:
        return SCENARIO_ALIASES[value]
    except KeyError as exc:
        valid = ", ".join(SCENARIO_ALIASES)
        raise argparse.ArgumentTypeError(
            f"invalid scenario {value!r}; choose one of: {valid}"
        ) from exc


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Compare clean-build and warmed edit-loop wall-clock time for pdflatex, latexmk, and texpilot runners."
    )
    parser.add_argument(
        "main",
        nargs="?",
        default="examples/arXiv-2605.26379v1/main.tex",
        help="root TeX file to benchmark",
    )
    parser.add_argument("--runs", type=int, default=5)
    parser.add_argument(
        "--workdir",
        type=Path,
        help="temporary benchmark workspace; defaults to a fresh /tmp/texpilot-bench-* directory",
    )
    parser.add_argument(
        "--profile",
        choices=("release", "debug"),
        default="release",
        help="Cargo profile used for the texpilot binary",
    )
    parser.add_argument(
        "--texpilot",
        type=Path,
        help="prebuilt texpilot binary to benchmark instead of building one",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="print raw JSON in addition to the summary table",
    )
    parser.add_argument(
        "--scenario",
        type=parse_scenario,
        default="clean",
        metavar="{clean,warm-edits}",
        help="benchmark fresh clean builds or warmed edit-loop rebuilds; warm and warm-edit are aliases",
    )
    parser.add_argument(
        "--gate",
        action="store_true",
        help="exit nonzero if texpilot-direct exceeds the latexmk median threshold",
    )
    parser.add_argument(
        "--max-latexmk-ratio",
        type=float,
        default=1.0,
        metavar="RATIO",
        help=(
            "maximum allowed texpilot-direct median divided by latexmk median "
            "when --gate is enabled"
        ),
    )
    parser.add_argument(
        "--gate-absolute-tolerance",
        type=float,
        default=0.05,
        metavar="SECONDS",
        help=(
            "absolute timing tolerance added to the gate threshold; this keeps "
            "sub-timer-noise ties from failing noisy one-run edit-loop checks"
        ),
    )
    return parser.parse_args()


def run_command(command: list[str], cwd: Path) -> float:
    started = time.perf_counter()
    subprocess.run(
        command,
        cwd=cwd,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=True,
    )
    return time.perf_counter() - started


def run_command_output(command: list[str], cwd: Path) -> tuple[float, str]:
    started = time.perf_counter()
    completed = subprocess.run(
        command,
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
        check=True,
    )
    return time.perf_counter() - started, completed.stdout


@dataclass
class CommandMeasurement:
    elapsed: float
    report: dict | None = None


def command_result(measurement: CommandMeasurement) -> dict:
    return {
        "elapsed": measurement.elapsed,
        "report": measurement.report,
    }


def copy_source(source_dir: Path, destination: Path) -> None:
    if destination.exists():
        shutil.rmtree(destination)
    ignore = shutil.ignore_patterns(
        "*.aux",
        "*.bbl",
        "*.blg",
        "*.brf",
        "*.fdb_latexmk",
        "*.fls",
        "*.log",
        "*.out",
        "*.pdfsync",
        "*.synctex.gz",
    )
    shutil.copytree(source_dir, destination, ignore=ignore)


def output_dir(cwd: Path) -> Path:
    out = cwd / "out"
    out.mkdir(exist_ok=True)
    return out


def pdflatex_args(main_name: str, out: Path) -> list[str]:
    return [
        "pdflatex",
        "-interaction=nonstopmode",
        "-halt-on-error",
        "-file-line-error",
        f"-output-directory={out}",
        main_name,
    ]


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
    text = log_path.read_text(errors="ignore")
    return any(line_requests_rerun(line) for line in text.splitlines())


def aux_requests_bibtex(aux_path: Path) -> bool:
    return aux_path.exists() and r"\bibdata" in aux_path.read_text(errors="ignore")


def latexmk_args(main_name: str, out: Path) -> list[str]:
    return [
        "latexmk",
        "-pdf",
        "-interaction=nonstopmode",
        "-halt-on-error",
        "-file-line-error",
        "-recorder",
        f"-outdir={out}",
        main_name,
    ]


def texpilot_args(
    root: Path,
    texpilot: Path,
    cwd: Path,
    main_name: str,
    out: Path,
    extra: list[str] | None = None,
) -> list[str]:
    command = [
        str(texpilot),
        "build",
        str(cwd / main_name),
        "--out-dir",
        str(out),
        "--quiet",
    ]
    if extra:
        command.extend(extra)
    return command


def run_texpilot_report(
    root: Path,
    texpilot: Path,
    cwd: Path,
    main_name: str,
    out: Path,
    extra: list[str] | None = None,
) -> CommandMeasurement:
    report_args = ["--report-json"]
    if extra:
        report_args = extra + report_args
    elapsed, stdout = run_command_output(
        texpilot_args(root, texpilot, cwd, main_name, out, report_args),
        root,
    )
    return CommandMeasurement(elapsed, json.loads(stdout))


def clean_command_specs(root: Path, main_name: str, texpilot: Path):
    def pdflatex_one(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return CommandMeasurement(run_command(pdflatex_args(main_name, out), cwd))

    def pdflatex_full(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        started = time.perf_counter()
        run_command(pdflatex_args(main_name, out), cwd)
        stem = Path(main_name).with_suffix("").name
        if aux_requests_bibtex(out / f"{stem}.aux"):
            run_command(["bibtex", str(Path("out") / stem)], cwd)
        run_command(pdflatex_args(main_name, out), cwd)
        run_command(pdflatex_args(main_name, out), cwd)
        return CommandMeasurement(time.perf_counter() - started)

    def pdflatex_converged(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        stem = Path(main_name).with_suffix("").name
        log = out / f"{stem}.log"
        started = time.perf_counter()
        run_command(pdflatex_args(main_name, out), cwd)
        if aux_requests_bibtex(out / f"{stem}.aux"):
            run_command(["bibtex", str(Path("out") / stem)], cwd)
        for _ in range(8):
            run_command(pdflatex_args(main_name, out), cwd)
            if not needs_rerun(log):
                return CommandMeasurement(time.perf_counter() - started)
        raise RuntimeError(f"{main_name} did not converge after 8 post-BibTeX pdflatex runs")

    def latexmk(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return CommandMeasurement(run_command(latexmk_args(main_name, out), cwd))

    def texpilot_direct(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return run_texpilot_report(root, texpilot, cwd, main_name, out)

    def texpilot_direct_format(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return run_texpilot_report(
            root, texpilot, cwd, main_name, out, ["--precompile-preamble"]
        )

    def texpilot_pdftex(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return run_texpilot_report(
            root, texpilot, cwd, main_name, out, ["--engine", "texpilot-pdftex"]
        )

    def texpilot_latexmk(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return run_texpilot_report(
            root, texpilot, cwd, main_name, out, ["--runner", "latexmk"]
        )

    def texpilot_no_draft_prepass(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return run_texpilot_report(
            root, texpilot, cwd, main_name, out, ["--draft-prepass", "never"]
        )

    def texpilot_draft_prepass_always(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return run_texpilot_report(
            root, texpilot, cwd, main_name, out, ["--draft-prepass", "always"]
        )

    def texpilot_once(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return run_texpilot_report(root, texpilot, cwd, main_name, out, ["--once"])

    def texpilot_fast_once(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return run_texpilot_report(
            root, texpilot, cwd, main_name, out, ["--once", "--fast"]
        )

    def texpilot_fast_format_cold(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        return run_texpilot_report(
            root,
            texpilot,
            cwd,
            main_name,
            out,
            ["--once", "--fast", "--precompile-preamble"],
        )

    def texpilot_fast_format_cached(cwd: Path) -> CommandMeasurement:
        out = output_dir(cwd)
        run_texpilot_report(
            root,
            texpilot,
            cwd,
            main_name,
            out,
            ["--once", "--fast", "--precompile-preamble"],
        )
        return run_texpilot_report(
            root,
            texpilot,
            cwd,
            main_name,
            out,
            ["--once", "--fast", "--precompile-preamble", "--force"],
        )

    return {
        "pdflatex-1pass": pdflatex_one,
        "pdflatex-full": pdflatex_full,
        "pdflatex-converged": pdflatex_converged,
        "latexmk": latexmk,
        "texpilot-direct": texpilot_direct,
        "texpilot-direct-format": texpilot_direct_format,
        "texpilot-pdftex": texpilot_pdftex,
        "texpilot-no-draft": texpilot_no_draft_prepass,
        "texpilot-draft-all": texpilot_draft_prepass_always,
        "texpilot-latexmk": texpilot_latexmk,
        "texpilot-once": texpilot_once,
        "texpilot-fast-once": texpilot_fast_once,
        "texpilot-fast-format-cold": texpilot_fast_format_cold,
        "texpilot-fast-format-cached": texpilot_fast_format_cached,
    }


def warm_command_specs(root: Path, main_name: str, texpilot: Path):
    def latexmk(cwd: Path) -> CommandMeasurement:
        return CommandMeasurement(run_command(latexmk_args(main_name, output_dir(cwd)), cwd))

    def texpilot_runner(extra: list[str] | None = None):
        def run(cwd: Path) -> CommandMeasurement:
            return run_texpilot_report(root, texpilot, cwd, main_name, output_dir(cwd), extra)

        return run

    return {
        "latexmk": latexmk,
        "texpilot-direct": texpilot_runner(),
        "texpilot-no-draft": texpilot_runner(["--draft-prepass", "never"]),
        "texpilot-draft-all": texpilot_runner(["--draft-prepass", "always"]),
    }


def apply_trailing_comment_edit(cwd: Path, main_name: str, run_index: int) -> None:
    main_path = cwd / main_name
    with main_path.open("a", encoding="utf-8") as handle:
        handle.write(f"\n% texpilot benchmark trailing comment edit {run_index}\n")


def apply_body_comment_edit(cwd: Path, main_name: str, run_index: int) -> None:
    main_path = cwd / main_name
    source = main_path.read_text(encoding="utf-8")
    end_start, _ = uncommented_end_document_span(source)
    prefix = "" if end_start > 0 and source[end_start - 1] in "\r\n" else "\n"
    insertion = f"{prefix}% texpilot benchmark in-document comment edit {run_index}\n"
    main_path.write_text(source[:end_start] + insertion + source[end_start:], encoding="utf-8")


def seed_full_line_comment(cwd: Path, main_name: str) -> None:
    main_path = cwd / main_name
    source = main_path.read_text(encoding="utf-8")
    end_start, _ = uncommented_end_document_span(source)
    main_path.write_text(
        source[:end_start] + f"{FULL_LINE_COMMENT_PREFIX}\n" + source[end_start:],
        encoding="utf-8",
    )


def seed_inline_comment_padding_target(cwd: Path, main_name: str) -> None:
    main_path = cwd / main_name
    source = main_path.read_text(encoding="utf-8")
    end_start, _ = uncommented_end_document_span(source)
    prefix = "" if end_start > 0 and source[end_start - 1] in "\r\n" else "\n"
    insertion = f"{prefix}Texpilot inline comment padding target. % seed\n"
    main_path.write_text(source[:end_start] + insertion + source[end_start:], encoding="utf-8")


def seed_inline_verb_comment_target(cwd: Path, main_name: str) -> None:
    main_path = cwd / main_name
    source = main_path.read_text(encoding="utf-8")
    end_start, _ = uncommented_end_document_span(source)
    prefix = "" if end_start > 0 and source[end_start - 1] in "\r\n" else "\n"
    insertion = (
        f"{prefix}Texpilot inline verb target "
        r"\verb|% literal \end{document}| text. % seed"
        "\n"
    )
    main_path.write_text(source[:end_start] + insertion + source[end_start:], encoding="utf-8")


def apply_full_line_comment_edit(cwd: Path, main_name: str, run_index: int) -> None:
    main_path = cwd / main_name
    lines = main_path.read_text(encoding="utf-8").splitlines(keepends=True)
    for index, line in enumerate(lines):
        body, ending = split_line_ending(line)
        if body.startswith(FULL_LINE_COMMENT_PREFIX) or body.startswith(
            "% texpilot benchmark full-line comment edit"
        ):
            lines[index] = f"% texpilot benchmark full-line comment edit {run_index}{ending}"
            main_path.write_text("".join(lines), encoding="utf-8")
            return
    raise ValueError("could not find seeded full-line benchmark comment")


def apply_inline_comment_padding_edit(cwd: Path, main_name: str, run_index: int) -> None:
    main_path = cwd / main_name
    lines = main_path.read_text(encoding="utf-8").splitlines(keepends=True)
    for index, line in enumerate(lines):
        body, ending = split_line_ending(line)
        if body.startswith("Texpilot inline comment padding target."):
            lines[index] = (
                "Texpilot inline comment padding target.    "
                f"% padded benchmark comment edit {run_index}{ending}"
            )
            main_path.write_text("".join(lines), encoding="utf-8")
            return
    raise ValueError("could not find seeded inline-comment benchmark target")


def apply_inline_verb_comment_edit(cwd: Path, main_name: str, run_index: int) -> None:
    main_path = cwd / main_name
    lines = main_path.read_text(encoding="utf-8").splitlines(keepends=True)
    for index, line in enumerate(lines):
        body, ending = split_line_ending(line)
        if body.startswith("Texpilot inline verb target "):
            lines[index] = (
                "Texpilot inline verb target "
                r"\verb|% literal \end{document}| text. "
                f"% edited inline verb benchmark comment {run_index}{ending}"
            )
            main_path.write_text("".join(lines), encoding="utf-8")
            return
    raise ValueError("could not find seeded inline-verb benchmark target")


def split_line_ending(line: str) -> tuple[str, str]:
    if line.endswith("\r\n"):
        return line[:-2], "\r\n"
    if line.endswith("\n"):
        return line[:-1], "\n"
    if line.endswith("\r"):
        return line[:-1], "\r"
    return line, ""


def uncommented_end_document_span(source: str) -> tuple[int, int]:
    pattern = re.compile(r"\\end\s*\{\s*document\s*\}")
    offset = 0
    for line in source.splitlines(keepends=True):
        visible = strip_tex_comment(line)
        match = pattern.search(visible)
        if match:
            return offset + match.start(), offset + match.end()
        offset += len(line)
    raise ValueError("could not find uncommented \\end{document}")


def strip_tex_comment(line: str) -> str:
    escaped = False
    for index, char in enumerate(line):
        if char == "%" and not escaped:
            return line[:index]
        if char == "\\":
            escaped = not escaped
        else:
            escaped = False
    return line


def apply_touch_edit(cwd: Path, main_name: str) -> None:
    main_path = cwd / main_name
    main_path.touch()


def apply_trailing_space_edit(cwd: Path, main_name: str) -> None:
    main_path = cwd / main_name
    source = main_path.read_text(encoding="utf-8")
    updated_lines = []
    for line in source.splitlines(keepends=True):
        body, ending = split_line_ending(line)
        updated_lines.append(f"{body}   {ending}")
    main_path.write_text("".join(updated_lines), encoding="utf-8")


def apply_unused_bib_edit(cwd: Path, run_index: int) -> None:
    bib_path = first_bib_file(cwd)
    with bib_path.open("a", encoding="utf-8") as handle:
        handle.write(
            "\n"
            f"@book{{texpilotUnused{run_index},\n"
            "  author = {Texpilot Benchmark},\n"
            "  title = {Unused Bibliography Entry},\n"
            "  year = {2026}\n"
            "}\n"
        )


def apply_bib_edit(cwd: Path, run_index: int) -> None:
    bib_path = first_bib_file(cwd)
    source = bib_path.read_text(encoding="utf-8")

    def replace_title(match: re.Match[str]) -> str:
        return f"{match.group(1)}{match.group(2)} TexpilotBench{run_index}{match.group(3)}"

    updated, count = re.subn(
        r"(title\s*=\s*[{\"'])(.*?)([}\"'])",
        replace_title,
        source,
        flags=re.IGNORECASE,
    )
    if count == 0:
        updated = source + f"\n% texpilot benchmark bib edit {run_index}\n"
    bib_path.write_text(updated, encoding="utf-8")


def first_bib_file(cwd: Path) -> Path:
    bib_files = sorted(path for path in cwd.rglob("*.bib") if "out" not in path.parts)
    if not bib_files:
        raise FileNotFoundError(f"no .bib file found in {cwd}")
    return bib_files[0]


def summarize(values: list[float]) -> dict[str, float]:
    return {
        "min": min(values),
        "median": statistics.median(values),
        "mean": statistics.fmean(values),
        "max": max(values),
    }


def performance_gate_check(
    label: str,
    texpilot_median: float,
    latexmk_median: float,
    max_ratio: float,
    absolute_tolerance: float,
) -> dict:
    ratio = texpilot_median / latexmk_median if latexmk_median else float("inf")
    threshold = latexmk_median * max_ratio + absolute_tolerance
    return {
        "label": label,
        "texpilot_direct_median": texpilot_median,
        "latexmk_median": latexmk_median,
        "ratio": ratio,
        "max_latexmk_ratio": max_ratio,
        "absolute_tolerance": absolute_tolerance,
        "threshold_seconds": threshold,
        "passed": texpilot_median <= threshold,
    }


def clean_performance_gate(
    summary: dict, max_ratio: float, absolute_tolerance: float
) -> list[dict]:
    return [
        performance_gate_check(
            "clean",
            summary["texpilot-direct"]["median"],
            summary["latexmk"]["median"],
            max_ratio,
            absolute_tolerance,
        )
    ]


def warm_performance_gate(
    summary: dict,
    edit_names: list[str],
    max_ratio: float,
    absolute_tolerance: float,
) -> list[dict]:
    return [
        performance_gate_check(
            edit,
            summary["texpilot-direct"][edit]["median"],
            summary["latexmk"][edit]["median"],
            max_ratio,
            absolute_tolerance,
        )
        for edit in edit_names
    ]


def print_performance_gate(checks: list[dict]) -> None:
    if not checks:
        return
    max_ratio = checks[0]["max_latexmk_ratio"]
    tolerance = checks[0]["absolute_tolerance"]
    print()
    print(
        "performance gate: "
        f"texpilot-direct median <= {max_ratio:.3f}x latexmk median "
        f"+ {tolerance:.3f}s"
    )
    for check in checks:
        status = "PASS" if check["passed"] else "FAIL"
        print(
            f"{status:<4} {check['label']:<32} "
            f"texpilot={check['texpilot_direct_median']:.3f}s "
            f"latexmk={check['latexmk_median']:.3f}s "
            f"ratio={check['ratio']:.3f}x"
        )


def pass_summary(values: list[dict]) -> str:
    reports = [value.get("report") for value in values if value.get("report")]
    if not reports:
        return "-"
    summaries = {
        (
            report.get("tex_runs", 0),
            report.get("draft_tex_runs", 0),
            report.get("final_tex_runs", 0),
            report.get("pdf_tex_runs", report.get("final_tex_runs", 0)),
            report.get("bibliography_runs", 0),
            report.get("index_runs", 0),
            report.get("external_runs", 0),
        )
        for report in reports
    }
    if len(summaries) != 1:
        return "varies"
    tex, draft, final, pdf, bib, index, external = next(iter(summaries))
    parts = [f"{tex}T"]
    if draft or final:
        parts.append(f"{draft}d/{final}f/{pdf}p")
    if bib:
        parts.append(f"{bib}B")
    if index:
        parts.append(f"{index}I")
    if external:
        parts.append(f"{external}X")
    if reports and all(report.get("preamble_format_used") for report in reports):
        if any(report.get("preamble_format_built") for report in reports):
            parts.append("PFb")
        else:
            parts.append("PF")
    return "+".join(parts)


def slowest_pass_summary(values: list[dict]) -> str:
    entries: list[tuple[int, str, float]] = []
    for value in values:
        report = value.get("report")
        if not report:
            continue
        passes = report.get("passes") or []
        if not passes:
            continue
        index, pass_report = max(
            enumerate(passes, start=1),
            key=lambda item: item[1].get("elapsed_ms", 0.0),
        )
        kind = "d" if pass_report.get("draft") else "p" if pass_report.get("pdf_output") else "f"
        entries.append((index, kind, pass_report.get("elapsed_ms", 0.0) / 1000.0))
    if not entries:
        return "-"
    labels = {(index, kind) for index, kind, _ in entries}
    if len(labels) == 1:
        label_index, label_kind = next(iter(labels))
        label = f"{label_kind}{label_index}"
    else:
        label = "varies"
    return f"{label}:{statistics.median(elapsed for _, _, elapsed in entries):.2f}s"


def main() -> int:
    args = parse_args()
    if args.max_latexmk_ratio <= 0:
        raise ValueError("--max-latexmk-ratio must be positive")
    if args.gate_absolute_tolerance < 0:
        raise ValueError("--gate-absolute-tolerance must be nonnegative")
    root = Path(__file__).resolve().parents[1]
    main_path = (root / args.main).resolve()
    source_dir = main_path.parent
    main_name = main_path.name
    workdir = (
        args.workdir.resolve()
        if args.workdir
        else Path(tempfile.mkdtemp(prefix="texpilot-bench-")).resolve()
    )
    if args.texpilot:
        texpilot = args.texpilot.resolve()
        build_command = None
        profile = "custom"
    else:
        profile = args.profile
        texpilot = root / "target" / profile / "texpilot"
        build_command = ["cargo", "build", "--locked", "--quiet"]
        if profile == "release":
            build_command.append("--release")

    if build_command is not None:
        subprocess.run(build_command, cwd=root, check=True)
    if not texpilot.exists():
        raise FileNotFoundError(f"texpilot binary not found: {texpilot}")
    if args.workdir and workdir.exists():
        shutil.rmtree(workdir)
    workdir.mkdir(parents=True, exist_ok=True)

    gate: list[dict] | None = None
    if args.scenario == "clean":
        specs = clean_command_specs(root, main_name, texpilot)
        results: dict[str, list[dict]] = {name: [] for name in specs}

        for run_index in range(1, args.runs + 1):
            for name, runner in specs.items():
                case_dir = workdir / f"{run_index:02d}-{name}"
                copy_source(source_dir, case_dir)
                results[name].append(command_result(runner(case_dir)))

        summary = {
            name: summarize([value["elapsed"] for value in values])
            for name, values in results.items()
        }
        fastest_complete = summary["pdflatex-converged"]["median"]
        print(f"paper: {main_path}")
        print(f"texpilot: {texpilot} ({profile})")
        print(f"runs: {args.runs} clean builds per command")
        print()
        print(
            f"{'command':<31} {'min':>8} {'median':>8} {'mean':>8} "
            f"{'max':>8} {'vs conv':>9} {'passes':>14} {'slowest':>14}"
        )
        for name in [
            "pdflatex-1pass",
            "pdflatex-full",
            "pdflatex-converged",
            "latexmk",
            "texpilot-direct",
            "texpilot-direct-format",
            "texpilot-pdftex",
            "texpilot-no-draft",
            "texpilot-draft-all",
            "texpilot-latexmk",
            "texpilot-once",
            "texpilot-fast-once",
            "texpilot-fast-format-cold",
            "texpilot-fast-format-cached",
        ]:
            row = summary[name]
            ratio = row["median"] / fastest_complete
            print(
                f"{name:<31} {row['min']:8.3f} {row['median']:8.3f} "
                f"{row['mean']:8.3f} {row['max']:8.3f} {ratio:8.2f}x "
                f"{pass_summary(results[name]):>14} "
                f"{slowest_pass_summary(results[name]):>14}"
            )
        if args.gate:
            gate = clean_performance_gate(
                summary,
                args.max_latexmk_ratio,
                args.gate_absolute_tolerance,
            )
    else:
        specs = warm_command_specs(root, main_name, texpilot)
        edit_names = [
            "noop",
            "touch-main",
            "trailing-space-edit",
            "full-line-comment-edit",
            "trailing-comment-edit",
            "body-comment-edit",
            "inline-comment-padding-edit",
            "inline-verb-comment-edit",
            "unused-bib-edit",
            "bib-edit",
        ]
        results: dict[str, dict[str, list[dict]]] = {
            name: {edit: [] for edit in edit_names} for name in specs
        }

        for run_index in range(1, args.runs + 1):
            for name, runner in specs.items():
                case_dir = workdir / f"{run_index:02d}-{name}"
                copy_source(source_dir, case_dir)
                seed_full_line_comment(case_dir, main_name)
                seed_inline_comment_padding_target(case_dir, main_name)
                seed_inline_verb_comment_target(case_dir, main_name)
                runner(case_dir)
                measurement = runner(case_dir)
                results[name]["noop"].append(command_result(measurement))
                apply_touch_edit(case_dir, main_name)
                measurement = runner(case_dir)
                results[name]["touch-main"].append(command_result(measurement))
                apply_trailing_space_edit(case_dir, main_name)
                measurement = runner(case_dir)
                results[name]["trailing-space-edit"].append(command_result(measurement))
                apply_full_line_comment_edit(case_dir, main_name, run_index)
                measurement = runner(case_dir)
                results[name]["full-line-comment-edit"].append(command_result(measurement))
                apply_trailing_comment_edit(case_dir, main_name, run_index)
                measurement = runner(case_dir)
                results[name]["trailing-comment-edit"].append(command_result(measurement))
                apply_body_comment_edit(case_dir, main_name, run_index)
                measurement = runner(case_dir)
                results[name]["body-comment-edit"].append(command_result(measurement))
                apply_inline_comment_padding_edit(case_dir, main_name, run_index)
                measurement = runner(case_dir)
                results[name]["inline-comment-padding-edit"].append(command_result(measurement))
                apply_inline_verb_comment_edit(case_dir, main_name, run_index)
                measurement = runner(case_dir)
                results[name]["inline-verb-comment-edit"].append(command_result(measurement))
                apply_unused_bib_edit(case_dir, run_index)
                measurement = runner(case_dir)
                results[name]["unused-bib-edit"].append(command_result(measurement))
                apply_bib_edit(case_dir, run_index)
                measurement = runner(case_dir)
                results[name]["bib-edit"].append(command_result(measurement))

        summary = {
            name: {
                edit: summarize([value["elapsed"] for value in values])
                for edit, values in edits.items()
            }
            for name, edits in results.items()
        }
        print(f"paper: {main_path}")
        print(f"texpilot: {texpilot} ({profile})")
        print(f"runs: {args.runs} warmed edit loops per command")
        print()
        print(f"{'command':<20}" + "".join(f" {edit:>16}" for edit in edit_names))
        for name in ["latexmk", "texpilot-direct", "texpilot-no-draft", "texpilot-draft-all"]:
            row = summary[name]
            print(
                f"{name:<20}"
                + "".join(f" {row[edit]['median']:16.3f}" for edit in edit_names)
            )
        if args.gate:
            gate = warm_performance_gate(
                summary,
                edit_names,
                args.max_latexmk_ratio,
                args.gate_absolute_tolerance,
            )
    if gate is not None:
        print_performance_gate(gate)
    if args.json:
        print()
        print(
            json.dumps(
                {
                    "texpilot": str(texpilot),
                    "profile": profile,
                    "scenario": args.scenario,
                    "runs": results,
                    "summary": summary,
                    "gate": gate,
                },
                indent=2,
            )
        )
    return 1 if gate is not None and any(not check["passed"] for check in gate) else 0


if __name__ == "__main__":
    raise SystemExit(main())

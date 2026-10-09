#!/usr/bin/env python3
"""Measure recursive lookup, decoded images and watch retention on macOS.

Uses temporary copies and isolated caches. No TeX installation is needed.
Timings are evidence, not CI thresholds. Unit tests enforce scan counts and
retention budgets independently of machine speed.
"""

import argparse
import json
import os
from pathlib import Path
import re
import shutil
import statistics
import subprocess
import tempfile
import time
import zlib
import struct

REPO = Path(__file__).resolve().parent.parent


def measured(command, cwd, env, repeats):
    samples = []
    for _ in range(repeats):
        started = time.monotonic()
        process = subprocess.Popen(["/usr/bin/time", "-l", *map(str, command)], cwd=cwd,
                                   env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                   text=True, start_new_session=True)
        try:
            stdout, stderr = process.communicate(timeout=180)
        except subprocess.TimeoutExpired:
            # time is a wrapper. Stop the owned process group, not just time.
            import signal
            os.killpg(process.pid, signal.SIGKILL)
            process.communicate()
            raise
        if process.returncode:
            raise RuntimeError(f"{command}\n{stdout[-2000:]}\n{stderr[-2000:]}")
        rss = re.search(r"(\d+)\s+maximum resident set size", stderr)
        samples.append({"seconds": time.monotonic() - started,
                        "rss_mib": int(rss.group(1)) / 1048576})
    return {"median_s": statistics.median(row["seconds"] for row in samples),
            "max_rss_mib": max(row["rss_mib"] for row in samples), "samples": samples}


def png(path, number):
    def chunk(kind, payload):
        return struct.pack(">I", len(payload)) + kind + payload + struct.pack(">I", zlib.crc32(kind + payload))
    pixel = bytes([number * 7 % 256, number * 13 % 256, 90, 128])
    rows = (b"\0" + pixel * 1024) * 1024
    path.write_bytes(b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", 1024, 1024, 8, 6, 0, 0, 0))
                     + chunk(b"IDAT", zlib.compress(rows)) + chunk(b"IEND", b""))


def document(project, source=None):
    project.mkdir(parents=True, exist_ok=True)
    (project / "main.tex").write_text(source or "\\documentclass{article}\n\\begin{document}Probe.\\end{document}\n")


def pad(project, count):
    for n in range(count):
        path = project / f"unused-tree/bucket{n // 100}/group{n // 10}/leaf{n}"
        path.mkdir(parents=True, exist_ok=True)
        (path / "unused.dat").touch()


def engine_command(binary, out):
    return [binary, "__tekai-engine", "-interaction=nonstopmode", "-halt-on-error",
            "-no-shell-escape", f"-output-directory={out}", "main.tex"]


def watch_retention(binary, project, out, env):
    document(project)
    chunk = ("%" + "x" * 1022 + "\n") * 1024 + "Body text.\n"
    for n in range(25):
        (project / f"part{n}.tex").write_text(chunk)
    def update(n):
        document(project, f"\\documentclass{{article}}\n\\begin{{document}}\n\\input{{part{n}}}\nEdit {n}.\n\\end{{document}}\n")
    def wait_build(previous):
        deadline = time.monotonic() + 60
        while time.monotonic() < deadline:
            count = len(re.findall(r"^(?:built|cached) ", log.read_text(errors="replace"), re.M))
            if count > previous:
                time.sleep(0.3)
                return len(re.findall(r"^(?:built|cached) ", log.read_text(errors="replace"), re.M))
            if process.poll() is not None:
                raise RuntimeError(log.read_text()[-2000:])
            time.sleep(0.05)
        raise RuntimeError("watch timed out\n" + log.read_text()[-2000:])
    update(0)
    log = out.parent / (out.name + "-watch.log")
    samples = []
    with log.open("w") as handle:
        process = subprocess.Popen([str(binary), "watch", str(project / "main.tex"), "--preview",
                                    "--root", str(project), "--out-dir", str(out), "--no-lint", "--quiet"],
                                   cwd=project, env=env, stdout=subprocess.DEVNULL, stderr=handle)
        try:
            count = wait_build(0)
            time.sleep(0.8)
            count = len(re.findall(r"^(?:built|cached) ", log.read_text(), re.M))
            for n in range(25):
                if n:
                    update(n)
                    count = wait_build(count)
                rss = subprocess.check_output(["ps", "-o", "rss=", "-p", str(process.pid)], text=True)
                samples.append({"edit": n, "rss_mib": int(rss.strip()) / 1024})
        finally:
            process.terminate()
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
    return {"samples": samples, "first_rss_mib": samples[0]["rss_mib"],
            "last_rss_mib": samples[-1]["rss_mib"]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--engine", type=Path, default=REPO / "target/release/tekai")
    parser.add_argument("--baseline", type=Path)
    parser.add_argument("--repeats", type=int, default=3)
    parser.add_argument("--watch", action="store_true")
    parser.add_argument("--report", type=Path, default=REPO / "target/runtime-performance/report.json")
    args = parser.parse_args()
    if args.repeats < 1 or os.uname().sysname != "Darwin":
        parser.error("requires macOS and at least one repetition")
    binaries = [("candidate", args.engine.resolve())]
    if args.baseline:
        binaries.insert(0, ("baseline", args.baseline.resolve()))
    report = {"machine": subprocess.check_output(["sysctl", "-n", "machdep.cpu.brand_string"], text=True).strip(),
              "repeats": args.repeats, "results": []}
    args.report.parent.mkdir(parents=True, exist_ok=True)
    def record(label, case, **values):
        row = {"binary": label, "case": case, **values}
        report["results"].append(row)
        args.report.write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps(row), flush=True)
    with tempfile.TemporaryDirectory(prefix="tekai-runtime-bench-") as temporary:
        work = Path(temporary)
        # The OS may expose /tmp through a symlink. Use one spelling throughout.
        work = work.resolve()
        for label, binary in binaries:
            env = {"PATH": "", "HOME": str(work / "home"), "TEKAI_TEXMF_MODE": "bundled"}
            for kind in ["ENGINE", "FORMAT", "AUX", "BIBTEX"]:
                env[f"TEKAI_{kind}_CACHE"] = str(work / label / kind.lower())
            # Extract the bundle before measuring, with each binary's own cache.
            subprocess.run([binary, "locate", "article.cls"], env=env, check=True, capture_output=True)
            for count in [0, 1000, 4000]:
                project = work / f"nested-{count}"
                document(project)
                pad(project, count)
                out = work / f"{label}-nested-out-{count}"
                out.mkdir(exist_ok=True)
                paths = dict(env, TEXINPUTS=f"{project}//:{out}//:")
                command = engine_command(binary, out)
                measured(command, project, paths, 1)
                record(label, "nested-pass", directories=count, **measured(command, project, paths, args.repeats))
            for count in [1, 8, 32]:
                project = work / f"images-{count}"
                document(project)
                for n in range(count):
                    png(project / f"i{n}.png", n)
                body = "\n".join(f"\\includegraphics[width=1cm]{{i{n}.png}}\\newpage" for n in range(count))
                document(project, "\\documentclass{article}\n\\usepackage{graphicx}\n\\begin{document}\n" + body + "\n\\end{document}\n")
                out = work / f"{label}-images-out-{count}"
                out.mkdir(exist_ok=True)
                paths = dict(env, TEXINPUTS=f"{project}//:{out}//:")
                command = engine_command(binary, out)
                measured(command, project, paths, 1)
                record(label, "image-pass", images=count, **measured(command, project, paths, args.repeats))
            for kind in ["home", "site"]:
                for count in [0, 4000]:
                    tree = work / f"shared-{kind}-{count}"
                    pad(tree / "doc", count)
                    tree.mkdir(parents=True, exist_ok=True)
                    (tree / "ls-R").write_text("% ls-R\n")
                    project = work / "shared-project"
                    document(project)
                    paths = dict(env, TEKAI_TEXMF_MODE="shared", TEXMFHOME=str(tree if kind == "home" else work / "no-home"),
                                 TEXMFLOCAL=str(tree if kind == "site" else work / "no-site"))
                    out = work / f"{label}-shared-out-{kind}-{count}"
                    command = [binary, "build", project / "main.tex", "--out-dir", out, "--once", "--quiet", "--report-json"]
                    measured(command, project, paths, 1)
                    record(label, "shared-cache-hit", tree_kind=kind, directories=count,
                           **measured(command, project, paths, args.repeats))
            for paper in ["arXiv-2511.08544v3", "arXiv-2605.26379v1"]:
                for count in [0, 1000]:
                    project = work / f"{label}-{paper}-{count}"
                    shutil.copytree(REPO / "examples" / paper, project)
                    pad(project, count)
                    out = work / f"{label}-{paper}-out-{count}"
                    command = [binary, "build", project / "main.tex", "--out-dir", out, "--force", "--quiet", "--report-json"]
                    measured(command, project, env, 1)
                    record(label, "paper-forced-build", paper=paper, directories=count,
                           **measured(command, project, env, args.repeats))
            if args.watch:
                record(label, "preview-rotating-includes", **watch_retention(binary, work / f"{label}-watch",
                                                                            work / f"{label}-watch-out", env))


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Regenerate the embedded LaTeX format using Tekai itself, without system TeX.

Build Tekai after updating the package bundle, run this, then rebuild Tekai to
embed the resulting matching format. Only the maintainer runs this script.
"""

import argparse
import hashlib
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--engine", type=Path, default=ROOT / "target/debug/tekai")
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="tekai-format-") as work:
        work = Path(work)
        result = subprocess.run([
            str(args.engine.resolve()), "__tekai-engine", "-ini", "-etex",
            "-no-shell-escape", "-interaction=nonstopmode", "-halt-on-error",
            "-jobname=pdflatex", "pdflatex.ini",
        ], cwd=work, env={
            "PATH": "", "TEKAI_ENGINE_CACHE": str(work / "cache"),
            "SOURCE_DATE_EPOCH": "0", "FORCE_SOURCE_DATE": "1",
        }, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        sys.stdout.buffer.write(result.stdout)
        result.check_returncode()
        if b"No file latex2e-first-aid-for-external-files.ltx." in result.stdout:
            raise RuntimeError("The bundle is missing LaTeX's required compatibility layer")
        data = (work / "pdflatex.fmt").read_bytes()
    (ROOT / "formats/pdflatex.fmt").write_bytes(data)
    (ROOT / "formats/format-id.txt").write_text(hashlib.sha256(data).hexdigest() + "\n")


if __name__ == "__main__":
    main()

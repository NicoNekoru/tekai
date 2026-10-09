#!/usr/bin/env python3
"""Compare the bundled runtime against upstream pdfTeX without installing TeX.

Maintainer-only gate. The reference is a checksum-pinned 1.7 MB archive, not
MacTeX. Poppler is used only to verify results. Neither is a runtime dependency.
"""

import argparse
import concurrent.futures
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import shutil
import subprocess
import tarfile
import tempfile

import bundle_tex_runtime as bundle

REFERENCE = {
    "url": bundle.MIRROR + "/archive/pdftex.universal-darwin.tar.xz",
    "sha512": "507a60bf72697230e2a27df5b9ccd7d636a36fb4b3c534be4353a672c924394563691649b04492a2c5aecf001166cf7585452cc6f48bb94a7e7cb23d2fc2d046",
    "revision": "78096",
}
CASES = [("arXiv-2605.26379v1", "one"), ("arXiv-2511.08544v3", "two")]


def command(args, **kwargs):
    result = subprocess.run([str(arg) for arg in args], capture_output=True, **kwargs)
    if result.returncode:
        raise RuntimeError(result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))
    return result.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--engine", type=Path, default=bundle.ROOT / "target/debug/tekai")
    parser.add_argument("--images", action="store_true", help="also compare a page of 32 unique and repeated transparent PNGs")
    args = parser.parse_args()
    poppler = {name: shutil.which(name) for name in ["pdfinfo", "pdftotext", "pdftoppm", "pdffonts"]}
    if not all(poppler.values()):
        raise SystemExit("This maintainer gate requires Poppler; Tekai itself does not")
    target = bundle.ROOT / "target/runtime-parity"
    target.mkdir(parents=True, exist_ok=True)
    renders = bundle.ROOT / "tmp/pdfs"
    renders.mkdir(parents=True, exist_ok=True)
    cache = bundle.ROOT / "target/runtime-downloads" / (REFERENCE["sha512"] + ".tar.xz")
    data = cache.read_bytes() if cache.exists() else bundle.download(REFERENCE["url"])
    if hashlib.sha512(data).hexdigest() != REFERENCE["sha512"]:
        raise ValueError("Reference pdfTeX archive checksum mismatch")
    cache.write_bytes(data)
    format_id = (bundle.ROOT / "formats/format-id.txt").read_text().strip()
    report = {"reference": REFERENCE, "engine": str(args.engine.resolve()), "format": format_id,
              "bundle": (bundle.DEST / "bundle-id.txt").read_text().strip(), "cases": []}
    with tempfile.TemporaryDirectory(prefix="reference-", dir=target) as temporary:
        work = Path(temporary)
        engine = work / "pdftex"
        with tarfile.open(fileobj=io.BytesIO(data), mode="r:xz") as archive:
            entries = [entry for entry in archive if entry.isfile() and PurePosixPath(entry.name).name == "pdftex"]
            if len(entries) != 1:
                raise ValueError("Reference archive must contain exactly one pdftex binary")
            engine.write_bytes(archive.extractfile(entries[0]).read())
        engine.chmod(0o755)
        report["reference_version"] = command([engine, "--version"]).decode().splitlines()[0]
        (work / "texmf.cnf").write_text("\n".join([
            "main_memory = 5000000", "pool_size = 6250000", "max_strings = 500000",
            "hash_extra = 600000", "font_mem_size = 8000000", "font_max = 9000",
            "trie_size = 1000000", "hyph_size = 8191", "buf_size = 200000",
            "stack_size = 10000", "save_size = 200000", "param_size = 20000",
            "shell_escape = f", "openout_any = a",
        ]) + "\n")
        cases = [(case, suffix, bundle.ROOT / "examples" / case) for case, suffix in CASES]
        if args.images:
            from benchmark_runtime import png
            source = work / "transparent-images-source"
            source.mkdir()
            for index in range(32):
                png(source / f"image{index}.png", index)
            rows = []
            for start in range(0, 32, 8):
                rows.append("".join(f"\\includegraphics[width=1cm]{{image{index}.png}}" for index in range(start, start + 8)) + "\\par\n")
            # Repeat after eviction as well as immediately, exercising both paths.
            rows.append("\\includegraphics[width=1cm]{image0.png}\\includegraphics[width=1cm]{image0.png}")
            (source / "main.tex").write_text("\\documentclass{article}\n\\usepackage{graphicx}\n\\begin{document}\n" + "".join(rows) + "\n\\end{document}\n")
            cases.append(("transparent-images", "images", source))
        for case, suffix, source in cases:
            candidate = bundle.ROOT / f"target/runtime-complete-paper-{suffix}"
            env = {"PATH": "", "TEKAI_ENGINE_CACHE": str(work / "cache"),
                   "TEKAI_TEXMF_MODE": "bundled"}
            command([args.engine.resolve(), "build", source / "main.tex",
                     "--out-dir", candidate, "--force", "--quiet"], env=env)
            tree = next((work / "cache").glob("texmf-*/texmf-dist"))
            if tree.parent.name != "texmf-" + report["bundle"] or not (work / "cache" / f"pdflatex-{format_id}.fmt.raw").is_file():
                raise ValueError("Rebuild the candidate; it does not embed the current bundle and format")
            reference = work / case
            reference.mkdir()
            for path in candidate.iterdir():
                if path.is_file() and path.suffix not in {".pdf", ".log", ".fls"}:
                    shutil.copy2(path, reference / path.name)
            env = {
                "PATH": "", "TEXMFCNF": str(work), "TEXMF": str(tree),
                "TEXINPUTS": f"{source}//:{reference}//:{tree}/tex//",
                "TEXFORMATS": str(bundle.ROOT / "formats"),
                "TFMFONTS": f"{tree}/fonts/tfm//", "VFFONTS": f"{tree}/fonts/vf//",
                "T1FONTS": f"{tree}/fonts/type1//", "TTFONTS": f"{tree}/fonts/truetype//",
                "ENCFONTS": f"{tree}/fonts/enc//", "TEXFONTMAPS": f"{tree}/fonts/map//",
                "PKFONTS": f"{tree}/fonts/pk//", "MKTEXPK": "0", "MKTEXTFM": "0", "MKTEXFMT": "0",
            }
            command([engine, "-fmt=pdflatex", "-no-shell-escape", "-interaction=nonstopmode",
                     "-halt-on-error", f"-output-directory={reference}", "main.tex"], cwd=source, env=env)
            left, right = candidate / "main.pdf", reference / "main.pdf"
            info = command([poppler["pdfinfo"], left]).decode()
            text_matches = command([poppler["pdftotext"], left, "-"]) == command([poppler["pdftotext"], right, "-"])
            with tempfile.TemporaryDirectory(prefix="runtime-parity-", dir=renders) as rendered:
                rendered = Path(rendered)
                with concurrent.futures.ThreadPoolExecutor(max_workers=2) as pool:
                    list(pool.map(lambda item: command([poppler["pdftoppm"], "-r", "144", item[0], rendered / item[1]]),
                                  [(left, "candidate"), (right, "reference")]))
                pages = sorted(rendered.glob("candidate-*.ppm"))
                references = sorted(rendered.glob("reference-*.ppm"))
                changed = [index + 1 for index, (a, b) in enumerate(zip(pages, references)) if a.read_bytes() != b.read_bytes()]
                result = {"case": case, "pages": len(pages), "reference_pages": len(references),
                          "changed_pages": changed, "text_matches": text_matches, "pdfinfo": info,
                          "pdffonts": command([poppler["pdffonts"], left]).decode()}
            report["cases"].append(result)
            (target / "report.json").write_text(json.dumps(report, indent=2) + "\n")
            print(json.dumps({key: value for key, value in result.items() if key not in {"pdfinfo", "pdffonts"}}), flush=True)
            if changed or not text_matches or len(pages) != len(references):
                raise SystemExit("PDF parity failed; see target/runtime-parity/report.json")


if __name__ == "__main__":
    main()

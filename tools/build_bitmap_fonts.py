#!/usr/bin/env python3
"""Prebuild BBM bitmap fonts for the runtime. Never invoked by Tekai or Cargo.

This macOS maintainer tool downloads checksum-pinned, small TeX Live tools
into a temporary directory. It does not install TeX or change PATH. The shipped
runtime contains only their font output, not the tools or a runtime dependency.
"""

import argparse
import concurrent.futures
import hashlib
import io
import json
import lzma
from pathlib import Path, PurePosixPath
import platform
import re
import subprocess
import tarfile
import tempfile

import bundle_tex_runtime as bundle

LOCK = bundle.DEST / "font-build.lock.json"
TOOL_PACKAGES = {
    "metafont": [],
    "modes": [],
    "metafont.universal-darwin": ["mf-nowin"],
    "mfware.universal-darwin": ["gftopk"],
}
# Standard LaTeX sizes plus magsteps. Other bitmap resolutions are deliberately
# unsupported instead of launching Metafont on the user's machine.
RESOLUTIONS = [300, 360, 432, 480, 500, 600, 657, 720, 864, 1037, 1244, 1493]


def refresh_lock():
    database = bundle.download(bundle.MIRROR + "/tlpkg/texlive.tlpdb.xz")
    specs = []
    for block in lzma.decompress(database).decode().split("\n\n"):
        fields = dict(line.split(" ", 1) for line in block.splitlines() if " " in line)
        name = fields.get("name")
        if name in TOOL_PACKAGES:
            specs.append({
                "name": name, "revision": fields["revision"],
                "url": f"{bundle.MIRROR}/archive/{name}.tar.xz",
                "sha512": fields["containerchecksum"], "kind": "run",
                "programs": TOOL_PACKAGES[name],
            })
    if len(specs) != len(TOOL_PACKAGES):
        raise ValueError("Missing font-generation tool packages")
    LOCK.write_text(json.dumps({"packages": specs, "resolutions": RESOLUTIONS}, indent=2) + "\n")


def run(command, cwd, env):
    result = subprocess.run(command, cwd=cwd, env=env, capture_output=True)
    if result.returncode:
        raise RuntimeError(result.stdout.decode(errors="replace") + result.stderr.decode(errors="replace"))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--refresh-lock", action="store_true")
    args = parser.parse_args()
    if platform.system() != "Darwin":
        raise SystemExit("The pinned font-generation tools currently target macOS")
    if args.refresh_lock:
        refresh_lock()
    lock = json.loads(LOCK.read_text())
    cache = bundle.ROOT / "target/runtime-downloads"
    cache.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="tekai-fonts-") as temporary:
        work = Path(temporary)
        tree = work / "texmf"
        programs = work / "bin"
        programs.mkdir()
        sources = {}
        for spec in lock["packages"]:
            if not spec["programs"]:
                sources.update(bundle.archive_files(spec, cache))
                continue
            cached = cache / (spec["sha512"] + ".tar.xz")
            data = cached.read_bytes() if cached.exists() else bundle.download(spec["url"])
            if hashlib.sha512(data).hexdigest() != spec["sha512"]:
                raise ValueError(f"Checksum mismatch for {spec['name']}")
            cached.write_bytes(data)
            with tarfile.open(fileobj=io.BytesIO(data), mode="r:xz") as archive:
                for entry in archive:
                    name = PurePosixPath(entry.name).name
                    if entry.isfile() and name in spec["programs"]:
                        path = programs / name
                        path.write_bytes(archive.extractfile(entry).read())
                        path.chmod(0o755)
        packages = json.loads((bundle.DEST / "packages.lock.json").read_text())["packages"]
        for package in packages:
            if package["name"] in {"cm", "bbm"}:
                for spec in package["archives"]:
                    if spec["kind"] == "run":
                        sources.update(bundle.archive_files(spec, cache))
        for name, data in sources.items():
            path = tree / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        (work / "texmf.cnf").write_text("\n")
        env = {
            "PATH": "", "TEXMFCNF": str(work), "MFBASES": str(work),
            "MFINPUTS": str(tree) + "//", "SOURCE_DATE_EPOCH": "0",
        }
        run([str(programs / "mf-nowin"), "-ini", "-interaction=nonstopmode",
             r"\input plain; input modes; dump"], work, env)
        metrics = {PurePosixPath(name).stem: data for name, data in sources.items()
                   if "/tfm/public/bbm/" in name and re.fullmatch(r"bbm\d+\.tfm", PurePosixPath(name).name)}

        def generate(job):
            name, dpi = job
            directory = work / f"{name}-{dpi}"
            directory.mkdir()
            run([str(programs / "mf-nowin"), "-interaction=nonstopmode", "&plain",
                 rf"\mode:=ljfour; mag:={dpi}/600; nonstopmode; input {name}"], directory, env)
            if (directory / f"{name}.tfm").read_bytes() != metrics[name]:
                raise ValueError(f"Generated metrics differ for {name}")
            glyph = directory / f"{name}.{dpi}gf"
            packed = directory / f"{name}.{dpi}pk"
            run([str(programs / "gftopk"), str(glyph), str(packed)], directory, env)
            return f"texmf-dist/fonts/pk/ljfour/public/bbm/{name}.{dpi}pk", packed.read_bytes()

        jobs = [(name, dpi) for name in sorted(metrics) for dpi in lock["resolutions"]]
        with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
            files = dict(pool.map(generate, jobs))
        files["font-build.lock.json"] = LOCK.read_bytes()
        data = bundle.write_archive(bundle.DEST / "bitmap-fonts.tar.gz", files)
        print(f"Generated {len(jobs)} bitmap fonts in {len(data):,} bytes")


if __name__ == "__main__":
    main()

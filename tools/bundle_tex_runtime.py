#!/usr/bin/env python3
"""Build the checked-in TeX data bundle. Never run by Cargo or the CLI.

Use --refresh-lock to select a new TeX Live snapshot. Normal runs verify every
download against runtime/packages.lock.json. No TeX executable is used.
"""

import argparse
import concurrent.futures
import gzip
import hashlib
import io
import json
import lzma
from pathlib import Path, PurePosixPath
import tarfile
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
DEST = ROOT / "runtime"
MIRROR = "https://ctan.math.illinois.edu/systems/texlive/tlnet"
PACKAGES = """
latex firstaid latex-fonts latexconfig cm amsfonts amsmath amscls graphics graphics-cfg
graphics-def tools l3kernel l3packages pdftex pdftexcmds
hyphen-base hyph-utf8 unicode-data tex-ini-files
hyperref hycolor pdfescape stringenc etoolbox kvoptions kvsetkeys kvdefinekeys ltxcmds infwarerr iftex
ifplatform catchfile intcalc bigintcalc bitset rerunfilecheck
gettitlestring auxhook atbegshi atveryend refcount uniquecounter letltxmacro
epstopdf-pkg grfext xcolor geometry booktabs caption float fancyhdr eso-pic
microtype natbib url listings mathtools enumitem wrapfig placeins pgf tcolorbox
environ trimspaces pgfopts xkeyval xpatch currfile capt-of titlesec siunitx
translations lm newpx newtx pxfonts txfonts tex-gyre kastrup fontaxes xstring
inconsolata lineno ulem upquote cmbright ec bibtex mylatexformat pdfcol fancyvrb
tikzfill subfigure appendix minitoc makecell dblfloatfix cleveref sttools
bbm bbm-macros varwidth algorithmicx mdframed multirow setspace colortbl framed
psnfss times helvetic courier cm-super
carlisle mweights
centernot listingsutf8 zref needspace
dvips etexcmds
""".split()


def download(url):
    with urllib.request.urlopen(url, timeout=90) as response:
        return response.read()


def refresh_lock():
    database = download(MIRROR + "/tlpkg/texlive.tlpdb.xz")
    blocks = {}
    for block in lzma.decompress(database).decode().split("\n\n"):
        lines = block.splitlines()
        if lines and lines[0].startswith("name "):
            blocks[lines[0][5:]] = lines
    # Some CTAN packages are grouped differently in TeX Live, such as
    # centernot in oberdiek. Resolve those names from the pinned database's
    # runtime file list instead of guessing a container name.
    owners = {}
    for name, lines in blocks.items():
        for line in lines:
            if line.startswith((" RELOC/tex/", " texmf-dist/tex/")) and line.endswith(".sty"):
                owners.setdefault(PurePosixPath(line.strip()).stem, set()).add(name)
    selected = set()
    for requested in PACKAGES:
        matches = {requested} if requested in blocks else owners.get(requested, set())
        if len(matches) != 1:
            raise ValueError(f"Cannot uniquely resolve TeX Live package {requested}: {matches}")
        selected.update(matches)
    packages = []
    for name in sorted(selected):
        fields = {}
        for line in blocks[name]:
            key, _, value = line.partition(" ")
            if key:
                fields.setdefault(key, []).append(value)
        archives = []
        for prefix, suffix in [("", ""), ("doc", ".doc"), ("src", ".source")]:
            checksum = fields.get(prefix + "containerchecksum")
            if checksum:
                archives.append({
                    "url": f"{MIRROR}/archive/{name}{suffix}.tar.xz",
                    "sha512": checksum[0],
                    "kind": prefix or "run",
                })
        packages.append({
            "name": name,
            "revision": fields["revision"][0],
            "license": fields.get("catalogue-license", ["See upstream files"])[0],
            "ctan": fields.get("catalogue-ctan", [""])[0],
            "maps": [v.split()[-1] for v in fields.get("execute", [])
                     if v.startswith(("addMap ", "addMixedMap "))],
            "archives": archives,
        })
    DEST.mkdir(exist_ok=True)
    (DEST / "packages.lock.json").write_text(json.dumps({
        "repository": MIRROR,
        "database_sha512": hashlib.sha512(database).hexdigest(),
        "packages": packages,
    }, indent=2) + "\n")


def archive_files(spec, cache):
    cached = cache / (spec["sha512"] + ".tar.xz")
    data = cached.read_bytes() if cached.exists() else download(spec["url"])
    if hashlib.sha512(data).hexdigest() != spec["sha512"]:
        raise ValueError(f"Checksum mismatch for {spec['url']}; do not silently refresh the lock")
    cached.write_bytes(data)
    files = {}
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:xz") as archive:
        for entry in archive:
            if not entry.isfile():
                continue
            path = PurePosixPath(entry.name)
            if path.is_absolute() or ".." in path.parts:
                raise ValueError(f"Unsafe upstream path: {path}")
            name = str(path)
            if name.startswith("RELOC/"):
                name = "texmf-dist/" + name[6:]
            elif path.parts[0] in {"tex", "fonts", "bibtex", "makeindex", "doc", "source", "web2c"}:
                name = "texmf-dist/" + name
            if not name.startswith("texmf-dist/"):
                continue
            # Source and textual documentation retain upstream licenses and
            # corresponding sources. PDF manuals aren't needed by the engine.
            if spec["kind"] == "doc" and path.suffix.lower() in {
                ".pdf", ".png", ".jpg", ".jpeg", ".eps", ".ps", ".dvi"
            }:
                continue
            files[name] = archive.extractfile(entry).read()
    return files


def write_archive(destination, files):
    with io.BytesIO() as buffer:
        with tarfile.open(fileobj=buffer, mode="w", format=tarfile.USTAR_FORMAT) as archive:
            for name, content in sorted(files.items()):
                entry = tarfile.TarInfo(name)
                entry.size = len(content)
                entry.mode = 0o644
                archive.addfile(entry, io.BytesIO(content))
        with io.BytesIO() as compressed:
            with gzip.GzipFile(fileobj=compressed, mode="wb", compresslevel=9, mtime=0) as stream:
                stream.write(buffer.getvalue())
            data = compressed.getvalue()
    destination.write_bytes(data)
    return data


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--refresh-lock", action="store_true")
    parser.add_argument("--cache", type=Path, default=ROOT / "target/runtime-downloads")
    args = parser.parse_args()
    if args.refresh_lock:
        refresh_lock()
    lock = json.loads((DEST / "packages.lock.json").read_text())
    args.cache.mkdir(parents=True, exist_ok=True)
    specs = [a for p in lock["packages"] for a in p["archives"]]
    files = {}
    outline_names = set()
    with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
        for spec, result in zip(specs, pool.map(lambda a: archive_files(a, args.cache), specs)):
            for name, content in result.items():
                if name in files and files[name] != content:
                    raise ValueError(f"Conflicting upstream file: {name}")
                files[name] = content
                if "/cm-super" in name:
                    outline_names.add(name)
            print(spec["url"].rsplit("/", 1)[-1], flush=True)
    with tarfile.open(DEST / "bitmap-fonts.tar.gz", "r:gz") as archive:
        for entry in archive:
            path = PurePosixPath(entry.name)
            if not entry.isfile() or path.is_absolute() or ".." in path.parts:
                raise ValueError(f"Unsafe generated font path: {path}")
            if entry.name.startswith("texmf-dist/"):
                files[entry.name] = archive.extractfile(entry).read()
    # Generate the combined font map directly, without updmap or mktexlsr.
    by_name = {PurePosixPath(p).name: data for p, data in files.items()}
    maps = sorted({m for p in lock["packages"] for m in p["maps"]})
    combined = b"\n".join(by_name[m] for m in maps)
    # PSNFSS uses Adobe metric names while URW distributes equivalent outlines
    # under `u...` names. Match updmap's embedded Base35 mapping, without running it.
    keys = {line.split()[0] for line in combined.splitlines() if line.split() and not line.lstrip().startswith(b"%")}
    aliases = []
    for line in combined.splitlines():
        if not line.startswith(b"u"):
            continue
        key = line.split()[0]
        alias = b"p" + key[1:]
        if alias not in keys and alias.decode() + ".tfm" in by_name:
            aliases.append(alias + line[len(key):])
            keys.add(alias)
    files["texmf-dist/fonts/map/pdftex/tekai/pdftex.map"] = combined + b"\n" + b"\n".join(aliases) + b"\n"
    directories = {}
    for name in files:
        path = PurePosixPath(name).relative_to("texmf-dist")
        if path.parts[0] not in {"doc", "source"}:
            directories.setdefault(str(path.parent), []).append(path.name)
    files["texmf-dist/ls-R"] = "\n".join(
        f"./{directory}:\n" + "\n".join(sorted(names)) + "\n"
        for directory, names in sorted(directories.items())
    ).encode()
    files["packages.lock.json"] = (DEST / "packages.lock.json").read_bytes()
    files["README.Tekai"] = (DEST / "README.md").read_bytes()
    files["LICENSE.TeX-Live"] = (DEST / "LICENSE.TeX-Live").read_bytes()
    files["font-build.lock.json"] = (DEST / "font-build.lock.json").read_bytes()
    # Keep each checked-in archive below hosting file-size limits. Both are
    # embedded in the executable and installed together, not downloaded at run time.
    core = {name: data for name, data in files.items() if name not in outline_names}
    outlines = {name: data for name, data in files.items() if name in outline_names}
    data = write_archive(DEST / "texmf.tar.gz", core)
    fonts = write_archive(DEST / "font-outlines.tar.gz", outlines)
    (DEST / "bundle-id.txt").write_text(hashlib.sha256(data + fonts).hexdigest() + "\n")
    print(f"Bundled {len(files)} files in {len(data) + len(fonts):,} bytes")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Build, regenerate, and verify the generated Rust pdfTeX port."""

from __future__ import annotations

import argparse
import filecmp
import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TEXLIVE_SOURCE = ROOT / "third_party" / "texlive-source"
PORT_ROOT = ROOT / "target" / "pdftex-port"
BUILD_DIR = PORT_ROOT / "texlive-build"
WEB2C_DIR = BUILD_DIR / "texk" / "web2c"
CRATE_DIR = ROOT / "crates" / "pdftex-rust"
GENERATED_DIR = CRATE_DIR / "src" / "generated"
GENERATED_BACKEND_DIR = GENERATED_DIR / "backend"
RUST_ARCHIVE = ROOT / "target" / "release" / "libpdftex_rust.a"
RUST_BINARY = WEB2C_DIR / "pdftex-rust-full"
SOURCE_DATE_EPOCH = "1783191600"
PDFTEX_BACKEND_C_SOURCES = [
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "avl.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "avlstuff.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "epdf.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "mapfile.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "pkin.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "subfont.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "tounicode.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "vfpacket.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "writeenc.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "writefont.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "writeimg.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "writejbig2.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "writejpg.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "writepng.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "writet1.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "writet3.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "writettf.c",
    TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "writezip.c",
]


SYNCTEX_PROTOTYPES = """\
extern void synctexinitcommand(void);
extern void synctexabort(boolean log_opened);
extern void synctexstartinput(void);
extern void synctexterminate(boolean log_opened);
extern void synctexsheet(integer mag);
extern void synctexteehs(void);
extern void synctexpdfxform(halfword p);
extern void synctexmrofxfdp(void);
extern void synctexpdfrefxform(int objnum);
extern void synctexvlist(halfword this_box);
extern void synctextsilv(halfword this_box);
extern void synctexvoidvlist(halfword p, halfword this_box);
extern void synctexhlist(halfword this_box);
extern void synctextsilh(halfword this_box);
extern void synctexvoidhlist(halfword p, halfword this_box);
extern void synctexmath(halfword p, halfword this_box);
extern void synctexhorizontalruleorglue(halfword p, halfword this_box);
extern void synctexkern(halfword p, halfword this_box);
extern void synctexchar(halfword p, halfword this_box);
extern void synctexnode(halfword p, halfword this_box);
extern void synctexcurrent(void);
"""


SMOKE_TEX = """\
\\catcode`\\{=1
\\catcode`\\}=2
\\pdfoutput=1
\\shipout\\vbox{\\hbox{Hi}}
\\end
"""


def run(
    cmd: list[str],
    *,
    cwd: Path = ROOT,
    env: dict[str, str] | None = None,
    capture: bool = False,
) -> subprocess.CompletedProcess[str]:
    print("+", " ".join(cmd), flush=True)
    merged_env = os.environ.copy()
    if env:
        merged_env.update(env)
    return subprocess.run(
        cmd,
        cwd=cwd,
        env=merged_env,
        check=True,
        text=True,
        stdout=subprocess.PIPE if capture else None,
        stderr=subprocess.STDOUT if capture else None,
    )


def require_texlive_source() -> None:
    if not (TEXLIVE_SOURCE / "configure").exists():
        raise SystemExit(
            "missing third_party/texlive-source; run "
            "`git submodule update --init --recursive`"
        )


def ensure_texlive_build(force: bool = False) -> None:
    require_texlive_source()
    if force and BUILD_DIR.exists():
        shutil.rmtree(BUILD_DIR)
    if not (WEB2C_DIR / "pdftex").exists():
        BUILD_DIR.mkdir(parents=True, exist_ok=True)
        configure = [
            str(TEXLIVE_SOURCE / "configure"),
            "--without-x",
            "--disable-shared",
            "--disable-all-pkgs",
            "--enable-pdftex",
            "--enable-missing",
            "-C",
            "CFLAGS=-g -O0",
            "CXXFLAGS=-g -O0",
        ]
        run(configure, cwd=BUILD_DIR)
        run(["make", f"-j{os.cpu_count() or 1}"], cwd=BUILD_DIR)
    else:
        print(f"using existing TeX Live build: {WEB2C_DIR}")


def ensure_synctex_prototypes() -> None:
    header = WEB2C_DIR / "pdftexd.h"
    text = header.read_text()
    if "extern void synctexabort(boolean log_opened);" in text:
        return
    marker = '#include "pdftexcoerce.h"'
    if marker not in text:
        raise SystemExit(f"cannot place SyncTeX prototypes in {header}")
    header.write_text(text.replace(marker, f"\n{SYNCTEX_PROTOTYPES}\n{marker}", 1))


def c2rust_include_args() -> list[str]:
    source_web2c = TEXLIVE_SOURCE / "texk" / "web2c"
    return [
        "-std=gnu89",
        "-DHAVE_CONFIG_H",
        "-DNO_DEBUG",
        "-I.",
        "-I..",
        "-I./w2c",
        "-I./pdftexdir",
        "-I./libmd5",
        f"-I{source_web2c}",
        f"-I{source_web2c / 'lib'}",
        f"-I{source_web2c / 'libmd5'}",
        f"-I{source_web2c / 'pdftexdir'}",
        f"-I{source_web2c / 'synctexdir'}",
        f"-I{TEXLIVE_SOURCE / 'texk'}",
        "-I../..",
        "-I../../libs/zlib",
        f"-I{TEXLIVE_SOURCE / 'libs' / 'zlib' / 'zlib-src'}",
        "-I../../libs/libpng",
        f"-I{TEXLIVE_SOURCE / 'libs' / 'libpng' / 'libpng-src'}",
        "-I../../libs/xpdf",
        f"-I{TEXLIVE_SOURCE / 'libs' / 'xpdf' / 'xpdf-src' / 'goo'}",
        f"-I{TEXLIVE_SOURCE / 'libs' / 'xpdf' / 'xpdf-src' / 'fofi'}",
        f"-I{TEXLIVE_SOURCE / 'libs' / 'xpdf' / 'xpdf-src' / 'xpdf'}",
        "-I../kpathsea",
    ]


def normalize_generated_rust(path: Path) -> None:
    text = path.read_text()
    text = text.replace(
        'extern "C" {\n    pub type __sFILEX;',
        '#[repr(C)]\n'
        'pub struct __sFILEX {\n'
        '    _unused: [u8; 0],\n'
        '}\n\n'
        'extern "C" {',
    )
    for name in ("png_struct_def", "png_info_def", "internal_state", "re_guts"):
        declaration = f"    pub type {name};\n"
        if declaration in text:
            text = text.replace(declaration, "")
            struct_decl = (
                "#[repr(C)]\n"
                f"pub struct {name} {{\n"
                "    _unused: [u8; 0],\n"
                "}\n\n"
            )
            if struct_decl not in text:
                text = text.replace('extern "C" {', f"{struct_decl}extern \"C\" {{", 1)
    text = re.sub(r"\bgetc\(", "fgetc(", text)
    text = re.sub(
        r'#\[no_mangle\]\n#\[inline\]\n#\[linkage = "external"\]\n'
        r"pub unsafe extern \"C\" fn ([A-Za-z_][A-Za-z0-9_]*)",
        r"unsafe fn \1",
        text,
    )
    if path.name == "writet1.rs":
        text = normalize_writet1_rust(text)
    text = rewrite_pdftex_vararg_calls(text)
    path.write_text(text)


def normalize_writet1_rust(text: str) -> str:
    """Replace local C varargs helpers with fixed Rust-callable forms."""
    text = text.replace(
        "    fn vsprintf(\n"
        "        _: *mut ::core::ffi::c_char,\n"
        "        _: *const ::core::ffi::c_char,\n"
        "        _: ::core::ffi::VaList,\n"
        "    ) -> ::core::ffi::c_int;\n",
        "",
    )
    text = re.sub(
        r'unsafe extern "C" fn t1_printf\([\s\S]*?\n\}',
        'unsafe extern "C" fn t1_printf(\n'
        "    mut fmt: *const ::core::ffi::c_char,\n"
        "    mut code: ::core::ffi::c_int,\n"
        "    mut glyph: *mut ::core::ffi::c_char,\n"
        ") {\n"
        "    sprintf(t1_line_array as *mut ::core::ffi::c_char, fmt, code, glyph);\n"
        "    t1_puts(t1_line_array);\n"
        "}",
        text,
        count=1,
    )
    start = text.index('unsafe extern "C" fn cs_fail(')
    end = text.index('\nunsafe extern "C" fn append_cs_return', start)
    fixed_cs_fail = (
        'unsafe extern "C" fn cs_fail(\n'
        "    mut cs_name: *const ::core::ffi::c_char,\n"
        "    mut subr: ::core::ffi::c_int,\n"
        "    mut fmt: *const ::core::ffi::c_char,\n"
        "    mut arg1: ::core::ffi::c_int,\n"
        "    mut arg2: ::core::ffi::c_int,\n"
        ") {\n"
        "    let mut buf: [::core::ffi::c_char; 256] = [0; 256];\n"
        "    sprintf(&raw mut buf as *mut ::core::ffi::c_char, fmt, arg1, arg2);\n"
        "    if cs_name.is_null() {\n"
        "        pdftex_fail(\n"
        '            b"Subr (%i): %s\\0" as *const u8 as *const ::core::ffi::c_char,\n'
        "            subr,\n"
        "            &raw mut buf as *mut ::core::ffi::c_char,\n"
        "        );\n"
        "    } else {\n"
        "        pdftex_fail(\n"
        '            b"CharString (/%s): %s\\0" as *const u8 as *const ::core::ffi::c_char,\n'
        "            cs_name,\n"
        "            &raw mut buf as *mut ::core::ffi::c_char,\n"
        "        );\n"
        "    };\n"
        "}"
    )
    text = f"{text[:start]}{fixed_cs_fail}{text[end:]}"
    text = text.replace(
        "b\"command value out of range: %i\\0\" as *const u8 as *const ::core::ffi::c_char,\n"
        "                    b,\n"
        "                );",
        "b\"command value out of range: %i\\0\" as *const u8 as *const ::core::ffi::c_char,\n"
        "                    b,\n"
        "                    0 as ::core::ffi::c_int,\n"
        "                );",
    )
    text = text.replace(
        "b\"command not valid: %i\\0\" as *const u8 as *const ::core::ffi::c_char,\n"
        "                        b,\n"
        "                    );",
        "b\"command not valid: %i\\0\" as *const u8 as *const ::core::ffi::c_char,\n"
        "                        b,\n"
        "                        0 as ::core::ffi::c_int,\n"
        "                    );",
    )
    text = text.replace(
        "b\"cannot call subr (%i)\\0\" as *const u8\n"
        "                                        as *const ::core::ffi::c_char,\n"
        "                                    a1,\n"
        "                                );",
        "b\"cannot call subr (%i)\\0\" as *const u8\n"
        "                                        as *const ::core::ffi::c_char,\n"
        "                                    a1,\n"
        "                                    0 as ::core::ffi::c_int,\n"
        "                                );",
    )
    return text


VARARG_REWRITES = {
    "pdf_printf": "pdf_printf_args",
    "tex_printf": "tex_printf_args",
    "pdftex_warn": "pdftex_warn_args",
    "pdftex_fail": "pdftex_fail_args",
}


def rewrite_pdftex_vararg_calls(text: str) -> str:
    out: list[str] = []
    i = 0
    while i < len(text):
        match = None
        for name in VARARG_REWRITES:
            if (
                text.startswith(name, i)
                and _is_word_boundary(text, i - 1)
                and _is_word_boundary(text, i + len(name))
            ):
                match = name
                break
        if match is None:
            out.append(text[i])
            i += 1
            continue
        prefix = text[max(0, i - 4):i]
        if prefix.endswith("fn "):
            out.append(text[i])
            i += 1
            continue
        j = i + len(match)
        while j < len(text) and text[j].isspace():
            j += 1
        if j >= len(text) or text[j] != "(":
            out.append(text[i])
            i += 1
            continue
        end = _find_matching_paren(text, j)
        if end is None:
            out.append(text[i])
            i += 1
            continue
        args = _split_top_level_args(text[j + 1:end])
        if not args:
            out.append(text[i:end + 1])
        else:
            fmt = args[0].strip()
            wrapped_args = [
                f"crate::utils::PrintfArg::from({arg.strip()})" for arg in args[1:]
            ]
            if wrapped_args:
                arg_expr = "&[" + ", ".join(wrapped_args) + "]"
            else:
                arg_expr = "&[]"
            out.append(f"crate::utils::{VARARG_REWRITES[match]}({fmt}, {arg_expr})")
        i = end + 1
    return "".join(out)


def _is_word_boundary(text: str, index: int) -> bool:
    if index < 0 or index >= len(text):
        return True
    return not (text[index].isalnum() or text[index] == "_")


def _find_matching_paren(text: str, start: int) -> int | None:
    depth = 0
    i = start
    while i < len(text):
        ch = text[i]
        if ch == '"':
            i = _skip_string(text, i)
            continue
        if ch == "'":
            i = _skip_char(text, i)
            continue
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return None


def _split_top_level_args(source: str) -> list[str]:
    args: list[str] = []
    depth = 0
    start = 0
    i = 0
    while i < len(source):
        ch = source[i]
        if ch == '"':
            i = _skip_string(source, i)
            continue
        if ch == "'":
            i = _skip_char(source, i)
            continue
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        elif ch == "," and depth == 0:
            args.append(source[start:i])
            start = i + 1
        i += 1
    tail = source[start:].strip()
    if tail:
        args.append(source[start:])
    return args


def _skip_string(text: str, start: int) -> int:
    i = start + 1
    while i < len(text):
        if text[i] == "\\":
            i += 2
            continue
        if text[i] == '"':
            return i + 1
        i += 1
    return i


def _skip_char(text: str, start: int) -> int:
    i = start + 1
    while i < len(text):
        if text[i] == "\\":
            i += 2
            continue
        if text[i] == "'":
            return i + 1
        i += 1
    return i


def transpile(write_crate: bool) -> None:
    ensure_texlive_build()
    ensure_synctex_prototypes()
    output_dir = PORT_ROOT / "rust-seed"
    shutil.rmtree(output_dir, ignore_errors=True)
    cmd = [
        "c2rust",
        "transpile",
        "--overwrite-existing",
        "--emit-modules",
        "--output-dir",
        str(output_dir),
        str(TEXLIVE_SOURCE / "texk" / "web2c" / "pdftexdir" / "pdftexextra.c"),
        *(str(path) for path in PDFTEX_BACKEND_C_SOURCES),
        "pdftexini.c",
        "pdftex0.c",
        "pdftex-pool.c",
        "--",
        *c2rust_include_args(),
    ]
    run(cmd, cwd=WEB2C_DIR)
    emitted: dict[str, Path] = {}
    for path in (output_dir / "src").rglob("*.rs"):
        normalize_generated_rust(path)
        emitted[path.name] = path
    if write_crate:
        GENERATED_DIR.mkdir(parents=True, exist_ok=True)
        for name in ("pdftexextra.rs", "pdftexini.rs", "pdftex0.rs", "pdftex_pool.rs"):
            shutil.copy2(emitted[name], GENERATED_DIR / name)
        GENERATED_BACKEND_DIR.mkdir(parents=True, exist_ok=True)
        for name in (
            "avl.rs",
            "avlstuff.rs",
            "epdf.rs",
            "mapfile.rs",
            "pkin.rs",
            "subfont.rs",
            "tounicode.rs",
            "vfpacket.rs",
            "writeenc.rs",
            "writefont.rs",
            "writeimg.rs",
            "writejbig2.rs",
            "writejpg.rs",
            "writepng.rs",
            "writet1.rs",
            "writet3.rs",
            "writettf.rs",
            "writezip.rs",
        ):
            shutil.copy2(emitted[name], GENERATED_BACKEND_DIR / name)
    print(
        json.dumps(
            {
                "generated": str(output_dir / "src"),
                "wrote_crate": write_crate,
            },
            indent=2,
        )
    )


def build_rust_archive() -> None:
    run(["cargo", "build", "--release", "-p", "pdftex-rust", "--lib"], cwd=ROOT)
    if not RUST_ARCHIVE.exists():
        raise SystemExit(f"missing Rust archive: {RUST_ARCHIVE}")


def link_rust_pdftex(force: bool = False) -> None:
    ensure_texlive_build()
    build_rust_archive()
    if force and RUST_BINARY.exists():
        RUST_BINARY.unlink()
    link_cmd = [
        "/bin/sh",
        "./libtool",
        "--tag=CXX",
        "--mode=link",
        "g++",
        "-Wreturn-type",
        "-Wno-write-strings",
        "-g",
        "-O0",
        "-o",
        RUST_BINARY.name,
        str(RUST_ARCHIVE),
        str(BUILD_DIR / "libs" / "libpng" / "libpng.a"),
        str(BUILD_DIR / "libs" / "zlib" / "libz.a"),
        str(BUILD_DIR / "libs" / "xpdf" / "libxpdf.a"),
        str(BUILD_DIR / "texk" / "kpathsea" / "libkpathsea.la"),
    ]
    run(link_cmd, cwd=WEB2C_DIR)


def run_initex_smoke(binary: Path, out_dir: Path, extra_args: list[str] | None = None) -> None:
    shutil.rmtree(out_dir, ignore_errors=True)
    out_dir.mkdir(parents=True)
    (out_dir / "test.tex").write_text(SMOKE_TEX)
    env = {
        "SOURCE_DATE_EPOCH": SOURCE_DATE_EPOCH,
        "FORCE_SOURCE_DATE": "1",
        "TEXINPUTS": ".:",
    }
    run(
        [
            str(binary),
            "-ini",
            *(extra_args or []),
            "-interaction=nonstopmode",
            "-jobname=test",
            "./test.tex",
        ],
        cwd=out_dir,
        env=env,
        capture=True,
    ).stdout


def smoke(force_link: bool = False) -> None:
    link_rust_pdftex(force=force_link)
    c_dir = PORT_ROOT / "smoke" / "c"
    rust_dir = PORT_ROOT / "smoke" / "rust"
    run_initex_smoke(WEB2C_DIR / "pdftex", c_dir)
    run_initex_smoke(RUST_BINARY, rust_dir)
    comparisons = {
        "pdf": filecmp.cmp(c_dir / "test.pdf", rust_dir / "test.pdf", shallow=False),
        "log": filecmp.cmp(c_dir / "test.log", rust_dir / "test.log", shallow=False),
    }
    if shutil.which("pdftoppm"):
        for name, directory in (("c", c_dir), ("rust", rust_dir)):
            render_dir = directory / "render"
            render_dir.mkdir()
            run(
                [
                    "pdftoppm",
                    "-r",
                    "144",
                    "-png",
                    str(directory / "test.pdf"),
                    str(render_dir / "page"),
                ]
            )
        comparisons["pixels"] = filecmp.cmp(
            c_dir / "render" / "page-1.png",
            rust_dir / "render" / "page-1.png",
            shallow=False,
        )
    synctex_c_dir = PORT_ROOT / "smoke-synctex" / "c"
    synctex_rust_dir = PORT_ROOT / "smoke-synctex" / "rust"
    run_initex_smoke(WEB2C_DIR / "pdftex", synctex_c_dir, extra_args=["-synctex=1"])
    run_initex_smoke(RUST_BINARY, synctex_rust_dir, extra_args=["-synctex=1"])
    comparisons["synctex_pdf"] = filecmp.cmp(
        synctex_c_dir / "test.pdf",
        synctex_rust_dir / "test.pdf",
        shallow=False,
    )
    comparisons["rust_synctex_sidecar_omitted"] = not (
        synctex_rust_dir / "test.synctex.gz"
    ).exists()
    print(json.dumps(comparisons, indent=2))
    if not all(comparisons.values()):
        raise SystemExit("pdfTeX Rust smoke parity failed")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)

    build_parser = sub.add_parser("build-upstream", help="configure and build upstream pdfTeX")
    build_parser.add_argument("--force", action="store_true")

    transpile_parser = sub.add_parser("transpile", help="regenerate Rust from web2c C")
    transpile_parser.add_argument("--write-crate", action="store_true")

    link_parser = sub.add_parser("link", help="link pdftex-rust-full")
    link_parser.add_argument("--force", action="store_true")

    smoke_parser = sub.add_parser("smoke", help="link and byte-compare a deterministic fixture")
    smoke_parser.add_argument("--force-link", action="store_true")

    args = parser.parse_args()
    if args.command == "build-upstream":
        ensure_texlive_build(force=args.force)
    elif args.command == "transpile":
        transpile(write_crate=args.write_crate)
    elif args.command == "link":
        link_rust_pdftex(force=args.force)
    elif args.command == "smoke":
        smoke(force_link=args.force_link)
    else:
        raise AssertionError(args.command)


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as exc:
        if exc.stdout:
            sys.stderr.write(exc.stdout)
        raise SystemExit(exc.returncode)

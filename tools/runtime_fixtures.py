"""Generated ordinary TeX/PNG fixtures shared by runtime checks."""

import struct
import zlib


def png(path, number):
    def chunk(kind, payload):
        return struct.pack('>I', len(payload)) + kind + payload + struct.pack('>I', zlib.crc32(kind + payload))
    pixel = bytes([number * 7 % 256, number * 13 % 256, 90, 128])
    rows = (b'\0' + pixel * 1024) * 1024
    path.write_bytes(b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', 1024, 1024, 8, 6, 0, 0, 0))
                     + chunk(b'IDAT', zlib.compress(rows)) + chunk(b'IEND', b''))


def document(project, source=None):
    project.mkdir(parents=True, exist_ok=True)
    (project / 'main.tex').write_text(source or '\\documentclass{article}\n\\begin{document}Probe.\\end{document}\n', encoding='utf-8')


def pad(project, count):
    for n in range(count):
        path = project / f'unused-tree/bucket{n // 100}/group{n // 10}/leaf{n}'
        path.mkdir(parents=True, exist_ok=True)
        (path / 'unused.dat').touch()

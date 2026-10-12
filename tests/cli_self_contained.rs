use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

struct Project(PathBuf);

impl Project {
    fn new() -> Self {
        static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);
        for _ in 0..128 {
            let root = std::env::temp_dir().join(format!(
                "tekai-self-contained-{}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed),
            ));
            // Wall-clock resolution cannot establish exclusive fixture ownership.
            // Never reuse a stale directory, even after the OS recycles a PID.
            match fs::create_dir(&root) {
                Ok(()) => {
                    let project = Self(root);
                    fs::create_dir(project.0.join("empty-bin")).unwrap();
                    return project;
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("cannot create fixture {}: {error}", root.display()),
            }
        }
        panic!("cannot reserve a self-contained test fixture after 128 attempts")
    }

    fn write(&self, name: &str, source: &str) {
        self.write_bytes(name, source.as_bytes());
    }

    fn write_bytes(&self, name: &str, source: &[u8]) {
        let path = self.0.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, source).unwrap();
    }

    fn run(&self, args: &[&str]) -> Output {
        self.run_with_env(args, &[])
    }

    fn run_with_env(&self, args: &[&str], extra: &[(&str, &str)]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_tekai"))
            .current_dir(&self.0)
            .env_clear()
            .env("PATH", self.0.join("empty-bin"))
            .env("TEKAI_TEXMF_MODE", "bundled")
            .env("TEKAI_ENGINE_CACHE", self.0.join("cache/engine"))
            .env("TEKAI_FORMAT_CACHE", self.0.join("cache/formats"))
            .env("TEKAI_AUX_CACHE", self.0.join("cache/aux"))
            .env("TEKAI_BIBTEX_CACHE", self.0.join("cache/bibtex"))
            // Poison old system-data hints as well as executable discovery.
            .env("TEXMFDIST", self.0.join("missing-texlive"))
            .env("TEXMFVAR", self.0.join("missing-texlive"))
            .env("TEXMFCONFIG", self.0.join("missing-texlive"))
            .env("shell_escape", "t")
            .envs(extra.iter().copied())
            .args(args)
            .output()
            .unwrap()
    }

    fn success(&self, args: &[&str]) -> Value {
        let output = self.run(args);
        assert!(
            output.status.success(),
            "stdout: {}\nstderr: {}\nlog: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            fs::read_to_string(self.0.join("build/main.log")).unwrap_or_default()
        );
        serde_json::from_slice(&output.stdout).unwrap()
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn bundled_engine_width_capacity_failures_are_normal_diagnostics() {
    let project = Project::new();
    fs::create_dir(project.0.join("build")).unwrap();
    for (name, body) in [
        ("paragraph", format!("{}é more text", "x".repeat(8190))),
        ("hbox", format!("\\setbox0=\\hbox{{{}}}", "x".repeat(8190))),
        (
            "stretch",
            "\\setbox0=\\hbox{\\hskip0pt plus16000pt\\hskip0pt plus16000pt\\hskip0pt plus16000pt}"
                .to_owned(),
        ),
        (
            "negative-kerns",
            "\\setbox0=\\hbox{\\kern-16000pt\\kern-16000pt\\kern-16000pt}".to_owned(),
        ),
    ] {
        project.write(
            &format!("{name}.tex"),
            &format!(
                "\\documentclass{{article}}\n\\begin{{document}}\n{body}\n\\end{{document}}\n"
            ),
        );
        let source = format!("{name}.tex");
        let job = format!("-jobname={name}");
        let output = project.run(&[
            "__tekai-engine",
            "-interaction=nonstopmode",
            "-halt-on-error",
            "-output-directory=build",
            &job,
            &source,
        ]);
        // This invokes the engine child directly, so SIGABRT cannot be hidden
        // behind the outer build command's ordinary error exit status.
        assert_eq!(output.status.code(), Some(1), "{name}: {output:?}");
        let diagnostic = format!(
            "{}\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            fs::read_to_string(project.0.join(format!("build/{name}.log"))).unwrap_or_default()
        );
        assert!(
            diagnostic.contains("TeX capacity exceeded"),
            "{name}: {diagnostic}"
        );
        let normalized: String = diagnostic.split_ascii_whitespace().collect();
        assert!(
            normalized.contains("paragraphorboxwidthinscaledpoints=2147483647"),
            "{name}: {diagnostic}"
        );
        assert!(!diagnostic.contains("panicked"), "{name}: {diagnostic}");
        assert!(!diagnostic.contains("SIGABRT"), "{name}: {diagnostic}");
        assert!(!project.0.join(format!("build/{name}.pdf")).exists());
    }
}

#[test]
fn bundled_engine_long_breakable_unicode_paragraph_keeps_valid_layout() {
    let project = Project::new();
    // Preserve the former preview fixture's UTF-8 byte offset while making
    // every word legal and short enough for ordinary paragraph line breaking.
    project.write(
        "main.tex",
        &format!(
            "\\documentclass{{article}}\n\\begin{{document}}\n{}é more text\n\\end{{document}}\n",
            "x ".repeat(4095)
        ),
    );
    let report = project.success(&["build", "main.tex", "--once", "--report-json"]);
    assert_eq!(report["tex_runs"], 1);
    let pdf = fs::read(project.0.join("build/main.pdf")).unwrap();
    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.windows(5).any(|bytes| bytes == b"%%EOF"));
    let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    assert!(!log.contains("TeX capacity exceeded"), "{log}");
    assert!(!log.contains("Overfull \\hbox"), "{log}");
    assert!(log.contains("Output written on"), "{log}");
}

#[test]
fn bundled_engine_embeds_owned_truetype_glyphs_with_reserved_composite_mapping() {
    fn put16(bytes: &mut [u8], offset: usize, value: u16) {
        bytes[offset..offset + 2].copy_from_slice(&value.to_be_bytes());
    }
    fn put32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }
    fn read16(bytes: &[u8], offset: usize) -> u16 {
        u16::from_be_bytes(bytes[offset..offset + 2].try_into().unwrap())
    }
    fn read32(bytes: &[u8], offset: usize) -> u32 {
        u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap())
    }
    fn checksum(bytes: &[u8]) -> u32 {
        bytes.chunks(4).fold(0_u32, |sum, chunk| {
            let mut word = [0; 4];
            word[..chunk.len()].copy_from_slice(chunk);
            sum.wrapping_add(u32::from_be_bytes(word))
        })
    }
    fn table_range(font: &[u8], tag: &[u8; 4]) -> std::ops::Range<usize> {
        for record in font[12..12 + usize::from(read16(font, 4)) * 16]
            .as_chunks::<16>()
            .0
        {
            if &record[..4] == tag {
                let offset = read32(record, 8) as usize;
                return offset..offset + read32(record, 12) as usize;
            }
        }
        panic!("missing sfnt table {tag:?}");
    }
    fn table<'a>(font: &'a [u8], tag: &[u8; 4]) -> &'a [u8] {
        &font[table_range(font, tag)]
    }
    fn replace_table(font: &[u8], tag: &[u8; 4], replacement: &[u8]) -> Vec<u8> {
        let tables = font[12..12 + usize::from(read16(font, 4)) * 16]
            .as_chunks::<16>()
            .0
            .iter()
            .map(|record| {
                let current: [u8; 4] = record[..4].try_into().unwrap();
                (
                    current,
                    if &current == tag {
                        replacement.to_vec()
                    } else {
                        table(font, &current).to_vec()
                    },
                )
            })
            .collect();
        assemble(tables)
    }
    fn owned_name_table(
        version: u16,
        records: &[[u16; 6]],
        languages: &[[u16; 2]],
        gap: usize,
        strings: &[u8],
    ) -> Vec<u8> {
        let storage = 6
            + records.len() * 12
            + if version == 1 {
                2 + languages.len() * 4
            } else {
                0
            }
            + gap;
        let mut bytes = Vec::new();
        for word in [version, records.len() as u16, storage as u16] {
            bytes.extend_from_slice(&word.to_be_bytes());
        }
        for record in records {
            for word in record {
                bytes.extend_from_slice(&word.to_be_bytes());
            }
        }
        if version == 1 {
            bytes.extend_from_slice(&(languages.len() as u16).to_be_bytes());
            for language in languages {
                for word in language {
                    bytes.extend_from_slice(&word.to_be_bytes());
                }
            }
        }
        bytes.resize(storage, 0xdd);
        bytes.extend_from_slice(strings);
        bytes
    }
    fn name_string(names: &[u8], index: usize) -> &[u8] {
        let record = 6 + index * 12;
        let start = usize::from(read16(names, 4)) + usize::from(read16(names, record + 10));
        &names[start..start + usize::from(read16(names, record + 8))]
    }
    fn rectangle(width: i16, height: i16) -> Vec<u8> {
        let mut glyph = Vec::new();
        for word in [1_i16, 0, 0, width, height, 3, 0] {
            glyph.extend_from_slice(&word.to_be_bytes());
        }
        glyph.extend_from_slice(&[1; 4]); // Four on-curve points, signed 16-bit deltas.
        for delta in [0, width, 0, -width, 0, 0, height, 0] {
            glyph.extend_from_slice(&delta.to_be_bytes());
        }
        glyph
    }
    fn assemble(mut tables: Vec<([u8; 4], Vec<u8>)>) -> Vec<u8> {
        tables.sort_by_key(|(tag, _)| *tag);
        // The head checksum is computed with checkSumAdjustment cleared.
        put32(
            &mut tables.iter_mut().find(|(tag, _)| tag == b"head").unwrap().1,
            8,
            0,
        );
        let mut font = vec![0; 12 + tables.len() * 16];
        put32(&mut font, 0, 0x00010000);
        put16(&mut font, 4, tables.len() as u16);
        put16(&mut font, 6, 128);
        put16(&mut font, 8, 3);
        put16(&mut font, 10, 32);
        for (index, (tag, bytes)) in tables.into_iter().enumerate() {
            let offset = font.len();
            let record = 12 + index * 16;
            font[record..record + 4].copy_from_slice(&tag);
            put32(&mut font, record + 4, checksum(&bytes));
            put32(&mut font, record + 8, offset as u32);
            put32(&mut font, record + 12, bytes.len() as u32);
            font.extend_from_slice(&bytes);
            while !font.len().is_multiple_of(4) {
                font.push(0);
            }
        }
        let adjustment = 0xb1b0afba_u32.wrapping_sub(checksum(&font));
        let head_offset = table_range(&font, b"head").start;
        put32(&mut font, head_offset + 8, adjustment);
        assert_eq!(checksum(&font), 0xb1b0afba);
        font
    }
    fn synthetic_font() -> (Vec<u8>, [Vec<u8>; 4]) {
        // Standard sfnt tables and owned rectangle outlines, no OS font data.
        // https://learn.microsoft.com/en-us/typography/opentype/spec/otff
        let mut composite = Vec::new();
        for word in [-1_i16, 0, 0, 700, 400] {
            composite.extend_from_slice(&word.to_be_bytes());
        }
        for (flags, glyph, x) in [(0x23_u16, 0_u16, 0_i16), (0x23, 1, 200), (0x03, 2, 400)] {
            composite.extend_from_slice(&flags.to_be_bytes());
            composite.extend_from_slice(&glyph.to_be_bytes());
            composite.extend_from_slice(&x.to_be_bytes());
            composite.extend_from_slice(&0_i16.to_be_bytes());
        }
        let glyphs = [
            rectangle(100, 100),
            rectangle(200, 200),
            rectangle(300, 400),
            composite,
        ];
        let mut glyf = Vec::new();
        let mut loca = Vec::new();
        for glyph in &glyphs {
            loca.extend_from_slice(&(glyf.len() as u32).to_be_bytes());
            glyf.extend_from_slice(glyph);
        }
        loca.extend_from_slice(&(glyf.len() as u32).to_be_bytes());

        let mut head = vec![0; 54];
        put32(&mut head, 0, 0x00010000);
        put32(&mut head, 4, 0x00010000);
        put32(&mut head, 12, 0x5f0f3cf5);
        put16(&mut head, 18, 1000);
        put16(&mut head, 40, 700);
        put16(&mut head, 42, 400);
        put16(&mut head, 46, 8);
        put16(&mut head, 48, 2);
        put16(&mut head, 50, 1); // Long loca offsets.
        let mut hhea = vec![0; 36];
        put32(&mut hhea, 0, 0x00010000);
        put16(&mut hhea, 4, 800);
        put16(&mut hhea, 6, (-200_i16) as u16);
        put16(&mut hhea, 10, 630);
        put16(&mut hhea, 14, (-70_i16) as u16);
        put16(&mut hhea, 16, 700);
        put16(&mut hhea, 18, 1);
        put16(&mut hhea, 34, 4);
        let mut hmtx = Vec::new();
        for advance in [600_u16, 610, 620, 630] {
            hmtx.extend_from_slice(&advance.to_be_bytes());
            hmtx.extend_from_slice(&0_i16.to_be_bytes());
        }
        let mut maxp = vec![0; 32];
        put32(&mut maxp, 0, 0x00010000);
        for (offset, value) in [
            (4, 4),
            (6, 4),
            (8, 1),
            (10, 12),
            (12, 3),
            (14, 1),
            (28, 3),
            (30, 1),
        ] {
            put16(&mut maxp, offset, value);
        }
        let mut os2 = vec![0; 78];
        for (offset, value) in [
            (2, 615),
            (4, 400),
            (6, 5),
            (62, 64),
            (64, 65),
            (66, 72),
            (68, 800),
            (72, 0),
            (74, 800),
            (76, 200),
        ] {
            put16(&mut os2, offset, value);
        }
        put16(&mut os2, 70, (-200_i16) as u16);
        os2[58..62].copy_from_slice(b"TEST");
        let mut post = vec![0; 32];
        put32(&mut post, 0, 0x00030000); // No glyph names, numeric fallback is required.
        put16(&mut post, 8, (-75_i16) as u16);
        put16(&mut post, 10, 50);
        let names = [
            (1, "SyntheticGlyphs"),
            (2, "Regular"),
            (4, "SyntheticGlyphs Regular"),
            (6, "SyntheticGlyphs"),
        ];
        let mut name = vec![0; 6 + names.len() * 12];
        put16(&mut name, 2, names.len() as u16);
        let storage = name.len();
        put16(&mut name, 4, storage as u16);
        for (index, (id, text)) in names.iter().enumerate() {
            let offset = 6 + index * 12;
            put16(&mut name, offset, 1); // Macintosh Roman, ASCII-only owned names.
            put16(&mut name, offset + 6, *id);
            put16(&mut name, offset + 8, text.len() as u16);
            let string_offset = name.len() - storage;
            put16(&mut name, offset + 10, string_offset as u16);
            name.extend_from_slice(text.as_bytes());
        }
        let mut cmap = Vec::new();
        for word in [0_u16, 1, 3, 1] {
            cmap.extend_from_slice(&word.to_be_bytes());
        }
        cmap.extend_from_slice(&12_u32.to_be_bytes());
        for word in [
            4_u16, 48, 0, 8, 8, 2, 0, 65, 66, 67, 0xffff, 0, 65, 66, 67, 0xffff,
        ] {
            cmap.extend_from_slice(&word.to_be_bytes());
        }
        for delta in [-63_i16, -66, -64, 1] {
            // U+0041 -> 2, U+0042 -> 0, U+0043 -> 3.
            cmap.extend_from_slice(&delta.to_be_bytes());
        }
        cmap.extend_from_slice(&[0; 8]);
        let tables = vec![
            (*b"OS/2", os2),
            (*b"cmap", cmap),
            (*b"glyf", glyf),
            (*b"head", head),
            (*b"hhea", hhea),
            (*b"hmtx", hmtx),
            (*b"loca", loca),
            (*b"maxp", maxp),
            (*b"name", name),
            (*b"post", post),
        ];
        (assemble(tables), glyphs)
    }
    fn named_font(font: &[u8], source_glyphs: &[Vec<u8>; 4]) -> (Vec<u8>, Vec<Vec<u8>>) {
        let glyph_count = 304_u16;
        let mut glyphs = source_glyphs[..3].to_vec();
        let mut composite = Vec::new();
        for word in [-1_i16, 0, 0, 700, 400] {
            composite.extend_from_slice(&word.to_be_bytes());
        }
        for index in 4..glyph_count {
            let flags = if index + 1 == glyph_count {
                0x03_u16
            } else {
                0x23
            };
            for word in [flags, index, 0, 0] {
                composite.extend_from_slice(&word.to_be_bytes());
            }
        }
        glyphs.push(composite);
        glyphs.extend((4..glyph_count).map(|_| rectangle(300, 400)));
        let mut glyf = Vec::new();
        let mut loca = Vec::new();
        let mut hmtx = Vec::new();
        for glyph in &glyphs {
            loca.extend_from_slice(&(glyf.len() as u32).to_be_bytes());
            glyf.extend_from_slice(glyph);
            hmtx.extend_from_slice(&600_u16.to_be_bytes());
            hmtx.extend_from_slice(&0_u16.to_be_bytes());
        }
        loca.extend_from_slice(&(glyf.len() as u32).to_be_bytes());
        let names: Vec<_> = (2..glyph_count)
            .map(|index| match index {
                // Exact post names must win before numeric-name fallbacks.
                2 => b"uni10000".to_vec(),
                3 => b"index-1".to_vec(),
                _ => format!("ownedGlyph{index}").into_bytes(),
            })
            .collect();
        let mut post = table(font, b"post")[..32].to_vec();
        put32(&mut post, 0, 0x00020000);
        post.extend_from_slice(&glyph_count.to_be_bytes());
        for index in 0..glyph_count {
            let name_index = if index < 2 { index } else { 258 + index - 2 };
            post.extend_from_slice(&name_index.to_be_bytes());
        }
        for name in &names {
            post.push(name.len() as u8);
            post.extend_from_slice(name);
        }
        let mut tables: Vec<_> = font[12..12 + usize::from(read16(font, 4)) * 16]
            .as_chunks::<16>()
            .0
            .iter()
            .map(|record| {
                let tag: [u8; 4] = record[..4].try_into().unwrap();
                (tag, table(font, &tag).to_vec())
            })
            .collect();
        for (tag, bytes) in &mut tables {
            match &*tag {
                b"glyf" => *bytes = glyf.clone(),
                b"loca" => *bytes = loca.clone(),
                b"hmtx" => *bytes = hmtx.clone(),
                b"post" => *bytes = post.clone(),
                b"hhea" => put16(bytes, 34, glyph_count),
                b"maxp" => {
                    put16(bytes, 4, glyph_count);
                    put16(bytes, 10, 1200);
                    put16(bytes, 12, 300);
                    put16(bytes, 28, 300);
                }
                _ => {}
            }
        }
        (assemble(tables), names)
    }
    fn embedded_font(pdf: &[u8]) -> &[u8] {
        let marker = b"/FontFile2 ";
        let reference = pdf
            .windows(marker.len())
            .position(|part| part == marker)
            .expect("the PDF must embed a TrueType FontFile2");
        let number = &pdf[reference + marker.len()..];
        let digits = number
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        let object = format!(
            "\n{} 0 obj\n",
            std::str::from_utf8(&number[..digits]).unwrap()
        );
        let offset = pdf
            .windows(object.len())
            .position(|part| part == object.as_bytes())
            .unwrap();
        let body = &pdf[offset + object.len()..];
        let start = body
            .windows(7)
            .position(|part| part == b"stream\n")
            .unwrap()
            + 7;
        let payload = &body[start..];
        let end = payload
            .windows(10)
            .position(|part| part == b"\nendstream")
            .unwrap();
        &payload[..end]
    }

    let project = Project::new();
    let (font, glyphs) = synthetic_font();
    project.write_bytes("synthetic.ttf", &font);
    let mut encoding = String::from("/SyntheticEncoding [\n");
    for code in 0..256 {
        let name = match code {
            65 => "index2",
            66 => "index1",
            67 => "index3",
            68 => "index0",
            69 => "index-1",
            70 => "uni10000",
            71 => "uni0042",
            72 => "uni0041",
            73 => "index2147483648",
            74 => "uni100000000",
            75 => "index2tail",
            76 => "uni0041tail",
            77 => "index02",
            78 => "index0x2",
            _ => ".notdef",
        };
        encoding.push_str(&format!("/{name}\n"));
    }
    encoding.push_str("] def\n");
    project.write("synthetic.enc", &encoding);
    project.write(
        "main.tex",
        r#"\pdfcompresslevel=0
\pdfobjcompresslevel=0
\pdfmapline{=cmr10 SyntheticGlyphs <synthetic.enc <synthetic.ttf}
\pdfmapline{=cmr10 MustNotReplace PidEid=32768,1 <synthetic.ttf}
\pdfmapline{=cmr10 MustNotReplace PidEid=3,-32769 <synthetic.ttf}
\documentclass{article}
\pagestyle{empty}
\begin{document}
\font\probe=cmr10 at 10pt
\probe ABCDEFGHIJKLMN
\end{document}
"#,
    );
    project.success(&["build", "main.tex", "--report-json", "--once", "--force"]);
    let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    let diagnostic: String = log.split_ascii_whitespace().collect();
    for warning in [
        "`index0' out of valid range [1..4)",
        "`index-1' out of valid range [1..4)",
        "`uni10000' out of valid unicode range [0..65536)",
        "`uni0042' is mapped to index 0 which is out of valid range [1..4)",
        "numeric glyph `index2147483648' out of range",
        "numeric glyph `uni100000000' out of range",
        "glyph `index2tail' not found",
        "glyph `uni0041tail' not found",
    ] {
        assert!(
            diagnostic.contains(&warning.split_ascii_whitespace().collect::<String>()),
            "{log}"
        );
    }
    assert_eq!(
        diagnostic.matches("PidEidvalueoutofrange").count(),
        2,
        "both overflowing ID fields must reject the replacement map entry: {log}"
    );
    let pdf = fs::read(project.0.join("build/main.pdf")).unwrap();
    let embedded = embedded_font(&pdf);
    assert_eq!(read16(table(embedded, b"maxp"), 4), 4);
    assert_eq!(read16(table(embedded, b"hhea"), 34), 4);
    assert_eq!(table(embedded, b"hmtx"), table(&font, b"hmtx"));
    let loca = table(embedded, b"loca");
    let long_loca = read16(table(embedded, b"head"), 50) == 1;
    let offsets = (0..=4)
        .map(|index| {
            if long_loca {
                read32(loca, index * 4) as usize
            } else {
                usize::from(read16(loca, index * 2)) * 2
            }
        })
        .collect::<Vec<_>>();
    assert!(
        offsets.windows(2).all(|pair| pair[0] < pair[1]),
        "{offsets:?}"
    );
    let glyf = table(embedded, b"glyf");
    for (index, expected) in glyphs.iter().enumerate() {
        assert_eq!(&glyf[offsets[index]..offsets[index + 1]], expected);
    }
    // Small subsets use the Macintosh format-0 byte cmap, with reserved glyph1
    // retained at output1 and composite3 still referring to outputs0,1,2.
    let cmap = table(embedded, b"cmap");
    let macintosh = (0..usize::from(read16(cmap, 2)))
        .map(|index| 4 + index * 8)
        .find(|&record| read16(cmap, record) == 1 && read16(cmap, record + 2) == 0)
        .expect("Macintosh byte cmap");
    let format0 = read32(cmap, macintosh + 4) as usize;
    assert_eq!(read16(cmap, format0), 0);
    assert_eq!(read16(cmap, format0 + 2), 262);
    let mut expected_encoding = [0_u8; 256];
    for (code, expected) in [
        (65, 2),
        (66, 1),
        (67, 3),
        (68, 0),
        (69, 0),
        (70, 0),
        (71, 0),
        (72, 2),
        (73, 0),
        (74, 0),
        (75, 0),
        (76, 0),
        (77, 2),
        (78, 2),
    ] {
        expected_encoding[code] = expected;
    }
    assert_eq!(&cmap[format0 + 6..format0 + 262], &expected_encoding);

    fn write_named_project(project: &Project, font: &[u8]) {
        project.write_bytes("synthetic.ttf", font);
        let mut encoding = String::from("/SyntheticEncoding [\n");
        for code in 0..256 {
            let name = match code {
                65 => "uni10000",
                66 => "index1",
                67 => "index-1",
                _ => ".notdef",
            };
            encoding.push_str(&format!("/{name}\n"));
        }
        encoding.push_str("] def\n");
        project.write("synthetic.enc", &encoding);
        project.write(
            "main.tex",
            r#"\pdfcompresslevel=0
\pdfobjcompresslevel=0
\pdfmapline{=cmr10 SyntheticGlyphs <synthetic.enc <synthetic.ttf}
\documentclass{article}
\pagestyle{empty}
\begin{document}
\font\probe=cmr10 at 10pt
\probe ABC
\end{document}
"#,
        );
    }

    // One selected composite pulls in 300 owned components, crossing the old
    // byte-sized post writer counter while preserving every source name.
    let (named, names) = named_font(&font, &glyphs);
    let named_project = Project::new();
    write_named_project(&named_project, &named);
    named_project.success(&["build", "main.tex", "--report-json", "--once", "--force"]);
    let log = fs::read_to_string(named_project.0.join("build/main.log")).unwrap();
    assert!(
        !log.contains("out of valid"),
        "exact post-name precedence must win: {log}"
    );
    let pdf = fs::read(named_project.0.join("build/main.pdf")).unwrap();
    let embedded = embedded_font(&pdf);
    assert_eq!(read16(table(embedded, b"maxp"), 4), 304);
    let post = table(embedded, b"post");
    assert_eq!(read32(post, 0), 0x00020000);
    assert_eq!(&post[4..32], &table(&named, b"post")[4..32]);
    assert_eq!(read16(post, 32), 304);
    assert_eq!(read16(post, 34), 0);
    assert_eq!(read16(post, 36), 1);
    let mut position = 34 + 304 * 2;
    for (index, expected) in names.iter().enumerate() {
        assert_eq!(read16(post, 34 + (index + 2) * 2), 258 + index as u16);
        let length = usize::from(post[position]);
        position += 1;
        assert_eq!(&post[position..position + length], expected);
        position += length;
    }
    assert_eq!(position, post.len());
    assert_eq!(read16(post, 34 + 258 * 2), 514);
    assert_eq!(read16(post, 34 + 303 * 2), 559);

    let post_range = table_range(&named, b"post");
    let strings = post_range.start + 34 + 304 * 2;
    let last_string = strings
        + names[..names.len() - 1]
            .iter()
            .map(|name| name.len() + 1)
            .sum::<usize>();
    for (case, expected) in [
        (0, "post: truncated glyph names"),
        (1, "post: glyph name contains an embedded NUL"),
        (2, "post: glyph name index has no Pascal string"),
        (3, "post: table extent exceeds font file"),
    ] {
        let mut malformed = named.clone();
        match case {
            0 => malformed[last_string] = 255,
            1 => malformed[strings + 1] = 0,
            2 => put16(&mut malformed, post_range.start + 34 + 303 * 2, u16::MAX),
            3 => {
                let record = (0..usize::from(read16(&malformed, 4)))
                    .map(|index| 12 + index * 16)
                    .find(|&record| &malformed[record..record + 4] == b"post")
                    .unwrap();
                put32(&mut malformed, record + 12, u32::MAX);
            }
            _ => unreachable!(),
        }
        let malformed_project = Project::new();
        write_named_project(&malformed_project, &malformed);
        let output =
            malformed_project.run(&["build", "main.tex", "--report-json", "--once", "--force"]);
        assert_eq!(
            output.status.code(),
            Some(1),
            "malformed post case {case}: {output:?}"
        );
        let diagnostics = format!(
            "{}\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            fs::read_to_string(malformed_project.0.join("build/main.log")).unwrap_or_default()
        );
        let normalized: String = diagnostics.split_ascii_whitespace().collect();
        let expected: String = expected.split_ascii_whitespace().collect();
        assert!(normalized.contains(&expected), "case {case}: {diagnostics}");
        assert!(
            !diagnostics.contains("panicked"),
            "case {case}: {diagnostics}"
        );
    }

    // Real format-1 storage has its own header offset, language-tag records and
    // UTF-16 strings. Long license text and unused padding must not enter a
    // fixed sanitizing buffer or be duplicated in the subset.
    let utf16 = |bytes: &[u8]| {
        bytes
            .iter()
            .copied()
            .flat_map(|byte| [0, byte])
            .collect::<Vec<_>>()
    };
    let windows_family = utf16(b"Owned Windows Family");
    let windows_name = utf16(b"SyntheticGlyphs");
    let license = utf16(&vec![b'L'; 2048]);
    let language = utf16(b"en");
    let windows_strings = [
        windows_family.as_slice(),
        windows_name.as_slice(),
        license.as_slice(),
        language.as_slice(),
        &vec![0xcc; 8192],
    ]
    .concat();
    let windows_records = [
        [3, 1, 0x8000, 4, windows_family.len() as u16, 0],
        [
            3,
            1,
            0x8000,
            6,
            windows_name.len() as u16,
            windows_family.len() as u16,
        ],
        [
            3,
            1,
            0x8000,
            13,
            license.len() as u16,
            (windows_family.len() + windows_name.len()) as u16,
        ],
    ];
    let windows_language_offset = windows_family.len() + windows_name.len() + license.len();
    let windows_table = owned_name_table(
        1,
        &windows_records,
        &[[language.len() as u16, windows_language_offset as u16]],
        17,
        &windows_strings,
    );
    let windows_font = replace_table(&named, b"name", &windows_table);
    let windows_project = Project::new();
    write_named_project(&windows_project, &windows_font);
    windows_project.success(&["build", "main.tex", "--report-json", "--once", "--force"]);
    let windows_pdf = fs::read(windows_project.0.join("build/main.pdf")).unwrap();
    let output_names = table(embedded_font(&windows_pdf), b"name");
    assert_eq!(read16(output_names, 0), 1);
    assert_eq!(read16(output_names, 2), 3);
    assert_eq!(read16(output_names, 4), 48);
    let prefix = &name_string(output_names, 1)[..14];
    assert!(
        prefix
            .as_chunks::<2>()
            .0
            .iter()
            .take(6)
            .all(|pair| pair[0] == 0 && pair[1].is_ascii_uppercase())
    );
    assert_eq!(&prefix[12..], &[0, b'+']);
    assert_eq!(
        name_string(output_names, 0),
        [prefix, &windows_family].concat()
    );
    assert_eq!(
        name_string(output_names, 1),
        [prefix, &windows_name].concat()
    );
    assert_eq!(name_string(output_names, 2), license);
    assert_eq!(read16(output_names, 42), 1);
    assert_eq!(read16(output_names, 44), language.len() as u16);
    let language_start =
        usize::from(read16(output_names, 4)) + usize::from(read16(output_names, 46));
    assert_eq!(&output_names[language_start..], &language);
    assert_eq!(
        output_names.len(),
        48 + 28 + windows_family.len() + windows_name.len() + license.len() + language.len()
    );

    // Full embedding still copies the original valid table byte-for-byte,
    // including its format-1 layout and padding.
    let copy_project = Project::new();
    write_named_project(&copy_project, &windows_font);
    let source = fs::read_to_string(copy_project.0.join("main.tex")).unwrap();
    copy_project.write(
        "main.tex",
        &source
            .replace("<synthetic.enc ", "")
            .replace("<synthetic.ttf", "<<synthetic.ttf"),
    );
    copy_project.success(&["build", "main.tex", "--report-json", "--once", "--force"]);
    let copy_pdf = fs::read(copy_project.0.join("build/main.pdf")).unwrap();
    assert_eq!(table(embedded_font(&copy_pdf), b"name"), &windows_table);

    // A valid source may share long strings. When naive subset concatenation
    // exceeds u16 offsets, the checked planner retains that sharing.
    let mut shared_records = vec![[1, 0, 0, 6, 15, 0]];
    shared_records.extend((0..8).map(|language| [1, 0, language, 4, 32000, 15]));
    shared_records.sort_by_key(|record| (record[0], record[1], record[2], record[3]));
    let shared_table = owned_name_table(
        0,
        &shared_records,
        &[],
        23,
        &[b"SyntheticGlyphs".as_slice(), &vec![b'A'; 32000]].concat(),
    );
    let shared_font = replace_table(&named, b"name", &shared_table);
    let shared_project = Project::new();
    write_named_project(&shared_project, &shared_font);
    shared_project.success(&["build", "main.tex", "--report-json", "--once", "--force"]);
    let shared_pdf = fs::read(shared_project.0.join("build/main.pdf")).unwrap();
    let shared_output = table(embedded_font(&shared_pdf), b"name");
    assert_eq!(read16(shared_output, 2), 9);
    assert_eq!(shared_output.len(), 6 + 9 * 12 + 22 + 32007);
    let postscript = shared_records
        .iter()
        .position(|record| record[3] == 6)
        .unwrap();
    let prefix = &name_string(shared_output, postscript)[..7];
    assert!(prefix[..6].iter().all(u8::is_ascii_uppercase));
    assert_eq!(prefix[6], b'+');
    for (index, record) in shared_records.iter().enumerate() {
        let source = if record[3] == 6 {
            b"SyntheticGlyphs".to_vec()
        } else {
            vec![b'A'; 32000]
        };
        assert_eq!(
            name_string(shared_output, index),
            [prefix, &source].concat()
        );
    }

    let mut too_long = vec![b'A'; usize::from(u16::MAX)];
    too_long[..15].copy_from_slice(b"SyntheticGlyphs");
    let too_long_table = owned_name_table(
        0,
        &[[1, 0, 0, 4, u16::MAX, 0], [1, 0, 0, 6, 15, 0]],
        &[],
        0,
        &too_long,
    );
    for (case, expected) in [
        (0, "name: table extent exceeds font file"),
        (1, "name: string storage overlaps records or exceeds table"),
        (2, "name: string span exceeds table storage"),
        (3, "name: UTF-16 string has an odd byte length"),
        (4, "name: unsupported table version"),
        (5, "name: string storage overlaps records or exceeds table"),
        (6, "name: subset string exceeds unsigned 16-bit length"),
    ] {
        let mut malformed = if case == 6 {
            replace_table(&named, b"name", &too_long_table)
        } else {
            named.clone()
        };
        let name_range = table_range(&malformed, b"name");
        match case {
            0 => {
                let record = (0..usize::from(read16(&malformed, 4)))
                    .map(|index| 12 + index * 16)
                    .find(|&record| &malformed[record..record + 4] == b"name")
                    .unwrap();
                put32(&mut malformed, record + 12, u32::MAX);
            }
            1 => put16(&mut malformed, name_range.start + 4, 5),
            2 => put16(&mut malformed, name_range.start + 14, u16::MAX),
            3 => {
                put16(&mut malformed, name_range.start + 6, 3);
                put16(&mut malformed, name_range.start + 8, 1);
            }
            4 => put16(&mut malformed, name_range.start, 2),
            5 => put16(&mut malformed, name_range.start, 1),
            6 => {}
            _ => unreachable!(),
        }
        let malformed_project = Project::new();
        write_named_project(&malformed_project, &malformed);
        let output =
            malformed_project.run(&["build", "main.tex", "--report-json", "--once", "--force"]);
        assert_eq!(
            output.status.code(),
            Some(1),
            "malformed name case {case}: {output:?}"
        );
        let diagnostics = format!(
            "{}\n{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            fs::read_to_string(malformed_project.0.join("build/main.log")).unwrap_or_default()
        );
        let normalized: String = diagnostics.split_ascii_whitespace().collect();
        let expected: String = expected.split_ascii_whitespace().collect();
        assert!(
            normalized.contains(&expected),
            "name case {case}: {diagnostics}"
        );
        assert!(
            !diagnostics.contains("panicked"),
            "name case {case}: {diagnostics}"
        );
    }
}

#[test]
fn bundled_engine_checks_subfont_map_name_capacity_before_assembly() {
    let project = Project::new();
    fn document(prefix: &str, sfd_name: &str) -> String {
        format!(
            "\\pdfmapline{{={prefix}@{sfd_name}@ <unused.ttf}}\n\
\\documentclass{{article}}\n\
\\pagestyle{{empty}}\n\
\\begin{{document}}\n\
Subfont map name control.\n\
\\end{{document}}\n"
        )
    }
    let longest_sfd_name = "s".repeat(251);
    project.write(&format!("{longest_sfd_name}.sfd"), "x 65\n");
    project.write("main.tex", &document("probe", &longest_sfd_name));
    project.success(&["build", "main.tex", "--report-json", "--once", "--force"]);

    project.write("synthetic.sfd", "x 65\n");
    project.write("main.tex", &document(&"p".repeat(254), "synthetic"));
    project.success(&["build", "main.tex", "--report-json", "--once", "--force"]);

    for (prefix, sfd_name, infix) in [
        ("probe".to_owned(), "s".repeat(252), "x"),
        ("probe".to_owned(), "s".repeat(400), "x"),
        ("p".repeat(255), "synthetic".to_owned(), "x"),
        ("p".repeat(254), "synthetic".to_owned(), "xx"),
    ] {
        project.write("synthetic.sfd", &format!("{infix} 65\n"));
        project.write("main.tex", &document(&prefix, &sfd_name));
        let output = project.run(&["build", "main.tex", "--report-json", "--once", "--force"]);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
        let diagnostic: String = log.split_ascii_whitespace().collect();
        assert!(diagnostic.contains("bufferoverflowatfile"), "{log}");
        assert!(diagnostic.contains("subfont.rs"), "{log}");
        assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }
}

#[test]
fn bundled_engine_rejects_out_of_bounds_type1_encoding_intervals() {
    let project = Project::new();
    project.write(
        "main.tex",
        r"\pdfmapline{=cmr10 SyntheticInterval <synthetic.pfa}
\documentclass{article}
\pagestyle{empty}
\begin{document}
\font\probe=cmr10 at 10pt
\probe A
\end{document}
",
    );
    for interval in [
        "dup dup 255 2 getinterval 0 exch putinterval".to_owned(),
        "dup dup 0 2 getinterval 255 exch putinterval".to_owned(),
        // A long initialized numeric field crosses the former native scanner's
        // 512-byte limit. Leading zero selects octal, so 377 still means 255.
        format!(
            "dup dup {}377 2 getinterval 0 exch putinterval",
            "0".repeat(1024)
        ),
    ] {
        // The invalid encoding is in the plaintext portion, before eexec.
        // An exact interval diagnostic proves rejection before any glyph copy.
        project.write(
            "synthetic.pfa",
            &format!(
                "%!PS-AdobeFont-1.0: SyntheticInterval 1.0\n\
11 dict begin\n\
/FontName /SyntheticInterval def\n\
/FontType 1 def\n\
/FontMatrix [0.001 0 0 0.001 0 0] readonly def\n\
/FontBBox [0 0 1000 1000] readonly def\n\
/Encoding 256 array\n\
{interval}\n\
readonly def\n\
currentdict end\n\
currentfile eexec\n"
            ),
        );
        let output = project.run(&["build", "main.tex", "--report-json", "--once", "--force"]);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
        let diagnostic: String = log.split_ascii_whitespace().collect();
        assert!(
            diagnostic.contains("invalidpfb,encodingintervaloutofrange"),
            "{log}"
        );
        assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }
}

#[test]
fn bundled_engine_rejects_malformed_sfd_fields_without_panicking() {
    let project = Project::new();
    // Parsing the map expands this unused subfont immediately. No host font or
    // even a TrueType fixture is needed to reach the actual SFD reader.
    project.write(
        "main.tex",
        r"\pdfmapline{=probe@synthetic@ <unused.ttf}
\documentclass{article}
\pagestyle{empty}
\begin{document}
SFD parser control.
\end{document}
",
    );
    project.write("synthetic.sfd", "10 0: -1 010 0x10 65_66 \\\n67\n");
    project.success(&["build", "main.tex", "--report-json", "--once", "--force"]);
    let control_log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    assert!(
        control_log
            .split_ascii_whitespace()
            .collect::<String>()
            .contains("synthetic.sfd"),
        "{control_log}"
    );
    for (source, expected) in [
        ("10 -1: 65\n".to_owned(), "invalidindex:"),
        ("10 256: 65\n".to_owned(), "invalidindex:"),
        ("10 255: 65_66\n".to_owned(), "invalidrange:"),
        (format!("10 {}_\n", "0".repeat(200)), "invalidtoken:"),
        ("10 9223372036854775808\n".to_owned(), "invalidtoken:"),
        ("10 2_1\n".to_owned(), "invalidrange:"),
        ("10 255: 65 \\\n66\n".to_owned(), "invalidindex:"),
    ] {
        project.write("synthetic.sfd", &source);
        let output = project.run(&["build", "main.tex", "--report-json", "--once", "--force"]);
        assert_eq!(output.status.code(), Some(1), "{source:?}: {output:?}");
        let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
        let diagnostic: String = log.split_ascii_whitespace().collect();
        assert!(diagnostic.contains(expected), "{source:?}: {log}");
        assert!(diagnostic.contains("synthetic.sfd"), "{log}");
        assert!(!String::from_utf8_lossy(&output.stderr).contains("panicked"));
    }
}

#[test]
fn bundled_engine_warns_and_preserves_mapping_for_oversized_tounicode_sequences() {
    let project = Project::new();
    let valid_sequence = "0041 ".repeat(63);
    let at_capacity = "0041 ".repeat(64);
    let past_capacity = "0041 ".repeat(65);
    project.write(
        "main.tex",
        &format!(
            "\\pdfcompresslevel=0\n\
\\pdfobjcompresslevel=0\n\
\\pdfgentounicode=1\n\
\\pdfglyphtounicode{{A}}{{0042}}\n\
\\pdfglyphtounicode{{A}}{{{at_capacity}}}\n\
\\pdfglyphtounicode{{A}}{{{past_capacity}}}\n\
\\pdfglyphtounicode{{B}}{{{valid_sequence}}}\n\
\\pdfglyphtounicode{{C}}{{10000000000000000}}\n\
\\documentclass{{article}}\n\
\\pagestyle{{empty}}\n\
\\begin{{document}}\n\
\\font\\probe=cmr10 at 10pt\n\
\\probe ABC\n\
\\end{{document}}\n"
        ),
    );
    project.success(&["build", "main.tex", "--report-json", "--once", "--force"]);
    let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    let diagnostic: String = log.split_ascii_whitespace().collect();
    assert_eq!(
        diagnostic.matches("ToUnicode:invalidparameter(s):").count(),
        2,
        "{log}"
    );
    assert!(
        diagnostic.contains("ToUnicode:valueoutofrange[0,10FFFF]:"),
        "{log}"
    );
    let pdf = fs::read(project.0.join("build/main.pdf")).unwrap();
    let pdf = String::from_utf8_lossy(&pdf);
    assert!(pdf.contains("<41> <0042>"), "prior A mapping was lost");
    assert!(
        pdf.contains(&format!("<42> <{}>", "0041".repeat(63))),
        "the valid 252-byte B sequence was lost"
    );
    assert!(!pdf.contains(&format!("<41> <{}>", "0041".repeat(64))));
    assert!(!pdf.contains(&format!("<41> <{}>", "0041".repeat(65))));
}

#[test]
fn diagnostic_cache_routes_are_stable_and_profiling_preserves_outputs() {
    fn counter(profile: &Value, name: &str) -> u64 {
        profile["counters"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["name"] == name)
            .unwrap()["value"]
            .as_u64()
            .unwrap()
    }

    fn profiled(project: &Project) -> (Value, Value) {
        let output = project.run_with_env(
            &[
                "build",
                "main.tex",
                "--config",
                "tekai.toml",
                "--once",
                "--report-json",
            ],
            &[("TEKAI_DIAGNOSTIC_PROFILE", "1")],
        );
        assert!(output.status.success(), "{output:?}");
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        assert_eq!(stderr.lines().count(), 1, "{stderr}");
        assert!(stderr.len() <= 16 * 1024);
        let profile: Value =
            serde_json::from_str(stderr.strip_prefix("TEKAI_PROFILE ").unwrap()).unwrap();
        assert_eq!(profile["schema_version"], 4);
        assert_eq!(profile["untrusted"], true);
        assert_eq!(profile["status"], "success");
        assert!(profile["phases"].as_array().unwrap().iter().all(|phase| {
            phase["active_calls"] == 0
                && phase["elapsed_ms"]
                    .as_f64()
                    .is_none_or(|value| value.is_finite() && value >= 0.0)
        }));
        (report, profile)
    }

    let project = Project::new();
    project.write("tekai.toml", "# Explicit empty fixture configuration.\n");
    let mut main = String::from("\\documentclass{article}\n\\begin{document}\nProbe.\n");
    for index in 0..16 {
        let name = format!("deps/part-{index:02}.tex");
        project.write(&name, "% A recorded dependency.\n");
        main.push_str(&format!("\\input{{{name}}}\n"));
    }
    main.push_str("\\end{document}\n");
    project.write("main.tex", &main);
    let args = &[
        "build",
        "main.tex",
        "--config",
        "tekai.toml",
        "--once",
        "--report-json",
    ];
    let initial = project.success(args);
    assert_eq!(initial["tex_runs"], 1);
    let state_path = project.0.join("build/.tekai-main.state.json");
    let state = fs::read(&state_path).unwrap();
    let pdf = fs::read(project.0.join("build/main.pdf")).unwrap();
    let mut prior_counters = None;
    for _ in 0..4 {
        let (report, profile) = profiled(&project);
        assert_eq!(report["skipped"], true);
        assert_eq!(report["tex_runs"], 0);
        assert_eq!(counter(&profile, "warm_cache_hits"), 1);
        assert!(counter(&profile, "state_dependencies") >= 17);
        assert_eq!(counter(&profile, "state_bytes_read"), state.len() as u64);
        assert_eq!(
            counter(&profile, "freshness_requests"),
            counter(&profile, "state_dependencies")
        );
        for name in [
            "freshness_memo_hits",
            "stale_inputs",
            "content_hash_fallbacks",
            "effective_hash_fallbacks",
        ] {
            assert_eq!(counter(&profile, name), 0, "{name}: {profile}");
        }
        if let Some(prior) = &prior_counters {
            assert_eq!(&profile["counters"], prior);
        }
        prior_counters = Some(profile["counters"].clone());
        assert_eq!(fs::read(&state_path).unwrap(), state);
        assert_eq!(fs::read(project.0.join("build/main.pdf")).unwrap(), pdf);
    }
    let plain = project.run(args);
    assert!(plain.status.success(), "{plain:?}");
    assert!(plain.stderr.is_empty());
    assert_eq!(
        serde_json::from_slice::<Value>(&plain.stdout).unwrap()["skipped"],
        true
    );

    project.write(
        "deps/part-00.tex",
        "% A changed comment must invalidate the complete raw source.\n",
    );
    let (report, profile) = profiled(&project);
    assert_eq!(report["skipped"], false);
    assert_eq!(report["tex_runs"], 1);
    assert_eq!(counter(&profile, "warm_cache_hits"), 0);
    assert!(counter(&profile, "stale_inputs") > 0);
    let refreshed_state = fs::read(&state_path).unwrap();
    assert_ne!(refreshed_state, state);

    // Identical bytes with changed metadata still exercise the full-source
    // hash fallback. Comments are not removed or interpreted for freshness.
    project.write(
        "deps/part-00.tex",
        "% A changed comment must invalidate the complete raw source.\n",
    );
    let (report, profile) = profiled(&project);
    assert_eq!(report["skipped"], true);
    assert_eq!(counter(&profile, "effective_hash_fallbacks"), 1);
    assert_eq!(counter(&profile, "stale_inputs"), 0);
    assert_eq!(fs::read(&state_path).unwrap(), refreshed_state);

    project.write("deps/part-00.tex", "Changed dependency.\n");
    let (report, profile) = profiled(&project);
    assert_eq!(report["skipped"], false);
    assert_eq!(report["tex_runs"], 1);
    assert_eq!(counter(&profile, "warm_cache_hits"), 0);
    assert!(counter(&profile, "stale_inputs") > 0);
    assert_ne!(fs::read(project.0.join("build/main.pdf")).unwrap(), pdf);
}

#[test]
fn bundled_engine_profiles_valid_png_and_rejects_oversized_metadata() {
    fn append_chunk(png: &mut Vec<u8>, typ: &[u8; 4], payload: &[u8]) {
        png.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        png.extend_from_slice(typ);
        png.extend_from_slice(payload);
        let mut crc = u32::MAX;
        for byte in typ.iter().chain(payload) {
            crc ^= u32::from(*byte);
            for _ in 0..8 {
                crc = (crc >> 1) ^ (0xedb88320 & 0u32.wrapping_sub(crc & 1));
            }
        }
        png.extend_from_slice(&(!crc).to_be_bytes());
    }

    let project = Project::new();
    project.write(
        "main.tex",
        "\\documentclass{article}\n\\usepackage{graphicx}\n\\begin{document}\n\\includegraphics[width=1cm]{image.png}\n\\end{document}\n",
    );
    let mut header = b"\x89PNG\r\n\x1a\n".to_vec();
    append_chunk(
        &mut header,
        b"IHDR",
        &[0, 0, 0, 1, 0, 0, 0, 1, 8, 2, 0, 0, 0],
    );
    let mut valid = header.clone();
    append_chunk(&mut valid, b"PLTE", &[0, 0, 0]);
    // Zlib stream for one filter byte and one black RGB pixel.
    append_chunk(
        &mut valid,
        b"IDAT",
        &[120, 156, 99, 96, 96, 96, 0, 0, 0, 4, 0, 1],
    );
    append_chunk(&mut valid, b"IEND", &[]);
    fs::write(project.0.join("image.png"), valid).unwrap();
    project.success(&["build", "main.tex", "--report-json", "--once", "--force"]);
    assert!(project.0.join("build/main.pdf").is_file());

    // Reuse this prepared disk fixture. A transparent pixel forces the actual
    // decoder route, unlike opaque RGB PNGs that can copy their IDAT stream.
    let mut transparent = b"\x89PNG\r\n\x1a\n".to_vec();
    append_chunk(
        &mut transparent,
        b"IHDR",
        &[0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0],
    );
    append_chunk(
        &mut transparent,
        b"IDAT",
        &[120, 156, 99, 96, 96, 96, 104, 0, 0, 0, 133, 0, 129],
    );
    append_chunk(&mut transparent, b"IEND", &[]);
    project.write_bytes("transparent.png", &transparent);
    project.write(
        "profile.tex",
        "\\documentclass{article}\n\\usepackage{graphicx}\n\\pdftrailerid{<00112233445566778899AABBCCDDEEFF><00112233445566778899AABBCCDDEEFF>}\n\\begin{document}\nProfile probe. \\includegraphics[width=1cm]{transparent.png}\n\\end{document}\n",
    );
    let engine_args = [
        "__tekai-engine",
        "-interaction=nonstopmode",
        "-halt-on-error",
        "-no-shell-escape",
        "-output-directory=build",
        "profile.tex",
    ];
    let run = |setting| {
        project.run_with_env(
            &engine_args,
            &[
                ("SOURCE_DATE_EPOCH", "0"),
                ("FORCE_SOURCE_DATE", "1"),
                ("TEKAI_ENGINE_PROFILE", setting),
            ],
        )
    };
    let plain = run("0");
    assert!(plain.status.success(), "{plain:?}");
    assert!(!String::from_utf8_lossy(&plain.stderr).contains("TEKAI_ENGINE_PROFILE "));
    let plain_pdf = fs::read(project.0.join("build/profile.pdf")).unwrap();
    for setting in ["true", "1", "1"] {
        let output = run(setting);
        assert!(output.status.success(), "{output:?}");
        let stderr = String::from_utf8(output.stderr).unwrap();
        let records = stderr
            .lines()
            .filter_map(|line| line.strip_prefix("TEKAI_ENGINE_PROFILE "))
            .collect::<Vec<_>>();
        assert!(!stderr.contains("TEKAI_PROFILE "), "{stderr}");
        assert_eq!(records.len(), usize::from(setting == "1"), "{stderr}");
        if let Some(record) = records.first() {
            assert!(record.len() + "TEKAI_ENGINE_PROFILE ".len() < 4096);
            let record: Value = serde_json::from_str(record).unwrap();
            assert_eq!(record["schema_version"], 1);
            assert_eq!(record["scope"], "engine_invocation");
            assert_eq!(record["status"], "success");
            let phases = record["phases"].as_array().unwrap();
            assert_eq!(phases.len(), 7);
            let lifecycle = &phases[..5];
            assert!(
                lifecycle
                    .iter()
                    .all(|phase| phase["calls"] == 1 && phase["failed_calls"] == 0)
            );
            assert_eq!(
                lifecycle
                    .iter()
                    .map(|phase| phase["elapsed_ns"].as_u64().unwrap())
                    .sum::<u64>(),
                record["elapsed_ns"].as_u64().unwrap()
            );
            let counters = record["counters"].as_array().unwrap();
            let count = |name| {
                counters
                    .iter()
                    .find(|counter| counter["name"] == name)
                    .unwrap()["value"]
                    .as_u64()
                    .unwrap()
            };
            assert_eq!(count("decode_calls"), 1);
            assert!(count("decoded_bytes") > 0);
            assert!(count("compression_calls") > 0);
            assert!(count("compression_input_bytes") > 0);
            assert!(count("compression_output_bytes") > 0);
            assert!(
                record["worker_preparation"]["elapsed_ns"]
                    .as_u64()
                    .is_some()
            );
        }
        assert_eq!(
            fs::read(project.0.join("build/profile.pdf")).unwrap(),
            plain_pdf
        );
    }
    assert_eq!(
        fs::read(project.0.join("transparent.png")).unwrap(),
        transparent
    );
    project.write("profile-fatal.tex", "\\errmessage{profile fatal probe}\n");
    let fatal = project.run_with_env(
        &[
            "__tekai-engine",
            "-interaction=nonstopmode",
            "-halt-on-error",
            "profile-fatal.tex",
        ],
        &[("TEKAI_ENGINE_PROFILE", "1")],
    );
    assert_eq!(fatal.status.code(), Some(1), "{fatal:?}");
    let stderr = String::from_utf8(fatal.stderr).unwrap();
    let records = stderr
        .lines()
        .filter_map(|line| line.strip_prefix("TEKAI_ENGINE_PROFILE "))
        .collect::<Vec<_>>();
    assert_eq!(records.len(), 1, "{stderr}");
    assert_eq!(
        serde_json::from_str::<Value>(records[0]).unwrap()["status"],
        "errors"
    );

    for (typ, length, expected) in [
        (b"PLTE", 0u32, "invalid PNG PLTE length"),
        (b"PLTE", 4, "invalid PNG PLTE length"),
        (b"PLTE", 769, "invalid PNG PLTE length"),
        (b"PLTE", 24 * 1024 * 1024, "invalid PNG PLTE length"),
        (
            b"tRNS",
            24 * 1024 * 1024,
            "invalid or duplicate PNG tRNS length",
        ),
    ] {
        let mut invalid = header.clone();
        invalid.extend_from_slice(&length.to_be_bytes());
        invalid.extend_from_slice(typ);
        // Declared lengths are large, but the fixture has no chunk payload.
        fs::write(project.0.join("image.png"), invalid).unwrap();
        let output = project.run(&["build", "main.tex", "--report-json", "--once", "--force"]);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
        // TeX wraps diagnostics at a fixed column, including within words.
        // The fixture's path length must not change the asserted error text.
        let diagnostic: String = log.split_ascii_whitespace().collect();
        let expected: String = expected.split_ascii_whitespace().collect();
        assert!(diagnostic.contains(&expected), "{log}");
    }
}

#[test]
fn check_builds_and_caches_without_external_tools_or_tex_installation() {
    let project = Project::new();
    project.write(
        "main.tex",
        r"\documentclass{article}
\usepackage{amsmath,amsthm,graphicx,hyperref}
\begin{document}
\typeout{BOUND-MATCH=\pdfmatch subcount 2147483647{(a)}{a}}
\typeout{BOUND-CAPTURE-0=\pdflastmatch0}
\typeout{BOUND-CAPTURE-1=\pdflastmatch1}
\typeout{BOUND-CAPTURE-2=\pdflastmatch2}
\typeout{BOUND-CAPTURE-LAST=\pdflastmatch2147483646}
\typeout{BOUND-ZERO=\pdfmatch subcount 0{(a)}{a}}
\typeout{BOUND-ZERO-CAPTURE=\pdflastmatch0}
\section{Test}\label{sec:test}
\input{sections/body}
\end{document}
",
    );
    project.write(
        "sections/body.tex",
        "See Section \\ref{sec:test}. \\(x^2\\).\n",
    );
    let report = project.success(&["check", "main.tex", "--report-json"]);
    assert_eq!(report["error_count"], 0);
    assert_eq!(report["warning_count"], 0);
    assert!(report["tex_runs"].as_u64().unwrap() > 0);
    assert!(Path::new(report["pdf_path"].as_str().unwrap()).is_file());
    let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    for result in [
        "BOUND-MATCH=1",
        "BOUND-CAPTURE-0=0->a",
        "BOUND-CAPTURE-1=0->a",
        "BOUND-CAPTURE-2=-1->",
        "BOUND-CAPTURE-LAST=-1->",
        "BOUND-ZERO=1",
        "BOUND-ZERO-CAPTURE=-1->",
    ] {
        assert!(log.contains(result), "{log}");
    }
    assert!(
        fs::read(project.0.join("build/main.pdf"))
            .unwrap()
            .starts_with(b"%PDF-")
    );
    let cached = project.success(&["check", "main.tex", "--report-json"]);
    assert_eq!(cached["skipped"], true);
    let built = project.success(&["build", "main.tex", "--report-json", "--force"]);
    assert_eq!(built["skipped"], false);
    project.success(&["check", "main.tex", "--report-json", "--fast"]);

    project.write("sections/body.tex", "Inline $x$.\n");
    for extra in [vec![], vec!["--report-json"]] {
        let mut args = vec!["check", "main.tex"];
        args.extend(extra);
        assert_eq!(project.run(&args).status.code(), Some(1));
    }
    let warnings = project.success(&["check", "main.tex", "--report-json", "--allow-warnings"]);
    assert!(warnings["warning_count"].as_u64().unwrap() > 0);
    let fixed = project.success(&["check", "main.tex", "--report-json", "--fix"]);
    assert_eq!(fixed["warning_count"], 0);
    assert_eq!(
        fs::read_to_string(project.0.join("sections/body.tex")).unwrap(),
        "Inline \\(x\\).\n"
    );
}

#[cfg(unix)]
#[test]
fn bundled_cache_observes_source_edits_with_preserved_mtime() {
    use std::fs::{File, FileTimes};

    let project = Project::new();
    let source = |word| {
        format!("\\documentclass{{article}}\n\\begin{{document}}\n{word}\n\\end{{document}}\n")
    };
    let args = ["build", "main.tex", "--report-json"];
    project.write("main.tex", &source("OLD"));
    project.success(&args);
    assert_eq!(project.success(&args)["skipped"], true);

    let main = project.0.join("main.tex");
    let modified = fs::metadata(&main).unwrap().modified().unwrap();
    for (atomic, word) in [(false, "NEW"), (true, "NEW-LONGER")] {
        let destination = if atomic {
            project.0.join("replacement.tex")
        } else {
            main.clone()
        };
        fs::write(&destination, source(word)).unwrap();
        File::open(&destination)
            .unwrap()
            .set_times(FileTimes::new().set_modified(modified))
            .unwrap();
        if atomic {
            fs::rename(destination, &main).unwrap();
        }
        let built = project.success(&args);
        assert_eq!(built["skipped"], false);
        assert!(built["tex_runs"].as_u64().unwrap() > 0);
        assert_eq!(project.success(&args)["skipped"], true);
    }

    // An identical replacement changes inode but can retain the content hit.
    let replacement = project.0.join("identical.tex");
    fs::copy(&main, &replacement).unwrap();
    File::open(&replacement)
        .unwrap()
        .set_times(FileTimes::new().set_modified(modified))
        .unwrap();
    fs::rename(replacement, main).unwrap();
    assert_eq!(project.success(&args)["skipped"], true);
}

#[cfg(unix)]
#[test]
fn bundled_cache_observes_shared_database_replacement_with_preserved_mtime() {
    use std::fs::{File, FileTimes};

    let project = Project::new();
    let shared = Project::new();
    for (directory, word) in [("z", "OLD"), ("a", "NEW")] {
        shared.write(
            &format!("tex/latex/{directory}/auditchoice.sty"),
            &format!("\\ProvidesPackage{{auditchoice}}\n\\newcommand{{\\choiceword}}{{{word}}}\n"),
        );
    }
    shared.write("ls-R", "./tex/latex/z:\nauditchoice.sty\n");
    project.write(
        "tekai.toml",
        &format!(
            "[build.env]\nTEKAI_TEXMF_MODE = 'shared'\nTEXMFHOME = ''\nTEXMFLOCAL = '{}'\n",
            shared.0.display()
        ),
    );
    project.write(
        "main.tex",
        "\\documentclass{article}\n\\usepackage{auditchoice}\n\\begin{document}\n\\choiceword\n\\end{document}\n",
    );
    let args = ["build", "main.tex", "--report-json"];
    project.success(&args);
    assert_eq!(project.success(&args)["skipped"], true);
    let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    assert!(
        log.split_whitespace()
            .collect::<String>()
            .contains("tex/latex/z/auditchoice.sty")
    );

    let database = shared.0.join("ls-R");
    let initial = fs::metadata(&database).unwrap();
    let replacement = shared.0.join("ls-R.replacement");
    fs::write(&replacement, "./tex/latex/a:\nauditchoice.sty\n").unwrap();
    File::open(&replacement)
        .unwrap()
        .set_times(FileTimes::new().set_modified(initial.modified().unwrap()))
        .unwrap();
    fs::rename(replacement, &database).unwrap();
    let current = fs::metadata(database).unwrap();
    assert_eq!(current.len(), initial.len());
    assert_eq!(current.modified().unwrap(), initial.modified().unwrap());
    let built = project.success(&args);
    assert_eq!(built["skipped"], false);
    assert!(built["tex_runs"].as_u64().unwrap() > 0);
    let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    assert!(
        log.split_whitespace()
            .collect::<String>()
            .contains("tex/latex/a/auditchoice.sty")
    );
    assert_eq!(project.success(&args)["skipped"], true);
}

#[test]
#[ignore = "large-paper gate; CI runs it explicitly before installing optional TeX tools"]
fn bundled_runtime_builds_large_papers_without_external_tools() {
    for (fixture, pages) in [("arXiv-2605.26379v1", 48), ("arXiv-2511.08544v3", 50)] {
        let project = Project::new();
        let main = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("examples")
            .join(fixture)
            .join("main.tex");
        let report = project.success(&[
            "build",
            main.to_str().unwrap(),
            "--out-dir",
            "build",
            "--report-json",
        ]);
        assert_eq!(report["external_runs"], 0);
        assert!(report["bibliography_runs"].as_u64().unwrap() > 0);
        assert!(project.0.join("build/main.bbl").is_file());
        let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
        assert!(
            log.contains(&format!("({pages} pages,")),
            "{fixture}: wrong page count"
        );
    }
}

#[test]
fn check_loads_paper_packages_and_font_outlines_without_external_tools() {
    let project = Project::new();
    project.write(
        "main.tex",
        r"\documentclass{article}
\usepackage{newpxtext,newpxmath}
\usepackage{natbib,hyperref,microtype,graphicx,subfigure,booktabs}
\usepackage[toc,page,header]{appendix}
\usepackage{minitoc,makecell,dblfloatfix,cleveref,mathtools,cuted}
\usepackage[most]{tcolorbox}
\usepackage{bbm,varwidth,algpseudocode,mdframed,multirow,setspace,colortbl,framed}
\begin{document}
\section{Check}\label{sec:check}
\begin{figure}\caption{Bundled caption.}\end{figure}
\begin{tcolorbox}[enhanced,breakable]
Bundled fonts and packages: \(\mathbbm{1} + \frac{1}{2}\).
\end{tcolorbox}
\begin{mdframed}A framed paragraph.\end{mdframed}
\end{document}
",
    );
    project.success(&["check", "main.tex", "--report-json", "--allow-warnings"]);
    let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    assert!(log.contains("bbm10"), "BBM must load its bundled font");
    assert!(
        log.contains(".pfb"),
        "Text must embed a bundled outline font"
    );

    drop(project);
    let project = Project::new();
    project.write(
        "main.tex",
        r"\documentclass{article}
\usepackage{times,textcomp}
\begin{document}
Times text and a textcomp symbol: {\fontfamily{cmr}\selectfont\textcurrency}.
\end{document}
",
    );
    project.success(&["check", "main.tex", "--report-json", "--force"]);
    let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    // TeX wraps long absolute font paths, including within the filename.
    let compact_log = log.split_whitespace().collect::<String>();
    assert!(
        compact_log.contains("utmr8a.pfb"),
        "Times must embed its URW outline"
    );
    assert!(
        compact_log.contains("sfrm1000.pfb"),
        "Textcomp must use a CM-Super outline"
    );
}

#[test]
fn check_loads_conference_overlays_and_times_font_shapes_without_external_tools() {
    let project = Project::new();
    project.write(
        "main.tex",
        r"\documentclass{article}
\usepackage{eso-pic,fancyhdr,natbib,times}
\pagestyle{fancy}
\fancyhead{}
\AddToShipoutPicture{\AtTextUpperLeft{\put(0,0){\rule{1pt}{1pt}}}}
\begin{document}
{\LARGE\scshape Conference title\typeout{TEKAI-SMALLCAPS: \fontname\font}\par}
{\bfseries Anonymous authors\typeout{TEKAI-BOLD: \fontname\font}\par}
{\itshape Italic text\typeout{TEKAI-ITALIC: \fontname\font}\par}
Regular text.
\end{document}
",
    );
    let report = project.success(&["check", "main.tex", "--report-json"]);
    assert_eq!(report["external_runs"], 0);
    let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    for shape in ["SMALLCAPS: ptmrc7t", "BOLD: ptmb7t", "ITALIC: ptmri7t"] {
        assert!(log.contains(shape), "Missing Times shape {shape}: {log}");
    }
    assert!(!log.contains("LaTeX Font Warning"), "{log}");
    let compact_log = log.split_whitespace().collect::<String>();
    for outline in ["utmr8a.pfb", "utmb8a.pfb", "utmri8a.pfb"] {
        assert!(compact_log.contains(outline), "Missing outline {outline}");
    }
}

#[test]
fn check_resolves_recursive_project_search_paths_without_kpsewhich() {
    let project = Project::new();
    project.write("paper/main.tex", "\\documentclass{article}\n\\usepackage{sharedpkg}\n\\begin{document}\n\\input{shared}\n\\end{document}\n");
    project.write(
        "shared/deep/sharedpkg.sty",
        "\\ProvidesPackage{sharedpkg}\n\\newcommand{\\sharedword}{Shared text.}\n",
    );
    project.write("shared/deep/shared.tex", "\\sharedword\n");
    project.write(
        "paper/tekai.toml",
        "[build.env]\nTEXINPUTS = '../shared//:'\n",
    );
    let report = project.success(&["check", "paper/main.tex", "--report-json"]);
    assert_eq!(report["error_count"], 0);
    assert_eq!(report["warning_count"], 0);
    project.write("shared/deep/shared.tex", "Shared $x$.\n");
    let output = project.run(&["check", "paper/main.tex", "--report-json"]);
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(report["diagnostics"].as_array().unwrap().iter().any(|d| {
        d["path"]
            .as_str()
            .unwrap()
            .ends_with("shared/deep/shared.tex")
    }));
}

#[test]
fn check_runs_bibtex_in_process_with_bundled_styles() {
    let project = Project::new();
    project.write("main.tex", "\\documentclass{article}\n\\begin{document}\nSee \\cite{sample}.\n\\bibliographystyle{plain}\n\\bibliography{refs}\n\\end{document}\n");
    project.write("refs.bib", "@article{sample, author={Ada Example}, title={A bundled bibliography}, journal={Testing}, year={2026}}\n");
    let report = project.success(&["check", "main.tex", "--report-json"]);
    assert!(report["bibliography_runs"].as_u64().unwrap() > 0);
    assert!(
        fs::read_to_string(project.0.join("build/main.bbl"))
            .unwrap()
            .contains("bundled bibliography")
    );
}

#[cfg(unix)]
#[test]
fn declared_eps_conversion_uses_the_scheduled_pdf_without_shell_escape() {
    use std::os::unix::fs::PermissionsExt;
    let project = Project::new();
    project.write(
        "fixture.tex",
        "\\documentclass{article}\n\\begin{document}\nConverted graphic.\n\\end{document}\n",
    );
    project.success(&["build", "fixture.tex", "--report-json"]);
    fs::copy(
        project.0.join("build/fixture.pdf"),
        project.0.join("fig.pdf"),
    )
    .unwrap();
    project.write(
        "main.tex",
        r"\documentclass{article}
\usepackage{graphicx}
\DeclareGraphicsExtensions{.eps,.pdf}
\begin{document}
\includegraphics[width=1cm]{fig}
\end{document}
",
    );
    project.write(
        "fig.eps",
        "%!PS-Adobe-3.0 EPSF-3.0\n%%BoundingBox: 0 0 100 100\n",
    );
    project.write(
        "empty-bin/epstopdf",
        r#"#!/bin/sh
set -eu
for arg in "$@"; do
  case "$arg" in
    --outfile=*) /bin/cp build/fixture.pdf "${arg#--outfile=}" ;;
  esac
done
printf invoked >> build/epstopdf-invocations
"#,
    );
    fs::set_permissions(
        project.0.join("empty-bin/epstopdf"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();

    let disabled = project.run(&["build", "main.tex", "--report-json"]);
    assert!(!disabled.status.success());
    assert!(String::from_utf8_lossy(&disabled.stderr).contains("--external-tools"));
    assert!(!project.0.join("build/epstopdf-invocations").exists());

    let report = project.success(&["build", "main.tex", "--report-json", "--external-tools"]);
    assert_eq!(report["external_runs"], 1);
    assert!(project.0.join("build/fig-eps-converted-to.pdf").is_file());
    let log = fs::read_to_string(project.0.join("build/main.log")).unwrap();
    assert!(log.contains("fig-eps-converted-to.pdf"), "{log}");
    assert!(
        !log.contains("runsystem("),
        "TeX must not run a converter: {log}"
    );
    assert_eq!(
        fs::read_to_string(project.0.join("build/epstopdf-invocations")).unwrap(),
        "invoked"
    );
    let cached = project.success(&["build", "main.tex", "--report-json", "--external-tools"]);
    assert_eq!(cached["external_runs"], 0);
    assert_eq!(cached["skipped"], true);
}

#[cfg(unix)]
#[test]
fn check_never_launches_installed_lookup_or_auxiliary_tools_by_default() {
    use std::os::unix::fs::PermissionsExt;
    let project = Project::new();
    for tool in [
        "kpsewhich",
        "pdflatex",
        "latexmk",
        "mlatex",
        "bibtex",
        "biber",
        "makeindex",
        "python3",
        "python",
    ] {
        let path = project.0.join("empty-bin").join(tool);
        fs::write(
            &path,
            "#!/bin/sh\nprintf invoked > tool-was-invoked\nexit 99\n",
        )
        .unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    }
    project.write("main.tex", "\\documentclass{article}\n\\begin{document}\n\\immediate\\write18{python3}\nNo external commands.\n\\end{document}\n");
    project.success(&["check", "main.tex", "--report-json"]);
    assert!(!project.0.join("tool-was-invoked").exists());
    assert!(!project.0.join("build/tool-was-invoked").exists());
    project.write("main.tex", "\\documentclass{article}\n\\usepackage{makeidx}\n\\makeindex\n\\begin{document}\nTest\\index{Test}.\n\\printindex\n\\end{document}\n");
    let output = project.run(&["check", "main.tex", "--report-json"]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("--external-tools"),
        "{output:?}"
    );
    assert!(!project.0.join("tool-was-invoked").exists());
    assert!(!project.0.join("build/tool-was-invoked").exists());

    // Opting in must reach the installed helper, rather than silently ignoring
    // either the requested auxiliary workflow or the user's explicit choice.
    let output = project.run(&["check", "main.tex", "--report-json", "--external-tools"]);
    assert!(!output.status.success());
    assert!(project.0.join("build/tool-was-invoked").is_file());
}

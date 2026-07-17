use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tekai::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const NATIVE_DOC: &str = r#"\documentclass{article}
\pdfcompresslevel=0
\pdfobjcompresslevel=0
\title{Native Smoke}
\newcommand{\nativeword}{native macro}
\begin{document}
\maketitle
\section{Hello}
\label{sec:hello}
This tiny document is rendered by the \nativeword backend. See Section~\ref{sec:hello}.
\end{document}
"#;

const FALLBACK_DOC: &str = r#"\documentclass{article}
\pdfcompresslevel=0
\pdfobjcompresslevel=0
\begin{document}
Fallback through a primitive special.
\special{pdf:literal direct 0 0 m}
\end{document}
"#;

const FALLBACK_BIB_DOC: &str = r#"\documentclass{article}
\pdfcompresslevel=0
\pdfobjcompresslevel=0
\begin{document}
Fallback citation \cite{knuth}.
\special{pdf:literal direct 0 0 m}
\bibliographystyle{plain}
\bibliography{refs}
\end{document}
"#;

const NATIVE_BIB_DOC: &str = r#"\documentclass{article}
\pdfcompresslevel=0
\pdfobjcompresslevel=0
\usepackage{graphicx}
\begin{document}
Native citation \cite{knuth}.
\bibliographystyle{plain}
\bibliography{refs}
\end{document}
"#;

const NATIVE_TOC_DOC: &str = r#"\documentclass{article}
\pdfcompresslevel=0
\pdfobjcompresslevel=0
\begin{document}
\tableofcontents
\section{Intro}
Native table of contents.
\subsection{Details}
More native table of contents.
\section*{Acknowledgments}
\addcontentsline{toc}{section}{Acknowledgments}
Unnumbered native table of contents.
\appendix
\section{Supplement}
Native appendix table of contents.
\subsection{Extra Details}
More appendix table of contents.
\end{document}
"#;

const NATIVE_FLOAT_LIST_DOC: &str = r#"\documentclass{article}
\pdfcompresslevel=0
\pdfobjcompresslevel=0
\begin{document}
\listoffigures
\listoftables
\begin{figure}
\caption[Short figure]{Long figure caption.}
\label{fig:native}
\end{figure}
Figure ref \ref{fig:native}.
\begin{table}
\caption[Short table]{Long table caption.}
\label{tab:native}
\end{table}
Table ref \ref{tab:native}.
\end{document}
"#;

const NATIVE_HYPERREF_DOC: &str = r#"\documentclass{article}
\pdfcompresslevel=0
\pdfobjcompresslevel=0
\usepackage[pagebackref=true]{hyperref}
\begin{document}
\pdfbookmark[1]{Front Matter}{front:matter}
\section{Intro}
\label{sec:intro}
Follow \href{https://example.com}{Example Site}, \url{https://example.com/raw},
and \hyperref[sec:intro]{the intro}.
Native citation \cite{knuth}.
\subsection{Details}
Autoref \autoref{sec:intro}.
\bibliographystyle{plain}
\bibliography{refs}
\end{document}
"#;

const NATIVE_INDEX_DOC: &str = r#"\documentclass{article}
\pdfcompresslevel=0
\pdfobjcompresslevel=0
\usepackage{makeidx}
\makeindex
\begin{document}
Native alpha\index{alpha@Alpha entry}.
Native beta\index{beta!sub item|textbf}.
\printindex
\end{document}
"#;

const NATIVE_PDF_METADATA_DOC: &str = r#"\documentclass{article}
\pdfcompresslevel=0
\pdfobjcompresslevel=0
\usepackage{hyperref}
\pdfinfo{/Title (Primitive Title) /Author (Ada Native)}
\hypersetup{pdfsubject={Native Subject}, pdfkeywords={alpha, beta}}
\pdfcatalog{/PageMode /UseNone}
\begin{document}
Native metadata body.
\end{document}
"#;

const NATIVE_PDFTEX_PRIMITIVES_DOC: &str = r#"\documentclass{article}
\pdfcompresslevel=0
\pdfobjcompresslevel=0
\usepackage{pdftexcmds}
\ifx\pdfoutput\undefined
\newcommand{\pdfmode}{DVI}
\else
\newcommand{\pdfmode}{PDF}
\fi
\pdfminorversion=5
\pdfpagewidth=8.5 true in
\pdfpageattr{/Rotate 0}
\ifpdfprimitive\pdfstrcmp
\newcommand{\pdfprimprobe}{Primitive probe.}
\else
\newcommand{\pdfprimprobe}{Missing primitive probe.}
\fi
\ifpdfprimitive\pdfoutput
\newcommand{\pdfregprobe}{Registerprobe.}
\else
\newcommand{\pdfregprobe}{Missing register primitive.}
\fi
\ifnum\pdftexversion>139
\newcommand{\pdfversionprobe}{Version primitive.}
\else
\newcommand{\pdfversionprobe}{Missing version primitive.}
\fi
\ifpdfprimitive\pdffilesize
\newcommand{\pdffileprimprobe}{File primitive.}
\else
\newcommand{\pdffileprimprobe}{Missing file primitive.}
\fi
\ifnum\pdffilesize{native-data.txt}=3
\newcommand{\pdffilesizeprobe}{File size.}
\else
\newcommand{\pdffilesizeprobe}{Wrong file size.}
\fi
\edef\nativehex{\pdfescapehex{Native}}
\ifnum\pdfstrcmp{\pdfunescapehex{\nativehex}}{Native}=0
\newcommand{\pdfprimdecode}{Hex roundtrip.}
\else
\newcommand{\pdfprimdecode}{Broken hex.}
\fi
\let\nativepdfprimitive\pdfprimitive
\edef\nativeprimhex{\nativepdfprimitive\pdfescapehex{Probe}}
\ifnum\nativepdfprimitive\pdfstrcmp{\pdfunescapehex{\nativeprimhex}}{Probe}=0
\newcommand{\pdfprimitivealiasprobe}{Primitive alias.}
\else
\newcommand{\pdfprimitivealiasprobe}{Broken primitive alias.}
\fi
\edef\nativehash{\pdfmdfivesum{abc}}
\edef\nativefilehash{\pdfmdfivesum file {native-data.txt}}
\ifnum\pdfstrcmp{\nativefilehash}{900150983CD24FB0D6963F7D28E17F72}=0
\newcommand{\pdffilehashprobe}{File hash.}
\else
\newcommand{\pdffilehashprobe}{Broken file hash.}
\fi
\edef\nativedirectfiledump{\pdffiledump offset 1 length 2 {native-data.txt}}
\ifnum\pdfstrcmp{\nativedirectfiledump}{6263}=0
\newcommand{\pdffiledumpprobe}{File dump.}
\else
\newcommand{\pdffiledumpprobe}{Broken file dump.}
\fi
\makeatletter
\edef\nativepkgsize{\pdf@filesize{native-data.txt}}
\edef\nativepkgdump{\pdf@filedump{1}{2}{native-data.txt}}
\edef\nativepkghash{\pdf@filemdfivesum{native-data.txt}}
\edef\nativepkghex{\pdf@escapehex{Native}}
\ifnum\pdf@strcmp{\nativepkgsize}{3}=0
\newcommand{\pdfcmdsizeprobe}{Package filesize.}
\else
\newcommand{\pdfcmdsizeprobe}{Broken package filesize.}
\fi
\ifnum\pdf@strcmp{\nativepkgdump}{6263}=0
\newcommand{\pdfcmddumpprobe}{Package filedump.}
\else
\newcommand{\pdfcmddumpprobe}{Broken package filedump.}
\fi
\ifnum\pdf@strcmp{\nativepkghash}{900150983CD24FB0D6963F7D28E17F72}=0
\newcommand{\pdfcmdhashprobe}{Package filehash.}
\else
\newcommand{\pdfcmdhashprobe}{Broken package filehash.}
\fi
\ifnum\pdf@strcmp{\pdf@unescapehex{\nativepkghex}}{Native}=0
\newcommand{\pdfcmdhexprobe}{Package hex.}
\else
\newcommand{\pdfcmdhexprobe}{Broken package hex.}
\fi
\ifnum\pdf@shellescape=0
\newcommand{\pdfcmdshellprobe}{Package shellescape.}
\else
\newcommand{\pdfcmdshellprobe}{Wrong package shellescape.}
\fi
\pdf@setdraftmode{1}
\pdf@ifdraftmode{\newcommand{\pdfcmddraftprobe}{Package draftmode.}}{\newcommand{\pdfcmddraftprobe}{Broken package draftmode.}}
\pdf@setdraftmode{0}
\ifnum\pdf@elapsedtime<0
\newcommand{\pdfcmdtimerprobe}{Broken package timer.}
\else
\newcommand{\pdfcmdtimerprobe}{Package timer.}
\fi
\pdf@ifprimitive\pdfstrcmp
\newcommand{\pdfcmdprimitiveprobe}{Pkg primitive.}
\else
\newcommand{\pdfcmdprimitiveprobe}{Broken package primitive alias.}
\fi
\makeatother
\edef\nativefiledate{\pdffilemoddate{native-data.txt}}
\ifnum\pdfstrcmp{\nativefiledate}{}>0
\newcommand{\pdffiledateprobe}{File date.}
\else
\newcommand{\pdffiledateprobe}{Missing file date.}
\fi
\edef\nativecreationdate{\pdfcreationdate}
\ifnum\pdfstrcmp{\nativecreationdate}{}>0
\newcommand{\pdfcreationdateprobe}{Creation date.}
\else
\newcommand{\pdfcreationdateprobe}{Missing creation date.}
\fi
\begin{document}
Mode \pdfmode.
\ifnum\pdfminorversion=5 Minor five.\else Wrong minor.\fi
\ifdim\pdfpagewidth>8truein Wide page.\else Narrow page.\fi
\pdfprimdecode
\pdfprimprobe
\pdfregprobe
\pdfversionprobe
\pdfprimitivealiasprobe
\pdffileprimprobe
\pdffilesizeprobe
\pdffilehashprobe
\pdffiledumpprobe
\pdfcmdsizeprobe
\pdfcmddumpprobe
\pdfcmdhashprobe
\pdfcmdhexprobe
\pdfcmdshellprobe
\pdfcmddraftprobe
\pdfcmdtimerprobe
\pdfcmdprimitiveprobe
\pdffiledateprobe
\pdfcreationdateprobe
Hash \nativehash.
\end{document}
"#;

#[test]
fn tekai_pdftex_native_backend_builds_minimal_document() {
    let root = unique_temp_dir("tekai-pdftex-native");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, NATIVE_DOC).expect("failed to write TeX source");

    let report = build(&options(&main, &out_dir)).expect("native tekai-pdftex build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert_eq!(report.pdf_tex_runs, 1, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main.aux").exists());
    assert!(out_dir.join("main.fls").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("log should be readable");
    assert!(log.contains("This is pdfTeX"), "{log}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_certified_backend_uses_pdflatex_final_artifact() {
    if !command_available("pdflatex") {
        eprintln!("skipping certified tekai-pdftex test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-pdftex-certified");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, NATIVE_DOC).expect("failed to write TeX source");

    let mut options = options(&main, &out_dir);
    options.engine = Engine::TekaiPdftexCertified;
    let report = build(&options).expect("certified tekai-pdftex build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert!(report.pdf_tex_runs >= 1, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main.aux").exists());
    assert!(out_dir.join("main.fls").exists());
    let trace = fs::read_to_string(out_dir.join("main.tekai-pdftex.trace"))
        .expect("trace should be readable");
    assert!(trace.contains("tekai-pdftex-native"), "{trace}");
    assert!(
        trace.contains("certification_policy\tpdftex-final-oracle"),
        "{trace}"
    );
    assert!(
        trace.contains("certification_final_pdf\tpdflatex"),
        "{trace}"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_native_backend_handles_pdftex_primitive_registers() {
    let root = unique_temp_dir("tekai-pdftex-native-pdf-primitives");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, NATIVE_PDFTEX_PRIMITIVES_DOC).expect("failed to write TeX source");
    fs::write(root.join("native-data.txt"), b"abc").expect("failed to write native data file");

    let report =
        build(&options(&main, &out_dir)).expect("native tekai-pdftex primitive build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert_eq!(report.pdf_tex_runs, 1, "{report:#?}");
    assert_eq!(report.bibliography_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let pdf_text = pdf_visible_text(&out_dir.join("main.pdf"));
    let compact_pdf_text = compact_text(&pdf_text);
    for expected in [
        "Mode PDF.",
        "Minor five.",
        "Wide page.",
        "Hex roundtrip.",
        "Primitive probe.",
        "Registerprobe.",
        "Version primitive.",
        "Primitive alias.",
        "File primitive.",
        "File size.",
        "File dump.",
        "Package filesize.",
        "Package filedump.",
        "Package filehash.",
        "Package shellescape.",
        "Package draftmode.",
        "Package timer.",
        "Pkg primitive.",
        "Creation date.",
    ] {
        let compact_expected = compact_text(expected);
        assert!(compact_pdf_text.contains(&compact_expected), "{pdf_text}");
    }
    assert!(pdf_text.contains("hash."), "{pdf_text}");
    assert!(pdf_text.contains("hex."), "{pdf_text}");
    assert!(
        compact_pdf_text.contains(&compact_text("File date.Creation date.")),
        "{pdf_text}"
    );
    assert!(
        pdf_text.contains("900150983CD24FB0D6963F7D28E17F72"),
        "{pdf_text}"
    );
    assert!(!pdf_text.contains("Wrong minor"), "{pdf_text}");
    assert!(!pdf_text.contains("Narrow page"), "{pdf_text}");
    assert!(!pdf_text.contains("Broken hex"), "{pdf_text}");
    assert!(!pdf_text.contains("Missing primitive probe"), "{pdf_text}");
    assert!(
        !pdf_text.contains("Missing register primitive"),
        "{pdf_text}"
    );
    assert!(
        !pdf_text.contains("Missing version primitive"),
        "{pdf_text}"
    );
    assert!(!pdf_text.contains("Broken primitive alias"), "{pdf_text}");
    assert!(!pdf_text.contains("Missing file primitive"), "{pdf_text}");
    assert!(!pdf_text.contains("Wrong file size"), "{pdf_text}");
    assert!(!pdf_text.contains("Broken file hash"), "{pdf_text}");
    assert!(!pdf_text.contains("Broken file dump"), "{pdf_text}");
    assert!(!pdf_text.contains("Broken package filesize"), "{pdf_text}");
    assert!(!pdf_text.contains("Broken package filedump"), "{pdf_text}");
    assert!(!pdf_text.contains("Broken package filehash"), "{pdf_text}");
    assert!(!pdf_text.contains("Broken package hex"), "{pdf_text}");
    assert!(
        !pdf_text.contains("Wrong package shellescape"),
        "{pdf_text}"
    );
    assert!(!pdf_text.contains("Broken package draftmode"), "{pdf_text}");
    assert!(!pdf_text.contains("Broken package timer"), "{pdf_text}");
    assert!(
        !pdf_text.contains("Broken package primitive alias"),
        "{pdf_text}"
    );
    assert!(!pdf_text.contains("Missing file date"), "{pdf_text}");
    assert!(!pdf_text.contains("Missing creation date"), "{pdf_text}");
    let log = fs::read_to_string(out_dir.join("main.log")).expect("log should be readable");
    assert!(log.contains("This is pdfTeX"), "{log}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_native_backend_writes_table_of_contents() {
    let root = unique_temp_dir("tekai-pdftex-native-toc");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, NATIVE_TOC_DOC).expect("failed to write TeX source");

    let report = build(&options(&main, &out_dir)).expect("native tekai-pdftex ToC build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert_eq!(report.pdf_tex_runs, 1, "{report:#?}");
    assert_eq!(report.bibliography_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main.toc").exists());
    let pdf_text = pdf_visible_text(&out_dir.join("main.pdf"));
    for expected in [
        "Contents",
        "1 Intro",
        "1.1 Details",
        "Acknowledgments",
        "A Supplement",
        "A.1 Extra Details",
    ] {
        assert!(pdf_text.contains(expected), "{pdf_text}");
    }

    let cached = build(&options(&main, &out_dir)).expect("cached native ToC build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_native_backend_writes_float_lists() {
    let root = unique_temp_dir("tekai-pdftex-native-float-lists");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, NATIVE_FLOAT_LIST_DOC).expect("failed to write TeX source");

    let report =
        build(&options(&main, &out_dir)).expect("native tekai-pdftex float-list build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert_eq!(report.pdf_tex_runs, 1, "{report:#?}");
    assert_eq!(report.bibliography_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());

    assert!(out_dir.join("main.lof").exists());
    assert!(out_dir.join("main.lot").exists());
    let pdf_text = pdf_visible_text(&out_dir.join("main.pdf"));
    for expected in [
        "List of Figures",
        "Short figure",
        "List of Tables",
        "Short table",
    ] {
        assert!(pdf_text.contains(expected), "{pdf_text}");
    }
    let log = fs::read_to_string(out_dir.join("main.log")).expect("log should be readable");
    assert!(log.contains("main.lof"), "{log}");
    assert!(log.contains("main.lot"), "{log}");

    let cached = build(&options(&main, &out_dir)).expect("cached native float-list build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_native_backend_writes_index_sidecar() {
    let root = unique_temp_dir("tekai-pdftex-native-index");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, NATIVE_INDEX_DOC).expect("failed to write TeX source");

    let report = build(&options(&main, &out_dir)).expect("native tekai-pdftex index build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert_eq!(report.pdf_tex_runs, 1, "{report:#?}");
    assert_eq!(report.index_runs, 1, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main.idx").exists());
    let pdf_text = pdf_visible_text(&out_dir.join("main.pdf"));
    assert!(pdf_text.contains("Index"), "{pdf_text}");
    assert!(pdf_text.contains("Alpha entry, 1"), "{pdf_text}");
    assert!(
        compact_text(&pdf_text).contains(&compact_text("beta sub item, 1")),
        "{pdf_text}"
    );
    assert!(out_dir.join("main.ind").exists());

    let cached = build(&options(&main, &out_dir)).expect("cached native index build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_native_backend_writes_pdf_metadata() {
    let root = unique_temp_dir("tekai-pdftex-native-pdf-metadata");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, NATIVE_PDF_METADATA_DOC).expect("failed to write TeX source");

    let report =
        build(&options(&main, &out_dir)).expect("native tekai-pdftex metadata build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert!(report.pdf_tex_runs >= 1, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let raw_pdf_text = raw_pdf_text(&out_dir.join("main.pdf"));
    assert!(raw_pdf_text.contains("/Info"), "{raw_pdf_text}");
    assert!(raw_pdf_text.contains("/Title"), "{raw_pdf_text}");
    assert!(raw_pdf_text.contains("Primitive Title"), "{raw_pdf_text}");
    assert!(raw_pdf_text.contains("/Author"), "{raw_pdf_text}");
    assert!(raw_pdf_text.contains("Ada Native"), "{raw_pdf_text}");
    assert!(raw_pdf_text.contains("/Subject"), "{raw_pdf_text}");
    assert!(raw_pdf_text.contains("/Keywords"), "{raw_pdf_text}");
    let pdf_info = pdf_info_text(&out_dir.join("main.pdf"));
    assert!(
        pdf_info.contains("Subject:         Native Subject"),
        "{pdf_info}"
    );
    assert!(
        pdf_info.contains("Keywords:        alpha, beta"),
        "{pdf_info}"
    );
    let pdf_text = pdf_visible_text(&out_dir.join("main.pdf"));
    assert!(pdf_text.contains("Native metadata body."), "{pdf_text}");
    let log = fs::read_to_string(out_dir.join("main.log")).expect("log should be readable");
    assert!(log.contains("This is pdfTeX"), "{log}");

    let cached = build(&options(&main, &out_dir)).expect("cached native metadata build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_native_backend_writes_hyperref_out() {
    let root = unique_temp_dir("tekai-pdftex-native-hyperref-out");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, NATIVE_HYPERREF_DOC).expect("failed to write TeX source");
    fs::write(
        &refs,
        "@book{knuth, author={Donald Knuth}, title={The TeXbook}, year={1984}}\n",
    )
    .expect("failed to write bibliography");

    let report =
        build(&options(&main, &out_dir)).expect("native tekai-pdftex hyperref build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert!(report.pdf_tex_runs >= 1, "{report:#?}");
    assert_eq!(report.bibliography_runs, 1, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let raw_pdf_text = raw_pdf_text(&out_dir.join("main.pdf"));
    assert!(raw_pdf_text.contains("/Outlines"), "{raw_pdf_text}");
    assert!(raw_pdf_text.contains("/Title"), "{raw_pdf_text}");
    let out = fs::read_to_string(out_dir.join("main.out")).expect("out should be readable");
    assert!(out.contains("{section.1}"), "{out}");
    assert!(out.contains("{subsection.1.1}"), "{out}");
    let pdf_text = pdf_visible_text(&out_dir.join("main.pdf"));
    assert!(pdf_text.contains("Intro"), "{pdf_text}");

    assert!(out_dir.join("main.out").exists());
    assert!(out_dir.join("main.brf").exists());

    let cached = build(&options(&main, &out_dir)).expect("cached native hyperref build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_native_auto_mode_stays_single_pass_without_external_bibtex() {
    let root = unique_temp_dir("tekai-pdftex-native-single-pass");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, NATIVE_BIB_DOC).expect("failed to write TeX source");
    fs::write(
        &refs,
        "@book{knuth, author={Donald Knuth}, title={The TeXbook}, year={1984}}\n",
    )
    .expect("failed to write bibliography");

    let report = build(&options(&main, &out_dir)).expect("native tekai-pdftex build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert_eq!(report.draft_tex_runs, 0, "{report:#?}");
    assert!(report.final_tex_runs >= 1, "{report:#?}");
    assert_eq!(report.pdf_tex_runs, 1, "{report:#?}");
    assert_eq!(report.bibliography_runs, 1, "{report:#?}");
    assert!(!report.draft_prepass_used, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main.bbl").exists());

    let cached = build(&options(&main, &out_dir)).expect("cached native build failed");
    assert!(cached.skipped, "{cached:#?}");
    assert_eq!(cached.tex_runs, 0, "{cached:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_falls_back_to_real_pdflatex_for_unsupported_documents() {
    if !command_available("pdflatex") {
        eprintln!("skipping tekai-pdftex fallback test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("tekai-pdftex-fallback");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, FALLBACK_DOC).expect("failed to write TeX source");

    let report = build(&options(&main, &out_dir)).expect("fallback tekai-pdftex build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("log should be readable");
    assert!(log.contains("This is pdfTeX"), "{log}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_native_backend_writes_synctex_when_requested() {
    let root = unique_temp_dir("tekai-pdftex-native-synctex");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, NATIVE_DOC).expect("failed to write TeX source");

    let mut options = options(&main, &out_dir);
    options.synctex = true;
    let report = build(&options).expect("native tekai-pdftex SyncTeX build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert_eq!(report.pdf_tex_runs, 1, "{report:#?}");
    assert_eq!(report.bibliography_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(!out_dir.join("main.synctex.gz").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("log should be readable");
    assert!(log.contains("This is pdfTeX"), "{log}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn tekai_pdftex_fallback_preserves_pdflatex_bibtex_scheduler() {
    if !command_available("pdflatex") || !command_available("bibtex") {
        eprintln!("skipping tekai-pdftex fallback BibTeX test; pdflatex or bibtex is unavailable");
        return;
    }

    let root = unique_temp_dir("tekai-pdftex-fallback-bibtex");
    fs::create_dir_all(&root).expect("failed to create temp directory");
    let main = root.join("main.tex");
    let refs = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(&main, FALLBACK_BIB_DOC).expect("failed to write TeX source");
    fs::write(
        &refs,
        "@book{knuth, author={Donald Knuth}, title={The TeXbook}, year={1984}}\n",
    )
    .expect("failed to write bibliography");

    let report = build(&options(&main, &out_dir)).expect("fallback tekai-pdftex build failed");

    assert!(!report.skipped, "{report:#?}");
    assert!(report.tex_runs >= 1, "{report:#?}");
    assert_eq!(report.bibliography_runs, 1, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(out_dir.join("main.bbl").exists());
    let log = fs::read_to_string(out_dir.join("main.log")).expect("log should be readable");
    assert!(log.contains("This is pdfTeX"), "{log}");

    let _ = fs::remove_dir_all(root);
}

fn options(main: &Path, out_dir: &Path) -> BuildOptions {
    BuildOptions {
        main: main.to_path_buf(),
        job_name: None,
        engine: Engine::TekaiPdftex,
        runner: Runner::Direct,
        bib_mode: BibMode::Auto,
        out_dir: out_dir.to_path_buf(),
        fast: false,
        draft_prepass: DraftPrepass::Auto,
        once: false,
        max_runs: 8,
        force: false,
        precompile_preamble: false,
        synctex: false,
        shell_escape: false,
        quiet: true,
        print_command: false,
    }
}

fn command_available(program: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {program} >/dev/null 2>&1"))
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn pdf_visible_text(pdf: &Path) -> String {
    match Command::new("pdftotext")
        .arg("-layout")
        .arg(pdf)
        .arg("-")
        .output()
    {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).into_owned()
        }
        _ => raw_pdf_text(pdf),
    }
}

fn raw_pdf_text(pdf: &Path) -> String {
    String::from_utf8_lossy(&fs::read(pdf).expect("PDF should exist")).into_owned()
}

fn pdf_info_text(pdf: &Path) -> String {
    match Command::new("pdfinfo").arg(pdf).output() {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).into_owned()
        }
        _ => raw_pdf_text(pdf),
    }
}

fn compact_text(text: &str) -> String {
    text.replace("-\n", "").split_whitespace().collect()
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("{prefix}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    path
}

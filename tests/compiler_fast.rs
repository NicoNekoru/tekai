use std::fs;
use std::path::{Path, PathBuf};

use texpilot::compiler::{BibMode, BuildOptions, DraftPrepass, Engine, Runner, build};

const FINAL_GRAPHICX_MISSING_IMAGE_DOC: &str = r#"\documentclass{article}
\usepackage[final]{graphicx}
\begin{document}
Before \includegraphics[width=1cm]{missing-image} After.
\end{document}
"#;

const SVG_MISSING_IMAGE_DOC: &str = r#"\documentclass{article}
\usepackage{svg}
\begin{document}
Before \includesvg[width=1cm]{missing-svg} After.
\end{document}
"#;

const PDFPAGES_MISSING_DOC: &str = r#"\documentclass{article}
\usepackage{pdfpages}
\begin{document}
Before.
\includepdf[pages=-]{missing-document}
After.
\end{document}
"#;

const MINTED_DOC: &str = r#"\documentclass{article}
\usepackage{minted}
\begin{document}
Before.
\begin{minted}{python}
print("fast")
\end{minted}
After.
\end{document}
"#;

const INPUTMINTED_MISSING_DOC: &str = r#"\documentclass{article}
\usepackage{minted}
\begin{document}
Before.
\inputminted{python}{generated-snippet.py}
After.
\end{document}
"#;

const ANIMATE_MISSING_DOC: &str = r#"\documentclass{article}
\usepackage{animate}
\begin{document}
Before.
\animategraphics[controls,width=2cm]{12}{frames/frame-}{0}{4}
After.
\end{document}
"#;

const STANDALONE_MISSING_DOC: &str = r#"\documentclass{article}
\usepackage{standalone}
\begin{document}
Before.
\includestandalone[mode=buildnew,width=2cm]{figures/generated-figure}
After.
\end{document}
"#;

const MEDIA9_MISSING_DOC: &str = r#"\documentclass{article}
\usepackage{media9}
\begin{document}
Before.
\includemedia[width=2cm,height=1cm]{Preview poster}{videos/generated-demo.mp4}
After.
\end{document}
"#;

const ATTACHFILE2_MISSING_DOC: &str = r#"\documentclass{article}
\usepackage{attachfile2}
\begin{document}
Before.
\attachfile[description={Generated data}]{artifacts/generated-data.csv}
\textattachfile[color=0 0 1]{artifacts/generated-report.json}{report}
\notextattachfile{attachment label}
\noattachfile
After.
\end{document}
"#;

const ATTACHFILE_MISSING_DOC: &str = r#"\documentclass{article}
\usepackage{attachfile}
\begin{document}
Before.
\attachfile[description={Generated data}]{artifacts/generated-data.csv}
\textattachfile[color=0 0 1]{artifacts/generated-report.json}{report}
\notextattachfile{attachment label}
\noattachfile
After.
\end{document}
"#;

const TIKZ_EXTERNAL_DOC: &str = r#"\documentclass{article}
\usepackage{tikz}
\usetikzlibrary{external}
\tikzexternalize
\begin{document}
Before.
\begin{tikzpicture}
\draw[blue, thick] (0,0) circle (1cm);
\end{tikzpicture}
After.
\end{document}
"#;

#[test]
fn fast_preview_uses_demo_graphics_even_when_document_requests_final_graphicx() {
    if !command_available("pdflatex") {
        eprintln!("skipping fast graphicx test; pdflatex is not available");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-graphicx-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, FINAL_GRAPHICX_MISSING_IMAGE_DOC).expect("failed to write test document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should not load missing graphics");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_precompile_preamble_reuses_format_after_body_edit() {
    if !command_available("pdflatex") || !tex_file_available("mylatexformat.ltx") {
        eprintln!("skipping precompiled preamble test; pdflatex or mylatexformat is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-precompile-preamble-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nHello.\n\\end{document}\n",
    )
    .expect("failed to write test document");

    let first = build(&BuildOptions {
        fast: true,
        once: true,
        precompile_preamble: true,
        ..options(&main, &out_dir)
    })
    .expect("first fast preview should build a precompiled preamble");

    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(first.preamble_format_used, "{first:#?}");
    assert!(first.preamble_format_built, "{first:#?}");

    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nHello again.\n\\end{document}\n",
    )
    .expect("failed to edit document body");

    let second = build(&BuildOptions {
        fast: true,
        once: true,
        precompile_preamble: true,
        ..options(&main, &out_dir)
    })
    .expect("body edit should reuse the precompiled preamble");

    assert_eq!(second.tex_runs, 1, "{second:#?}");
    assert!(second.preamble_format_used, "{second:#?}");
    assert!(!second.preamble_format_built, "{second:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_precompile_preamble_falls_back_for_predocument_inputs() {
    if !command_available("pdflatex") || !tex_file_available("mylatexformat.ltx") {
        eprintln!(
            "skipping unsafe precompiled preamble test; pdflatex or mylatexformat is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("texpilot-fast-unsafe-precompile-preamble-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let abstract_file = root.join("abstract.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\newcommand{\\paperabstract}{\\input{abstract}}\n\\begin{document}\n\\paperabstract\n\\end{document}\n",
    )
    .expect("failed to write test document");
    fs::write(&abstract_file, "Preview abstract.").expect("failed to write abstract");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        precompile_preamble: true,
        ..options(&main, &out_dir)
    })
    .expect("unsafe precompiled preamble should fall back to normal fast preview");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert!(!report.preamble_format_used, "{report:#?}");
    assert!(!report.preamble_format_built, "{report:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn full_build_precompile_preamble_reuses_format_after_body_edit() {
    if !command_available("pdflatex") || !tex_file_available("mylatexformat.ltx") {
        eprintln!(
            "skipping full precompiled preamble test; pdflatex or mylatexformat is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("texpilot-full-precompile-preamble-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nFull build.\n\\end{document}\n",
    )
    .expect("failed to write test document");

    let first = build(&BuildOptions {
        precompile_preamble: true,
        ..options(&main, &out_dir)
    })
    .expect("first full build should build a precompiled preamble");

    assert_eq!(first.tex_runs, 1, "{first:#?}");
    assert!(first.preamble_format_used, "{first:#?}");
    assert!(first.preamble_format_built, "{first:#?}");

    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nFull build edited.\n\\end{document}\n",
    )
    .expect("failed to edit document body");

    let second = build(&BuildOptions {
        precompile_preamble: true,
        ..options(&main, &out_dir)
    })
    .expect("body edit should reuse the full-build precompiled preamble");

    assert_eq!(second.tex_runs, 1, "{second:#?}");
    assert!(second.preamble_format_used, "{second:#?}");
    assert!(!second.preamble_format_built, "{second:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn full_build_does_not_opportunistically_build_preamble_format_without_draft_prepass() {
    if !command_available("pdflatex") {
        eprintln!("skipping no-draft opportunistic preamble test; pdflatex is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-no-draft-opportunistic-preamble-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\begin{document}\nNo draft.\n\\end{document}\n",
    )
    .expect("failed to write test document");

    let report = build(&options(&main, &out_dir))
        .expect("ordinary no-draft build should not precompile opportunistically");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert!(!report.preamble_format_used, "{report:#?}");
    assert!(!report.preamble_format_built, "{report:#?}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_replaces_svg_includes_without_image_lookup() {
    if !command_available("pdflatex") || !tex_file_available("svg.sty") {
        eprintln!("skipping fast SVG test; pdflatex or svg.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-svg-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, SVG_MISSING_IMAGE_DOC).expect("failed to write test document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should not load missing SVGs");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(!out_dir.join("svg-inkscape").exists());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_replaces_pdfpages_includes_without_pdf_lookup() {
    if !command_available("pdflatex") || !tex_file_available("pdfpages.sty") {
        eprintln!("skipping fast pdfpages test; pdflatex or pdfpages.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-pdfpages-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, PDFPAGES_MISSING_DOC).expect("failed to write pdfpages document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should not load missing included PDFs");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(!fls.contains("missing-document"), "{fls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_uses_minted_placeholder_without_highlighting_cache() {
    if !command_available("pdflatex") || !tex_file_available("minted.sty") {
        eprintln!("skipping fast minted test; pdflatex or minted.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-minted-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, MINTED_DOC).expect("failed to write minted document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should not run minted highlighting");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(!out_dir.join("_minted").exists());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(!fls.contains(".data.minted"), "{fls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_replaces_inputminted_without_source_lookup() {
    if !command_available("pdflatex") || !tex_file_available("minted.sty") {
        eprintln!("skipping fast inputminted test; pdflatex or minted.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-inputminted-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, INPUTMINTED_MISSING_DOC).expect("failed to write inputminted document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should not load missing inputminted files");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(!out_dir.join("_minted").exists());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(!fls.contains("generated-snippet.py"), "{fls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_replaces_animategraphics_without_frame_lookup() {
    if !command_available("pdflatex") || !tex_file_available("animate.sty") {
        eprintln!("skipping fast animate test; pdflatex or animate.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-animate-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, ANIMATE_MISSING_DOC).expect("failed to write animate document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should not load missing animation frames");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(!fls.contains("frames/frame-"), "{fls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_replaces_includestandalone_without_subdocument_lookup() {
    if !command_available("pdflatex") || !tex_file_available("standalone.sty") {
        eprintln!("skipping fast standalone test; pdflatex or standalone.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-standalone-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, STANDALONE_MISSING_DOC).expect("failed to write standalone document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should not load missing standalone subdocuments");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(!fls.contains("generated-figure"), "{fls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_replaces_includemedia_without_media_lookup() {
    if !command_available("pdflatex") || !tex_file_available("media9.sty") {
        eprintln!("skipping fast media9 test; pdflatex or media9.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-media9-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, MEDIA9_MISSING_DOC).expect("failed to write media9 document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should not load missing media files");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(!fls.contains("generated-demo.mp4"), "{fls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_replaces_attachfile2_without_attachment_lookup() {
    if !command_available("pdflatex") || !tex_file_available("attachfile2.sty") {
        eprintln!("skipping fast attachfile2 test; pdflatex or attachfile2.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-attachfile2-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, ATTACHFILE2_MISSING_DOC).expect("failed to write attachfile2 document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should not load missing attachfile2 attachments");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(!fls.contains("generated-data.csv"), "{fls}");
    assert!(!fls.contains("generated-report.json"), "{fls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_replaces_attachfile_without_attachment_lookup() {
    if !command_available("pdflatex") || !tex_file_available("attachfile.sty") {
        eprintln!("skipping fast attachfile test; pdflatex or attachfile.sty is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-attachfile-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, ATTACHFILE_MISSING_DOC).expect("failed to write attachfile document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should not load missing attachfile attachments");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    let fls = fs::read_to_string(out_dir.join("main.fls")).expect("failed to read recorder file");
    assert!(!fls.contains("generated-data.csv"), "{fls}");
    assert!(!fls.contains("generated-report.json"), "{fls}");

    let _ = fs::remove_dir_all(root);
}

#[test]
fn fast_preview_disables_tikz_externalization_without_shell_escape() {
    if !command_available("pdflatex")
        || !tex_file_available("tikz.sty")
        || !tex_file_available("tikzlibraryexternal.code.tex")
    {
        eprintln!("skipping fast TikZ externalization test; pdflatex or TikZ is unavailable");
        return;
    }

    let root = unique_temp_dir("texpilot-fast-tikz-external-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let out_dir = root.join("out");
    fs::write(&main, TIKZ_EXTERNAL_DOC).expect("failed to write TikZ document");

    let report = build(&BuildOptions {
        fast: true,
        once: true,
        shell_escape: false,
        ..options(&main, &out_dir)
    })
    .expect("fast preview should disable TikZ externalization");

    assert_eq!(report.tex_runs, 1, "{report:#?}");
    assert_eq!(report.external_runs, 0, "{report:#?}");
    assert!(out_dir.join("main.pdf").exists());
    assert!(!out_dir.join("main-figure0.pdf").exists());
    assert!(!out_dir.join("main-figure0.md5").exists());

    let _ = fs::remove_dir_all(root);
}

#[test]
fn draft_prepass_precompile_preamble_builds_format_for_final_pass() {
    if !command_available("pdflatex")
        || !tex_file_available("mylatexformat.ltx")
        || !tex_file_available("mwe/example-image.pdf")
    {
        eprintln!(
            "skipping draft precompiled preamble test; pdflatex, mylatexformat, or mwe image is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("texpilot-draft-precompile-preamble-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let bibliography = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\usepackage{graphicx}\n\\begin{document}\n\\includegraphics{example-image}\n\\section{A}\\label{s:a}\nSee Section~\\ref{s:a} and \\cite{x}.\n\\bibliographystyle{plain}\n\\bibliography{refs}\n\\end{document}\n",
    )
    .expect("failed to write test document");
    fs::write(
        &bibliography,
        "@article{x, author={A. Author}, title={Title}, journal={Journal}, year={2024}}\n",
    )
    .expect("failed to write test bibliography");

    let report = build(&BuildOptions {
        precompile_preamble: true,
        draft_prepass: DraftPrepass::Auto,
        ..options(&main, &out_dir)
    })
    .expect("draft/full build should use a precompiled final preamble");

    assert!(report.draft_tex_runs > 0, "{report:#?}");
    assert!(report.preamble_format_used, "{report:#?}");
    assert!(report.preamble_format_built, "{report:#?}");
    assert!(
        report
            .passes
            .iter()
            .any(|pass| !pass.draft && pass.preamble_format_used && pass.preamble_format_built),
        "{report:#?}"
    );

    let _ = fs::remove_dir_all(root);
}

#[test]
fn draft_prepass_opportunistically_builds_safe_preamble_format_for_final_pass() {
    if !command_available("pdflatex")
        || !tex_file_available("mylatexformat.ltx")
        || !tex_file_available("mwe/example-image.pdf")
    {
        eprintln!(
            "skipping opportunistic preamble test; pdflatex, mylatexformat, or mwe image is unavailable"
        );
        return;
    }

    let root = unique_temp_dir("texpilot-draft-opportunistic-preamble-test");
    fs::create_dir_all(&root).expect("failed to create test directory");
    let main = root.join("main.tex");
    let bibliography = root.join("refs.bib");
    let out_dir = root.join("out");
    fs::write(
        &main,
        "\\documentclass{article}\n\\usepackage{graphicx}\n\\begin{document}\n\\includegraphics{example-image}\n\\section{A}\\label{s:a}\nSee Section~\\ref{s:a} and \\cite{x}.\n\\bibliographystyle{plain}\n\\bibliography{refs}\n\\end{document}\n",
    )
    .expect("failed to write test document");
    fs::write(
        &bibliography,
        "@article{x, author={A. Author}, title={Title}, journal={Journal}, year={2024}}\n",
    )
    .expect("failed to write test bibliography");

    let report = build(&BuildOptions {
        draft_prepass: DraftPrepass::Auto,
        ..options(&main, &out_dir)
    })
    .expect("draft/full build should opportunistically use a precompiled final preamble");

    assert!(report.draft_tex_runs > 0, "{report:#?}");
    assert!(report.preamble_format_used, "{report:#?}");
    assert!(report.preamble_format_built, "{report:#?}");
    assert!(
        report
            .passes
            .iter()
            .any(|pass| !pass.draft && pass.preamble_format_used && pass.preamble_format_built),
        "{report:#?}"
    );

    let _ = fs::remove_dir_all(root);
}

fn options(main: &Path, out_dir: &Path) -> BuildOptions {
    BuildOptions {
        main: main.to_path_buf(),
        job_name: None,
        engine: Engine::PdfLatex,
        runner: Runner::Direct,
        bib_mode: BibMode::Auto,
        out_dir: out_dir.to_path_buf(),
        fast: false,
        draft_prepass: DraftPrepass::Never,
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
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|directory| directory.join(program).is_file())
        })
        .unwrap_or(false)
}

fn tex_file_available(name: &str) -> bool {
    std::process::Command::new("kpsewhich")
        .arg(name)
        .status()
        .is_ok_and(|status| status.success())
}

fn unique_temp_dir(prefix: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        prefix,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock before UNIX epoch")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

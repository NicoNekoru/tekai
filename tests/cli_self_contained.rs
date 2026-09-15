use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Project(PathBuf);

impl Project {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "tekai-self-contained-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("empty-bin")).unwrap();
        Self(root)
    }

    fn write(&self, name: &str, source: &str) {
        let path = self.0.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, source).unwrap();
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_tekai"))
            .current_dir(&self.0)
            .env_clear()
            .env("PATH", self.0.join("empty-bin"))
            .env("TEKAI_ENGINE_CACHE", self.0.join("cache/engine"))
            .env("TEKAI_FORMAT_CACHE", self.0.join("cache/formats"))
            .env("TEKAI_AUX_CACHE", self.0.join("cache/aux"))
            .env("TEKAI_BIBTEX_CACHE", self.0.join("cache/bibtex"))
            // Poison old system-data hints as well as executable discovery.
            .env("TEXMFDIST", self.0.join("missing-texlive"))
            .env("TEXMFVAR", self.0.join("missing-texlive"))
            .env("TEXMFCONFIG", self.0.join("missing-texlive"))
            .env("shell_escape", "t")
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
fn check_builds_and_caches_without_external_tools_or_tex_installation() {
    let project = Project::new();
    project.write(
        "main.tex",
        r"\documentclass{article}
\usepackage{amsmath,amsthm,graphicx,hyperref}
\begin{document}
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

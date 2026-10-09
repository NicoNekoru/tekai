//! Native lookup tests run without TeX. The reference gate is required in CI,
//! where BasicTeX supplies kpsewhich, pdflatex, and a real CTAN package.

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

struct Project(PathBuf);

impl Project {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "tekai-texmf-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        for dir in ["paper", "home", "site", "empty-bin"] {
            fs::create_dir_all(path.join(dir)).unwrap();
        }
        Self(path.canonicalize().unwrap())
    }

    fn home_tree(&self) -> PathBuf {
        self.0.join("home").join(if cfg!(target_os = "macos") {
            "Library/texmf"
        } else {
            "texmf"
        })
    }

    fn write(&self, path: impl AsRef<Path>, content: &str) {
        let path = self.0.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn command(&self, program: impl AsRef<std::ffi::OsStr>, env: &[(&str, String)]) -> Command {
        let mut command = Command::new(program);
        command
            .current_dir(self.0.join("paper"))
            .env_clear()
            .env("HOME", self.0.join("home"))
            .env("PATH", self.0.join("empty-bin"))
            .env("TEXMFLOCAL", self.0.join("site"))
            .env("TEKAI_ENGINE_CACHE", self.0.join("cache/engine"))
            .env("TEKAI_FORMAT_CACHE", self.0.join("cache/formats"))
            .env("TEKAI_AUX_CACHE", self.0.join("cache/aux"))
            .env("TEKAI_BIBTEX_CACHE", self.0.join("cache/bibtex"));
        for (key, value) in env {
            command.env(key, value);
        }
        command
    }

    fn run(&self, args: &[&str], env: &[(&str, String)]) -> Output {
        self.command(env!("CARGO_BIN_EXE_tekai"), env)
            .args(args)
            .output()
            .unwrap()
    }

    fn success(&self, args: &[&str], env: &[(&str, String)]) -> Value {
        let output = self.run(args, env);
        assert!(
            output.status.success(),
            "args {args:?}\nstdout {}\nstderr {}\nlog {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
            fs::read_to_string(self.0.join("paper/build/main.log")).unwrap_or_default()
        );
        serde_json::from_slice(&output.stdout).unwrap()
    }

    fn locate(&self, name: &str, env: &[(&str, String)]) -> Value {
        self.success(&["locate", name, "--report-json"], env)
    }

    fn document(&self, package: &str) {
        self.write("paper/main.tex", &format!("\\documentclass{{article}}\n\\usepackage{{{package}}}\n\\begin{{document}}Text.\\end{{document}}\n"));
    }

    fn build(&self, env: &[(&str, String)]) -> Value {
        self.success(&["build", "main.tex", "--report-json"], env)
    }

    fn marker(&self, marker: &str) {
        assert!(
            fs::read_to_string(self.0.join("paper/build/main.log"))
                .unwrap()
                .contains(marker)
        );
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn default_personal_tree_loads_packages_and_tracks_changes() {
    let project = Project::new();
    let package = project.home_tree().join("tex/latex/probe/probe.sty");
    project.write(
        &package,
        "\\ProvidesPackage{probe}\n\\typeout{PERSONAL-FIRST}\n",
    );
    project.document("probe");
    let found = project.locate("probe.sty", &[]);
    assert_eq!(found["path"], package.to_str().unwrap());
    assert_eq!(found["source"], "TEXMFHOME");
    project.build(&[]);
    project.marker("PERSONAL-FIRST");
    assert_eq!(project.build(&[])["skipped"], true);
    project.write(
        &package,
        "\\ProvidesPackage{probe}\n\\typeout{PERSONAL-SECOND}\n",
    );
    assert_eq!(project.build(&[])["skipped"], false);
    project.marker("PERSONAL-SECOND");
}

#[test]
fn adding_a_personal_override_invalidates_a_bundled_build() {
    let project = Project::new();
    project.document("color");
    assert_eq!(project.locate("color.sty", &[])["source"], "bundled");
    project.build(&[]);
    assert_eq!(project.build(&[])["skipped"], true);
    project.write(
        project.home_tree().join("tex/latex/color/color.sty"),
        "\\ProvidesPackage{color}\n\\typeout{PERSONAL-OVERRIDE}\n",
    );
    assert_eq!(project.build(&[])["skipped"], false);
    project.marker("PERSONAL-OVERRIDE");
}

#[test]
fn site_tree_requires_a_filename_database_and_obeys_type_directories() {
    let project = Project::new();
    project.write(
        "site/tex/latex/probe/siteprobe.sty",
        "\\ProvidesPackage{siteprobe}\n\\typeout{SITE-PACKAGE}\n",
    );
    assert!(
        !project
            .run(&["locate", "siteprobe.sty"], &[])
            .status
            .success()
    );
    project.write("site/ls-R", "./tex/latex/probe:\nsiteprobe.sty\n");
    assert_eq!(project.locate("siteprobe.sty", &[])["source"], "TEXMFLOCAL");
    project.document("siteprobe");
    project.build(&[]);
    project.marker("SITE-PACKAGE");
    project.write("site/ls-R", "./doc/latex/probe:\nsiteprobe.sty\n");
    assert!(
        !project
            .run(&["locate", "siteprobe.sty"], &[])
            .status
            .success()
    );
}

#[test]
fn default_insertion_and_explicit_path_order_are_preserved() {
    let project = Project::new();
    let personal = project.home_tree().join("tex/latex/probe/order.sty");
    let explicit = project.0.join("extra/order.sty");
    project.write(&personal, "personal");
    project.write(&explicit, "explicit");
    for (path, expected) in [
        (
            format!(":{}//", project.0.join("extra").display()),
            &personal,
        ),
        (
            format!("{}//:", project.0.join("extra").display()),
            &explicit,
        ),
        (
            format!(
                "{}//::{}//",
                project.0.join("missing").display(),
                project.0.join("extra").display()
            ),
            &personal,
        ),
    ] {
        assert_eq!(
            project.locate("order.sty", &[("TEXINPUTS", path)])["path"],
            expected.to_str().unwrap()
        );
    }
    let no_defaults = [(
        "TEXINPUTS",
        project.0.join("extra").to_string_lossy().into_owned(),
    )];
    assert!(
        !project
            .run(&["locate", "article.cls"], &no_defaults)
            .status
            .success()
    );
    assert_eq!(
        project.locate("order.sty", &no_defaults)["path"],
        explicit.to_str().unwrap()
    );
}

#[test]
fn variables_braces_tilde_and_program_specific_paths_work() {
    let project = Project::new();
    let package = project.0.join("extra/b/deep/expanded.sty");
    project.write(
        &package,
        "\\ProvidesPackage{expanded}\n\\typeout{EXPANDED-PATH}\n",
    );
    let env = [
        (
            "EXTRA_ROOT",
            project.0.join("extra").to_string_lossy().into_owned(),
        ),
        ("TEXINPUTS", "${EXTRA_ROOT}/{a,b}//:".into()),
    ];
    assert_eq!(
        project.locate("expanded.sty", &env)["path"],
        package.to_str().unwrap()
    );
    project.document("expanded");
    project.build(&env);
    project.marker("EXPANDED-PATH");
    project.write("home/custom/expanded.sty", "tilde");
    let env = [
        ("TEXINPUTS", "~/custom//:".into()),
        ("TEXINPUTS_pdflatex", "$EXTRA_ROOT/b//:".into()),
        (
            "EXTRA_ROOT",
            project.0.join("extra").to_string_lossy().into_owned(),
        ),
    ];
    assert_eq!(
        project.locate("expanded.sty", &env)["path"],
        package.to_str().unwrap()
    );
    assert_eq!(
        project.locate("expanded.sty", &[("TEXINPUTS", "~/custom//:".into())])["path"],
        project.0.join("home/custom/expanded.sty").to_str().unwrap()
    );
}

#[test]
fn bundled_mode_excludes_automatic_trees_but_accepts_explicit_paths() {
    let project = Project::new();
    let root = project.home_tree().join("tex/latex/probe");
    project.write(root.join("isolated.sty"), "\\ProvidesPackage{isolated}\n");
    let mode = [("TEKAI_TEXMF_MODE", "bundled".into())];
    assert!(
        !project
            .run(&["locate", "isolated.sty"], &mode)
            .status
            .success()
    );
    assert_eq!(project.locate("article.cls", &mode)["source"], "bundled");
    project.document("isolated");
    let env = [
        ("TEKAI_TEXMF_MODE", "bundled".into()),
        ("TEXINPUTS", format!("{}//:", root.display())),
    ];
    project.build(&env);
    assert!(
        !project
            .run(
                &["locate", "article.cls"],
                &[("TEKAI_TEXMF_MODE", "typo".into())]
            )
            .status
            .success()
    );
}

#[test]
fn shared_bibliography_and_font_inputs_use_their_tds_directories() {
    let project = Project::new();
    let tree = project.home_tree();
    project.write(
        tree.join("bibtex/bib/probe/sharedrefs.bib"),
        "@book{sample, author={Ada Example}, title={Shared bibliography}, year={2026}}\n",
    );
    let plain = project.locate("plain.bst", &[])["path"]
        .as_str()
        .unwrap()
        .to_owned();
    let style = fs::read_to_string(plain).unwrap();
    project.write(tree.join("bibtex/bst/probe/sharedstyle.bst"), &style);
    project.write(
        tree.join("fonts/tfm/probe/probefont.tfm"),
        "lookup-only font fixture",
    );
    project.write(
        tree.join("fonts/enc/probe/probeencoding.enc"),
        "lookup-only encoding fixture",
    );
    for name in [
        "sharedrefs.bib",
        "sharedstyle.bst",
        "probefont.tfm",
        "probeencoding.enc",
    ] {
        assert_eq!(project.locate(name, &[])["source"], "TEXMFHOME");
    }
    project.write("paper/main.tex", "\\documentclass{article}\n\\begin{document}\n\\cite{sample}\n\\bibliographystyle{sharedstyle}\n\\bibliography{sharedrefs}\n\\end{document}\n");
    project.build(&[]);
    assert!(
        fs::read_to_string(project.0.join("paper/build/main.bbl"))
            .unwrap()
            .contains("Shared bibliography")
    );
}

#[test]
fn custom_tree_lists_and_project_configuration_are_respected() {
    let project = Project::new();
    project.write(
        "custom/two/tex/latex/probe/custom.sty",
        "\\ProvidesPackage{custom}\n\\typeout{CUSTOM-TREE}\n",
    );
    project.write(
        "paper/tekai.toml",
        &format!(
            "[build.env]\nTEXMFHOME = '{{{}/custom/one,{}/custom/two}}'\n",
            project.0.display(),
            project.0.display()
        ),
    );
    assert_eq!(project.locate("custom.sty", &[])["source"], "TEXMFHOME");
    project.document("custom");
    project.build(&[]);
    project.marker("CUSTOM-TREE");
}

#[test]
fn files_created_after_inventory_are_found_recursively_in_the_same_pass() {
    let project = Project::new();
    fs::create_dir_all(project.0.join("paper/build/generated/deep")).unwrap();
    project.write(
        "paper/main.tex",
        r"\documentclass{article}
\begin{filecontents*}[overwrite]{generated/deep/newinput.tex}
\typeout{GENERATED-AFTER-INVENTORY}
Generated body.
\end{filecontents*}
\begin{document}
\input{newinput}
\end{document}
",
    );
    project.build(&[]);
    project.marker("GENERATED-AFTER-INVENTORY");
    assert!(
        project
            .0
            .join("paper/build/generated/deep/newinput.tex")
            .is_file()
    );
}

#[test]
fn referenced_variable_changes_invalidate_the_build_cache() {
    let project = Project::new();
    for (tree, marker) in [("one", "FIRST-VARIABLE"), ("two", "SECOND-VARIABLE")] {
        project.write(
            format!("{tree}/variable.sty"),
            &format!("\\ProvidesPackage{{variable}}\n\\typeout{{{marker}}}\n"),
        );
    }
    project.document("variable");
    let mut env = vec![
        ("TEXINPUTS", "$PACKAGES//:".into()),
        (
            "PACKAGES",
            project.0.join("one").to_string_lossy().into_owned(),
        ),
    ];
    project.build(&env);
    assert_eq!(project.build(&env)["skipped"], true);
    env[1].1 = project.0.join("two").to_string_lossy().into_owned();
    assert_eq!(project.build(&env)["skipped"], false);
    project.marker("SECOND-VARIABLE");
}

#[test]
fn relative_shared_trees_are_fingerprinted_from_the_document_directory() {
    let project = Project::new();
    project.document("color");
    let run = || {
        let output = project
            .command(
                env!("CARGO_BIN_EXE_tekai"),
                &[("TEXMFHOME", "../relative".into())],
            )
            .current_dir(&project.0)
            .args(["build", "paper/main.tex", "--report-json"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice::<Value>(&output.stdout).unwrap()
    };
    run();
    assert_eq!(run()["skipped"], true);
    project.write(
        "relative/tex/latex/color/color.sty",
        "\\ProvidesPackage{color}\n\\typeout{RELATIVE-OVERRIDE}\n",
    );
    assert_eq!(run()["skipped"], false);
    assert!(
        fs::read_to_string(project.0.join("build/main.log"))
            .unwrap()
            .contains("RELATIVE-OVERRIDE")
    );
}

#[test]
fn missing_inputs_report_paths_and_foreign_distribution_hints_do_not_replace_the_kernel() {
    let project = Project::new();
    let output = project.run(&["locate", "missing.sty", "--report-json"], &[]);
    assert!(!output.status.success());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["path"], Value::Null);
    assert!(report["search_paths"].as_array().unwrap().len() > 1);
    project.write("foreign/tex/latex/base/article.cls", "foreign kernel");
    let env = [(
        "TEXMFDIST",
        project.0.join("foreign").to_string_lossy().into_owned(),
    )];
    assert_eq!(project.locate("article.cls", &env)["source"], "bundled");
    assert!(
        !project
            .run(&["locate", "pdflatex.fmt"], &env)
            .status
            .success()
    );
}

#[cfg(unix)]
#[test]
fn recursive_paths_follow_symlinks_without_looping() {
    let project = Project::new();
    project.write("extra/linked.sty", "linked");
    let directory = project.home_tree().join("tex/latex");
    fs::create_dir_all(&directory).unwrap();
    std::os::unix::fs::symlink(project.0.join("extra"), directory.join("linked")).unwrap();
    std::os::unix::fs::symlink(&directory, directory.join("cycle")).unwrap();
    assert_eq!(
        project.locate("linked.sty", &[])["path"],
        project.0.join("extra/linked.sty").to_str().unwrap()
    );
    assert!(
        !project
            .run(&["locate", "missing.sty"], &[])
            .status
            .success()
    );
}

#[test]
fn real_tex_installation_agrees_on_shared_paths_and_compiles_ctan_package() {
    let required = std::env::var_os("TEKAI_REQUIRE_SYSTEM_TEX").is_some();
    let available = ["kpsewhich", "pdflatex", "mktexlsr"]
        .into_iter()
        .all(|program| {
            Command::new(program)
                .arg("--version")
                .output()
                .is_ok_and(|output| output.status.success())
        });
    if !available {
        assert!(
            !required,
            "CI reference gate requires kpsewhich, pdflatex, and mktexlsr"
        );
        eprintln!("skipping real TeX reference gate; no system TeX installation");
        return;
    }
    let project = Project::new();
    let system_path = std::env::var("PATH").unwrap();
    let env = [("PATH", system_path.clone())];
    let ctan = Command::new("kpsewhich")
        .args(["--progname=pdflatex", "xspace.sty"])
        .output()
        .unwrap();
    assert!(
        ctan.status.success() && !ctan.stdout.is_empty(),
        "BasicTeX must provide xspace.sty"
    );
    let ctan_path = String::from_utf8(ctan.stdout).unwrap();
    project.write(
        project.home_tree().join("tex/latex/probe/xspace.sty"),
        &fs::read_to_string(ctan_path.trim()).unwrap(),
    );
    let metrics = Command::new("kpsewhich").arg("cmr10.tfm").output().unwrap();
    assert!(
        metrics.status.success() && !metrics.stdout.is_empty(),
        "BasicTeX must provide cmr10.tfm"
    );
    let metrics_path = String::from_utf8(metrics.stdout).unwrap();
    let shared_metrics = project.home_tree().join("fonts/tfm/probe/cmr10.tfm");
    fs::create_dir_all(shared_metrics.parent().unwrap()).unwrap();
    fs::copy(metrics_path.trim(), &shared_metrics).unwrap();
    project.write(
        "site/tex/latex/probe/siteprobe.sty",
        "\\ProvidesPackage{siteprobe}\n\\newcommand{\\siteword}{Shared site text}\n",
    );
    let indexed = project
        .command("mktexlsr", &env)
        .arg(project.0.join("site"))
        .output()
        .unwrap();
    assert!(
        indexed.status.success(),
        "{}",
        String::from_utf8_lossy(&indexed.stderr)
    );
    project.write("extra/a/order.sty", "a");
    project.write("extra/b/order.sty", "b");
    project.write(
        project.home_tree().join("tex/latex/probe/order.sty"),
        "home",
    );
    let cases = [
        ("xspace.sty", None),
        ("cmr10.tfm", None),
        ("siteprobe.sty", None),
        (
            "order.sty",
            Some(format!("{}/{{a,b}}//:", project.0.join("extra").display())),
        ),
        (
            "order.sty",
            Some(format!(":{}/a//", project.0.join("extra").display())),
        ),
        ("order.sty", Some("~/Library/texmf/tex//:".into())),
        (
            "siteprobe.sty",
            Some(format!("!!{}//:", project.0.join("site").display())),
        ),
    ];
    for (name, texinputs) in cases {
        let mut env = env.to_vec();
        if let Some(texinputs) = texinputs {
            env.push(("TEXINPUTS", texinputs));
        }
        let reference = project
            .command("kpsewhich", &env)
            .args(["--progname=pdflatex", "--must-exist", name])
            .output()
            .unwrap();
        assert!(
            reference.status.success(),
            "{}",
            String::from_utf8_lossy(&reference.stderr)
        );
        let path = PathBuf::from(String::from_utf8(reference.stdout).unwrap().trim());
        assert_eq!(
            project.locate(name, &env)["path"],
            path.canonicalize().unwrap().to_str().unwrap(),
            "{name}, {env:?}"
        );
    }
    project.write("paper/main.tex", "\\documentclass{article}\n\\usepackage{xspace,siteprobe}\n\\begin{document}\n\\siteword\\xspace from CTAN.\n\\end{document}\n");
    project.build(&env);
    let reference = project
        .command("pdflatex", &env)
        .args([
            "-interaction=nonstopmode",
            "-halt-on-error",
            "-jobname=reference",
            "main.tex",
        ])
        .output()
        .unwrap();
    assert!(
        reference.status.success(),
        "{}",
        String::from_utf8_lossy(&reference.stdout)
    );
    assert!(project.0.join("paper/reference.pdf").is_file());
}

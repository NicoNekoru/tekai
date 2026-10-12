//! Shared TeX trees and the supported Kpathsea search-path syntax.
//!
//! This does not discover distribution trees or read system texmf.cnf files.
//! The embedded format continues to use its matching distribution data.

use std::path::Path;

#[derive(Clone, Debug)]
pub struct SearchPath {
    pub path: String,
    pub source: &'static str,
    pub bundled: bool,
}

pub fn mode() -> std::io::Result<String> {
    let mode = crate::environment::var("TEKAI_TEXMF_MODE").unwrap_or_else(|_| "shared".into());
    match mode.as_str() {
        "shared" | "bundled" => Ok(mode),
        _ => Err(std::io::Error::other(format!(
            "invalid TEKAI_TEXMF_MODE {mode:?}; expected shared or bundled"
        ))),
    }
}

fn variable(name: &str) -> Option<String> {
    crate::environment::var(name)
        .ok()
        .or_else(|| crate::environment::cnf_value(name).and_then(|value| value.into_string().ok()))
        .or_else(|| match name {
            "TEXMFHOME" => crate::environment::var("HOME").ok().map(|home| {
                Path::new(&home)
                    .join(if cfg!(target_os = "macos") {
                        "Library/texmf"
                    } else {
                        "texmf"
                    })
                    .to_string_lossy()
                    .into_owned()
            }),
            "TEXMFLOCAL" => Some("/usr/local/texlive/texmf-local".into()),
            _ => None,
        })
}

fn expand_variables(value: &str, get: &impl Fn(&str) -> Option<String>, depth: usize) -> String {
    if depth == 16 {
        return String::new();
    }
    let mut result = String::new();
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '$' {
            result.push(ch);
            continue;
        }
        let braced = chars.peek() == Some(&'{');
        let mut name = String::new();
        if braced {
            chars.next();
            for ch in chars.by_ref() {
                if ch == '}' {
                    break;
                }
                name.push(ch);
            }
        } else {
            while chars
                .peek()
                .is_some_and(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
            {
                name.push(chars.next().unwrap());
            }
        }
        if name.is_empty() {
            result.push('$');
        } else if let Some(value) = get(&name) {
            result.push_str(&expand_variables(&value, get, depth + 1));
        }
    }
    result
}

fn split_path(value: &str) -> Vec<&str> {
    let mut depth = 0usize;
    let mut start = 0;
    let mut entries = Vec::new();
    for (i, ch) in value.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            ':' if depth == 0 => {
                entries.push(&value[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    entries.push(&value[start..]);
    entries
}

fn expand_braces(value: &str, depth: usize) -> Vec<String> {
    if depth == 16 {
        return Vec::new();
    }
    let Some(start) = value.find('{') else {
        return vec![value.into()];
    };
    let mut level = 0usize;
    let mut end = None;
    let mut alternatives = Vec::new();
    let mut from = start + 1;
    for (i, ch) in value.char_indices().skip_while(|(i, _)| *i < start) {
        match ch {
            '{' => level += 1,
            '}' => {
                level -= 1;
                if level == 0 {
                    alternatives.push(&value[from..i]);
                    end = Some(i);
                    break;
                }
            }
            ',' if level == 1 => {
                alternatives.push(&value[from..i]);
                from = i + 1;
            }
            _ => {}
        }
    }
    let Some(end) = end else {
        return vec![value.into()];
    };
    alternatives
        .into_iter()
        .flat_map(|part| {
            expand_braces(
                &format!("{}{part}{}", &value[..start], &value[end + 1..]),
                depth + 1,
            )
        })
        .collect()
}

fn expand_with(value: &str, get: &impl Fn(&str) -> Option<String>) -> Vec<String> {
    let value = expand_variables(value, get, 0);
    split_path(&value)
        .into_iter()
        .flat_map(|entry| expand_braces(entry, 0))
        .map(|entry| {
            let (prefix, entry) = entry
                .strip_prefix("!!")
                .map_or(("", entry.as_str()), |s| ("!!", s));
            if entry == "~" || entry.starts_with("~/") {
                if let Some(home) = get("HOME") {
                    return format!("{prefix}{home}{}", &entry[1..]);
                }
            }
            format!("{prefix}{entry}")
        })
        .collect()
}

pub fn expand_path(value: &str) -> Vec<String> {
    expand_with(value, &variable)
}

/// User trees are searched on disk. Site trees follow TeX Live's database-only
/// convention, so additions there become visible after mktexlsr/texhash.
pub fn shared_roots() -> Vec<SearchPath> {
    if mode().ok().as_deref() != Some("shared") {
        return Vec::new();
    }
    let mut roots = Vec::new();
    for (key, database_only) in [("TEXMFHOME", false), ("TEXMFLOCAL", true)] {
        if let Some(value) = variable(key) {
            for root in expand_path(&value)
                .into_iter()
                .filter(|root| !root.is_empty())
            {
                roots.push(SearchPath {
                    path: if database_only && !root.starts_with("!!") {
                        format!("!!{root}")
                    } else {
                        root
                    },
                    source: key,
                    bundled: false,
                });
            }
        }
    }
    roots
}

pub fn paths(variables: &[&'static str], subdirs: &[&str]) -> Vec<SearchPath> {
    let mut defaults = vec![SearchPath {
        path: ".//".into(),
        source: "project",
        bundled: false,
    }];
    for root in shared_roots() {
        for dir in subdirs {
            defaults.push(SearchPath {
                path: format!("{}/{dir}//", root.path.trim_end_matches('/')),
                source: root.source,
                bundled: false,
            });
        }
    }
    defaults.push(SearchPath {
        path: "<bundled>".into(),
        source: "bundled",
        bundled: true,
    });

    configured_paths(defaults, variables, &variable)
}

fn configured_paths(
    defaults: Vec<SearchPath>,
    variables: &[&'static str],
    get: &impl Fn(&str) -> Option<String>,
) -> Vec<SearchPath> {
    let configured = variables.iter().find_map(|key| {
        let (source, value) = if *key == "TEXINPUTS" {
            match get("TEXINPUTS_pdflatex") {
                Some(value) => ("TEXINPUTS_pdflatex", Some(value)),
                None => (*key, get(key)),
            }
        } else {
            (*key, get(key))
        };
        value.map(|value| (source, expand_with(&value, get)))
    });
    let Some((key, entries)) = configured else {
        return defaults;
    };
    // Only the compiler-injected generic TEXINPUTS prefix is implicit.
    // User program-specific paths and later duplicates retain explicit lookup
    // semantics, including access to generated/output namespaces.
    let project_prefix = get("TEKAI_ENGINE_PROJECT_ROOT").map(|root| format!("{root}//"));
    let output_prefix = get("TEKAI_ENGINE_OUTPUT_ROOT").map(|root| format!("{root}//"));
    let format_prefix = get("TEKAI_ENGINE_FORMAT_ROOT").map(|root| format!("{root}//"));
    let has_format_prefix =
        key == "TEXINPUTS" && format_prefix.as_deref() == entries.first().map(String::as_str);
    let project_position = usize::from(has_format_prefix);
    let has_project_prefix = key == "TEXINPUTS"
        && project_prefix.as_deref() == entries.get(project_position).map(String::as_str);
    // Kpathsea expands one empty element, preferring leading, trailing, then
    // doubled separators. A path without an empty element replaces defaults.
    let insertion = if entries.first().is_some_and(String::is_empty) {
        Some(0)
    } else if entries.last().is_some_and(String::is_empty) {
        Some(entries.len() - 1)
    } else {
        entries.iter().position(String::is_empty)
    };
    let mut paths = Vec::new();
    for (i, path) in entries.into_iter().enumerate() {
        if Some(i) == insertion {
            paths.extend(defaults.iter().cloned());
        } else if !path.is_empty() {
            paths.push(SearchPath {
                source: if (has_format_prefix && i == 0)
                    || (has_project_prefix
                        && (i == project_position
                            || (i == project_position + 1
                                && output_prefix.as_deref() == Some(path.as_str()))))
                {
                    "project"
                } else {
                    key
                },
                path,
                bundled: false,
            });
        }
    }
    paths
}

/// Search configuration identity. Mutable membership is captured once by
/// `lookup::dependency_paths` and checked through persisted directory stamps.
pub fn tree_signature(base: &Path) -> String {
    signature_for_roots(base, &shared_roots())
}

fn signature_for_roots(base: &Path, roots: &[SearchPath]) -> String {
    use std::hash::{Hash, Hasher};
    let mut signature = std::collections::hash_map::DefaultHasher::new();
    let base = crate::working_directory::resolve(base).ok();
    for root in roots {
        root.path.hash(&mut signature);
        let database_only = root.path.starts_with("!!");
        let Some(base) = base.as_ref() else {
            continue;
        };
        let root = base.join(root.path.strip_prefix("!!").unwrap_or(&root.path));
        if database_only {
            // Database-only lookup never walks the tree. Its invalidation must
            // follow the same rule, not inspect unrelated manuals or sources.
            let db = root
                .ancestors()
                .map(|root| root.join("ls-R"))
                .find(|db| db.is_file())
                .unwrap_or_else(|| root.join("ls-R"));
            hash_metadata(&db, &mut signature);
            continue;
        }
        root.hash(&mut signature);
    }
    format!("{:016x}", signature.finish())
}

fn hash_metadata(path: &Path, signature: &mut impl std::hash::Hasher) {
    use std::hash::Hash;
    path.hash(signature);
    path.canonicalize().ok().hash(signature);
    if let Ok(metadata) = std::fs::metadata(path) {
        metadata.modified().ok().hash(signature);
        metadata.len().hash(signature);
        crate::file_identity::change_identity(&metadata).hash(signature);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn configured_test_paths(
        values: &[(&str, &str)],
        variables: &[&'static str],
    ) -> Vec<SearchPath> {
        let defaults = vec![
            SearchPath {
                path: ".//".into(),
                source: "project",
                bundled: false,
            },
            SearchPath {
                path: "<bundled>".into(),
                source: "bundled",
                bundled: true,
            },
        ];
        configured_paths(defaults, variables, &|key| {
            values
                .iter()
                .find_map(|(name, value)| (*name == key).then(|| (*value).to_owned()))
        })
    }

    #[test]
    fn compiler_prefix_is_implicit_but_later_duplicates_are_explicit() {
        let paths = configured_test_paths(
            &[
                ("TEKAI_ENGINE_PROJECT_ROOT", "/project"),
                ("TEKAI_ENGINE_OUTPUT_ROOT", "/project/build"),
                (
                    "TEXINPUTS",
                    "/project//:/project/build//:/project//:/project/build//:",
                ),
            ],
            &["TEXINPUTS"],
        );
        let actual: Vec<_> = paths
            .iter()
            .map(|entry| (entry.path.as_str(), entry.source))
            .collect();
        assert_eq!(
            actual,
            [
                ("/project//", "project"),
                ("/project/build//", "project"),
                ("/project//", "TEXINPUTS"),
                ("/project/build//", "TEXINPUTS"),
                (".//", "project"),
                ("<bundled>", "bundled"),
            ]
        );
    }

    #[test]
    fn program_specific_texinputs_never_acquires_project_exclusions() {
        let paths = configured_test_paths(
            &[
                ("TEKAI_ENGINE_PROJECT_ROOT", "/project"),
                ("TEKAI_ENGINE_OUTPUT_ROOT", "/project/build"),
                ("TEXINPUTS", "/project//:"),
                ("TEXINPUTS_pdflatex", "/project//:/user//"),
            ],
            &["TEXINPUTS"],
        );
        assert_eq!(paths.len(), 2);
        assert!(paths
            .iter()
            .all(|entry| entry.source == "TEXINPUTS_pdflatex"));
    }

    #[test]
    fn preamble_format_prefix_shifts_only_compiler_injected_project_entries() {
        let paths = configured_test_paths(
            &[
                ("TEKAI_ENGINE_FORMAT_ROOT", "/formats"),
                ("TEKAI_ENGINE_PROJECT_ROOT", "/project"),
                ("TEKAI_ENGINE_OUTPUT_ROOT", "/project/build"),
                (
                    "TEXINPUTS",
                    "/formats//:/project//:/project/build//:/formats//:/project//",
                ),
            ],
            &["TEXINPUTS"],
        );
        assert_eq!(
            paths.iter().map(|entry| entry.source).collect::<Vec<_>>(),
            ["project", "project", "project", "TEXINPUTS", "TEXINPUTS"]
        );
    }

    #[test]
    fn typed_variables_keep_precedence_and_explicit_lookup_policy() {
        let paths = configured_test_paths(
            &[
                ("TEKAI_ENGINE_PROJECT_ROOT", "/project"),
                ("TEXINPUTS", "/project//:"),
                ("TFMFONTS", "/project//"),
            ],
            &["TFMFONTS", "TEXINPUTS"],
        );
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].path, "/project//");
        assert_eq!(paths[0].source, "TFMFONTS");
    }

    #[test]
    fn prefix_tag_requires_internal_marker_and_first_generic_position() {
        for values in [
            vec![("TEXINPUTS", "/project//")],
            vec![
                ("TEKAI_ENGINE_PROJECT_ROOT", "/project"),
                ("TEKAI_ENGINE_OUTPUT_ROOT", "/project/build"),
                ("TEXINPUTS", "/user//:/project/build//:/project//"),
            ],
        ] {
            let paths = configured_test_paths(&values, &["TEXINPUTS"]);
            assert!(paths.iter().all(|entry| entry.source == "TEXINPUTS"));
        }
    }

    #[test]
    fn variables_braces_tilde_and_empty_entries_expand_in_order() {
        let get = |name: &str| match name {
            "HOME" => Some("/home/test".into()),
            "TREES" => Some("{$HOME/one,$HOME/two}".into()),
            _ => None,
        };
        assert_eq!(
            expand_with("!!$TREES/{tex,fonts/{tfm,type1}}//::~/extra//:", &get),
            vec![
                "!!/home/test/one/tex//",
                "!!/home/test/one/fonts/tfm//",
                "!!/home/test/one/fonts/type1//",
                "!!/home/test/two/tex//",
                "!!/home/test/two/fonts/tfm//",
                "!!/home/test/two/fonts/type1//",
                "",
                "/home/test/extra//",
                "",
            ]
        );
        assert_eq!(expand_with("${HOME}/tex//", &get), vec!["/home/test/tex//"]);
    }

    #[test]
    fn cyclic_variables_and_brace_expansion_are_bounded() {
        assert_eq!(expand_with("$LOOP", &|_| Some("$LOOP".into())), vec![""]);
        assert_eq!(expand_with("a{broken", &|_| None), vec!["a{broken"]);
    }

    #[test]
    fn relative_database_signatures_use_owned_directory() {
        let parent = std::env::current_dir().unwrap();
        let root = std::env::temp_dir().join(format!(
            "tekai-owned-signature-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(root.join("database")).unwrap();
        let database = root.join("database/ls-R");
        std::fs::write(&database, "./tex/latex:\nfirst.sty\n").unwrap();
        let roots = [SearchPath {
            path: "!!database".into(),
            source: "test",
            bundled: false,
        }];
        let mut directory =
            crate::working_directory::WorkingDirectory::new(Some(root.clone())).unwrap();
        directory
            .as_mut()
            .scope(|| {
                let before = signature_for_roots(Path::new("."), &roots);
                assert_eq!(before, signature_for_roots(&root, &roots));
                std::fs::write(&database, "./tex/latex:\nfirst.sty\nsecond.sty\n").unwrap();
                let after = signature_for_roots(Path::new("."), &roots);
                assert_eq!(after, signature_for_roots(&root, &roots));
                assert_ne!(after, before);
                assert_eq!(std::env::current_dir().unwrap(), parent);
            })
            .unwrap();
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn signatures_ignore_unsearched_docs_and_database_only_disk_changes() {
        let root = std::env::temp_dir().join(format!("tekai-signature-{}", std::process::id()));
        std::fs::create_dir_all(root.join("doc")).unwrap();
        std::fs::create_dir_all(root.join("tex/latex")).unwrap();
        let roots = vec![SearchPath {
            path: root.to_string_lossy().into_owned(),
            source: "test",
            bundled: false,
        }];
        let before = signature_for_roots(Path::new("."), &roots);
        for n in 0..1000 {
            std::fs::create_dir_all(root.join(format!("doc/{n}/unused"))).unwrap();
        }
        assert_eq!(signature_for_roots(Path::new("."), &roots), before);
        std::fs::create_dir(root.join("tex/latex/new-package")).unwrap();
        assert_eq!(signature_for_roots(Path::new("."), &roots), before);
        std::fs::write(root.join("ls-R"), "./tex/latex:\n").unwrap();
        let roots = vec![SearchPath {
            path: format!("!!{}", root.display()),
            source: "test",
            bundled: false,
        }];
        let before = signature_for_roots(Path::new("."), &roots);
        std::fs::write(root.join("tex/latex/not-in-database.sty"), "ignored").unwrap();
        assert_eq!(signature_for_roots(Path::new("."), &roots), before);
        std::fs::write(root.join("ls-R"), "./tex/latex:\nnot-in-database.sty\n").unwrap();
        assert_ne!(signature_for_roots(Path::new("."), &roots), before);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn database_signatures_observe_edits_and_replacements_with_preserved_mtime() {
        use std::fs::{self, File, FileTimes};

        let root = std::env::temp_dir().join(format!(
            "tekai-database-signature-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        let database = root.join("ls-R");
        let roots = [SearchPath {
            path: format!("!!{}", root.display()),
            source: "test",
            bundled: false,
        }];
        for atomic in [false, true] {
            fs::write(&database, "./tex/latex/old:\nchoice.sty\n").unwrap();
            let initial = fs::metadata(&database).unwrap();
            let before = signature_for_roots(Path::new("."), &roots);
            let destination = if atomic {
                root.join("ls-R.replacement")
            } else {
                database.clone()
            };
            fs::write(&destination, "./tex/latex/new:\nchoice.sty\n").unwrap();
            File::open(&destination)
                .unwrap()
                .set_times(FileTimes::new().set_modified(initial.modified().unwrap()))
                .unwrap();
            if atomic {
                fs::rename(destination, &database).unwrap();
            }
            let current = fs::metadata(&database).unwrap();
            assert_eq!(current.len(), initial.len());
            assert_eq!(current.modified().unwrap(), initial.modified().unwrap());
            assert_ne!(signature_for_roots(Path::new("."), &roots), before);
            assert_eq!(
                signature_for_roots(Path::new("."), &roots),
                signature_for_roots(Path::new("."), &roots)
            );
        }
        fs::remove_dir_all(root).unwrap();
    }
}

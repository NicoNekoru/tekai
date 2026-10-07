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
    let mode = std::env::var("TEKAI_TEXMF_MODE").unwrap_or_else(|_| "shared".into());
    match mode.as_str() {
        "shared" | "bundled" => Ok(mode),
        _ => Err(std::io::Error::other(format!(
            "invalid TEKAI_TEXMF_MODE {mode:?}; expected shared or bundled"
        ))),
    }
}

fn variable(name: &str) -> Option<String> {
    std::env::var(name).ok().or_else(|| match name {
        "TEXMFHOME" => std::env::var("HOME").ok().map(|home| {
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

    let configured = variables.iter().find_map(|key| {
        let value = if *key == "TEXINPUTS" {
            std::env::var("TEXINPUTS_pdflatex")
                .ok()
                .or_else(|| std::env::var(key).ok())
        } else {
            std::env::var(key).ok()
        };
        value.map(|value| (*key, expand_path(&value)))
    });
    let Some((key, entries)) = configured else {
        return defaults;
    };
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
                path,
                source: key,
                bundled: false,
            });
        }
    }
    paths
}

/// Include directory membership in build signatures. Used-file content is
/// fingerprinted separately by the recorder. This catches newly added files
/// that shadow a previously used package without changing that old file.
pub fn tree_signature(base: &Path) -> String {
    let mut signature = String::new();
    for root in shared_roots() {
        signature.push_str(&root.path);
        let root = base.join(root.path.strip_prefix("!!").unwrap_or(&root.path));
        for entry in walkdir::WalkDir::new(root)
            .follow_links(true)
            .sort_by_file_name()
            .into_iter()
        {
            let Ok(entry) = entry else {
                continue;
            };
            if entry.file_type().is_dir() || entry.file_name() == "ls-R" {
                signature.push_str(&entry.path().to_string_lossy());
                if let Ok(metadata) = entry.metadata() {
                    signature.push_str(&format!(
                        "{:?}:{}",
                        metadata.modified().ok(),
                        metadata.len()
                    ));
                }
            }
        }
    }
    signature
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

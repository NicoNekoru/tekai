//! Mutable search trees are indexed once per lookup session, not once per
//! filename or recursive wildcard. A build/pass owns a session. TeX writes are
//! registered as they happen; external edits are observed by the next session.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

use crate::cache::BudgetCache;

const INDEX_BUDGET: usize = 32 * 1024 * 1024;
const INDEX_LIMIT: usize = 128;

#[derive(Clone, Debug, Eq, PartialEq)]
struct FileStamp {
    modified: Option<SystemTime>,
    len: u64,
    #[cfg(unix)]
    identity: (u64, u64, i64, i64),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum IndexKey {
    Disk(PathBuf),
    Database(PathBuf),
}

struct FileIndex {
    root: PathBuf,
    by_name: HashMap<OsString, Vec<PathBuf>>,
    bytes: usize,
    stamp: Option<FileStamp>,
    database: bool,
    aliases: Vec<(PathBuf, PathBuf)>,
}

impl FileIndex {
    fn new(root: PathBuf) -> Self {
        let bytes = root.capacity() * 2 + std::mem::size_of::<Self>() + 128;
        Self {
            root,
            by_name: HashMap::new(),
            bytes,
            stamp: None,
            database: false,
            aliases: Vec::new(),
        }
    }

    fn add(&mut self, path: PathBuf) {
        let Some(name) = path.file_name() else { return };
        let paths = self.by_name.entry(name.to_os_string()).or_insert_with(|| {
            // Include key, bucket and allocation overhead in the retention cost.
            self.bytes += name.len() + 128;
            Vec::new()
        });
        self.bytes += path.capacity() + 64;
        paths.push(path);
    }

    fn find(&self, requested: &Path, pattern: &DirectoryPattern) -> Option<PathBuf> {
        let paths = self.by_name.get(requested.file_name()?)?;
        if self.database {
            return paths
                .iter()
                .find(|path| match_rank(path, requested, pattern).is_some() && path.is_file())
                .cloned();
        }
        best_match(paths.iter().map(PathBuf::as_path), requested, pattern)
    }
}

struct LookupSession {
    indices: BudgetCache<IndexKey, FileIndex>,
    budget: usize,
    // Oversized trees use a streaming fallback, without repeatedly allocating
    // an index that cannot be retained. This set is bounded too.
    oversized: Vec<IndexKey>,
    #[cfg(test)]
    disk_scans: usize,
    #[cfg(test)]
    database_reads: usize,
}

impl LookupSession {
    fn new() -> Self {
        Self {
            indices: BudgetCache::new(INDEX_BUDGET, INDEX_LIMIT),
            budget: INDEX_BUDGET,
            oversized: Vec::new(),
            #[cfg(test)]
            disk_scans: 0,
            #[cfg(test)]
            database_reads: 0,
        }
    }

    fn find(&mut self, base: &Path, candidate: &str, entry: &str) -> Option<PathBuf> {
        let database_only = entry.starts_with("!!");
        let entry = entry.strip_prefix("!!").unwrap_or(entry);
        if entry.is_empty() {
            return None;
        }
        if !database_only && !entry.contains("//") {
            let path = base.join(entry).join(candidate);
            return path.is_file().then_some(path);
        }
        let prefix = base.join(entry.split("//").next()?);
        let root = if database_only {
            absolute_lexical(&prefix)?
        } else {
            prefix.canonicalize().ok()?
        };
        let suffix = entry.split_once("//").map(|(_, suffix)| suffix);
        let pattern = DirectoryPattern::new(&root, suffix);
        let requested = Path::new(candidate);
        if database_only {
            let db = root
                .ancestors()
                .map(|root| root.join("ls-R"))
                .find(|db| db.is_file())?;
            let key = IndexKey::Database(db.clone());
            let stamp = file_stamp(&db);
            if let Some(index) = self.indices.get(&key) {
                if index.stamp == stamp {
                    return index.find(requested, &pattern);
                }
            }
            self.indices.remove(&key);
            #[cfg(test)]
            {
                self.database_reads += 1;
            }
            let mut index = FileIndex::new(db.parent()?.to_path_buf());
            index.database = true;
            if self.oversized.contains(&key) {
                index.bytes = self.budget + 1;
            }
            let found = read_database(&db, &mut index, requested, &pattern, self.budget);
            index.stamp = stamp;
            if index.bytes <= self.budget {
                let bytes = index.bytes;
                self.indices.insert(key, index, bytes);
            } else if !self.oversized.contains(&key) {
                if self.oversized.len() == INDEX_LIMIT {
                    self.oversized.remove(0);
                }
                self.oversized.push(key);
            }
            return found;
        }
        // A zero-directory wildcard always searches the root first. This also
        // sees freshly generated files without a recursive scan.
        if let Some(suffix) = suffix {
            if !suffix.contains("//") {
                let path = root.join(suffix).join(requested);
                if path.is_file() {
                    return Some(path);
                }
            }
        }
        // A previously indexed ancestor supplies views for overlapping paths
        // such as project//, project/out// and the default .//.
        let key = self
            .indices
            .keys()
            .filter_map(|key| match key {
                IndexKey::Disk(path) if root.starts_with(path) => Some(key.clone()),
                _ => None,
            })
            .min_by_key(|key| match key {
                IndexKey::Disk(path) => path.components().count(),
                _ => 0,
            });
        if let Some(key) = key {
            return self.indices.get(&key)?.find(requested, &pattern);
        }
        let key = IndexKey::Disk(root.clone());
        #[cfg(test)]
        {
            self.disk_scans += 1;
        }
        if self.oversized.contains(&key) {
            return best_match(
                disk_entries(&root)
                    .filter(|entry| entry.file_type().is_file())
                    .map(|entry| entry.into_path()),
                requested,
                &pattern,
            );
        }
        let mut index = FileIndex::new(root.clone());
        let mut best: Option<(Vec<PathBuf>, PathBuf)> = None;
        for entry in disk_entries(&root) {
            if entry.file_type().is_dir() {
                if index.bytes <= self.budget && entry.path_is_symlink() {
                    if let Ok(target) = entry.path().canonicalize() {
                        let alias = entry.into_path();
                        index.bytes += target.capacity() + alias.capacity() + 128;
                        index.aliases.push((target, alias));
                    }
                }
                continue;
            }
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.into_path();
            consider_match(&path, requested, &pattern, &mut best);
            if index.bytes <= self.budget {
                index.add(path);
            }
        }
        if index.bytes <= self.budget {
            let bytes = index.bytes;
            self.indices.insert(key, index, bytes);
        } else {
            if self.oversized.len() == INDEX_LIMIT {
                self.oversized.remove(0);
            }
            self.oversized.push(key);
        }
        best.map(|(_, path)| path)
    }

    fn record_output(&mut self, path: &Path) {
        if !self
            .indices
            .keys()
            .any(|key| matches!(key, IndexKey::Disk(_)))
        {
            return;
        }
        let Some(parent) = path.parent().and_then(|parent| {
            if parent.as_os_str().is_empty() {
                Path::new(".")
            } else {
                parent
            }
            .canonicalize()
            .ok()
        }) else {
            return;
        };
        let Some(name) = path.file_name() else { return };
        let path = parent.join(name);
        // A leaf symlink can create a different basename in another tree.
        // Reset rather than inventing alias spellings for an arbitrary target.
        if fs::symlink_metadata(&path).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
            *self = Self::new();
            return;
        }
        let keys = self
            .indices
            .keys()
            .filter(|key| matches!(key, IndexKey::Disk(_)))
            .cloned()
            .collect::<Vec<_>>();
        for key in keys {
            if !self.indices.peek(&key).is_some_and(|index| {
                path.starts_with(&index.root)
                    || index
                        .aliases
                        .iter()
                        .any(|(target, _)| path.starts_with(target))
            }) {
                continue;
            }
            if let Some(mut index) = self.indices.remove(&key) {
                let mut spellings = Vec::new();
                if path.starts_with(&index.root) {
                    spellings.push(path.clone());
                }
                // Alias directories can be empty when inventoried. A later
                // output must become visible through every alias, not just its
                // canonical parent or the spelling used by the writer.
                for (target, alias) in &index.aliases {
                    if let Ok(tail) = path.strip_prefix(target) {
                        spellings.push(alias.join(tail));
                    }
                }
                for spelling in spellings {
                    if !index
                        .by_name
                        .get(name)
                        .is_some_and(|paths| paths.contains(&spelling))
                    {
                        index.add(spelling);
                        if index.bytes > self.budget {
                            break;
                        }
                    }
                }
                let bytes = index.bytes;
                self.indices.insert(key, index, bytes);
            }
        }
    }
}

fn disk_entries(root: &Path) -> impl Iterator<Item = walkdir::DirEntry> {
    let bundle = crate::runtime::installed_bundle_root();
    walkdir::WalkDir::new(root)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(move |entry| {
            !bundle.is_some_and(|path| {
                entry.path() == path
                    || (entry.path_is_symlink()
                        && entry.path().canonicalize().ok().as_deref() == Some(path))
            })
        })
        .filter_map(Result::ok)
}

fn file_stamp(path: &Path) -> Option<FileStamp> {
    let metadata = fs::metadata(path).ok()?;
    #[cfg(unix)]
    use std::os::unix::fs::MetadataExt;
    Some(FileStamp {
        modified: metadata.modified().ok(),
        len: metadata.len(),
        #[cfg(unix)]
        identity: (
            metadata.dev(),
            metadata.ino(),
            metadata.ctime(),
            metadata.ctime_nsec(),
        ),
    })
}

fn read_database(
    db: &Path,
    index: &mut FileIndex,
    requested: &Path,
    pattern: &DirectoryPattern,
    budget: usize,
) -> Option<PathBuf> {
    let reader = BufReader::new(File::open(db).ok()?);
    let mut directory = index.root.clone();
    let mut best = None;
    for line in reader.lines().map_while(Result::ok) {
        if let Some(dir) = line.strip_suffix(':') {
            directory = if Path::new(dir)
                .components()
                .all(|part| matches!(part, Component::Normal(_) | Component::CurDir))
            {
                index.root.join(dir)
            } else {
                PathBuf::new()
            };
        } else if !directory.as_os_str().is_empty()
            && Path::new(&line).components().count() == 1
            && matches!(
                Path::new(&line).components().next(),
                Some(Component::Normal(_))
            )
        {
            let path = directory.join(&line);
            if best.is_none() {
                if let Some(rank) = match_rank(&path, requested, pattern).filter(|_| path.is_file())
                {
                    best = Some((rank, path.clone()));
                }
            }
            if index.bytes <= budget {
                index.add(path);
            }
        }
    }
    best.map(|(_, path)| path)
}

fn best_match<P: AsRef<Path>>(
    paths: impl Iterator<Item = P>,
    requested: &Path,
    pattern: &DirectoryPattern,
) -> Option<PathBuf> {
    let mut best = None;
    for path in paths {
        consider_match(path.as_ref(), requested, pattern, &mut best);
    }
    best.map(|(_, path)| path)
}

fn consider_match(
    path: &Path,
    requested: &Path,
    pattern: &DirectoryPattern,
    best: &mut Option<(Vec<PathBuf>, PathBuf)>,
) {
    let Some(rank) = match_rank(path, requested, pattern) else {
        return;
    };
    if best.as_ref().is_none_or(|(previous, _)| rank < *previous) && path.is_file() {
        *best = Some((rank, path.to_path_buf()));
    }
}

fn match_rank(path: &Path, requested: &Path, pattern: &DirectoryPattern) -> Option<Vec<PathBuf>> {
    if !path.ends_with(requested) {
        return None;
    }
    let mut directory = path.to_path_buf();
    for _ in requested.components() {
        directory.pop();
    }
    pattern.rank(&directory)
}

fn absolute_lexical(path: &Path) -> Option<PathBuf> {
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(path)
    };
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            component => normalized.push(component.as_os_str()),
        }
    }
    Some(normalized)
}

/// A dynamic-programming wildcard matcher. The rank reproduces nested
/// recursive searches' outer-first, sorted-directory order without revisiting
/// subtrees or exponentially backtracking through repeated `//` wildcards.
struct DirectoryPattern {
    root: PathBuf,
    parts: Vec<Option<OsString>>,
}

impl DirectoryPattern {
    fn new(root: &Path, suffix: Option<&str>) -> Self {
        let mut parts = Vec::new();
        if let Some(suffix) = suffix {
            parts.push(None);
            for (i, part) in suffix.split("//").enumerate() {
                if i > 0 {
                    parts.push(None);
                }
                parts.extend(Path::new(part).components().filter_map(|part| match part {
                    Component::CurDir => None,
                    _ => Some(Some(part.as_os_str().to_os_string())),
                }));
            }
        }
        Self {
            root: root.to_path_buf(),
            parts,
        }
    }

    fn rank(&self, directory: &Path) -> Option<Vec<PathBuf>> {
        let relative = directory.strip_prefix(&self.root).ok()?;
        let dirs = relative
            .components()
            .map(|part| part.as_os_str())
            .collect::<Vec<_>>();
        let mut dp = vec![vec![false; dirs.len() + 1]; self.parts.len() + 1];
        dp[self.parts.len()][dirs.len()] = true;
        for p in (0..self.parts.len()).rev() {
            for d in (0..=dirs.len()).rev() {
                dp[p][d] = match &self.parts[p] {
                    None => dp[p + 1][d] || (d < dirs.len() && dp[p][d + 1]),
                    Some(part) => d < dirs.len() && part == dirs[d] && dp[p + 1][d + 1],
                };
            }
        }
        if !dp[0][0] {
            return None;
        }
        let mut rank = Vec::new();
        let mut d = 0;
        for (p, part) in self.parts.iter().enumerate() {
            if part.is_none() {
                while !dp[p + 1][d] {
                    d += 1;
                }
                rank.push(dirs[..d].iter().collect());
            } else {
                d += 1;
            }
        }
        rank.push(relative.to_path_buf());
        Some(rank)
    }
}

thread_local! { static SESSION: RefCell<LookupSession> = RefCell::new(LookupSession::new()); }

/// Begin a build/pass's view of mutable search trees. Call again after external
/// filesystem edits. This drops both successful and unsuccessful old lookups.
pub fn reset() {
    SESSION.with(|session| *session.borrow_mut() = LookupSession::new());
}

pub(crate) fn find(base: &Path, candidate: &str, entry: &Path) -> Option<PathBuf> {
    SESSION.with(|session| {
        session
            .borrow_mut()
            .find(base, candidate, &entry.to_string_lossy())
    })
}

pub(crate) fn record_output(path: &Path) {
    SESSION.with(|session| session.borrow_mut().record_output(path));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    fn fixture() -> PathBuf {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let root = std::env::temp_dir().join(format!(
            "tekai-lookup-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&root).unwrap();
        root.canonicalize().unwrap()
    }

    #[test]
    fn overlapping_paths_and_misses_share_one_scan() {
        let root = fixture();
        for n in 0..1000 {
            fs::create_dir_all(root.join(format!("junk/{n}/leaf"))).unwrap();
        }
        fs::create_dir_all(root.join("out")).unwrap();
        fs::write(root.join("junk/500/leaf/local.sty"), "local").unwrap();
        let mut session = LookupSession::new();
        assert!(session.find(&root, "local.sty", ".//").is_some());
        for n in 0..100 {
            assert!(session
                .find(&root, &format!("missing-{n}.sty"), ".//")
                .is_none());
            assert!(session.find(&root, "missing.sty", "out//").is_none());
        }
        assert_eq!(session.disk_scans, 1);
        let generated = root.join("junk/500/leaf/generated.tex");
        fs::write(&generated, "generated").unwrap();
        session.record_output(&generated);
        assert_eq!(session.find(&root, "generated.tex", ".//"), Some(generated));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn wildcard_priority_qualified_names_and_database_refresh() {
        let root = fixture();
        for dir in ["tex/z", "a/tex/y"] {
            fs::create_dir_all(root.join(dir)).unwrap();
            fs::write(root.join(dir).join("local.sty"), dir).unwrap();
        }
        let mut session = LookupSession::new();
        assert_eq!(
            session.find(&root, "local.sty", ".//tex//"),
            Some(root.join("tex/z/local.sty"))
        );
        assert_eq!(
            session.find(&root, "y/local.sty", ".//tex//"),
            Some(root.join("a/tex/y/local.sty"))
        );
        assert!(session.find(&root, "local.sty", "!!.//tex//").is_none());
        fs::write(root.join("ls-R"), "./tex/z:\nlocal.sty\n").unwrap();
        assert_eq!(
            session.find(&root, "local.sty", "!!.//tex//"),
            Some(root.join("tex/z/local.sty"))
        );
        for _ in 0..100 {
            assert!(session.find(&root, "absent", "!!.//").is_none());
        }
        assert_eq!(session.database_reads, 1);
        fs::write(
            root.join("ls-R"),
            "./a/tex/y:\nlocal.sty\n../../escape:\nlocal.sty\n",
        )
        .unwrap();
        assert_eq!(
            session.find(&root, "local.sty", "!!.//"),
            Some(root.join("a/tex/y/local.sty"))
        );
        assert_eq!(session.database_reads, 2);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn many_wildcards_do_not_backtrack_exponentially() {
        let pattern = DirectoryPattern::new(
            Path::new("/tree"),
            Some(&format!("{}missing", "a//".repeat(40))),
        );
        let directory = PathBuf::from(format!("/tree/{}", "a/".repeat(80)));
        assert!(pattern.rank(&directory).is_none());
    }

    #[test]
    fn oversized_indices_stream_without_retaining_the_tree() {
        let root = fixture();
        for n in 0..20 {
            fs::create_dir_all(root.join(format!("{n}"))).unwrap();
            fs::write(root.join(format!("{n}/local.sty")), "local").unwrap();
        }
        let mut session = LookupSession::new();
        session.budget = 512;
        session.indices = BudgetCache::new(512, INDEX_LIMIT);
        assert_eq!(
            session.find(&root, "local.sty", ".//"),
            Some(root.join("0/local.sty"))
        );
        assert_eq!(
            session.find(&root, "local.sty", ".//"),
            Some(root.join("0/local.sty"))
        );
        assert_eq!(session.indices.retained_bytes(), 0);
        assert_eq!(session.oversized.len(), 1);
        fs::write(
            root.join("ls-R"),
            (0..20)
                .map(|n| format!("./{n}:\nlocal.sty\n"))
                .collect::<String>(),
        )
        .unwrap();
        for _ in 0..2 {
            assert_eq!(
                session.find(&root, "local.sty", "!!.//"),
                Some(root.join("0/local.sty"))
            );
        }
        assert_eq!(session.indices.retained_bytes(), 0);
        assert_eq!(session.oversized.len(), 2);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn indexed_search_matches_recursive_directory_order() {
        fn oracle(base: &Path, candidate: &str, entry: &str) -> Option<PathBuf> {
            if let Some((prefix, suffix)) = entry.split_once("//") {
                for dir in walkdir::WalkDir::new(base.join(prefix))
                    .follow_links(true)
                    .sort_by_file_name()
                    .into_iter()
                    .filter_map(Result::ok)
                {
                    if dir.file_type().is_dir() {
                        if let Some(path) = oracle(
                            dir.path(),
                            candidate,
                            if suffix.is_empty() { "." } else { suffix },
                        ) {
                            return Some(path);
                        }
                    }
                }
                None
            } else {
                let path = base.join(entry).join(candidate);
                path.is_file().then_some(path)
            }
        }
        let root = fixture();
        for dir in [
            "tex/z",
            "a/tex/y",
            "tex/a/tex",
            "a/tex/a/tex",
            "a/tex/y/end",
            "empty",
            "tex/z/end",
        ] {
            fs::create_dir_all(root.join(dir)).unwrap();
            fs::write(root.join(dir).join("match.sty"), dir).unwrap();
        }
        let mut session = LookupSession::new();
        for pattern in [
            ".//",
            ".//tex",
            ".//tex//",
            ".//tex//tex//",
            ".//tex//end",
            ".//tex//tex//end",
        ] {
            for name in ["match.sty", "end/match.sty", "z/match.sty", "missing.sty"] {
                let actual = session
                    .find(&root, name, pattern)
                    .map(|path| path.canonicalize().unwrap());
                let expected =
                    oracle(&root, name, pattern).map(|path| path.canonicalize().unwrap());
                assert_eq!(actual, expected, "pattern {pattern}, name {name}");
            }
        }
        assert_eq!(session.disk_scans, 1);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn replacing_a_database_with_preserved_mtime_and_size_refreshes_it() {
        let root = fixture();
        for dir in ["a", "b"] {
            fs::create_dir(root.join(dir)).unwrap();
            fs::write(root.join(dir).join("match.sty"), dir).unwrap();
        }
        let db = root.join("ls-R");
        fs::write(&db, "./a:\nmatch.sty\n").unwrap();
        let mtime = fs::metadata(&db).unwrap().modified().unwrap();
        let mut session = LookupSession::new();
        assert_eq!(
            session.find(&root, "match.sty", "!!.//"),
            Some(root.join("a/match.sty"))
        );
        let replacement = root.join("new-ls-R");
        fs::write(&replacement, "./b:\nmatch.sty\n").unwrap();
        File::options()
            .write(true)
            .open(&replacement)
            .unwrap()
            .set_times(fs::FileTimes::new().set_modified(mtime))
            .unwrap();
        fs::rename(&replacement, &db).unwrap();
        assert_eq!(
            session.find(&root, "match.sty", "!!.//"),
            Some(root.join("b/match.sty"))
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn database_prefixes_preserve_symlink_directory_names() {
        let root = fixture();
        let external = fixture();
        fs::create_dir(external.join("deep")).unwrap();
        fs::write(external.join("deep/shared.sty"), "shared").unwrap();
        std::os::unix::fs::symlink(&external, root.join("tex")).unwrap();
        fs::write(root.join("ls-R"), "./tex/deep:\nshared.sty\n").unwrap();
        let mut session = LookupSession::new();
        assert_eq!(
            session.find(&root, "shared.sty", "!!tex//"),
            Some(root.join("tex/deep/shared.sty"))
        );
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn outputs_in_initially_empty_symlink_directories_update_every_alias() {
        let root = fixture();
        let external = fixture();
        for alias in ["a", "z"] {
            std::os::unix::fs::symlink(&external, root.join(alias)).unwrap();
        }
        let mut session = LookupSession::new();
        assert!(session.find(&root, "created.tex", ".//").is_none());
        fs::write(root.join("z/created.tex"), "generated").unwrap();
        session.record_output(&root.join("z/created.tex"));
        assert_eq!(
            session.find(&root, "created.tex", ".//"),
            Some(root.join("a/created.tex"))
        );
        assert_eq!(
            session.find(&root, "created.tex", ".//z//"),
            Some(root.join("z/created.tex"))
        );
        assert_eq!(session.disk_scans, 1);
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn writing_through_a_dangling_file_symlink_refreshes_other_roots() {
        let root = fixture();
        let external = fixture();
        fs::create_dir(external.join("deep")).unwrap();
        let target = external.join("deep/created.tex");
        let alias = root.join("alias.tex");
        std::os::unix::fs::symlink(&target, &alias).unwrap();
        let mut session = LookupSession::new();
        assert!(session.find(&root, "absent", ".//").is_none());
        assert!(session.find(&external, "created.tex", ".//").is_none());
        fs::write(&alias, "created").unwrap();
        session.record_output(&alias);
        assert_eq!(session.find(&external, "created.tex", ".//"), Some(target));
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }
}

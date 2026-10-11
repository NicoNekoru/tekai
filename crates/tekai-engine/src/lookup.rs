//! Mutable search trees are indexed once per lookup session, not once per
//! filename or recursive wildcard. A build/pass owns a session. TeX writes are
//! registered as they happen; external edits are observed by the next session.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::path::{Component, Path, PathBuf};
use std::rc::Rc;
use std::time::SystemTime;

use crate::cache::BudgetCache;
use crate::directory_graph::DirectoryGraph;

const INDEX_BUDGET: usize = 32 * 1024 * 1024;
const INDEX_LIMIT: usize = 128;

#[derive(Clone, Debug, Eq, PartialEq)]
struct FileStamp {
    modified: Option<SystemTime>,
    len: u64,
    identity: Option<crate::file_identity::ChangeIdentity>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum IndexKey {
    Disk(PathBuf, bool),
    Database(PathBuf),
}

struct FileIndex {
    root: PathBuf,
    by_name: HashMap<OsString, Vec<PathBuf>>,
    bytes: usize,
    stamp: Option<FileStamp>,
    database: bool,
    graph: Option<DirectoryGraph>,
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
            graph: None,
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
        if let Some(graph) = &self.graph {
            return if pattern.root == self.root {
                graph.find(pattern.suffix.as_deref(), requested)
            } else {
                graph.find_from(&pattern.root, pattern.suffix.as_deref(), requested)
            };
        }
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
    exclusion_config: Option<OsString>,
    policy_generation: u64,
    project_exclusions: Vec<PathBuf>,
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
            exclusion_config: std::env::var_os("TEKAI_ENGINE_PROJECT_EXCLUDES"),
            policy_generation: POLICY_GENERATION.with(Cell::get),
            project_exclusions: project_exclusions(),
            #[cfg(test)]
            disk_scans: 0,
            #[cfg(test)]
            database_reads: 0,
        }
    }

    fn find(&mut self, base: &Path, candidate: &str, entry: &str) -> Option<PathBuf> {
        self.find_policy(base, candidate, entry, false)
    }

    fn find_policy(
        &mut self,
        base: &Path,
        candidate: &str,
        entry: &str,
        implicit_project: bool,
    ) -> Option<PathBuf> {
        if self.exclusion_config != std::env::var_os("TEKAI_ENGINE_PROJECT_EXCLUDES")
            || self.policy_generation != POLICY_GENERATION.with(Cell::get)
        {
            *self = Self::new();
        }
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
                if stamp.as_ref().is_some_and(|stamp| stamp.identity.is_some())
                    && index.stamp == stamp
                {
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
        // Explicit path components retain direct access to generated inputs;
        // only implicit recursive filename discovery skips owned namespaces.
        if implicit_project && requested.components().count() > 1 {
            let direct = root.join(requested);
            if direct.is_file() {
                return Some(direct);
            }
        }
        if implicit_project
            && self
                .project_exclusions
                .iter()
                .any(|excluded| root.starts_with(excluded))
        {
            return None;
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
        // Physical nodes supply views for overlapping roots, including an
        // external directory reached through a previously inventoried alias.
        let key = self
            .indices
            .keys()
            .filter_map(|key| match key {
                IndexKey::Disk(_, policy) if *policy == implicit_project => self
                    .indices
                    .peek(key)
                    .and_then(|index| index.graph.as_ref())
                    .filter(|graph| graph.contains_directory(&root))
                    .map(|_| key.clone()),
                _ => None,
            })
            .min_by(|left, right| match (left, right) {
                (IndexKey::Disk(left, _), IndexKey::Disk(right, _)) => left
                    .components()
                    .count()
                    .cmp(&right.components().count())
                    .then_with(|| left.cmp(right)),
                _ => std::cmp::Ordering::Equal,
            });
        if let Some(key) = key {
            return self.indices.get(&key)?.find(requested, &pattern);
        }
        let key = IndexKey::Disk(root.clone(), implicit_project);
        let mut excluded = crate::runtime::installed_bundle_root()
            .map(Path::to_path_buf)
            .into_iter()
            .collect::<Vec<_>>();
        if implicit_project {
            excluded.extend(self.project_exclusions.iter().cloned());
        }
        #[cfg(test)]
        {
            self.disk_scans += 1;
        }
        if self.oversized.contains(&key) {
            return best_match(
                disk_entries(&root, excluded)
                    .filter(|entry| entry.file_type().is_file())
                    .map(|entry| entry.into_path()),
                requested,
                &pattern,
            );
        }
        let mut index = FileIndex::new(root.clone());
        if let Some(graph) = DirectoryGraph::build_excluding(
            &root,
            &excluded.iter().map(PathBuf::as_path).collect::<Vec<_>>(),
            self.budget.saturating_sub(index.bytes),
        ) {
            index.bytes += graph.retained_bytes();
            index.graph = Some(graph);
            let found = index.find(requested, &pattern);
            let bytes = index.bytes;
            self.indices.insert(key, index, bytes);
            return found;
        }
        if self.oversized.len() == INDEX_LIMIT {
            self.oversized.remove(0);
        }
        self.oversized.push(key);
        best_match(
            disk_entries(&root, excluded)
                .filter(|entry| entry.file_type().is_file())
                .map(|entry| entry.into_path()),
            requested,
            &pattern,
        )
    }

    fn record_output(&mut self, path: &Path) {
        if !self
            .indices
            .keys()
            .any(|key| matches!(key, IndexKey::Disk(_, _)))
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
            .filter(|key| matches!(key, IndexKey::Disk(_, _)))
            .cloned()
            .collect::<Vec<_>>();
        for key in keys {
            if !self.indices.peek(&key).is_some_and(|index| {
                path.starts_with(&index.root)
                    || index.graph.as_ref().is_some_and(|graph| {
                        parent
                            .ancestors()
                            .any(|ancestor| graph.contains_directory(ancestor))
                    })
            }) {
                continue;
            }
            if let Some(mut index) = self.indices.remove(&key) {
                let Some(graph) = &mut index.graph else {
                    continue;
                };
                let previous = graph.retained_bytes();
                // A newly created directory invalidates the inventory. Known
                // parents only need one file insertion, shared by every alias.
                if !graph.add_output(&parent, name) {
                    continue;
                }
                index.bytes = index.bytes - previous + graph.retained_bytes();
                let bytes = index.bytes;
                self.indices.insert(key, index, bytes);
            }
        }
    }
}

fn disk_entries(root: &Path, excluded: Vec<PathBuf>) -> impl Iterator<Item = walkdir::DirEntry> {
    walkdir::WalkDir::new(root)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(move |entry| {
            !excluded.iter().any(|path| {
                entry.path().starts_with(path)
                    || (entry.path_is_symlink()
                        && entry
                            .path()
                            .canonicalize()
                            .is_ok_and(|target| target.starts_with(path)))
            })
        })
        .filter_map(Result::ok)
}

fn project_exclusions() -> Vec<PathBuf> {
    if let Some(paths) = SCOPED_PROJECT_EXCLUSIONS.with(|paths| paths.borrow().clone()) {
        return paths;
    }
    std::env::var_os("TEKAI_ENGINE_PROJECT_EXCLUDES")
        .map(|value| {
            std::env::split_paths(&value)
                .filter_map(|path| path.canonicalize().ok())
                .collect()
        })
        .unwrap_or_default()
}

pub(crate) fn find_search_path(
    base: &Path,
    candidate: &str,
    entry: &crate::search::SearchPath,
) -> Option<PathBuf> {
    SESSION.with(|session| {
        session
            .borrow_mut()
            .get_or_insert_with(LookupSession::new)
            .find_policy(base, candidate, &entry.path, entry.source == "project")
    })
}

fn file_stamp(path: &Path) -> Option<FileStamp> {
    let metadata = fs::metadata(path).ok()?;
    Some(FileStamp {
        modified: metadata.modified().ok(),
        len: metadata.len(),
        identity: crate::file_identity::change_identity(&metadata),
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
    suffix: Option<String>,
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
            suffix: suffix.map(str::to_owned),
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

thread_local! {
    static SESSION: RefCell<Option<LookupSession>> = const { RefCell::new(None) };
    static SCOPED_PROJECT_EXCLUSIONS: RefCell<Option<Vec<PathBuf>>> = const { RefCell::new(None) };
    static POLICY_GENERATION: Cell<u64> = const { Cell::new(0) };
}

/// A thread-owned implicit-search policy for compiler preflight. Explicit
/// search entries and explicit relative paths keep their normal semantics.
/// Dropping the guard restores the previous policy and invalidates its session.
pub struct ProjectLookupPolicyGuard {
    previous: Option<Vec<PathBuf>>,
    _thread_owned: PhantomData<Rc<()>>,
}

pub fn scoped_project_exclusions(paths: Vec<PathBuf>) -> ProjectLookupPolicyGuard {
    let previous = SCOPED_PROJECT_EXCLUSIONS.with(|current| current.replace(Some(paths)));
    POLICY_GENERATION.with(|generation| generation.set(generation.get().wrapping_add(1)));
    ProjectLookupPolicyGuard {
        previous,
        _thread_owned: PhantomData,
    }
}

impl Drop for ProjectLookupPolicyGuard {
    fn drop(&mut self) {
        SCOPED_PROJECT_EXCLUSIONS.with(|current| current.replace(self.previous.take()));
        POLICY_GENERATION.with(|generation| generation.set(generation.get().wrapping_add(1)));
    }
}

/// Begin a build/pass's view of mutable search trees. Call again after external
/// filesystem edits. This drops both successful and unsuccessful old lookups.
pub fn reset() {
    // A cache-hit build never performs a lookup. Drop prior indices without
    // constructing an empty session that would immediately remain unused.
    SESSION.with(|session| *session.borrow_mut() = None);
}

/// Capture directory membership dependencies for a compiler generation.
///
/// Inventory each physical directory once, with the same alias and exclusion
/// rules as recursive lookup. Persisted directory identities can then be
/// checked without enumerating entries on a warm-cache build. An oversized or
/// unreadable tree declines caching rather than publishing partial coverage.
pub fn dependency_paths(
    base: &Path,
    entries: &[crate::search::SearchPath],
    excluded: &[&Path],
) -> Option<Vec<PathBuf>> {
    dependency_snapshot(base, entries, excluded).map(|dependencies| {
        let mut paths = dependencies
            .into_iter()
            .filter_map(|dependency| match dependency {
                LookupDependency::Path(path) => Some(path),
                LookupDependency::AbsentDirectory(path) | LookupDependency::AbsentFile(path) => {
                    nearest_existing_directory(&path)
                }
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        paths.sort();
        paths
    })
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LookupDependency {
    Path(PathBuf),
    AbsentDirectory(PathBuf),
    AbsentFile(PathBuf),
}

pub fn dependency_snapshot(
    base: &Path,
    entries: &[crate::search::SearchPath],
    excluded: &[&Path],
) -> Option<Vec<LookupDependency>> {
    dependency_paths_with_budget(base, entries, excluded, INDEX_BUDGET)
}

fn dependency_paths_with_budget(
    base: &Path,
    entries: &[crate::search::SearchPath],
    excluded: &[&Path],
    mut budget: usize,
) -> Option<Vec<LookupDependency>> {
    let mut dependencies = HashSet::new();
    let mut graphs: Vec<DirectoryGraph> = Vec::new();
    for entry in entries.iter().filter(|entry| !entry.bundled) {
        let database_only = entry.path.starts_with("!!");
        let spelling = entry.path.strip_prefix("!!").unwrap_or(&entry.path);
        let prefix = base.join(spelling.split("//").next()?);
        if excluded.iter().any(|path| prefix.starts_with(path)) {
            continue;
        }
        add_alias_parent_dependencies(&prefix, &mut dependencies, &mut budget)?;
        if database_only {
            // The nearest ancestral database wins. Track every candidate
            // directory up to that winner, including absent candidates, so a
            // newly created closer ls-R cannot leave the old choice fresh.
            for candidate in prefix.ancestors() {
                let database = candidate.join("ls-R");
                if database.is_file() {
                    add_alias_parent_dependencies(&database, &mut dependencies, &mut budget)?;
                    add_dependency(&mut dependencies, database, &mut budget)?;
                    break;
                }
                add_absent_file_dependency(&mut dependencies, database, &mut budget)?;
            }
            continue;
        }
        let Some(root) = prefix.canonicalize().ok() else {
            if fs::symlink_metadata(&prefix).is_ok_and(|metadata| metadata.is_symlink()) {
                // A missing symlink target can appear without changing the
                // link's parent. Partial membership coverage is unsafe.
                return None;
            }
            add_absent_dependency(&mut dependencies, prefix, &mut budget)?;
            continue;
        };
        if !root.is_dir() {
            add_absent_dependency(&mut dependencies, root, &mut budget)?;
            continue;
        }
        if excluded.iter().any(|path| root.starts_with(path)) {
            continue;
        }
        // Keep the lexical root too, so replacement of a root symlink changes
        // its identity even when the old physical tree remains unchanged.
        add_dependency(&mut dependencies, prefix, &mut budget)?;
        if !spelling.contains("//") {
            add_dependency(&mut dependencies, root, &mut budget)?;
            continue;
        }
        if graphs.iter().any(|graph| graph.contains_directory(&root)) {
            continue;
        }
        let graph = DirectoryGraph::build_excluding(&root, excluded, budget)?;
        if !graph.dependencies_complete() {
            return None;
        }
        budget = budget.checked_sub(graph.retained_bytes())?;
        for directory in graph.dependency_directories() {
            add_dependency(&mut dependencies, directory.to_path_buf(), &mut budget)?;
        }
        graphs.push(graph);
    }
    let mut dependencies = dependencies.into_iter().collect::<Vec<_>>();
    dependencies.sort();
    Some(dependencies)
}

fn add_alias_parent_dependencies(
    path: &Path,
    dependencies: &mut HashSet<LookupDependency>,
    budget: &mut usize,
) -> Option<()> {
    for prefix in path.ancestors() {
        match fs::symlink_metadata(prefix) {
            Ok(metadata) if metadata.is_symlink() => {
                // Any intermediate dangling alias can acquire a target without
                // changing its lexical parent. Decline partial coverage.
                prefix.canonicalize().ok()?;
                add_dependency(dependencies, prefix.parent()?.to_path_buf(), budget)?;
            }
            Ok(_) => {}
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory
                ) => {}
            Err(_) => return None,
        }
    }
    Some(())
}

fn add_dependency(
    dependencies: &mut HashSet<LookupDependency>,
    path: PathBuf,
    budget: &mut usize,
) -> Option<()> {
    let dependency = LookupDependency::Path(path);
    if !dependencies.contains(&dependency) {
        let LookupDependency::Path(path) = &dependency else {
            unreachable!()
        };
        *budget = budget.checked_sub(path.capacity() + std::mem::size_of::<PathBuf>() + 128)?;
        dependencies.insert(dependency);
    }
    Some(())
}

fn add_absent_dependency(
    dependencies: &mut HashSet<LookupDependency>,
    path: PathBuf,
    budget: &mut usize,
) -> Option<()> {
    let dependency = LookupDependency::AbsentDirectory(path);
    if !dependencies.contains(&dependency) {
        let LookupDependency::AbsentDirectory(path) = &dependency else {
            unreachable!()
        };
        *budget = budget.checked_sub(path.capacity() + std::mem::size_of::<PathBuf>() + 128)?;
        dependencies.insert(dependency);
    }
    Some(())
}

fn add_absent_file_dependency(
    dependencies: &mut HashSet<LookupDependency>,
    path: PathBuf,
    budget: &mut usize,
) -> Option<()> {
    let dependency = LookupDependency::AbsentFile(path);
    if !dependencies.contains(&dependency) {
        let LookupDependency::AbsentFile(path) = &dependency else {
            unreachable!()
        };
        *budget = budget.checked_sub(path.capacity() + std::mem::size_of::<PathBuf>() + 128)?;
        dependencies.insert(dependency);
    }
    Some(())
}

fn nearest_existing_directory(path: &Path) -> Option<PathBuf> {
    path.ancestors()
        .find(|path| path.is_dir())
        .map(Path::to_path_buf)
}

pub(crate) fn find(base: &Path, candidate: &str, entry: &Path) -> Option<PathBuf> {
    SESSION.with(|session| {
        session
            .borrow_mut()
            .get_or_insert_with(LookupSession::new)
            .find(base, candidate, &entry.to_string_lossy())
    })
}

pub(crate) fn record_output(path: &Path) {
    SESSION.with(|session| {
        // Before the first lookup there is no cached inventory to update.
        // The first scan observes every output already written on disk.
        if let Some(session) = session.borrow_mut().as_mut() {
            session.record_output(path);
        }
    });
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
    fn implicit_project_exclusions_preserve_explicit_inputs_and_streaming_fallback() {
        let root = fixture();
        fs::create_dir_all(root.join("a-generated/deep")).unwrap();
        fs::create_dir_all(root.join("z-source")).unwrap();
        let generated = root.join("a-generated/deep/choice.sty");
        let source = root.join("z-source/choice.sty");
        fs::write(&generated, "generated").unwrap();
        fs::write(&source, "source").unwrap();
        for budget in [INDEX_BUDGET, 1] {
            let mut session = LookupSession::new();
            session.budget = budget;
            session.project_exclusions = vec![root.join("a-generated")];
            assert_eq!(
                session.find_policy(&root, "choice.sty", ".//", true),
                Some(source.clone())
            );
            assert_eq!(
                session.find(&root, "choice.sty", ".//"),
                Some(generated.clone())
            );
            assert_eq!(
                session.find_policy(&root, "a-generated/deep/choice.sty", ".//", true),
                Some(generated.clone())
            );
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn scoped_policy_restores_sessions_and_is_thread_local() {
        let root = fixture();
        fs::create_dir_all(root.join("a-generated")).unwrap();
        fs::create_dir_all(root.join("z-source")).unwrap();
        let generated = root.join("a-generated/choice.sty");
        let source = root.join("z-source/choice.sty");
        fs::write(&generated, "generated").unwrap();
        fs::write(&source, "source").unwrap();
        let entry = crate::search::SearchPath {
            path: ".//".to_string(),
            source: "project",
            bundled: false,
        };
        reset();
        assert_eq!(
            find_search_path(&root, "choice.sty", &entry),
            Some(generated.clone())
        );
        {
            let _guard = scoped_project_exclusions(vec![root.join("a-generated")]);
            assert_eq!(find_search_path(&root, "choice.sty", &entry), Some(source));
            let thread_root = root.clone();
            let thread_entry = entry.clone();
            assert_eq!(
                std::thread::spawn(move || find_search_path(
                    &thread_root,
                    "choice.sty",
                    &thread_entry
                ))
                .join()
                .unwrap(),
                Some(generated.clone())
            );
        }
        assert_eq!(
            find_search_path(&root, "choice.sty", &entry),
            Some(generated)
        );
        reset();
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn dependency_inventory_deduplicates_overlapping_roots_and_excludes_outputs() {
        let root = fixture();
        fs::create_dir_all(root.join("tree/deep")).unwrap();
        fs::create_dir_all(root.join("out/generated/deep")).unwrap();
        let entries = [".//", "tree//", "missing/nested//"].map(|path| crate::search::SearchPath {
            path: path.to_string(),
            source: "test",
            bundled: false,
        });
        let output = root.join("out");
        let paths = dependency_paths(&root, &entries, &[&output]).unwrap();
        assert_eq!(
            paths
                .iter()
                .filter(|path| **path == root.join("tree/deep"))
                .count(),
            1
        );
        assert!(paths.iter().all(|path| !path.starts_with(&output)));
        assert!(paths.contains(&root));
        assert!(dependency_paths_with_budget(&root, &entries, &[&output], 128).is_none());
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn dependency_inventory_tracks_external_aliases_and_declines_broken_targets() {
        use std::os::unix::fs::symlink;
        let root = fixture();
        let external = fixture();
        fs::create_dir_all(external.join("deep")).unwrap();
        symlink(&external, root.join("alias")).unwrap();
        let entries = [crate::search::SearchPath {
            path: ".//".to_string(),
            source: "test",
            bundled: false,
        }];
        let paths = dependency_paths(&root, &entries, &[]).unwrap();
        assert!(paths.contains(&external.join("deep")));
        symlink(external.join("absent"), root.join("broken")).unwrap();
        assert!(dependency_paths(&root, &entries, &[]).is_none());
        let intermediate = [crate::search::SearchPath {
            path: "broken/subdir//".to_string(),
            source: "test",
            bundled: false,
        }];
        assert!(dependency_paths(&root, &intermediate, &[]).is_none());
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }

    #[test]
    fn dependency_inventory_tracks_nearer_database_choices_and_small_path_budgets() {
        let root = fixture();
        let leaf = root.join("tex/latex");
        fs::create_dir_all(&leaf).unwrap();
        fs::write(root.join("ls-R"), "./tex/latex:\nchoice.sty\n").unwrap();
        let entries = [crate::search::SearchPath {
            path: "!!tex/latex//".to_string(),
            source: "test",
            bundled: false,
        }];
        let paths = dependency_paths(&root, &entries, &[]).unwrap();
        assert!(paths.contains(&root.join("ls-R")));
        assert!(paths.contains(&leaf));
        assert!(paths.contains(&root.join("tex")));
        assert!(dependency_paths_with_budget(&root, &entries, &[], 1).is_none());
        for entry in [".", "missing/leaf"] {
            let entries = [crate::search::SearchPath {
                path: entry.to_string(),
                source: "test",
                bundled: false,
            }];
            assert!(dependency_paths_with_budget(&root, &entries, &[], 1).is_none());
        }
        fs::write(root.join("not-a-directory"), "file").unwrap();
        let entries = [crate::search::SearchPath {
            path: "not-a-directory/missing//".to_string(),
            source: "test",
            bundled: false,
        }];
        assert_eq!(
            dependency_paths(&root, &entries, &[]).unwrap(),
            vec![root.clone()]
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn unused_sessions_stay_lazy_and_reset_drops_retained_indices() {
        let root = fixture();
        let entry = PathBuf::from(format!("{}//", root.display()));
        let nested = root.join("nested");
        fs::create_dir(&nested).unwrap();
        reset();
        SESSION.with(|session| assert!(session.borrow().is_none()));
        fs::write(nested.join("first.tex"), "first").unwrap();
        record_output(&nested.join("first.tex"));
        SESSION.with(|session| assert!(session.borrow().is_none()));
        assert_eq!(
            find(&root, "first.tex", &entry),
            Some(nested.join("first.tex"))
        );
        SESSION.with(|session| {
            assert!(session.borrow().as_ref().unwrap().indices.retained_bytes() > 0);
        });
        assert!(find(&root, "second.tex", &entry).is_none());
        fs::write(nested.join("second.tex"), "second").unwrap();
        record_output(&nested.join("second.tex"));
        assert_eq!(
            find(&root, "second.tex", &entry),
            Some(nested.join("second.tex"))
        );
        fs::remove_file(nested.join("first.tex")).unwrap();
        reset();
        SESSION.with(|session| assert!(session.borrow().is_none()));
        assert!(find(&root, "first.tex", &entry).is_none());
        assert_eq!(
            find(&root, "second.tex", &entry),
            Some(nested.join("second.tex"))
        );
        reset();
        fs::remove_dir_all(root).unwrap();
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
    fn physical_alias_targets_share_the_existing_inventory() {
        let root = fixture();
        let external = fixture();
        fs::create_dir(external.join("deep")).unwrap();
        fs::write(external.join("deep/shared.sty"), "shared").unwrap();
        std::os::unix::fs::symlink(&external, root.join("alias")).unwrap();
        let mut session = LookupSession::new();
        assert_eq!(
            session.find(&root, "shared.sty", ".//"),
            Some(root.join("alias/deep/shared.sty"))
        );
        assert_eq!(
            session.find(&external, "shared.sty", ".//"),
            Some(external.join("deep/shared.sty"))
        );
        assert_eq!(session.disk_scans, 1);
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn outputs_in_new_alias_target_directories_rebuild_the_inventory() {
        let root = fixture();
        let external = fixture();
        for alias in ["a", "z"] {
            std::os::unix::fs::symlink(&external, root.join(alias)).unwrap();
        }
        let mut session = LookupSession::new();
        assert!(session.find(&root, "created.tex", ".//").is_none());
        fs::create_dir(external.join("new")).unwrap();
        let output = external.join("new/created.tex");
        fs::write(&output, "generated").unwrap();
        session.record_output(&output);
        assert_eq!(session.indices.retained_bytes(), 0);
        assert_eq!(
            session.find(&root, "created.tex", ".//z//"),
            Some(root.join("z/new/created.tex"))
        );
        assert_eq!(session.disk_scans, 2);
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn cyclic_aliases_keep_lexical_search_priority() {
        let root = fixture();
        let external = fixture();
        let x = external.join("x");
        let y = external.join("y");
        fs::create_dir(&x).unwrap();
        fs::create_dir(&y).unwrap();
        std::os::unix::fs::symlink(&x, root.join("a")).unwrap();
        std::os::unix::fs::symlink(&y, root.join("z")).unwrap();
        std::os::unix::fs::symlink(&y, x.join("b")).unwrap();
        std::os::unix::fs::symlink(&x, y.join("back")).unwrap();
        fs::write(x.join("match.sty"), "match").unwrap();
        let mut session = LookupSession::new();
        assert_eq!(
            session.find(&root, "match.sty", ".//back//"),
            Some(root.join("z/back/match.sty"))
        );
        assert!(session
            .find(&root, "match.sty", ".//back//back//")
            .is_none());
        assert_eq!(session.disk_scans, 1);
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn reused_subtree_views_start_a_fresh_cycle_context() {
        let root = fixture();
        let subtree = root.join("sub");
        fs::create_dir(&subtree).unwrap();
        std::os::unix::fs::symlink(&root, subtree.join("up")).unwrap();
        fs::write(root.join("match.sty"), "match").unwrap();
        let mut session = LookupSession::new();
        assert!(session.find(&root, "absent.sty", ".//").is_none());
        assert_eq!(
            session.find(&subtree, "match.sty", ".//up//"),
            Some(subtree.join("up/match.sty"))
        );
        assert_eq!(session.disk_scans, 1);
        fs::remove_dir_all(root).unwrap();
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

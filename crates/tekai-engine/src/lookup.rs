//! Mutable search trees are indexed once per lookup session, not once per
//! filename or recursive wildcard. A build/pass owns a session. TeX writes are
//! registered as they happen; external edits are observed by the next session.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::marker::{PhantomData, PhantomPinned};
use std::path::{Component, Path, PathBuf};
use std::pin::Pin;
use std::ptr::NonNull;
use std::rc::Rc;
use std::time::SystemTime;

use crate::cache::BudgetCache;
use crate::directory_graph::DirectoryGraph;

const INDEX_BUDGET: usize = 32 * 1024 * 1024;
const INDEX_LIMIT: usize = 128;
const OBSERVED_FILE_BUDGET: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
struct FileStamp {
    modified: Option<SystemTime>,
    len: u64,
    identity: Option<crate::file_identity::ChangeIdentity>,
}

#[derive(Eq, PartialEq)]
struct ObservedInputStamp {
    file: FileStamp,
    parent: FileStamp,
    leaf: FileStamp,
}

struct ObservedFileReceipt {
    // A lexical route is permission to find this physical destination only.
    // Retargeting an alias must not expose the new target's old artifacts.
    physical: PathBuf,
    input_stamp: Option<ObservedInputStamp>,
}

fn observed_input_stamp(path: &Path) -> Option<ObservedInputStamp> {
    let metadata = fs::symlink_metadata(path).ok()?;
    let leaf = metadata_stamp(&metadata);
    leaf.identity?;
    let file = if metadata.is_symlink() {
        file_stamp(path).filter(|stamp| stamp.identity.is_some())?
    } else {
        leaf.clone()
    };
    let parent = file_stamp(path.parent()?).filter(|stamp| stamp.identity.is_some())?;
    Some(ObservedInputStamp { file, parent, leaf })
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
    explicit_policy: Option<Vec<PathBuf>>,
    project_exclusions: Vec<PathBuf>,
    observed_files: HashMap<PathBuf, ObservedFileReceipt>,
    observed_bytes: usize,
    observed_budget: usize,
    #[cfg(test)]
    disk_scans: usize,
    #[cfg(test)]
    database_reads: usize,
    #[cfg(test)]
    input_resolutions: usize,
}

impl LookupSession {
    fn new() -> Self {
        Self::new_with_policy(None)
    }

    fn new_with_policy(explicit_policy: Option<Vec<PathBuf>>) -> Self {
        Self {
            indices: BudgetCache::new(INDEX_BUDGET, INDEX_LIMIT),
            budget: INDEX_BUDGET,
            oversized: Vec::new(),
            exclusion_config: crate::environment::var_os("TEKAI_ENGINE_PROJECT_EXCLUDES"),
            project_exclusions: explicit_policy.clone().unwrap_or_else(project_exclusions),
            explicit_policy,
            observed_files: HashMap::new(),
            observed_bytes: 0,
            observed_budget: OBSERVED_FILE_BUDGET,
            #[cfg(test)]
            disk_scans: 0,
            #[cfg(test)]
            database_reads: 0,
            #[cfg(test)]
            input_resolutions: 0,
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
        let base = crate::working_directory::resolve(base).ok()?;
        if self.exclusion_config != crate::environment::var_os("TEKAI_ENGINE_PROJECT_EXCLUDES") {
            *self = Self::new_with_policy(self.explicit_policy.clone());
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
        let opaque_root = implicit_project
            && self
                .project_exclusions
                .iter()
                .any(|excluded| root.starts_with(excluded));
        // A zero-directory wildcard always searches the root first. This also
        // sees freshly generated files without a recursive scan.
        if let Some(suffix) = suffix.filter(|_| !opaque_root) {
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
        let excluded = crate::runtime::installed_bundle_root()
            .into_iter()
            .collect::<Vec<_>>();
        let generated = if implicit_project {
            &self.project_exclusions[..]
        } else {
            &[]
        };
        #[cfg(test)]
        {
            self.disk_scans += 1;
        }
        if self.oversized.contains(&key) {
            return streaming_find(
                &root,
                &excluded,
                generated,
                &self.observed_files,
                requested,
                &pattern,
            );
        }
        let mut index = FileIndex::new(root.clone());
        if let Some(mut graph) = DirectoryGraph::build_with_generated(
            &root,
            &excluded.iter().map(PathBuf::as_path).collect::<Vec<_>>(),
            &generated.iter().map(PathBuf::as_path).collect::<Vec<_>>(),
            self.budget.saturating_sub(index.bytes),
        ) {
            for (path, receipt) in &self.observed_files {
                if !observed_route_is_current(path, receipt) {
                    continue;
                }
                if let Some(parent) = path.parent() {
                    if parent
                        .ancestors()
                        .any(|ancestor| graph.contains_directory(ancestor))
                        && !graph.add_output_route(path)
                    {
                        return streaming_find(
                            &root,
                            &excluded,
                            generated,
                            &self.observed_files,
                            requested,
                            &pattern,
                        );
                    }
                }
            }
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
        streaming_find(
            &root,
            &excluded,
            generated,
            &self.observed_files,
            requested,
            &pattern,
        )
    }

    fn record_input(&mut self, path: &Path, immutable_bundle: Option<&Path>) -> bool {
        if immutable_bundle.is_some_and(|root| {
            path.strip_prefix(root).is_ok_and(|relative| {
                relative
                    .components()
                    .all(|part| matches!(part, Component::Normal(_)))
            })
        }) {
            // Bundled lookup already uses the immutable archive index. Its
            // normal returned paths cannot alias into generated namespaces.
            return true;
        }
        if let Some(previous) = self
            .observed_files
            .get(path)
            .and_then(|receipt| receipt.input_stamp.as_ref())
        {
            if observed_input_stamp(path).as_ref() == Some(previous) {
                return true;
            }
        }
        #[cfg(test)]
        {
            self.input_resolutions += 1;
        }
        let owned = physical_output_paths(path).is_some_and(|paths| {
            paths.iter().any(|path| {
                self.project_exclusions
                    .iter()
                    .any(|root| path.starts_with(root))
            })
        });
        if !owned {
            return true;
        }
        if !self.record_output(path) {
            return false;
        }
        // There is at most one stamp for each admitted path. Its reserved
        // allocation is included in observed_file_cost, never a separate
        // unbounded cache. Changed identities must resolve aliases again.
        if let Some(receipt) = self.observed_files.get_mut(path) {
            if let Some(stamp) = observed_input_stamp(path) {
                receipt.input_stamp = Some(stamp);
            }
        }
        true
    }

    fn can_record_output(&self, path: &Path) -> bool {
        let Some(paths) = physical_output_paths(path) else {
            return true;
        };
        self.can_record_paths(&paths)
    }

    fn can_record_paths(&self, paths: &[PathBuf]) -> bool {
        let Some(physical) = paths.last() else {
            return true;
        };
        paths
            .iter()
            .try_fold(self.observed_bytes, |bytes, path| {
                let previous = self
                    .observed_files
                    .get(path)
                    .map_or(0, |receipt| observed_file_cost(path, &receipt.physical));
                bytes
                    .checked_sub(previous)?
                    .checked_add(observed_file_cost(path, physical))
            })
            .is_some_and(|bytes| bytes <= self.observed_budget)
    }

    fn record_output(&mut self, path: &Path) -> bool {
        let Some(paths) = physical_output_paths(path) else {
            return true;
        };
        if !self.can_record_paths(&paths) {
            return false;
        }
        let physical = paths.last().unwrap();
        for path in &paths {
            let previous = self
                .observed_files
                .get(path)
                .map_or(0, |receipt| observed_file_cost(path, &receipt.physical));
            self.observed_bytes =
                self.observed_bytes - previous + observed_file_cost(path, physical);
            self.observed_files.insert(
                path.clone(),
                ObservedFileReceipt {
                    physical: physical.clone(),
                    input_stamp: None,
                },
            );
        }
        let keys = self
            .indices
            .keys()
            .filter(|key| matches!(key, IndexKey::Disk(_, _)))
            .cloned()
            .collect::<Vec<_>>();
        for path in paths {
            let Some(parent) = path.parent() else {
                continue;
            };
            for key in &keys {
                if !self.indices.peek(key).is_some_and(|index| {
                    path.starts_with(&index.root)
                        || index.graph.as_ref().is_some_and(|graph| {
                            parent
                                .ancestors()
                                .any(|ancestor| graph.contains_directory(ancestor))
                        })
                }) {
                    continue;
                }
                if let Some(mut index) = self.indices.remove(key) {
                    let Some(graph) = &mut index.graph else {
                        continue;
                    };
                    let previous = graph.retained_bytes();
                    // A newly created directory invalidates the inventory. Known
                    // parents only need one file insertion, shared by every alias.
                    if !graph.add_output_route(&path) {
                        continue;
                    }
                    index.bytes = index.bytes - previous + graph.retained_bytes();
                    let bytes = index.bytes;
                    self.indices.insert(key.clone(), index, bytes);
                }
            }
        }
        true
    }
}

fn observed_file_cost(path: &Path, physical: &Path) -> usize {
    path.as_os_str().len()
        + physical.as_os_str().len()
        + std::mem::size_of::<PathBuf>() * 2
        + std::mem::size_of::<ObservedFileReceipt>()
        + 256
}

fn observed_route_is_current(path: &Path, receipt: &ObservedFileReceipt) -> bool {
    path.canonicalize()
        .is_ok_and(|physical| physical == receipt.physical)
}

fn physical_output_paths(path: &Path) -> Option<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let mut path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        crate::working_directory::current().ok()?.join(path)
    };
    // Resolve the native parent before removing dot-dot components. A parent
    // such as output/link/.. can name a different directory than output when
    // link is a symlink. Retain a lexical route only when it resolves to the
    // same parent, otherwise register the actual physical destination alone.
    let parent = path.parent()?.canonicalize().ok()?;
    let lexical = absolute_lexical(&path)?;
    if lexical
        .parent()?
        .canonicalize()
        .is_ok_and(|lexical_parent| lexical_parent == parent)
    {
        paths.push(lexical);
    }
    for _ in 0..=40 {
        let parent = path.parent()?.canonicalize().ok()?;
        path = parent.join(path.file_name()?);
        if !paths.contains(&path) {
            paths.push(path.clone());
        }
        let Ok(target) = fs::read_link(&path) else {
            return Some(paths);
        };
        path = if target.is_absolute() {
            target
        } else {
            parent.join(target)
        };
        if paths.contains(&path) {
            return None;
        }
    }
    None
}

fn streaming_find(
    root: &Path,
    excluded: &[PathBuf],
    generated: &[PathBuf],
    observed: &HashMap<PathBuf, ObservedFileReceipt>,
    requested: &Path,
    pattern: &DirectoryPattern,
) -> Option<PathBuf> {
    let mut best = None;
    let mut entries = walkdir::WalkDir::new(root)
        .follow_links(true)
        .sort_by_file_name()
        .into_iter();
    while let Some(entry) = entries.next() {
        let Ok(entry) = entry else { continue };
        let physical = if entry.path_is_symlink() {
            entry.path().canonicalize().ok()
        } else {
            Some(entry.path().to_path_buf())
        };
        let under = |roots: &[PathBuf]| {
            roots.iter().any(|path| {
                entry.path().starts_with(path)
                    || physical
                        .as_ref()
                        .is_some_and(|target| target.starts_with(path))
            })
        };
        if under(excluded) {
            entries.skip_current_dir();
            continue;
        }
        if under(generated) {
            entries.skip_current_dir();
            if let Some(physical) = physical {
                for (path, receipt) in observed {
                    if let Ok(relative) = path.strip_prefix(&physical) {
                        let candidate = entry.path().join(relative);
                        if match_rank(&candidate, requested, pattern).is_some()
                            && observed_route_is_current(&candidate, receipt)
                        {
                            consider_match(&candidate, requested, pattern, &mut best);
                        }
                    }
                }
            }
        } else if entry.file_type().is_file() {
            consider_match(entry.path(), requested, pattern, &mut best);
        }
    }
    best.map(|(_, path)| path)
}

fn project_exclusions() -> Vec<PathBuf> {
    crate::environment::var_os("TEKAI_ENGINE_PROJECT_EXCLUDES")
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
    with_session(|session| {
        session.find_policy(base, candidate, &entry.path, entry.source == "project")
    })
}

fn file_stamp(path: &Path) -> Option<FileStamp> {
    let metadata = fs::metadata(path).ok()?;
    Some(metadata_stamp(&metadata))
}

fn metadata_stamp(metadata: &fs::Metadata) -> FileStamp {
    FileStamp {
        modified: metadata.modified().ok(),
        len: metadata.len(),
        identity: crate::file_identity::change_identity(metadata),
    }
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
        crate::working_directory::current().ok()?.join(path)
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
    static ACTIVE: Cell<Option<NonNull<LookupOwner>>> = const { Cell::new(None) };
}

/// Invocation-owned mutable search state. TLS only routes to the pinned owner.
pub(crate) struct LookupOwner {
    session: RefCell<Option<LookupSession>>,
    explicit_policy: Option<Vec<PathBuf>>,
    _thread: PhantomData<Rc<()>>,
    _pinned: PhantomPinned,
}

impl LookupOwner {
    pub(crate) fn new() -> Pin<Box<Self>> {
        Self::with_policy(None)
    }

    fn with_policy(explicit_policy: Option<Vec<PathBuf>>) -> Pin<Box<Self>> {
        Box::pin(Self {
            session: RefCell::new(None),
            explicit_policy,
            _thread: PhantomData,
            _pinned: PhantomPinned,
        })
    }

    pub(crate) fn scope<T>(session: Pin<&mut Self>, action: impl FnOnce() -> T) -> T {
        struct Restore {
            pointer: NonNull<LookupOwner>,
            previous: Option<NonNull<LookupOwner>>,
        }
        impl Drop for Restore {
            fn drop(&mut self) {
                let _ = ACTIVE.try_with(|active| {
                    if active.get() == Some(self.pointer) {
                        active.set(self.previous);
                    }
                });
            }
        }
        // SAFETY The owner stays pinned and exclusively borrowed until the
        // private restoration guard has returned, including during unwinding.
        let pointer = NonNull::from(unsafe { session.get_unchecked_mut() });
        let previous = ACTIVE.with(|active| {
            if let Some(previous) = active.get() {
                // Nested policy closures may restore a previous owner, but
                // cannot route away while one of its operations is borrowing.
                let owner = unsafe { previous.as_ref() };
                assert!(
                    owner.session.try_borrow_mut().is_ok(),
                    "recursive lookup scope"
                );
            }
            active.replace(Some(pointer))
        });
        let _restore = Restore { pointer, previous };
        action()
    }
}

/// Apply an implicit-project policy for a closure without exposing a guard
/// whose safe abandonment could leave a borrowed pointer registered in TLS.
pub fn with_lookup_policy<T>(paths: Vec<PathBuf>, action: impl FnOnce() -> T) -> T {
    with_owned_lookup_policy(Some(paths), action)
}

/// Own caches for a public utility/build closure while preserving its default
/// environment-derived implicit-project policy. No state survives the closure.
pub fn with_lookup_session<T>(action: impl FnOnce() -> T) -> T {
    with_owned_lookup_policy(None, action)
}

fn with_owned_lookup_policy<T>(paths: Option<Vec<PathBuf>>, action: impl FnOnce() -> T) -> T {
    let mut runtime = crate::runtime::RuntimeSession::new();
    let mut index = crate::kpathsea::IndexSession::new();
    let mut owner = LookupOwner::with_policy(paths);
    crate::runtime::RuntimeSession::scope(runtime.as_mut(), || {
        crate::kpathsea::IndexSession::scope(index.as_mut(), || {
            LookupOwner::scope(owner.as_mut(), action)
        })
    })
}

fn with_session<T>(action: impl FnOnce(&mut LookupSession) -> T) -> T {
    ACTIVE.with(|active| {
        let Some(pointer) = active.get() else {
            // Unscoped public utilities retain no policy or environment cache
            // between calls. Compiler/engine closures use their own owner.
            return action(&mut LookupSession::new());
        };
        // SAFETY The private scope retains the owner. RefCell rejects recursive
        // mutable access and no reference to its state escapes this callback.
        let owner = unsafe { pointer.as_ref() };
        let mut session = owner.session.borrow_mut();
        action(
            session.get_or_insert_with(|| {
                LookupSession::new_with_policy(owner.explicit_policy.clone())
            }),
        )
    })
}

/// Begin a build/pass's view of mutable search trees. Call again after external
/// filesystem edits. This drops both successful and unsuccessful old lookups.
pub fn reset() {
    // A cache-hit build never performs a lookup. Drop prior indices without
    // constructing an empty session that would immediately remain unused.
    ACTIVE.with(|active| {
        if let Some(pointer) = active.get() {
            // SAFETY Private scope keeps this owner alive; reset never escapes
            // its short exclusive interior borrow.
            *unsafe { pointer.as_ref() }.session.borrow_mut() = None;
        }
    });
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
    let base = crate::working_directory::resolve(base).ok()?;
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
    with_session(|session| session.find(base, candidate, &entry.to_string_lossy()))
}

pub(crate) fn can_record_output(path: &Path) -> bool {
    with_session(|session| session.can_record_output(path))
}

pub(crate) fn record_input(path: &Path) -> bool {
    let bundle = crate::runtime::installed_bundle_root();
    with_session(|session| session.record_input(path, bundle.as_deref()))
}

pub(crate) fn record_output(path: &Path) -> bool {
    with_session(|session| {
        // Outputs may precede the first lookup. Keep the bounded observation
        // ledger, but leave all disk inventories lazy until actually needed.
        session.record_output(path)
    })
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
    fn nested_policy_unwind_restores_the_outer_cache_and_busy_scopes_are_rejected() {
        use std::panic::{catch_unwind, AssertUnwindSafe};
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
        with_lookup_policy(vec![root.join("a-generated")], || {
            assert_eq!(
                find_search_path(&root, "choice.sty", &entry),
                Some(source.clone())
            );
            let pointer = ACTIVE.with(Cell::get);
            assert!(catch_unwind(AssertUnwindSafe(|| {
                with_lookup_policy(Vec::new(), || {
                    assert_eq!(
                        find_search_path(&root, "choice.sty", &entry),
                        Some(generated.clone())
                    );
                    panic!("partial preflight");
                });
            }))
            .is_err());
            assert_eq!(ACTIVE.with(Cell::get), pointer);
            assert_eq!(
                find_search_path(&root, "choice.sty", &entry),
                Some(source.clone())
            );
            assert!(catch_unwind(AssertUnwindSafe(|| {
                with_session(|_| with_lookup_policy(Vec::new(), || ()));
            }))
            .is_err());
            assert_eq!(ACTIVE.with(Cell::get), pointer);
            assert_eq!(
                find_search_path(&root, "choice.sty", &entry),
                Some(source.clone())
            );
        });
        assert!(ACTIVE.with(Cell::get).is_none());
        assert_eq!(
            find_search_path(&root, "choice.sty", &entry),
            Some(generated)
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn public_default_session_keeps_environment_policy_and_restores_after_explicit_policy() {
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
        let mut environment = crate::environment::EnvironmentSession::new().unwrap();
        crate::environment::EnvironmentSession::scope(environment.as_mut(), || {
            crate::environment::set(
                std::ffi::OsStr::new("TEKAI_ENGINE_PROJECT_EXCLUDES"),
                root.join("a-generated").as_os_str(),
            )
            .unwrap();
            with_lookup_session(|| {
                assert_eq!(
                    find_search_path(&root, "choice.sty", &entry),
                    Some(source.clone())
                );
                with_lookup_policy(Vec::new(), || {
                    assert_eq!(
                        find_search_path(&root, "choice.sty", &entry),
                        Some(generated.clone())
                    );
                });
                assert_eq!(
                    find_search_path(&root, "choice.sty", &entry),
                    Some(source.clone())
                );
                with_session(|session| assert_eq!(session.disk_scans, 1));
            });
            assert!(ACTIVE.with(Cell::get).is_none());
        })
        .unwrap();
        fs::remove_dir_all(root).unwrap();
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
    fn registered_outputs_survive_eviction_and_fallback_but_not_session_reset() {
        let root = fixture();
        let output = root.join("a-output");
        fs::create_dir_all(output.join("generated/deep")).unwrap();
        fs::create_dir_all(root.join("z-source")).unwrap();
        let old = output.join("generated/deep/old.tex");
        let created = output.join("generated/deep/choice.tex");
        let source = root.join("z-source/choice.tex");
        fs::write(&old, "old unobserved artifact").unwrap();
        fs::write(&source, "source").unwrap();
        for budget in [INDEX_BUDGET, 1] {
            let mut session = LookupSession::new();
            session.project_exclusions = vec![output.clone()];
            session.budget = budget;
            assert_eq!(
                session.find_policy(&root, "choice.tex", ".//", true),
                Some(source.clone())
            );
            assert!(session.find_policy(&root, "old.tex", ".//", true).is_none());
            fs::write(&created, "current output").unwrap();
            assert!(session.record_output(&created));
            assert_eq!(
                session.find_policy(&root, "choice.tex", ".//", true),
                Some(created.clone())
            );
            assert_eq!(
                session.find_policy(&root, "choice.tex", "a-output//generated//", true),
                Some(created.clone())
            );
            assert!(session
                .find_policy(&root, "old.tex", "a-output//", true)
                .is_none());
            let keys = session.indices.keys().cloned().collect::<Vec<_>>();
            for key in keys {
                session.indices.remove(&key);
            }
            assert_eq!(
                session.find_policy(&root, "choice.tex", ".//", true),
                Some(created.clone())
            );
            assert!(session
                .find_policy(&root, "choice.tex", "!!a-output//", true)
                .is_none());
            fs::write(root.join("ls-R"), "./a-output/generated/deep:\nold.tex\n").unwrap();
            assert!(session
                .find_policy(&root, "choice.tex", "!!a-output//", true)
                .is_none());
            assert_eq!(
                session.find_policy(&root, "old.tex", "!!a-output//", true),
                Some(old.clone())
            );
            let mut next = LookupSession::new();
            next.project_exclusions = vec![output.clone()];
            assert_eq!(
                next.find_policy(&root, "choice.tex", ".//", true),
                Some(source.clone())
            );
            assert!(next.find_policy(&root, "old.tex", ".//", true).is_none());
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn observed_file_admission_is_bounded_and_deduplicated_without_inventory() {
        let root = fixture();
        let first = root.join("first.tex");
        let refused = root.join("refused.tex");
        fs::write(&first, "first").unwrap();
        let mut session = LookupSession::new();
        let paths = physical_output_paths(&first).unwrap();
        session.observed_budget = paths
            .iter()
            .map(|path| observed_file_cost(path, paths.last().unwrap()))
            .sum();
        assert!(session.can_record_output(&first));
        assert!(session.record_output(&first));
        assert_eq!(session.observed_bytes, session.observed_budget);
        assert!(session.record_output(&first));
        assert_eq!(session.observed_bytes, session.observed_budget);
        assert!(!session.can_record_output(&refused));
        assert!(!session.record_output(&refused));
        assert_eq!(session.observed_files.len(), paths.len());
        assert_eq!(session.disk_scans, 0);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn immutable_bundle_and_unchanged_observed_inputs_avoid_repeated_resolution() {
        let root = fixture();
        let cache = root.join("cache");
        let bundle = cache.join("texmf-id/texmf-dist");
        let output = root.join("output");
        fs::create_dir_all(&bundle).unwrap();
        fs::create_dir_all(&output).unwrap();
        let package = bundle.join("article.cls");
        let input = output.join("input.tex");
        fs::write(&package, "immutable").unwrap();
        fs::write(&input, "observed").unwrap();
        let mut session = LookupSession::new();
        session.project_exclusions = vec![output.clone(), cache];
        for _ in 0..100 {
            assert!(session.record_input(&package, Some(&bundle)));
        }
        assert_eq!(session.input_resolutions, 0);
        assert!(session.observed_files.is_empty());
        assert!(session.record_input(&input, Some(&bundle)));
        let supports_identity = file_stamp(&input).unwrap().identity.is_some();
        for _ in 0..100 {
            assert!(session.record_input(&input, Some(&bundle)));
        }
        assert_eq!(
            session.input_resolutions,
            if supports_identity { 1 } else { 101 }
        );
        assert!(
            session
                .observed_files
                .values()
                .any(|receipt| receipt.input_stamp.is_some())
                || !supports_identity
        );
        assert!(session.record_input(&bundle.join("../../../output/input.tex"), Some(&bundle)));
        assert_eq!(
            session.input_resolutions,
            if supports_identity { 2 } else { 102 }
        );
        assert!(session.observed_bytes <= session.observed_budget);
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn changed_observed_alias_identity_resolves_and_updates_the_new_target() {
        let root = fixture();
        let output = root.join("output");
        fs::create_dir_all(&output).unwrap();
        for target in ["first", "second"] {
            fs::create_dir_all(root.join(target)).unwrap();
            fs::write(root.join(target).join("input.tex"), target).unwrap();
        }
        let alias = output.join("alias");
        std::os::unix::fs::symlink(root.join("first"), &alias).unwrap();
        let input = alias.join("input.tex");
        let mut session = LookupSession::new();
        session.project_exclusions = vec![output.clone()];
        assert!(session
            .find_policy(&root, "input.tex", "output//", true)
            .is_none());
        assert!(session.record_input(&input, None));
        assert_eq!(session.input_resolutions, 1);
        assert_eq!(
            session.find_policy(&root, "input.tex", "output//", true),
            Some(input.clone())
        );
        assert!(session.record_input(&input, None));
        assert_eq!(session.input_resolutions, 1);
        fs::remove_file(&alias).unwrap();
        std::os::unix::fs::symlink(root.join("second"), &alias).unwrap();
        assert!(session.record_input(&input, None));
        assert_eq!(session.input_resolutions, 2);
        assert!(session
            .observed_files
            .contains_key(&root.join("second/input.tex")));
        assert_eq!(
            session.find_policy(&root, "input.tex", "output//", true),
            Some(input)
        );
        let graph = session
            .indices
            .keys()
            .find_map(|key| {
                session
                    .indices
                    .peek(key)
                    .and_then(|index| index.graph.as_ref())
            })
            .unwrap();
        let output_node = graph.find(Some("alias"), Path::new("input.tex"));
        assert_eq!(output_node, Some(alias.join("input.tex")));
        fs::remove_dir_all(root).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn hardlinked_input_identity_does_not_hide_an_observed_parent_alias_retarget() {
        let root = fixture();
        let external = fixture();
        let output = root.join("output");
        fs::create_dir_all(&output).unwrap();
        for directory in ["first", "second"] {
            fs::create_dir_all(external.join(directory)).unwrap();
            fs::write(external.join(directory).join("sibling.tex"), directory).unwrap();
        }
        fs::write(external.join("first/input.tex"), "same inode").unwrap();
        fs::hard_link(
            external.join("first/input.tex"),
            external.join("second/input.tex"),
        )
        .unwrap();
        let alias = output.join("alias");
        std::os::unix::fs::symlink(external.join("first"), &alias).unwrap();
        let mut session = LookupSession::new();
        session.project_exclusions = vec![output];
        assert!(session
            .find_policy(&root, "input.tex", "output//", true)
            .is_none());
        assert!(session.record_input(&alias.join("input.tex"), None));
        assert!(session.record_input(&alias.join("sibling.tex"), None));
        let other_alias = root.join("output/other-alias");
        std::os::unix::fs::symlink(external.join("second"), &other_alias).unwrap();
        assert!(session.record_input(&other_alias.join("input.tex"), None));
        assert_eq!(
            session.find_policy(&root, "sibling.tex", "output//", true),
            Some(alias.join("sibling.tex"))
        );
        let first = file_stamp(&alias.join("input.tex"));
        fs::remove_file(&alias).unwrap();
        std::os::unix::fs::symlink(external.join("second"), &alias).unwrap();
        assert_eq!(first, file_stamp(&alias.join("input.tex")));
        assert!(session.record_input(&alias.join("input.tex"), None));
        assert_eq!(session.input_resolutions, 4);
        assert_eq!(
            session.find_policy(&root, "input.tex", "output//", true),
            Some(alias.join("input.tex"))
        );
        assert!(session
            .find_policy(&root, "sibling.tex", "output//", true)
            .is_none());
        session.indices = BudgetCache::new(INDEX_BUDGET, INDEX_LIMIT);
        assert!(session
            .find_policy(&root, "sibling.tex", "output//", true)
            .is_none());
        session.budget = 1;
        session.indices = BudgetCache::new(1, INDEX_LIMIT);
        assert!(session
            .find_policy(&root, "sibling.tex", "output//", true)
            .is_none());
        let leaf_alias = root.join("output/leaf.tex");
        std::os::unix::fs::symlink(external.join("first/input.tex"), &leaf_alias).unwrap();
        assert!(session.record_input(&leaf_alias, None));
        let resolutions = session.input_resolutions;
        fs::remove_file(&leaf_alias).unwrap();
        std::os::unix::fs::symlink(external.join("second/input.tex"), &leaf_alias).unwrap();
        assert!(session.record_input(&leaf_alias, None));
        assert_eq!(session.input_resolutions, resolutions + 1);
        assert_eq!(
            session.observed_files[&leaf_alias].physical,
            external.join("second/input.tex")
        );
        assert_eq!(
            session.find_policy(&root, "leaf.tex", "output//", true),
            Some(leaf_alias)
        );
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn observed_outputs_preserve_known_aliases_without_admitting_old_siblings() {
        let root = fixture();
        let output = root.join("m-output");
        let external = fixture();
        fs::create_dir_all(output.join("deep")).unwrap();
        fs::create_dir_all(external.join("deep")).unwrap();
        std::os::unix::fs::symlink(&output, root.join("a-output-alias")).unwrap();
        std::os::unix::fs::symlink(&external, output.join("link")).unwrap();
        fs::write(external.join("deep/old.tex"), "old").unwrap();
        let created = output.join("deep/created.tex");
        let alias_created = output.join("link/deep/aliased.tex");
        for budget in [INDEX_BUDGET, 1] {
            let mut session = LookupSession::new();
            session.project_exclusions = vec![output.clone()];
            session.budget = budget;
            assert!(session
                .find_policy(&root, "created.tex", ".//", true)
                .is_none());
            fs::write(&created, "current").unwrap();
            assert!(session.record_output(&created));
            fs::write(&alias_created, "alias current").unwrap();
            assert!(session.record_output(&alias_created));
            assert_eq!(
                session.find_policy(&root, "created.tex", ".//", true),
                Some(root.join("a-output-alias/deep/created.tex"))
            );
            assert_eq!(
                session.find_policy(&root, "aliased.tex", ".//", true),
                Some(root.join("a-output-alias/link/deep/aliased.tex"))
            );
            assert_eq!(
                session.find_policy(&root, "aliased.tex", "m-output//link//", true),
                Some(alias_created.clone())
            );
            assert!(session.find_policy(&root, "old.tex", ".//", true).is_none());
        }
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn observed_output_updates_normal_and_opaque_views_of_the_same_alias_target() {
        let root = fixture();
        let output = root.join("output");
        let external = fixture();
        fs::create_dir_all(output.join("deep")).unwrap();
        fs::create_dir_all(external.join("deep")).unwrap();
        std::os::unix::fs::symlink(&external, output.join("link")).unwrap();
        std::os::unix::fs::symlink(&external, root.join("z-source-alias")).unwrap();
        fs::write(external.join("deep/old.tex"), "old source").unwrap();
        let route = output.join("link/deep/new.tex");
        let mut session = LookupSession::new();
        session.project_exclusions = vec![output.clone()];
        assert!(session.find_policy(&root, "new.tex", ".//", true).is_none());
        fs::write(&route, "current output").unwrap();
        assert!(session.record_output(&route));
        assert_eq!(
            session.find_policy(&root, "new.tex", ".//", true),
            Some(route.clone())
        );
        assert_eq!(
            session.find_policy(&root, "new.tex", "z-source-alias//", true),
            Some(external.join("deep/new.tex"))
        );
        assert_eq!(
            session.find_policy(&external.join("deep"), "new.tex", ".//", true),
            Some(external.join("deep/new.tex"))
        );
        assert!(session
            .find_policy(&root, "old.tex", "output//", true)
            .is_none());
        assert_eq!(
            session.find_policy(&root, "old.tex", "z-source-alias//", true),
            Some(external.join("deep/old.tex"))
        );
        assert_eq!(session.disk_scans, 1);
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn output_parent_resolution_preserves_symlink_dot_dot_semantics() {
        let root = fixture();
        let output = root.join("output");
        let external = fixture();
        fs::create_dir_all(&output).unwrap();
        fs::create_dir_all(external.join("deep")).unwrap();
        std::os::unix::fs::symlink(external.join("deep"), output.join("link")).unwrap();
        std::os::unix::fs::symlink(&external, root.join("source-alias")).unwrap();
        let route = output.join("link/../created.tex");
        let mut session = LookupSession::new();
        session.project_exclusions = vec![output.clone()];
        assert!(session
            .find_policy(&root, "created.tex", ".//", true)
            .is_none());
        fs::write(&route, "native destination").unwrap();
        assert!(session.record_output(&route));
        assert_eq!(
            physical_output_paths(&route),
            Some(vec![external.join("created.tex")])
        );
        assert_eq!(
            session.find_policy(&root, "created.tex", ".//", true),
            Some(root.join("source-alias/created.tex"))
        );
        assert!(session
            .find_policy(&root, "created.tex", "output//", true)
            .is_none());
        assert_eq!(session.disk_scans, 1);
        fs::remove_dir_all(root).unwrap();
        fs::remove_dir_all(external).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn streaming_search_continues_after_earlier_dangling_alias_errors() {
        let root = fixture();
        std::os::unix::fs::symlink(root.join("missing"), root.join("a-broken")).unwrap();
        fs::create_dir(root.join("z-source")).unwrap();
        let source = root.join("z-source/choice.sty");
        fs::write(&source, "source").unwrap();
        let mut session = LookupSession::new();
        session.budget = 1;
        assert_eq!(session.find(&root, "choice.sty", ".//"), Some(source));
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
        with_lookup_policy(vec![root.join("a-generated")], || {
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
        });
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
        let mut owner = LookupOwner::new();
        LookupOwner::scope(owner.as_mut(), || {
            let root = fixture();
            let entry = PathBuf::from(format!("{}//", root.display()));
            let nested = root.join("nested");
            fs::create_dir(&nested).unwrap();
            reset();
            ACTIVE.with(|active| {
                assert!(unsafe { active.get().unwrap().as_ref() }
                    .session
                    .borrow()
                    .is_none())
            });
            fs::write(nested.join("first.tex"), "first").unwrap();
            record_output(&nested.join("first.tex"));
            ACTIVE.with(|active| {
                let session = &unsafe { active.get().unwrap().as_ref() }.session;
                assert_eq!(
                    session.borrow().as_ref().unwrap().indices.retained_bytes(),
                    0
                );
            });
            assert_eq!(
                find(&root, "first.tex", &entry),
                Some(nested.join("first.tex"))
            );
            ACTIVE.with(|active| {
                let session = &unsafe { active.get().unwrap().as_ref() }.session;
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
            ACTIVE.with(|active| {
                assert!(unsafe { active.get().unwrap().as_ref() }
                    .session
                    .borrow()
                    .is_none())
            });
            assert!(find(&root, "first.tex", &entry).is_none());
            assert_eq!(
                find(&root, "second.tex", &entry),
                Some(nested.join("second.tex"))
            );
            reset();
            fs::remove_dir_all(root).unwrap();
        });
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

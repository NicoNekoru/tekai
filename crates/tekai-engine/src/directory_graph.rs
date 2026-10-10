//! A physical directory inventory with named edges for every directory alias.
//!
//! DAG lookup memoizes node/pattern misses. Cyclic components add their local
//! physical ancestor set to the key, preserving the flattened walk's rule that
//! a directory cannot occur twice along one spelling. Both inventory and
//! matching use explicit stacks. Supported Unix routes include their consumed
//! symlink hops in memo keys. Only a terminal candidate's spelling is built.
//! Path-length failures still need conservative uncached evaluation. Cyclic
//! matching can be exponential, and oversized inventories are left to callers.

use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Component, Path, PathBuf};

// Cyclic components can have exponentially many distinct ancestor sets. Keep
// their memo table bounded. Exact search still terminates after the table fills,
// but an adversarial cyclic component can require exponential work.
const MATCH_MEMO_LIMIT: usize = 32 * 1024 * 1024;

struct Edge {
    name: OsString,
    target: usize,
    hops: Option<usize>,
}

struct Directory {
    canonical: PathBuf,
    edges: Vec<Edge>,
    files: HashMap<OsString, Option<usize>>,
    component: usize,
}

pub(crate) struct DirectoryGraph {
    root: PathBuf,
    root_hops: Option<usize>,
    nodes: Vec<Directory>,
    by_path: HashMap<PathBuf, usize>,
    basenames: HashSet<OsString>,
    cyclic_components: Vec<bool>,
    bytes: usize,
    budget: usize,
    #[cfg(test)]
    read_dirs: usize,
    #[cfg(test)]
    entry_metadata_followups: usize,
    #[cfg(test)]
    directory_canonicalizations: usize,
    #[cfg(test)]
    ordinary_directory_canonicalizations: usize,
}

enum Part {
    Recursive,
    Literal(OsString),
}

#[derive(Eq, Hash, PartialEq)]
struct MissKey {
    node: usize,
    part: usize,
    hops: Option<usize>,
    ancestors: Vec<usize>,
}

struct Frame {
    node: usize,
    part: usize,
    next: usize,
    hops: Option<usize>,
    // Epsilon transitions do not enter a directory or change ancestry.
    incoming: Option<(usize, usize)>,
    // An unusable lexical spelling depends on the prefix, so its miss cannot
    // be reused for another route to the same physical directory.
    cacheable: bool,
}

impl DirectoryGraph {
    /// Inventory each canonical directory once. Stop admitting entries as soon
    /// as the byte estimate exceeds the retention budget.
    pub(crate) fn build(
        root: &Path,
        excluded_bundle: Option<&Path>,
        budget: usize,
    ) -> Option<Self> {
        let canonical = root.canonicalize().ok()?;
        let excluded =
            excluded_bundle.map(|path| path.canonicalize().unwrap_or_else(|_| path.to_path_buf()));
        if excluded.as_ref() == Some(&canonical) {
            return None;
        }
        let mut graph = Self {
            root: root.to_path_buf(),
            root_hops: root_hops(root),
            nodes: Vec::new(),
            by_path: HashMap::new(),
            basenames: HashSet::new(),
            cyclic_components: Vec::new(),
            bytes: root.as_os_str().len() + std::mem::size_of::<Self>() + 256,
            budget,
            #[cfg(test)]
            read_dirs: 0,
            #[cfg(test)]
            entry_metadata_followups: 0,
            #[cfg(test)]
            directory_canonicalizations: 1 + usize::from(excluded_bundle.is_some()),
            #[cfg(test)]
            ordinary_directory_canonicalizations: 0,
        };
        graph.add_directory(canonical)?;
        let mut next = 0;
        while next < graph.nodes.len() {
            #[cfg(test)]
            {
                graph.read_dirs += 1;
            }
            if let Ok(entries) = fs::read_dir(&graph.nodes[next].canonical) {
                for entry in entries.filter_map(Result::ok) {
                    let name = entry.file_name();
                    let Ok(file_type) = entry.file_type() else {
                        continue;
                    };
                    // Directory-entry type normally comes from read_dir. Only
                    // symlinks need a separate metadata call to classify the
                    // target, and regular files need no path allocation here.
                    let symlink = file_type.is_symlink();
                    let hops = if symlink {
                        resolve_hops(&graph.nodes[next].canonical, Path::new(&name))
                    } else {
                        hop_limit().map(|_| 0)
                    };
                    let file_type = if symlink {
                        #[cfg(test)]
                        {
                            graph.entry_metadata_followups += 1;
                        }
                        let Ok(metadata) = fs::metadata(entry.path()) else {
                            continue;
                        };
                        metadata.file_type()
                    } else {
                        file_type
                    };
                    if file_type.is_dir() {
                        let path = entry.path();
                        let target = if symlink || !cfg!(unix) {
                            #[cfg(test)]
                            {
                                graph.directory_canonicalizations += 1;
                                graph.ordinary_directory_canonicalizations += usize::from(!symlink);
                            }
                            let Ok(target) = path.canonicalize() else {
                                continue;
                            };
                            target
                        } else {
                            // Unix read_dir returns the actual child spelling.
                            // Its canonical parent has no symlink components.
                            // Keep canonicalization on other platforms, where
                            // reparse points and namespace normalization differ.
                            path
                        };
                        if excluded.as_ref() == Some(&target) {
                            continue;
                        }
                        let target = if let Some(&target) = graph.by_path.get(&target) {
                            target
                        } else {
                            graph.add_directory(target)?
                        };
                        graph.charge(name.capacity() + std::mem::size_of::<Edge>() + 64)?;
                        graph.nodes[next].edges.push(Edge { name, target, hops });
                    } else if file_type.is_file() && !graph.add_file(next, &name, hops) {
                        return None;
                    }
                }
            }
            graph.nodes[next]
                .edges
                .sort_unstable_by(|a, b| a.name.cmp(&b.name));
            next += 1;
        }
        graph.compute_components();
        graph.charge(graph.cyclic_components.len() + 64)?;
        Some(graph)
    }

    pub(crate) fn retained_bytes(&self) -> usize {
        self.bytes
    }

    pub(crate) fn contains_directory(&self, canonical: &Path) -> bool {
        self.by_path.contains_key(canonical)
    }

    pub(crate) fn find(&self, suffix: Option<&str>, requested: &Path) -> Option<PathBuf> {
        self.match_from(0, &self.root, suffix, requested).0
    }

    /// Reuse an inventoried physical subtree, including an external alias
    /// target. Its returned spelling starts at the caller's canonical root.
    pub(crate) fn find_from(
        &self,
        canonical_root: &Path,
        suffix: Option<&str>,
        requested: &Path,
    ) -> Option<PathBuf> {
        let &node = self.by_path.get(canonical_root)?;
        self.match_from(node, canonical_root, suffix, requested).0
    }

    /// File membership belongs to a physical node, so one insertion updates
    /// every alias route. The caller must evict on false and rebuild on demand.
    pub(crate) fn add_output(&mut self, canonical_parent: &Path, name: &OsStr) -> bool {
        let Some(&node) = self.by_path.get(canonical_parent) else {
            return false;
        };
        let hops = resolve_hops(canonical_parent, Path::new(name));
        self.add_file(node, name, hops)
    }

    fn charge(&mut self, bytes: usize) -> Option<()> {
        self.bytes = self.bytes.checked_add(bytes)?;
        (self.bytes <= self.budget).then_some(())
    }

    fn add_directory(&mut self, canonical: PathBuf) -> Option<usize> {
        // Include both owned canonical paths, map buckets and node allocation.
        self.charge(
            canonical.capacity() * 2
                + std::mem::size_of::<Directory>()
                + std::mem::size_of::<PathBuf>()
                + 256,
        )?;
        let node = self.nodes.len();
        self.by_path.insert(canonical.clone(), node);
        self.nodes.push(Directory {
            canonical,
            edges: Vec::new(),
            files: HashMap::new(),
            component: 0,
        });
        Some(node)
    }

    fn add_file(&mut self, node: usize, name: &OsStr, hops: Option<usize>) -> bool {
        if self.nodes[node].files.contains_key(name) {
            self.nodes[node].files.insert(name.to_os_string(), hops);
            return true;
        }
        let mut cost = name.len() + std::mem::size_of::<(OsString, Option<usize>)>() + 96;
        let new_basename = !self.basenames.contains(name);
        if new_basename {
            cost += name.len() + std::mem::size_of::<OsString>() + 96;
        }
        if self
            .bytes
            .checked_add(cost)
            .is_none_or(|bytes| bytes > self.budget)
        {
            return false;
        }
        self.bytes += cost;
        self.nodes[node].files.insert(name.to_os_string(), hops);
        if new_basename {
            self.basenames.insert(name.to_os_string());
        }
        true
    }

    // Iterative Kosaraju. The reverse edges and DFS stacks are temporary and
    // contain O(nodes + edges) directory IDs, without alias path spellings.
    fn compute_components(&mut self) {
        let mut seen = vec![false; self.nodes.len()];
        let mut order = Vec::with_capacity(self.nodes.len());
        let mut reverse = vec![Vec::new(); self.nodes.len()];
        for (node, directory) in self.nodes.iter().enumerate() {
            for edge in &directory.edges {
                reverse[edge.target].push(node);
            }
        }
        for start in 0..self.nodes.len() {
            if seen[start] {
                continue;
            }
            seen[start] = true;
            let mut stack = vec![(start, 0)];
            while let Some((node, edge)) = stack.last_mut() {
                if *edge < self.nodes[*node].edges.len() {
                    let target = self.nodes[*node].edges[*edge].target;
                    *edge += 1;
                    if !seen[target] {
                        seen[target] = true;
                        stack.push((target, 0));
                    }
                } else {
                    order.push(*node);
                    stack.pop();
                }
            }
        }
        seen.fill(false);
        for start in order.into_iter().rev() {
            if seen[start] {
                continue;
            }
            let component = self.cyclic_components.len();
            let mut members = 0;
            let mut self_loop = false;
            let mut stack = vec![start];
            seen[start] = true;
            while let Some(node) = stack.pop() {
                members += 1;
                self.nodes[node].component = component;
                self_loop |= self.nodes[node]
                    .edges
                    .iter()
                    .any(|edge| edge.target == node);
                for &target in &reverse[node] {
                    if !seen[target] {
                        seen[target] = true;
                        stack.push(target);
                    }
                }
            }
            self.cyclic_components.push(members > 1 || self_loop);
        }
    }

    fn miss_key(
        &self,
        node: usize,
        part: usize,
        hops: Option<usize>,
        ancestors: &[Vec<usize>],
    ) -> MissKey {
        let component = self.nodes[node].component;
        let mut local = if self.cyclic_components[component] {
            ancestors[component].clone()
        } else {
            Vec::new()
        };
        local.sort_unstable();
        MissKey {
            node,
            part,
            hops,
            ancestors: local,
        }
    }

    fn match_from(
        &self,
        start: usize,
        lexical_root: &Path,
        suffix: Option<&str>,
        requested: &Path,
    ) -> (Option<PathBuf>, usize) {
        let Some(name) = requested.file_name() else {
            return (None, 0);
        };
        // Preserve indexed O(1) misses for basenames absent from the entire tree.
        if !self.basenames.contains(name) {
            return (None, 0);
        }
        let mut parts = Vec::new();
        if let Some(suffix) = suffix {
            parts.push(Part::Recursive);
            for (i, chunk) in suffix.split("//").enumerate() {
                if i != 0 {
                    parts.push(Part::Recursive);
                }
                parts.extend(
                    Path::new(chunk)
                        .components()
                        .filter(|component| !matches!(component, Component::CurDir))
                        .map(|component| Part::Literal(component.as_os_str().to_os_string())),
                );
            }
        }
        let requested_parts = requested.components().collect::<Vec<_>>();
        parts.extend(
            requested_parts[..requested_parts.len() - 1]
                .iter()
                .filter(|component| !matches!(component, Component::CurDir))
                .map(|component| Part::Literal(component.as_os_str().to_os_string())),
        );

        let memo_budget = self
            .bytes
            .saturating_mul(parts.len().saturating_add(1))
            .min(MATCH_MEMO_LIMIT);
        let mut memo_bytes = 0usize;
        let mut misses = HashSet::new();
        let mut active = vec![false; self.nodes.len()];
        let mut ancestors = vec![Vec::new(); self.cyclic_components.len()];
        active[start] = true;
        ancestors[self.nodes[start].component].push(start);
        let mut stack = vec![Frame {
            node: start,
            part: 0,
            next: 0,
            hops: if lexical_root == self.root {
                self.root_hops
            } else {
                hop_limit().map(|_| 0)
            },
            incoming: None,
            cacheable: true,
        }];
        let mut visited = 1;

        while let Some(frame) = stack.last_mut() {
            if frame.part == parts.len() {
                if self.nodes[frame.node]
                    .files
                    .get(name)
                    .is_some_and(|&file_hops| !over_hop_limit(add_hops(frame.hops, file_hops)))
                    && self.nodes[frame.node].canonical.join(name).is_file()
                {
                    let mut path = lexical_root.to_path_buf();
                    for frame in &stack {
                        if let Some((from, edge)) = frame.incoming {
                            path.push(&self.nodes[from].edges[edge].name);
                        }
                    }
                    path.push(name);
                    if path.is_file() {
                        return (Some(path), visited);
                    }
                    // A long alias route can exceed the OS symlink-resolution
                    // limit even when its physical terminal remains readable.
                    stack
                        .last_mut()
                        .expect("the terminal frame exists")
                        .cacheable = false;
                }
            } else {
                let next = match &parts[frame.part] {
                    Part::Recursive if frame.next == 0 => {
                        frame.next += 1;
                        Some((frame.node, frame.part + 1, None))
                    }
                    Part::Recursive => {
                        let edge = frame.next - 1;
                        frame.next += 1;
                        self.nodes[frame.node]
                            .edges
                            .get(edge)
                            .map(|entry| (entry.target, frame.part, Some((frame.node, edge))))
                    }
                    Part::Literal(name) if frame.next == 0 => {
                        frame.next = 1;
                        self.nodes[frame.node]
                            .edges
                            .binary_search_by(|edge| edge.name.as_os_str().cmp(name))
                            .ok()
                            .map(|edge| {
                                (
                                    self.nodes[frame.node].edges[edge].target,
                                    frame.part + 1,
                                    Some((frame.node, edge)),
                                )
                            })
                    }
                    Part::Literal(_) => None,
                };
                if let Some((node, part, incoming)) = next {
                    let hops = incoming.map_or(frame.hops, |(from, edge)| {
                        add_hops(frame.hops, self.nodes[from].edges[edge].hops)
                    });
                    if incoming.is_some() {
                        if active[node] || over_hop_limit(hops) {
                            continue;
                        }
                        active[node] = true;
                        ancestors[self.nodes[node].component].push(node);
                    }
                    let key = self.miss_key(node, part, hops, &ancestors);
                    if misses.contains(&key) {
                        if incoming.is_some() {
                            active[node] = false;
                            ancestors[self.nodes[node].component].pop();
                        }
                        continue;
                    }
                    stack.push(Frame {
                        node,
                        part,
                        next: 0,
                        hops,
                        incoming,
                        cacheable: true,
                    });
                    visited += 1;
                    continue;
                }
            }
            let frame = stack.pop().expect("the evaluation frame exists");
            if frame.cacheable {
                let key = self.miss_key(frame.node, frame.part, frame.hops, &ancestors);
                let cost = std::mem::size_of::<MissKey>()
                    + key.ancestors.capacity() * std::mem::size_of::<usize>()
                    + 128;
                if memo_bytes
                    .checked_add(cost)
                    .is_some_and(|bytes| bytes <= memo_budget)
                    && misses.insert(key)
                {
                    memo_bytes += cost;
                }
            } else if let Some(parent) = stack.last_mut() {
                parent.cacheable = false;
            }
            if frame.incoming.is_some() {
                active[frame.node] = false;
                ancestors[self.nodes[frame.node].component].pop();
            }
        }
        (None, visited)
    }
}

// Darwin's MAXSYMLINKS is 32 in sys/param.h. Linux's include/linux/namei.h
// defines MAXSYMLINKS as 40. Unsupported platforms never prune using this model.
fn hop_limit() -> Option<usize> {
    if cfg!(target_os = "macos") {
        Some(32)
    } else if cfg!(target_os = "linux") {
        Some(40)
    } else {
        None
    }
}

fn add_hops(left: Option<usize>, right: Option<usize>) -> Option<usize> {
    left?.checked_add(right?)
}

fn over_hop_limit(hops: Option<usize>) -> bool {
    hops.zip(hop_limit())
        .is_some_and(|(hops, limit)| hops > limit)
}

fn root_hops(root: &Path) -> Option<usize> {
    hop_limit()?;
    let absolute = if root.is_absolute() {
        root.to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(root)
    };
    resolve_hops(Path::new("/"), &absolute)
}

enum ResolvePart {
    Root,
    Parent,
    Normal(OsString),
}

fn prepend_parts(queue: &mut VecDeque<ResolvePart>, path: &Path) -> Option<()> {
    for component in path.components().rev() {
        queue.push_front(match component {
            Component::RootDir => ResolvePart::Root,
            Component::ParentDir => ResolvePart::Parent,
            Component::Normal(name) => ResolvePart::Normal(name.to_os_string()),
            Component::CurDir => continue,
            Component::Prefix(_) => return None,
        });
    }
    Some(())
}

/// Resolve one edge from a canonical parent, including every symlink hidden
/// inside its raw target. No host recursion or unbounded symbolic-link loop.
/// An unmeasurable edge gets an unknown weight, preserving conservative lookup.
fn resolve_hops(parent: &Path, tail: &Path) -> Option<usize> {
    let limit = hop_limit()?;
    let mut resolved = parent.to_path_buf();
    let mut pending = VecDeque::new();
    prepend_parts(&mut pending, tail)?;
    let mut hops = 0;
    let mut steps = 0;
    while let Some(part) = pending.pop_front() {
        // Bound work even if external edits race canonicalization. Normal Unix
        // target strings are also bounded by the OS pathname/readlink limits.
        steps += 1;
        if steps > 64 * 1024 {
            return None;
        }
        match part {
            ResolvePart::Root => resolved = PathBuf::from("/"),
            ResolvePart::Parent => {
                resolved.pop();
            }
            ResolvePart::Normal(name) => {
                let path = resolved.join(&name);
                let metadata = fs::symlink_metadata(&path).ok()?;
                if metadata.file_type().is_symlink() {
                    hops += 1;
                    if hops > limit {
                        return None;
                    }
                    prepend_parts(&mut pending, &fs::read_link(path).ok()?)?;
                } else {
                    resolved.push(name);
                }
            }
        }
    }
    Some(hops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    const BUDGET: usize = 1024 * 1024;

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let root = std::env::temp_dir().join(format!(
                "tekai-directory-graph-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&root).unwrap();
            Self(root.canonicalize().unwrap())
        }

        fn directory(&self, tail: &str) -> PathBuf {
            let path = self.0.join(tail);
            fs::create_dir_all(&path).unwrap();
            path
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    const LABELS: [&str; 3] = ["a", "b", "c"];
    const FILENAMES: [&str; 3] = ["f.sty", "g.tex", "created.tex"];

    #[derive(Clone, Debug, Default)]
    struct ReferenceDirectory {
        edges: [Option<usize>; 3],
        files: u8,
    }

    #[derive(Clone, Debug)]
    struct Query {
        // Each chunk is literal. The gaps between chunks, and before the first
        // chunk, can consume any number of directory components.
        suffix: Option<Vec<Vec<u8>>>,
        qualified: Vec<u8>,
        file: u8,
    }

    fn queries() -> impl Strategy<Value = Vec<Query>> {
        prop::collection::vec(
            (
                prop::option::of(prop::collection::vec(
                    prop::collection::vec(0..3u8, 0..3),
                    1..4,
                )),
                prop::collection::vec(0..3u8, 0..3),
                0..3u8,
            )
                .prop_map(|(suffix, qualified, file)| Query {
                    suffix,
                    qualified,
                    file,
                }),
            1..9,
        )
    }

    fn with_root_queries(mut queries: Vec<Query>) -> Vec<Query> {
        for file in 0..3 {
            for suffix in [None, Some(vec![Vec::new()])] {
                queries.push(Query {
                    suffix,
                    qualified: Vec::new(),
                    file,
                });
            }
        }
        queries
    }

    fn lexical_path(root: &Path, labels: &[u8]) -> PathBuf {
        labels.iter().fold(root.to_path_buf(), |path, &label| {
            path.join(LABELS[usize::from(label)])
        })
    }

    impl Query {
        fn suffix_string(&self) -> Option<String> {
            self.suffix.as_ref().map(|chunks| {
                chunks
                    .iter()
                    .map(|chunk| {
                        chunk
                            .iter()
                            .map(|&label| LABELS[usize::from(label)])
                            .collect::<Vec<_>>()
                            .join("/")
                    })
                    .collect::<Vec<_>>()
                    .join("//")
            })
        }

        fn requested(&self) -> PathBuf {
            lexical_path(Path::new(""), &self.qualified).join(FILENAMES[usize::from(self.file)])
        }
    }

    // Flatten every simple physical route first, without memoization or the
    // production matcher. Small graphs make exhaustive reference search cheap.
    fn reference_routes(
        nodes: &[ReferenceDirectory],
        start: usize,
        excluded: Option<usize>,
    ) -> Vec<(usize, Vec<u8>)> {
        fn visit(
            nodes: &[ReferenceDirectory],
            node: usize,
            excluded: Option<usize>,
            ancestors: &mut Vec<usize>,
            route: &mut Vec<u8>,
            result: &mut Vec<(usize, Vec<u8>)>,
        ) {
            if Some(node) == excluded || ancestors.contains(&node) {
                return;
            }
            result.push((node, route.clone()));
            ancestors.push(node);
            for (label, target) in nodes[node].edges.iter().enumerate() {
                if let Some(target) = target {
                    route.push(label as u8);
                    visit(nodes, *target, excluded, ancestors, route, result);
                    route.pop();
                }
            }
            ancestors.pop();
        }
        let mut result = Vec::new();
        visit(
            nodes,
            start,
            excluded,
            &mut Vec::new(),
            &mut Vec::new(),
            &mut result,
        );
        result
    }

    // Enumerate placements of literal chunks in one flattened route. Sorting
    // the wildcard endpoint prefixes reproduces outer-first nested searches.
    // This is deliberately exhaustive, unlike the production graph traversal.
    fn reference_rank(
        chunks: &[Vec<u8>],
        route: &[u8],
        position: usize,
        end: usize,
    ) -> Option<Vec<Vec<u8>>> {
        if chunks.is_empty() {
            return (position == end).then(Vec::new);
        }
        let mut best = None;
        for gap_end in position..=end {
            let literal_end = gap_end + chunks[0].len();
            if literal_end > end || route[gap_end..literal_end] != chunks[0] {
                continue;
            }
            if let Some(tail) = reference_rank(&chunks[1..], route, literal_end, end) {
                let mut rank = vec![route[..gap_end].to_vec()];
                rank.extend(tail);
                if best.as_ref().is_none_or(|previous| &rank < previous) {
                    best = Some(rank);
                }
            }
        }
        best
    }

    fn reference_find(
        nodes: &[ReferenceDirectory],
        start: usize,
        excluded: Option<usize>,
        query: &Query,
    ) -> Option<Vec<u8>> {
        reference_routes(nodes, start, excluded)
            .into_iter()
            .filter_map(|(node, route)| {
                if nodes[node].files & (1 << query.file) == 0 || !route.ends_with(&query.qualified)
                {
                    return None;
                }
                let prefix_end = route.len() - query.qualified.len();
                let mut rank = match &query.suffix {
                    None => (prefix_end == 0).then(Vec::new)?,
                    Some(chunks) => reference_rank(chunks, &route, 0, prefix_end)?,
                };
                rank.push(route.clone());
                Some((rank, route))
            })
            .min_by(|left, right| left.0.cmp(&right.0))
            .map(|(_, route)| route)
    }

    fn expected_path(
        nodes: &[ReferenceDirectory],
        start: usize,
        excluded: Option<usize>,
        root: &Path,
        query: &Query,
    ) -> Option<PathBuf> {
        reference_find(nodes, start, excluded, query)
            .map(|route| lexical_path(root, &route).join(FILENAMES[usize::from(query.file)]))
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 64,
            max_shrink_iters: 4096,
            ..ProptestConfig::default()
        })]

        #[test]
        fn generated_trees_match_flattened_wildcard_precedence(
            entries in prop::collection::vec((prop::collection::vec(0..3u8, 0..4), 0..4u8), 0..13),
            generated_queries in queries(),
            subtree in 0..40usize,
        ) {
            let fixture = Fixture::new();
            let mut nodes = vec![ReferenceDirectory::default()];
            let mut routes = vec![Vec::new()];
            for (route, files) in entries {
                let mut node = 0;
                for label in route {
                    let edge = usize::from(label);
                    node = if let Some(target) = nodes[node].edges[edge] {
                        target
                    } else {
                        let target = nodes.len();
                        let mut route = routes[node].clone();
                        route.push(label);
                        nodes.push(ReferenceDirectory::default());
                        routes.push(route);
                        nodes[node].edges[edge] = Some(target);
                        target
                    };
                }
                nodes[node].files |= files;
            }
            let physical = routes.iter().map(|route| lexical_path(&fixture.0, route)).collect::<Vec<_>>();
            for (node, path) in nodes.iter().zip(&physical) {
                fs::create_dir_all(path).unwrap();
                for file in 0..2 {
                    if node.files & (1 << file) != 0 {
                        fs::write(path.join(FILENAMES[file]), "file").unwrap();
                    }
                }
            }
            let graph = DirectoryGraph::build(&fixture.0, None, BUDGET).unwrap();
            prop_assert_eq!(graph.read_dirs, nodes.len());
            let start = subtree % nodes.len();
            for query in with_root_queries(generated_queries) {
                let suffix = query.suffix_string();
                let requested = query.requested();
                prop_assert_eq!(
                    graph.find(suffix.as_deref(), &requested),
                    expected_path(&nodes, 0, None, &fixture.0, &query),
                    "query {:?}", query
                );
                prop_assert_eq!(
                    graph.find_from(&physical[start], suffix.as_deref(), &requested),
                    expected_path(&nodes, start, None, &physical[start], &query),
                    "subtree {}, query {:?}", start, query
                );
            }
        }

        #[cfg(unix)]
        #[test]
        fn generated_alias_cycles_exclusions_and_outputs_match_simple_routes(
            raw_nodes in prop::collection::vec((prop::array::uniform3(prop::option::of(0..6usize)), 0..4u8), 1..7),
            excluded in prop::option::of(0..6usize),
            generated_queries in queries(),
            output_node in 0..6usize,
        ) {
            let fixture = Fixture::new();
            let mut nodes = raw_nodes.iter().map(|(edges, files)| ReferenceDirectory {
                edges: edges.map(|target| target.map(|target| target % raw_nodes.len())),
                files: *files,
            }).collect::<Vec<_>>();
            let physical = (0..nodes.len()).map(|node| fixture.directory(&format!("physical/{node}"))).collect::<Vec<_>>();
            for (node, path) in nodes.iter().zip(&physical) {
                for (label, target) in node.edges.iter().enumerate() {
                    if let Some(target) = target {
                        std::os::unix::fs::symlink(&physical[*target], path.join(LABELS[label])).unwrap();
                    }
                }
                for file in 0..2 {
                    if node.files & (1 << file) != 0 {
                        fs::write(path.join(FILENAMES[file]), "file").unwrap();
                    }
                }
            }
            let excluded = excluded.map(|node| node % nodes.len());
            let graph = DirectoryGraph::build(&physical[0], excluded.map(|node| physical[node].as_path()), BUDGET);
            if excluded == Some(0) {
                prop_assert!(graph.is_none());
                return Ok(());
            }
            let mut graph = graph.unwrap();
            let reachable = reference_routes(&nodes, 0, excluded).into_iter().map(|(node, _)| node).collect::<HashSet<_>>();
            prop_assert_eq!(graph.read_dirs, reachable.len());
            for (node, path) in physical.iter().enumerate() {
                prop_assert_eq!(graph.contains_directory(path), reachable.contains(&node));
            }
            let start = output_node % nodes.len();
            for query in with_root_queries(generated_queries) {
                let suffix = query.suffix_string();
                let requested = query.requested();
                prop_assert_eq!(
                    graph.find(suffix.as_deref(), &requested),
                    expected_path(&nodes, 0, excluded, &physical[0], &query),
                    "query {:?}", query
                );
                let subtree_match = reachable.contains(&start).then(|| expected_path(&nodes, start, excluded, &physical[start], &query)).flatten();
                prop_assert_eq!(graph.find_from(&physical[start], suffix.as_deref(), &requested), subtree_match);
            }

            fs::write(physical[start].join(FILENAMES[2]), "output").unwrap();
            prop_assert_eq!(graph.add_output(&physical[start], OsStr::new(FILENAMES[2])), reachable.contains(&start));
            nodes[start].files |= 1 << 2;
            let bytes = graph.retained_bytes();
            prop_assert_eq!(graph.add_output(&physical[start], OsStr::new(FILENAMES[2])), reachable.contains(&start));
            prop_assert_eq!(graph.retained_bytes(), bytes);
            let output_query = Query { suffix: Some(vec![Vec::new()]), qualified: Vec::new(), file: 2 };
            prop_assert_eq!(graph.find(Some(""), Path::new(FILENAMES[2])), expected_path(&nodes, 0, excluded, &physical[0], &output_query));
            for (label, target) in nodes[0].edges.iter().enumerate() {
                if target.is_some() {
                    let query = Query { suffix: Some(vec![vec![label as u8], Vec::new()]), qualified: Vec::new(), file: 2 };
                    let suffix = query.suffix_string().unwrap();
                    prop_assert_eq!(graph.find(Some(&suffix), Path::new(FILENAMES[2])), expected_path(&nodes, 0, excluded, &physical[0], &query));
                }
            }
        }
    }

    #[test]
    fn wildcard_order_qualified_names_and_overlapping_roots() {
        let fixture = Fixture::new();
        for directory in ["tex/z", "a/tex/y", "tex/a/tex"] {
            fs::write(fixture.directory(directory).join("f.sty"), directory).unwrap();
        }
        let graph = DirectoryGraph::build(&fixture.0, None, BUDGET).unwrap();
        assert_eq!(graph.read_dirs, graph.nodes.len());
        assert_eq!(
            graph.find(Some("tex//"), Path::new("f.sty")),
            Some(fixture.0.join("tex/a/tex/f.sty"))
        );
        assert_eq!(
            graph.find(Some("tex//"), Path::new("y/f.sty")),
            Some(fixture.0.join("a/tex/y/f.sty"))
        );
        let nested = fixture.0.join("a/tex");
        assert!(graph.contains_directory(&nested));
        assert_eq!(
            graph.find_from(&nested, Some(""), Path::new("f.sty")),
            Some(nested.join("y/f.sty"))
        );
    }

    #[test]
    fn normal_entries_need_no_metadata_followups_or_unix_child_canonicalization() {
        let fixture = Fixture::new();
        let directories = [
            fixture.0.clone(),
            fixture.directory("MiXeD"),
            fixture.directory("MiXeD/SubDir"),
            fixture.directory("Other"),
        ];
        for directory in &directories {
            for n in 0..20 {
                fs::write(directory.join(format!("file-{n}.sty")), "file").unwrap();
            }
        }
        let graph = DirectoryGraph::build(&fixture.0, None, BUDGET).unwrap();
        assert_eq!(graph.read_dirs, 4);
        // These count explicit directory-entry followups, not file_type's
        // platform-internal fallback or the separate symlink-hop resolver.
        assert_eq!(graph.entry_metadata_followups, 0);
        assert_eq!(
            graph.ordinary_directory_canonicalizations,
            if cfg!(unix) { 0 } else { 3 }
        );
        assert_eq!(
            graph.directory_canonicalizations,
            if cfg!(unix) { 1 } else { 4 }
        );
        let nested = fixture.0.join("MiXeD/SubDir").canonicalize().unwrap();
        assert!(graph.contains_directory(&nested));
        assert_eq!(
            graph.find(Some("MiXeD//SubDir"), Path::new("file-0.sty")),
            Some(fixture.0.join("MiXeD/SubDir/file-0.sty"))
        );
    }

    #[cfg(unix)]
    #[test]
    fn only_symlink_entries_need_metadata_followups_and_child_canonicalization() {
        let fixture = Fixture::new();
        let root = fixture.directory("search");
        let target = fixture.directory("external");
        let deep = fixture.directory("external/DeEp");
        fs::write(deep.join("f.sty"), "file").unwrap();
        fs::write(root.join("ordinary.sty"), "file").unwrap();
        for alias in ["a", "z"] {
            std::os::unix::fs::symlink(&target, root.join(alias)).unwrap();
        }
        std::os::unix::fs::symlink(deep.join("f.sty"), root.join("leaf.sty")).unwrap();
        std::os::unix::fs::symlink(deep.join("missing.sty"), root.join("dangling.sty")).unwrap();
        let graph = DirectoryGraph::build(&root, None, BUDGET).unwrap();
        assert_eq!(graph.read_dirs, 3);
        assert_eq!(graph.entry_metadata_followups, 4);
        assert_eq!(graph.directory_canonicalizations, 3);
        assert_eq!(graph.ordinary_directory_canonicalizations, 0);
        assert_eq!(
            graph.find(Some(""), Path::new("f.sty")),
            Some(root.join("a/DeEp/f.sty"))
        );
        assert_eq!(
            graph.find(Some(""), Path::new("leaf.sty")),
            Some(root.join("leaf.sty"))
        );
        assert_eq!(graph.find(Some(""), Path::new("dangling.sty")), None);
    }

    #[test]
    fn ordinary_bundle_child_is_excluded_without_child_canonicalization_on_unix() {
        let fixture = Fixture::new();
        let bundle = fixture.directory("bundle");
        fs::write(bundle.join("private.sty"), "bundle").unwrap();
        let graph = DirectoryGraph::build(&fixture.0, Some(&bundle), BUDGET).unwrap();
        assert_eq!(graph.read_dirs, 1);
        assert_eq!(graph.entry_metadata_followups, 0);
        assert_eq!(
            graph.ordinary_directory_canonicalizations,
            usize::from(!cfg!(unix))
        );
        assert_eq!(
            graph.directory_canonicalizations,
            if cfg!(unix) { 2 } else { 3 }
        );
        assert_eq!(graph.find(Some(""), Path::new("private.sty")), None);
    }

    #[cfg(unix)]
    #[test]
    fn deep_alias_dag_inventory_and_matching_are_linear() {
        let fixture = Fixture::new();
        let root = fixture.directory("search");
        let mut directories = vec![root.clone()];
        directories.extend((0..24).map(|n| fixture.directory(&format!("physical/{n}"))));
        for pair in directories.windows(2) {
            for alias in ["a", "z"] {
                std::os::unix::fs::symlink(&pair[1], pair[0].join(alias)).unwrap();
            }
        }
        fs::write(directories.last().unwrap().join("f.sty"), "file").unwrap();
        let graph = DirectoryGraph::build(&root, None, BUDGET).unwrap();
        assert_eq!(graph.read_dirs, 25);
        assert_eq!(graph.nodes.len(), 25);
        assert_eq!(
            graph
                .nodes
                .iter()
                .map(|node| node.edges.len())
                .sum::<usize>(),
            48
        );
        assert_eq!(graph.find(Some(""), Path::new("missing.sty")), None);
        let (missing, states) = graph.match_from(0, &root, Some("absent//"), Path::new("f.sty"));
        assert_eq!(missing, None);
        assert_eq!(states, 50);
        let all_a = (0..24).fold(root.clone(), |path, _| path.join("a"));
        assert_eq!(
            graph.find(Some(""), Path::new("f.sty")),
            Some(all_a.join("f.sty"))
        );
        let outer_z = (0..23).fold(root.join("z"), |path, _| path.join("a"));
        assert_eq!(
            graph.find(Some("z//"), Path::new("f.sty")),
            Some(outer_z.join("f.sty"))
        );
        let qualified_z = (0..23)
            .fold(root.clone(), |path, _| path.join("a"))
            .join("z/f.sty");
        assert_eq!(
            graph.find(Some(""), Path::new("z/f.sty")),
            Some(qualified_z)
        );
        let physical = directories.last().unwrap();
        assert_eq!(
            graph.find_from(physical, Some(""), Path::new("f.sty")),
            Some(physical.join("f.sty"))
        );
    }

    #[cfg(unix)]
    #[test]
    fn aliases_keep_their_names_and_output_insertion_updates_all_routes() {
        let fixture = Fixture::new();
        let root = fixture.directory("search");
        let target = fixture.directory("external");
        for alias in ["a", "z"] {
            std::os::unix::fs::symlink(&target, root.join(alias)).unwrap();
        }
        let mut graph = DirectoryGraph::build(&root, None, BUDGET).unwrap();
        assert_eq!(graph.read_dirs, 2);
        assert_eq!(graph.find(Some("z//"), Path::new("created.tex")), None);
        fs::write(target.join("created.tex"), "output").unwrap();
        assert!(graph.add_output(&target, OsStr::new("created.tex")));
        assert_eq!(
            graph.find(Some(""), Path::new("created.tex")),
            Some(root.join("a/created.tex"))
        );
        assert_eq!(
            graph.find(Some("z//"), Path::new("created.tex")),
            Some(root.join("z/created.tex"))
        );
        assert!(!graph.add_output(&target.join("new"), OsStr::new("missing.tex")));
        assert_eq!(graph.read_dirs, 2);
    }

    #[cfg(unix)]
    #[test]
    fn an_unusable_deep_spelling_does_not_hide_a_short_alias() {
        let fixture = Fixture::new();
        let root = fixture.directory("search");
        let mut directories = vec![root.clone()];
        directories.extend((0..80).map(|n| fixture.directory(&format!("physical/{n}"))));
        for pair in directories.windows(2) {
            std::os::unix::fs::symlink(&pair[1], pair[0].join("a")).unwrap();
        }
        let terminal = directories.last().unwrap();
        std::os::unix::fs::symlink(terminal, root.join("z")).unwrap();
        fs::write(terminal.join("f.sty"), "file").unwrap();
        let graph = DirectoryGraph::build(&root, None, BUDGET).unwrap();
        assert_eq!(graph.read_dirs, 81);
        assert_eq!(
            graph.find(Some(""), Path::new("f.sty")),
            Some(root.join("z/f.sty"))
        );
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn deep_binary_alias_dag_respects_os_hop_limits_without_alias_expansion() {
        let fixture = Fixture::new();
        let root = fixture.directory("search");
        let mut directories = vec![root.clone()];
        directories.extend((0..48).map(|n| fixture.directory(&format!("physical/{n}"))));
        for pair in directories.windows(2) {
            for alias in ["a", "b"] {
                std::os::unix::fs::symlink(&pair[1], pair[0].join(alias)).unwrap();
            }
        }
        let terminal = directories.last().unwrap();
        fs::write(terminal.join("f.sty"), "file").unwrap();
        let graph = DirectoryGraph::build(&root, None, BUDGET).unwrap();
        let (found, states) = graph.match_from(0, &root, Some(""), Path::new("f.sty"));
        assert_eq!(found, None);
        assert_eq!(graph.read_dirs, 49);
        assert_eq!(states, 2 * (hop_limit().unwrap() + 1));

        std::os::unix::fs::symlink(terminal, root.join("z")).unwrap();
        let graph = DirectoryGraph::build(&root, None, BUDGET).unwrap();
        let (found, states) = graph.match_from(0, &root, Some(""), Path::new("f.sty"));
        assert_eq!(found, Some(root.join("z/f.sty")));
        assert_eq!(graph.read_dirs, 49);
        assert_eq!(states, 2 * (hop_limit().unwrap() + 1) + 2);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn supported_hop_boundary_matches_actual_filesystem_resolution() {
        let fixture = Fixture::new();
        let root = fixture.directory("search");
        let limit = hop_limit().unwrap();
        let mut physical = root.clone();
        let mut lexical = root.clone();
        for n in 0..=limit {
            let next = fixture.directory(&format!("physical/{n}"));
            std::os::unix::fs::symlink(&next, physical.join("a")).unwrap();
            fs::write(next.join("f.sty"), "file").unwrap();
            physical = next;
            lexical.push("a");
            if n + 1 == limit {
                assert!(lexical.join("f.sty").is_file());
            } else if n == limit {
                assert!(!lexical.join("f.sty").is_file());
            }
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn nested_raw_targets_and_leaf_symlinks_count_every_hop() {
        let fixture = Fixture::new();
        let root = fixture.directory("search");
        let target = fixture.directory("physical/leaf");
        let bridges = fixture.directory("bridges");
        let files = fixture.directory("files");
        std::os::unix::fs::symlink(&target, bridges.join("second")).unwrap();
        std::os::unix::fs::symlink("second", bridges.join("first")).unwrap();
        std::os::unix::fs::symlink("../bridges/first", root.join("a")).unwrap();
        fs::write(files.join("plain.sty"), "file").unwrap();
        std::os::unix::fs::symlink("plain.sty", files.join("second.sty")).unwrap();
        std::os::unix::fs::symlink("second.sty", files.join("first.sty")).unwrap();
        std::os::unix::fs::symlink("../../files/first.sty", target.join("f.sty")).unwrap();
        assert_eq!(resolve_hops(&root, Path::new("a")), Some(3));
        assert_eq!(resolve_hops(&target, Path::new("f.sty")), Some(3));
        let graph = DirectoryGraph::build(&root, None, BUDGET).unwrap();
        assert_eq!(graph.nodes[0].edges[0].hops, Some(3));
        assert_eq!(
            graph.find(Some(""), Path::new("f.sty")),
            Some(root.join("a/f.sty"))
        );

        // A long route uses limit - 1 directory hops, then three leaf hops.
        // Its miss must not hide the six-hop a/f.sty spelling above.
        let mut previous = root.clone();
        for n in 0..hop_limit().unwrap() - 2 {
            let next = fixture.directory(&format!("long/{n}"));
            std::os::unix::fs::symlink(&next, previous.join("0")).unwrap();
            previous = next;
        }
        std::os::unix::fs::symlink(&target, previous.join("0")).unwrap();
        let graph = DirectoryGraph::build(&root, None, BUDGET).unwrap();
        assert_eq!(
            graph.find(Some(""), Path::new("f.sty")),
            Some(root.join("a/f.sty"))
        );
    }

    #[cfg(unix)]
    #[test]
    fn cyclic_misses_do_not_poison_other_ancestor_contexts() {
        let fixture = Fixture::new();
        let root = fixture.directory("search");
        let x = fixture.directory("x");
        let y = fixture.directory("y");
        std::os::unix::fs::symlink(&x, root.join("a")).unwrap();
        std::os::unix::fs::symlink(&y, root.join("z")).unwrap();
        std::os::unix::fs::symlink(&y, x.join("b")).unwrap();
        std::os::unix::fs::symlink(&x, y.join("back")).unwrap();
        fs::write(x.join("f.sty"), "file").unwrap();
        let graph = DirectoryGraph::build(&root, None, BUDGET).unwrap();
        assert_eq!(graph.read_dirs, 3);
        assert_eq!(
            graph.find(Some("back//"), Path::new("f.sty")),
            Some(root.join("z/back/f.sty"))
        );
        assert_eq!(graph.find(Some("back//back//"), Path::new("f.sty")), None);
        assert_eq!(
            graph.find(Some(""), Path::new("back/f.sty")),
            Some(root.join("z/back/f.sty"))
        );
    }

    #[cfg(unix)]
    #[test]
    fn bundle_aliases_are_excluded_and_self_cycles_terminate() {
        let fixture = Fixture::new();
        let root = fixture.directory("search");
        let bundle = fixture.directory("bundle");
        std::os::unix::fs::symlink(&bundle, root.join("alias")).unwrap();
        std::os::unix::fs::symlink(&root, root.join("cycle")).unwrap();
        fs::write(bundle.join("bundled.sty"), "bundle").unwrap();
        fs::write(root.join("f.sty"), "file").unwrap();
        let graph = DirectoryGraph::build(&root, Some(&bundle), BUDGET).unwrap();
        assert_eq!(graph.read_dirs, 1);
        assert!(!graph.contains_directory(&bundle));
        assert_eq!(graph.find(Some(""), Path::new("bundled.sty")), None);
        assert_eq!(graph.find(Some("cycle//"), Path::new("f.sty")), None);
        assert!(DirectoryGraph::build(&bundle, Some(&bundle), BUDGET).is_none());
    }

    #[test]
    fn retention_admission_and_output_growth_respect_the_budget() {
        let fixture = Fixture::new();
        let mut graph = DirectoryGraph::build(&fixture.0, None, BUDGET).unwrap();
        let bytes = graph.retained_bytes();
        assert!(DirectoryGraph::build(&fixture.0, None, bytes - 1).is_none());
        graph.budget = bytes;
        assert!(!graph.add_output(&fixture.0, OsStr::new("over-budget.tex")));
        assert_eq!(graph.retained_bytes(), bytes);
    }
}

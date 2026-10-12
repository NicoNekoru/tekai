//! Bounded source regression checks, not a memory-safety proof or a dependency
//! audit. Follow compiled module declarations rather than every file in src.
//! Native test oracles are excluded only when their cfg requires test=true.
//! Cross-module namespace aliases are refused at their import rather than
//! assumed resolvable through arbitrary reexports.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::visit::{self, Visit};
use syn::{
    Attribute, Expr, Fields, GenericArgument, Item, Lit, Meta, PathArguments, Stmt, Token, Type,
};

const MAX_FILES: usize = 128;
const MAX_SOURCE_BYTES: usize = 16 * 1024 * 1024;

// These contain borrowed routing pointers and optional reentrancy flags only.
// Scope lifetime and cleanup behavior are covered by the owners' runtime tests.
const TLS: &[(&str, &str, &str)] = &[
    ("src/allocation.rs", "ACTIVE_ARENA", "ArenaState"),
    ("src/control.rs", "ACTIVE", "ControlSession"),
    ("src/environment.rs", "ACTIVE", "EnvironmentSession"),
    (
        "src/generated/backend/writezip.rs",
        "ACTIVE",
        "CompressionSession",
    ),
    ("src/invocation_state.rs", "ACTIVE", "Storage"),
    ("src/ipc.rs", "ACTIVE", "IpcSession"),
    ("src/kpathsea.rs", "ACTIVE_INDEX", "IndexSession"),
    ("src/lookup.rs", "ACTIVE", "LookupOwner"),
    ("src/pdftoepdf.rs", "ACTIVE", "PdfInclusionSession"),
    ("src/pngshim.rs", "ACTIVE", "PngSession"),
    ("src/regex_session.rs", "ACTIVE", "RegexSession"),
    ("src/runtime.rs", "ACTIVE", "RuntimeSession"),
    ("src/stdio.rs", "ACTIVE_SESSION", "StreamSession"),
    ("src/synctex.rs", "ACTIVE", "SyncTexSession"),
    ("src/time.rs", "ACTIVE", "TimeSession"),
    ("src/working_directory.rs", "ACTIVE", "WorkingDirectory"),
    ("src/xpdf.rs", "ACTIVE", "XpdfSession"),
];

#[derive(Clone, Copy, PartialEq)]
enum Truth {
    Yes,
    No,
    Maybe,
}
impl Truth {
    fn not(self) -> Self {
        match self {
            Self::Yes => Self::No,
            Self::No => Self::Yes,
            Self::Maybe => Self::Maybe,
        }
    }
    fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::No, _) | (_, Self::No) => Self::No,
            (Self::Yes, Self::Yes) => Self::Yes,
            _ => Self::Maybe,
        }
    }
    fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::Yes, _) | (_, Self::Yes) => Self::Yes,
            (Self::No, Self::No) => Self::No,
            _ => Self::Maybe,
        }
    }
}

fn metas(list: &syn::MetaList) -> Vec<Meta> {
    Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(list.tokens.clone())
        .expect("valid cfg metadata")
        .into_iter()
        .collect()
}

// Evaluate with test=false and every unrelated target/feature predicate unknown.
// Unknown keeps an item in scope. In particular, any(test, unix) is production.
fn cfg(meta: &Meta) -> Truth {
    match meta {
        Meta::Path(path) if path.is_ident("test") => Truth::No,
        Meta::List(list) if list.path.is_ident("all") => {
            metas(list).iter().fold(Truth::Yes, |a, b| a.and(cfg(b)))
        }
        Meta::List(list) if list.path.is_ident("any") => {
            metas(list).iter().fold(Truth::No, |a, b| a.or(cfg(b)))
        }
        Meta::List(list) if list.path.is_ident("not") => {
            let entries = metas(list);
            assert_eq!(entries.len(), 1, "cfg(not) has one operand");
            cfg(&entries[0]).not()
        }
        _ => Truth::Maybe,
    }
}

fn attribute_condition(meta: &Meta) -> Truth {
    match meta {
        Meta::List(list) if list.path.is_ident("cfg") => {
            let entries = metas(list);
            assert_eq!(entries.len(), 1, "cfg has one predicate");
            cfg(&entries[0])
        }
        Meta::List(list) if list.path.is_ident("cfg_attr") => {
            let entries = metas(list);
            let (condition, attributes) = entries.split_first().expect("cfg_attr predicate");
            cfg(condition).not().or(attributes
                .iter()
                .fold(Truth::Yes, |a, b| a.and(attribute_condition(b))))
        }
        _ => Truth::Yes,
    }
}

fn production(attrs: &[Attribute]) -> bool {
    !attrs.iter().any(|attr| attr.path().is_ident("test"))
        && attrs
            .iter()
            .fold(Truth::Yes, |a, b| a.and(attribute_condition(&b.meta)))
            != Truth::No
}

fn conditional_module_path(meta: &Meta) -> bool {
    let Meta::List(list) = meta else {
        return meta.path().is_ident("path");
    };
    if !list.path.is_ident("cfg_attr") {
        return false;
    }
    let entries = metas(list);
    let Some((condition, attributes)) = entries.split_first() else {
        return false;
    };
    cfg(condition) != Truth::No && attributes.iter().any(conditional_module_path)
}

fn item_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(x) => &x.attrs,
        Item::Enum(x) => &x.attrs,
        Item::ExternCrate(x) => &x.attrs,
        Item::Fn(x) => &x.attrs,
        Item::ForeignMod(x) => &x.attrs,
        Item::Impl(x) => &x.attrs,
        Item::Macro(x) => &x.attrs,
        Item::Mod(x) => &x.attrs,
        Item::Static(x) => &x.attrs,
        Item::Struct(x) => &x.attrs,
        Item::Trait(x) => &x.attrs,
        Item::TraitAlias(x) => &x.attrs,
        Item::Type(x) => &x.attrs,
        Item::Union(x) => &x.attrs,
        Item::Use(x) => &x.attrs,
        _ => &[],
    }
}

fn expr_attrs(expr: &Expr) -> &[Attribute] {
    match expr {
        Expr::Array(x) => &x.attrs,
        Expr::Assign(x) => &x.attrs,
        Expr::Async(x) => &x.attrs,
        Expr::Await(x) => &x.attrs,
        Expr::Binary(x) => &x.attrs,
        Expr::Block(x) => &x.attrs,
        Expr::Break(x) => &x.attrs,
        Expr::Call(x) => &x.attrs,
        Expr::Cast(x) => &x.attrs,
        Expr::Closure(x) => &x.attrs,
        Expr::Const(x) => &x.attrs,
        Expr::Continue(x) => &x.attrs,
        Expr::Field(x) => &x.attrs,
        Expr::ForLoop(x) => &x.attrs,
        Expr::Group(x) => &x.attrs,
        Expr::If(x) => &x.attrs,
        Expr::Index(x) => &x.attrs,
        Expr::Infer(x) => &x.attrs,
        Expr::Let(x) => &x.attrs,
        Expr::Lit(x) => &x.attrs,
        Expr::Loop(x) => &x.attrs,
        Expr::Macro(x) => &x.attrs,
        Expr::Match(x) => &x.attrs,
        Expr::MethodCall(x) => &x.attrs,
        Expr::Paren(x) => &x.attrs,
        Expr::Path(x) => &x.attrs,
        Expr::Range(x) => &x.attrs,
        Expr::RawAddr(x) => &x.attrs,
        Expr::Reference(x) => &x.attrs,
        Expr::Repeat(x) => &x.attrs,
        Expr::Return(x) => &x.attrs,
        Expr::Struct(x) => &x.attrs,
        Expr::Try(x) => &x.attrs,
        Expr::TryBlock(x) => &x.attrs,
        Expr::Tuple(x) => &x.attrs,
        Expr::Unary(x) => &x.attrs,
        Expr::Unsafe(x) => &x.attrs,
        Expr::While(x) => &x.attrs,
        Expr::Yield(x) => &x.attrs,
        _ => &[],
    }
}

#[derive(Default)]
struct Names {
    imports: BTreeMap<String, Vec<String>>,
    namespaces: BTreeSet<String>,
    structures: BTreeMap<String, syn::ItemStruct>,
}

fn imports(tree: &syn::UseTree, prefix: &[String], out: &mut Vec<(String, Vec<String>)>) {
    match tree {
        syn::UseTree::Path(p) => {
            let mut next = prefix.to_vec();
            next.push(p.ident.to_string());
            imports(&p.tree, &next, out);
        }
        syn::UseTree::Name(n) => {
            let mut path = prefix.to_vec();
            let name = if n.ident == "self" {
                prefix.last().expect("self import prefix").clone()
            } else {
                path.push(n.ident.to_string());
                n.ident.to_string()
            };
            out.push((name, path));
        }
        syn::UseTree::Rename(r) => {
            let mut path = prefix.to_vec();
            if r.ident != "self" {
                path.push(r.ident.to_string());
            }
            out.push((r.rename.to_string(), path));
        }
        syn::UseTree::Glob(_) => {
            let mut path = prefix.to_vec();
            path.push("*".into());
            out.push(("*".into(), path));
        }
        syn::UseTree::Group(g) => {
            for item in &g.items {
                imports(item, prefix, out);
            }
        }
    }
}

impl Names {
    fn from_items(items: &[Item]) -> Self {
        let mut names = Self::default();
        for item in items.iter().filter(|item| production(item_attrs(item))) {
            match item {
                Item::Use(u) => {
                    let mut rows = Vec::new();
                    imports(&u.tree, &[], &mut rows);
                    names.imports.extend(rows);
                }
                Item::ExternCrate(e) => {
                    names.imports.insert(
                        e.rename
                            .as_ref()
                            .map(|(_, n)| n)
                            .unwrap_or(&e.ident)
                            .to_string(),
                        vec![e.ident.to_string()],
                    );
                }
                Item::Struct(s) => {
                    names.namespaces.insert(s.ident.to_string());
                    names.structures.insert(s.ident.to_string(), s.clone());
                }
                Item::Enum(s) => {
                    names.namespaces.insert(s.ident.to_string());
                }
                Item::Type(s) => {
                    names.namespaces.insert(s.ident.to_string());
                }
                Item::Union(s) => {
                    names.namespaces.insert(s.ident.to_string());
                }
                Item::Mod(s) => {
                    names.namespaces.insert(s.ident.to_string());
                }
                _ => {}
            }
        }
        names
    }
}

#[derive(Clone)]
struct Source {
    file: PathBuf,
    module_dir: PathBuf,
    inline: bool,
}
#[derive(Default)]
struct Report {
    errors: Vec<String>,
    files: BTreeSet<PathBuf>,
    bytes: usize,
    tls: BTreeSet<(String, String)>,
    embedded_include: usize,
}

struct Audit<'a> {
    root: &'a Path,
    source: Source,
    names: Vec<Names>,
    report: &'a mut Report,
    pending: &'a mut VecDeque<Source>,
    item: String,
    cli: bool,
    embedded: bool,
}

impl Audit<'_> {
    fn relative(&self) -> String {
        self.source
            .file
            .strip_prefix(self.root)
            .unwrap_or(&self.source.file)
            .to_string_lossy()
            .into_owned()
    }
    fn error(&mut self, message: impl AsRef<str>) {
        self.report.errors.push(format!(
            "{} [{}] {}",
            self.relative(),
            self.item,
            message.as_ref()
        ));
    }
    fn resolved(&self, path: &syn::Path) -> Vec<String> {
        let parts = path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>();
        if path.leading_colon.is_some() || parts.is_empty() {
            return parts;
        }
        self.expand(parts)
    }
    fn expand(&self, mut parts: Vec<String>) -> Vec<String> {
        for _ in 0..16 {
            let Some(first) = parts.first() else {
                return parts;
            };
            let mut next = None;
            for names in self.names.iter().rev() {
                if names.namespaces.contains(first) {
                    return parts;
                }
                if let Some(imported) = names.imports.get(first) {
                    let mut expanded = imported.clone();
                    expanded.extend(parts.iter().skip(1).cloned());
                    next = Some(expanded);
                    break;
                }
            }
            let Some(next) = next else {
                return parts;
            };
            if next == parts {
                return parts;
            }
            parts = next;
        }
        parts
    }
    fn path_is(&self, path: &syn::Path, choices: &[&[&str]]) -> bool {
        let parts = self.resolved(path);
        choices
            .iter()
            .any(|choice| parts.iter().map(String::as_str).eq(choice.iter().copied()))
    }
    fn check_external_path(&mut self, parts: &[String]) {
        if parts.first().is_some_and(|p| p == "libc") && parts.len() >= 2 {
            let name = &parts[1];
            // Type/constant references are data, not utility implementation calls.
            let sdk_group_record =
                self.relative() == "src/process_group.rs" && name == "proc_bsdinfo";
            let sdk_terminal_mask =
                self.relative() == "src/cli_supervisor.rs" && name == "sigset_t";
            let data = sdk_group_record
                || sdk_terminal_mask
                || name.starts_with("c_")
                || matches!(
                    name.as_str(),
                    "size_t" | "ssize_t" | "off_t" | "dev_t" | "ino_t" | "time_t" | "wchar_t"
                )
                || name
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
            let os_group_query = self.relative() == "src/process_group.rs"
                && matches!(name.as_str(), "proc_listpids" | "proc_pidinfo");
            let os_terminal_mask = self.relative() == "src/cli_supervisor.rs"
                && matches!(
                    name.as_str(),
                    "pthread_sigmask" | "sigemptyset" | "sigaddset"
                );
            if !data
                && !os_group_query
                && !os_terminal_mask
                && !matches!(
                    name.as_str(),
                    "access" | "__error" | "__errno_location" | "getpid"
                )
            {
                self.error(format!(
                    "unapproved production libc reference {}",
                    parts.join("::")
                ));
            }
        }
        if !self.cli
            && parts.first().is_some_and(|p| p == "std")
            && (parts.get(1).is_some_and(|p| p == "*")
                || (parts.get(1).is_some_and(|p| p == "process")
                    && parts
                        .get(2)
                        .is_some_and(|p| matches!(p.as_str(), "exit" | "abort" | "*"))))
        {
            self.error(format!("host termination reference {}", parts.join("::")));
        }
    }
    fn scalar(&self, ty: &Type) -> bool {
        let Type::Path(p) = ty else {
            return false;
        };
        let parts = self.resolved(&p.path);
        parts.len() == 1
            && matches!(
                parts[0].as_str(),
                "bool"
                    | "char"
                    | "u8"
                    | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "usize"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "isize"
                    | "f32"
                    | "f64"
            )
    }
    fn readonly(&self, ty: &Type) -> bool {
        match ty {
            Type::Array(a) => self.readonly(&a.elem),
            Type::Slice(s) => self.readonly(&s.elem),
            Type::Reference(r) => r.mutability.is_none() && self.readonly(&r.elem),
            Type::Tuple(t) => t.elems.iter().all(|t| self.readonly(t)),
            _ => self.scalar(ty),
        }
    }
    fn single_type<'a>(&self, ty: &'a Type, paths: &[&[&str]]) -> Option<&'a Type> {
        let Type::Path(p) = ty else {
            return None;
        };
        if !self.path_is(&p.path, paths) {
            return None;
        }
        let PathArguments::AngleBracketed(a) = &p.path.segments.last()?.arguments else {
            return None;
        };
        if a.args.len() != 1 {
            return None;
        }
        match a.args.first()? {
            GenericArgument::Type(ty) => Some(ty),
            _ => None,
        }
    }
    fn cell<'a>(&self, ty: &'a Type) -> Option<&'a Type> {
        self.single_type(ty, &[&["std", "cell", "Cell"], &["core", "cell", "Cell"]])
    }
    fn borrowed_pointer(&self, ty: &Type, owner: &str) -> bool {
        let target = if let Type::Ptr(p) = ty {
            Some(p.elem.as_ref())
        } else {
            self.single_type(
                ty,
                &[
                    &["Option"],
                    &["std", "option", "Option"],
                    &["core", "option", "Option"],
                ],
            )
            .and_then(|ty| {
                self.single_type(
                    ty,
                    &[&["std", "ptr", "NonNull"], &["core", "ptr", "NonNull"]],
                )
            })
        };
        matches!(target, Some(Type::Path(p)) if p.qself.is_none() && p.path.is_ident(owner))
    }
    fn peeled<'a>(&self, expr: &'a Expr) -> Option<&'a Expr> {
        match expr {
            Expr::Const(c) => match c.block.stmts.as_slice() {
                [Stmt::Expr(e, _)] => self.peeled(e),
                _ => None,
            },
            Expr::Paren(p) => self.peeled(&p.expr),
            Expr::Group(g) => self.peeled(&g.expr),
            _ => Some(expr),
        }
    }
    fn cell_value<'a>(&self, expr: &'a Expr) -> Option<&'a Expr> {
        let Expr::Call(call) = self.peeled(expr)? else {
            return None;
        };
        let Expr::Path(p) = call.func.as_ref() else {
            return None;
        };
        if !self.path_is(
            &p.path,
            &[
                &["std", "cell", "Cell", "new"],
                &["core", "cell", "Cell", "new"],
            ],
        ) || call.args.len() != 1
        {
            return None;
        }
        self.peeled(call.args.first()?)
    }
    fn empty_pointer(&self, expr: &Expr) -> bool {
        match self.cell_value(expr) {
            Some(Expr::Path(p)) => self.path_is(
                &p.path,
                &[
                    &["None"],
                    &["Option", "None"],
                    &["std", "option", "Option", "None"],
                    &["core", "option", "Option", "None"],
                ],
            ),
            Some(Expr::Call(c)) if c.args.is_empty() => {
                matches!(c.func.as_ref(), Expr::Path(p) if self.path_is(&p.path, &[&["std", "ptr", "null"], &["std", "ptr", "null_mut"], &["core", "ptr", "null"], &["core", "ptr", "null_mut"]]))
            }
            _ => false,
        }
    }
    fn false_flag(&self, expr: &Expr) -> bool {
        matches!(self.cell_value(expr), Some(Expr::Lit(l)) if matches!(&l.lit, Lit::Bool(b) if !b.value))
    }
    fn routing_static(&mut self, item: &syn::ItemStatic, owner: &str) -> bool {
        if matches!(item.mutability, syn::StaticMutability::Mut(_)) {
            return false;
        }
        if let Some(inner) = self.cell(&item.ty) {
            return self.borrowed_pointer(inner, owner) && self.empty_pointer(&item.expr);
        }
        let Type::Path(p) = item.ty.as_ref() else {
            return false;
        };
        let Some(name) = p.path.get_ident() else {
            return false;
        };
        let structure = self
            .names
            .iter()
            .rev()
            .find_map(|n| n.structures.get(&name.to_string()))
            .cloned();
        let Some(structure) = structure else {
            return false;
        };
        let Fields::Named(fields) = structure.fields else {
            return false;
        };
        let fields = fields
            .named
            .iter()
            .filter(|f| production(&f.attrs))
            .collect::<Vec<_>>();
        if fields.len() != 2
            || !fields.iter().any(|f| {
                f.ident.as_ref().is_some_and(|n| n == "pointer")
                    && self
                        .cell(&f.ty)
                        .is_some_and(|t| self.borrowed_pointer(t, owner))
            })
            || !fields.iter().any(|f| {
                f.ident.as_ref().is_some_and(|n| n == "busy")
                    && self
                        .cell(&f.ty)
                        .is_some_and(|t| matches!(t, Type::Path(p) if p.path.is_ident("bool")))
            })
        {
            return false;
        }
        let Some(Expr::Struct(initial)) = self.peeled(&item.expr) else {
            return false;
        };
        initial.path.is_ident(name)
            && initial.rest.is_none()
            && initial.fields.len() == 2
            && initial.fields.iter().all(|f| match &f.member {
                syn::Member::Named(n) if n == "pointer" => self.empty_pointer(&f.expr),
                syn::Member::Named(n) if n == "busy" => self.false_flag(&f.expr),
                _ => false,
            })
            && initial
                .fields
                .iter()
                .any(|f| matches!(&f.member, syn::Member::Named(n) if n == "pointer"))
            && initial
                .fields
                .iter()
                .any(|f| matches!(&f.member, syn::Member::Named(n) if n == "busy"))
    }
    fn tls_macro(&mut self, mac: &syn::Macro) {
        let Ok(file) = syn::parse2::<syn::File>(mac.tokens.clone()) else {
            self.error("cannot parse thread_local declarations");
            return;
        };
        for item in file.items.iter().filter(|i| production(item_attrs(i))) {
            let Item::Static(s) = item else {
                self.error("thread_local contains a non-static item");
                continue;
            };
            let relative = self.relative();
            let name = s.ident.to_string();
            let Some((_, _, owner)) = TLS
                .iter()
                .find(|(file, expected, _)| *file == relative && *expected == name)
            else {
                self.error(format!("unapproved thread_local static {name}"));
                continue;
            };
            if !self.routing_static(s, owner) {
                self.error(format!("thread_local {name} is not an empty borrowed routing pointer with optional false busy flag"));
            }
            if !self.report.tls.insert((relative, name)) {
                self.error("duplicate scoped TLS declaration");
            }
            self.visit_expr(&s.expr);
        }
    }
    fn module(&mut self, m: &syn::ItemMod) {
        if m.attrs
            .iter()
            .any(|a| a.path().is_ident("cfg_attr") && conditional_module_path(&a.meta))
        {
            self.error("conditional module path requires an explicit inventory rule");
            return;
        }
        if let Some((_, items)) = &m.content {
            let old = self.source.clone();
            self.source.module_dir.push(m.ident.to_string());
            self.source.inline = true;
            self.names.push(Names::from_items(items));
            for item in items {
                self.visit_item(item);
            }
            self.names.pop();
            self.source = old;
        } else {
            let explicit =
                m.attrs
                    .iter()
                    .find(|a| a.path().is_ident("path"))
                    .and_then(|a| match &a.meta {
                        Meta::NameValue(n) => match &n.value {
                            Expr::Lit(l) => match &l.lit {
                                Lit::Str(s) => Some(s.value()),
                                _ => None,
                            },
                            _ => None,
                        },
                        _ => None,
                    });
            let file = if let Some(path) = explicit {
                (if self.source.inline {
                    &self.source.module_dir
                } else {
                    self.source.file.parent().expect("source parent")
                })
                .join(path)
            } else {
                let plain = self.source.module_dir.join(format!("{}.rs", m.ident));
                let directory = self
                    .source
                    .module_dir
                    .join(m.ident.to_string())
                    .join("mod.rs");
                if plain.is_file() && directory.is_file() {
                    self.error(format!("ambiguous module {}", m.ident));
                    return;
                }
                if plain.is_file() {
                    plain
                } else {
                    directory
                }
            };
            let dir = if file.file_name().is_some_and(|n| n == "mod.rs") {
                file.parent().unwrap().to_path_buf()
            } else {
                file.with_extension("")
            };
            self.pending.push_back(Source {
                file,
                module_dir: dir,
                inline: false,
            });
        }
    }
    fn standard_macro(&mut self, mac: &syn::Macro) {
        let parts = self.resolved(&mac.path);
        let name = parts.last().map(String::as_str).unwrap_or("");
        if name == "thread_local" && (parts.len() == 1 || parts.first().is_some_and(|p| p == "std"))
        {
            self.tls_macro(mac);
            return;
        }
        if name == "include" {
            if self.relative() == "src/embedded.rs"
                && mac.tokens.to_string()
                    == "concat ! (env ! (\"OUT_DIR\") , \"/embedded_data.rs\")"
            {
                self.report.embedded_include += 1;
            } else {
                self.error("unapproved source include macro");
            }
            return;
        }
        if matches!(name, "signed_arg" | "unsigned_arg") && self.relative() == "src/printf.rs" {
            match Punctuated::<Type, Token![,]>::parse_terminated.parse2(mac.tokens.clone()) {
                Ok(types) => {
                    for ty in &types {
                        self.visit_type(ty);
                    }
                }
                Err(e) => self.error(format!("cannot audit numeric argument macro {e}")),
            }
            return;
        }
        if name == "options" && self.relative() == "src/engine_options.rs" {
            if syn::parse2::<OptionsInput>(mac.tokens.clone()).is_err() {
                self.error("cannot audit option table macro");
            }
            return;
        }
        let standard = parts.len() == 1 || parts.first().is_some_and(|p| p == "std" || p == "core");
        if !standard {
            self.error(format!("unapproved production macro {}", parts.join("::")));
            return;
        }
        if self.embedded
            && parts
                .iter()
                .map(String::as_str)
                .eq(["core", "arch", "global_asm"])
        {
            return;
        }
        if name == "cfg" {
            if syn::parse2::<Meta>(mac.tokens.clone()).is_err() {
                self.error("cannot audit cfg macro metadata");
            }
            return;
        }
        // stringify produces text, not executable syntax. Its standard macro
        // identity is checked above, while user macro definitions fail closed.
        if name == "stringify" {
            return;
        }
        if name == "matches" {
            match syn::parse2::<MatchesInput>(mac.tokens.clone()) {
                Ok(input) => {
                    self.visit_expr(&input.expr);
                    self.visit_pat(&input.pattern);
                    if let Some(guard) = &input.guard {
                        self.visit_expr(guard);
                    }
                }
                Err(e) => self.error(format!("cannot audit matches macro {e}")),
            }
            return;
        }
        if name == "vec" {
            if let Ok(values) =
                Punctuated::<Expr, Token![,]>::parse_terminated.parse2(mac.tokens.clone())
            {
                for e in &values {
                    self.visit_expr(e);
                }
            } else if let Ok(repeat) = syn::parse2::<RepeatInput>(mac.tokens.clone()) {
                self.visit_expr(&repeat.value);
                self.visit_expr(&repeat.count);
            } else {
                self.error("cannot audit vec macro expressions");
            }
            return;
        }
        if !matches!(
            name,
            "assert"
                | "assert_eq"
                | "assert_ne"
                | "debug_assert"
                | "debug_assert_eq"
                | "debug_assert_ne"
                | "format"
                | "format_args"
                | "write"
                | "writeln"
                | "println"
                | "eprintln"
                | "print"
                | "eprint"
                | "panic"
                | "compile_error"
                | "unreachable"
                | "todo"
                | "concat"
                | "concat_bytes"
                | "env"
                | "option_env"
                | "include_str"
                | "include_bytes"
                | "addr_of"
                | "addr_of_mut"
                | "file"
                | "line"
                | "column"
                | "stringify"
        ) {
            self.error(format!("unapproved production macro {name}"));
            return;
        }
        match Punctuated::<Expr, Token![,]>::parse_terminated.parse2(mac.tokens.clone()) {
            Ok(values) => {
                for e in &values {
                    self.visit_expr(e);
                }
            }
            Err(e) => self.error(format!("cannot audit {name} macro expressions {e}")),
        }
    }
}

impl<'ast> Visit<'ast> for Audit<'_> {
    fn visit_item(&mut self, item: &'ast Item) {
        if !production(item_attrs(item)) {
            return;
        }
        let next = match item {
            Item::Fn(i) => format!("fn {}", i.sig.ident),
            Item::Static(i) => format!("static {}", i.ident),
            Item::Type(i) => format!("type {}", i.ident),
            Item::Mod(i) => format!("mod {}", i.ident),
            Item::Macro(i) => format!(
                "macro {}",
                i.ident
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| i.mac.path.segments.last().unwrap().ident.to_string())
            ),
            _ => self.item.clone(),
        };
        let old = std::mem::replace(&mut self.item, next);
        visit::visit_item(self, item);
        self.item = old;
    }
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        self.module(item);
    }
    fn visit_item_static(&mut self, item: &'ast syn::ItemStatic) {
        if matches!(item.mutability, syn::StaticMutability::Mut(_)) || !self.readonly(&item.ty) {
            self.error(format!("persistent static owner {}", item.ident));
        }
        visit::visit_item_static(self, item);
    }
    fn visit_item_foreign_mod(&mut self, item: &'ast syn::ItemForeignMod) {
        if !self.embedded {
            self.error("production foreign interface block");
        }
        // The separate embedded checker requires the exact six readonly symbols.
        // Do not exempt their types or any normal Rust functions surrounding them.
        if self.embedded {
            for foreign in &item.items {
                if let syn::ForeignItem::Static(s) = foreign {
                    self.visit_type(&s.ty);
                }
            }
        }
    }
    fn visit_signature(&mut self, signature: &'ast syn::Signature) {
        if signature
            .abi
            .as_ref()
            .is_some_and(|a| a.name.as_ref().is_none_or(|n| n.value() != "Rust"))
        {
            self.error(format!("non-Rust function ABI on {}", signature.ident));
        }
        visit::visit_signature(self, signature);
    }
    fn visit_type_bare_fn(&mut self, ty: &'ast syn::TypeBareFn) {
        if ty
            .abi
            .as_ref()
            .is_some_and(|a| a.name.as_ref().is_none_or(|n| n.value() != "Rust"))
        {
            self.error("non-Rust function-pointer ABI");
        }
        visit::visit_type_bare_fn(self, ty);
    }
    fn visit_impl_item(&mut self, item: &'ast syn::ImplItem) {
        let attrs: &[Attribute] = match item {
            syn::ImplItem::Const(x) => &x.attrs,
            syn::ImplItem::Fn(x) => &x.attrs,
            syn::ImplItem::Type(x) => &x.attrs,
            syn::ImplItem::Macro(x) => &x.attrs,
            _ => &[],
        };
        if production(attrs) {
            visit::visit_impl_item(self, item);
        }
    }
    fn visit_trait_item(&mut self, item: &'ast syn::TraitItem) {
        let attrs: &[Attribute] = match item {
            syn::TraitItem::Const(x) => &x.attrs,
            syn::TraitItem::Fn(x) => &x.attrs,
            syn::TraitItem::Type(x) => &x.attrs,
            syn::TraitItem::Macro(x) => &x.attrs,
            _ => &[],
        };
        if production(attrs) {
            visit::visit_trait_item(self, item);
        }
    }
    fn visit_expr(&mut self, expr: &'ast Expr) {
        if production(expr_attrs(expr)) {
            visit::visit_expr(self, expr);
        }
    }
    fn visit_block(&mut self, block: &'ast syn::Block) {
        let items = block
            .stmts
            .iter()
            .filter_map(|s| {
                if let Stmt::Item(i) = s {
                    Some(i.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        self.names.push(Names::from_items(&items));
        visit::visit_block(self, block);
        self.names.pop();
    }
    fn visit_local(&mut self, local: &'ast syn::Local) {
        if production(&local.attrs) {
            visit::visit_local(self, local);
        }
    }
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        let mut rows = Vec::new();
        imports(&item.tree, &[], &mut rows);
        for (_, path) in rows {
            let path = self.expand(path);
            if !self.cli
                && (path == ["std", "process"]
                    || (!matches!(item.vis, syn::Visibility::Inherited)
                        && (path == ["std"] || path == ["libc"])))
            {
                // Following arbitrary cross-file reexports is outside this
                // bounded resolver. Refuse these gateway namespaces instead.
                // Explicit Command/ExitStatus and native data imports remain
                // checked normally and do not provide a termination namespace.
                self.error(format!(
                    "unapproved external namespace import {}",
                    path.join("::")
                ));
            }
            self.check_external_path(&path);
        }
    }
    fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
        if !self.cli
            && !matches!(item.vis, syn::Visibility::Inherited)
            && matches!(item.ident.to_string().as_str(), "std" | "libc")
        {
            self.error(format!(
                "unapproved external namespace reexport {}",
                item.ident
            ));
        }
    }
    fn visit_expr_path(&mut self, expr: &'ast syn::ExprPath) {
        self.check_external_path(&self.resolved(&expr.path));
        visit::visit_expr_path(self, expr);
    }
    fn visit_item_macro(&mut self, item: &'ast syn::ItemMacro) {
        if item.mac.path.is_ident("macro_rules") {
            let name = item
                .ident
                .as_ref()
                .map(ToString::to_string)
                .unwrap_or_default();
            let expected = match (self.relative().as_str(), name.as_str()) {
                ("src/engine_options.rs", "options") => Some("macro_rules! options { ($($option:ident, $name:literal, $argument:ident;)*) => { &[$(OptionSpec { option: EngineOption::$option, name: $name, argument: ArgumentRule::$argument, },)*] }; }"),
                ("src/printf.rs", "signed_arg") => Some("macro_rules! signed_arg { ($($ty:ty),* $(,)?) => { $(impl From<$ty> for PrintfArg { fn from(value: $ty) -> Self { Self::Signed(value as i64) } })* }; }"),
                ("src/printf.rs", "unsigned_arg") => Some("macro_rules! unsigned_arg { ($($ty:ty),* $(,)?) => { $(impl From<$ty> for PrintfArg { fn from(value: $ty) -> Self { Self::Unsigned(value as u64) } })* }; }"),
                _ => None,
            };
            if expected.is_none_or(|e| {
                syn::parse_str::<syn::ItemMacro>(e)
                    .unwrap()
                    .mac
                    .tokens
                    .to_string()
                    != item.mac.tokens.to_string()
            }) {
                self.error(format!(
                    "unapproved or changed production macro template {name}"
                ));
            }
        } else {
            self.standard_macro(&item.mac);
        }
    }
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.standard_macro(mac);
    }
}

struct OptionsInput;
impl Parse for OptionsInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        while !input.is_empty() {
            input.parse::<syn::Ident>()?;
            input.parse::<Token![,]>()?;
            input.parse::<syn::LitByteStr>()?;
            input.parse::<Token![,]>()?;
            input.parse::<syn::Ident>()?;
            input.parse::<Token![;]>()?;
        }
        Ok(Self)
    }
}
struct MatchesInput {
    expr: Expr,
    pattern: syn::Pat,
    guard: Option<Expr>,
}
impl Parse for MatchesInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let pattern = syn::Pat::parse_multi_with_leading_vert(input)?;
        let guard = if input.peek(Token![if]) {
            input.parse::<Token![if]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
        Ok(Self {
            expr,
            pattern,
            guard,
        })
    }
}
struct RepeatInput {
    value: Expr,
    count: Expr,
}
impl Parse for RepeatInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let value = input.parse()?;
        input.parse::<Token![;]>()?;
        let count = input.parse()?;
        Ok(Self { value, count })
    }
}

fn audit_graph(root: &Path) -> Report {
    let mut report = Report::default();
    let mut pending = VecDeque::from([
        Source {
            file: root.join("src/lib.rs"),
            module_dir: root.join("src"),
            inline: false,
        },
        Source {
            file: root.join("src/bin/tekai-engine.rs"),
            module_dir: root.join("src/bin"),
            inline: false,
        },
    ]);
    while let Some(source) = pending.pop_front() {
        let canonical = source
            .file
            .canonicalize()
            .unwrap_or_else(|e| panic!("{} cannot resolve module {e}", source.file.display()));
        assert!(
            canonical.starts_with(root),
            "production module escapes crate {}",
            canonical.display()
        );
        if !report.files.insert(canonical.clone()) {
            continue;
        }
        assert!(
            report.files.len() <= MAX_FILES,
            "source inventory file limit exceeded"
        );
        let text = fs::read_to_string(&canonical).unwrap();
        report.bytes += text.len();
        assert!(
            report.bytes <= MAX_SOURCE_BYTES,
            "source inventory byte limit exceeded"
        );
        let file = syn::parse_file(&text)
            .unwrap_or_else(|e| panic!("{} cannot parse {e}", source.file.display()));
        let cli = source.file.starts_with(root.join("src/bin"));
        let mut audit = Audit {
            root,
            source,
            names: vec![Names::from_items(&file.items)],
            report: &mut report,
            pending: &mut pending,
            item: "file".into(),
            cli,
            embedded: false,
        };
        if production(&file.attrs) {
            for item in &file.items {
                audit.visit_item(item);
            }
        }
    }
    report
}

fn embedded_errors(text: &str) -> Vec<String> {
    struct Embedded {
        symbols: BTreeMap<String, String>,
        errors: Vec<String>,
    }
    impl<'ast> Visit<'ast> for Embedded {
        fn visit_item_foreign_mod(&mut self, block: &'ast syn::ItemForeignMod) {
            for item in &block.items {
                match item {
                    syn::ForeignItem::Static(s)
                        if !matches!(s.mutability, syn::StaticMutability::Mut(_)) =>
                    {
                        let ty = if let Type::Path(p) = s.ty.as_ref() {
                            p.path
                                .get_ident()
                                .map(ToString::to_string)
                                .unwrap_or_default()
                        } else {
                            String::new()
                        };
                        if self.symbols.insert(s.ident.to_string(), ty).is_some() {
                            self.errors
                                .push(format!("duplicate embedded symbol {}", s.ident));
                        }
                    }
                    _ => self.errors.push(
                        "embedded data contains a foreign function/type or mutable symbol".into(),
                    ),
                }
            }
        }
        fn visit_signature(&mut self, s: &'ast syn::Signature) {
            if s.abi.is_some() {
                self.errors
                    .push(format!("embedded function ABI {}", s.ident));
            }
            visit::visit_signature(self, s);
        }
        fn visit_item_static(&mut self, s: &'ast syn::ItemStatic) {
            self.errors
                .push(format!("unexpected embedded Rust static {}", s.ident));
        }
    }
    let file = syn::parse_file(text).expect("generated embedded data parses");
    let mut audit = Embedded {
        symbols: BTreeMap::new(),
        errors: Vec::new(),
    };
    audit.visit_file(&file);
    let expected = ["texmf", "font_outlines", "pdflatex_format"]
        .into_iter()
        .flat_map(|n| {
            [
                (n.to_owned(), "u8".to_owned()),
                (format!("{n}_len"), "usize".to_owned()),
            ]
        })
        .collect::<BTreeMap<_, _>>();
    if audit.symbols != expected {
        audit.errors.push("embedded foreign declarations must be exactly three immutable u8 data symbols and three immutable usize lengths".into());
    }
    let mut report = Report::default();
    let mut pending = VecDeque::new();
    let root = Path::new("/fixture");
    let mut normal = Audit {
        root,
        source: Source {
            file: root.join("OUT_DIR/embedded_data.rs"),
            module_dir: root.to_path_buf(),
            inline: false,
        },
        names: vec![Names::from_items(&file.items)],
        report: &mut report,
        pending: &mut pending,
        item: "embedded".into(),
        cli: false,
        embedded: true,
    };
    for item in &file.items {
        normal.visit_item(item);
    }
    audit.errors.extend(report.errors);
    audit.errors
}

#[test]
fn production_ownership_and_rust_interfaces_are_kept() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();
    let report = audit_graph(&root);
    assert!(
        report.files.len() >= 60,
        "production module graph unexpectedly incomplete"
    );
    assert_eq!(
        report.embedded_include, 1,
        "exactly one audited OUT_DIR source include"
    );
    assert_eq!(
        report.tls,
        TLS.iter()
            .map(|(f, n, _)| ((*f).to_owned(), (*n).to_owned()))
            .collect()
    );
    let embedded = fs::read_to_string(Path::new(env!("OUT_DIR")).join("embedded_data.rs")).unwrap();
    let mut errors = report.errors;
    errors.extend(
        embedded_errors(&embedded)
            .into_iter()
            .map(|e| format!("OUT_DIR/embedded_data.rs {e}")),
    );
    assert!(
        errors.is_empty(),
        "architecture regression\n{}",
        errors.join("\n")
    );
}

fn snippet(path: &str, source: &str) -> Vec<String> {
    let file = syn::parse_file(source).unwrap();
    let mut report = Report::default();
    let mut pending = VecDeque::new();
    let root = Path::new("/fixture");
    let mut audit = Audit {
        root,
        source: Source {
            file: root.join(path),
            module_dir: root.join("src"),
            inline: false,
        },
        names: vec![Names::from_items(&file.items)],
        report: &mut report,
        pending: &mut pending,
        item: "fixture".into(),
        cli: path.starts_with("src/bin/"),
        embedded: false,
    };
    for item in &file.items {
        audit.visit_item(item);
    }
    report.errors
}

#[test]
fn cfg_boolean_containers_and_cfg_attr_do_not_hide_production() {
    let accepted = snippet(
        "src/example.rs",
        r#"
        #[cfg(all(test, target_os="macos"))] mod native { static mut X: u8 = 0; extern "C" { fn malloc(); } }
        #[cfg(not(not(test)))] static mut Y: u8 = 0;
        #[cfg_attr(not(test), cfg(test))] static mut Z: u8 = 0;
        struct Thing; impl Thing { #[cfg(all(test, unix))] unsafe extern "C" fn oracle() {} }
        fn run() { #[cfg(test)] static mut LOCAL: u8 = 0; }
    "#,
    );
    assert!(accepted.is_empty(), "{accepted:?}");
    for predicate in ["any(test, unix)", "not(test)"] {
        let errors = snippet(
            "src/example.rs",
            &format!("#[cfg({predicate})] static mut BAD: u8 = 0;"),
        );
        assert!(errors.iter().any(|e| e.contains("BAD")), "{errors:?}");
    }
    assert!(!snippet(
        "src/example.rs",
        "#[cfg_attr(unix, path = \"native.rs\")] mod adapter;"
    )
    .is_empty());
}

#[test]
fn mutable_owners_foreign_interfaces_and_c_callback_types_are_rejected() {
    for source in [
        "static mut BAD: u8 = 0;",
        "static BAD: AtomicUsize = AtomicUsize::new(0);",
        "type Alias = std::sync::Mutex<()>; static BAD: Alias = todo!();",
        "fn run() { static BAD: OnceLock<Vec<u8>> = OnceLock::new(); }",
        "extern \"C\" { fn BAD(); }",
        "extern \"Rust\" { static BAD: u8; }",
        "unsafe extern \"C\" fn BAD() {}",
        "type BAD = Option<unsafe extern \"C-unwind\" fn()>;",
    ] {
        let errors = snippet("src/example.rs", source);
        assert!(!errors.is_empty(), "missed {source}");
        assert!(errors.iter().all(|e| e.contains("src/example.rs")));
    }
    assert!(snippet(
        "src/example.rs",
        "static TABLE: &[u8] = b\"data\"; type Callback = unsafe fn();"
    )
    .is_empty());
}

#[test]
fn macro_bodies_and_renamed_native_or_host_termination_references_are_checked() {
    for source in [
        "use std::process::exit as leave; fn run() { leave(0); }",
        "use std::process as p; fn run() { let stop = p::abort; stop(); }",
        "use std as host; use host::process as p; fn run() { p::exit(0); }",
        "use std::*; fn run() { process::exit(0); }",
        "mod host { pub use std::process as process; } fn run() { host::process::exit(0); }",
        "mod host { pub use std as root; } fn run() { host::root::process::exit(0); }",
        "mod host { pub(crate) use std::{self as root}; } fn run() { host::root::process::exit(0); }",
        "mod host { pub extern crate std as root; } fn run() { host::root::process::exit(0); }",
        "mod host { pub use libc as native; } fn run() { host::native::malloc(4); }",
        "mod host { pub extern crate libc as native; } fn run() { host::native::malloc(4); }",
        "extern crate libc as c; fn run() { c::malloc(4); }",
        "use libc as c; fn run() { c::malloc(4); }",
        "use libc::strcpy as copy;",
        "fn run() { assert!({ static mut BAD: u8 = 0; true }); }",
        "macro_rules! hidden { () => { static mut BAD: u8 = 0; }; }",
    ] {
        assert!(
            !snippet("src/example.rs", source).is_empty(),
            "missed {source}"
        );
    }
    assert!(snippet("src/example.rs", "fn run() { let message = \"libc::malloc and static mut are text\"; println!(\"{message}\"); let text = stringify!(extern \"C\" fn malloc()); }").is_empty());
    assert!(snippet(
        "src/bin/tekai-engine.rs",
        "fn main() { std::process::exit(0); }"
    )
    .is_empty());
    assert!(snippet(
        "src/example.rs",
        "fn run() { unsafe { libc::access(p, libc::R_OK); libc::__error(); } }"
    )
    .is_empty());
    assert!(snippet(
        "src/example.rs",
        "use std::process::{Command, ExitStatus}; #[cfg(panic = \"abort\")] compile_error!(\"unwind required\");"
    )
    .is_empty());
    let group_queries = "fn run() { let _: Option<libc::proc_bsdinfo> = None; unsafe { libc::proc_listpids(2, 1, p, 8); libc::proc_pidinfo(1, 3, 1, p, 136); } }";
    assert!(snippet("src/process_group.rs", group_queries).is_empty());
    assert!(!snippet("src/example.rs", group_queries).is_empty());
    let terminal_mask = "fn run() { let _: Option<libc::sigset_t> = None; unsafe { libc::sigemptyset(p); libc::sigaddset(p, libc::SIGTTOU); libc::pthread_sigmask(libc::SIG_BLOCK, p, old); } }";
    assert!(snippet("src/cli_supervisor.rs", terminal_mask).is_empty());
    assert!(!snippet("src/example.rs", terminal_mask).is_empty());
}

#[test]
fn scoped_tls_is_exactly_nonowning_and_starts_empty() {
    let prefix = "use std::cell::Cell; use std::ptr; struct ArenaState;";
    let valid = format!("{prefix} thread_local! {{ static ACTIVE_ARENA: Cell<*mut ArenaState> = const {{ Cell::new(ptr::null_mut()) }}; }}");
    assert!(snippet("src/allocation.rs", &valid).is_empty());
    for declaration in [
        "static OWNER: Cell<*mut ArenaState> = Cell::new(ptr::null_mut());",
        "static ACTIVE_ARENA: Cell<Box<ArenaState>> = todo!();",
        "static ACTIVE_ARENA: Cell<*mut ArenaState> = Cell::new(ptr::dangling_mut());",
    ] {
        assert!(
            !snippet(
                "src/allocation.rs",
                &format!("{prefix} thread_local! {{ {declaration} }}")
            )
            .is_empty(),
            "missed {declaration}"
        );
    }
}

#[test]
fn embedded_exception_cannot_gain_functions_mutable_or_extra_symbols() {
    let correct = "extern \"C\" { static texmf: u8; static texmf_len: usize; static font_outlines: u8; static font_outlines_len: usize; static pdflatex_format: u8; static pdflatex_format_len: usize; }";
    assert!(embedded_errors(correct).is_empty());
    for suffix in [
        "extern \"C\" { fn utility(); }",
        "extern \"C\" { static mut utility: u8; }",
        "extern \"C\" { static utility: u8; }",
    ] {
        assert!(!embedded_errors(&format!("{correct}{suffix}")).is_empty());
    }
}

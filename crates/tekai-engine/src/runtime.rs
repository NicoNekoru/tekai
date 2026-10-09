//! Bundled TeX data and native file lookup. Shared addition trees are read
//! without subprocesses; distribution data is pinned when the binary is built.

use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub const BUNDLE_ID: &str = include_str!("../../../runtime/bundle-id.txt");
pub const FORMAT_ID: &str = include_str!("../../../formats/format-id.txt");
static TEXMF_ROOT: OnceLock<Result<PathBuf, String>> = OnceLock::new();

pub fn texmf_root() -> io::Result<&'static Path> {
    crate::search::mode()?;
    TEXMF_ROOT
        .get_or_init(|| install_bundle(&cache_root()).map_err(|error| error.to_string()))
        .as_ref()
        .map(|root| root.as_path())
        .map_err(|error| io::Error::other(error.clone()))
}

fn cache_root() -> PathBuf {
    if let Some(path) = std::env::var_os("TEKAI_ENGINE_CACHE") {
        return PathBuf::from(path);
    }
    if cfg!(target_os = "macos") {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home).join("Library/Caches/tekai/engine");
        }
    }
    if let Some(path) = std::env::var_os("XDG_CACHE_HOME") {
        return PathBuf::from(path).join("tekai/engine");
    }
    if let Some(home) = std::env::var_os("HOME") {
        return PathBuf::from(home).join(".cache/tekai/engine");
    }
    std::env::temp_dir().join("tekai/engine")
}

fn install_bundle(cache: &Path) -> io::Result<PathBuf> {
    fs::create_dir_all(cache)?;
    // Absolute paths keep lookups valid after a child changes its working dir.
    let cache = cache.canonicalize()?;
    let destination = cache.join(format!("texmf-{}", BUNDLE_ID.trim()));
    if bundle_is_complete(&destination) {
        return Ok(destination.join("texmf-dist"));
    }
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let staging = cache.join(format!(".texmf-{}-{nonce}", std::process::id()));
    fs::create_dir(&staging)?;
    let result = (|| {
        unpack_bundle(crate::embedded::texmf_bytes(), &staging)?;
        unpack_bundle(crate::embedded::font_outlines_bytes(), &staging)?;
        fs::write(staging.join(".complete"), BUNDLE_ID)?;
        match fs::rename(&staging, &destination) {
            Ok(()) => {}
            Err(_) if bundle_is_complete(&destination) => {
                // A concurrent build finished installing the same bundle.
            }
            Err(error) => return Err(error),
        }
        Ok(destination.join("texmf-dist"))
    })();
    // Only our newly created staging directory is removed, never user data.
    if staging.exists() {
        let _ = fs::remove_dir_all(&staging);
    }
    result
}

fn bundle_is_complete(root: &Path) -> bool {
    fs::read_to_string(root.join(".complete")).is_ok_and(|id| id == BUNDLE_ID)
        && root.join("texmf-dist/ls-R").is_file()
}

fn unpack_bundle(bytes: &[u8], destination: &Path) -> io::Result<()> {
    let decoder = flate2::read::GzDecoder::new(bytes);
    let mut archive = tar::Archive::new(decoder);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if !entry.header().entry_type().is_file()
            || path
                .components()
                .any(|part| !matches!(part, Component::Normal(_)))
        {
            return Err(io::Error::other("unsafe file in embedded TeX bundle"));
        }
        if !entry.unpack_in(destination)? {
            return Err(io::Error::other("invalid path in embedded TeX bundle"));
        }
    }
    // tar stops at its end marker before gzip necessarily checks its footer.
    io::copy(&mut archive.into_inner(), &mut io::sink())?;
    Ok(())
}

pub(crate) fn find_in_path(base: &Path, candidate: &str, entry: &Path) -> Option<PathBuf> {
    crate::lookup::find(base, candidate, entry)
}

pub(crate) fn installed_bundle_root() -> Option<&'static Path> {
    TEXMF_ROOT
        .get()
        .and_then(|result| result.as_ref().ok())
        .map(PathBuf::as_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_paths_support_recursive_search_and_suffixes() {
        let root = std::env::temp_dir().join(format!("tekai-native-search-{}", std::process::id()));
        let child = root.join("tree/deep/tex");
        fs::create_dir_all(&child).unwrap();
        let root = root.canonicalize().unwrap();
        let child = child.canonicalize().unwrap();
        fs::write(child.join("shared.sty"), "test").unwrap();
        assert_eq!(
            find_in_path(&root, "shared.sty", Path::new("tree//tex")),
            Some(child.join("shared.sty"))
        );
        assert_eq!(
            find_in_path(&root, "shared.sty", Path::new("!!tree//")),
            None
        );
        fs::write(root.join("tree/ls-R"), "./deep/tex:\nshared.sty\n").unwrap();
        assert_eq!(
            find_in_path(&root, "shared.sty", Path::new("!!tree//")),
            Some(child.join("shared.sty"))
        );
        assert_eq!(
            find_in_path(&root, "shared.sty", Path::new("!!tree//tex//")),
            Some(child.join("shared.sty"))
        );
        assert_eq!(
            find_in_path(&root, "shared.sty", Path::new("tree//tex//")),
            Some(child.join("shared.sty"))
        );
        assert_eq!(find_in_path(&root, "shared.sty", Path::new("tree")), None);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn corrupt_gzip_footer_is_rejected() {
        use std::io::Write;
        let root =
            std::env::temp_dir().join(format!("tekai-corrupt-bundle-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&[0; 1024]).unwrap();
        let mut bytes = encoder.finish().unwrap();
        let crc_offset = bytes.len() - 8;
        bytes[crc_offset] ^= 1;
        assert!(unpack_bundle(&bytes, &root).is_err());
        fs::remove_dir(root).unwrap();
    }

    #[test]
    fn bundle_contains_packages_fonts_and_notices() {
        let root = texmf_root().unwrap();
        assert!(root.join("tex/latex/base/article.cls").is_file());
        assert!(root.join("fonts/tfm/public/cm/cmr10.tfm").is_file());
        assert!(root.parent().unwrap().join("packages.lock.json").is_file());
        assert!(root.join("source/latex/base").is_dir());
    }
}

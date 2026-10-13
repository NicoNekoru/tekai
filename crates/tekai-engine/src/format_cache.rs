//! Content-addressed, atomically published raw TeX formats.

use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const CACHE_DIRECTORY: &str = "raw-formats-v1";
const BUFFER_SIZE: usize = 64 * 1024;

/// The same open file is hashed and decoded. Replacements cannot redirect the
/// decoder to a different source, and in-place edits invalidate the result.
pub(super) fn materialize_file(path: &Path, cache: &Path) -> io::Result<PathBuf> {
    for _ in 0..3 {
        if !fs::metadata(path)?.is_file() {
            return Err(io::Error::other("format source is not a regular file"));
        }
        let mut source = File::open(path)?;
        let initial = source.metadata()?;
        if !initial.is_file() {
            return Err(io::Error::other("format source is not a regular file"));
        }
        let mut header = [0; 2];
        let count = source.read(&mut header)?;
        if count != 2 || header != [0x1f, 0x8b] {
            return Ok(path.to_path_buf());
        }
        source.seek(SeekFrom::Start(0))?;
        let source_id = digest_reader(&mut source)?;
        if metadata_identity(&initial) != metadata_identity(&source.metadata()?) {
            continue;
        }
        let cache = prepare_cache(cache)?;
        let entry = CacheEntry::new(&cache, &source_id);
        if entry.is_valid()? {
            return Ok(entry.raw);
        }
        source.seek(SeekFrom::Start(0))?;
        let mut input = HashingReader::new(source);
        let mut staging = StagingFile::new(&cache)?;
        let raw_id = decode_raw(&mut input, &mut staging.file)?;
        // GzDecoder may stop before trailing source bytes. Hash those too so
        // the published name always identifies the complete compressed file.
        io::copy(&mut input, &mut io::sink())?;
        let final_identity = metadata_identity(&input.inner.metadata()?);
        if input.digest() != source_id || metadata_identity(&initial) != final_identity {
            continue;
        }
        return entry.publish(staging, &raw_id);
    }
    Err(io::Error::other("format source changed while being read"))
}

/// Optional cache failures may fall back to the original file only after the
/// selected first gzip member is validated through decoder EOF. As with
/// materialization, trailing source bytes are not additional gzip members to
/// validate. Stable descriptor metadata does not prevent a later path change.
/// False means absent, nonregular or unopenable. Once a regular source is open,
/// read/decompression failures are errors, not permission to search elsewhere.
pub(super) fn validate_file(path: &Path) -> io::Result<bool> {
    for _ in 0..3 {
        if !fs::metadata(path).is_ok_and(|metadata| metadata.is_file()) {
            return Ok(false);
        }
        let mut source = match File::open(path) {
            Ok(source) => source,
            Err(_) => return Ok(false),
        };
        let initial = source.metadata()?;
        if !initial.is_file() {
            return Ok(false);
        }
        let mut header = [0; 2];
        let count = source.read(&mut header)?;
        if count == 2 && header == [0x1f, 0x8b] {
            source.seek(SeekFrom::Start(0))?;
            decode_raw(&mut source, &mut io::sink())?;
        }
        if metadata_identity(&initial) == metadata_identity(&source.metadata()?) {
            return Ok(true);
        }
    }
    Err(io::Error::other("format source changed while being read"))
}

/// Embedded bytes are immutable and their build-time SHA-256 is already known.
pub(super) fn materialize_embedded(
    bytes: &[u8],
    source_id: &str,
    cache: &Path,
) -> io::Result<PathBuf> {
    if !is_digest(source_id) {
        return Err(io::Error::other("invalid embedded format identifier"));
    }
    let cache = prepare_cache(cache)?;
    let entry = CacheEntry::new(&cache, source_id);
    if entry.is_valid()? {
        return Ok(entry.raw);
    }
    let mut staging = StagingFile::new(&cache)?;
    let raw_id = decode_raw(bytes, &mut staging.file)?;
    entry.publish(staging, &raw_id)
}

fn prepare_cache(root: &Path) -> io::Result<PathBuf> {
    let cache = root.join(CACHE_DIRECTORY);
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&cache)?;
    if !fs::symlink_metadata(&cache)?.file_type().is_dir() {
        return Err(io::Error::other(
            "raw format cache must be a real directory",
        ));
    }
    // Engine children can change directory after finding the format.
    cache.canonicalize()
}

struct CacheEntry {
    raw: PathBuf,
    receipt: PathBuf,
    source_id: String,
}

impl CacheEntry {
    fn new(cache: &Path, source_id: &str) -> Self {
        Self {
            raw: cache.join(format!("{source_id}.fmt.raw")),
            receipt: cache.join(format!("{source_id}.meta")),
            source_id: source_id.to_owned(),
        }
    }

    fn is_valid(&self) -> io::Result<bool> {
        // Symlinks and non-files are never cache hits. Atomic publication
        // replaces a stale symlink without following its target.
        let Some(raw_metadata) = regular_file_metadata(&self.raw)? else {
            return Ok(false);
        };
        if regular_file_metadata(&self.receipt)?.is_none() {
            return Ok(false);
        }
        let mut receipt = String::new();
        if File::open(&self.receipt)?
            .take(1025)
            .read_to_string(&mut receipt)
            .is_err()
        {
            return Ok(false);
        }
        if receipt.len() > 1024 {
            return Ok(false);
        }
        let mut lines = receipt.lines();
        if lines.next() != Some("tekai-format-v1") || lines.next() != Some(self.source_id.as_str())
        {
            return Ok(false);
        }
        let Some(raw_id) = lines.next().filter(|id| is_digest(id)) else {
            return Ok(false);
        };
        let Some(identity) = lines.next() else {
            return Ok(false);
        };
        if lines.next().is_some() {
            return Ok(false);
        }
        // A trusted receipt avoids rereading the multi-megabyte embedded raw
        // format on each process launch. ctime also catches preserved-mtime
        // edits. Metadata changes require checking the actual raw digest.
        if crate::file_identity::change_identity(&raw_metadata).is_some()
            && identity == metadata_identity(&raw_metadata)
        {
            return Ok(true);
        }
        let mut raw = File::open(&self.raw)?;
        let initial = raw.metadata()?;
        if digest_reader(&mut raw)? != raw_id
            || metadata_identity(&initial) != metadata_identity(&raw.metadata()?)
        {
            return Ok(false);
        }
        self.write_receipt(raw_id, &initial)?;
        Ok(true)
    }

    fn publish(&self, mut staging: StagingFile, raw_id: &str) -> io::Result<PathBuf> {
        staging.file.flush()?;
        staging.file.sync_all()?;
        fs::rename(&staging.path, &self.raw)?;
        // A raw file without its matching receipt is an incomplete entry and
        // is regenerated. Concurrent writers only publish the same contents.
        // Read metadata from our descriptor, not the shared destination path.
        // Another writer may already have replaced that path with its own
        // complete entry. A stale inode ticket triggers validation on reuse.
        self.write_receipt(raw_id, &staging.file.metadata()?)?;
        Ok(self.raw.clone())
    }

    fn write_receipt(&self, raw_id: &str, metadata: &fs::Metadata) -> io::Result<()> {
        let mut staging = StagingFile::new(self.raw.parent().unwrap())?;
        writeln!(
            staging.file,
            "tekai-format-v1\n{}\n{raw_id}\n{}",
            self.source_id,
            metadata_identity(metadata)
        )?;
        staging.file.flush()?;
        staging.file.sync_all()?;
        fs::rename(&staging.path, &self.receipt)
    }
}

fn regular_file_metadata(path: &Path) -> io::Result<Option<fs::Metadata>> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => Ok(metadata.is_file().then_some(metadata)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn metadata_identity(metadata: &fs::Metadata) -> String {
    format!(
        "{} {:?} {:?}",
        metadata.len(),
        metadata.modified().ok(),
        crate::file_identity::change_identity(metadata)
    )
}

fn is_digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn hex_digest(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        hex.push(HEX[(byte >> 4) as usize] as char);
        hex.push(HEX[(byte & 0x0f) as usize] as char);
    }
    hex
}

fn digest_reader(reader: &mut impl Read) -> io::Result<String> {
    let mut hasher = Sha256::new();
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            return Ok(hex_digest(&hasher.finalize()));
        }
        hasher.update(&buffer[..count]);
    }
}

fn decode_raw(input: impl Read, output: &mut impl Write) -> io::Result<String> {
    let mut decoder = flate2::read::GzDecoder::new(input);
    let mut hasher = Sha256::new();
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        let count = decoder.read(&mut buffer)?;
        if count == 0 {
            return Ok(hex_digest(&hasher.finalize()));
        }
        hasher.update(&buffer[..count]);
        output.write_all(&buffer[..count])?;
    }
}

struct HashingReader<R> {
    inner: R,
    hasher: Sha256,
}

impl<R> HashingReader<R> {
    fn new(inner: R) -> Self {
        Self {
            inner,
            hasher: Sha256::new(),
        }
    }

    fn digest(self) -> String {
        hex_digest(&self.hasher.finalize())
    }
}

impl<R: Read> Read for HashingReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let count = self.inner.read(buffer)?;
        self.hasher.update(&buffer[..count]);
        Ok(count)
    }
}

struct StagingFile {
    path: PathBuf,
    file: File,
}

impl StagingFile {
    fn new(cache: &Path) -> io::Result<Self> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        Self::with_nonce(cache, nonce)
    }

    fn with_nonce(cache: &Path, nonce: u128) -> io::Result<Self> {
        for attempt in 0..100 {
            let path = cache.join(format!(
                ".format-{}-{nonce}-{attempt}.tmp",
                std::process::id()
            ));
            // Names only reduce collisions. Exclusive creation remains the
            // authority, including across concurrent independent invocations.
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&path)
            {
                Ok(file) => return Ok(Self { path, file }),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error),
            }
        }
        Err(io::Error::other("cannot allocate a format staging file"))
    }
}

impl Drop for StagingFile {
    fn drop(&mut self) {
        // The path was exclusively created by this operation.
        let _ = fs::remove_file(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::{write::GzEncoder, Compression};
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Barrier};

    static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "tekai-format-cache-test-{}-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos(),
                TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&root).unwrap();
            Self(root)
        }

        fn source(&self) -> PathBuf {
            self.0.join("active.fmt")
        }

        fn cache(&self) -> PathBuf {
            self.0.join("cache")
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn staging_collisions_never_overwrite_or_remove_foreign_paths() {
        let fixture = TestDirectory::new();
        let occupied = |attempt| {
            fixture
                .0
                .join(format!(".format-{}-0-{attempt}.tmp", std::process::id()))
        };
        let sentinel = occupied(0);
        fs::write(&sentinel, b"foreign contents").unwrap();
        let staging = StagingFile::with_nonce(&fixture.0, 0).unwrap();
        assert_eq!(staging.path, occupied(1));
        drop(staging);
        assert_eq!(fs::read(&sentinel).unwrap(), b"foreign contents");
        assert!(!occupied(1).exists());
        for attempt in 1..100 {
            fs::write(occupied(attempt), b"foreign contents").unwrap();
        }
        assert!(StagingFile::with_nonce(&fixture.0, 0).is_err());
        assert_eq!(fs::read_dir(&fixture.0).unwrap().count(), 100);
        for attempt in 0..100 {
            assert_eq!(fs::read(occupied(attempt)).unwrap(), b"foreign contents");
        }
    }

    #[test]
    fn concurrent_same_nonce_staging_files_have_separate_exclusive_owners() {
        let fixture = TestDirectory::new();
        let barrier = Arc::new(Barrier::new(8));
        let workers = (0..8)
            .map(|_| {
                let directory = fixture.0.clone();
                let barrier = barrier.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    let staging = StagingFile::with_nonce(&directory, 0).unwrap();
                    let path = staging.path.clone();
                    barrier.wait();
                    assert!(path.is_file());
                    drop(staging);
                    path
                })
            })
            .collect::<Vec<_>>();
        let paths = workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<std::collections::HashSet<_>>();
        assert_eq!(paths.len(), 8);
        assert_eq!(fs::read_dir(&fixture.0).unwrap().count(), 0);
    }

    fn gzip(bytes: &[u8]) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(bytes).unwrap();
        encoder.finish().unwrap()
    }

    fn preserve_times(file: &Path, metadata: &fs::Metadata) {
        let times = fs::FileTimes::new()
            .set_accessed(metadata.accessed().unwrap())
            .set_modified(metadata.modified().unwrap());
        File::options()
            .write(true)
            .open(file)
            .unwrap()
            .set_times(times)
            .unwrap();
    }

    fn assert_contents(path: &Path, bytes: &[u8]) {
        assert_eq!(fs::read(path).unwrap(), bytes);
    }

    #[test]
    fn compressed_source_ignores_adjacent_raw_and_uses_owned_cache() {
        let test = TestDirectory::new();
        let installation = test.0.join("installation");
        fs::create_dir(&installation).unwrap();
        let source = installation.join("active.fmt");
        let companion = installation.join("active.fmt.raw");
        fs::write(&source, gzip(b"current-format")).unwrap();
        fs::write(&companion, b"stale-format").unwrap();
        fs::set_permissions(&installation, fs::Permissions::from_mode(0o555)).unwrap();
        let result = materialize_file(&source, &test.cache());
        fs::set_permissions(&installation, fs::Permissions::from_mode(0o755)).unwrap();
        let raw = result.unwrap();
        assert!(raw.starts_with(test.cache().join(CACHE_DIRECTORY).canonicalize().unwrap()));
        assert_contents(&raw, b"current-format");
        assert_contents(&companion, b"stale-format");
        assert_eq!(fs::read_dir(&installation).unwrap().count(), 2);
    }

    #[test]
    fn uncompressed_format_never_prefers_adjacent_raw() {
        let test = TestDirectory::new();
        fs::write(test.source(), b"current-uncompressed-format").unwrap();
        fs::write(test.0.join("active.fmt.raw"), b"stale-format").unwrap();
        assert_eq!(
            materialize_file(&test.source(), &test.cache()).unwrap(),
            test.source()
        );
        assert!(!test.cache().exists());
    }

    #[test]
    fn format_content_replacement_with_preserved_mtime_changes_cache_key() {
        for atomic in [false, true] {
            let test = TestDirectory::new();
            let old = gzip(b"OLD-FORMAT");
            let new = gzip(b"NEW-FORMAT");
            assert_eq!(old.len(), new.len());
            fs::write(test.source(), old).unwrap();
            let initial = fs::metadata(test.source()).unwrap();
            let first = materialize_file(&test.source(), &test.cache()).unwrap();
            let destination = if atomic {
                test.0.join("replacement.fmt")
            } else {
                test.source()
            };
            fs::write(&destination, new).unwrap();
            preserve_times(&destination, &initial);
            if atomic {
                fs::rename(&destination, test.source()).unwrap();
            }
            assert_eq!(
                fs::metadata(test.source()).unwrap().modified().unwrap(),
                initial.modified().unwrap()
            );
            let second = materialize_file(&test.source(), &test.cache()).unwrap();
            assert_ne!(first, second);
            assert_contents(&first, b"OLD-FORMAT");
            assert_contents(&second, b"NEW-FORMAT");
        }
    }

    #[test]
    fn identical_compressed_contents_reuse_raw_across_source_paths() {
        let test = TestDirectory::new();
        let bytes = gzip(b"same-format");
        fs::write(test.source(), &bytes).unwrap();
        let first = materialize_file(&test.source(), &test.cache()).unwrap();
        let identity = crate::file_identity::change_identity(&fs::metadata(&first).unwrap());
        let other = test.0.join("other.fmt");
        fs::write(&other, bytes).unwrap();
        assert_eq!(materialize_file(&other, &test.cache()).unwrap(), first);
        assert_eq!(
            crate::file_identity::change_identity(&fs::metadata(&first).unwrap()),
            identity
        );
    }

    #[test]
    fn corrupt_raw_entry_with_preserved_mtime_is_rebuilt() {
        let test = TestDirectory::new();
        fs::write(test.source(), gzip(b"correct-format")).unwrap();
        let raw = materialize_file(&test.source(), &test.cache()).unwrap();
        let metadata = fs::metadata(&raw).unwrap();
        fs::write(&raw, b"corrupt-format").unwrap();
        preserve_times(&raw, &metadata);
        assert_eq!(fs::metadata(&raw).unwrap().len(), metadata.len());
        assert_eq!(
            materialize_file(&test.source(), &test.cache()).unwrap(),
            raw
        );
        assert_contents(&raw, b"correct-format");
    }

    #[test]
    fn interrupted_and_corrupt_receipts_are_rebuilt() {
        let test = TestDirectory::new();
        fs::write(test.source(), gzip(b"correct-format")).unwrap();
        let raw = materialize_file(&test.source(), &test.cache()).unwrap();
        // The receipt filename uses the source digest, without fmt.raw.
        let source_id = raw.file_name().unwrap().to_str().unwrap()[..64].to_owned();
        let receipt = raw.with_file_name(format!("{source_id}.meta"));
        fs::remove_file(&receipt).unwrap();
        fs::write(&raw, b"interrupted-write").unwrap();
        let abandoned = raw.parent().unwrap().join(".format-abandoned.tmp");
        fs::write(&abandoned, b"unpublished").unwrap();
        assert_eq!(
            materialize_file(&test.source(), &test.cache()).unwrap(),
            raw
        );
        assert_contents(&raw, b"correct-format");
        assert_contents(&abandoned, b"unpublished");
        fs::write(&receipt, [0xff, 0xfe]).unwrap();
        fs::write(&raw, b"corrupt-receipt-write").unwrap();
        assert_eq!(
            materialize_file(&test.source(), &test.cache()).unwrap(),
            raw
        );
        assert_contents(&raw, b"correct-format");
    }

    #[test]
    fn concurrent_materializations_publish_complete_data_without_shared_staging() {
        let test = TestDirectory::new();
        let expected = vec![b'f'; BUFFER_SIZE * 3];
        fs::write(test.source(), gzip(&expected)).unwrap();
        let source = test.source();
        let cache = test.cache();
        let barrier = Arc::new(Barrier::new(8));
        let handles = (0..8)
            .map(|_| {
                let source = source.clone();
                let cache = cache.clone();
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    barrier.wait();
                    materialize_file(&source, &cache).unwrap()
                })
            })
            .collect::<Vec<_>>();
        let paths = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>();
        for path in &paths {
            assert_eq!(path, &paths[0]);
            assert_contents(path, &expected);
        }
        assert_eq!(
            fs::read_dir(cache.join(CACHE_DIRECTORY)).unwrap().count(),
            2
        );
        assert_eq!(materialize_file(&source, &cache).unwrap(), paths[0]);
    }

    #[test]
    fn corrupt_gzip_never_publishes_raw_or_receipt() {
        let test = TestDirectory::new();
        let mut bytes = gzip(b"format-content");
        let crc_offset = bytes.len() - 8;
        bytes[crc_offset] ^= 1;
        fs::write(test.source(), bytes).unwrap();
        assert!(materialize_file(&test.source(), &test.cache()).is_err());
        assert_eq!(
            fs::read_dir(test.cache().join(CACHE_DIRECTORY))
                .unwrap()
                .count(),
            0
        );
    }

    #[test]
    fn fallback_validation_checks_first_member_footer_without_a_cache() {
        let test = TestDirectory::new();
        let compressed = gzip(b"format-content");
        fs::write(test.source(), &compressed).unwrap();
        assert!(validate_file(&test.source()).unwrap());
        assert!(!test.cache().exists());

        let mut trailing = compressed.clone();
        trailing.extend_from_slice(&gzip(b"another member is not selected"));
        fs::write(test.source(), trailing).unwrap();
        assert!(validate_file(&test.source()).unwrap());

        for offset in [8, 4] {
            let mut corrupt = compressed.clone();
            let index = corrupt.len() - offset;
            corrupt[index] ^= 1;
            fs::write(test.source(), corrupt).unwrap();
            assert!(validate_file(&test.source()).is_err());
        }
        let mut truncated = compressed;
        truncated.truncate(truncated.len() - 3);
        fs::write(test.source(), truncated).unwrap();
        assert!(validate_file(&test.source()).is_err());

        fs::write(test.source(), b"uncompressed-format-content").unwrap();
        assert!(validate_file(&test.source()).unwrap());
        assert!(!test.cache().exists());
        assert!(!validate_file(&test.0.join("missing.fmt")).unwrap());
        assert!(!validate_file(&test.0).unwrap());
    }

    #[test]
    fn embedded_entry_is_checked_and_recovers_from_corruption() {
        let test = TestDirectory::new();
        let bytes = gzip(b"embedded-format-content");
        let source_id = hex_digest(&Sha256::digest(&bytes));
        let raw = materialize_embedded(&bytes, &source_id, &test.cache()).unwrap();
        let identity = crate::file_identity::change_identity(&fs::metadata(&raw).unwrap());
        assert_eq!(
            materialize_embedded(&bytes, &source_id, &test.cache()).unwrap(),
            raw
        );
        assert_eq!(
            crate::file_identity::change_identity(&fs::metadata(&raw).unwrap()),
            identity
        );
        fs::write(&raw, b"corrupt").unwrap();
        assert_eq!(
            materialize_embedded(&bytes, &source_id, &test.cache()).unwrap(),
            raw
        );
        assert_contents(&raw, b"embedded-format-content");
    }

    #[test]
    fn cache_symlinks_are_replaced_without_writing_their_targets() {
        use std::os::unix::fs::symlink;
        let test = TestDirectory::new();
        let bytes = gzip(b"correct-format");
        fs::write(test.source(), &bytes).unwrap();
        let cache = prepare_cache(&test.cache()).unwrap();
        let source_id = hex_digest(&Sha256::digest(&bytes));
        let entry = CacheEntry::new(&cache, &source_id);
        let target = test.0.join("outside-cache");
        fs::write(&target, b"untouched").unwrap();
        symlink(&target, &entry.raw).unwrap();
        symlink(&target, &entry.receipt).unwrap();
        let raw = materialize_file(&test.source(), &test.cache()).unwrap();
        assert_contents(&raw, b"correct-format");
        assert_contents(&target, b"untouched");
        assert!(fs::symlink_metadata(raw).unwrap().is_file());
        assert!(fs::symlink_metadata(entry.receipt).unwrap().is_file());
    }

    #[test]
    fn cache_directory_symlink_never_redirects_format_publication() {
        use std::os::unix::fs::symlink;
        let test = TestDirectory::new();
        fs::write(test.source(), gzip(b"correct-format")).unwrap();
        fs::create_dir(test.cache()).unwrap();
        let outside = test.0.join("installation");
        fs::create_dir(&outside).unwrap();
        symlink(&outside, test.cache().join(CACHE_DIRECTORY)).unwrap();
        assert!(materialize_file(&test.source(), &test.cache()).is_err());
        assert_eq!(fs::read_dir(outside).unwrap().count(), 0);
    }

    proptest::proptest! {
        #[test]
        fn arbitrary_format_replacements_and_corruption_never_reuse_stale_bytes(
            first in proptest::collection::vec(proptest::prelude::any::<u8>(), 0..8192),
            second in proptest::collection::vec(proptest::prelude::any::<u8>(), 0..8192),
        ) {
            let test = TestDirectory::new();
            fs::write(test.source(), gzip(&first)).unwrap();
            let old_metadata = fs::metadata(test.source()).unwrap();
            let old_raw = materialize_file(&test.source(), &test.cache()).unwrap();
            proptest::prop_assert_eq!(fs::read(old_raw).unwrap(), first);
            fs::write(test.source(), gzip(&second)).unwrap();
            preserve_times(&test.source(), &old_metadata);
            let raw = materialize_file(&test.source(), &test.cache()).unwrap();
            let current_bytes = fs::read(&raw).unwrap();
            proptest::prop_assert_eq!(current_bytes.as_slice(), second.as_slice());
            let old_raw_metadata = fs::metadata(&raw).unwrap();
            fs::write(&raw, vec![0; second.len()]).unwrap();
            preserve_times(&raw, &old_raw_metadata);
            let recovered = materialize_file(&test.source(), &test.cache()).unwrap();
            proptest::prop_assert_eq!(fs::read(recovered).unwrap(), second);
        }
    }
}

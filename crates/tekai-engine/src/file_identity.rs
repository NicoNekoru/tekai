//! The change identity shared by filesystem lookup and build-cache checks.

/// Device, inode, and change time distinguish atomic replacements and edits
/// that preserve modification time. Size and mtime are checked separately.
pub type ChangeIdentity = (u64, u64, i64, i64);

/// `None` means metadata alone cannot establish that the contents are unchanged.
pub fn change_identity(metadata: &std::fs::Metadata) -> Option<ChangeIdentity> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        Some((
            metadata.dev(),
            metadata.ino(),
            metadata.ctime(),
            metadata.ctime_nsec(),
        ))
    }
    #[cfg(not(unix))]
    {
        let _ = metadata;
        None
    }
}

use crate::unix::{getegid, geteuid};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

pub fn can_delete(path: &Path) -> io::Result<bool> {
    let meta = match path.parent().unwrap().symlink_metadata() {
        Ok(meta) => meta,
        Err(err) => match err.kind() {
            io::ErrorKind::PermissionDenied | io::ErrorKind::ReadOnlyFilesystem => {
                return Ok(false);
            }
            _ => return Err(err),
        },
    };

    let mode = meta.mode();

    Ok(if geteuid() == meta.uid() {
        (mode & 0o0200) != 0
    } else if getegid() == meta.gid() {
        (mode & 0o0020) != 0
    } else {
        (mode & 0o0002) != 0
    })
}

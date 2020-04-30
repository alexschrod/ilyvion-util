//! Useful file utilities

use std::path::Path;

/// Returns the final component of the `Path`, if there is one.
///
/// If the path is a normal file, this is the file name. If it's the path of a directory, this
/// is the directory name.
///
/// Returns [`None`] if the path terminates in `..`.
///
/// Converts from `OsStr` to `String` where any non-Unicode sequences are replaced with
/// `U+FFFD REPLACEMENT CHARACTER`.
pub fn file_name_from_file_path<P: AsRef<Path>>(path: P) -> Option<String> {
    let path = path.as_ref();
    path.file_name().map(|s| s.to_string_lossy().into_owned())
}

use std::path::Path;

/// Filesystem capability boundary used by application/domain services.
///
/// Intent-focused contract:
/// - read text content
/// - write text content
/// - check path existence
/// - create directories required by workflows
#[allow(dead_code)]
pub trait FilesystemPort {
    type Error;

    fn read_text(&self, path: &Path) -> Result<String, Self::Error>;
    fn write_text(&self, path: &Path, content: &str) -> Result<(), Self::Error>;
    fn exists(&self, path: &Path) -> bool;
    fn create_dir_all(&self, path: &Path) -> Result<(), Self::Error>;
}

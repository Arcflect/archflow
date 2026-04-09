use std::path::Path;

pub struct LocalFilesystemAdapter;

impl crate::ports::FilesystemPort for LocalFilesystemAdapter {
    type Error = std::io::Error;

    fn read_text(&self, path: &Path) -> Result<String, Self::Error> {
        std::fs::read_to_string(path)
    }

    fn write_text(&self, path: &Path, content: &str) -> Result<(), Self::Error> {
        std::fs::write(path, content)
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn create_dir_all(&self, path: &Path) -> Result<(), Self::Error> {
        std::fs::create_dir_all(path)
    }
}

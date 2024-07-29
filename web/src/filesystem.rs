use ruffle_core::backend::filesystem::{File, FileOpenMode, FileSystemBackend, KnownDirectories};
use std::io::{Cursor, Read, Result, Seek, Write};
use std::path::{Path, PathBuf};

pub struct NullFile(Cursor<Vec<u8>>, PathBuf);

impl File for NullFile {
    fn truncate(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Seek for NullFile {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> Result<u64> {
        Ok(0)
    }
}

impl Read for NullFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}

impl Write for NullFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        tracing::info!(
            "write {} {}",
            self.1.display(),
            String::from_utf8_lossy(buf)
        );
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct VirtualFileSystemStorageBackend(KnownDirectories);

impl VirtualFileSystemStorageBackend {
    pub fn new() -> Self {
        Self(KnownDirectories {
            app_storage: "/app-storage/Local Storage".into(),
            app: "/app".into(),
            documents: "".into(),
            desktop: "".into(),
            user: "".into(),
            temp: "".into(),
            trash: None,
        })
    }
}

impl FileSystemBackend for VirtualFileSystemStorageBackend {
    fn known_directories(&self) -> &KnownDirectories {
        &self.0
    }

    fn exists(&self, _path: &Path) -> bool {
        tracing::info!("exists: {}", _path.display());
        false
    }

    fn size(&self, path: &Path) -> u64 {
        tracing::info!("size: {}", path.display());
        0
    }

    fn is_hidden(&self, _path: &Path) -> bool {
        false
    }

    fn is_directory(&self, _path: &Path) -> bool {
        false
    }

    fn available_space(&self, _path: &Path) -> u64 {
        0
    }

    fn copy(&mut self, _source: &Path, _destination: &Path, _overwrite: bool) -> Result<()> {
        tracing::info!("copy: {} -> {}", _source.display(), _destination.display());
        Ok(())
    }

    fn rename(&mut self, _source: &Path, _destination: &Path, _overwrite: bool) -> Result<()> {
        tracing::info!("mov: {} -> {}", _source.display(), _destination.display());
        Ok(())
    }

    fn create_directory(&mut self, _path: &Path) -> Result<()> {
        tracing::info!("create_directory: {}", _path.display());
        Ok(())
    }

    fn read_directory(&self, _path: &Path) -> Result<Vec<PathBuf>> {
        tracing::info!("read_directory: {}", _path.display());
        Ok(vec![])
    }

    fn delete_directory(&mut self, _path: &Path, _delete_contents: bool) -> Result<()> {
        tracing::info!("delete_directory: {}", _path.display());
        Ok(())
    }

    fn delete_file(&mut self, _path: &Path) -> Result<()> {
        tracing::info!("delete_file: {}", _path.display());
        Ok(())
    }

    fn open(&mut self, path: &Path, _mode: FileOpenMode) -> Result<Box<dyn File>> {
        tracing::info!("open: {}", path.display());
        Ok(Box::new(NullFile(Cursor::default(), path.into())))
    }
}

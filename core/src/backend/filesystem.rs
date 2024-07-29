use std::io::{Read, Result, Seek, Write};
use std::path::{Path, PathBuf};

pub trait File: Read + Write + Seek {
    fn truncate(&mut self) -> Result<()>;
}

pub enum FileOpenMode {
    Read,
    Write,
    Append,
    Update,
}

impl From<&str> for FileOpenMode {
    fn from(s: &str) -> Self {
        match s {
            "write" => FileOpenMode::Write,
            "append" => FileOpenMode::Append,
            "update" => FileOpenMode::Update,
            // fallback to read
            _ => FileOpenMode::Read,
        }
    }
}

pub struct KnownDirectories {
    pub app_storage: PathBuf,
    pub app: PathBuf,
    pub documents: PathBuf,
    pub desktop: PathBuf,
    pub user: PathBuf,
    pub temp: PathBuf,
    pub trash: Option<PathBuf>,
}

pub trait FileSystemBackend {
    fn known_directories(&self) -> &KnownDirectories;

    fn exists(&self, path: &Path) -> bool;
    fn is_hidden(&self, path: &Path) -> bool;
    fn is_directory(&self, path: &Path) -> bool;
    fn size(&self, path: &Path) -> u64;

    fn available_space(&self, path: &Path) -> u64;

    fn copy(&mut self, source: &Path, destination: &Path, overwrite: bool) -> Result<()>;
    fn rename(&mut self, source: &Path, destination: &Path, overwrite: bool) -> Result<()>;
    fn create_directory(&mut self, path: &Path) -> Result<()>;
    fn read_directory(&self, path: &Path) -> Result<Vec<PathBuf>>;
    fn delete_directory(&mut self, path: &Path, delete_contents: bool) -> Result<()>;
    fn delete_file(&mut self, path: &Path) -> Result<()>;
    fn open(&mut self, path: &Path, mode: FileOpenMode) -> Result<Box<dyn File>>;
}

pub struct NullFile;

impl File for NullFile {
    fn truncate(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Read for NullFile {
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }
}

impl Seek for NullFile {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> Result<u64> {
        Ok(0)
    }
}

impl Write for NullFile {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct NullFileSystemBackend(KnownDirectories);

impl NullFileSystemBackend {
    pub fn new() -> Self {
        Self(KnownDirectories {
            app_storage: PathBuf::from(""),
            app: PathBuf::from(""),
            documents: PathBuf::from(""),
            desktop: PathBuf::from(""),
            user: PathBuf::from(""),
            temp: PathBuf::from(""),
            trash: None,
        })
    }
}

impl FileSystemBackend for NullFileSystemBackend {
    fn known_directories(&self) -> &KnownDirectories {
        &self.0
    }

    fn exists(&self, _path: &Path) -> bool {
        false
    }

    fn is_hidden(&self, _path: &Path) -> bool {
        false
    }

    fn is_directory(&self, _path: &Path) -> bool {
        false
    }

    fn size(&self, _path: &Path) -> u64 {
        0
    }

    fn available_space(&self, _path: &Path) -> u64 {
        0
    }

    fn copy(&mut self, _source: &Path, _destination: &Path, _overwrite: bool) -> Result<()> {
        Ok(())
    }

    fn rename(&mut self, _source: &Path, _destination: &Path, _overwrite: bool) -> Result<()> {
        Ok(())
    }

    fn create_directory(&mut self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn read_directory(&self, _path: &Path) -> Result<Vec<PathBuf>> {
        Ok(vec![])
    }

    fn delete_directory(&mut self, _path: &Path, _delete_contents: bool) -> Result<()> {
        Ok(())
    }

    fn delete_file(&mut self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn open(&mut self, _path: &Path, _mode: FileOpenMode) -> Result<Box<dyn File>> {
        Ok(Box::new(NullFile))
    }
}

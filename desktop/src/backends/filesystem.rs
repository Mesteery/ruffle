use directories::{ProjectDirs, UserDirs};
use ruffle_core::backend::filesystem::{File, FileOpenMode, FileSystemBackend, KnownDirectories};
use std::io::{Read, Result, Seek, Write};
use std::path::{Path, PathBuf};

pub struct NullFile(std::fs::File);

impl File for NullFile {
    fn truncate(&mut self) -> Result<()> {
        let pos = self.0.stream_position()?;
        self.0.set_len(pos).and(Ok(()))
    }
}

impl Drop for NullFile {
    fn drop(&mut self) {
        tracing::info!("filesystem: closing file");
    }
}

impl Seek for NullFile {
    fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64> {
        self.0.seek(pos)
    }
}

impl Read for NullFile {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}

impl Write for NullFile {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}

pub struct OsFileSystemBackend(KnownDirectories);

impl OsFileSystemBackend {
    pub fn new(base_url: PathBuf) -> Self {
        let user_dirs = UserDirs::new().unwrap();
        Self(KnownDirectories {
            app: base_url,
            // TODO use ProjectDirs
            app_storage: ProjectDirs::from("TODO", "TODO", "TODO")
                .unwrap()
                .data_dir()
                .into(),
            documents: user_dirs.document_dir().unwrap().into(),
            desktop: user_dirs.desktop_dir().unwrap().into(),
            user: user_dirs.home_dir().into(),
            temp: std::env::temp_dir(),
            // TODO
            trash: None,
        })
    }
}

impl FileSystemBackend for OsFileSystemBackend {
    fn known_directories(&self) -> &KnownDirectories {
        &self.0
    }

    fn exists(&self, path: &Path) -> bool {
        tracing::info!("filesystem: exists {:?}", path);
        path.exists()
    }

    fn is_hidden(&self, path: &Path) -> bool {
        tracing::warn!("filesystem: stubbed is_hidden has been called");
        path.file_name()
            .map(|n| n.to_string_lossy().starts_with('.'))
            .unwrap_or(false)
    }

    fn is_directory(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn size(&self, path: &Path) -> u64 {
        tracing::info!("filesystem: size {:?}", path);
        path.metadata().map(|m| m.len()).unwrap_or(0)
    }

    fn available_space(&self, _path: &Path) -> u64 {
        tracing::warn!("filesystem: stubbed available_space has been called");
        u64::MAX
    }

    fn copy(&mut self, source: &Path, destination: &Path, _overwrite: bool) -> Result<()> {
        tracing::info!("filesystem: copy {:?} -> {:?}", source, destination);
        std::fs::copy(source, destination).and(Ok(()))
    }

    fn rename(&mut self, source: &Path, destination: &Path, _overwrite: bool) -> Result<()> {
        tracing::info!("filesystem: rename {:?} -> {:?}", source, destination);
        std::fs::rename(source, destination).and(Ok(()))
    }

    fn create_directory(&mut self, path: &Path) -> Result<()> {
        tracing::info!("filesystem: create_directory {:?}", path);
        std::fs::create_dir_all(path).and(Ok(()))
    }

    fn read_directory(&self, path: &Path) -> Result<Vec<PathBuf>> {
        tracing::info!("filesystem: read_directory {:?}", path);
        Ok(path
            .read_dir()?
            .filter_map(|entry| Some(entry.ok()?.path()))
            .collect())
    }

    fn delete_directory(&mut self, path: &Path, delete_contents: bool) -> Result<()> {
        tracing::info!(
            "filesystem: delete_directory {:?} delete_contents={}",
            path,
            delete_contents
        );
        if delete_contents {
            std::fs::remove_dir_all(path)?;
        } else {
            std::fs::remove_dir(path)?;
        }
        Ok(())
    }

    fn delete_file(&mut self, path: &Path) -> Result<()> {
        tracing::info!("filesystem: delete_file {:?}", path);
        std::fs::remove_file(path).and(Ok(()))
    }

    fn open(&mut self, path: &Path, mode: FileOpenMode) -> Result<Box<dyn File>> {
        tracing::info!("filesystem: open {:?}", path);
        let mut options = std::fs::OpenOptions::new();
        match mode {
            FileOpenMode::Read => options.read(true),
            FileOpenMode::Write => options.write(true).create(true).truncate(true),
            FileOpenMode::Append => options.append(true).create(true),
            FileOpenMode::Update => options.read(true).write(true).create(true),
        };
        Ok(Box::new(NullFile(options.open(path)?)))
    }
}

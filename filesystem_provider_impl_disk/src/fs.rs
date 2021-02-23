use ::{
    filesystem_provider_api::{
        fs as api_fs,
        fs::{entity as api_entity, ops as api_ops},
    },
    std::{
        io,
        path::{Path, PathBuf},
    },
};

#[derive(Debug)]
pub struct File(std::fs::File);

impl io::Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl io::Seek for File {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.0.seek(pos)
    }
}

impl io::Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl api_entity::File for File {
    fn size(&self) -> u64 {
        self.0.metadata().unwrap().len()
    }

    fn is_file(&self) -> bool {
        self.0.metadata().unwrap().is_file()
    }

    fn is_dir(&self) -> bool {
        self.0.metadata().unwrap().is_dir()
    }
}

#[derive(Debug)]
pub struct DirEntry(std::fs::DirEntry);

impl api_entity::File for DirEntry {
    fn size(&self) -> u64 {
        self.0.metadata().unwrap().len()
    }

    fn is_file(&self) -> bool {
        self.0.metadata().unwrap().is_file()
    }

    fn is_dir(&self) -> bool {
        self.0.metadata().unwrap().is_dir()
    }
}

impl api_entity::DirEntry for DirEntry {
    fn path(&self) -> PathBuf {
        self.0.path()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OpenEntityError {
    #[error("out of access {0:?}")]
    AccessError(PathBuf),
    #[error("entity not readable")]
    ReadError,
    #[error("{0:?}")]
    #[rustfmt::skip]
    IoError(#[from]#[source]io::Error),
}

#[derive(Debug)]
pub struct DirEntries {
    read_dir: std::fs::ReadDir,
}

impl std::iter::Iterator for DirEntries {
    type Item = Result<DirEntry, OpenEntityError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_dir.next() {
            Some(Ok(entry)) => Some(Ok(DirEntry(entry))),
            Some(Err(err)) => Some(Err(err.into())),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct Dir(PathBuf);

impl api_entity::File for Dir {
    fn size(&self) -> u64 {
        self.0.metadata().unwrap().len()
    }

    fn is_file(&self) -> bool {
        self.0.metadata().unwrap().is_file()
    }

    fn is_dir(&self) -> bool {
        self.0.metadata().unwrap().is_dir()
    }
}

impl api_entity::Dir for Dir {
    type Entries = DirEntries;
    type EntriesE = io::Error;
    type Entry = DirEntry;
    type IterE = OpenEntityError;

    fn total_size(&self) -> u64 {
        self.0
            .read_dir()
            .unwrap()
            .map(|entry| entry.unwrap().metadata().unwrap().len())
            .sum::<u64>()
    }

    fn count(&self) -> usize {
        std::fs::read_dir(self.0.as_path()).unwrap().count()
    }

    fn entries(&self) -> Result<Self::Entries, Self::EntriesE> {
        let read_dir = self.0.read_dir()?;
        Ok(DirEntries { read_dir })
    }
}

/// このファイルシステムのサブパスはカレントディレクトリか通常のコンポーネントで開始し基底パス下階のみを指さなければならない。
#[derive(Debug)]
pub struct FileSystem {
    // 基底パスが変更されないようにPathBufではなくPathを利用する。
    // 所有権を持つのでBox化する。
    pub(crate) root: Box<Path>,
}

impl FileSystem {
    fn current<P: AsRef<Path>>(&self, sub: &P) -> PathBuf {
        self.root.join(&sub)
    }
}

impl api_fs::Introspect for FileSystem {
    fn is_readable(&self) -> bool {
        true
    }

    fn is_writable(&self) -> bool {
        true
    }

    fn is_appendable(&self) -> bool {
        true
    }

    fn is_truncatable(&self) -> bool {
        true
    }

    fn is_removable(&self) -> bool {
        true
    }
}

impl api_fs::FileSystem for FileSystem {
    type MetadataE = std::io::Error;

    fn metadata<P: AsRef<Path>>(&self, sub: P) -> Result<api_entity::Metadata, Self::MetadataE>
    where
        Self: Sized,
    {
        let sub = sub.as_ref();
        let metadata = sub.metadata()?;
        Ok(api_entity::Metadata::new(
            sub.to_path_buf().into_boxed_path(),
            if metadata.is_file() {
                api_entity::Type::File
            } else {
                assert!(metadata.is_dir());
                api_entity::Type::Dir
            },
            metadata.len(),
        ))
    }

    fn exists<P: AsRef<Path>>(&self, path: P) -> bool
    where
        Self: Sized,
    {
        self.current(&path).exists()
    }

    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool
    where
        Self: Sized,
    {
        self.current(&path).is_file()
    }

    fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool
    where
        Self: Sized,
    {
        self.current(&path).is_dir()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveEntityError {
    #[error("out of access {0:?}")]
    AccessError(PathBuf),
    #[error("entity not removable")]
    RemoveError,
    #[error("{0:?}")]
    #[rustfmt::skip]
    IoError(#[from]#[source]io::Error),
}

/// パスはカレントディレクトリか通常のコンポーネントで始まり、カレント・親ディレクトリか通常のコンポーネントが続かなければならない。
/// そうでなければエラーとする。
/// さらにパスがファイルシステムの基底パス（ルートパス）の下階以外を表しているときも同様とする。
fn check_path<R, F: FnOnce(PathBuf) -> R>(ctor: F, path: &Path) -> Result<(), R> {
    use std::path::Component;

    let mut components = path.components();

    let mut level = match components.next() {
        Some(Component::CurDir) => 0i32,
        Some(Component::Normal(_)) => 1i32,
        _ => return Err(ctor(path.to_owned())),
    };

    for comp in components {
        match comp {
            Component::CurDir => (),
            Component::ParentDir => level -= 1,
            Component::Normal(_) => level += 1,
            _ => return Err(ctor(path.to_owned())),
        }
    }

    if level < 0 {
        return Err(ctor(path.to_owned()));
    } else {
        Ok(())
    }
}

macro_rules! def_impl_ops_trait_for_filesystem {
    ($trait_name:path, $fn_name:path) => {
        impl $trait_name for FileSystem {
            type E = crate::fs::RemoveEntityError;

            fn remove<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::E> {
                let path = self.current(&path);
                let _ = check_path(RemoveEntityError::AccessError, &path)?;
                $fn_name(self.current(&path)).map_err(RemoveEntityError::IoError)
            }
        }
    };
}

def_impl_ops_trait_for_filesystem!(filesystem_provider_api::fs::ops::RemoveFile, std::fs::remove_file);
def_impl_ops_trait_for_filesystem!(filesystem_provider_api::fs::ops::RemoveDir, std::fs::remove_dir_all);

#[derive(Debug, thiserror::Error)]
pub enum CreateEntityError {
    #[error("out of access {0:?}")]
    AccessError(PathBuf),
    #[error("entity not writable")]
    WriteError,
    #[error("{0:?}")]
    #[rustfmt::skip]
    IoError(#[from]#[source]io::Error),
}

impl api_ops::CreateFile for FileSystem {
    type E = CreateEntityError;
    type File = File;

    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::File, Self::E> {
        let path = self.current(&path);
        let _ = check_path(CreateEntityError::AccessError, &path)?;

        std::fs::File::create(path)
            .map(File)
            .map_err(CreateEntityError::IoError)
    }

    fn create_new<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::File, Self::E> {
        let path = self.current(&path);
        let _ = check_path(CreateEntityError::AccessError, &path)?;

        std::fs::OpenOptions::new()
            .create_new(true)
            .open(path)
            .map(File)
            .map_err(CreateEntityError::IoError)
    }
}

impl FileSystem {
    fn create_new_dir_impl(&self, path: &Path) -> Result<Dir, CreateEntityError> {
        std::fs::create_dir_all(path)
            .map(|_| Dir(path.to_path_buf()))
            .map_err(Into::into)
    }
}

impl api_ops::CreateDir for FileSystem {
    type Dir = Dir;
    type E = CreateEntityError;

    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::Dir, Self::E> {
        let path = self.current(&path);
        let _ = check_path(CreateEntityError::AccessError, &path)?;

        if path.exists() {
            Ok(Dir(path))
        } else {
            self.create_new_dir_impl(path.as_ref())
        }
    }

    fn create_new<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::Dir, Self::E> {
        let path = self.current(&path);
        let _ = check_path(CreateEntityError::AccessError, &path)?;

        self.create_new_dir_impl(path.as_ref())
    }
}

impl api_ops::OpenFile for FileSystem {
    type E = OpenEntityError;
    type File = File;

    fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::File, Self::E> {
        let path = self.current(&path);
        let _ = check_path(OpenEntityError::AccessError, &path)?;

        std::fs::OpenOptions::new()
            .read(true)
            .open(path)
            .map(File)
            .map_err(Into::into)
    }
}

impl api_ops::OpenDir for FileSystem {
    type Dir = Dir;
    type E = OpenEntityError;

    /// XXX: 現在、この呼出しではディレクトリを開かない
    fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::Dir, Self::E> {
        let path = self.current(&path);
        let _ = check_path(OpenEntityError::AccessError, &path)?;
        Ok(Dir(path))
    }
}

#[cfg(test)]
mod check_path {
    use ::{filesystem_provider_api::fs::ops, std::path::Path};

    use crate::fs::{self, OpenEntityError};

    #[test]
    fn llegal_subpath_start_with_current() -> Result<(), Box<dyn std::error::Error>> {
        let root = Box::from(Path::new("."));
        let mut filesystem = fs::FileSystem { root };

        let sub = Path::new(".");
        match ops::OpenFile::open(&mut filesystem, sub) {
            Err(OpenEntityError::AccessError(_)) => unreachable!(),
            _ => (),
        }
        Ok(())
    }

    #[test]
    fn llegal_subpath_start_with_normal() -> Result<(), Box<dyn std::error::Error>> {
        let root = Box::from(Path::new("."));
        let mut filesystem = fs::FileSystem { root };

        let sub = Path::new("src");
        match ops::OpenFile::open(&mut filesystem, sub) {
            Err(OpenEntityError::AccessError(_)) => unreachable!(),
            _ => (),
        }
        Ok(())
    }

    #[test]
    fn illegal_subpath() -> Result<(), Box<dyn std::error::Error>> {
        let root = Box::from(Path::new("."));
        let mut filesystem = fs::FileSystem { root };

        let sub = Path::new("..");
        match ops::OpenFile::open(&mut filesystem, sub).err().unwrap() {
            OpenEntityError::AccessError(path) => assert_eq!(path, Path::new(".").join("..")),
            _ => unreachable!(),
        }
        Ok(())
    }

    #[test]
    fn illegal_subpath2() -> Result<(), Box<dyn std::error::Error>> {
        let root = Box::from(Path::new("."));
        let mut filesystem = fs::FileSystem { root };

        let sub = Path::new(".").join(".").join("..");
        match ops::OpenFile::open(&mut filesystem, sub).err().unwrap() {
            OpenEntityError::AccessError(path) => assert_eq!(path, Path::new(".").join(".").join("..")),
            _ => unreachable!(),
        }
        Ok(())
    }

    #[test]
    fn open_file() -> Result<(), Box<dyn std::error::Error>> {
        let root = Box::from(Path::new("."));
        let mut filesystem = fs::FileSystem { root };

        let sub = Path::new(".").join("src").join("fs.rs");
        assert!(ops::OpenFile::open(&mut filesystem, sub).is_ok());
        Ok(())
    }

    #[test]
    fn open_dir() -> Result<(), Box<dyn std::error::Error>> {
        let root = Box::from(Path::new("."));
        let mut filesystem = fs::FileSystem { root };

        let sub = Path::new(".").join("src");
        assert!(ops::OpenDir::open(&mut filesystem, sub).is_ok());
        Ok(())
    }
}

#[cfg(test)]
mod dir_entries {
    use ::filesystem_provider_api::fs::{
        entity::{Dir as _, DirEntry as _},
        ops::OpenDir as _,
        FileSystem as _,
    };

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let root = Box::from(std::path::Path::new("."));
        let mut filesystem = crate::fs::FileSystem { root };

        let dir = filesystem.open("src")?;
        let entries = dir.entries()?;
        for entry in entries {
            assert!(filesystem.exists(entry.unwrap().path()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod filesystem {
    use ::{
        filesystem_provider_api::fs::{entity::Type, FileSystem as _},
        std::path::Path,
    };

    use crate::fs::{self};

    #[test]
    fn metadata() -> Result<(), Box<dyn std::error::Error>> {
        let root = Box::from(Path::new("."));
        let filesystem = fs::FileSystem { root };

        let metadata = filesystem.metadata(".")?;

        assert_eq!(metadata.r#type(), &Type::Dir);
        assert_eq!(metadata.path(), Path::new("."));
        Ok(())
    }

    #[test]
    fn exists() {
        let root = Box::from(Path::new("."));
        let filesystem = fs::FileSystem { root };

        assert!(filesystem.exists("."));
    }

    #[test]
    fn is_file() {
        let root = Box::from(Path::new("."));
        let filesystem = fs::FileSystem { root };

        assert!(!filesystem.is_file("."));
    }

    #[test]
    fn is_dir() {
        let root = Box::from(Path::new("."));
        let filesystem = fs::FileSystem { root };

        assert!(filesystem.is_dir("."));
    }
}

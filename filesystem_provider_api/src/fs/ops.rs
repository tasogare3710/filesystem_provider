//! このモジュールにはエンティティを操作するために必要なtraitが定義されています。
//!
//! - open
//! - create
//! - remove
//!
//! # See also
//! [crate::fs]

use ::std::path::Path;

use crate::fs::entity;

pub trait OpenFile {
    type File: entity::File;
    type E;

    /// ファイルを開きます。ファイルが存在しない場合は失敗します。
    /// `path`が少なくとも`Readable`である必要があります。
    fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::File, Self::E>;
}

pub trait OpenDir {
    type Dir: entity::Dir;
    type E;

    /// ディレクトリを開きます。ディレクトリが存在しない場合は失敗します。
    /// `path`が少なくとも`Readable`である必要があります。
    fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::Dir, Self::E>;
}

pub trait CreateFile {
    type File: entity::File;
    type E;

    /// 新しいファイルを作成するか、ファイルが既に存在する場合は開きます。
    ///
    /// `path`が少なくとも`Writable`か`Appendable`である必要があります。
    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::File, Self::E>;

    /// 新しいファイルを作成します。ファイルが既に存在する場合は失敗します。
    ///
    /// `path`が少なくとも`Writable`か`Appendable`である必要がありまが、`Truncate`は無視されます。
    ///
    /// 不可分操作であるかは想定されません。
    fn create_new<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::File, Self::E>;
}

pub trait CreateDir {
    type Dir: entity::Dir;
    type E;

    /// 新しいファイルを作成するか、ファイルが既に存在する場合は開きます。
    /// `path`が少なくとも`Writable`である必要があります。
    fn create<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::Dir, Self::E>;

    /// 新しいファイルを作成します。ファイルが既に存在する場合は失敗します。
    /// `path`が少なくとも`Writable`である必要があります。
    ///
    /// 不可分操作であるかは想定されません。
    fn create_new<P: AsRef<Path>>(&mut self, path: P) -> Result<Self::Dir, Self::E>;
}

/// ファイルの削除
pub trait RemoveFile {
    type E;

    fn remove<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::E>;
}

/// ディレクトリの削除
pub trait RemoveDir {
    type E;

    fn remove<P: AsRef<Path>>(&self, path: P) -> Result<(), Self::E>;
}

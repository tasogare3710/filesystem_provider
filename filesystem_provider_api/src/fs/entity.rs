//! このモジュールにはエンティティを利用するのに必要なtraitが定義されています。

use ::std::path::Path;

pub trait File {
    /// このtraitの実装がディレクトリであってもエンティティそのもののドライブに占める容量です。
    ///
    /// ファイルシステムによってはこのメソッドの返す値に意味はありません。
    fn size(&self) -> u64;
    fn is_file(&self) -> bool;
    fn is_dir(&self) -> bool;
}

/// このtraitのメソッドの呼び出しには潜在的なコストがかかる場合があります。
pub trait Dir: File {
    type Entry: DirEntry;
    type IterE;
    type Entries: std::iter::Iterator<Item = Result<Self::Entry, Self::IterE>>;
    type EntriesE;

    /// ディレクトリ内のすべてのエンティティのドライブに占める容量です。自身を含みません。
    fn total_size(&self) -> u64;
    /// ディレクトリ内のすべてのエンティティの数です。自身を含みません。
    fn count(&self) -> usize;

    /// ディレクトリエントリを返す`Self::Entries`イテレータ。
    ///
    ///
    fn entries(&self) -> Result<Self::Entries, Self::EntriesE>;
}

/// ディレクトリ内の各エントリを表します。
pub trait DirEntry: File {
    /// [Dir::entries]を呼び出したディレクトリを基準とした相対パスを返します。
    fn path(&self) -> std::path::PathBuf;
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Type {
    File,
    Dir,
}

pub struct Metadata {
    path: Box<std::path::Path>,
    r#type: Type,
    size: u64,
}

impl Metadata {
    pub fn new(path: Box<std::path::Path>, r#type: Type, size: u64) -> Self {
        Self { path, r#type, size }
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    pub fn r#type(&self) -> &Type {
        &self.r#type
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

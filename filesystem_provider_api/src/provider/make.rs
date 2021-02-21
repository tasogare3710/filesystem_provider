//! このモジュールにはプロバイダにファイルシステムを作る能力を与えるtraitが定義されています。
//!
//! 作ることのできるファイルシステムは基本的な権限ごとに以下の五つがあります。
//!
//! - [Readable]
//! - [Writable]
//! - [Appendable]
//! - [Truncatable]
//!
//! 複数の権限を持つファイルシステムには複数のtraitが実装されています。
//!
//! このモジュールで定義されるトレイトメソッド`make*`の`root`引数は新しく作られるファイルシステムの基底パスを表します。
//! ファイルシステムのルートに関する詳細は[fsのモジュールレベルドキュメント](crate::fs)を参照してください。

use ::std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub type Result<T> = std::result::Result<T, self::Error>;

/// デフォルトの権限でファイルシステムを作ります。
pub trait Make {
    type FS;

    fn make(root: PathBuf) -> Self::FS;
}

pub trait Readable {
    type Readable;

    fn make_readable(&mut self, root: PathBuf) -> Result<Self::Readable>;
}

pub trait Writable {
    type Writable;

    fn make_writable(&mut self, root: PathBuf) -> Result<Self::Writable>;
}

pub trait Appendable {
    type Appendable;

    fn make_appendable(&mut self, root: PathBuf) -> Result<Self::Appendable>;
}

pub trait Truncatable {
    type Truncatable;

    fn make_truncatable(&mut self, root: PathBuf) -> Result<Self::Truncatable>;
}

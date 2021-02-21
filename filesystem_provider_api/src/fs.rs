//! ファイルシステムとそのエンティティに関するモジュール。
//!
//! ファイルシステムは基底パスを持ち、これはファイルシステムのルートと見なされルート下階のパスにしかアクセス出来ません。
//! したがって、ルートの上階や兄弟などのパスにアクセスすることはエラーとして扱われます。
//!
//! このクレートではディレクトリをファイルとは見なしません。
//! すなわち、ファイルかディレクトリであるかを調べるトレイトメソッドは以下の不変条件が成り立ちます。
//!
//! ```ignore
//! let entity = /* ... */;
//!
//! assert!(entity.is_file() && !entity.is_dir());
//! assert!(!entity.is_file() && entity.is_dir());
//! ```
//!
//! 今のところ、サポートするファイルシステムのエンティティはファイルとディレクトリだけです。
//!
//! # ファイルシステムのcapability
//!
//! ファイルシステムの能力に関する以下の可能性があります。
//!
//! - `Readable`
//! - `Writable`
//! - `Appendable`
//! - `Truncatable`
//! - `Removable`
//!
//! ファイルシステムがどの様な能力を持つかは以下のトレイトメソッドで確認します。
//!
//! - [Introspect::is_readable]
//! - [Introspect::is_writable]
//! - [Introspect::is_appendable]
//! - [Introspect::is_truncatable]
//! - [Introspect::is_removable]
//!
//! 実際のエンティティの操作は以下のモジュールにあるトレイトが行います。
//!
//! - [self::ops]
//!
//! これらは複数を合わせ持つ場合があります。
//!
//! ## See also
//!
//! - [crate::provider]
//!
//! # XXX: ファイルシステムの変更を伴う操作がinterior mutabilityのみを要求するだけで実現できるか不明なため、現在、exterior mutabilityを要求する
//!
//! - exterior mutabilityが不要なAPIを決定する必要がある

pub mod entity;
pub mod ops;

use std::path::Path;

/// ファイルシステムが持つ能力を調べるトレイトメソッドを定義します。
pub trait Introspect {
    /// このファイルシステムが`Readable`か調べます。
    fn is_readable(&self) -> bool;

    /// このファイルシステムが`Writable`か調べます。
    fn is_writable(&self) -> bool;

    /// このファイルシステムが`Appendable`か調べます。
    fn is_appendable(&self) -> bool;

    /// このファイルシステムが`Truncatable`か調べます。
    fn is_truncatable(&self) -> bool;

    /// このファイルシステムが`Removable`か調べます。
    fn is_removable(&self) -> bool;
}

pub trait FileSystem: Introspect {
    type MetadataE;

    /// `sub`が表す[entity::Metadata]を作ります。失敗する場合は`Err(Self::MetadataE)`を返します。
    /// 引数`sub`はこのファイルシステムの基底パスを基準としたサブパスと見なされます。
    fn metadata<P: AsRef<Path>>(&self, sub: P) -> Result<entity::Metadata, Self::MetadataE>
    where
        Self: Sized;

    fn exists<P: AsRef<Path>>(&self, path: P) -> bool
    where
        Self: Sized;

    /// パスがファイルか調べます。
    /// ファイルシステムによっては実際にエンティティへのアクセスが必要になるかもしれません。
    ///
    /// ディレクトリしか含まないファイルシステムの場合、不変条件として常に`false`を返します。
    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool
    where
        Self: Sized;

    /// パスがディレクトリか調べます。
    /// ファイルシステムによっては実際にエンティティへのアクセスが必要になるかもしれません。
    ///
    /// ファイルしか含まないファイルシステムの場合、不変条件として常に`false`を返します。
    fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool
    where
        Self: Sized;
}

//! パーミッションベースのアクセス制御に関するモジュール。

/// エンティティを読み取ることが出来るパーミッション。
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Read(String);

/// エンティティを書き込むことが出来るパーミッション。
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Write(String);

/// エンティティに追記することが出来るパーミッション。
///
/// 追記が困難な場合があるので[Write]と[Append]は分かれています。
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Append(String);

/// エンティティを切り詰めることが出来るパーミッション。
///
/// 切り詰めが困難な場合があるので[Write]と[Truncate]は分かれています。
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Truncate(String);

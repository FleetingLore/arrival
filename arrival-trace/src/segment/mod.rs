/// 路径段的抽象 trait.
///
/// 每个段必须可以转为字符串 (`ToString`), 支持调试输出 (`Debug`),
/// 以及判断是否为空.
pub trait Segment: ToString + std::fmt::Debug {
    /// 判断该段是否为空.
    fn is_empty(&self) -> bool;
}

/// 用 `String` 实现的路径段.
///
/// 仅在启用 `segment-string` feature 时可用.
#[cfg(feature = "segment-string")]
pub mod string;

//! Lore 生态通用的路径方法.
//!
//! 与 `std::path` 不同, 这里的 Trace 是抽象的, 中间数据不作指定, 所以单独做成一个 crate.
//!
//! 为了和 String 互相转换, 提供了一个默认实现.

pub mod segment;
pub mod trace;

pub use trace::Trace;

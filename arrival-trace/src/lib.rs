//! Lore 生态通用的抽象路径方法.
//!
//! 与 `std::path` 不同, `Trace` 不对路径的内部结构做具体约定 -- 各段
//! (segment) 的类型由使用者决定, 只要求实现 [`Segment`] trait. 这种设计
//! 使得 `Trace` 可以表示任意层次的路径, 而不局限于文件系统.
//!
//! # 核心概念
//!
//! - [`Trace`]: 一条由若干段组成的路径, 可以标记左右两端是否"开放"
//!   (即路径前后是否允许继续延伸).
//! - [`Segment`]: 路径中的一段, 要求实现 `ToString + Debug` 并提供
//!   `is_empty` 判断.
//!
//! # 默认实现
//!
//! 通过 feature `segment-string` (默认启用) 提供了 [`StringSegment`],
//! 它是对 `String` 的简单封装. 此时 `Trace` 的 `Display` 输出为各段
//! 以 `::` 连接的形式, 例如 `root::child::leaf`.
//!
//! # Features
//!
//! - `segment-string` (默认): 启用 [`StringSegment`] 及相关的便捷构造方法.
//!
//! # 示例
//!
//! ```
//! use arrival_trace::Trace;
//!
//! // 从字符串构造
//! let trace = Trace::from_str("root::child::leaf");
//! assert_eq!(trace.to_string(), "root::child::leaf");
//!
//! // 从空开始构造
//! let mut trace = Trace::new();
//! trace.push_str("root");
//! trace.push_str("child");
//! assert_eq!(trace.to_string(), "root::child");
//!
//! // 序列化为字符串列表
//! assert_eq!(
//!     trace.segments_str(),
//!     vec!["root".to_string(), "child".to_string()]
//! );
//! ```

pub mod segment;
pub mod trace;

pub use trace::Trace;

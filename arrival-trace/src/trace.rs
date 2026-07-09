use crate::segment::Segment;
use std::fmt::{Debug, Display, Formatter, Result};

/// 一条抽象路径.
///
/// `Trace` 由若干段 ([`Segment`]) 组成, 并以 `left_open` / `right_open`
/// 标记两端是否开放. 开放的含义是: 路径前/后还可以继续拼接更多段.
///
/// # Display 格式
///
/// 各段以 `::` 连接. 若 `left_open` 为真, 则在最前面加上 `::`;
/// 若 `right_open` 为真, 则在最后面加上 `::`.
///
/// 例如 `left_open = true, segments = [a, b]` 输出 `::a::b`.
#[derive(Debug)]
pub struct Trace {
    segments: Vec<Box<dyn Segment>>,
    left_open: bool,
    right_open: bool,
}

impl Trace {
    /// 创建一条空的、两端均闭合的路径.
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            left_open: false,
            right_open: false,
        }
    }

    /// 从字符串解析路径.
    ///
    /// 各段以 `::` 分隔. 前缀 `::` 表示 `left_open`, 后缀 `::`
    /// 表示 `right_open`.
    ///
    /// 仅在启用 `segment-string` feature 时可用.
    #[cfg(feature = "segment-string")]
    pub fn from_str(s: &str) -> Self {
        let s = s.trim();
        let left_open = s.starts_with("::");
        let right_open = s.ends_with("::") && s.len() > 2;

        let inner = if left_open { &s[2..] } else { s };
        let inner = if right_open {
            &inner[..inner.len() - 2]
        } else {
            inner
        };

        let segments: Vec<Box<dyn Segment>> = if inner.is_empty() {
            Vec::new()
        } else {
            inner
                .split("::")
                .map(|part| {
                    Box::new(crate::segment::string::StringSegment::new(part.to_string()))
                        as Box<dyn Segment>
                })
                .collect()
        };

        Self {
            segments,
            left_open,
            right_open,
        }
    }

    /// 追加一个段.
    pub fn push(&mut self, segment: impl Segment + 'static) {
        self.segments.push(Box::new(segment));
    }

    /// 以字符串形式追加一个段.
    ///
    /// 仅在启用 `segment-string` feature 时可用.
    #[cfg(feature = "segment-string")]
    pub fn push_str(&mut self, s: &str) {
        self.segments
            .push(Box::new(crate::segment::string::StringSegment::new(
                s.to_string(),
            )));
    }

    /// 返回所有段的字符串表示.
    pub fn segments_str(&self) -> Vec<String> {
        self.segments.iter().map(|s| s.to_string()).collect()
    }

    /// 判断路径是否为空 (无段且两端闭合).
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() && !self.left_open && !self.right_open
    }
}

impl Default for Trace {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Trace {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let inner = self
            .segments
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("::");

        let result = std::iter::empty()
            .chain(self.left_open.then_some("::"))
            .chain(Some(inner.as_str()))
            .chain(self.right_open.then_some("::"))
            .collect::<String>();

        write!(f, "{}", result)
    }
}

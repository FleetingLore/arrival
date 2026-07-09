use super::Segment;

/// 用 `String` 实现的路径段.
///
/// 这是 [`Segment`] trait 的默认实现, 将一段路径表示为一个普通字符串.
#[derive(Clone, Debug)]
pub struct StringSegment(String);

impl StringSegment {
    /// 从字符串创建新的路径段.
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

impl ToString for StringSegment {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Segment for StringSegment {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

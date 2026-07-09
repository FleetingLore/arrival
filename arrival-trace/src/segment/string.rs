use super::Segment;

/// 字符串链
#[derive(Clone, Debug)]
pub struct StringSegment(String);

impl StringSegment {
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

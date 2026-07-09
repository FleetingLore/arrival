use crate::segment::Segment;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
pub struct Trace {
    segments: Vec<Box<dyn Segment>>,
    left_open: bool,
    right_open: bool,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            left_open: false,
            right_open: false,
        }
    }

    pub fn push(&mut self, segment: impl Segment + 'static) {
        self.segments.push(Box::new(segment));
    }

    #[cfg(feature = "segment-string")]
    pub fn push_str(&mut self, s: &str) {
        self.segments
            .push(Box::new(crate::segment::string::StringSegment::new(
                s.to_string(),
            )));
    }

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

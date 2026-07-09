pub trait Segment: ToString + std::fmt::Debug {
    fn is_empty(&self) -> bool;
}

#[cfg(feature = "segment-string")]
pub mod string;

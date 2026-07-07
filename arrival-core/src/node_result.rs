use crate::{Arg, Target, Path};

pub enum NodeResult {
    Next(Box<dyn Arg>, Path),
    Done(Box<dyn Target>),
}

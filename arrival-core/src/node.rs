use crate::{Arg, NodeResult};

pub trait Node {
    fn path(&self) -> crate::Path;
    fn process(&self, arg: &dyn Arg) -> NodeResult;
}

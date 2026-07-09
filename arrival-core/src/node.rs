use arrival_trace::Trace;

pub trait Node {
    fn path(&self) -> Trace;
    fn process(&self, arg: &dyn Arg) -> NodeResult;
}

/// 参考
pub trait Arg {
    fn to_string(&self) -> String;
}

pub enum NodeResult {
    Next(Box<dyn Arg>, Trace),
    Done(Box<dyn Target>),
}

pub trait Target {
    fn to_string(&self) -> String;
}

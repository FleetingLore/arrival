use arrival_core::Arg;

pub struct StringArg {
    pub raw: String,
}

impl Arg for StringArg {
    fn to_string(&self) -> String {
        self.raw.clone()
    }
}

pub struct StringTarget {
    pub value: String,
}

impl arrival_core::Target for StringTarget {
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

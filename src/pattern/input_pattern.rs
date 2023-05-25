use crate::pattern::argument::Argument;

#[derive(Clone, Debug)]
pub struct InputPattern {
    pub arguments: Vec<Argument>,
    pub info: String,
}

impl InputPattern {
    pub fn new(arguments: Vec<Argument>, info: String) -> Self {
        InputPattern { arguments, info }
    }
}
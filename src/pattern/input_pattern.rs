use crate::bot::Input;
use crate::pattern::argument::{ArgumentPattern, KeywordArgumentPattern};

#[derive(Clone, Debug)]
pub struct InputPattern {
    pub arguments: Vec<ArgumentPattern>,
    pub kw_arguments: Vec<KeywordArgumentPattern>,
    pub info: String,
}

impl InputPattern {
    pub fn new(arguments: Vec<ArgumentPattern>, kw_arguments: Vec<KeywordArgumentPattern>, info: String) -> Self {
        InputPattern { arguments, kw_arguments, info }
    }

    // TODO pub fn check_arguments(&self, input: Input) -> Result<(), String>
}
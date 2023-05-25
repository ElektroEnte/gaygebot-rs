#[derive(Clone, Debug)]
pub enum ArgumentType {
    Normal,
    Keyword,
    Text,
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub argument_type: ArgumentType,
    pub is_required: bool,
    pub identifier: String,
    pub default: String,
    pub info: String,
}
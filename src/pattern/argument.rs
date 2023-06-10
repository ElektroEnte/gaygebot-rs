// #[derive(Clone, Debug)]
// pub enum ArgumentType {
//     Normal,
//     Keyword,
//     // Text,
// }

#[derive(Clone, Debug)]
pub struct ArgumentPattern {
    name: String,
    default: String,
    is_required: bool,
}

#[derive(Clone, Debug)]
pub struct KeywordArgumentPattern {
    key: String,
    default_value: String,
    is_required: bool,
}


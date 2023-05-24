use std::collections::HashMap;
use crate::bot::Context;

pub struct NormalInput {
    pub text: String,
    pub ctx: Context,
}

pub struct CommandInput {
    pub text: String,
    pub identifier: String,
    pub args: Vec<String>,
    pub kwargs: HashMap<String, String>,
    pub ctx: Context,
}

pub enum Input {
    Normal,
    Command(CommandInput),
}


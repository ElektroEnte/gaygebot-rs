use std::collections::HashMap;
use crate::bot::Context;
use crate::bot_env::variable::Variable;
use crate::pattern::Pattern;

#[derive(Clone, Debug)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub variables: HashMap<String, Variable>,
    pub patterns: Vec<Pattern>
}

impl Environment {
    pub fn match_pattern(&self, ctx: Context) -> Option<Pattern> { todo!() }
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            id: "0".to_string(),
            name: "default".to_string(),
            prefix: "!".to_string(),
            variables: HashMap::new(),
            patterns: vec![],
        }
    }
}
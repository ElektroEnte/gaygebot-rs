use std::collections::HashMap;
use crate::bot_env::variable::Variable;

pub struct Environment {
    id: String,
    name: String,
    prefix: String,
    variables: HashMap<String, Variable>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            id: "0".to_string(),
            name: "default".to_string(),
            prefix: "!".to_string(),
            variables: HashMap::new(),
        }
    }
}
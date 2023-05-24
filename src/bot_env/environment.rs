use std::collections::HashMap;
use crate::bot_env::variable::Variable;

#[derive(Clone, Debug)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub variables: HashMap<String, Variable>,
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
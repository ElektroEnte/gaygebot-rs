use std::collections::HashMap;
use crate::bot_env::variable::Variable;

pub struct Environment {
    id: String,
    name: String,
    prefix: String,
    variables: HashMap<String, Variable>,
}
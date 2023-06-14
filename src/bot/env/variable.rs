use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Variable {
    Null,
    Str(String),
    Int(i64),
    // Bool(bool),
    // List(Vec<Variable>),
    // Map(HashMap<String, Variable>)
}


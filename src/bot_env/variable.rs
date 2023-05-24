use std::collections::HashMap;

pub enum Variable {
    Null,
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Variable>),
    HashMap(HashMap<String, Variable>)
}
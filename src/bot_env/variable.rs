use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Variable {
    Null,
    Str(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Variable>),
    Map(HashMap<String, Variable>)
}
use std::collections::HashMap;
use crate::bot::Context;
use crate::bot::env::counter::Counter;
use super::variable::Variable;
use crate::job::job_parameter::JobParameter;
use crate::job::job_pattern::JobPattern;

#[derive(Clone, Debug)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub variables: HashMap<String, Variable>,
    pub patterns: Vec<JobPattern>,
    pub counters: Vec<Counter>,
}

impl Environment {
    pub fn get_pattern(&self, pattern_name: &String) -> Option<&JobPattern> {
        for pattern in &self.patterns {
            if &pattern.name == pattern_name {
                return Some(pattern);
            }
        }

        None
    }

    pub fn get_mut_pattern(&mut self, pattern_name: &String) -> Option<&mut JobPattern> {
        for pattern in &mut self.patterns {
            if &pattern.name == pattern_name {
                return Some(pattern);
            }
        }

        None
    }

    pub fn has_pattern(&self, pattern_name: &String) -> bool {
        for pattern in self.patterns.iter() {
            if &pattern.name == pattern_name { return true; }
        }

        false
    }

    pub fn get_parameter(&self, pattern_name: &String, parameter_name: &String) -> Option<&JobParameter> {
        if let Some(pattern) = self.get_pattern(pattern_name) {
            return pattern.get_parameter(parameter_name);
        }

        None
    }

    pub fn get_mut_parameter(&mut self, pattern_name: &String, parameter_name: &String) -> Option<&mut JobParameter> {
        if let Some(pattern) = self.get_mut_pattern(pattern_name) {
            return pattern.get_mut_parameter(parameter_name);
        }

        None
    }

    pub fn set_prefix(&mut self, prefix: String) {
        self.prefix = prefix;
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            id: "0".to_string(),
            name: "default".to_string(),
            prefix: "!".to_string(),
            variables: HashMap::new(),
            patterns: vec![],
            counters: vec![],
        }
    }
}
use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::Mutex;

use tokio::time::Instant;

use crate::bot::Context;
use crate::bot::env::counter::Counter;
use crate::bot::env::output_manager::OutputQueue;
use super::variable::Variable;
use crate::job::job_parameter::JobParameter;
use crate::job::job_pattern::JobPattern;
use crate::job::library::counter::CounterJob;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BotPermissions {
    Default,
    Super,
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub id: String,
    pub name: String,
    pub prefix: String,
    pub variables: HashMap<String, Variable>,
    pub patterns: Vec<JobPattern>,
    pub counters: HashMap<String, Counter>,
    pub permissions: BotPermissions,
    pub output_queue: Arc<Mutex<OutputQueue>>,
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

    pub fn has_counter(&self, counter_name: &String) -> bool {
        self.counters.contains_key(counter_name.as_str())
    }

    pub fn create_counter(&mut self, counter_name: String, value: i64) {
        self.counters.insert(counter_name, Counter::new(value));
    }

    pub fn delete_counter(&mut self, counter_name: String) {
        self.counters.remove(counter_name.as_str());
    }

    pub fn get_counter(&self, counter_name: &String) -> Option<&Counter> {
        self.counters.get(counter_name.as_str())
    }

    pub fn get_mut_counter(&mut self, counter_name: &String) -> Option<&mut Counter> {
        self.counters.get_mut(counter_name.as_str())
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
            counters: HashMap::new(),
            permissions: BotPermissions::Default,
            output_queue: Arc::new(Mutex::new(OutputQueue::default())),
        }
    }
}
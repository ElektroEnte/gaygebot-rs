use std::collections::HashMap;
use crate::bot::env::counter::Counter;
use crate::bot::env::environment::Environment;
use crate::bot::Input;
use crate::job::job::Job;
use crate::job::job_parameter::JobParameter;

pub struct CounterJob;

impl CounterJob {
    pub fn new() -> Self { CounterJob {} }

    fn create_counter(&self, env: &mut Environment, name: String, value: i64) -> String {
        if env.has_counter(&name) {
            return "\\.meCounter already exists.".to_string();
        }

        env.create_counter(name.to_owned(), value);
        format!("\\.meCreated counter '{}'.", name)
    }

    fn get_counter(&self, env: &mut Environment, name: String) -> String {
        if let Some(counter) = env.get_counter(&name) {
            return format!("{}", counter.get());
        }

        "".to_string()
    }

    fn increase_counter(&self, env: &mut Environment, name: String, value: i64) -> String {
        if let Some(counter) = env.get_mut_counter(&name) {
            counter.add(value);
        }

        "".to_string()
    }

    fn decrease_counter(&self, env: &mut Environment, name: String, value: i64) -> String {
        if let Some(counter) = env.get_mut_counter(&name) {
            counter.subtract(value);
        }

        "".to_string()
    }

    fn set_counter(&self, env: &mut Environment, name: String, value: i64) -> String {
        if let Some(counter) = env.get_mut_counter(&    name) {
            counter.set(value);
        }

        "".to_string()
    }
}

impl Job for CounterJob {
    fn execute(&self, input: &mut Input, params: HashMap<String, String>) -> String {
        if let Some(action) = params.get("action") {
            let words: Vec<&str> = input.text.split(" ").collect();

            let name = words
                .get(2)
                .unwrap_or(&"")
                .to_string();
            let value = match action.as_str() {
                "set" => {
                    words
                        .get(3)
                        .unwrap_or(&"")
                        .to_string()
                        .parse::<i64>()
                        .unwrap_or(input.ctx.environment.get_counter(&name).unwrap_or(&Counter::default()).get())
                }
                _ => {
                    words
                        .get(3)
                        .unwrap_or(&"")
                        .to_string()
                        .parse::<i64>()
                        .unwrap_or(0)
                }
            };

            let result = match action.as_str() {
                "create" | "new" => { self.create_counter(&mut input.ctx.environment, name, value) }
                "get" => { self.get_counter(&mut input.ctx.environment, name) }
                "increase" | "add" => { self.increase_counter(&mut input.ctx.environment, name, value) }
                "decrease" | "subtract" => { self.decrease_counter(&mut input.ctx.environment, name, value) }
                "set" => { self.set_counter(&mut input.ctx.environment, name, value) }
                _ => { format!("Invalid action.") }
            };

            return result;
        }

        format!("No action provided.")
    }

    fn get_params(&self) -> Vec<JobParameter> {
        vec![JobParameter::new("action".to_string(), "get".to_string())]
    }
}
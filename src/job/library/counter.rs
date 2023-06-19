use std::sync::Arc;
use std::collections::HashMap;

use tokio;
use tokio::sync::Mutex;

use async_trait::async_trait;

use crate::bot::env::counter::Counter;
use crate::bot::env::environment::Environment;
use crate::bot::Input;
use crate::job::job::Job;
use crate::job::job_parameter::JobParameter;

pub struct CounterJob;

impl CounterJob {
    pub fn new() -> Self { CounterJob {} }

    async fn create_counter(&self, env: Arc<Mutex<Environment>>, name: String, value: i64) -> String {
        let mut env = env.lock().await;
        
        if env.has_counter(&name) {
            return "\\.meCounter already exists.".to_string();
        }

        env.create_counter(name.to_owned(), value);
        format!("\\.meCreated counter '{}'.", name)
    }

    async fn get_counter(&self, env: Arc<Mutex<Environment>>, name: String) -> String {
        let mut env = env.lock().await;
        
        if let Some(counter) = env.get_counter(&name) {
            return format!("{}", counter.get());
        }

        "".to_string()
    }

    async fn increase_counter(&self, env: Arc<Mutex<Environment>>, name: String, value: i64) -> String {
        let mut env = env.lock().await;
        
        if let Some(counter) = env.get_mut_counter(&name) {
            counter.add(value);
        }

        "".to_string()
    }

    async fn decrease_counter(&self, env: Arc<Mutex<Environment>>, name: String, value: i64) -> String {
        let mut env = env.lock().await;
        
        if let Some(counter) = env.get_mut_counter(&name) {
            counter.subtract(value);
        }

        "".to_string()
    }

    async fn set_counter(&self, env: Arc<Mutex<Environment>>, name: String, value: i64) -> String {
        let mut env = env.lock().await;
        
        if let Some(counter) = env.get_mut_counter(&    name) {
            counter.set(value);
        }

        "".to_string()
    }
}

#[async_trait]
impl Job for CounterJob {
    async fn execute(&self, input: &mut Input, params: HashMap<String, String>) -> String {
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
                        .parse::<i64>()
                        .unwrap_or(input.ctx.environment.lock().await.get_counter(&name).unwrap_or(&Counter::default()).get())
                }
                _ => {
                    words
                        .get(3)
                        .unwrap_or(&"")
                        .parse::<i64>()
                        .unwrap_or(0)
                }
            };

            let result = match action.as_str() {
                "create" | "new" => { self.create_counter(Arc::clone(&input.ctx.environment), name, value).await }
                "get" => { self.get_counter(Arc::clone(&input.ctx.environment), name).await }
                "increase" | "add" => { self.increase_counter(Arc::clone(&input.ctx.environment), name, value).await }
                "decrease" | "subtract" => { self.decrease_counter(Arc::clone(&input.ctx.environment), name, value).await }
                "set" => { self.set_counter(Arc::clone(&input.ctx.environment), name, value).await }
                _ => { format!("Invalid action.") }
            };

            return result;
        }

        format!("No action provided.")
    }

    async fn get_params(&self) -> Vec<JobParameter> {
        vec![JobParameter::new("action".to_string(), "get".to_string())]
    }
}
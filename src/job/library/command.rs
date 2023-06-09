use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use tokio::sync::Mutex;

use crate::bot::{
    Input,
    env::environment::Environment,
};

use super::super::{
    job_pattern::JobPattern,
    job_parameter::JobParameter,
    job::Job,
};


pub struct CommandJob;

#[async_trait]
impl Job for CommandJob {
    async fn execute(&self, input: &mut Input, params: HashMap<String, String>) -> String {
        // Prevent this job from being used in another job since it uses the original chat message.
        if input.ctx.command_history.len() > 0 { return "".to_string(); }

        let words: Vec<&str> = input.text.split(" ").collect();

        let mut action = String::new();
        let mut command = String::new();
        let mut content = String::new();

        for (word, i) in words.iter().zip(0..words.len()) {
            if i == 1 {
                action = word.to_string();
            } else if i == 2 {
                command = word.to_string();
            } else if i != 0 {
                content.push_str(word);
                content.push(' ');
            }
        }

        let return_string = match action.as_str() {
            "create" | "new" => { self.create_pattern(Arc::clone(&input.ctx.environment), command, content.trim().to_string()).await }
            "edit" => { self.edit_pattern(Arc::clone(&input.ctx.environment), command, content.trim().to_string()).await }
            "remove" | "delete" | "rmv" => { self.remove_pattern(Arc::clone(&input.ctx.environment), command).await }
            "addargument" | "addarg" | "addparameter" | "addparam" | "addprm" => {
                self.add_pattern_parameter(
                    Arc::clone(&input.ctx.environment),
                    command,
                    words.get(3).unwrap_or(&"").to_string(),
                    words.get(4).unwrap_or(&"").to_string(),
                )
                    .await
            }
            "removeargument" | "removearg" | "rmvarg" | "removeparameter" | "removeparam" | "rmvparam" | "rmvprm" | "deleteargument" | "deletearg" | "deleteparameter" | "deleteparam" => {
                self.remove_pattern_parameter(
                    Arc::clone(&input.ctx.environment),
                    command,
                    words.get(3).unwrap_or(&"").to_string()
                )
                    .await
            }
            "info" | "debug" => { self.info(Arc::clone(&input.ctx.environment), command).await }
            _ => { "Invalid action.".to_string() }
        };

        format!("\\.me{}", return_string)
    }

    async fn get_params(&self) -> Vec<JobParameter> {
        vec![]
    }
}

impl CommandJob {
    pub fn new() -> Self { CommandJob {} }

    async fn info(&self, env: Arc<Mutex<Environment>>, name: String) -> String {
        let env = env.lock().await;
        if let Some(pattern) = env.get_pattern(&name) {
            return format!("name: '{}', args: {:?}", pattern.name, pattern.input_params);
        }
        drop(env);

        "Command not found.".to_string()
    }

    async fn create_pattern(&self, env: Arc<Mutex<Environment>>, name: String, output_pattern: String) -> String {
        let mut env = env.lock().await;
        if env.has_pattern(&name) {
            return format!("Command pattern name '{}' already exists.", &name);
        }

        if env.patterns.len() >= 1000 {
            return "Limit of 1000 commands patterns reached.".to_string();
        }

        env.patterns.push(JobPattern::new(name.clone(), output_pattern));
        format!("Created command pattern '{}' in environment '{}'.", &name, &env.name)
    }

    
    async fn edit_pattern(&self, env: Arc<Mutex<Environment>>, name: String, output_pattern: String) -> String {
        let mut env = env.lock().await;
        
        if let Some(pattern) = env.get_mut_pattern(&name) {
            pattern.output_string = output_pattern
        } else {
            return "Command doesn't exist.".to_string();
        }

        format!("Edited command pattern '{}' in environment '{}'.", &name, &env.name)
    }

    // Removes a pattern with the given name from the current env.
    async fn remove_pattern(&self, env: Arc<Mutex<Environment>>, name: String) -> String {
        let mut env = env.lock().await;
        
        for (pattern, i) in env.patterns.iter().zip(0..env.patterns.len()) {
            if &pattern.name == &name {
                env.patterns.remove(i);
                return format!("Removed command pattern '{}'.", name);
            }
        }

        format!("Command pattern '{}' doesn't exist in this environment.", name)
    }

    // Adds a parameter to the given pattern name in the current env.
    async fn add_pattern_parameter(&self, env: Arc<Mutex<Environment>>, pattern_name: String, param_name: String, param_default: String) -> String {
        let mut env = env.lock().await;
        
        if param_name == "".to_string() {
            return "Name of argument missing.".to_string();
        }

        if param_name.contains(&['[', ']', '\\'][..]) {
            return "Parameter name can't contain '[', ']' or '\\'.".to_string();
        }

        return if let Some(pattern) = env.get_mut_pattern(&pattern_name) {
            if pattern.input_params.len() >= 255 { return "Maximum of 255 parameters reached.".to_string(); }

            pattern.add_parameter(param_name.to_owned(), param_default.to_owned());
            format!("Added argument '{}' to pattern '{}' in environment '{}'.", param_name, pattern_name, env.name)
        } else {
            format!("Command pattern '{}' doesn't exist in this environment.", pattern_name)
        }
    }

    async fn remove_pattern_parameter(&self, env: Arc<Mutex<Environment>>, pattern_name: String, param_name: String) -> String {
        let mut env = env.lock().await;
        if param_name == "".to_string() {
            return "Name of parameter missing.".to_string();
        }

        for pattern in env.patterns.iter_mut() {
            if pattern.name == pattern_name && pattern.has_parameter(&param_name) {
                pattern.remove_parameter(&param_name);
                return format!("Removed parameter '{}' from pattern '{}'.", param_name, pattern_name);
            }
        }

        format!("Command pattern '{}' doesn't exist in this environment, or doesn't have an parameter called {}.", pattern_name, param_name)
    }
}
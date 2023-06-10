use std::collections::HashMap;
use crate::bot::Input;
use crate::bot_env::environment::Environment;
use crate::job::job::Job;
use crate::job::job_parameter::JobParameter;
use crate::job::job_pattern::JobPattern;

pub struct CommandJob;

impl Job for CommandJob {
    fn execute(&self, input: &mut Input, params: HashMap<String, String>) -> String {
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

        return match action.as_str() {
            "add" => { self.add_pattern(&mut input.ctx.environment, command, content.trim().to_string()) }
            "edit" => { self.edit_pattern(&mut input.ctx.environment, command, content.trim().to_string()) }
            "remove" => { self.remove_pattern(&mut input.ctx.environment, command) }
            "addargument" => {
                self.add_pattern_argument(
                    &mut input.ctx.environment,
                    command,
                    words.get(3).unwrap_or(&"").to_string(),
                    words.get(4).unwrap_or(&"").to_string(),
                )
            }
            "removeargument" => {
                self.remove_pattern_argument(
                    &mut input.ctx.environment,
                    command,
                    words.get(3).unwrap_or(&"").to_string())
            }
            "info" => { self.info(&mut input.ctx.environment, command) }
            _ => { "Invalid action.".to_string() }
        };
    }

    fn get_params(&self) -> Vec<JobParameter> {
        vec![]
    }
}

impl CommandJob {
    pub fn new() -> Self { CommandJob {} }

    fn info(&self, env: &mut Environment, name: String) -> String {
        for pattern in &env.patterns {
            if pattern.name == name {
                return format!("name: '{}', args: {:?}", pattern.name, pattern.input_params);
            }
        }

        "Command not found.".to_string()
    }

    fn add_pattern(&self, env: &mut Environment, name: String, output_pattern: String) -> String {
        for pattern in &env.patterns {
            if &pattern.name == &name { return format!("Command pattern name '{}' already exists.", &name); }
        }

        env.patterns.push(JobPattern::new(name.clone(), output_pattern));
        format!("Added command pattern '{}' to environment '{}'.", &name, &env.name)
    }

    fn edit_pattern(&self, env: &mut Environment, name: String, output_pattern: String) -> String {
        for (pattern, i) in env.patterns.clone().iter().zip(0..env.patterns.len()) {
            if &pattern.name == &name {
                env.patterns.remove(i);
            }
        }

        env.patterns.push(JobPattern::new(name.clone(), output_pattern));
        format!("Edited command pattern '{}' in environment '{}'.", &name, &env.name)
    }

    fn remove_pattern(&self, env: &mut Environment, name: String) -> String {
        for (pattern, i) in env.patterns.iter().zip(0..env.patterns.len()) {
            if &pattern.name == &name {
                env.patterns.remove(i);
                return format!("Removed command pattern '{}'.", name);
            }
        }

        format!("Command pattern '{}' doesn't exist in this environment.", name)
    }

    fn add_pattern_argument(&self, env: &mut Environment, pattern_name: String, arg_name: String, arg_default: String) -> String {
        if arg_name == "".to_string() {
            return "Name of argument missing.".to_string();
        }

        for pattern in env.patterns.iter_mut() {
            if pattern.name == pattern_name && !pattern.has_argument(&arg_name) {
                pattern.add_argument(arg_name.to_owned(), arg_default);
                return format!("Added argument '{}' to pattern '{}' in environment '{}'.", arg_name, pattern_name, env.name);
            }
        }

        format!("Command pattern '{}' doesn't exist in this environment, or already has an argument called {}.", pattern_name, arg_name)
    }

    fn remove_pattern_argument(&self, env: &mut Environment, pattern_name: String, arg_name: String) -> String {
        if arg_name == "".to_string() {
            return "Name of argument missing.".to_string();
        }

        for pattern in env.patterns.iter_mut() {
            if pattern.name == pattern_name && pattern.has_argument(&arg_name) {
                pattern.remove_argument(arg_name.to_owned());
                return format!("Removed argument '{}' from pattern '{}'.", arg_name, pattern_name);
            }
        }

        format!("Command pattern '{}' doesn't exist in this environment, or doesn't have an argument called {}.", pattern_name, arg_name)
    }
}
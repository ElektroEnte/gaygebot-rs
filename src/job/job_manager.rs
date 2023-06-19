use std::collections::HashMap;

use crate::bot::Input;
use crate::bot::env::environment::Environment;
use crate::bot::identifier::IdentifierType;
use crate::job::{
    job::Job,
    job_parameter::JobParameter,
    job_pattern::JobPattern,
};
use crate::job::library::counter::CounterJob;
use crate::job::library::PrefixJob;
use super::library::{
    CommandJob,
    CustomJob,
    PingJob,
};

pub struct JobManager;

impl JobManager {
    pub async fn execute_job(name: &String, input: &mut Input, params: HashMap<String, String>) -> String {
        match name.as_str() {
            "ping" => { return PingJob::new().execute(input, params).await; }
            "command" | "cmd" | "pattern" => { return CommandJob::new().execute(input, params).await; }
            "prefix" => { return PrefixJob::new().execute(input, params).await; }
            "counter" => { return CounterJob::new().execute(input, params).await; }
            _ => {}
        }

        let mut locked_env = input.ctx.environment.lock().await;
        let patterns = locked_env.patterns.clone();
        drop(locked_env);
        
        for env_pattern in patterns.iter() {
            match &env_pattern.identifier.identifier_type {
                IdentifierType::Command(cmd) => {
                    if cmd == name {
                        return CustomJob::new(env_pattern.clone()).execute(input, params).await;
                    }
                }
                IdentifierType::Username(username) => {
                    if username == &input.ctx.chatter.login {
                        return CustomJob::new(env_pattern.clone()).execute(input, params).await;
                    }
                }
                _ => {}
            };
        }
        
        

        "".to_string()
    }

    pub async fn get_job_params(name: &String, input: &Input) -> Vec<JobParameter> {
        match name.as_str() {
            "ping" => { return PingJob::new().get_params().await; }
            "command" | "cmd" | "pattern" => { return CommandJob::new().get_params().await; }
            "prefix" => { return PrefixJob::new().get_params().await; }
            "counter" => { return CounterJob::new().get_params().await; }
            _ => {}
        }
        
        
        let mut locked_env = input.ctx.environment.lock().await;
        let patterns = locked_env.patterns.clone();
        drop(locked_env);

        for env_pattern in patterns.iter() {
            match &env_pattern.identifier.identifier_type {
                IdentifierType::Command(cmd) => {
                    if cmd == name {
                        return CustomJob::new(env_pattern.clone()).get_params().await;
                    }
                }
                IdentifierType::Username(username) => {
                    if username == &input.ctx.chatter.login {
                        return CustomJob::new(env_pattern.clone()).get_params().await;
                    }
                }
                _ => {}
            };
        }

        vec![]
    }
}

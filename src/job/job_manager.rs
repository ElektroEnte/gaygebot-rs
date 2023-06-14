use std::collections::HashMap;

use crate::bot::Input;
use crate::bot::env::environment::Environment;
use crate::bot::identifier::IdentifierType;
use crate::job::{
    job::Job,
    job_parameter::JobParameter,
    job_pattern::JobPattern,
};
use crate::job::library::PrefixJob;
use super::library::{
    CommandJob,
    CustomJob,
    PingJob,
};

pub struct JobManager;

impl JobManager {
    pub fn execute_job(name: &String, input: &mut Input, params: HashMap<String, String>) -> String {
        match name.as_str() {
            "ping" => { return PingJob::new().execute(input, params); }
            "command" | "cmd" | "pattern" => { return CommandJob::new().execute(input, params); }
            "prefix" => { return PrefixJob::new().execute(input, params); }
            _ => {}
        }

        for env_pattern in &input.ctx.environment.patterns {
            match &env_pattern.identifier.identifier_type {
                IdentifierType::Command(cmd) => {
                    if cmd == name {
                        return CustomJob::new(env_pattern.clone()).execute(input, params);
                    }
                }
                IdentifierType::Username(username) => {
                    if username == &input.ctx.chatter.login {
                        return CustomJob::new(env_pattern.clone()).execute(input, params);
                    }
                }
                _ => {}
            };
        }

        "".to_string()
    }

    pub fn get_job_params(name: &String, input: &Input) -> Vec<JobParameter> {
        match name.as_str() {
            "ping" => { return PingJob::new().get_params(); }
            "command" | "cmd" | "pattern" => { return CommandJob::new().get_params(); }
            "prefix" => { return PrefixJob::new().get_params(); }
            _ => {}
        }


        for env_pattern in &input.ctx.environment.patterns {
            match &env_pattern.identifier.identifier_type {
                IdentifierType::Command(cmd) => {
                    if cmd == name {
                        return CustomJob::new(env_pattern.clone()).get_params();
                    }
                }
                IdentifierType::Username(username) => {
                    if username == &input.ctx.chatter.login {
                        return CustomJob::new(env_pattern.clone()).get_params();
                    }
                }
                _ => {}
            };
        }

        vec![]
    }
}

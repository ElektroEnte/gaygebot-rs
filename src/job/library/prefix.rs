use std::collections::HashMap;
use crate::bot::Input;
use crate::job::job_parameter::JobParameter;
use super::super::job::Job;

pub struct PrefixJob;

impl Job for PrefixJob {
    fn execute(&self, input: &mut Input, params: HashMap<String, String>) -> String {
        if input.ctx.command_history.len() > 0 {
            return "".to_string();
        }

        if let Some(prefix) = params.get("prefix") {
            input.ctx.environment.set_prefix(prefix.to_owned());
            return format!("Set prefix to '{}'.", prefix);
        }

        "Invalid prefix.".to_string()
    }

    fn get_params(&self) -> Vec<JobParameter> {
        vec![JobParameter::new("prefix".to_string(), "!".to_string())]
    }
}

impl PrefixJob {
    pub fn new() -> Self { PrefixJob {} }
}
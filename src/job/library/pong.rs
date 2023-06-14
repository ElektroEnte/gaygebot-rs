use std::collections::HashMap;
use crate::bot::Input;
use crate::job::job::Job;
use crate::job::job_parameter::JobParameter;

pub struct PingJob;

impl Job for PingJob {
    fn execute(&self, input: &mut Input, params: HashMap<String, String>) -> String { "\\.me\\.replyPong".to_string() }

    fn get_params(&self) -> Vec<JobParameter> { vec![] }
}

impl PingJob {
    pub fn new() -> Self { Self {} }
}
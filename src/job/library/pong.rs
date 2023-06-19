use std::collections::HashMap;

use async_trait::async_trait;

use crate::bot::Input;
use crate::job::job::Job;
use crate::job::job_parameter::JobParameter;

pub struct PingJob;

#[async_trait]
impl Job for PingJob {
    async fn execute(&self, input: &mut Input, params: HashMap<String, String>) -> String { "\\.me\\.replyPong".to_string() }

    async fn get_params(&self) -> Vec<JobParameter> { vec![] }
}

impl PingJob {
    pub fn new() -> Self { Self {} }
}
use std::collections::HashMap;
use async_trait::async_trait;
use crate::bot::Input;
use crate::job::job_parameter::JobParameter;

#[async_trait]
pub trait Job {
    async fn execute(&self, input: &mut Input, params: HashMap<String, String>) -> String;  // Should return a Result eventually.

    async fn get_params(&self) -> Vec<JobParameter>;
}
use std::collections::HashMap;
use crate::bot::Input;
use crate::job::job_parameter::JobParameter;

pub trait Job {
    fn execute(&self, input: &mut Input, params: HashMap<String, String>) -> String;  // Should return a Result eventually.

    fn get_params(&self) -> Vec<JobParameter>;
}
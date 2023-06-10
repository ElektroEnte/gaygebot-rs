use crate::bot::Input;
use crate::job::job::Job;
use crate::job::job_parameter::JobParameter;
use crate::pattern::{Identifier, IdentifierType};

#[derive(Clone, Debug)]
pub struct JobPattern {
    pub name: String,
    pub identifier: Identifier,
    pub output_string: String,
    pub input_params: Vec<JobParameter>,
}

impl JobPattern {
    pub fn new(name: String, output_string: String) -> Self {
        JobPattern { name: name.clone(), identifier: Identifier::new(IdentifierType::Command(name)), output_string, input_params: vec![] }
    }

    pub fn add_argument(&mut self, name: String, default: String) {
        self.input_params.push(JobParameter::new(name, default));
    }

    pub fn has_argument(&self, name: &String) -> bool {
        for param in self.input_params.iter() {
            if &param.name == name {
                return true;
            }
        }

        false
    }

    pub fn remove_argument(&mut self, name: String) {
        let length = self.input_params.len();
        for (param, i) in self.input_params.clone().iter().zip(0..length) {
            if &param.name == &name {
                self.input_params.remove(i);
            }
        }
    }
}

use crate::bot::Input;
use crate::job::job::Job;
use crate::job::job_parameter::JobParameter;
use crate::bot::identifier::{Identifier, IdentifierType};

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

    pub fn get_parameter(&self, param_name: &String) -> Option<&JobParameter> {
        for param in self.input_params.iter() {
            if &param.name == param_name {
                return Some(param);
            }
        }

        None
    }

    pub fn get_mut_parameter(&mut self, param_name: &String) -> Option<&mut JobParameter> {
        for param in self.input_params.iter_mut() {
            if &param.name == param_name {
                return Some(param);
            }
        }

        None
    }

    pub fn add_parameter(&mut self, name: String, default: String) {
        self.input_params.push(JobParameter::new(name, default));
    }

    pub fn has_parameter(&self, name: &String) -> bool {
        for param in self.input_params.iter() {
            if &param.name == name {
                return true;
            }
        }

        false
    }

    pub fn remove_parameter(&mut self, name: &String) {
        let length = self.input_params.len();
        for (param, i) in self.input_params.clone().iter().zip(0..length) {
            if &param.name == name {
                self.input_params.remove(i);
            }
        }
    }
}

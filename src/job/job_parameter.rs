#[derive(Clone, Debug)]
pub struct JobParameter {
    pub name: String,
    pub default: String,
}

impl JobParameter {
    pub fn new(name: String, default: String) -> Self {
        JobParameter { name, default }
    }
}
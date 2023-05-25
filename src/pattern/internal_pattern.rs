#[derive(Clone, Debug)]
pub struct InternalPattern {
    pub tasks_before_output: Vec<String>,
    pub tasks_after_output: Vec<String>,
}

impl InternalPattern {
    pub fn new(tasks_before_output: Vec<String>, tasks_after_output: Vec<String>) -> Self {
        InternalPattern { tasks_before_output, tasks_after_output }
    }
}
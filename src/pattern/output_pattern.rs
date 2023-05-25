#[derive(Clone, Debug, PartialEq)]
pub enum ResponseType {
    Normal,
    Reply,
    Whisper,
}

#[derive(Clone, Debug)]
pub struct OutputPattern {
    pub text: String,
    pub tasks: Vec<String>,
    pub is_me: bool,
    pub response_type: ResponseType,
}

impl OutputPattern {
    pub fn new(text: String, is_me: bool, response_type: ResponseType) -> Self {
        OutputPattern {
            tasks: OutputPattern::generate_tasks_from(&text),
            text,
            is_me,
            response_type,
        }
    }

    pub fn generate_tasks(&mut self) {
        self.tasks = OutputPattern::generate_tasks_from(&self.text)
    }

    // generate tasks based on the text
    pub fn generate_tasks_from(text: &String) -> Vec<String> {
        let mut tasks: Vec<String> = vec![];

        for word in text.split(" ") {
            match word.strip_prefix(('\\')) {
                Some(task) => {tasks.push(task.to_string())}
                _ => {}
            }
        }

        tasks
    }
}


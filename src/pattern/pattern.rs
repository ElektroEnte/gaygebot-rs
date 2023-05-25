use crate::bot::{Input, Output};
use crate::pattern::{Identifier, IdentifierType, InputPattern, InternalPattern, OutputPattern, ResponseType};

#[derive(Clone, Debug)]
pub struct Pattern {
    pub input_pattern: InputPattern,
    pub internal_pattern: InternalPattern,
    pub output_pattern: OutputPattern,
    pub identifier: Identifier,
}

impl Pattern {
    pub fn new(input_pattern: InputPattern, internal_pattern: InternalPattern, output_pattern: OutputPattern, identifier: Identifier) -> Self {
        Pattern {
            input_pattern,
            internal_pattern,
            output_pattern,
            identifier,
        }
    }

    pub fn new_simple_command(identifier: String, response: String, is_me: bool, info: String) -> Self {
        Pattern::new(
            InputPattern::new(vec![], info),
            InternalPattern::new(vec![], vec![]),
            OutputPattern::new(response, is_me, ResponseType::Normal),
            Identifier::new(IdentifierType::Command(identifier)),
        )
    }

    pub fn matches_input(&self, input: &Input) -> bool {
        return match self.identifier.identifier_type.clone() {
            IdentifierType::Any => { true }
            IdentifierType::Command(pattern_identifier) => {
                return match input.identifier.clone() {
                    Some(input_identifier) => {
                        if input_identifier == input_identifier { true } else { false }
                    }
                    None => { false }
                };
            }
            _ => { false }
        };
    }

    pub fn execute_with(&self, input: Input) -> Output {
        let mut output_text = String::new();

        for internal_task in &self.internal_pattern.tasks_before_output { todo!() }

        output_text.push_str(self.output_pattern.text.as_str());

        for external_task in &self.output_pattern.tasks { todo!() }

        for internal_task in &self.internal_pattern.tasks_after_output { todo!() }

        Output { text: output_text, response_type: self.clone().output_pattern.response_type, is_me: self.output_pattern.is_me, input }
    }
}
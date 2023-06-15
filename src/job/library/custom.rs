use std::collections::HashMap;

use crate::bot::Input;
use crate::job::{
    job_parameter::JobParameter,
    job_pattern::JobPattern,
    job::Job,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LevelChangeType {
    Up,
    Down,
}

#[derive(Copy, Clone, Debug)]
pub struct LevelChange {
    index: usize,
    change: LevelChangeType,
}

#[derive(Clone, Debug)]
pub struct LevelMap {
    changes: Vec<LevelChange>,
}

impl From<String> for LevelMap {
    fn from(s: String) -> Self {
        let mut changes: Vec<LevelChange> = vec![];

        let bytes = s.as_bytes();

        for i in 0..s.len() {
            if bytes[i] == "{".as_bytes()[0] {
                changes.push(LevelChange { index: i, change: LevelChangeType::Up });
            } else if bytes[i] == "}".as_bytes()[0] {
                changes.push(LevelChange { index: i, change: LevelChangeType::Down });
            }
        }

        LevelMap { changes }
    }
}

impl LevelMap {
    pub fn get_inner_pair(&mut self) -> Option<(LevelChange, LevelChange)> {
        if self.changes.len() == 0 { return None; }
        for i in 0..self.changes.len() - 1 {
            let current_option = self.changes.get(i);
            let next_option = self.changes.get(i + 1);
            if let (Some(current), Some(next)) = (current_option, next_option) {
                if current.change == LevelChangeType::Up && next.change == LevelChangeType::Down {
                    let return_pair = (*current, *next);
                    for _ in 0..2 { self.changes.remove(i); }
                    return Some(return_pair);
                }
            }
        }

        None
    }
}

pub struct CustomJob {
    pattern: JobPattern,
}

impl Job for CustomJob {
    fn execute(&self, input: &mut Input, params: HashMap<String, String>) -> String {
        let mut output = self.pattern.output_string.to_owned();

        if input.ctx.command_history.contains(&self.pattern.name.to_owned()) {
            return "".to_string();
        } else {
            input.ctx.command_history.push(self.pattern.name.to_owned());
        }

        for param in self.get_params() {
            let mut given_param = params
                .get(param.name.as_str())
                .unwrap_or(&param.default);

            // output = output.replace(
            //     format!("\\arg.{}", param.name).as_str(),
            //     given_param,
            // );

            output = output.replace(
                format!("\\[{}]", param.name).as_str(),
                given_param,
            );
        }

        let mut level_map = LevelMap::from(output.clone());

        loop {
            if let Some((start, end)) = level_map.get_inner_pair() {
                if let Some(inner_content) = output.get(start.index + 1..end.index) {
                    let mut sim_input = Input::from_context_with_custom_text(input.ctx.clone(), inner_content.to_string(), "".to_string());

                    output.replace_range(start.index..end.index + 1, sim_input.get_job_result().as_str());

                    input.ctx.environment = sim_input.ctx.environment;
                }
            } else { break; }

            level_map = LevelMap::from(output.clone());
        }




        // let pattern_parts: Vec<String> = output_pattern.split(&['{', '}'][..]).collect();
        // for n in 0..((pattern_parts.len() as f32) / 2.0).floor() as usize {
        //     let index = (n * 2) + 1;
        //     output_pattern = output_pattern.replace(
        //         format!("{}{}{}", "{", pattern_parts.get(index).unwrap_or(&"".to_string()), "}"),
        //
        //     );
        // }


        // output

        output
    }

    fn get_params(&self) -> Vec<JobParameter> {
        self.pattern.input_params.clone()
    }
}

impl CustomJob {
    // fn fetch_sub_jobs(s: String) -> Vec<String> {}

    pub fn new(pattern: JobPattern) -> Self {
        CustomJob { pattern }
    }
}

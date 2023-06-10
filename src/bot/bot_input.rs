use std::collections::HashMap;

use crate::bot::{Context, Output};
use crate::job::job::Job;
use crate::job::job_manager::JobManager;
use crate::job::job_parameter::JobParameter;
use crate::pattern::ResponseType;

#[derive(Clone, Debug)]
pub struct Input {
    pub text: String,
    pub identifier: Option<String>,
    pub args: Vec<String>,
    pub kwargs: HashMap<String, String>,
    pub ctx: Context,
}

impl From<Context> for Input {
    fn from(ctx: Context) -> Self {
        let mut identifier: Option<String> = None;
        let mut args: Vec<String> = vec![];
        let mut kwargs: HashMap<String, String> = HashMap::new();

        let message_text = ctx.message.text.clone();
        let env_prefix = ctx.environment.prefix.clone();

        if let Some(message_text_no_prefix) = message_text.strip_prefix(&env_prefix) {
            for word in message_text_no_prefix.split(' ').into_iter() {
                if identifier == None {
                    identifier = Some(word.to_string());
                } else {
                    match word.split_once(":") {
                        Some((key, value)) => { kwargs.insert(key.to_string(), value.to_string()); }
                        None => { args.push(word.to_string()) }
                    }
                }
            }
        }

        return Input { text: message_text, identifier, args, kwargs, ctx };
    }
}

impl Input {
    pub fn get_job_result(&mut self) -> String {
        let job_name = &self.identifier.clone().unwrap_or("".to_string());
        let mut job_parameter: HashMap<String, String> = HashMap::new();

        for parameter in JobManager::get_job_params(job_name, &self) {
            let key = parameter.name;

            if self.kwargs.contains_key(key.clone().as_str()) {
                job_parameter.insert(key.clone(), self.kwargs.get(key.as_str()).unwrap().to_owned());
            } else if self.args.len() > 0 {
                job_parameter.insert(key.clone(), self.args.remove(0));
            } else {
                job_parameter.insert(key, parameter.default);
            }
        }

        // println!("{}\n{:?}", job_name, job_parameter);

        JobManager::execute_job(job_name, self, job_parameter)
    }

    pub fn execute_as_job(mut self) -> Output {
        let mut job_result = self.get_job_result();
        let mut response_type: ResponseType = ResponseType::Normal;
        let mut is_me: bool = false;

        // let mut job_result_words: Vec<&str> = job_result.split(" ").collect();
        // // print!("{} | {:?}", job_result, job_result_words);
        // while job_result_words.len() > 0 {
        //     if let Some(word) = job_result_words.pop() {
        //         match word {
        //             "\\me" => { is_me = true; }
        //             "\\notme" => { is_me = false; }
        //             "\\reply" => { response_type = ResponseType::Reply; }
        //             "\\normal" => { response_type = ResponseType::Normal; }
        //             "\\whisper" => { response_type = ResponseType::Whisper; }
        //             _ => {
        //                 text.insert_str(0, word);
        //                 text.insert(0, ' ');
        //             }
        //         }
        //     }
        // }

        let first_me = job_result
            .find("\\me")
            .unwrap_or(usize::MAX);
        let first_notme = job_result
            .find("\\notme")
            .unwrap_or(usize::MAX);

        if first_me < first_notme {
            is_me = true;
        } else {
            is_me = false;
        }

        let first_reply = job_result
            .find("\\reply")
            .unwrap_or(usize::MAX);
        let first_normal = job_result
            .find("\\normal")
            .unwrap_or(usize::MAX);
        let first_whisper = job_result
            .find("\\whisper")
            .unwrap_or(usize::MAX);

        let first_responsetype_index = *vec![first_normal, first_reply, first_whisper]
            .iter()
            .min()
            .unwrap_or(&usize::MAX);

        if first_responsetype_index == first_normal {
            response_type = ResponseType::Normal;
        } else if first_responsetype_index == first_reply {
            response_type = ResponseType::Reply;
        } else if first_responsetype_index == first_whisper {
            response_type = ResponseType::Whisper;
        }

        for p in ["\\me", "\\notme", "\\reply", "\\normal", "\\whisper"] {
            job_result = job_result.replace(p, "");
        }

        Output {
            text: job_result.trim().to_string(),
            response_type,
            is_me,
            input: self,
        }
    }

    // Creates an Input from the given context using a given text and prefix.
    pub fn from_context_with_custom_text(ctx: Context, text: String, prefix: String) -> Self {
        let mut identifier: Option<String> = None;
        let mut args: Vec<String> = vec![];
        let mut kwargs: HashMap<String, String> = HashMap::new();

        let message_text = text;
        let env_prefix = prefix;

        if let Some(message_text_no_prefix) = message_text.strip_prefix(&env_prefix) {
            for word in message_text_no_prefix.split(' ').into_iter() {
                if identifier == None {
                    identifier = Some(word.to_string());
                } else {
                    match word.split_once(":") {
                        Some((key, value)) => { kwargs.insert(key.to_string(), value.to_string()); }
                        None => { args.push(word.to_string()) }
                    }
                }
            }
        }

        return Input { text: message_text, identifier, args, kwargs, ctx };
    }
}
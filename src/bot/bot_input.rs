use std::collections::HashMap;
use crate::bot::Context;

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
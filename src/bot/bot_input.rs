use std::collections::HashMap;
use crate::bot::Context;

#[derive(Clone, Debug)]
pub struct NormalInput {
    pub text: String,
    pub ctx: Context,
}

impl From<Context> for NormalInput {
    fn from(ctx: Context) -> Self {
        NormalInput {
            text: ctx.clone().message.text,
            ctx,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandInput {
    pub text: String,
    pub identifier: String,
    pub args: Vec<String>,
    pub kwargs: HashMap<String, String>,
    pub ctx: Context,
}

impl From<Context> for CommandInput {
    fn from(ctx: Context) -> Self {
        // Get identifier
        let arguments: String;
        let identifier: String;

        let message_text =  ctx.message.text.clone();
        let env_prefix = ctx.bot_env.prefix.clone();

        let identifier_split = message_text.split_once(" ");
        match identifier_split {
            Some(s) => {
                identifier = s.0.to_string().strip_prefix(&env_prefix).unwrap_or_default().to_string();
                arguments = s.1.to_string();
            }
            None => {
                identifier = message_text.strip_prefix(&env_prefix).unwrap_or_default().to_string();
                arguments = String::new();
            }
        }

        let mut args: Vec<String> = vec![];
        let mut kwargs: HashMap<String, String> = HashMap::new();

        for argument in arguments.split(" ") {
            let arg_split = argument.split_once(":");
            match arg_split {
                Some(kw_content) => { kwargs.insert(kw_content.0.to_string(), kw_content.1.to_string()); }
                None => { args.push(argument.to_string()) }
            }
        }

        CommandInput {
            text: ctx.clone().message.text,
            identifier,
            args,
            kwargs,
            ctx: ctx.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Input {
    Normal(NormalInput),
    Command(CommandInput),
}

impl From<Context> for Input {
    fn from(ctx: Context) -> Self {
        match ctx.message.text.strip_prefix(&ctx.bot_env.prefix) {
            Some(_) => { Input::Command(CommandInput::from(ctx.clone())) }
            None => { Input::Normal(NormalInput::from(ctx.clone())) }
        }
    }
}
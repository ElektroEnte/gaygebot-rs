use std::collections::HashMap;
pub use context::Context;
pub use chatter::Chatter;
pub use channel::Channel;
pub use message::Message;
pub use bot_input::{Input};
pub use bot_output::Output;

pub mod context;
pub mod channel;
pub mod message;
pub mod chatter;
pub mod bot_input;
pub mod bot_output;

use tokio;
use twitch_irc::{login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport, TwitchIRCClient, Error};
use twitch_irc::message::PrivmsgMessage;
use crate::bot_env::environment::Environment;
use crate::job::job_manager::JobManager;
use crate::job::job_parameter::JobParameter;
use crate::job::job_pattern::JobPattern;
use crate::pattern::{Identifier, IdentifierType, InputPattern, InternalPattern, OutputPattern, ResponseType};
use crate::pattern::pattern::Pattern;

pub enum LogMode {
    None,
    Debug,
}

#[derive(Clone)]
pub struct Bot {
    pub login: String,
    oauth: Option<String>,
}

impl Bot {
    pub fn new(login: String, oauth: Option<String>) -> Self {
        Bot {
            login,
            oauth,
        }
    }

    pub fn generate_config(&self) -> ClientConfig<StaticLoginCredentials> {
        ClientConfig::new_simple(StaticLoginCredentials::new(self.login.to_owned(), self.oauth.to_owned()))
    }

    /// runs the bot instance
    pub async fn run(&mut self, initial_channels: Vec<String>, log_mode: LogMode) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        let config = self.generate_config();
        let (mut incoming_messages, mut client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        let mut envs= HashMap::new();
        envs.insert("default".to_string(), Environment::default());

        let join_handle = tokio::spawn(async move {
            for channel in initial_channels {
                client.join(channel).unwrap();
            }

            while let Some(message) = incoming_messages.recv().await {
                match log_mode {
                    LogMode::Debug => { println!("{:?}", message) }
                    LogMode::None => {}
                };

                match message {
                    ServerMessage::Privmsg(msg) => {
                        Bot::on_private_message(&mut client, msg, &mut envs).await?;
                    }
                    _ => {}
                };
            };

            Ok(())
        });

        join_handle.await.unwrap()?;

        Ok(())
    }

    async fn on_private_message(client: &mut TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>, msg: PrivmsgMessage, envs: &mut HashMap<String, Environment>) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        // let mut test_pattern = JobPattern::new("test".to_string(), "\\me\\replytest: Arg 1: '\\arg.1'".to_string());
        // test_pattern.input_params.push(JobParameter::new("1".to_string(), "default arg 1".to_string()));
        // test_env.patterns.push(test_pattern);
        //
        // let mut one_pattern = JobPattern::new("combineargs".to_string(), "\\arg.1\\arg.2".to_string());
        // one_pattern.input_params.push(JobParameter::new("1".to_string(), "arg1".to_string()));
        // one_pattern.input_params.push(JobParameter::new("2".to_string(), "arg2".to_string()));
        // test_env.patterns.push(one_pattern);
        //
        // let mut pipe_test_pattern = JobPattern::new("pipetest".to_string(), "result from piping combine args to test: '{test {combineargs \\arg.1 \\arg.2 }}'".to_string());
        // pipe_test_pattern.input_params.push(JobParameter::new("1".to_string(), "pipe_arg1".to_string()));
        // pipe_test_pattern.input_params.push(JobParameter::new("2".to_string(), "pipe_arg2".to_string()));
        // test_env.patterns.push(pipe_test_pattern);
        //
        // let mut ping_pipe_pattern = JobPattern::new("jobpipe".to_string(), "piped ping: '{ping}'".to_string());
        // test_env.patterns.push(ping_pipe_pattern);
        //
        // let mut pipe1 = JobPattern::new("pipe1".to_string(), "pipe pipe2: '{pipe2}'".to_string());
        // test_env.patterns.push(pipe1);
        //
        // let mut pipe2 = JobPattern::new("pipe2".to_string(), "pipe pipe1: '{pipe1}'".to_string());
        // test_env.patterns.push(pipe2);

        // let mut dinkdonk = JobPattern::new("dinkdonk".to_string(), "\\reply".to_string());
        // dinkdonk.identifier = Identifier::new(IdentifierType::Username("mzntori".to_string()));
        // test_env.patterns.push(dinkdonk);

        let mut ctx = Context::from(msg);
        ctx.environment = envs.get("default").unwrap_or(&Environment::default()).clone();
        let mut input = Input::from(ctx);
        let mut output = input.execute_as_job();

        output.send_in_context_using(client).await?;

        envs.insert("default".to_string(), output.input.ctx.environment);

        Ok(())
    }
}
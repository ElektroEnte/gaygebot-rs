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
                        Bot::on_private_message(&mut client, msg).await?;
                    }
                    _ => {}
                };
            };

            Ok(())
        });

        join_handle.await.unwrap()?;

        Ok(())
    }

    async fn on_private_message(client: &mut TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>, msg: PrivmsgMessage) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        let ctx = Context::from(msg);
        let input = Input::from(ctx.clone());

        let ping_pattern: Pattern = Pattern::new_simple_command("ping".to_string(), "pong".to_string(), false, "pings and pongs".to_string());

        if ping_pattern.matches_input(&input) {
            ping_pattern.execute_with(input).send_in_context_using(client).await?;
        }

        Ok(())
    }
}
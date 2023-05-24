pub use context::Context;
pub use chatter::Chatter;
pub use channel::Channel;
pub use message::Message;
pub use bot_input::{CommandInput, NormalInput, Input};
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
        let ctx =  Context::from(msg);
        let cmd_input = Input::from(ctx.clone());

        println!("{:?}", cmd_input);

        match cmd_input {
            Input::Normal(input) => {},
            Input::Command(input) => {
                if input.identifier == "ping" {
                    client.say(ctx.channel.login, "pong".to_string()).await?;
                }

                else if input.identifier == "debug" {
                    let mut say_string = String::new();

                    // push identifier
                    say_string.push_str(format!("i: {} - ", input.identifier).as_str());

                    // push normal args
                    say_string.push_str("a: ");
                    for arg in input.args {
                        say_string.push_str(format!("{}, ", arg.as_str()).as_str());
                    }
                    say_string.push_str("- ");

                    // push keyword args
                    say_string.push_str("kw: ");
                    for (key, value) in input.kwargs {
                        say_string.push_str(format!("{}:{}, ", key, value).as_str());
                    }

                    client.say(ctx.channel.login, say_string).await?;
                }
            },
        }

        Ok(())
    }
}
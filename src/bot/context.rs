use twitch_irc::message::PrivmsgMessage;
use twitch_irc::{Error, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use crate::bot::{Chatter, Channel, Message};
use crate::bot_env::environment::Environment;

#[derive(Clone, Debug)]
pub struct Context {
    pub source: PrivmsgMessage,
    pub chatter: Chatter,
    // id? FeelsDankMan
    pub channel: Channel,
    pub message: Message,
    pub bot_env: Environment,
}

impl Context {
    pub async fn send(&self, client:  &mut TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>, s: String) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        let channel = self.channel.login.clone();
        client.say(channel, s).await?;
        Ok(())
    }
}

impl From<PrivmsgMessage> for Context {
    fn from(privmsg: PrivmsgMessage) -> Self {
        Context {
            source: privmsg.clone(),
            chatter: Chatter::from(privmsg.clone()),
            channel: Channel::from(privmsg.clone()),
            message: Message::from(privmsg.clone()),
            bot_env: Environment::default(), // Change to using actual environments once ready
        }
    }
}
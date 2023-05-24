use twitch_irc::message::PrivmsgMessage;
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
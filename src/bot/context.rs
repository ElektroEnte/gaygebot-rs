use twitch_irc::message::PrivmsgMessage;
use crate::bot::{Chatter, Channel, Message};
use crate::bot_env::environment::Environment;

pub struct Context {
    pub source: PrivmsgMessage,
    pub chatter: Chatter,
    // id? FeelsDankMan
    pub channel: Channel,
    pub message: Message,
    pub bot_env: Environment,
}
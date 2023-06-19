use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use async_std::task;

use tokio::sync::Mutex;

use twitch_irc::message::PrivmsgMessage;
use twitch_irc::{Error, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;

use crate::bot::{Chatter, Channel, Message};
use super::env::environment::Environment;

#[derive(Clone, Debug)]
pub struct Context {
    pub source: PrivmsgMessage,
    pub chatter: Chatter,
    // id? FeelsDankMan
    pub channel: Channel,
    pub message: Message,
    pub environment: Arc<Mutex<Environment>>,
    pub command_history: Vec<String>,
}

impl Context {
    pub fn set_env(&mut self, env: &Arc<Mutex<Environment>>) {
        self.environment = Arc::clone(env);
    }
    
    pub async fn say(&self, client: Arc<Mutex<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>>, s: String) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        let c = client.lock().await;
        c.say(self.channel.login.clone(), s).await?;
        Ok(())
    }

    pub async fn say_in_reply_to(&self, client: Arc<Mutex<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>>, s: String) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        let c = client.lock().await;
        c.say_in_reply_to(&self.source, s).await?;
        Ok(())
    }

    pub async fn me(&self, client: Arc<Mutex<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>>, s: String) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        let c = client.lock().await;
        c.me(self.channel.login.clone(), s).await?;
        Ok(())
    }

    pub async fn me_in_reply_to(&self, client: Arc<Mutex<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>>, s: String) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        let c = client.lock().await;
        c.me_in_reply_to(&self.source, s).await?;
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
            environment: Arc::new(Mutex::new(Environment::default())), // Change to using actual environments once ready
            command_history: vec![],
        }
    }
}
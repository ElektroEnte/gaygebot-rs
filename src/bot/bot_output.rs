use std::sync::Arc;

use tokio::sync::Mutex;

use twitch_irc::{Error, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;

use crate::bot::Input;
use super::ResponseType;

#[derive(Clone, Debug)]
pub struct Output {
    pub text: String,
    pub response_type: ResponseType,
    pub is_me: bool,
    pub input: Input,
}

impl Output {
    pub async fn send_in_context_using(&mut self, client: Arc<Mutex<TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>>>) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        // match self.response_type {
        //     ResponseType::Normal => {
        //         if self.is_me {
        //             self.input.ctx.me(client, self.text.clone()).await?
        //         } else {
        //             self.input.ctx.say(client, self.text.clone()).await?
        //         }
        //     }
        //     ResponseType::Reply => {
        //         if self.is_me {
        //             self.input.ctx.me_in_reply_to(client, self.text.clone()).await?
        //         } else {
        //             self.input.ctx.say_in_reply_to(client, self.text.clone()).await?
        //         }
        //     }
        //     _ => {}
        // }
        
        let response_type = self.response_type;
        let is_me = self.is_me;
        let ctx = self.input.ctx.clone();
        let text = self.text.clone();
    
        match (response_type, is_me) {
            (ResponseType::Normal, true) => { ctx.me(client, text).await }
            (ResponseType::Normal, false) => { ctx.say(client, text).await }
            (ResponseType::Reply, true) => { ctx.me_in_reply_to(client, text).await }
            (ResponseType::Reply, false) => { ctx.say_in_reply_to(client, text).await }
            _ => {Ok(())}
        }?;
        
        Ok(())
    }
}


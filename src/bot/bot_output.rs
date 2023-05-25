use twitch_irc::{Error, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use crate::bot::Input;
use crate::pattern::ResponseType;

#[derive(Clone, Debug)]
pub struct Output {
    pub text: String,
    pub response_type: ResponseType,
    pub is_me: bool,
    pub input: Input,
}

impl Output {
    pub async fn send_in_context_using(&mut self, client: &mut TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>) -> Result<(), Error<SecureTCPTransport, StaticLoginCredentials>> {
        match self.response_type {
            ResponseType::Normal => {
                if self.is_me {
                    self.input.ctx.me(client, self.text.clone()).await?
                } else {
                    self.input.ctx.say(client, self.text.clone()).await?
                }
            }
            ResponseType::Reply => {
                if self.is_me {
                    self.input.ctx.me_in_reply_to(client, self.text.clone()).await?
                } else {
                    self.input.ctx.say_in_reply_to(client, self.text.clone()).await?
                }
            }
            _ => {}
        }

        Ok(())
    }
}


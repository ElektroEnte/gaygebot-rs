use twitch_irc::message::PrivmsgMessage;

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: String,
    pub login: String,
}

impl Channel {
    pub fn new(id: String, login: String) -> Self {
        Channel { id, login }
    }
}

impl Default for Channel {
    fn default() -> Self {
        Channel {
            id: "0".to_string(),
            login: "login".to_string(),
        }
    }
}

impl From<PrivmsgMessage> for Channel {
    fn from(privmsg: PrivmsgMessage) -> Self {
        Channel {
            id: privmsg.channel_id,
            login: privmsg.channel_login,
        }
    }
}
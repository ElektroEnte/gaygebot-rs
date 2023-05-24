use twitch_irc::message::PrivmsgMessage;

#[derive(Clone)]
pub struct Channel {
    pub id: String,
    pub login: String,
}

impl Channel {
    pub fn new(id: String, login: String) -> Self {
        Channel { id, login }
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
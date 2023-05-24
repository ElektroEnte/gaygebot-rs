use twitch_irc::message::{Badge, PrivmsgMessage, RGBColor};

#[derive(Clone)]
pub struct Chatter {
    pub id: String,
    pub login: String,
    pub username: String,
    pub color: Option<RGBColor>,
    pub badges: Vec<Badge>,
}

impl From<PrivmsgMessage> for Chatter {
    fn from(privmsg: PrivmsgMessage) -> Self {
        Chatter {
            id: privmsg.sender.id,
            login: privmsg.sender.login,
            username: privmsg.sender.name,
            color: privmsg.name_color,
            badges: privmsg.badges,
        }
    }
}
use twitch_irc::message::PrivmsgMessage;

#[derive(Clone)]
pub struct Message {
    pub id: String,
    pub text: String,
    pub is_me: bool,
    pub bits: Option<u64>,
    // pub timestamp
}

impl Default for Message {
    fn default() -> Self {
        Message {
            id: "0".to_string(),
            text: "".to_string(),
            is_me: false,
            bits: None,
        }
    }
}

impl From<PrivmsgMessage> for Message {
    fn from(privmsg: PrivmsgMessage) -> Self {
        Message {
            id: privmsg.message_id,
            text: privmsg.message_text,
            is_me: privmsg.is_action,
            bits: privmsg.bits,
        }
    }
}
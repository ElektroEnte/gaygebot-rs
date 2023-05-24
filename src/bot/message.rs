use twitch_irc::message::PrivmsgMessage;

#[derive(Clone)]
pub struct Message {
    pub text: String,
    pub is_me: bool,
    pub bits: Option<u64>,
    pub message_id: String,
    // pub timestamp
}

impl From<PrivmsgMessage> for Message {
    fn from(privmsg: PrivmsgMessage) -> Self {
        Message {
            text: privmsg.message_text,
            is_me: privmsg.is_action,
            bits: privmsg.bits,
            message_id: privmsg.message_id,
        }
    }
}
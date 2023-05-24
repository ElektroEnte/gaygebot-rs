pub enum ResponseType {
    Normal,
    Reply,
    Whisper,
}

pub struct OutputPattern {
    pub raw: String,
    pub tasks: String,
    pub is_me: bool,
    pub response_type: ResponseType,
}
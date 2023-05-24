pub enum IdentifierType {
    Any,
    Command(String),
    Regex(String),
    Username(String),
    UserID(String),
    //UserRole
}

pub struct Identifier {
    pub identifier_type: IdentifierType
}
#[derive(Clone, Debug)]
pub enum IdentifierType {
    Any,
    Command(String),
    Regex(String),
    Username(String),
    UserID(String),
    //UserRole
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub identifier_type: IdentifierType,
}

impl Identifier {
    pub fn new(identifier_type: IdentifierType) -> Self {
        Identifier { identifier_type }
    }
}
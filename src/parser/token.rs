use crate::parser::location::Location;
use crate::parser::token_kind::TokenKind;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub struct Token {
    kind: TokenKind,
    location: Location,
    lexeme: String
}

#[allow(dead_code)]
impl Token {

    pub fn kind(&self) -> TokenKind {
        self.kind
    }
    pub fn location(&self) -> Location {
        self.location
    }
    pub fn lexeme(&self) -> &str {
        self.lexeme.as_str()
    }
    pub fn new(kind: TokenKind, location: Location, lexeme: impl AsRef<str>) -> Self {
        Self {
            kind,
            location,
            lexeme: lexeme.as_ref().to_string(),
        }
    }

    pub fn un_located(kind: TokenKind, lexeme: impl AsRef<str>) -> Self {
        Self {
            kind,
            location: Location::new(0, 0),
            lexeme: lexeme.as_ref().to_string(),
        }
    }

    pub fn content_equal(&self, rhs: &Self) -> bool {
        self.kind == rhs.kind && self.lexeme == rhs.lexeme
    }
}
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::parser::token::Token;

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    UnexpectedToken,
    InvalidName
}

impl Display for ParserErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParserErrorKind::UnexpectedToken => "Unknown Token",
            ParserErrorKind::InvalidName => {"Invalid Name"}
        })
    }
}

#[derive(Debug, Clone)]
pub struct ParserError {
    kind: ParserErrorKind,
    offending_token: Token,
    message: Option<String>
}

impl ParserError {
    pub fn new(
        kind: ParserErrorKind,
        offending_token: Token,
        message: Option<String>
    ) -> Self {
        Self {
            kind,
            offending_token,
            message,
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = self.message
            .clone()
            .map(|x| format!("\n\t{}", x))
            .unwrap_or_else(|| String::from(""));

        write!(f, "[ERROR {}]: {} at ({}, {}){}",
               self.kind,
               self.offending_token.lexeme(),
               self.offending_token.location().row(),
               self.offending_token.location().column(),
               message
        )
    }
}

impl Error for ParserError {}
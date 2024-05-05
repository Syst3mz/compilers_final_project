use crate::parser::token::Token;
use crate::parser::token_kind::TokenKind;
use crate::parser::token_kind::TokenKind::EOI;

#[derive(Debug)]
pub struct TokenHolder {
    tokens: Vec<Token>,
    index: usize
}
#[allow(dead_code)]
impl TokenHolder {
    pub fn new(tokens: Vec<Token>) -> TokenHolder {
        Self {
            tokens,
            index: 0,
        }
    }

    pub fn current(&self) -> Token {
        if let Some(t) = self.tokens.get(self.index) {
            t.clone()
        } else {
            Token::un_located(EOI, "")
        }
    }

    pub fn empty(&self) -> bool {
        match self.current().kind() {
            EOI => true,
            _ => false
        }
    }
    pub fn previous(&self) -> Token {self.tokens[self.index - 1].clone()}

    pub fn rewind(&mut self) {
        self.index -= 1;
    }

    /// Test if the current token is a specified kind.
    pub fn expect(&mut self, kind: TokenKind) -> Option<Token> {
        if self.tokens[self.index].kind() == kind {
            return Some(self.tokens[self.index].clone());
        }

        None
    }

    /// Called t_match b/c match is a reserved keyword.
    pub fn t_match(&mut self, kind: TokenKind) -> Option<Token> {
        if let Some(t) = self.expect(kind) {
            self.index += 1;
            return Some(t);
        }

        None
    }
}

impl Iterator for TokenHolder {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.current();
        self.index += 1;
        return Some(ret);
    }
}


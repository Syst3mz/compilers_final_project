use crate::parser::lex_table::LEX_TABLE;
use crate::parser::location::Location;
use crate::parser::token::Token;
use crate::parser::token_kind::TokenKind;
use crate::parser::token_kind::TokenKind::*;

pub struct Lexer {
    text: String,
    index: usize,
    tokens: Vec<Token>,
    row: usize,
    column: usize,
}

impl Lexer {
    pub fn new(text: impl AsRef<str>) -> Self {
        Self {
            text: text.as_ref().to_string(),
            index: 0,
            tokens: vec![],
            row: 1,
            column: 1,
        }
    }
    fn take_while(&mut self, predicate: fn(char)->bool) -> Option<String> {
        let mut accumulator = String::from("");
        for c in self.text[self.index..].chars() {
            if predicate(c) {
                accumulator.push(c)
            }
            else {
                break;
            }
        }

        return if accumulator.is_empty() { None } else {Some(accumulator)}
    }

    fn take_name(&mut self) -> Option<String> {
        let first_char = self.text[self.index..].chars().next().unwrap();
        if first_char != '_' && !first_char.is_alphabetic() {
            return None;
        }
        let first_char = String::from(first_char);
        self.index += 1;

        let next_chars = self.take_while(|x| x == '_' || x.is_alphanumeric());
        if next_chars.is_none() {
            return Some(first_char);
        }

        return Some(format!("{}{}", first_char, next_chars.unwrap()))
    }

    fn accept_token(&mut self, kind: TokenKind, lexeme: impl AsRef<str>) {
        let lexeme = lexeme.as_ref().to_string();
        let start_column = self.column;
        self.column += lexeme.len();
        self.index += lexeme.len();

        self.tokens.push(Token::new(
            kind,
            Location::new(self.row, start_column),
            lexeme
        ))
    }

    fn run_lexer(&mut self) {
        'outer: while self.index < self.text.len() {
            for (rep, kind) in LEX_TABLE {
                if self.text[self.index..].starts_with(rep) {
                    self.accept_token(kind, rep);
                    continue 'outer;
                }
            }

            if let Some(int) = self.take_while(|x| x.is_numeric()){
                self.accept_token(Int, int);
                continue
            }

            if let Some(name) = self.take_name() {
                self.accept_token(Name, name);
                continue
            }

            if self.text[self.index..].starts_with("\n") {
                self.row += 1;
                self.column = 1;
            }
            self.index += 1;
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        self.run_lexer();
        self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let text = "1234";
        let mut lexer = Lexer::new(text);
        lexer.run_lexer();
        assert!(lexer.tokens[0].content_equal(&Token::un_located(Int, text)))
    }

    #[test]
    fn name() {
        let text = "cat";
        let mut lexer = Lexer::new(text);
        lexer.run_lexer();
        assert!(lexer.tokens[0].content_equal(&Token::un_located(Name, text)))
    }

    #[test]
    fn name_and_number() {
        let text = "cat 123";
        let tokens = Lexer::new(text).lex();
        assert!(
            tokens[0].content_equal(&Token::un_located(Name, "cat")) &&
            tokens[1].content_equal(&Token::un_located(Int, "123"))
        )
    }

    #[test]
    fn number_and_name() {
        let text = "123 cat";
        let tokens = Lexer::new(text).lex();
        assert!(
            tokens[0].content_equal(&Token::un_located(Int, "123")) &&
            tokens[1].content_equal(&Token::un_located(Name, "cat"))
        )
    }

    #[test]
    fn simple_math() {
        let text = "123 + 456 - sam";
        let tokens = Lexer::new(text).lex();
        assert!(tokens[0].content_equal(&Token::un_located(Int, "123")));
        assert!(tokens[1].content_equal(&Token::un_located(Plus, "+")));
        assert!(tokens[2].content_equal(&Token::un_located(Int, "456")));
        assert!(tokens[3].content_equal(&Token::un_located(Minus, "-")));
        assert!(tokens[4].content_equal(&Token::un_located(Name, "sam")));
    }

    #[test]
    fn new_line() {
        let text = "fn\nfn";
        let tokens = Lexer::new(text).lex();
        assert_eq!(tokens, vec![
            Token::new(TokenKind::Fn, Location::new(1, 1), "fn"),
            Token::new(TokenKind::Fn, Location::new(2, 1), "fn")
        ])
    }
}
use std::fmt::Debug;
use std::ops::Range;
use std::str::FromStr;
use crate::testing::s_expr::SExpr::Function;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum SExpr<T> {
    Value(T),
    Function(T, Vec<SExpr<T>>)
}

impl<T> SExpr<T> {

    /// Releases the first element of a SExpr, discarding the rest of the tree.
    pub fn release(self) -> T {
        match self {
            SExpr::Value(t) => t,
            SExpr::Function(t, _) => t
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Token<'a> {
    LParen, RParen, Value(&'a str)
}


impl<T: FromStr + Debug> SExpr<T> where <T as FromStr>::Err: Debug {

    fn parse_rec<'a>(tokens: &mut impl Iterator<Item=Token<'a>>) -> Option<SExpr<T>> {
        let token = tokens.next();
        if token.is_none() {
            return None
        }
        let token = token.unwrap();

        match token {
            Token::RParen => None,
            Token::Value(t) => Some(SExpr::Value(T::from_str(t).unwrap())),
            Token::LParen => {
                let mut args = vec![];
                while let Some(n) = Self::parse_rec(tokens) {
                    args.push(n)
                }

                let func_name = args.remove(0);
                return Some(Function(func_name.release(), args));
            }
        }

    }

    fn tokenize(text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let chars = text.chars();
        let mut range: Option<Range<usize>> = None;


        fn handle_range<'a>(range: &mut Option<Range<usize>>, text: &'a str, tokens: &mut Vec<Token<'a>>) {
            if range.is_some() {
                tokens.push(Token::Value(&text[std::mem::replace(range, None).unwrap()]));
            }
        }

        for (index, char) in chars.enumerate() {
            if char == '(' {
                handle_range(&mut range, text, &mut tokens);
                tokens.push(Token::LParen);
                continue
            }

            if char == ')' {
                handle_range(&mut range, text, &mut tokens);
                tokens.push(Token::RParen);
                continue
            }

            if char.is_whitespace() {
                handle_range(&mut range, text, &mut tokens);
                continue
            }

            if range.is_none() {
                range = Some(index..index+1)
            } else {
                range = range.map(|x| x.start..x.end + 1)
            }
        }

        if let Some(range) = range {
            tokens.push(Token::Value(&text[range]))
        }

        return tokens
    }

    pub fn parse(text: impl AsRef<str>) -> SExpr<T> {
        let mut tokens = Self::tokenize(text.as_ref()).into_iter().peekable();
        return Self::parse_rec(&mut tokens).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_word() {
        let text = "cat";
        let tokens = SExpr::<String>::tokenize(text);
        assert_eq!(tokens, vec![Token::Value("cat")])
    }

    #[test]
    fn token_function() {
        let text = "(+ 1 2)";
        let tokens = SExpr::<String>::tokenize(text);
        assert_eq!(tokens, vec![
            Token::LParen,
            Token::Value("+"),
            Token::Value("1"),
            Token::Value("2"),
            Token::RParen
        ])
    }

    #[test]
    fn token_function_nested() {
        let text = "(+ (- 1 2) 3)";
        let tokens = SExpr::<String>::tokenize(text);
        assert_eq!(tokens, vec![
            Token::LParen,
            Token::Value("+"),
            Token::LParen,
            Token::Value("-"),
            Token::Value("1"),
            Token::Value("2"),
            Token::RParen,
            Token::Value("3"),
            Token::RParen
        ])
    }

    #[test]
    fn no_rec() {
        let text = "cat";
        let test = SExpr::parse(text);
        assert_eq!(test, SExpr::Value(String::from("cat")))
    }

    #[test]
    fn func_call() {
        let text = "(+ 1 2)";
        let test = SExpr::parse(text);
        assert_eq!(test,
                   SExpr::Function(String::from("+"), vec![
                       SExpr::Value(String::from("1")),
                       SExpr::Value(String::from("2")),
                   ])
        )
    }

    #[test]
    fn nested_func() {
        let text = "(+ (- 1 2) 3)";
        let test = SExpr::parse(text);
        assert_eq!(test,
                   SExpr::Function(String::from("+"), vec![
                       SExpr::Function(String::from("-"), vec![
                           SExpr::Value(String::from("1")),
                           SExpr::Value(String::from("2")),
                       ]),
                       SExpr::Value(String::from("3")),
                   ])
        )
    }
}
use thiserror::Error;
use crate::ast::statement::Statement;

pub mod lexer;
mod lex_table;
pub mod location;
pub mod token;
pub mod token_kind;

#[derive(Debug, Error)]
pub enum ParserErr {

}
pub struct Parser {

}



impl Parser {
    pub fn parse(text: impl AsRef<str>) -> Result<Vec<Statement>, ParserErr>{
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::s_expr::SExpr;
    use crate::testing::to_s_expr::ToSExpr;
    use super::*;

    fn to_s_expr(statements: Vec<Statement>) -> Vec<SExpr<String>> {
        statements.into_iter().map(|x| x.to_s_expr()).collect()
    }

    #[test]
    fn one() {
        let text = "1";
        let ast = Parser::parse(text).unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("1")])
    }

    #[test]
    fn simple_math() {
        let text = "1 + 2";
        let ast = Parser::parse(text).unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(+ 1 2)")])
    }

    #[test]
    fn ordered_math() {
        let text = "1 + 2 + 3";
        let ast = Parser::parse(text).unwrap();
        assert_eq!(to_s_expr(ast), vec![SExpr::parse("(+ (+ 1 2) 3)")])
    }
}

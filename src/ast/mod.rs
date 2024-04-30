pub mod unary_operator;
pub mod binary_operator;
pub mod statement;
pub mod expression;
pub mod ast_type;

use crate::ast::statement::Statement;
use crate::testing::s_expr::SExpr;
use crate::testing::to_s_expr::ToSExpr;


pub type Block = Vec<Statement>;

impl ToSExpr for Block {
    fn to_s_expr(self) -> SExpr<String> {
        let mut args: Vec<SExpr<String>> = self.into_iter().map(|x| x.to_s_expr()).collect();

        let (first, mut released_args) = args.remove(0).release();
        released_args.append(&mut args);
        SExpr::Function(first, released_args)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::expression::Expression;
    use crate::parser::token::Token;
    use crate::parser::token_kind::TokenKind;
    use super::*;

    #[test]
    fn block_if() {
        let if_internals = Expression::If {
            condition: Box::new(Expression::Name(Token::un_located(TokenKind::Name, "y"))),
            true_block: vec![Statement::Expression(Expression::Name(Token::un_located(TokenKind::Name, "z")))],
            else_block: None,
        };
        let b: Block = vec![Statement::Expression(
            if_internals.clone()
        )];

        let s_expr = b.to_s_expr();
        assert_eq!(s_expr, SExpr::parse("(if y (z))"))
    }
}




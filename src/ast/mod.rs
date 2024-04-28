pub mod unary_operator;
pub mod binary_operator;
pub mod statement;
pub mod expression;

use crate::ast::statement::Statement;
use crate::testing::s_expr::SExpr;
use crate::testing::to_s_expr::ToSExpr;


pub type Block = Vec<Statement>;

impl ToSExpr for Block {
    fn to_s_expr(self) -> SExpr<String> {
        let mut args: Vec<SExpr<String>> = self.into_iter().map(|x| x.to_s_expr()).collect();
        let first = args.remove(0);
        SExpr::Function(first.release(), args)
    }
}
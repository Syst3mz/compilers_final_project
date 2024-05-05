use crate::testing::s_expr::SExpr;
use crate::testing::to_s_expr::ToSExpr;
use crate::typed_ast::r#type::Type;
use crate::typed_ast::typed_statement::TypedStatement;


pub mod r#type;
pub mod typed_statement;
pub mod typed_expression;

#[derive(Debug, Clone)]
pub struct TypedBlock {
    pub body: Vec<TypedStatement>,
    pub type_: Type,
}

impl ToSExpr for TypedBlock {
    fn to_s_expr(self) -> SExpr<String> {
        if self.body.len() == 0 {
            return SExpr::Function(String::from("empty_block"), vec![]);
        }
        let mut args: Vec<SExpr<String>> = self.body.into_iter().map(|x| x.to_s_expr()).collect();

        let (first, mut released_args) = args.remove(0).release();
        released_args.append(&mut args);
        SExpr::Function(first, released_args)
    }
}
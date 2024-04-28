use crate::ast::Block;
use crate::ast::expression::Expression;
use crate::parser::token::Token;
use crate::testing::s_expr::SExpr;
use crate::testing::to_s_expr::ToSExpr;

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration {
        name: Token,
        type_: Token,
        value: Expression
    },
    FunctionDefinition {
        name: Token,
        type_: Token,
        arg_list: Vec<(Token, Token)>,
        body: Block
    },
    Assignment {
        to: Token,
        value: Expression
    },
    While {
        condition: Expression,
        body: Block
    },
    Return(Expression),
    Expression(Expression)
}
impl ToSExpr for Statement {
    fn to_s_expr(self) -> SExpr<String> {
        type S = Statement;
        match self {
            S::VariableDeclaration { name, type_, value } => {
                SExpr::Function(String::from("variable_declaration"), vec![
                    SExpr::Value(format!("{}: {}", name.lexeme(), type_.lexeme())),
                    value.to_s_expr()
                ])
            }
            S::FunctionDefinition { name, type_, arg_list, body } => {
                let mut args = vec![SExpr::Value(name.lexeme().to_string())];
                for t in arg_list
                    .into_iter()
                    .map(|(name, type_)| {
                        SExpr::Value(format!("{}: {}", name.lexeme(), type_.lexeme()))
                    }) {
                    args.push(t)
                }
                args.push(body.to_s_expr());
                args.push(SExpr::Value(format!("->{}", type_.lexeme())));

                SExpr::Function(String::from("function_define"), args)
            }
            S::Assignment { to, value } => {
                SExpr::Function(String::from("="), vec![
                    SExpr::Value(to.lexeme().to_string()), value.to_s_expr()])
            }
            S::While { condition, body } => {
                SExpr::Function(String::from("while"), vec![condition.to_s_expr(), body.to_s_expr()])
            }
            S::Return(e) | S::Expression(e) => e.to_s_expr(),
        }
    }
}
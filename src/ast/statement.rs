use crate::ast::ast_type::AstType;
use crate::ast::Block;
use crate::ast::expression::Expression;
use crate::parser::token::Token;
use crate::testing::s_expr::SExpr;
use crate::testing::to_s_expr::ToSExpr;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        name: Token,
        type_: AstType,
        value: Expression
    },
    FunctionDefinition {
        name: Token,
        type_: AstType,
        arg_list: Vec<(Token, AstType)>,
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
                    SExpr::Value(format!("{}:{}", name.lexeme(), type_.to_string())),
                    value.to_s_expr()
                ])
            }
            S::FunctionDefinition { name, type_, arg_list, body } => {
                let mut args = vec![SExpr::Value(name.lexeme().to_string())];
                for t in arg_list
                    .into_iter()
                    .map(|(name, ast_type)| {
                        SExpr::Value(format!("{}:{}", name.lexeme(), ast_type))
                    }) {
                    args.push(t)
                }
                args.push(body.to_s_expr());
                args.push(SExpr::Value(format!("->{}", type_.to_string())));

                SExpr::Function(String::from("function_define"), args)
            }
            S::Assignment { to, value } => {
                SExpr::Function(String::from("="), vec![
                    SExpr::Value(to.lexeme().to_string()),
                    value.to_s_expr(),
                ])
            }
            S::While { condition, body } => {
                SExpr::Function(String::from("while"), vec![condition.to_s_expr(), body.to_s_expr()])
            }
            S::Return(e) => SExpr::Function(String::from("return"), vec![e.to_s_expr()]),
            S::Expression(e) => e.to_s_expr(),
        }
    }
}
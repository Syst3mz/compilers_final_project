use crate::ast::expression::Expression;
use crate::ast::statement::FunctionDefinition;
use crate::parser::token::Token;
use crate::testing::s_expr::SExpr;
use crate::testing::to_s_expr::ToSExpr;
use crate::typed_ast::typed_expression::TypedExpression;
use crate::typed_ast::r#type::Type;
use crate::typed_ast::TypedBlock;
#[derive(Debug, Clone)]
pub enum TypedStatement {
    VariableDeclaration {
        name: Token,
        type_: Type,
        value: TypedExpression
    },
    FunctionDefinitionStatement(FunctionDefinition),
    Assignment {
        to: Token,
        value: TypedExpression
    },
    While {
        condition: TypedExpression,
        body: TypedBlock
    },
    Return(TypedExpression),
    Expression(TypedExpression)
}
impl ToSExpr for TypedStatement {
    fn to_s_expr(self) -> SExpr<String> {
        type S = TypedStatement;
        match self {
            S::VariableDeclaration { name, type_, value } => {
                SExpr::Function(String::from("variable_declaration"), vec![
                    SExpr::Value(format!("{}:{}", name.lexeme(), type_.to_string())),
                    value.to_s_expr()
                ])
            }
            S::FunctionDefinitionStatement(def) => {
                let mut args = vec![SExpr::Value(def.name.lexeme().to_string())];
                for t in def. arg_list
                    .into_iter()
                    .map(|(name, ast_type)| {
                        SExpr::Value(format!("{}:{}", name.lexeme(), ast_type))
                    }) {
                    args.push(t)
                }
                args.push(def.body.to_s_expr());
                args.push(SExpr::Value(format!("->{}", def.type_.to_string())));

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
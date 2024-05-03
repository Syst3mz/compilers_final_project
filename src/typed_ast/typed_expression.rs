use crate::ast::binary_operator::BinaryOperator;
use crate::ast::Block;
use crate::ast::unary_operator::UnaryOperator;
use crate::parser::token::Token;
use crate::testing::s_expr::SExpr;
use crate::testing::to_s_expr::ToSExpr;
use crate::typed_ast::r#type::Type;

#[derive(Debug, Clone)]
pub enum TypedExpression {
    If {
        condition: Box<TypedExpression>,
        true_block: Block,
        else_block: Option<Block>,
    },
    BinaryOperation {
        lhs: Box<TypedExpression>,
        operator: BinaryOperator,
        rhs: Box<TypedExpression>
    },
    FunctionCall {
        name: Token,
        arguments: Vec<TypedExpression>
    },
    UnaryOperation {
        operator: UnaryOperator,
        rhs: Box<TypedExpression>
    },
    Int(Token),
    Bool(bool, Token),
    List(Vec<TypedExpression>),
    Name(Token, Type)
}

impl ToSExpr for TypedExpression {
    fn to_s_expr(self) -> SExpr<String> {
        match self {
            TypedExpression::If { condition, true_block, else_block } => {
                let t_block = true_block.to_s_expr();
                let mut args = vec![condition.to_s_expr(), t_block];
                if let Some(else_block) = else_block {
                    args.push(else_block.to_s_expr());
                }

                let t = SExpr::Function(String::from("if"), args);
                t
            }
            TypedExpression::BinaryOperation { lhs, operator, rhs } => {
                SExpr::Function(operator.to_string(), vec![lhs.to_s_expr(), rhs.to_s_expr()])
            }
            TypedExpression::FunctionCall { name, arguments } => {
                SExpr::Function(
                    name.lexeme().to_string(),
                    arguments.into_iter().map(|x| x.to_s_expr()).collect()
                )
            }
            TypedExpression::UnaryOperation { operator, rhs } => {
                SExpr::Function(operator.to_string(), vec![rhs.to_s_expr()])
            }
            TypedExpression::Int(value) => SExpr::Value(value.lexeme().to_string()),
            TypedExpression::Bool(value, _) => SExpr::Value(value.to_string()),
            TypedExpression::List(elements) => {
                SExpr::Function(
                    String::from("list"),
                    elements.into_iter().map(|x| x.to_s_expr()).collect()
                )
            }
            TypedExpression::Name(n, _) => {SExpr::Value(n.lexeme().to_string())}
        }
    }
}
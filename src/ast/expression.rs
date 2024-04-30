use crate::ast::binary_operator::BinaryOperator;
use crate::ast::Block;
use crate::ast::unary_operator::UnaryOperator;
use crate::parser::token::Token;
use crate::testing::s_expr::SExpr;
use crate::testing::to_s_expr::ToSExpr;

#[derive(Debug, Clone)]
pub enum Expression {
    If {
        condition: Box<Expression>,
        true_block: Block,
        else_block: Option<Block>,
    },
    BinaryOperation {
        lhs: Box<Expression>,
        operator: BinaryOperator,
        rhs: Box<Expression>
    },
    FunctionCall {
        name: Token,
        arguments: Vec<Expression>
    },
    UnaryOperation {
        operator: UnaryOperator,
        rhs: Box<Expression>
    },
    Int(Token),
    Bool(bool, Token),
    List(Vec<Expression>),
    Name(Token)
}

impl ToSExpr for Expression {
    fn to_s_expr(self) -> SExpr<String> {
        match self {
            Expression::If { condition, true_block, else_block } => {
                let t_block = true_block.to_s_expr();
                let mut args = vec![condition.to_s_expr(), t_block];
                if let Some(else_block) = else_block {
                    args.push(else_block.to_s_expr());
                }

                let t = SExpr::Function(String::from("if"), args);
                t
            }
            Expression::BinaryOperation { lhs, operator, rhs } => {
                SExpr::Function(operator.to_string(), vec![lhs.to_s_expr(), rhs.to_s_expr()])
            }
            Expression::FunctionCall { name, arguments } => {
                SExpr::Function(
                    name.lexeme().to_string(),
                    arguments.into_iter().map(|x| x.to_s_expr()).collect()
                )
            }
            Expression::UnaryOperation { operator, rhs } => {
                SExpr::Function(operator.to_string(), vec![rhs.to_s_expr()])
            }
            Expression::Int(value) => SExpr::Value(value.lexeme().to_string()),
            Expression::Bool(value, _) => SExpr::Value(value.to_string()),
            Expression::List(elements) => {
                SExpr::Function(
                    String::from("list"),
                    elements.into_iter().map(|x| x.to_s_expr()).collect()
                )
            }
            Expression::Name(n) => {SExpr::Value(n.lexeme().to_string())}
        }
    }
}
use crate::ast::binary_operator::BinaryOperator;
use crate::ast::unary_operator::UnaryOperator;
use crate::parser::token::Token;
use crate::testing::s_expr::SExpr;
use crate::testing::to_s_expr::ToSExpr;
use crate::typed_ast::r#type::Type;
use crate::typed_ast::TypedBlock;

#[derive(Debug, Clone)]
pub enum TypedExpression {
    If {
        condition: Box<TypedExpression>,
        true_block: TypedBlock,
        else_block: Option<TypedBlock>,
    },
    BinaryOperation {
        lhs: Box<TypedExpression>,
        operator: BinaryOperator,
        rhs: Box<TypedExpression>,
        type_: Type
    },
    FunctionCall {
        name: Token,
        arguments: Vec<TypedExpression>,
        type_: Type
    },
    UnaryOperation {
        operator: UnaryOperator,
        rhs: Box<TypedExpression>
    },
    Int(Token),
    Bool(bool, Token),
    List(Vec<TypedExpression>, Type),
    Name(Token, Type)
}

impl TypedExpression {
    pub fn get_type(&self) -> Type {
        match self {
            TypedExpression::If { condition: _condition, true_block, else_block: _else_block } => true_block.type_.clone(),
            TypedExpression::BinaryOperation { lhs: _, operator: _, rhs: _, type_ } => type_.clone(),
            TypedExpression::FunctionCall { name: _, arguments: _, type_ } => type_.clone(),
            TypedExpression::UnaryOperation { operator: _, rhs } => rhs.get_type(),
            TypedExpression::Int(_) => Type::Int,
            TypedExpression::Bool(_, _) => Type::Bool,
            TypedExpression::List(_, t) => t.clone(),
            TypedExpression::Name(_, t) => t.clone()
        }
    }
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
            TypedExpression::BinaryOperation { lhs, operator, rhs, type_: _ } => {
                SExpr::Function(operator.to_string(), vec![lhs.to_s_expr(), rhs.to_s_expr()])
            }
            TypedExpression::FunctionCall { name, arguments, type_ } => {
                SExpr::Function(
                    name.lexeme().to_string(),
                    arguments.into_iter().map(|x| x.to_s_expr()).collect()
                )
            }
            TypedExpression::UnaryOperation { operator, rhs } => {
                SExpr::Function(operator.to_string(), vec![rhs.to_s_expr()])
            }
            TypedExpression::Int(_) => SExpr::Value(Type::Int.to_string()),
            TypedExpression::Bool(_, _) => SExpr::Value(Type::Bool.to_string()),
            TypedExpression::List(elements, _) => {
                SExpr::Function(
                    String::from("list"),
                    elements.into_iter().map(|x| x.to_s_expr()).collect()
                )
            }
            TypedExpression::Name(_, t) => SExpr::Value(t.to_string())
        }
    }
}
use std::collections::HashMap;
use itertools::Itertools;
use thiserror::Error;
use crate::ast::binary_operator::BinaryOperator;
use crate::ast::Block;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::parser::token::Token;
use crate::parser::token_kind::TokenKind;
use crate::typed_ast::r#type::Type;
use crate::typed_ast::r#type::Type::Unit;
use crate::typed_ast::typed_expression::TypedExpression;
use crate::typed_ast::typed_statement::{FunctionDefinition, TypedStatement};
use crate::typed_ast::typed_statement::TypedStatement::VariableDeclaration;
use crate::typed_ast::TypedBlock;
use crate::typer::TypingError::{ConflictingTypes, InvalidType, NameNotFound};

#[derive(Debug, Error)]
pub enum TypingError {
    #[error("Unable to find {0} in enclosing scopes.")]
    NameNotFound(Token),

    #[error("{0} declared as {1} but assigned to {1}.")]
    ConflictingTypes(Token, Type, Type),

    #[error("{0} MUST be {1}")]
    InvalidType(Type, Type)
}

pub struct Typer {
    scopes: Vec<HashMap<String, Type>>,
    typed_ast: Vec<TypedStatement>
}

impl Typer {
    pub fn type_ast(ast: Vec<Statement>) -> Result<Vec<TypedStatement>, TypingError> {
        let mut typer = Self {
            scopes: vec![Default::default()],
            typed_ast: vec![],
        };

        typer.run_typer(ast)?;
        Ok(typer.typed_ast)
    }

    fn find_in_scopes(&self, name: impl AsRef<str>) -> Option<Type> {
        let name = name.as_ref();
        for scope_index in (0..self.scopes.len()).rev() {
            if self.scopes[scope_index].contains_key(name) {
                return Some(self.scopes[scope_index][name].clone())
            }
        }

        return None
    }

    fn current_scope(&self) -> &HashMap<String, Type> {
        self.scopes.last().unwrap()
    }

    fn current_scope_mut(&mut self) -> &mut HashMap<String, Type> {
        self.scopes.last_mut().unwrap()
    }

    pub fn run_typer(&mut self, on: Vec<Statement>) -> Result<(), TypingError> {
        for statement in on {
            let statement = self.type_statement(statement)?;
            self.typed_ast.push(statement);
        }

        Ok(())
    }

    fn type_var_assignment(&mut self, var: Token, value: Expression) -> Result<TypedStatement, TypingError> {
        let typed_value = self.type_expression(value)?;

        if !self.current_scope().contains_key(var.lexeme()) {
            return Err(TypingError::NameNotFound(var))
        }

        let value_in_scope = self.current_scope_mut().get_mut(var.lexeme()).unwrap();

        if value_in_scope.clone() != typed_value.get_type() {
            return Err(ConflictingTypes(var, value_in_scope.clone(), typed_value.get_type()))
        }

        *value_in_scope = typed_value.get_type();
        Ok(TypedStatement::Assignment {
            to: var,
            value: typed_value,
        })
    }

    fn type_statement(&mut self, statement: Statement) -> Result<TypedStatement, TypingError> {
        type S = Statement;
        type TS = TypedStatement;
        match statement {
            S::FunctionDefinitionStatement(def) => {
                let def_clone = def.clone();
                let func_def_in_scope = self.current_scope_mut()
                    .entry(def.name.lexeme().to_string())
                    .or_insert(def.type_.clone());
                *func_def_in_scope = def.type_;

                self.push_function(&def.arg_list);
                let typed_func = TS::FunctionDefinitionStatement(FunctionDefinition {
                    name: def_clone.name,
                    type_: def_clone.type_,
                    arg_list: def_clone.arg_list,
                    body: self.type_block(def_clone.body)?,
                });
                self.scopes.pop();

                Ok(typed_func)
            }
            S::VariableDeclaration { name:to, type_:t, value } => {
                let decl = self.current_scope_mut()
                    .entry(to.lexeme().to_string())
                    .or_insert(t.clone());
                *decl = t.clone();
                let assignment = self.type_var_assignment(to.clone(), value.clone())?;

                // type checking yeah!
                if assignment.get_type() != t.clone() {
                    return Err(ConflictingTypes(to, assignment.get_type(), t));
                }

                Ok(VariableDeclaration {
                    name: to,
                    type_: t,
                    value: self.type_expression(value)?,
                })
            }

            S::Assignment { to, value } => {
                self.type_var_assignment(to, value)
            },
            S::While { condition, body } => {
                Ok(TypedStatement::While {
                    condition: self.type_expression(condition)?,
                    body: self.type_block(body)?
                })
            },
            S::Return(e) => Ok(TS::Return(self.type_expression(e)?)),
            S::Expression(e) => Ok(TS::Expression(self.type_expression(e)?)),
        }
    }



    fn push_function(&mut self, args: &Vec<(Token, Type)>) {
        let mut new_scope = HashMap::new();
        for (token, type_) in args {
            new_scope.insert(token.lexeme().to_string(), type_.clone());
        }

        self.scopes.push(new_scope)
    }

    fn type_block(&mut self, block: Block) -> Result<TypedBlock, TypingError> {
        let mut final_type = Unit;
        let mut typed_statements = vec![];

        for statement in block {
            let typed_statement = self.type_statement(statement)?;
            typed_statements.push(typed_statement.clone());
            final_type = typed_statement.get_type();
        }

        return Ok(TypedBlock {
            body: typed_statements,
            type_: final_type,
        })
    }

    // Hack to make if x comparisons work
    fn int_to_bool_demote(&mut self, condition: Expression) -> Result<TypedExpression, TypingError> {
        match self.type_expression(condition)? {
            TypedExpression::Int(c) => {
                Ok(TypedExpression::BinaryOperation {
                    lhs: Box::new(TypedExpression::Int(c)),
                    operator: BinaryOperator::GreaterThan,
                    rhs: Box::new(TypedExpression::Int(Token::un_located(TokenKind::Int, "0"))),
                    type_: Type::Int,
                })
            }
            t => Ok(t)
        }
    }

    fn type_expression(&mut self, expression: Expression) -> Result<TypedExpression, TypingError> {
        match expression {
            Expression::If { condition, true_block, else_block } => {

                let condition = self.int_to_bool_demote(*condition)?;

                if condition.get_type() != Type::Bool {
                    return Err(InvalidType(condition.get_type(), Type::Bool))
                }

                let true_block = self.type_block(true_block)?;
                let else_block = if let Some(else_block) = else_block{
                    Some(self.type_block(else_block)?)
                } else { None };
                Ok(TypedExpression::If {
                    condition: Box::new(condition),
                    true_block,
                    else_block,
                })
            },
            Expression::BinaryOperation { lhs, operator, rhs } => {
                let lhs = self.type_expression(*lhs)?;
                let rhs = self.type_expression(*rhs)?;

                let mut new_type = lhs.get_type();

                match operator {
                    BinaryOperator::Add => {}
                    _ => {new_type = Type::Bool}
                }
                Ok(TypedExpression::BinaryOperation {
                    lhs: Box::new(lhs),
                    operator,
                    rhs: Box::new(rhs),
                    type_: new_type,
                })
            },
            Expression::FunctionCall { name, arguments } => {
                Ok(TypedExpression::FunctionCall {
                    name: name.clone(),
                    arguments: arguments.into_iter().map(|x| self.type_expression(x)).try_collect()?,
                    type_: self.find_in_scopes(name.lexeme()).ok_or(NameNotFound(name))?,
                })
            },
            Expression::UnaryOperation { operator, rhs } =>  {
                Ok(TypedExpression::UnaryOperation {
                    operator, rhs: Box::new(self.type_expression(*rhs)?)
                })
            },
            Expression::Int(i) => Ok(TypedExpression::Int(i)),
            Expression::Bool(b, t) => Ok(TypedExpression::Bool(b, t)),
            Expression::List(_) => unimplemented!(),
            Expression::Name(name) => {
                self.find_in_scopes(name.lexeme())
                    .map(|x| TypedExpression::Name(name.clone(), x))
                    .ok_or(TypingError::NameNotFound(name))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::testing::s_expr::SExpr;
    use crate::testing::to_s_expr::ToSExpr;
    use super::*;

    fn to_s_expr(statements: Vec<TypedStatement>) -> Vec<SExpr<String>> {
        statements.into_iter().map(|x| x.to_s_expr()).collect()
    }

    #[test]
    fn forty_two() {
        let ast = Parser::new(crate::testing::demo_programs::THE_UNIVERSE).parse().unwrap();
        let typed = Typer::type_ast(ast).unwrap();
        assert_eq!(to_s_expr(typed)[0], SExpr::parse("(function_define main (int int) ->int)"))
    }
}
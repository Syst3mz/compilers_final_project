use std::collections::HashMap;
use thiserror::Error;
use crate::ast::Block;
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::parser::token::Token;
use crate::typed_ast::r#type::Type;
use crate::typed_ast::r#type::Type::Unit;
use crate::typed_ast::typed_expression::TypedExpression;
use crate::typed_ast::typed_statement::{FunctionDefinition, TypedStatement};
use crate::typed_ast::TypedBlock;

#[derive(Debug, Error)]
pub enum TypingError {
    #[error("Unable to find {0} in enclosing scopes.")]
    NameNotFound(Token)
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

    fn type_statement(&mut self, statement: Statement) -> Result<TypedStatement, TypingError> {
        match statement {
            Statement::VariableDeclaration { .. } => unimplemented!(),
            Statement::FunctionDefinitionStatement(def) => {
                let def_clone = def.clone();
                let func_def_in_scope = self.current_scope_mut()
                    .entry(def.name.lexeme().to_string())
                    .or_insert(def.type_.clone());
                *func_def_in_scope = def.type_;

                self.push_function(&def.arg_list);
                let typed_func = TypedStatement::FunctionDefinitionStatement(FunctionDefinition {
                    name: def_clone.name,
                    type_: def_clone.type_,
                    arg_list: def_clone.arg_list,
                    body: self.type_block(def_clone.body)?,
                });
                self.scopes.pop();

                Ok(typed_func)
            }
            Statement::Assignment { .. } => unimplemented!(),
            Statement::While { .. } => unimplemented!(),
            Statement::Return(e) => Ok(TypedStatement::Return(self.type_expression(e)?)),
            Statement::Expression(e) => Ok(TypedStatement::Expression(self.type_expression(e)?)),
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
            final_type = typed_statement.get_type()
        }

        return Ok(TypedBlock { body: typed_statements, type_: final_type })
    }

    fn type_expression(&self, expression: Expression) -> Result<TypedExpression, TypingError>{
        match expression {
            Expression::If { .. } => unimplemented!(),
            Expression::BinaryOperation { lhs, operator, rhs } => {
                let lhs = self.type_expression(*lhs)?;
                let rhs = self.type_expression(*rhs)?;

                let new_type = lhs.get_type();
                Ok(TypedExpression::BinaryOperation {
                    lhs: Box::new(lhs),
                    operator,
                    rhs: Box::new(rhs),
                    type_: new_type,
                })
            },
            Expression::FunctionCall { .. } => unimplemented!(),
            Expression::UnaryOperation { .. } => unimplemented!(),
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
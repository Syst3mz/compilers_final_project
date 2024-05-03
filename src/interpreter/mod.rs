/*mod interpreter_value;

use std::collections::HashMap;
use std::ops::Deref;
use anyhow::Context;
use thiserror::Error;
use crate::ast::ast_type::AstType;
use crate::ast::ast_type::AstType::Bool;
use crate::ast::Block;
use crate::ast::expression::Expression;
use crate::ast::statement::{FunctionDefinition, Statement};
use crate::ast::unary_operator::UnaryOperator;
use crate::interpreter::interpreter_value::InterpreterValue;
use crate::interpreter::InterpreterError::{FunctionNoReturn, NameNotFound, ReturnEarly, TypeError, TypeErrorAssignment};
use crate::parser::token::Token;



#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("Not actually an error, just an easy way to return {0} early.")]
    ReturnEarly(InterpreterValue),

    #[error("Unable to find {0} in scope.")]
    NameNotFound(Token),

    #[error("Unable to assign {2} to {0} which has type {1}.")]
    TypeErrorAssignment(Token, AstType, AstType),

    #[error("Incompatible type {0}, expected a {1} for {2}")]
    TypeError(AstType, AstType, String),

    #[error("Function {0} does not return a value")]
    FunctionNoReturn(String)
}

type Scope = HashMap<String, (InterpreterValue, AstType)>;
pub struct Interpreter {
    scopes: Vec<Scope>,
    instructions: Vec<Statement>,
    functions: HashMap<String, FunctionDefinition>
}

impl Interpreter {
    pub fn new(instructions: Vec<Statement>) -> Self {
        Self {
            scopes: vec![Default::default()],
            instructions,
            functions: Default::default()
        }
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().expect("No frames left in the stack.")
    }

    fn current_scope(&self) -> &Scope {
        self.scopes.last().expect("No frames left in the stack.")
    }

    pub fn evaluate(&mut self) -> Option<InterpreterValue> {
        let mut final_value = None;

        for instruction in self.instructions.clone() {
            match self.evaluate_statement(instruction) {
                Ok(v) => final_value = v,
                Err(e) => {
                    if let Ok(interpreter_error_kind) = e.downcast() {
                        match interpreter_error_kind {
                            ReturnEarly(v) => {
                                return Some(v)
                            },
                            e => panic!("{}", e)
                        }
                    }
                }
            }
        }

        return final_value
    }

    fn check_type(lhs: &AstType, rhs: &AstType) -> bool {
        match (lhs, rhs) {
            (&AstType::Int, &AstType::Int) => true,
            (&AstType::Bool, &AstType::Bool) => true,
            (&AstType::List(ref l), &AstType::List(ref r)) => Self::check_type(l.deref(), r.deref()),
            (_, _) => false
        }
    }

    pub fn evaluate_statement(&mut self, statement: Statement) -> anyhow::Result<Option<InterpreterValue>> {
        match statement {
            Statement::VariableDeclaration { name, type_, value } => {
                // hack to make lists type_check
                let value = match self.evaluate_expression(value)? {
                    InterpreterValue::List(_, v) => InterpreterValue::List(type_.clone(), v),
                    v => v
                };

                self.current_scope_mut().insert(name.lexeme().to_string(), (value, type_));
                return Ok(None)
            }
            Statement::FunctionDefinitionStatement(def) => {
                self.functions.insert(def.name.lexeme().to_string(), def.clone());
                return Ok(None)
            }
            Statement::Assignment { to, value } => {
                let value = self.evaluate_expression(value)?;
                let scope = self.current_scope_mut();
                if !scope.contains_key(to.lexeme()) {
                    return Err(NameNotFound(to).into());
                }

                let (assign_to, type_) = scope.get_mut(to.lexeme()).unwrap();
                if !Self::check_type(type_, &value.get_type()) {
                    return Err(TypeErrorAssignment(to, type_.clone(), value.get_type()).into())
                }

                *assign_to = value;
                return Ok(None)
            }
            Statement::While { condition, body } => {
                let mut last = None;
                loop {
                    let cond = self.evaluate_expression(condition.clone())?;
                    match cond {
                        InterpreterValue::Bool(b) => {
                            if !b {
                                break;
                            }
                        }
                        _ => {return Err(TypeError(Bool, cond.get_type(), String::from("While expects a bool.")).into())}
                    }

                    last = self.evaluate_block(body.clone())?
                }

                return Ok(last);
            }
            Statement::Return(e) => {
                // stop being in what ever scope I am
                self.scopes.pop();
                Err(ReturnEarly(self.evaluate_expression(e)?).into())
            },
            Statement::Expression(e) => Ok(Some(self.evaluate_expression(e)?))
        }
    }

    fn evaluate_block(&mut self, block: Block) -> anyhow::Result<Option<InterpreterValue>> {
        let mut last = None;
        for statement in block {
            last = self.evaluate_statement(statement)?
        }

        Ok(last)
    }

    fn evaluate_function(&mut self, name: String) -> anyhow::Result<Option<InterpreterValue>> {
        let return_value = self.evaluate_block(self.functions[name.as_str()].body.clone());
        if let Ok(e) = return_value {
            self.scopes.pop();
            return Ok(e)
        }

        let Err(error_value) = return_value else {panic!("Error value in eval function is fucked.")};
        let e = match error_value.downcast() {
            Ok(e) => e,
            Err(e) => return Err(e)
        };
        match e {
            ReturnEarly(v) => Ok(Some(v)),
            t => return Err(t.into())
        }
    }
    pub fn evaluate_expression(&mut self, expression: Expression) -> anyhow::Result<InterpreterValue> {
        match expression {
            Expression::If { .. } => unimplemented!(),
            Expression::BinaryOperation { .. } => unimplemented!(),
            Expression::FunctionCall { name, arguments } => {
                if !self.functions.contains_key(name.lexeme()) {
                    return Err(NameNotFound(name)).context("Unknown function called.")
                }
                let mut scope = Scope::new();

                let func_args = self.functions[name.lexeme()].arg_list.clone().into_iter();
                let zipped_arguments = arguments
                    .into_iter()
                    .map(|x| self.evaluate_expression(x).unwrap())
                    .zip(func_args);

                for (value, (arg_name, arg_type)) in zipped_arguments {
                    if !Self::check_type(&value.get_type(), &arg_type) {
                        return Err(TypeError(
                                arg_type.clone(),
                                value.get_type(),
                                format!("Function call argument does not match expected type for argument {}", arg_name.lexeme())
                            ).into())
                    }

                    scope.insert(arg_name.to_string(), (value.clone(), value.get_type()));
                }
                self.scopes.push(scope);
                let value = self.evaluate_function(name.to_string())?;
                match value {
                    None => Err(FunctionNoReturn(name.to_string()).into()),
                    Some(v) => Ok(v)
                }
            }
            Expression::UnaryOperation { operator, rhs } => {
                let rhs = self.evaluate_expression(*rhs)?;
                let rhs_type = rhs.get_type();
                match (operator, rhs_type.clone(), rhs) {
                    (UnaryOperator::Not, Bool, InterpreterValue::Bool(v)) => Ok(InterpreterValue::Bool(!v)),
                    (UnaryOperator::Not, _, _) => Err(TypeError(Bool, rhs_type, String::from("Not only applies to bools.")).into()),
                    (UnaryOperator::Sub, AstType::Int, InterpreterValue::Int(i)) => Ok(InterpreterValue::Int(-i)),
                    (UnaryOperator::Sub, _, _) => Err(TypeError(Bool, rhs_type, String::from("Can only make ints negative.")).into())
                }
            }
            Expression::Int(i) => {
                return Ok(InterpreterValue::Int(i.lexeme().parse()?))
            }
            Expression::Bool(v, _) => {
                return Ok(InterpreterValue::Bool(v))
            }
            Expression::List(l) => {
                let mut v = vec![];
                for e in l.into_iter() {
                    v.push(self.evaluate_expression(e)?);
                }
                if v.len() == 0 {
                    return Ok(InterpreterValue::List(AstType::List(Box::new(AstType::UntypedList)), v))
                }

                return Ok(InterpreterValue::List(AstType::List(Box::new(v[0].get_type())), v))
            }
            Expression::Name(n) => {
                let scope = self.current_scope();
                if !scope.contains_key(n.lexeme()) {
                    return Err(NameNotFound(n.clone())).context("name not found when evaluating expression.")
                }

                return Ok(scope[n.lexeme()].0.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_instructions() {
        let mut interpreter = Interpreter::new(vec![]);
        assert_eq!(interpreter.evaluate(), None)
    }
}*/
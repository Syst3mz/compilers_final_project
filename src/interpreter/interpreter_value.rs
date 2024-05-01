use std::fmt::{Display, Formatter};
use itertools::Itertools;
use crate::ast::ast_type::AstType;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InterpreterValue {
    Int(i32),
    Bool(bool),
    List(AstType, Vec<InterpreterValue>),
}

impl InterpreterValue {
    pub fn get_type(&self) -> AstType {
        match self {
            InterpreterValue::Int(_) => AstType::Int,
            InterpreterValue::Bool(_) => AstType::Bool,
            InterpreterValue::List(t, _) => AstType::List(Box::new(t.clone()))
        }
    }
}

impl Display for InterpreterValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            InterpreterValue::Int(i) => format!("{}", i),
            InterpreterValue::Bool(b) => format!("{}", b),
            InterpreterValue::List(_, l) => format!("[{}]", l.iter().map(|x| x.to_string()).join(", ")),
        })
    }
}
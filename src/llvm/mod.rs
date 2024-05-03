use crate::ast::statement::FunctionDefinition;
use crate::llvm::value::{Constant, Value};
use crate::llvm::r#type::Type;

mod counters;
mod converter;
mod r#type;
mod function;
mod convert;
mod value;

pub enum LLVM {
    Type(Type),
    Function(FunctionDefinition),
    Val(Value),
    Return(Type, Value)
}
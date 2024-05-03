use crate::llvm::LLVM;
use crate::llvm::r#type::Type;

pub struct Function {
    type_: Type,
    name: String,
    args: Vec<(Type, String)>,
    block: Vec<LLVM>
}
use crate::llvm::ir_builder::MemoryValue;
use crate::typed_ast::r#type::Type;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub type_: Type
}
impl Variable {
    pub fn new(name: String, type_: Type) -> Self {
        Self {
            name,
            type_,
        }
    }

    /// Load from a variable into a temp
    pub fn load(&self, into: MemoryValue) -> String {
        format!("{} = load {}, {}* %{}",
                   into.to_ir(false),
                   self.type_.llvm_type(),
                   self.type_.llvm_type(),
                   self.name)
    }

    /// Store from a temp to a variable
    pub fn store(&self, from: MemoryValue) -> String {
        format!("store {}, {}* %{}",
                   from.to_ir(true),
                   self.type_.llvm_type(),
                   self.name)
    }
}
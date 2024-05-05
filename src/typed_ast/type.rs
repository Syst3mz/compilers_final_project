use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Type {
    Int,
    Bool,
    List(Box<Type>),
    Unit,
}

impl Type {
    pub fn llvm_type(&self) -> String {
        match self {
            Type::Int => String::from("i32"),
            Type::Bool => String::from("i1"),
            Type::List(_) => unimplemented!(),
            Type::Unit => String::from(""),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Type::Int => String::from("int"),
            Type::Bool => String::from("bool"),
            Type::List(t) => format!("list<{}>", t),
            Type::Unit => String::from("unit"),
        })
    }
}
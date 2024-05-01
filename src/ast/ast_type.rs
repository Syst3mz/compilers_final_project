use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AstType{
    Int,
    Bool,
    List(Box<AstType>),
    /// Should never appear in actual code, but does here because I didn't want to make more types for the interpreter
    UntypedList
}

impl Display for AstType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AstType::Int => String::from("int"),
            AstType::Bool => String::from("bool"),
            AstType::List(t) => format!("list<{}>", t),
            AstType::UntypedList => String::from("list<Untyped>"),
        })
    }
}
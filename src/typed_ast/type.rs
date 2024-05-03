use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Type {
    Int,
    Bool,
    List(Box<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Type::Int => String::from("int"),
            Type::Bool => String::from("bool"),
            Type::List(t) => format!("list<{}>", t),
        })
    }
}
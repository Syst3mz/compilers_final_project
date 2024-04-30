use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum AstType{
    Int,
    Bool,
    List(Box<AstType>)
}

impl Display for AstType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            AstType::Int => String::from("int"),
            AstType::Bool => String::from("bool"),
            AstType::List(t) => {format!("list<{}>", t)}
        })
    }
}
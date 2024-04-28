use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum UnaryOperator {
    Sub,
    Not
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            UnaryOperator::Sub => "-",
            UnaryOperator::Not => "!"
        })
    }
}
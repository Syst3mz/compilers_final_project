use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,
    Equals,
    GreaterThan,
    Or,
    And
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Equals => "==",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::Or => "||",
            BinaryOperator::And => "&&"
        })
    }
}
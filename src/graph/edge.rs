use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use crate::graph::GraphIndex;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Ord)]
pub struct Edge {
    pub from: GraphIndex,
    pub to: GraphIndex
}

impl PartialOrd for Edge{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let from_order = self.from.partial_cmp(&other.from);
        from_order.map(|from_order| {
            if from_order.is_eq() {
                self.to.partial_cmp(&other.to)
            } else {
                Some(from_order)
            }
        }).flatten()
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
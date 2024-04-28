#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub struct Location {
    row: usize,
    column: usize
}

#[allow(dead_code)]
impl Location {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
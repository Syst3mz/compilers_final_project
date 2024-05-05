use std::collections::HashMap;



pub struct Counters {
    counters: HashMap<String, usize>
}

impl Counters {
    pub fn new() -> Self {
        Self {
            counters: Default::default(),
        }
    }

    pub fn next(&mut self, what: impl AsRef<str>) -> String {
        let count = self.counters.entry(what.as_ref().to_string()).or_insert(0);
        *count += 1;
        return format!("%{}_{}", what.as_ref(), *count)
    }
}
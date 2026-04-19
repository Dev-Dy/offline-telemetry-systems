use std::collections::HashSet;

pub struct Dedup {
    seen: HashSet<String>,
}

impl Dedup {
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    pub fn is_duplicate(&self, id: &str) -> bool {
        self.seen.contains(id)
    }

    pub fn mark_seen(&mut self, id: String) {
        self.seen.insert(id);
    }
}

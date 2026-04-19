use dashmap::DashSet;

pub struct Dedup {
    seen: DashSet<String>,
}

impl Dedup {
    pub fn new() -> Self {
        Self {
            seen: DashSet::new(),
        }
    }

    pub fn check_and_insert(&self, id: String) -> bool {
        !self.seen.insert(id)
    }
}

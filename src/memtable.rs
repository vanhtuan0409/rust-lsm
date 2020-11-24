use crate::entry::Entry;
use std::collections::BTreeSet;

pub struct MemTable {
    pub entries: BTreeSet<Entry>,
    pub max_cap: usize,
}

impl MemTable {
    pub fn new(max_cap: usize) -> Self {
        Self {
            entries: BTreeSet::new(),
            max_cap,
        }
    }
}

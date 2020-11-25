use crate::{Entry, Key};
use std::collections::*;

pub struct MemTable {
    entries: BTreeMap<Key, Entry>,
    max_cap: usize,
}

impl MemTable {
    pub fn new(max_cap: usize) -> Self {
        Self {
            entries: BTreeMap::new(),
            max_cap,
        }
    }

    pub fn is_full(&self) -> bool {
        self.entries.len() == self.max_cap
    }

    pub fn search(&mut self, key: &[u8]) -> Option<Entry> {
        self.entries.get(key).cloned()
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &Entry> + '_ {
        self.entries.iter().map(|(_, val)| val)
    }

    pub fn insert(&mut self, entry: Entry) -> Result<(), ()> {
        self.entries.insert(entry.key.clone(), entry);
        Ok(())
    }
}

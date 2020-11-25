use crate::{Entry, Key};
use std::collections::*;

pub struct MemTable {
    pub entries: BTreeMap<Key, Entry>,
    pub max_cap: usize,
}

impl MemTable {
    pub fn new(max_cap: usize) -> Self {
        Self {
            entries: BTreeMap::new(),
            max_cap,
        }
    }

    pub fn search(&mut self, key: &[u8]) -> Option<Entry> {
        self.entries.get(key).cloned()
    }

    pub fn scan<F>(&mut self, f: F)
    where
        F: Fn(&Entry),
    {
        self.entries.iter().for_each(|(_, entry)| f(entry))
    }

    pub fn insert(&mut self, entry: Entry) -> Result<(), ()> {
        self.entries.insert(entry.key.clone(), entry);
        Ok(())
    }
}

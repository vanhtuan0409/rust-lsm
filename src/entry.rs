use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;

#[derive(Eq, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("key", &String::from_utf8_lossy(&self.key))
            .field("value", &String::from_utf8_lossy(&self.value))
            .finish()
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.key.cmp(&other.key))
    }
}

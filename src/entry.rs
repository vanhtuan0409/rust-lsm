use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
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

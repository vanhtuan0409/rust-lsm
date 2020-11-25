use crate::entry::Entry;
use std::io::{Read, Seek, Write};

pub struct SSTable<T>
where
    T: Read + Write + Seek,
{
    handle: T,
}

impl<T: Read + Write + Seek> SSTable<T> {
    pub fn new(handle: T) -> Self {
        Self { handle }
    }

    #[allow(dead_code)]
    pub fn offset(&mut self) -> Result<u64, ()> {
        self.handle
            .seek(std::io::SeekFrom::Current(0))
            .map_err(|_| ())
    }

    #[allow(dead_code)]
    pub fn search(&mut self, key: &[u8]) -> Option<Entry> {
        self.handle.seek(std::io::SeekFrom::Start(0)).ok()?;
        while let Ok(decoded) = bincode::deserialize_from::<_, Entry>(&mut self.handle) {
            if decoded.key.as_slice() == key {
                return Some(decoded);
            }
        }
        None
    }

    pub fn for_each<F>(&mut self, f: F)
    where
        F: Fn(Entry),
    {
        self.handle.seek(std::io::SeekFrom::Start(0)).unwrap();
        while let Ok(decoded) = bincode::deserialize_from::<_, Entry>(&mut self.handle) {
            f(decoded);
        }
    }

    pub fn insert(&mut self, entry: Entry) -> Result<(), ()> {
        self.handle
            .seek(std::io::SeekFrom::End(0))
            .map_err(|_| ())?;
        bincode::serialize_into(&mut self.handle, &entry).map_err(|_| ())?;
        Ok(())
    }
}

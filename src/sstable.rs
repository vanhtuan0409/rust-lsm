use crate::encoding::{bincode::BincodeEncoder, Encoder};
use crate::Entry;
use std::io::{Cursor, Read, Seek, Write};

pub struct SSTable<S: Read + Write + Seek, E: Encoder<S, S>> {
    handle: S,
    encoder: E,
}

type InMemDataSink = Cursor<Vec<u8>>;
impl SSTable<InMemDataSink, BincodeEncoder> {
    pub fn new_in_mem() -> Self {
        Self {
            handle: Cursor::new(Vec::new()),
            encoder: BincodeEncoder::new(),
        }
    }
}

impl<S: Read + Write + Seek, E: Encoder<S, S>> SSTable<S, E> {
    #[allow(dead_code)]
    pub fn offset(&mut self) -> Result<u64, ()> {
        self.handle
            .seek(std::io::SeekFrom::Current(0))
            .map_err(|_| ())
    }

    #[allow(dead_code)]
    pub fn search(&mut self, key: &[u8]) -> Option<Entry> {
        self.handle.seek(std::io::SeekFrom::Start(0)).ok()?;
        while let Ok(decoded) = self.encoder.read_record(&mut self.handle) {
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
        while let Ok(decoded) = self.encoder.read_record(&mut self.handle) {
            f(decoded);
        }
    }

    pub fn insert(&mut self, entry: &Entry) -> Result<(), ()> {
        self.handle
            .seek(std::io::SeekFrom::End(0))
            .map_err(|_| ())?;
        self.encoder.write_record(&mut self.handle, entry)?;
        Ok(())
    }
}

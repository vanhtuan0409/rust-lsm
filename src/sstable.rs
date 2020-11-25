use crate::encoding::Encoder;
use crate::Entry;
use std::io::{Read, Seek, Write};

pub trait DataSink: Read + Write + Seek {}
impl<T> DataSink for T where T: Read + Write + Seek {}

pub struct SSTable<S: DataSink, E: Encoder<S, S>> {
    sink: S,
    encoder: E,
}

impl<S: DataSink, E: Encoder<S, S>> SSTable<S, E> {
    pub fn new(sink: S, encoder: E) -> Self {
        Self { sink, encoder }
    }

    #[allow(dead_code)]
    pub fn offset(&mut self) -> Result<u64, ()> {
        self.sink
            .seek(std::io::SeekFrom::Current(0))
            .map_err(|_| ())
    }

    #[allow(dead_code)]
    pub fn search(&mut self, key: &[u8]) -> Option<Entry> {
        self.sink.seek(std::io::SeekFrom::Start(0)).ok()?;
        while let Ok(decoded) = self.encoder.read_record(&mut self.sink) {
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
        self.sink.seek(std::io::SeekFrom::Start(0)).unwrap();
        while let Ok(decoded) = self.encoder.read_record(&mut self.sink) {
            f(decoded);
        }
    }

    pub fn insert(&mut self, entry: &Entry) -> Result<(), ()> {
        self.sink.seek(std::io::SeekFrom::End(0)).map_err(|_| ())?;
        self.encoder.write_record(&mut self.sink, entry)?;
        Ok(())
    }
}

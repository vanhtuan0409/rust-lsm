use crate::encoding::{BincodeEncoder, Encoder};
use crate::Entry;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

pub trait DataSink: Read + Write + Seek + Clone {}
impl<T> DataSink for T where T: Read + Write + Seek + Clone {}

pub struct SSTableBuilder<S: DataSink, E: Encoder> {
    id: Option<String>,
    sink: Option<S>,
    encoder: Option<E>,
}

pub type InMemSink = Cursor<Vec<u8>>;
impl<S: DataSink, E: Encoder> SSTableBuilder<S, E> {
    pub fn new() -> Self {
        Self {
            id: None,
            sink: None,
            encoder: None,
        }
    }

    pub fn with_id(self, id: String) -> Self {
        Self {
            id: Some(id),
            sink: self.sink,
            encoder: self.encoder,
        }
    }

    pub fn build(self) -> Option<SSTable<S, E>> {
        Some(SSTable {
            id: self.id?,
            sink: self.sink?,
            encoder: self.encoder?,
        })
    }
}

impl<E: Encoder> SSTableBuilder<InMemSink, E> {
    pub fn with_inmem_sink(self) -> SSTableBuilder<InMemSink, E> {
        SSTableBuilder {
            id: self.id,
            sink: Some(Cursor::new(Vec::new())),
            encoder: self.encoder,
        }
    }
}

impl<S: DataSink> SSTableBuilder<S, BincodeEncoder> {
    pub fn with_bincode_encoder(self) -> SSTableBuilder<S, BincodeEncoder> {
        SSTableBuilder {
            id: self.id,
            sink: self.sink,
            encoder: Some(BincodeEncoder::new()),
        }
    }
}

#[derive(Debug)]
pub struct SSTable<S: DataSink, E: Encoder> {
    id: String,
    sink: S,
    encoder: E,
}

impl<S: DataSink, E: Encoder> SSTable<S, E> {
    #[allow(dead_code)]
    pub fn offset(&mut self) -> Result<u64, ()> {
        self.sink.seek(SeekFrom::Current(0)).map_err(|_| ())
    }

    #[allow(dead_code)]
    pub fn search(&mut self, key: &[u8]) -> Option<Entry> {
        self.sink.seek(SeekFrom::Start(0)).ok()?;
        while let Ok(decoded) = self.encoder.read_record(&mut self.sink) {
            if decoded.key.as_slice() == key {
                return Some(decoded);
            }
        }
        None
    }

    pub fn iter(&self) -> SSTableIter<S, E> {
        let mut sink = self.sink.clone();
        sink.seek(SeekFrom::Start(0)).unwrap();
        SSTableIter {
            sink,
            encoder: self.encoder.clone(),
        }
    }

    pub fn insert(&mut self, entry: &Entry) -> Result<(), ()> {
        self.sink.seek(SeekFrom::End(0)).map_err(|_| ())?;
        self.encoder.write_record(&mut self.sink, entry)?;
        Ok(())
    }
}

pub struct SSTableIter<S: DataSink, E: Encoder> {
    sink: S,
    encoder: E,
}

impl<S: DataSink, E: Encoder> Iterator for SSTableIter<S, E> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.encoder.read_record(&mut self.sink).ok()
    }
}

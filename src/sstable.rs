use crate::encoding::{BincodeEncoder, Encoder};
use crate::Entry;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

pub struct SSTableBuilder<E: Encoder> {
    id: Option<String>,
    data_dir: Option<String>,
    encoder: Option<E>,
}

impl<E: Encoder> SSTableBuilder<E> {
    pub fn new() -> Self {
        Self {
            id: None,
            data_dir: None,
            encoder: None,
        }
    }

    pub fn with_id(self, id: String) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }

    pub fn with_data_dir(self, data_dir: String) -> Self {
        Self {
            data_dir: Some(data_dir),
            ..self
        }
    }

    pub fn build(self) -> Option<SSTable<E>> {
        let file_path = Path::new(&self.data_dir?).join(self.id.clone()?);
        let sink = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path.clone())
            .ok()?;
        Some(SSTable {
            id: self.id?,
            sink,
            file_path,
            encoder: self.encoder?,
        })
    }
}

impl SSTableBuilder<BincodeEncoder> {
    pub fn with_bincode_encoder(self) -> SSTableBuilder<BincodeEncoder> {
        SSTableBuilder {
            encoder: Some(BincodeEncoder::new()),
            ..self
        }
    }
}

#[derive(Debug)]
pub struct SSTable<E: Encoder> {
    id: String,
    file_path: PathBuf,
    sink: File,
    encoder: E,
}

impl<E: Encoder> SSTable<E> {
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

    pub fn iter(&self) -> SSTableIter<E> {
        let mut sink = self.sink.try_clone().unwrap();
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

pub struct SSTableIter<E: Encoder> {
    sink: File,
    encoder: E,
}

impl<E: Encoder> Iterator for SSTableIter<E> {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.encoder.read_record(&mut self.sink).ok()
    }
}

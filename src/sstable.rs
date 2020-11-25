use crate::Entry;
use crate::{
    encoding::{BincodeEncoder, Encoder},
    Key,
};
use std::io::{Seek, SeekFrom};
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

pub struct SSTableBuilder<E: Encoder> {
    id: Option<usize>,
    data_dir: Option<PathBuf>,
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

    pub fn with_id(self, id: usize) -> Self {
        Self {
            id: Some(id),
            ..self
        }
    }

    pub fn with_data_dir(self, data_dir: PathBuf) -> Self {
        Self {
            data_dir: Some(data_dir),
            ..self
        }
    }

    pub fn build(self) -> Option<SSTable<E>> {
        let file_path = self.data_dir?.join(self.id?.to_string());
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
    id: usize,
    file_path: PathBuf,
    sink: File,
    encoder: E,
}

impl<E: Encoder> SSTable<E> {
    #[allow(dead_code)]
    pub fn search(&mut self, key: &Key) -> Option<Entry> {
        self.sink.seek(SeekFrom::Start(0)).ok()?;
        while let Ok(decoded) = self.encoder.read_record(&mut self.sink) {
            if &decoded.key == key {
                return Some(decoded);
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> SSTableIter<E> {
        let mut sink = self.sink.try_clone().unwrap();
        sink.seek(SeekFrom::Start(0)).unwrap();
        SSTableIter {
            sink,
            encoder: self.encoder.clone(),
        }
    }

    pub fn flush(&mut self, entries: Vec<&Entry>) -> Result<(), ()> {
        self.sink.seek(SeekFrom::Start(0)).map_err(|_| ())?;
        for entry in entries.into_iter() {
            self.encoder.write_record(&mut self.sink, entry)?;
        }
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

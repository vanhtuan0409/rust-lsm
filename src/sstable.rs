use crate::encoding::{BincodeEncoder, Encoder};
use crate::{Entry, Key};
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
};

const BLOCK_SIZE: usize = 2;

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
        let mut table = SSTable {
            id: self.id?,
            sink,
            file_path,
            encoder: self.encoder?,
            index: BTreeMap::new(),
        };
        table.rehydrate_index().ok()?;
        Some(table)
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
    index: BTreeMap<Key, u64>,
    sink: File,
    encoder: E,
}

impl<E: Encoder> SSTable<E> {
    pub fn search(&mut self, key: &Key) -> Option<Entry> {
        let offset = self.get_offset_for_key(key)?;
        self.iter_at(offset)
            .take(BLOCK_SIZE) // take a block at offset with BLOCK_SIZE items
            .find(|entry| &entry.key == key)
    }

    #[allow(dead_code)]
    pub fn iter(&self) -> SSTableIter<E> {
        self.iter_at(0)
    }

    pub fn iter_at(&self, offset: u64) -> SSTableIter<E> {
        let mut sink = self.sink.try_clone().unwrap();
        sink.seek(SeekFrom::Start(offset)).unwrap();
        SSTableIter {
            sink,
            encoder: self.encoder.clone(),
        }
    }

    pub fn flush(&mut self, entries: Vec<&Entry>) -> Result<(), ()> {
        self.sink.set_len(0).map_err(|_| ())?; // truncate file
        self.sink.seek(SeekFrom::Start(0)).map_err(|_| ())?;
        for entry in entries.into_iter() {
            self.encoder.write_record(&mut self.sink, entry)?;
        }
        self.rehydrate_index()?;
        Ok(())
    }

    fn rehydrate_index(&mut self) -> Result<(), ()> {
        self.sink.seek(SeekFrom::Start(0)).map_err(|_| ())?;
        let mut index = 0;
        while let Ok(entry) = self.encoder.read_record(&mut self.sink) {
            if index % BLOCK_SIZE == 0 {
                let offset = self.offset()? - self.encoder.sized(&entry)?;
                self.index.insert(entry.key.clone(), offset);
            }
            index += 1;
        }
        Ok(())
    }

    fn get_offset_for_key(&mut self, key: &Key) -> Option<u64> {
        Some(
            *self
                .index
                .iter()
                .take_while(|(curr_key, _)| *curr_key <= key)
                .last()?
                .1,
        )
    }

    fn offset(&mut self) -> Result<u64, ()> {
        self.sink.seek(SeekFrom::Current(0)).map_err(|_| ())
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

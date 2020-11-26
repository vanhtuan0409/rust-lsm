use crate::encoding::BincodeEncoder;
use crate::entry::{Entry, Key};
use crate::memtable::MemTable;
use crate::sstable::{SSTable, SSTableBuilder};
use std::collections::BTreeMap;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

const MEMTABLE_SIZE: usize = 10;

#[derive(Debug)]
pub struct Database {
    mtb: MemTable,
    sstables: BTreeMap<usize, SSTable<BincodeEncoder>>,
    next_id: usize,
    root_dir: PathBuf,
}

fn get_segment_id_from_path(path: DirEntry) -> Result<usize, ()> {
    path.file_name()
        .into_string()
        .map_err(|_| ())?
        .parse::<usize>()
        .map_err(|_| ())
}

impl Database {
    pub fn open(root_dir: &str) -> Result<Database, ()> {
        let mut db = Database {
            mtb: MemTable::new(MEMTABLE_SIZE),
            sstables: BTreeMap::new(),
            next_id: 0,
            root_dir: Path::new(root_dir).to_path_buf(),
        };
        db.open_sstables()?;
        Ok(db)
    }

    pub fn insert(&mut self, entry: &Entry) -> Result<(), ()> {
        self.mtb.insert(entry)
    }

    pub fn search(&mut self, key: &Key) -> Option<Entry> {
        // look up in memtable 1st
        if let Some(found) = self.mtb.search(key) {
            return Some(found);
        }

        // then look up in sstable segment in reverse order
        for (_, segment) in self.sstables.iter_mut().rev() {
            if let Some(found) = segment.search(key) {
                return Some(found);
            }
        }

        None
    }

    #[allow(dead_code)]
    fn flush(&mut self) -> Result<(), ()> {
        if self.mtb.is_empty() {
            return Ok(());
        }

        let data_dir = self.get_data_dir();
        let mut new_segment: SSTable<BincodeEncoder> = SSTableBuilder::new()
            .with_id(self.next_id)
            .with_data_dir(data_dir)
            .with_bincode_encoder()
            .build()
            .ok_or(())?;
        new_segment.flush(self.mtb.iter().collect::<Vec<_>>())?;
        self.sstables.insert(self.next_id, new_segment);
        self.mtb = MemTable::new(MEMTABLE_SIZE);
        self.next_id += 1;
        Ok(())
    }

    fn get_data_dir(&self) -> PathBuf {
        self.root_dir.join("segments")
    }

    fn open_sstables(&mut self) -> Result<(), ()> {
        let data_dir = self.get_data_dir();
        for path in read_dir(data_dir.clone()).map_err(|_| ())? {
            let segment_id = get_segment_id_from_path(path.map_err(|_| ())?)?;
            self.next_id = std::cmp::max(self.next_id, segment_id + 1);
            let segment: SSTable<BincodeEncoder> = SSTableBuilder::new()
                .with_id(segment_id)
                .with_data_dir(data_dir.clone())
                .with_bincode_encoder()
                .build()
                .ok_or(())?;
            self.sstables.insert(segment_id, segment);
        }
        Ok(())
    }
}

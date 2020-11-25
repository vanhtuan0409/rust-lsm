use crate::encoding::BincodeEncoder;
use crate::entry::Entry;
use crate::memtable::MemTable;
use crate::sstable::SSTableBuilder;
use crate::{sstable::SSTable, Key};
use std::collections::*;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub struct Database {
    mtb: MemTable,
    sstables: BTreeMap<String, SSTable<BincodeEncoder>>,

    #[allow(dead_code)]
    root_dir: PathBuf,
}

impl Database {
    pub fn open(root_dir: &str) -> Result<Database, ()> {
        let mut sstables = BTreeMap::new();

        // Open all segment files
        let data_dir = Path::new(root_dir).join("segments");
        for path in read_dir(data_dir.clone()).map_err(|_| ())? {
            let segment_name = path
                .map_err(|_| ())?
                .file_name()
                .into_string()
                .map_err(|_| ())?;
            let segment: SSTable<BincodeEncoder> = SSTableBuilder::new()
                .with_id(segment_name.clone())
                .with_data_dir(data_dir.clone())
                .with_bincode_encoder()
                .build()
                .ok_or(())?;
            sstables.insert(segment_name, segment).ok_or(())?;
        }

        Ok(Database {
            mtb: MemTable::new(10),
            sstables,
            root_dir: Path::new(root_dir).to_path_buf(),
        })
    }

    pub fn insert(&mut self, entry: &Entry) -> Result<(), ()> {
        self.mtb.insert(entry)
    }

    pub fn search(&mut self, key: &Key) -> Option<Entry> {
        if let Some(found) = self.mtb.search(key) {
            return Some(found);
        }

        for (_, segment) in self.sstables.iter_mut() {
            if let Some(found) = segment.search(key) {
                return Some(found);
            }
        }

        None
    }
}

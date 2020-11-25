use crate::encoding::BincodeEncoder;
use crate::entry::{Entry, Key};
use crate::memtable::MemTable;
use crate::sstable::{SSTable, SSTableBuilder};
use std::collections::BTreeMap;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Database {
    mtb: MemTable,
    sstables: BTreeMap<usize, SSTable<BincodeEncoder>>,
    next_id: usize,

    #[allow(dead_code)]
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
        let mut sstables = BTreeMap::new();
        let mut next_id = 0;

        // Open all segment files
        let data_dir = Path::new(root_dir).join("segments");
        for path in read_dir(data_dir.clone()).map_err(|_| ())? {
            let segment_id = get_segment_id_from_path(path.map_err(|_| ())?)?;
            next_id = std::cmp::max(next_id, segment_id + 1);
            let segment: SSTable<BincodeEncoder> = SSTableBuilder::new()
                .with_id(segment_id)
                .with_data_dir(data_dir.clone())
                .with_bincode_encoder()
                .build()
                .ok_or(())?;
            sstables.insert(segment_id, segment);
        }

        Ok(Database {
            mtb: MemTable::new(10),
            sstables,
            next_id,
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

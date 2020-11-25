use crate::sstable::SSTable;
use crate::{encoding::BincodeEncoder, memtable::MemTable};
use std::collections::*;
use std::path::PathBuf;

pub struct Database {
    mtb: MemTable,
    sstables: BTreeMap<String, SSTable<BincodeEncoder>>,
    root_dir: PathBuf,
}

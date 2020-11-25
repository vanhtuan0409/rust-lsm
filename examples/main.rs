extern crate lsm;

use lsm::{Entry, SSTable, SSTableBuilder};

fn get_entry(index: usize) -> Entry {
    let key = format!("foo{}", index);
    let value = format!("bar{}", index);
    Entry {
        key: key.as_bytes().to_vec(),
        value: value.as_bytes().to_vec(),
    }
}

fn main() {
    let mut table: SSTable<_, _> = SSTableBuilder::new()
        .with_id("sstable01".to_string())
        .with_inmem_sink()
        .with_bincode_encoder()
        .build()
        .unwrap();

    for i in 0..10 {
        let entry = get_entry(i);
        table.insert(&entry).unwrap();
    }

    table.iter().for_each(|entry| println!("{:?}", entry));
}

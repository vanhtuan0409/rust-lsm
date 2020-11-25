extern crate lsm;

use lsm::{encoding::BincodeEncoder, Entry, SSTable};
use std::io::Cursor;

fn get_entry(index: usize) -> Entry {
    let key = format!("foo{}", index);
    let value = format!("bar{}", index);
    Entry {
        key: key.as_bytes().to_vec(),
        value: value.as_bytes().to_vec(),
    }
}

fn main() {
    let mut table = SSTable::new(Cursor::new(Vec::<u8>::new()), BincodeEncoder::new());

    for i in 0..10 {
        let entry = get_entry(i);
        table.insert(&entry).unwrap();
    }

    table.for_each(|decoded| println!("{:?}", decoded));
}

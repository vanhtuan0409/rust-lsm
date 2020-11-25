extern crate lsm;

use lsm::{Database, Entry};

fn main() {
    let mut db = Database::open("./data").unwrap();
    db.insert(&Entry::new("foo", "bar")).unwrap();
    let found = db.search(&Entry::new_key("foo")).unwrap();
    println!("{:?}", found)
}

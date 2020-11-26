extern crate lsm;

use lsm::{Database, Entry};

fn main() {
    let mut db = Database::open("./data").unwrap();
    // db.insert(&Entry::new("foo6", "bar6")).unwrap();
    // db.insert(&Entry::new("foo0", "bar0")).unwrap();
    // db.insert(&Entry::new("foo1", "bar1")).unwrap();
    // db.insert(&Entry::new("foo3", "bar3")).unwrap();
    // db.insert(&Entry::new("foo5", "bar5")).unwrap();
    // db.insert(&Entry::new("foo9", "bar9")).unwrap();
    // db.insert(&Entry::new("foo2", "bar2")).unwrap();
    // db.insert(&Entry::new("foo8", "bar8")).unwrap();
    // db.insert(&Entry::new("foo7", "bar7")).unwrap();
    // db.insert(&Entry::new("foo4", "bar4")).unwrap();
    // db.insert(&Entry::new("foo10", "bar10")).unwrap();

    println!("{:?}", db.search(&Entry::new_key("foo0")));
    println!("{:?}", db.search(&Entry::new_key("foo1")));
    println!("{:?}", db.search(&Entry::new_key("foo2")));
    println!("{:?}", db.search(&Entry::new_key("foo3")));
    println!("{:?}", db.search(&Entry::new_key("foo4")));
    println!("{:?}", db.search(&Entry::new_key("foo5")));
    println!("{:?}", db.search(&Entry::new_key("foo6")));
    println!("{:?}", db.search(&Entry::new_key("foo7")));
    println!("{:?}", db.search(&Entry::new_key("foo8")));
    println!("{:?}", db.search(&Entry::new_key("foo9")));
    println!("{:?}", db.search(&Entry::new_key("foo10")));
}

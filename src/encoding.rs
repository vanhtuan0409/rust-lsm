use crate::entry::Entry;
use std::io::{Read, Write};

mod bincode;
pub use self::bincode::BincodeEncoder;

pub trait Encoder: Clone {
    fn read_record<R: Read>(&self, input: R) -> Result<Entry, ()>;
    fn write_record<W: Write>(&self, output: W, entry: &Entry) -> Result<(), ()>;
    fn sized(&self, entry: &Entry) -> Result<u64, ()>;
}

use crate::entry::Entry;
use std::io::{Read, Write};

mod bincode;
pub use self::bincode::BincodeEncoder;

pub trait Encoder: Clone {
    fn read_record<R: Read>(&self, input: &mut R) -> Result<Entry, ()>;
    fn write_record<W: Write>(&self, output: &mut W, entry: &Entry) -> Result<(), ()>;
}

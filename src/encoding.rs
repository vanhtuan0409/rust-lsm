pub mod bincode;

use crate::entry::Entry;
use std::io::{Read, Write};

pub trait Encoder<R: Read, W: Write> {
    fn read_record(&self, input: &mut R) -> Result<Entry, ()>;
    fn write_record(&self, output: &mut W, entry: &Entry) -> Result<(), ()>;
}

extern crate bincode;

use crate::encoding::Encoder;
use crate::Entry;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub struct BincodeEncoder {}

impl BincodeEncoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder for BincodeEncoder {
    fn read_record<R: Read>(&self, input: &mut R) -> Result<Entry, ()> {
        bincode::deserialize_from::<_, Entry>(input).map_err(|_| ())
    }

    fn write_record<W: Write>(&self, output: &mut W, entry: &Entry) -> Result<(), ()> {
        bincode::serialize_into(output, entry).map_err(|_| ())
    }
}

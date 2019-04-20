// TODO: Remove

use crate::serialization::slice_reader::SliceReader;

pub struct VecReader {
    slice_reader : SliceReader<'static>
}

impl From<Vec<u8>> for VecReader {
    fn from(vec : Vec<u8>) -> Self {
        Self {
            slice_reader : SliceReader::from()
        }
    }
}
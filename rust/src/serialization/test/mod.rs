use crate::serialization::slice_reader::MemReader;
use crate::serialization::core::Reader;
use crate::serialization::core::Type;
use crate::serialization::core::VecWriter;
use crate::serialization::core::Writer;

pub mod binary;
pub mod utf8;

fn new_writer() -> VecWriter {
    VecWriter::default()
}

fn de_serialize<T>(input: Vec<u8>) -> T::ReadItem
where
    T: Type,
    T::ReadItem : Clone
{
    let mut reader: MemReader = MemReader::from(input);
    let got = reader.read::<T>().unwrap();
    got.clone()
}

fn serialize<T>(input: &T::WriteItem) -> Vec<u8>
where
    T: Type,
{
    let mut writer = new_writer();
    writer.write::<T>(input).unwrap();
    writer.finish()
}

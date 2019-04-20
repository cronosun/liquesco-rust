use crate::serialization::core::TypeReader;
use crate::serialization::core::TypeWriter;
use crate::serialization::slice_reader::MemReader;
use crate::serialization::core::Reader;
use crate::serialization::core::VecWriter;
use crate::serialization::core::Writer;

pub mod binary;
pub mod utf8;
pub mod person;

fn new_writer() -> VecWriter {
    VecWriter::default()
}

/*
fn de_serialize<'a, T>(input: Vec<u8>) -> T::Item
where
    T: TypeReader<'a>,
{
    {
        let mut reader = MemReader::from(input);    
        reader.read::<T>();
    }
    unimplemented!()
}*/

fn serialize<T>(input: &T::Item) -> Vec<u8>
where
    T: TypeWriter,
{
    let mut writer = new_writer();
    writer.write::<T>(input).unwrap();
    writer.finish()
}

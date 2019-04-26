//use std::fmt::Debug;
//use crate::serialization::core::Serializable;
//use crate::serialization::core::DeSerializable;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::slice_reader::SliceReader;
use crate::serialization::core::Reader;
use crate::serialization::vec_writer::VecWriter;
use crate::serialization::core::Writer;

pub mod binary;
pub mod utf8;
pub mod person;

fn new_writer() -> VecWriter {
    VecWriter::default()
}

fn de_serialize<'a, T>(input: &'a [u8]) -> T::Item
where
    T: DeSerializer<'a>,
{
    let mut reader = SliceReader::from(input);    
    reader.read::<T>().unwrap()
}

fn serialize<T>(input: &T::Item) -> Vec<u8>
where
    T: Serializer,
{
    let mut writer = new_writer();
    writer.write::<T>(input).unwrap();
    writer.finish()
}

fn assert_binary<T>(input : &T::Item, expected : &[u8]) where
    T: Serializer {
    let binary = serialize::<T>(input);
    assert_eq!(expected, binary.as_slice());
}

/*
fn assert_serde_eq<TValue>(value : &TValue) where
    TValue : PartialEq + Debug + DeSerializable<'static> + Serializable,
 {
     let mut writer = new_writer();
     value.serialize(&mut writer).unwrap();
     let binary = writer.finish();
     let mut reader : SliceReader<'static> = SliceReader::from(binary.as_slice());
     let deserialized = TValue::de_serialize(&mut reader).unwrap();
     assert_eq!(value, &deserialized);
}*/
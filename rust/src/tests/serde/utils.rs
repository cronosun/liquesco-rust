use crate::common::error::LqError;
use crate::serde::new_deserializer;
use crate::serde::serialize;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::slice_reader::SliceReader;
use crate::serialization::value::Value;
use crate::serialization::vec_writer::VecWriter;
use serde::de;
use serde::ser;
use serde::{Serialize, Deserialize};


pub fn assert_serde<'de, S: ser::Serialize + de::Deserialize<'de>>(item: &S) {
    let mut writer = VecWriter::default();
    serialize(&mut writer, item).expect("Unable to serialize value");
    let serialized_data = writer.finish();

    // now de-serialize the data
    let mut reader: SliceReader = (&serialized_data).into();
    let mut de = new_deserializer(reader);

    //let mut de = crate::serde::deserializer::Deserializer::new(&mut reader);

      Demo::deserialize(&mut de);


    //let item: S = de_serialize(&mut reader).expect("Unable to de-serialize");
    //de_serialize::<'static, SliceReader, S>(&mut reader).expect("Unable to de-serialize");
}

#[derive(Deserialize, Serialize)]
pub struct Demo<'a> {
    value : &'a str
}
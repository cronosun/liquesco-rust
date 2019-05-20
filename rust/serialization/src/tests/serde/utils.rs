use crate::serde::new_deserializer;
use crate::serde::serialize;
use crate::slice_reader::SliceReader;
use crate::vec_writer::VecWriter;
use std::fmt::Debug;

pub fn assert_serde<S>(item: S)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
{
    let mut writer = VecWriter::default();
    serialize(&mut writer, &item).expect("Unable to serialize value");
    let serialized_data = writer.finish();

    // now de-serialize the data
    let reader: SliceReader = (&serialized_data).into();
    let mut de = new_deserializer(reader);
    let value = S::deserialize(&mut de).expect("Unable to de-serialize");

    // make sure we got the same values
    assert_eq!(item, value);
}

pub fn serialize_to_same<S1, S2>(item1: S1, item2: S2)
where
    S1: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    S2: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
{
    // item 1
    let mut writer1 = VecWriter::default();
    serialize(&mut writer1, &item1).expect("Unable to serialize value 1");
    let serialized_data1 = writer1.finish();

    // item 2
    let mut writer2 = VecWriter::default();
    serialize(&mut writer2, &item2).expect("Unable to serialize value 1");
    let serialized_data2 = writer2.finish();

    assert_eq!(serialized_data1, serialized_data2);

    // of course can also serialize and de-serialize
    assert_serde(item1);
    assert_serde(item2);
}

pub fn can_decode_from<Source, Destination>(source: Source, destination: Destination)
where
    Source: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    Destination: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
{
    // serialize source
    let mut writer = VecWriter::default();
    serialize(&mut writer, &source).expect("Unable to serialize value");
    let serialized_data = writer.finish();

    // now de-serialize destination
    let reader: SliceReader = (&serialized_data).into();
    let mut de = new_deserializer(reader);
    let value = Destination::deserialize(&mut de).expect("Unable to de-serialize");

    assert_eq!(destination, value);

    // and of course are serializable on their own
    assert_serde(source);
    assert_serde(destination);
}

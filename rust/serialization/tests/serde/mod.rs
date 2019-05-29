use liquesco_serialization::serde::{serialize_to_vec, de_serialize_from_slice};
use liquesco_serialization::slice_reader::SliceReader;
use liquesco_serialization::vec_writer::VecWriter;
use std::fmt::Debug;

pub mod enum_extensible;
pub mod seq_extensible;
pub mod simple_enum;
pub mod simple_map;
pub mod simple_scalars;
pub mod simple_sequences;
pub mod struct_defaults;
pub mod struct_demo;

pub fn assert_serde<S>(item: S)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
{
    let serialized_data = serialize_to_vec(&item)
        .expect("Unable to serialize value");

    // now de-serialize the data
    let value = de_serialize_from_slice::<S>(&serialized_data)
        .expect("Unable to de-serialize");

    // make sure we got the same values
    assert_eq!(item, value);
}

pub fn serialize_to_same<S1, S2>(item1: S1, item2: S2)
where
    S1: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    S2: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
{
    // item 1
    let serialized_data1 = serialize_to_vec(&item1)
        .expect("Unable to serialize value 1");
    // item 2
    let serialized_data2 = serialize_to_vec(&item2)
        .expect("Unable to serialize value 2");

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
    let serialized_data = serialize_to_vec(&source)
        .expect("Unable to serialize value");

    // now de-serialize destination
    let value = de_serialize_from_slice::<Destination>(&serialized_data)
        .expect("Unable to de-serialize");

    assert_eq!(destination, value);

    // and of course are serializable on their own
    assert_serde(source);
    assert_serde(destination);
}

use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::slice_reader::SliceReader;
use crate::serialization::value::Value;
use crate::serialization::vec_writer::VecWriter;

pub fn check_value(value: &Value) {
    // serialize the data
    let mut writer = VecWriter::default();
    Value::serialize(&mut writer, value).expect("Unable to serialize value");
    let serialized_data = writer.finish();
    // now de-serialize the data
    let mut reader: SliceReader = (&serialized_data).into();
    let de_serialized_value = Value::de_serialize(&mut reader).expect(&format!(
        "Unable to de-serialize value. binary: {:?}",
        serialized_data
    ));
    // are equal
    if value != &de_serialized_value {
        panic!(format!(
            "wanted: {:?}, have: {:?}. binary: {:?}",
            value, de_serialized_value, serialized_data
        ))
    }
}

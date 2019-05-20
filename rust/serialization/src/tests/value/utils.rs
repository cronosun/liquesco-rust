use crate::core::DeSerializer;
use crate::core::Serializer;
use crate::slice_reader::SliceReader;
use crate::value::Value;
use crate::vec_writer::VecWriter;

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

pub fn serialize_de_serialize<F>(value: &Value, value_receiver: F)
where
    F: FnOnce(&Value),
{
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
    value_receiver(&de_serialized_value);
}

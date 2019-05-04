use crate::schema::core::new_deserialzation_context;
use crate::schema::core::new_schema_builder;
use crate::schema::core::Config;
use crate::schema::core::DeSerializationContext;
use crate::schema::core::Schema;
use crate::schema::core::SchemaBuilder;
use crate::schema::core::ValidatorReceiver;
use crate::schema::core::ValidatorRef;
use crate::serialization::slice_reader::SliceReader;
use crate::serialization::vec_writer::VecWriter;

pub fn check_serde<'a, F>(build_fn: F)
where
    F: FnOnce(&mut ValidatorReceiver<'a>) -> ValidatorRef,
{
    let mut schema_builder = new_schema_builder();
    let reference = build_fn(&mut schema_builder);
    let schema = schema_builder.into_schema(Config::new());
    let mut writer = VecWriter::default();
    schema
        .serialize(&mut writer, reference)
        .expect("Unable to serialize");
    let vec = writer.finish();

    // and de-serialize the same thing
    let mut reader: SliceReader = vec.as_slice().into();
    let deserialization = new_deserialzation_context(&mut reader);
    let (out_schema, out_reference) = deserialization
        .into_schema(Config::new())
        .expect("Into schema");

    // unfortunately we can't compare validators (since they contain references
    // and those could be differen) - so we just create binary again and compare binary.

    let mut out_writer = VecWriter::default();
    out_schema
        .serialize(&mut out_writer, out_reference)
        .expect("Unable to serialize (second)");
    let vec_2 = out_writer.finish();

    assert_eq!(vec, vec_2);
}

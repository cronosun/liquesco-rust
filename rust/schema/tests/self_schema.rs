// makes sure that the schema itself is valid

use liquesco_schema::core::Config;
use liquesco_schema::core::Schema;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_anchors::{SchemaAnchors, SchemaAnchorsBuilder};
use liquesco_schema::schema_builder::BuildsOwnSchema;
use liquesco_serialization::serde::new_deserializer;
use liquesco_serialization::serde::serialize;
use liquesco_serialization::slice_reader::SliceReader;
use liquesco_serialization::vec_writer::VecWriter;
use serde::Deserialize;
use std::convert::TryInto;

#[test]
fn test_self_schema_is_valid() {
    let mut builder = SchemaAnchorsBuilder::default();
    let main_type = SchemaAnchors::build_schema(&mut builder);
    let anchors: SchemaAnchors = builder.try_into().unwrap();

    let mut writer = VecWriter::default();
    serialize(&mut writer, &anchors).expect("Unable to serialize value");
    let serialized_data = writer.finish();
    let mut reader: SliceReader = (&serialized_data).into();

    // Now validate using itself

    let type_container = anchors;
    let mut schema = DefaultSchema::new(type_container, main_type);

    schema.set_extended_diagnostics(true);
    schema
        .validate(
            Config {
                no_extension: true,
                weak_reference_validation: true, // TODO: Must also work when this is false
            },
            &mut reader,
        )
        .expect("The schema itself is not valid");
}

/// Makes sure that we can correctly serialize and deserialize the schema and get the
/// same value back.
#[test]
fn can_serde_self_schema() {
    let mut builder = SchemaAnchorsBuilder::default();
    SchemaAnchors::build_schema(&mut builder);
    let anchors: SchemaAnchors = builder.try_into().unwrap();

    // serialize
    let mut writer = VecWriter::default();
    serialize(&mut writer, &anchors).expect("Unable to serialize schema");
    let serialized_data = writer.finish();
    let reader: SliceReader = (&serialized_data).into();

    // now try to de-serialize that
    let mut deserializer = new_deserializer(reader);
    let deserialized_value =
        SchemaAnchors::deserialize(&mut deserializer).expect("Unable to de-serialize schema");

    assert_eq!(&anchors, &deserialized_value);
}

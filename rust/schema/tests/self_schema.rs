// makes sure that the schema itself is valid

use liquesco_schema::core::Config;
use liquesco_schema::core::Schema;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_anchors::{SchemaAnchors, SchemaAnchorsBuilder};
use liquesco_schema::schema_builder::BuildsOwnSchema;
use liquesco_serialization::serde::{serialize_to_vec, de_serialize_from_slice};
use liquesco_serialization::slice_reader::SliceReader;
use std::convert::TryInto;

#[test]
fn test_self_schema_is_valid() {
    let mut builder = SchemaAnchorsBuilder::default();
    let main_type = SchemaAnchors::build_schema(&mut builder);
    let anchors: SchemaAnchors = builder.try_into().unwrap();

    let serialized_data = serialize_to_vec(&anchors)
        .expect("Unable to serialize value");

    // Now validate using itself

    let type_container = anchors;
    let mut schema = DefaultSchema::new(type_container, main_type);
    let mut reader: SliceReader = (&serialized_data).into();
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
    let serialized_data = serialize_to_vec(&anchors)
        .expect("Unable to serialize schema");

    // now try to de-serialize that
    let de_serialized_value =
        de_serialize_from_slice::<SchemaAnchors>(&serialized_data)
            .expect("Unable to de-serialize schema");

    assert_eq!(&anchors, &de_serialized_value);
}

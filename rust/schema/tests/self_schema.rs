// makes sure that the schema itself is valid

use liquesco_schema::core::Config;
use liquesco_schema::core::Schema;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_anchors::{SchemaAnchors, SchemaAnchorsBuilder};
use liquesco_schema::schema_builder::BuildsOwnSchema;
use liquesco_serialization::serde::serialize;
use liquesco_serialization::slice_reader::SliceReader;
use liquesco_serialization::vec_writer::VecWriter;
use std::convert::TryInto;

// TODO: This does not yet work #[test]
fn test_self_schema_is_valid() {
    let mut builder = SchemaAnchorsBuilder::default();
    SchemaAnchors::build_schema(&mut builder);
    let anchors: SchemaAnchors = builder.try_into().unwrap();

    let mut writer = VecWriter::default();
    serialize(&mut writer, &anchors).expect("Unable to serialize value");
    let serialized_data = writer.finish();
    let mut reader: SliceReader = (&serialized_data).into();

    // Now validate using itself

    let main_type = anchors.main_type();
    let type_container = anchors;
    let schema = DefaultSchema::new(type_container, main_type);
    schema
        .validate(Config { no_extension: true }, &mut reader)
        .expect("The schema itself is not valid");
}

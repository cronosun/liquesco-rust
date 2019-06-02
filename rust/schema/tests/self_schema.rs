// makes sure that the schema itself is valid

use liquesco_schema::core::Config;
use liquesco_schema::core::Schema;
use liquesco_schema::schema::{DefaultSchema, schema_schema};
use liquesco_schema::schema_builder::{BuildsOwnSchema, DefaultSchemaBuilder};
use liquesco_serialization::serde::{de_serialize_from_slice, serialize_to_vec};
use liquesco_serialization::slice_reader::SliceReader;
use std::convert::TryInto;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_schema::schema_builder::RootBuildsOwnSchema;

#[test]
fn test_self_schema_is_valid() {
    let type_container = build_liquesco_type_container();

    let serialized_data = serialize_to_vec(&type_container).expect("Unable to serialize value");

    // Now validate using itself
    let mut schema = build_liquesco_schema();
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
    let container = build_liquesco_type_container();

    // serialize
    let serialized_data = serialize_to_vec(&container).expect("Unable to serialize schema");

    // now try to de-serialize that
    let de_serialized_value = de_serialize_from_slice::<DefaultTypeContainer>(&serialized_data)
        .expect("Unable to de-serialize schema");

    assert_eq!(&container, &de_serialized_value);
}

fn build_liquesco_type_container() -> DefaultTypeContainer<'static> {
    let mut builder = DefaultSchemaBuilder::default();
    schema_schema(builder).unwrap()
}

fn build_liquesco_schema() ->  DefaultSchema<'static, DefaultTypeContainer<'static>>  {
    let type_container = build_liquesco_type_container();
    type_container.into()
}
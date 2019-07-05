use liquesco_parsing::yaml::parse_from_yaml_str;
use liquesco_schema::core::{Config, Schema};
use liquesco_schema::schema::{schema_schema, DefaultSchema};
use liquesco_schema::schema_builder::DefaultSchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_serialization::serde::de_serialize_from_slice;
use liquesco_serialization::slice_reader::SliceReader;

#[test]
fn test_simple_schema() {
    validate_schema_and_data(
        include_str!("simple_schema.yaml"),
        include_str!("simple_schema_data.yaml"),
    );
}

/// This performs multiple steps:
/// - It first converts your supplied yaml schema `schema` to binary data.
/// - Then it validates the supplied `schema` against the liquesco schema.
/// - Then it serializes the provided `data`.
/// - Then it validates the provided `data` against the supplied `schema`.
fn validate_schema_and_data(schema: &str, data: &str) {
    // this builds the liquesco schema
    let builder = DefaultSchemaBuilder::default();
    let type_container = schema_schema(builder).unwrap();
    let mut lq_schema: DefaultSchema<'static, DefaultTypeContainer<'static>> =
        type_container.into();
    lq_schema.set_extended_diagnostics(true);

    // parse my own test schema from yaml (this serializes the yaml to binary data)
    let my_own_schema = parse_from_yaml_str(&lq_schema, schema).unwrap();
    let mut reader: SliceReader = my_own_schema.as_slice().into();

    // now validate the parsed schema against the lq schema
    lq_schema
        .validate(Config { no_extension: true }, &mut reader)
        .expect("The schema itself is not valid");

    // now parse the supplied data (serialize to binary)
    let my_own_schema_de_serialized =
        de_serialize_from_slice::<DefaultTypeContainer>(my_own_schema.as_slice())
            .expect("Unable to de-serialize schema");
    let mut my_own_schema: DefaultSchema<'static, DefaultTypeContainer<'static>> =
        my_own_schema_de_serialized.into();
    let data_as_binary_vec = parse_from_yaml_str(&my_own_schema, data).unwrap();
    let mut data_as_reader: SliceReader = data_as_binary_vec.as_slice().into();

    // and now also validate the data against "my_own_schema"
    my_own_schema.set_extended_diagnostics(true);
    my_own_schema
        .validate(Config { no_extension: true }, &mut data_as_reader)
        .expect("Your own data is not valid (could not be validated against the provided schema)");
}

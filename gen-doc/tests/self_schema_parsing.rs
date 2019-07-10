use liquesco_gen_doc::create_model;
use liquesco_schema::core::TypeContainer;
use liquesco_schema::schema::schema_schema;
use liquesco_schema::schema_builder::DefaultSchemaBuilder;

#[test]
fn test_schema_to_model() {
    let builder = DefaultSchemaBuilder::default();
    let schema = schema_schema(builder).unwrap();
    let type_container: &TypeContainer = &schema;

    create_model(type_container).unwrap();
}

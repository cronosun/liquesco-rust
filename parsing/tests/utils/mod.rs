use liquesco_schema::core::TypeRef;
use liquesco_schema::identifier::Identifier;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::{DefaultSchemaBuilder, SchemaBuilder};
use liquesco_schema::type_container::DefaultTypeContainer;
use std::convert::TryInto;
use std::fmt::Debug;

pub fn assert_ok<T, R: Debug + Send + 'static>(result: Result<T, R>) {
    if result.is_err() {
        panic!(format!("Got error: {:?}", result.err().unwrap()))
    }
}

pub fn assert_err<T, R>(result: Result<T, R>) {
    assert!(result.is_err())
}

pub fn id(string: &'static str) -> Identifier<'static> {
    string.try_into().unwrap()
}

pub fn builder<'a>() -> DefaultSchemaBuilder<'a> {
    DefaultSchemaBuilder::default()
}

pub fn finish<'a>(
    builder: DefaultSchemaBuilder<'a>,
    root: TypeRef,
) -> DefaultSchema<'a, DefaultTypeContainer<'a>> {
    let finished = builder.finish(root).unwrap();
    let schema: DefaultSchema<'a, DefaultTypeContainer<'a>> = finished.into();
    schema.with_extended_diagnostics(true)
}

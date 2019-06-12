#![allow(dead_code)]

use liquesco_schema::core::TypeRef;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::{DefaultSchemaBuilder, SchemaBuilder};
use liquesco_schema::type_container::DefaultTypeContainer;

pub fn builder<'a>() -> DefaultSchemaBuilder<'a> {
    DefaultSchemaBuilder::default()
}

pub fn into_schema(
    builder: DefaultSchemaBuilder,
    root: TypeRef,
) -> DefaultSchema<DefaultTypeContainer> {
    let finished_builder: DefaultTypeContainer = builder.finish(root).unwrap();
    finished_builder.into()
}

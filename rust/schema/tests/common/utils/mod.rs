#![allow(dead_code)]

use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::Config;
use liquesco_schema::core::Schema;
use liquesco_schema::identifier::Identifier;
use liquesco_schema::schema::DefaultSchema;
use liquesco_schema::schema_builder::{DefaultSchemaBuilder, SchemaBuilder};
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_serialization::serde::serialize;
use liquesco_serialization::slice_reader::SliceReader;
use liquesco_serialization::vec_writer::VecWriter;
use std::convert::TryInto;
use std::fmt::Debug;

pub fn id(string: &'static str) -> Identifier<'static> {
    string.try_into().unwrap()
}

pub fn assert_valid_invalid<'a, S, TSchema>(
    item: S,
    schema: &TSchema,
    config: Config,
    expect_valid: bool,
) where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema<'a> + Sized,
{
    let mut writer = VecWriter::default();
    serialize(&mut writer, &item).expect("Unable to serialize value");
    let serialized_data = writer.finish();
    let mut reader: SliceReader = (&serialized_data).into();

    let result = schema.validate(config, &mut reader);

    if expect_valid {
        result.unwrap();
    } else {
        if result.is_ok() {
            panic!(format!(
                "Expecting value {:?} to be invalid but schema considers this as valid.",
                item
            ))
        }
    }
}

pub fn assert_valid_strict<'a, S, TSchema>(item: S, schema: &TSchema)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema<'a> + Sized,
{
    assert_valid_invalid(item, schema, Config::strict(), true);
}

pub fn assert_invalid_strict<'a, S, TSchema>(item: S, schema: &TSchema)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema<'a> + Sized,
{
    assert_valid_invalid(item, schema, Config::strict(), false);
}

pub fn assert_valid_extended<'a, S, TSchema>(item: S, schema: &TSchema)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema<'a> + Sized,
{
    assert_valid_invalid(
        item,
        schema,
        Config {
            no_extension: false,
        },
        true,
    );
}

pub fn assert_invalid_extended<'a, S, TSchema>(item: S, schema: &TSchema)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema<'a> + Sized,
{
    assert_valid_invalid(
        item,
        schema,
        Config {
            no_extension: false,
        },
        false,
    );
}

pub fn single_schema<'a, T: Into<AnyType<'a>>>(
    into_any_type: T,
) -> DefaultSchema<'a, DefaultTypeContainer<'a>> {
    let any_type = into_any_type.into();
    let builder = DefaultSchemaBuilder::default();
    builder.finish(any_type).unwrap().into()
}

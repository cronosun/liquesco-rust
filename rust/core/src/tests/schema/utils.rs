use crate::schema::core::Config;
use crate::schema::core::Schema;
use crate::schema::core::TypeContainer;
use crate::schema::core::TypeRef;
use crate::schema::identifier::Identifier;
use crate::schema::schema::DefaultSchema;
use crate::schema::any_type::AnyType;
use crate::serde::serialize;
use crate::serialization::slice_reader::SliceReader;
use crate::serialization::vec_writer::VecWriter;
use std::convert::TryInto;
use std::fmt::Debug;

pub fn id(string: &'static str) -> Identifier<'static> {
    string.try_into().unwrap()
}

pub fn assert_valid_invalid<S, TSchema>(
    item: S,
    schema: &TSchema,
    config: Config,
    expect_valid: bool,
) where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema + Sized,
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

pub fn assert_valid_strict<S, TSchema>(item: S, schema: &TSchema)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema + Sized,
{
    assert_valid_invalid(item, schema, Config::strict(), true);
}

pub fn assert_invalid_strict<S, TSchema>(item: S, schema: &TSchema)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema + Sized,
{
    assert_valid_invalid(item, schema, Config::strict(), false);
}

pub fn assert_valid_extended<S, TSchema>(item: S, schema: &TSchema)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema + Sized,
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

pub fn assert_invalid_extended<S, TSchema>(item: S, schema: &TSchema)
where
    S: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    TSchema: Schema + Sized,
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
) -> DefaultSchema<'a, SingleContainer<'a>> {
    let any_type = into_any_type.into();
    DefaultSchema::new(
        SingleContainer {
            any_type,
        },
        TypeRef(0),
    )
}

pub struct SingleContainer<'a> {
    any_type: AnyType<'a>,
}

impl<'a> TypeContainer<'a> for SingleContainer<'a> {
    fn maybe_type(&self, reference: TypeRef) -> Option<&AnyType<'a>> {
        if reference.0 == 0 {
            Option::Some(&self.any_type)
        } else {
            Option::None
        }
    }
}
use crate::schema::core::Config;
use crate::schema::core::Schema;
use crate::schema::core::ValidatorContainer;
use crate::schema::core::ValidatorRef;
use crate::schema::identifier::Identifier;
use crate::schema::schema::DefaultSchema;
use crate::schema::validators::AnyValidator;
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

pub fn single_schema<'a, V: Into<AnyValidator<'a>>>(
    validator: V,
) -> DefaultSchema<'a, SingleContainer<'a>> {
    let conv_validator = validator.into();
    DefaultSchema::new(
        SingleContainer {
            validator: conv_validator,
        },
        ValidatorRef(0),
    )
}

pub struct SingleContainer<'a> {
    validator: AnyValidator<'a>,
}

impl<'a> ValidatorContainer<'a> for SingleContainer<'a> {
    fn validator(&self, reference: ValidatorRef) -> Option<&AnyValidator<'a>> {
        if reference.0 == 0 {
            Option::Some(&self.validator)
        } else {
            Option::None
        }
    }
}

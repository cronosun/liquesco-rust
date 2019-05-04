use crate::common::error::LqError;
use crate::schema::context::DeSerializeContextStruct;
use crate::serialization::core::BinaryReader;

pub trait Validator<'a> {
    type DeSerItem;

    fn validate<S, R>(&self, schema: &S, reader: &mut R) -> Result<(), LqError>
    where
        S: Schema<'a>,
        R: BinaryReader<'a>;

    fn de_serialize<C>(context: &mut C) -> Result<Self::DeSerItem, LqError>
    where
        C: DeSerializationContext<'a>;
}

#[derive(new)]
pub struct Config {
    #[new(value = "false")]
    no_extension: bool,
}

impl Config {
    /// This returns true if e.g. extensions in structures (e.g. have more fields than defined in
    /// the schema) is not allowed.
    pub fn no_extension(&self) -> bool {
        self.no_extension
    }
}

pub trait DeSerializationContext<'a> {
    type Reader: BinaryReader<'a>;
    type Schema: Schema<'a>;

    fn reader(&mut self) -> &mut Self::Reader;
    fn de_serialize(&mut self) -> Result<ValidatorRef, LqError>;

    fn into_schema(self, config: Config) -> Result<(Self::Schema, ValidatorRef), LqError>;

    fn validate<R>(self, config: Config, reader: &mut R) -> Result<(), LqError>
    where
        R: BinaryReader<'a>,
        Self: Sized,
    {
        let (schema, reference) = self.into_schema(config)?;
        schema.validate(reader, reference)
    }
}

pub fn new_deserialzation_context<'a, R: BinaryReader<'a>>(
    reader: &'a mut R,
) -> impl DeSerializationContext<'a> {
    DeSerializeContextStruct::new(reader)
}

pub trait Schema<'a> {
    fn validate<R>(&self, reader: &mut R, reference: ValidatorRef) -> Result<(), LqError>
    where
        R: BinaryReader<'a>;

    fn config(&self) -> &Config;
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ValidatorRef(pub(crate) usize);

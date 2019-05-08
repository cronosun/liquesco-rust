use crate::common::error::LqError;
//use crate::schema::default_de_serialization_context::DefaultDeSerializationContext;
//use crate::schema::default_schema_builder::DefaultSchemaBuilder;
use crate::schema::validators::Validators;
use crate::serialization::core::BinaryReader;

pub trait Validator<'a> : Into<Validators<'a>> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>;

    fn into_any_validator(self) -> Validators<'a> {
        self.into()
    }
}

pub trait Context<'a> {
    type Reader: BinaryReader<'a>;

    fn validate(&mut self, reference: ValidatorRef) -> Result<(), LqError>;

    fn config(&self) -> &Config;

    fn reader(&mut self) -> &mut Self::Reader;
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

    pub fn strict() -> Self {
        Self {
            no_extension : true
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ValidatorRef(pub(crate) usize);

pub trait ValidatorContainer<'a> {
    // TODO: Rename to "validator"
    fn validators(&self, reference: ValidatorRef) -> Option<&Validators<'a>>;
}

pub trait Schema {
    fn validate<'r, R: BinaryReader<'r>>(
        &self,
        config: Config,
        reader: &mut R,
    ) -> Result<(), LqError>;
}

/****************************************** OLD ****************************************/

/*
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

pub fn new_de_serialization_context<'a, R: BinaryReader<'a>>(
    reader: &'a mut R,
) -> impl DeSerializationContext<'a> {
    DefaultDeSerializationContext::new(reader)
}

pub trait ValidatorReceiver<'a> {
    fn add(&mut self, validator: Validators<'a>) -> ValidatorRef;
}

pub trait SchemaBuilder<'a> : ValidatorReceiver<'a> {
    type Schema: Schema<'a>;

    fn into_schema(self, config: Config) -> Self::Schema;
}

pub fn new_schema_builder<'a>() -> impl SchemaBuilder<'a> {
    DefaultSchemaBuilder::default()
}

pub trait Schema<'a> {
    fn validate<R>(&self, reader: &mut R, reference: ValidatorRef) -> Result<(), LqError>
    where
        R: BinaryReader<'a>;

    fn config(&self) -> &Config;

    // TODO: Denke das kann weg... "nur noch validate"
    fn validator(&self, reference: ValidatorRef) -> Result<&Validators<'a>, LqError>;

    // TODO: Denke das kann weg
    fn serialize<W: BinaryWriter>(
        &self,
        writer: &mut W,
        reference: ValidatorRef,
    ) -> Result<(), LqError>
    where
        Self: Sized,
    {
        let validator = self.validator(reference)?;
        validator.serialize(self, writer)
    }
}*/

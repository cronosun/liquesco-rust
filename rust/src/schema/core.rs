use crate::common::error::LqError;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::BinaryReader;

pub trait Validator<'a> : Into<AnyValidator<'a>> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>;

    fn into_any_validator(self) -> AnyValidator<'a> {
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
    fn validator(&self, reference: ValidatorRef) -> Option<&AnyValidator<'a>>;
}

pub trait Schema {
    fn validate<'r, R: BinaryReader<'r>>(
        &self,
        config: Config,
        reader: &mut R,
    ) -> Result<(), LqError>;
}

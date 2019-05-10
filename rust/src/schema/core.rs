use crate::common::error::LqError;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::LqReader;

pub trait Validator<'a>: Into<AnyValidator<'a>> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>;

    fn into_any_validator(self) -> AnyValidator<'a> {
        self.into()
    }
}

pub trait Context<'a> {
    type Reader: LqReader<'a>;

    fn validate(&mut self, reference: ValidatorRef) -> Result<(), LqError>;

    fn config(&self) -> &Config;

    fn reader(&mut self) -> &mut Self::Reader;

    fn anchor_index(&self) -> Option<u32>;
    fn set_anchor_index(&mut self, value: Option<u32>);

    fn max_used_anchor_index(&self) -> Option<u32>;
    fn set_max_used_anchor_index(&mut self, value: Option<u32>);
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
        Self { no_extension: true }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ValidatorRef(pub(crate) usize);

pub trait ValidatorContainer<'a> {
    fn validator(&self, reference: ValidatorRef) -> Option<&AnyValidator<'a>>;
}

pub trait Schema {
    fn validate<'r, R: LqReader<'r>>(
        &self,
        config: Config,
        reader: &mut R,
    ) -> Result<(), LqError>;
}

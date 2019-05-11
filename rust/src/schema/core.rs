use crate::common::error::LqError;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::LqReader;
use std::cmp::Ordering;

pub trait Validator<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>;

    /// Compares r1 to r2. It's expected that you call this function only
    /// on data that has been validated successfully (if you call this on
    /// invalid data the behaviour is undefined).
    ///
    /// Important: The state of the given reader `r1` and `r2` is undefined
    /// unless this function returns `Ordering::Equal`. When inequality has
    /// been detected not all data will be read. Only reads all data when
    /// `Ordering::Equal` is returned.
    ///
    /// - Greater: if r1 > r2
    /// - Less: if r1 < r2
    /// - Equal: if r1 == r2
    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>;
}

pub trait Context<'a> {
    type Reader: LqReader<'a>;

    fn validate(&mut self, reference: ValidatorRef) -> Result<(), LqError>;

    /// See `Validator::compare`.
    fn compare(
        &self,
        reference: ValidatorRef,
        r1: &mut Self::Reader,
        r2: &mut Self::Reader,
    ) -> Result<Ordering, LqError>;

    fn reader(&mut self) -> &mut Self::Reader;

    fn config(&self) -> &Config;

    fn anchor_index(&self) -> Option<u32>;
    fn set_anchor_index(&mut self, value: Option<u32>);

    fn max_used_anchor_index(&self) -> Option<u32>;
    fn set_max_used_anchor_index(&mut self, value: Option<u32>);
}

#[derive(new)]
pub struct Config {
    #[new(value = "false")]
    pub no_extension: bool,
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
    fn validate<'r, R: LqReader<'r>>(&self, config: Config, reader: &mut R) -> Result<(), LqError>;
}

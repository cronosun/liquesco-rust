use crate::common::error::LqError;
use crate::serialization::core::BinaryReader;
use crate::serialization::dyn_reader::DynReader;

pub trait DynValidator<'a> {
    // TODO: Denke da wir nun das Enum haben kann man das DynReader wieder generisch machen
    fn validate(&self, reader: &mut DynReader<'a>, config: &Config) -> Result<(), LqError>;

    // TODO: max_len(&self) -> u64
}

pub trait Validator<'a> {
    fn validate<T: BinaryReader<'a>>(&self, reader: &mut T, config: &Config) -> Result<(), LqError>;
}

pub struct Config {
    no_extension: bool
}

impl Config {
    pub fn no_extension(&self) -> bool {
        self.no_extension
    }
}
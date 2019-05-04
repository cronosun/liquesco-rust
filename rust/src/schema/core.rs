use crate::common::error::LqError;
use crate::serialization::core::BinaryReader;
use crate::serialization::dyn_reader::DynReader;

pub trait Validator<'a> {
    type DeSerItem;

    fn validate<TContext, T>(
        &self,
        context: &mut TContext,
        reader: &mut T) -> Result<(), LqError> where
        TContext: ValidateContext,
        T: BinaryReader<'a>;

    fn de_serialize<TContext, T>(
        context: &mut TContext,
        reader: &mut T) -> Result<Self::DeSerItem, LqError> where
        TContext: DeSerializeContext,
        T: BinaryReader<'a>;
}

// TODO: Can be removed?
pub struct Config {
    no_extension: bool
}

// TODO: Can be removed?
impl Config {
    pub fn no_extension(&self) -> bool {
        self.no_extension
    }
}

pub trait DeSerializeContext<'a> {
    type Reader : BinaryReader<'a>;
    fn de_serialize(&mut self, reader: &mut Self::Reader) -> Result<ValidatorRef, LqError>;
}

pub trait ValidateContext {
    fn validate(&mut self, reference: ValidatorRef) -> Result<(), LqError>;

    /// This returns true if e.g. extensions in structures (e.g. have more fields than defined in
    /// the schema) is not allowed.
    fn no_extension(&self) -> bool;
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ValidatorRef(usize);
use crate::schema::core::Config;
use crate::schema::core::Schema;
use crate::common::error::LqError;
use crate::schema::core::ValidatorRef;
use crate::schema::validators::Validators;
use crate::serialization::core::BinaryReader;
use smallvec::SmallVec;

// 64 items should be enough for most schemas
pub type ValidatorsVec<'a> = SmallVec<[Validators<'a>; 64]>;

pub struct DefaultSchema<'a> {
    pub validators: ValidatorsVec<'a>,
    pub config : Config,
}

impl<'a> Schema<'a> for DefaultSchema<'a> {
    
    fn validate<R>(&self, reader: &mut R, reference: ValidatorRef) -> Result<(), LqError>
    where
        R: BinaryReader<'a> {
        let index = reference.0;
        let number_of_items = self.validators.len();
        if index>=number_of_items {
            return LqError::err_new(format!("There's no such validator at index {:?} (validator ref)", reference));
        }
        let validator = &self.validators[index];

        validator.validate(self, reader)
    }

    fn config(&self) -> &Config {
        &self.config
    }
    
}
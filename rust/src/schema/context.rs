use crate::schema::core::Config;
use crate::schema::core::Schema;
use crate::common::error::LqError;
use crate::schema::core::DeSerializationContext;
use crate::schema::core::ValidatorRef;
use crate::schema::validators::Validators;
use crate::serialization::core::BinaryReader;
use smallvec::SmallVec;

// 64 items should be enough for most schemas
type ValidatorsVec<'a> = SmallVec<[Validators<'a>; 64]>;

pub struct DeSerializeContextStruct<'a, R: BinaryReader<'a>> {
    reader: &'a mut R,    
    validators: ValidatorsVec<'a>,
}

impl<'a, R : BinaryReader<'a>> DeSerializeContextStruct<'a, R> {
    pub fn new(reader : &'a mut R) -> Self {
        Self {
            reader,
            validators : ValidatorsVec::new()
        }
    }
}

impl<'a, R: BinaryReader<'a>> DeSerializationContext<'a> for DeSerializeContextStruct<'a, R> {
    type Reader = R;
    type Schema = SchemaStruct<'a>;

    fn reader(&mut self) -> &mut Self::Reader {
        self.reader
    }

    fn de_serialize(&mut self) -> Result<ValidatorRef, LqError> {
        let validator = Validators::de_serialize(self)?;
        let number_of_items = self.validators.len();
        self.validators.push(validator);
        Result::Ok(ValidatorRef(number_of_items))
    }

    fn into_schema(mut self, config : Config) -> Result<(Self::Schema, ValidatorRef), LqError> {
        let reference = self.de_serialize()?;
        let validators = self.validators;
        let schema = Self::Schema {
            validators,
            config
        };
        Result::Ok((schema, reference))
    }
}

pub struct SchemaStruct<'a> {
    validators: ValidatorsVec<'a>,
    config : Config,
}

impl<'a> Schema<'a> for SchemaStruct<'a> {
    
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




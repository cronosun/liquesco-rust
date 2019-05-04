use crate::common::error::LqError;
use crate::schema::core::Config;
use crate::schema::core::DeSerializationContext;
use crate::schema::core::ValidatorRef;
use crate::schema::default_schema::DefaultSchema;
use crate::schema::default_schema::ValidatorsVec;
use crate::schema::validators::Validators;
use crate::serialization::core::BinaryReader;

pub struct DefaultDeSerializationContext<'a, R: BinaryReader<'a>> {
    reader: &'a mut R,
    validators: ValidatorsVec<'a>,
}

impl<'a, R: BinaryReader<'a>> DefaultDeSerializationContext<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Self {
            reader,
            validators: ValidatorsVec::new(),
        }
    }
}

impl<'a, R: BinaryReader<'a>> DeSerializationContext<'a> for DefaultDeSerializationContext<'a, R> {
    type Reader = R;
    type Schema = DefaultSchema<'a>;

    fn reader(&mut self) -> &mut Self::Reader {
        self.reader
    }

    fn de_serialize(&mut self) -> Result<ValidatorRef, LqError> {
        let validator = Validators::de_serialize(self)?;
        let number_of_items = self.validators.len();
        self.validators.push(validator);
        Result::Ok(ValidatorRef(number_of_items))
    }

    fn into_schema(mut self, config: Config) -> Result<(Self::Schema, ValidatorRef), LqError> {
        let reference = self.de_serialize()?;
        let validators = self.validators;
        let schema = Self::Schema { validators, config };
        Result::Ok((schema, reference))
    }
}

use crate::schema::core::Config;
use crate::schema::core::SchemaBuilder;
use crate::schema::core::ValidatorReceiver;
use crate::schema::core::ValidatorRef;
use crate::schema::default_schema::DefaultSchema;
use crate::schema::default_schema::ValidatorsVec;
use crate::schema::validators::Validators;

pub struct DefaultSchemaBuilder<'a> {
    validators: ValidatorsVec<'a>,
}

impl<'a> Default for DefaultSchemaBuilder<'a> {
    fn default() -> Self {
        Self {
            validators: ValidatorsVec::new(),
        }
    }
}

impl<'a> SchemaBuilder<'a> for DefaultSchemaBuilder<'a> {
    type Schema = DefaultSchema<'a>;

    fn into_schema(self, config: Config) -> Self::Schema {
        let validators = self.validators;
        Self::Schema { validators, config }
    }
}

impl<'a> ValidatorReceiver<'a> for DefaultSchemaBuilder<'a> {
    fn add(&mut self, validator: Validators<'a>) -> ValidatorRef {
        let index = self.validators.len();
        self.validators.push(validator);
        ValidatorRef(index)
    }
}

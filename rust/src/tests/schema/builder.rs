use crate::schema::core::Validator;
use crate::schema::core::ValidatorContainer;
use crate::schema::core::ValidatorRef;
use crate::schema::schema::DefaultSchema;
use crate::schema::validators::AnyValidator;

pub struct Builder<'a> {
    validators: Vec<AnyValidator<'a>>,
}

pub fn builder<'a>() -> Builder<'a> {
    Builder::default()
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Self {
            validators: Vec::new(),
        }
    }
}

pub struct Container<'a> {
    validators: Vec<AnyValidator<'a>>,
}

impl<'a> ValidatorContainer<'a> for Container<'a> {
    fn validator(&self, reference: ValidatorRef) -> Option<&AnyValidator<'a>> {
        let len = self.validators.len();
        if reference.0 >= len {
            Option::None
        } else {
            Option::Some(&self.validators[reference.0])
        }
    }
}

impl<'a> Builder<'a> {
    pub fn add<V: Validator<'a>>(&mut self, validator: V) -> ValidatorRef
    where AnyValidator<'a>: std::convert::From<V> {
        let reference = ValidatorRef(self.validators.len());
        self.validators.push(AnyValidator::from(validator));
        reference
    }

    pub fn finish<V: Validator<'a>>(
        mut self,
        validator: V,
    ) -> DefaultSchema<'a, Container<'a>>
    where AnyValidator<'a>: std::convert::From<V> {
        let reference = self.add(validator);
        let container = Container {
            validators: self.validators,
        };
        DefaultSchema::new(container, reference)
    }
}

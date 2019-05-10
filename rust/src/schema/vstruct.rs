use crate::common::error::LqError;
use crate::common::internal_utils::try_from_int_result;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::schema::core::ValidatorRef;
use crate::schema::identifier::Identifier;
use crate::schema::validators::AnyValidator;
use crate::serialization::core::LqReader;
use crate::serialization::core::DeSerializer;
use crate::serialization::tseq::SeqHeader;
use smallvec::SmallVec;
use std::convert::TryFrom;

/// Use a small vec with 5 items (should be enough for maybe 80% of all structs)
type Fields<'a> = SmallVec<[Field<'a>; 5]>;

#[derive(new, Clone)]
pub struct VStruct<'a>(Fields<'a>);

#[derive(new, Clone)]
pub struct Field<'a> {
    pub identifier: Identifier<'a>,
    pub validator: ValidatorRef,
}

impl<'a> Field<'a> {
    pub fn identifier(&self) -> &Identifier<'a> {
        &self.identifier
    }
}

impl<'a> Default for VStruct<'a> {
    fn default() -> Self {
        Self(Fields::new())
    }
}

impl<'a> VStruct<'a> {
    pub fn add(&mut self, field: Field<'a>) {
        self.0.push(field)
    }
}

impl<'a> Validator<'a> for VStruct<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let list = SeqHeader::de_serialize(context.reader())?;
        let schema_number_of_fields = try_from_int_result(u32::try_from(self.0.len()))?;
        let number_of_items = list.length();
        // length check
        if context.config().no_extension() {
            if number_of_items != schema_number_of_fields {
                return LqError::err_new(format!(
                    "Invalid number of items in struct. \
                     Need {:?} fields, have {:?} fields (strict mode)",
                    schema_number_of_fields, number_of_items
                ));
            }
        } else if number_of_items < schema_number_of_fields {
            return LqError::err_new(format!(
                "Some fields are missing in the given struct. \
                 Need at least {:?} fields, have {:?} fields.",
                schema_number_of_fields, number_of_items
            ));
        }
        // validate each item
        let schema_number_of_fields_usize =
            try_from_int_result(usize::try_from(schema_number_of_fields))?;
        for index in 0..schema_number_of_fields_usize {
            let field = &self.0[index];
            let validator = field.validator;
            context.validate(validator)?;
        }
        // skip the rest of the fields
        let to_skip = number_of_items - schema_number_of_fields;
        for _ in 0..to_skip {
            context.reader().skip()?;
        }
        Result::Ok(())
    }
}

impl<'a> From<VStruct<'a>> for AnyValidator<'a> {
    fn from(value: VStruct<'a>) -> Self {
        AnyValidator::Struct(value)
    }
}

impl<'a> VStruct<'a> {
    pub fn builder() -> Builder<'a> {
        Builder {
            fields: Fields::new(),
        }
    }
}

pub struct Builder<'a> {
    fields: Fields<'a>,
}

impl<'a> Builder<'a> {
    pub fn field<I: Into<Identifier<'a>>>(
        mut self,
        identifier: I,
        validator: ValidatorRef,
    ) -> Self {
        self.fields.push(Field {
            identifier: identifier.into(),
            validator,
        });
        self
    }

    pub fn build(self) -> VStruct<'a> {
        VStruct(self.fields)
    }
}

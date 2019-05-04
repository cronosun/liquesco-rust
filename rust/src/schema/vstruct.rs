use crate::common::error::LqError;
use crate::schema::core::Validator;
use crate::schema::core::{DeSerializationContext, Schema, ValidatorRef};
use crate::schema::identifier::Identifier;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::DeSerializer;
use crate::serialization::tlist::ListHeader;
use smallvec::SmallVec;

/// Use a small vec with 5 items (should be enough for maybe 80% of all structs)
type Fields<'a> = SmallVec<[Field<'a>; 5]>;

#[derive(new)]
pub struct VStruct<'a>(Fields<'a>);

#[derive(new)]
pub struct Field<'a> {
    identifier: Identifier<'a>,
    validator: ValidatorRef,
}

impl<'a> Field<'a> {
    pub fn identifier(&self) -> &Identifier<'a> {
        &self.identifier
    }
}

impl<'a> Validator<'a> for VStruct<'a> {
    type DeSerItem = Self;

    fn validate<S, R>(&self, schema: &S, reader : &mut R) -> Result<(), LqError>
    where
        S: Schema<'a>,
        R : BinaryReader<'a> {
        let list = ListHeader::de_serialize(reader)?;
        let schema_number_of_fields = self.0.len();
        let number_of_items = list.length();
        // length check
        if schema.config().no_extension() {
            if number_of_items != schema_number_of_fields {
                return LqError::err_new(format!(
                    "Invalid number of items in struct. \
                     Need {:?} fields, have {:?} fields (strict mode)",
                    schema_number_of_fields, number_of_items
                ));
            }
        } else {
            if number_of_items < schema_number_of_fields {
                return LqError::err_new(format!(
                    "Some fields are missing in the given struct. \
                     Need at least {:?} fields, have {:?} fields.",
                    schema_number_of_fields, number_of_items
                ));
            }
        }
        // validate each item
        for index in 0..schema_number_of_fields {
            let field = &self.0[index];
            let validator = field.validator;
            schema.validate(reader, validator)?;
        }
        // skip the rest
        let to_skip = number_of_items - schema_number_of_fields;
        for _ in 0..to_skip {
            reader.skip()?;
        }
        Result::Ok(())
    }

    fn de_serialize<TContext>(context: &mut TContext) -> Result<Self::DeSerItem, LqError>
    where
        TContext: DeSerializationContext<'a>,
    {
        let list_header = ListHeader::de_serialize(context.reader())?;
        let number_of_fields = list_header.length();
        let mut fields = Fields::with_capacity(number_of_fields);
        for _ in 0..number_of_fields {
            fields.push(de_serialize_field(context)?);
        }
        Result::Ok(Self(fields))
    }
}

fn de_serialize_field<'a, TContext>(context: &mut TContext) -> Result<Field<'a>, LqError>
where
    TContext: DeSerializationContext<'a>,
{
    let list_header = ListHeader::de_serialize(context.reader())?;

    let list_reader = list_header.begin(2)?;

    let identifier = Identifier::de_serialize(context.reader())?;
    let validator = context.de_serialize()?;
    let result = Result::Ok(Field {
        identifier,
        validator,
    });

    list_reader.finish(context.reader())?;

    result
}

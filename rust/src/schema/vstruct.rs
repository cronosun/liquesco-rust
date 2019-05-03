use crate::common::error::LqError;
use crate::schema::core::Config;
use crate::schema::core::Validator;
use crate::schema::identifier::Identifier;
use crate::schema::validators::Validators;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::DeSerializer;
use crate::serialization::tlist::ListHeader;

type Fields<'a> = Vec<Field<'a>>;

#[derive(new)]
pub struct VStruct<'a>(Fields<'a>);

#[derive(new)]
pub struct Field<'a> {
    identifier: Identifier<'a>,
    validator: Validators<'a>,
}

impl<'a> Field<'a> {
    pub fn identifier(&self) -> &Identifier<'a> {
        &self.identifier
    }
}

impl<'a> Validator<'a> for VStruct<'a> {
    fn validate<T: BinaryReader<'a>>(&self, reader: &mut T, config: &Config) -> Result<(), LqError> {
        let list = ListHeader::de_serialize(reader)?;
        let schema_number_of_fields = self.0.len();
        let number_of_items = list.length();
        // length check
        if config.no_extension() {
            if number_of_items != schema_number_of_fields {
                return LqError::err_new(format!("Invalid number of items in struct. \
                Need {:?} fields, have {:?} fields (strict mode)",
                                                schema_number_of_fields, number_of_items));
            }
        } else {
            if number_of_items < schema_number_of_fields {
                return LqError::err_new(format!("Some fields are missing in the given struct. \
                Need at least {:?} fields, have {:?} fields.",
                                                schema_number_of_fields, number_of_items));
            }
        }
        // check each item
        for index in 0..schema_number_of_fields {
            let field = &self.0[index];
            let validator = &field.validator;
            validator.validate(reader, config)?;
        }
        // skip the rest
        let to_skip = number_of_items - schema_number_of_fields;
        for _ in 0..to_skip {
            reader.skip()?;
        }
        Result::Ok(())
    }
}

impl<'a> DeSerializer<'a> for VStruct<'a> {
    type Item = Self;

    fn de_serialize<T: BinaryReader<'a>>(reader: &mut T) -> Result<Self::Item, LqError> {
        let list_header = ListHeader::de_serialize(reader)?;
        let number_of_fields = list_header.length();
        let mut fields = Vec::with_capacity(number_of_fields);
        for _ in 0..number_of_fields {
            fields.push(Field::de_serialize(reader)?);
        }
        Result::Ok(Self(fields))
    }
}

impl<'a> DeSerializer<'a> for Field<'a> {
    type Item = Self;

    fn de_serialize<T: BinaryReader<'a>>(reader: &mut T) -> Result<Self::Item, LqError> {
        let list_header = ListHeader::de_serialize(reader)?;
        list_header.read_struct(reader, 2, |reader| {
            let identifier = Identifier::de_serialize(reader)?;
            let validator = Validators::de_serialize(reader)?;
            Result::Ok(Field {
                identifier,
                validator,
            })
        })
    }
}
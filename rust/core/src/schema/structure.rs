use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::schema::core::TypeRef;
use crate::schema::identifier::Identifier;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LqReader;
use crate::serialization::seq::SeqHeader;
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::convert::TryFrom;

/// Use a small vec with 5 items (should be enough for maybe 80% of all structs)
type Fields<'a> = SmallVec<[Field<'a>; 5]>;

#[derive(new, Clone, Debug)]
pub struct TStruct<'a>(Fields<'a>);

#[derive(new, Clone, Debug)]
pub struct Field<'a> {
    pub identifier: Identifier<'a>,
    pub r#type: TypeRef,
}

impl<'a> Field<'a> {
    pub fn identifier(&self) -> &Identifier<'a> {
        &self.identifier
    }
}

impl<'a> Default for TStruct<'a> {
    fn default() -> Self {
        Self(Fields::new())
    }
}

impl<'a> TStruct<'a> {
    pub fn add(&mut self, field: Field<'a>) {
        self.0.push(field)
    }

    pub fn prepend(&mut self, field : Field<'a>) {
        self.0.insert(0, field)
    }
}

impl<'a> Type for TStruct<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let list = SeqHeader::de_serialize(context.reader())?;
        let schema_number_of_fields = u32::try_from(self.0.len())?;
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
            usize::try_from(schema_number_of_fields)?;
        for index in 0..schema_number_of_fields_usize {
            let field = &self.0[index];
            context.validate(field.r#type)?;
        }
        // skip the rest of the fields
        let to_skip = number_of_items - schema_number_of_fields;
        context.reader().skip_n_values_u32(to_skip)?;
        Result::Ok(())
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        // here it's important that we only compare what's defined in the schema. Why?
        // If we'd compare all fields it would be possible to add some arbitrary fields
        // and thus make it possible to add for example data that's not unique into a
        // sequence that requires uniqueness.

        let header1 = SeqHeader::de_serialize(r1)?;
        let header2 = SeqHeader::de_serialize(r2)?;

        let mut num_read: u32 = 0;
        for field in &self.0 {
            let cmp = context.compare(field.r#type, r1, r2)?;
            num_read = num_read + 1;
            if cmp != Ordering::Equal {
                // no need to finish to the end (see contract)
                return Result::Ok(cmp);
            }
        }

        // it's very important that we finish reading to the end (see contract)
        let finish_reading =
            |header: SeqHeader, reader: &mut LqReader, num_read: u32| -> Result<(), LqError> {
                let len = header.length();
                if len > num_read {
                    let missing = len - num_read;
                    reader.skip_n_values_u32(missing)
                } else {
                    Result::Ok(())
                }
            };

        // here we have to finish to the end
        finish_reading(header1, r1, num_read)?;
        finish_reading(header2, r2, num_read)?;

        Result::Ok(Ordering::Equal)
    }
}

impl<'a> TStruct<'a> {
    pub fn builder() -> Builder<'a> {
        Builder {
            fields: Fields::new(),
        }
    }

    pub fn fields(&self) -> &Fields<'a> {
       &self.0
    }
}

pub struct Builder<'a> {
    fields: Fields<'a>,
}

impl<'a> Builder<'a> {
    pub fn field<I: Into<Identifier<'a>>>(
        mut self,
        identifier: I,
        r#ype: TypeRef,
    ) -> Self {
        self.fields.push(Field {
            identifier: identifier.into(),
            r#type: r#ype,
        });
        self
    }

    pub fn build(self) -> TStruct<'a> {
        TStruct(self.fields)
    }
}

use crate::core::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::doc_type::DocType;
use crate::identifier::Identifier;
use crate::reference::TReference;
use crate::schema_builder::BuildsOwnSchema;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::seq::Ordering as SeqOrdering;
use crate::seq::TSeq;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U32IneRange;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::core::LqReader;
use liquesco_serialization::seq::SeqHeader;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

type Fields<'a> = Vec<Field<'a>>;

#[derive(new, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TStruct<'a> {
    fields: Fields<'a>,
}

#[derive(new, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Field<'a> {
    pub name: Identifier<'a>,
    pub r#type: TypeRef,
}

impl<'a> Field<'a> {
    pub fn name(&self) -> &Identifier<'a> {
        &self.name
    }

    pub fn r#type(&self) -> TypeRef {
        self.r#type
    }
}

impl<'a> Default for TStruct<'a> {
    fn default() -> Self {
        Self {
            fields: Fields::new(),
        }
    }
}

impl<'a> TStruct<'a> {
    pub fn add(mut self, field: Field<'a>) -> Self {
        self.fields.push(field);
        self
    }

    pub fn prepend(&mut self, field: Field<'a>) {
        self.fields.insert(0, field);
    }
}

impl<'a> Type for TStruct<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let list = SeqHeader::de_serialize(context.reader())?;
        let schema_number_of_fields = u32::try_from(self.fields().len())?;
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
        let schema_number_of_fields_usize = usize::try_from(schema_number_of_fields)?;
        for index in 0..schema_number_of_fields_usize {
            let field = &self.fields()[index];
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
        for field in self.fields() {
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

    fn reference(&self, index: usize) -> Option<TypeRef> {
        let number_of_fields = self.fields().len();
        if index >= number_of_fields {
            None
        } else {
            Some(self.fields()[index].r#type)
        }
    }
}

impl<'a> TStruct<'a> {
    pub fn fields(&self) -> &[Field<'a>] {
        &self.fields
    }
}

impl<'a> BaseTypeSchemaBuilder for TStruct<'a> {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        let identifier = Identifier::build_schema(builder);
        let r#type =
            builder.add(DocType::from(TReference::default()).with_name_unwrap("field_type"));
        let field_struct = builder.add(
            DocType::from(
                TStruct::default()
                    .add(Field::new(
                        Identifier::try_from("name").unwrap(),
                        identifier,
                    ))
                    .add(Field::new(Identifier::try_from("type").unwrap(), r#type)),
            )
            .with_name_unwrap("field")
            .with_description(
                "A single field in a structure. A field contains a name \
                 and a type.",
            ),
        );

        let fields_field = builder.add(
            DocType::from(TSeq {
                element: field_struct,
                length: U32IneRange::try_new("", std::u32::MIN, std::u32::MAX).unwrap(),
                ordering: SeqOrdering::None,
                multiple_of: None,
            })
            .with_name_unwrap("fields")
            .with_description("A sequence of fields in a structure."),
        );

        DocType::from(TStruct::default().add(Field::new(
            Identifier::try_from("fields").unwrap(),
            fields_field,
        )))
        .with_name_unwrap("struct")
        .with_description(
            "A structure is similar to a sequence but has a defined length and \
             can contain fields of different types.",
        )
    }
}

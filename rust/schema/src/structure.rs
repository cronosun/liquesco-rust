use crate::context::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::key_ref::TKeyRef;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::BuildsOwnSchema;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
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

/// A structure consists of 0-n fields.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TStruct<'a> {
    meta: Meta<'a>,
    fields: Fields<'a>,
}

/// A single field in a structure.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Field<'a> {
    name: Identifier<'a>,
    r#type: TypeRef,
}

impl<'a> Field<'a> {
    pub fn new(name: Identifier<'a>, r#type: TypeRef) -> Self {
        Self { name, r#type }
    }

    /// The name of the field.
    pub fn name(&self) -> &Identifier<'a> {
        &self.name
    }

    /// The type of the field value.
    pub fn r#type(&self) -> &TypeRef {
        &self.r#type
    }
}

impl<'a> Default for TStruct<'a> {
    fn default() -> Self {
        Self {
            meta: Meta::empty(),
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
            context.validate(&field.r#type)?;
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
            let cmp = context.compare(&field.r#type, r1, r2)?;
            num_read += 1;
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

    fn reference(&self, index: usize) -> Option<&TypeRef> {
        let number_of_fields = self.fields().len();
        if index >= number_of_fields {
            None
        } else {
            Some(&self.fields()[index].r#type)
        }
    }

    fn set_reference(&mut self, index: usize, type_ref: TypeRef) -> Result<(), LqError> {
        let number_of_fields = self.fields().len();
        if index >= number_of_fields {
            LqError::err_new(format!("Structure has no type at index {}", index))
        } else {
            self.fields[index].r#type = type_ref;
            Ok(())
        }
    }
}

impl WithMetadata for TStruct<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TStruct<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl<'a> TStruct<'a> {
    pub fn fields(&self) -> &[Field<'a>] {
        &self.fields
    }
}

impl<'a> BaseTypeSchemaBuilder for TStruct<'a> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        let identifier = Identifier::build_schema(builder);
        let r#type = builder.add_unwrap("field_type", TKeyRef::default());
        let field_struct = builder.add_unwrap(
            "field",
            TStruct::default()
                .add(Field::new(
                    Identifier::try_from("name").unwrap(),
                    identifier,
                ))
                .add(Field::new(Identifier::try_from("type").unwrap(), r#type))
                .with_doc(
                    "A single field in a structure. A field contains a name \
                     and a type.",
                ),
        );

        let fields_field = builder.add_unwrap(
            "fields",
            TSeq::new(
                field_struct,
                U32IneRange::try_new("", std::u32::MIN, std::u32::MAX).unwrap(),
            )
            .with_doc("A sequence of fields in a structure."),
        );

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("fields").unwrap(),
                fields_field,
            ))
            .with_doc(
                "A structure is similar to a sequence but has a defined length and \
                 can contain fields of different types.",
            )
    }
}

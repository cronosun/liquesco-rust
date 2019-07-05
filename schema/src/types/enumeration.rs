use crate::context::CmpContext;
use crate::context::ValidationContext;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::BuildsOwnSchema;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::types::key_ref::TKeyRef;
use crate::types::option::TOption;
use crate::types::seq::TSeq;
use crate::types::structure::Field;
use crate::types::structure::TStruct;
use lazy_static::lazy_static;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U32IneRange;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::core::LqReader;
use liquesco_serialization::types::enumeration::EnumHeader;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

const MIN_VALUES: usize = 1;
const MAX_VALUES: usize = 32;
const MIN_VARIANTS: usize = 1;

lazy_static! {
    static ref OK_IDENTIFIER: Identifier<'static> = { Identifier::try_from("ok").unwrap() };
    static ref ERR_IDENTIFIER: Identifier<'static> = { Identifier::try_from("err").unwrap() };
}

type Variants<'a> = Vec<Variant<'a>>;
type Values<'a> = Vec<TypeRef>;

/// An enumeration contains 1-n variants. Variants can (optionally) carry data.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TEnum<'a> {
    meta: Meta<'a>,
    variants: Variants<'a>,
}

/// A single variant of an enumeration.
#[derive(Clone, Debug, PartialEq, Hash, Serialize, Eq, Deserialize)]
pub struct Variant<'a> {
    /// Textual identifier of the variant.
    name: Identifier<'a>,

    /// The values this variant carries: This must contain > 0 items. It should only
    /// contain more then one item when you want to extend an existing schema and the value
    /// at index 0 is something you can't extend (a.g. not a struct).
    ///
    /// For variants without value, this is empty.
    values: Option<Values<'a>>,
}

/// About this enum. Can give the code generator more information about the enum; can be
/// used to choose the correct platform implementation when generating code.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct About {
    value_variants: bool,
    specialization: Specialization,
}

impl About {
    /// This is true when at least one of the variants can have a value. This is useful for the
    /// code generator: Some languages do not support enum variants with values.
    pub fn value_variants(&self) -> bool {
        self.value_variants
    }

    pub fn specialization(&self) -> Specialization {
        self.specialization
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Specialization {
    /// No specialization; just a plain enum.
    None,

    /// Enumeration is a result type; result types always have exactly two variants where
    /// the first variant is called 'ok' and the second variant is called 'err'.
    Result,
}

impl<'a> Variant<'a> {
    /// Create a new variant without values.
    pub fn new(name: Identifier<'a>) -> Self {
        Self { name, values: None }
    }

    /// Name of the variant.
    pub fn name(&self) -> &Identifier<'a> {
        &self.name
    }

    pub fn values(&self) -> &[TypeRef] {
        match &self.values {
            Option::None => &[],
            Option::Some(values) => values,
        }
    }

    pub fn add_value(mut self, value: TypeRef) -> Self {
        if self.values.is_none() {
            self.values = Some(Values::default());
        }
        let borrowed_self: &mut Self = &mut self;
        if let Some(values) = &mut borrowed_self.values {
            values.push(value);
        }
        self
    }
}

impl<'a> Default for TEnum<'a> {
    fn default() -> Self {
        Self {
            meta: Meta::empty(),
            variants: Variants::new(),
        }
    }
}

impl<'a> TEnum<'a> {
    pub fn variants(&self) -> &[Variant<'a>] {
        &self.variants
    }

    pub fn add_variant(mut self, variant: Variant<'a>) -> Self {
        self.variants.push(variant);
        self
    }

    pub fn variant_by_id<'b>(&self, id: &Identifier<'b>) -> Option<(u32, &Variant<'a>)> {
        // maybe better use a map for the variants?
        for (ordinal, variant) in self.variants.iter().enumerate() {
            if &variant.name == id {
                return Option::Some((ordinal as u32, variant));
            }
        }
        Option::None
    }

    pub fn about(&self) -> About {
        let specialization = self.specialization();

        let variant_with_value = self
            .variants()
            .iter()
            .find(|candidate| !candidate.values().is_empty());
        let value_variants = variant_with_value != None;

        About {
            value_variants,
            specialization,
        }
    }

    fn find_type_by_index(&self, index: usize) -> Option<TypePosition> {
        let mut current = 0;
        for (variant_index, variant) in self.variants().iter().enumerate() {
            for index_in_variant in 0..variant.values().len() {
                if current == index {
                    return Some(TypePosition {
                        variant_index,
                        index_in_variant,
                    });
                }
                current += 1;
            }
        }
        None
    }

    fn specialization(&self) -> Specialization {
        if self.variants().len() == 2 {
            let variant_ok = &self.variants()[0];
            let variant_err = &self.variants()[1];
            let ok_id: &Identifier<'static> = &OK_IDENTIFIER;
            let err_id: &Identifier<'static> = &ERR_IDENTIFIER;
            if variant_ok.name() == ok_id && variant_err.name() == err_id {
                Specialization::Result
            } else {
                Specialization::None
            }
        } else {
            Specialization::None
        }
    }
}

impl<'a> Type for TEnum<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: ValidationContext<'c>,
    {
        let enum_header = EnumHeader::de_serialize(context.reader())?;
        let number_of_values = enum_header.number_of_values();
        let ordinal = enum_header.ordinal();

        let number_of_variants = self.variants.len();

        let usize_ordinal = usize::try_from(ordinal)?;
        if usize_ordinal >= number_of_variants {
            return LqError::err_new(format!(
                "Got ordinal value {:?} for enum. \
                 There's no such variant defined for that ordinal value in \
                 the schema.",
                ordinal
            ));
        }
        let variant = &self.variants[usize_ordinal];

        let usize_number_of_values = usize::try_from(number_of_values)?;
        let schema_number_of_values = variant.values().len();
        if context.config().no_extension() && (schema_number_of_values != usize_number_of_values) {
            return LqError::err_new(format!(
                "Error processing enum variant {} (ordinal \
                 {}); strict mode: Schema expects {} values - have {} values in \
                 data.",
                variant.name(),
                ordinal,
                schema_number_of_values,
                usize_number_of_values
            ));
        } else if usize_number_of_values < schema_number_of_values {
            return LqError::err_new(format!(
                "Error processing enum variant {} (ordinal \
                 {}): Schema expects at least {} values - have {} values in \
                 data.",
                variant.name(),
                ordinal,
                schema_number_of_values,
                usize_number_of_values
            ));
        }

        let to_skip = usize_number_of_values - schema_number_of_values;

        // validate each element
        for r#type in variant.values() {
            context.validate(r#type)?;
        }

        if to_skip > 0 {
            context.reader().skip_n_values(to_skip)?;
        }

        Result::Ok(())
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: CmpContext<'c>,
    {
        let header1 = EnumHeader::de_serialize(r1)?;
        let header2 = EnumHeader::de_serialize(r2)?;

        // compare ordinals
        let ordinal_cmp = header1.ordinal().cmp(&header2.ordinal());
        if ordinal_cmp != Ordering::Equal {
            Result::Ok(ordinal_cmp)
        } else {
            // same ordinal, we also have to compare content: but important: We do only compare
            // the values that are defined in the schema. Why? If we'd compare more we could
            // just add some arbitrary data and thus add data that's unique (according to the
            // values in the schema) into a a sequence with a unique constraint.

            let ordinal = header1.ordinal();
            let usize_ordinal = usize::try_from(ordinal)?;
            let number_of_variants = self.variants.len();
            if usize_ordinal >= number_of_variants {
                return LqError::err_new(format!(
                    "Got ordinal value {:?} for enum. \
                     There's no such variant defined for that ordinal value in \
                     the schema.",
                    ordinal
                ));
            }

            let variant = &self.variants[usize_ordinal];
            let mut num_read: u32 = 0;
            for r#type in variant.values() {
                let cmp = context.compare(r#type, r1, r2)?;
                num_read += 1;
                if cmp != Ordering::Equal {
                    // no need to finish to the end (see contract)
                    return Result::Ok(cmp);
                }
            }

            // equal: read the rest (see contract)
            // it's very important that we finish reading to the end (see contract)
            let finish_reading =
                |header: EnumHeader, reader: &mut LqReader, num_read: u32| -> Result<(), LqError> {
                    let len = header.number_of_values();
                    if len > num_read {
                        let missing = len - num_read;
                        reader.skip_n_values_u32(missing)
                    } else {
                        Result::Ok(())
                    }
                };

            finish_reading(header1, r1, num_read)?;
            finish_reading(header2, r2, num_read)?;

            Result::Ok(Ordering::Equal)
        }
    }

    fn reference(&self, index: usize) -> Option<&TypeRef> {
        let position = self.find_type_by_index(index);
        if let Some(position) = position {
            Some(&self.variants()[position.variant_index].values()[position.index_in_variant])
        } else {
            None
        }
    }

    fn set_reference(&mut self, index: usize, type_ref: TypeRef) -> Result<(), LqError> {
        let position = self.find_type_by_index(index);
        if let Some(position) = position {
            let variant = &mut self.variants[position.variant_index];
            if let Some(values) = &mut variant.values {
                values[position.index_in_variant] = type_ref;
                Ok(())
            } else {
                LqError::err_new(format!(
                    "Enum has no type at index {} (note: this should \
                     not happen and seems to a bug in this library)",
                    index
                ))
            }
        } else {
            LqError::err_new(format!("Enum has no type at index {}", index))
        }
    }
}

struct TypePosition {
    variant_index: usize,
    index_in_variant: usize,
}

impl WithMetadata for TEnum<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TEnum<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

fn build_variant_schema<B>(builder: &mut B) -> TypeRef
where
    B: SchemaBuilder<'static>,
{
    let field_name = Identifier::build_schema(builder);

    let single_value = builder.add_unwrap(
        "value_type",
        TKeyRef::default().with_doc("Value type in an enum variant."),
    );
    let values = builder.add_unwrap(
        "values",
        TSeq::new(
            single_value,
            U32IneRange::try_new("", MIN_VALUES as u32, MAX_VALUES as u32).unwrap(),
        )
        .with_doc(
            "Defines the one (or in rare cases more) value the enumeration \
             variant takes. You should only have two or more values when variant got extended - \
             do not use more than one value in the initial schema design.",
        ),
    );
    let field_values = builder.add_unwrap(
        "maybe_values",
        TOption::new(values).with_doc(
            "Enumeration variants have usually either no value (in this case \
             this is absent) or one value.",
        ),
    );

    builder.add_unwrap(
        "variant",
        TStruct::default()
            .add(Field::new(
                Identifier::try_from("name").unwrap(),
                field_name,
            ))
            .add(Field::new(
                Identifier::try_from("values").unwrap(),
                field_values,
            ))
            .with_doc("A single variant in an enumeration."),
    )
}

impl<'a> BaseTypeSchemaBuilder for TEnum<'a> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        let variant = build_variant_schema(builder);
        let field_variants = builder.add_unwrap(
            "variants",
            TSeq::new(
                variant,
                U32IneRange::try_new("", MIN_VARIANTS as u32, std::u32::MAX).unwrap(),
            )
            .with_doc(
                "Every enumeration has to have one or more variants (just one usually \
                 makes no sense but can be used to allow extension in future).",
            ),
        );

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("variants").unwrap(),
                field_variants,
            ))
            .with_doc("An enumeration of variants.")
    }
}

use crate::common::error::LqError;
use crate::common::internal_utils::try_from_int_result;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::schema::core::TypeRef;
use crate::schema::identifier::Identifier;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LqReader;
use crate::serialization::enumeration::EnumHeader;
use std::cmp::Ordering;

use smallvec::SmallVec;
use std::convert::TryFrom;

/// Use a small vec with 5 items (should be enough for many cases)
type Variants<'a> = SmallVec<[Variant<'a>; 5]>;
type Values = SmallVec<[TypeRef; 3]>;

#[derive(new, Clone, Debug)]
pub struct TEnum<'a>(pub Variants<'a>);

#[derive(new, Clone, Debug)]
pub struct Variant<'a> {
    /// Textual identifier of the variant.
    pub identifier: Identifier<'a>,

    /// The values this variant carries: This should usually contain 0 or 1 item. It should only
    /// contain more then one item when you want to extend an existing schema and the value
    /// at index 0 is something you can't extend (a.g. not a struct).
    pub values: Values,
}

impl<'a> Variant<'a> {
    pub fn identifier(&self) -> &Identifier<'a> {
        &self.identifier
    }
}

impl<'a> Default for TEnum<'a> {
    fn default() -> Self {
        Self(Variants::new())
    }
}

impl<'a> TEnum<'a> {
    pub fn add(&mut self, variant: Variant<'a>) {
        self.0.push(variant)
    }
}

impl<'a> Type<'a> for TEnum<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let enum_header = EnumHeader::de_serialize(context.reader())?;
        let number_of_values = enum_header.number_of_values();
        let ordinal = enum_header.ordinal();

        let number_of_variants = self.0.len();

        let usize_ordinal = try_from_int_result(usize::try_from(ordinal))?;
        if usize_ordinal >= number_of_variants {
            return LqError::err_new(format!(
                "Got ordinal value {:?} for enum. \
                 There's no such variant defined for that ordinal value in \
                 the schema.",
                ordinal
            ));
        }
        let variant = &self.0[usize_ordinal];

        let usize_number_of_values = try_from_int_result(usize::try_from(number_of_values))?;
        let schema_number_of_values = variant.values.len();
        if context.config().no_extension() && (schema_number_of_values != usize_number_of_values) {
            return LqError::err_new(format!(
                "Error processing enum variant {:?} (ordinal \
                 {:?}); strict mode: Schema expects {:?} values - have {:?} values in \
                 data.",
                variant.identifier, ordinal, schema_number_of_values, usize_number_of_values
            ));
        } else if usize_number_of_values < schema_number_of_values {
            return LqError::err_new(format!(
                "Error processing enum variant {:?} (ordinal \
                 {:?}): Schema expects at least {:?} values - have {:?} values in \
                 data.",
                variant.identifier, ordinal, schema_number_of_values, usize_number_of_values
            ));
        }

        let to_skip = usize_number_of_values - schema_number_of_values;

        // validate each element
        for r#type in &variant.values {
            context.validate(*r#type)?;
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
        C: Context<'c>,
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
            let usize_ordinal = try_from_int_result(usize::try_from(ordinal))?;
            let number_of_variants = self.0.len();
            if usize_ordinal >= number_of_variants {
                return LqError::err_new(format!(
                    "Got ordinal value {:?} for enum. \
                     There's no such variant defined for that ordinal value in \
                     the schema.",
                    ordinal
                ));
            }

            let variant = &self.0[usize_ordinal];
            let mut num_read: u32 = 0;
            for r#type in &variant.values {
                let cmp = context.compare(*r#type, r1, r2)?;
                num_read = num_read + 1;
                if cmp != Ordering::Equal {
                    // no need to finish to the end (see contract)
                    return Result::Ok(cmp);
                }
            }

            // equal: read the rest (see contract)
            // it's very imporant that we finish reading to the end (see contract)
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
}

impl<'a> TEnum<'a> {
    pub fn builder() -> Builder<'a> {
        Builder {
            variants: Variants::new(),
        }
    }
}

pub struct Builder<'a> {
    variants: Variants<'a>,
}

impl<'a> Builder<'a> {
    pub fn variant<I: Into<Identifier<'a>>>(
        mut self,
        identifier: I,
        r#type: TypeRef,
    ) -> Self {
        let mut values = Values::with_capacity(1);
        values.push(r#type);

        self.variants.push(Variant {
            identifier: identifier.into(),
            values,
        });
        self
    }

    pub fn empty_variant<I: Into<Identifier<'a>>>(mut self, identifier: I) -> Self {
        let values = Values::with_capacity(0);
        self.variants.push(Variant {
            identifier: identifier.into(),
            values,
        });
        self
    }

    pub fn build(self) -> TEnum<'a> {
        TEnum(self.variants)
    }
}

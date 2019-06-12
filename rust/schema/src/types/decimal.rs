use crate::context::{CmpContext, ValidationContext};
use crate::core::{Type, TypeRef};
use crate::identifier::Identifier;
use crate::metadata::{Meta, MetadataSetter, WithMetadata};
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::types::range::{Inclusion, TRange};
use crate::types::sint::TSInt;
use crate::types::structure::{Field, TStruct};
use liquesco_common::decimal::Decimal;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::{I128IneRange, I8IneRange};
use liquesco_common::range::NewFull;
use liquesco_common::range::{LqRangeBounds, Range};
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::core::{LqReader, LqWriter, Serializer};
use liquesco_serialization::types::seq::SeqHeader;
use liquesco_serialization::types::sint::{SInt128, SInt8};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

/// The decimal type is a composed type of a 128 bit (signed) coefficient and an 8 bit (signed)
/// exponent (`value = c*10^e`).
///
/// Note: Composed: It should be possible to read and write decimal numbers without
/// knowing the schema. The composition of coefficient and exponent is a compromise of: simple
/// to read and write (for humans); fast to process (especially when the exponent is 0) and space.
/// If you have a huge array of decimals you might want to consider a more efficient type.
///
/// Note: To make sure there's only one representation for a given decimal number there are
/// some rules to make sure a number is valid: The value "zero" must be written as `0*10^0`;
/// `0*10^4` for example is invalid. The exponent must always be as close to zero as possible; so
/// `40*10^1` for example is invalid and has to be written as `400*10^0`; `-30*10^-1` must
/// be written as `-3*10^0`.
///
/// Note: NaN ("not a number") and infinity is not supported. Those values can be represented
/// using a wrapping enum.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TDecimal<'a> {
    meta: Meta<'a>,
    range: Range<Decimal>,
    coefficient_range: I128IneRange,
    exponent_range: I8IneRange,
}

impl<'a> TDecimal<'a> {
    /// A new decimal type. Full i128/i8 ranges.
    pub fn new(range: Range<Decimal>) -> Self {
        Self {
            meta: Meta::empty(),
            range,
            coefficient_range: I128IneRange::full(),
            exponent_range: I8IneRange::full(),
        }
    }

    /// A new decimal type, start and end included. Full i128/i8 ranges.
    pub fn try_new(start: Decimal, end: Decimal) -> Result<Self, LqError> {
        Ok(Self {
            meta: Meta::empty(),
            range: Range::try_new_inclusive(start, end)?,
            coefficient_range: I128IneRange::full(),
            exponent_range: I8IneRange::full(),
        })
    }

    /// The range the decimal must be within.
    pub fn range(&self) -> &Range<Decimal> {
        &self.range
    }

    /// The range the coefficient must be within.
    pub fn coefficient_range(&self) -> &I128IneRange {
        &self.coefficient_range
    }

    /// Can be used to narrow the coefficient range.
    pub fn with_coefficient_range(mut self, range: I128IneRange) -> Self {
        self.coefficient_range = range;
        self
    }

    /// The range the exponent must be within.
    pub fn exponent_range(&self) -> &I8IneRange {
        &self.exponent_range
    }

    /// Can be used to narrow the exponent range.
    pub fn with_exponent_range(mut self, range: I8IneRange) -> Self {
        self.exponent_range = range;
        self
    }
}

impl Type for TDecimal<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: ValidationContext<'c>,
    {
        let decimal = DecimalSerialization::de_serialize(context.reader())?;
        if !decimal.is_normalized() {
            return LqError::err_new(format!(
                "The given decimal has not been normalized: \
                 0 has to be represented as 0*10^0 and the exponent has to be as close to 0 \
                 as possible. Given value: {:?}.",
                decimal
            ));
        }

        self.coefficient_range.require_within(
            "Range of the coefficient of the given decimal (schema)",
            &decimal.coefficient(),
        )?;

        self.exponent_range.require_within(
            "Range of the exponent of the given decimal (schema)",
            &decimal.exponent(),
        )?;

        self.range.require_within(
            "Decimal range validation \
             (schema)",
            &decimal,
        )?;

        Ok(())
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: CmpContext<'c>,
    {
        let decimal1 = DecimalSerialization::de_serialize(r1)?;
        let decimal2 = DecimalSerialization::de_serialize(r2)?;

        Result::Ok(decimal1.cmp(&decimal2))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }

    fn set_reference(&mut self, _: usize, _: TypeRef) -> Result<(), LqError> {
        LqError::err_new("This type has no references")
    }
}

impl WithMetadata for TDecimal<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TDecimal<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl<'a> BaseTypeSchemaBuilder for TDecimal<'a> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
    {
        // range
        let range_element = builder.add_unwrap(
            "decimal_range_element",
            TDecimal::try_new(Decimal::MIN, Decimal::MAX)
                .unwrap()
                .with_doc(
                    "The start or end of the decimal range bounds. Note: Whether this is \
                     included or not can be defined.",
                ),
        );
        let range_field = builder.add_unwrap(
            "decimal_range",
            TRange::new(range_element, Inclusion::Supplied, false)
                .with_doc("The range the decimal number must be contained within."),
        );

        // coefficient range
        let coefficient_range_element = builder.add_unwrap(
            "coefficient_range_element",
            TSInt::try_new(std::i128::MIN, std::i128::MAX)
                .unwrap()
                .with_doc("The start or end of the decimal coefficient range (inclusive)."),
        );
        let coefficient_range_field = builder.add_unwrap(
            "coefficient_range",
            TRange::new(coefficient_range_element, Inclusion::BothInclusive, false)
                .with_doc("The range the decimal coefficient must be contained within."),
        );

        // exponent range
        let exponent_range_element = builder.add_unwrap(
            "exponent_range_element",
            TSInt::try_new(std::i8::MIN, std::i8::MAX)
                .unwrap()
                .with_doc("The start or end of the decimal exponent range (inclusive)."),
        );
        let exponent_range_field = builder.add_unwrap(
            "exponent_range",
            TRange::new(exponent_range_element, Inclusion::BothInclusive, false)
                .with_doc("The range the decimal exponent must be contained within."),
        );

        TStruct::default()
            .add(Field::new(
                Identifier::try_from("range").unwrap(),
                range_field,
            ))
            .add(Field::new(
                Identifier::try_from("coefficient_range").unwrap(),
                coefficient_range_field,
            ))
            .add(Field::new(
                Identifier::try_from("exponent_range").unwrap(),
                exponent_range_field,
            ))
            .with_doc(
                "A normalized decimal number. It's composed of a signed 128 bit \
                 coefficient and a signed 8 bit exponent (c*10^e).",
            )
    }
}

/// Serialization for the decimal type. Note: it does not normalize the decimal type.
pub struct DecimalSerialization;

impl<'a> DeSerializer<'a> for DecimalSerialization {
    type Item = Decimal;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let seq = SeqHeader::de_serialize(reader)?;
        if seq.length() != 2 {
            return LqError::err_new(format!(
                "The decimal type is a composed type (seq) of 2 \
                 fields: coefficient and exponent. This sequence has {} fields instead of 2.",
                seq.length()
            ));
        }
        let coefficient = SInt128::de_serialize(reader)?;
        let exponent = SInt8::de_serialize(reader)?;
        Ok(Decimal::from_parts_de_normalized(coefficient, exponent))
    }
}

impl Serializer for DecimalSerialization {
    type Item = Decimal;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        SeqHeader::serialize(writer, &SeqHeader::new(2))?;
        SInt128::serialize(writer, &item.coefficient())?;
        SInt8::serialize(writer, &item.exponent())?;
        Ok(())
    }
}

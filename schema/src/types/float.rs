use crate::context::CmpContext;
use crate::context::ValidationContext;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::WithMetadata;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::types::boolean::TBool;
use crate::types::range::Inclusion;
use crate::types::range::TRange;
use crate::types::structure::Field;
use crate::types::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_common::float::F32Ext;
use liquesco_common::float::F64Ext;
use liquesco_common::range::LqRangeBounds;
use liquesco_common::range::Range;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::types::float::Float32;
use liquesco_serialization::types::float::Float64;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::num::FpCategory;

/// A 32-bit floating point number.
pub type TFloat32<'a> = TFloat<'a, F32Ext>;
/// A 64-bit floating point number.
pub type TFloat64<'a> = TFloat<'a, F64Ext>;

const NOT_A_NUMBER_ERR_STR: &str = "Expected a float value that is a number. \
                                    This value is not a number (float NaN).";
const NO_POSITIVE_INFINITY: &str = "Positive infinity is not allowed for \
                                    this float value according to the schema.";
const NO_NEGATIVE_INFINITY: &str = "Negative infinity is not allowed for \
                                    this float value according to the schema.";
const NO_POSITIVE_ZERO: &str = "Negative zeros are not allowed according to the schema. \
This float \
value is a negative zero.";
const NO_NEGATIVE_ZERO: &str = "Positive zeros are not allowed according to \
the schema. This float \
value is a positive zero.";
const NO_SUBNORMAL: &str = "The given float value is a subnormal value. Subnormal values are \
not allowed according to the schema.";

/// A 32- or 64-bit floating point number.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct TFloat<'a, F: Eq + PartialOrd + Debug> {
    meta: Meta<'a>,
    range: Range<F>,
    allow_positive_zero: bool,
    allow_negative_zero: bool,
    allow_nan: bool,
    allow_positive_infinity: bool,
    allow_negative_infinity: bool,
    allow_subnormal: bool,
}

impl<F: Eq + PartialOrd + Debug> TFloat<'_, F> {
    /// Creates a new float. No special values allowed.
    pub fn new(range: Range<F>) -> Self {
        Self {
            meta: Meta::empty(),
            range,
            allow_positive_zero: false,
            allow_negative_zero: false,
            allow_nan: false,
            allow_positive_infinity: false,
            allow_negative_infinity: false,
            allow_subnormal: false,
        }
    }

    /// creates a new float; range inclusive; nan and infinity not allowed.
    pub fn try_new(min: F, max: F) -> Result<Self, LqError> {
        let range = Range::<F>::try_new_inclusive(min, max)?;
        Ok(Self::new(range))
    }

    /// The range the float must be within.
    pub fn range(&self) -> &Range<F> {
        &self.range
    }

    pub fn with_allow_positive_zero(mut self, allow: bool) -> Self {
        self.allow_positive_zero = allow;
        self
    }

    pub fn with_allow_negative_zero(mut self, allow: bool) -> Self {
        self.allow_negative_zero = allow;
        self
    }

    pub fn with_allow_nan(mut self, allow: bool) -> Self {
        self.allow_nan = allow;
        self
    }

    pub fn with_allow_positive_infinity(mut self, allow: bool) -> Self {
        self.allow_positive_infinity = allow;
        self
    }

    pub fn with_allow_negative_infinity(mut self, allow: bool) -> Self {
        self.allow_negative_infinity = allow;
        self
    }

    pub fn with_allow_subnormal(mut self, allow: bool) -> Self {
        self.allow_subnormal = allow;
        self
    }

    /// True: positive zero numbers are allowed.
    pub fn allow_positive_zero(&self) -> bool {
        self.allow_positive_zero
    }

    /// True: negative zero numbers are allowed.
    pub fn allow_negative_zero(&self) -> bool {
        self.allow_negative_zero
    }

    /// True if "not a number" (NaN) is allowed.
    pub fn allow_nan(&self) -> bool {
        self.allow_nan
    }

    /// True if positive infinity is allowed.
    pub fn allow_positive_infinity(&self) -> bool {
        self.allow_positive_infinity
    }

    /// True if negative infinity is allowed.
    pub fn allow_negative_infinity(&self) -> bool {
        self.allow_negative_infinity
    }

    /// True: subnormal numbers are allowed (note: does not include the special case for zero).
    pub fn allow_subnormal(&self) -> bool {
        self.allow_subnormal
    }

    fn validate(
        &self,
        value: F,
        info: ValidationInfo,
    ) -> Result<(), LqError> {
        let need_range_check = match info.category {
            FpCategory::Nan => {
                if !self.allow_nan {
                    return LqError::err_new(NOT_A_NUMBER_ERR_STR);
                }
                false
            }
            FpCategory::Infinite => {
                if info.negative {
                    if !self.allow_negative_infinity {
                        return LqError::err_new(NO_NEGATIVE_INFINITY);
                    }
                } else {
                    if !self.allow_positive_infinity {
                        return LqError::err_new(NO_POSITIVE_INFINITY);
                    }
                }
                false
            }
            FpCategory::Zero => {
                if info.negative {
                    if !self.allow_negative_zero {
                        return LqError::err_new(NO_NEGATIVE_ZERO);
                    }
                    false
                } else {
                    if !self.allow_positive_zero {
                        return LqError::err_new(NO_POSITIVE_ZERO);
                    }
                    false
                }
            }
            FpCategory::Subnormal => {
                if !self.allow_subnormal {
                    return LqError::err_new(NO_SUBNORMAL);
                }
                true
            }
            FpCategory::Normal => {
                true
            }
        };

        if need_range_check {
            // it's a number
            self.range.require_within(
                "Float range validation \
                 (schema)",
                &value,
            )?;
        }
        Ok(())
    }
}

struct ValidationInfo {
    category: FpCategory,
    negative: bool,
}

impl Type for TFloat32<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
        where
            C: ValidationContext<'c>,
    {
        let float_value = Float32::de_serialize(context.reader())?;
        let category = float_value.classify();
        let negative = float_value.is_sign_negative();
        let info = ValidationInfo {
            category,
            negative,
        };
        self.validate(float_value.into(), info)
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
        let float1 = Float32::de_serialize(r1)?;
        let float2 = Float32::de_serialize(r2)?;

        Result::Ok(F32Ext::from(float1).cmp(&F32Ext::from(float2)))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }

    fn set_reference(&mut self, _: usize, _: TypeRef) -> Result<(), LqError> {
        LqError::err_new("This type has no references")
    }
}

impl WithMetadata for TFloat32<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TFloat32<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl Type for TFloat64<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
        where
            C: ValidationContext<'c>,
    {
        let float_value = Float64::de_serialize(context.reader())?;
        let category = float_value.classify();
        let negative = float_value.is_sign_negative();
        let info = ValidationInfo {
            category,
            negative,
        };
        self.validate(float_value.into(), info)
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
        let float1 = Float64::de_serialize(r1)?;
        let float2 = Float64::de_serialize(r2)?;
        Result::Ok(F64Ext::from(float1).cmp(&F64Ext::from(float2)))
    }

    fn reference(&self, _: usize) -> Option<&TypeRef> {
        None
    }

    fn set_reference(&mut self, _: usize, _: TypeRef) -> Result<(), LqError> {
        LqError::err_new("This type has no references")
    }
}

impl WithMetadata for TFloat64<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TFloat64<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

fn build_schema<B>(builder: &mut B, float_32: bool) -> TStruct<'static>
    where
        B: SchemaBuilder<'static>,
{
    // range
    let range_item = if float_32 {
        builder.add_unwrap(
            "float_32_range_element",
            TFloat32::try_new(std::f32::MIN.into(), std::f32::MAX.into())
                .unwrap()
                .with_doc(
                    "The start or end of the float range bounds. Note: Whether this is \
                     included or not can be defined.",
                ),
        )
    } else {
        builder.add_unwrap(
            "float_64_range_element",
            TFloat64::try_new(std::f64::MIN.into(), std::f64::MAX.into())
                .unwrap()

                .with_doc(
                    "The start or end of the float range bounds. Note: Whether this is \
                     included or not can be defined.",
                ),
        )
    };

    let range_field = builder.add_unwrap(
        if float_32 {
            "float_32_range"
        } else {
            "float_64_range"
        },
        TRange::new(range_item, Inclusion::Supplied, false)
            .with_doc("The range the float must be contained within. Note: Only normal \
            numbers are allowed (so no subnormal numbers and NaN are supported); so for \
            example to support floats >=0, set start to smallest positive number (inclusive), \
            allow positive zeros and (optionally) allow subnormal numbers."),
    );

    // other config

    let allow_positive_zero = builder.add_unwrap(
        "allow_positive_zero",
        TBool::default().with_doc(
            "If this is true, the subnormal positive zero value is allowed. When a positive \
            zero is hit, range check is skipped.",
        ),
    );
    let allow_negative_zero = builder.add_unwrap(
        "allow_negative_zero",
        TBool::default().with_doc(
            "If this is true, the subnormal negative zero value is allowed. This should \
            usually be false. When e negative zero value is hit, range check is skipped.",
        ),
    );
    let allow_nan_field = builder.add_unwrap(
        "allow_nan",
        TBool::default().with_doc(
            "This is true if NaN ('not a number') is allowed. This \
             should usually be false. When a NaN is hit, range check is skipped.",
        ),
    );
    let allow_positive_infinity_field = builder.add_unwrap(
        "allow_positive_infinity",
        TBool::default().with_doc("This is true if positive infinity is allowed. When \
        the positive infinity is hit, range check is skipped."),
    );
    let allow_negative_infinity_field = builder.add_unwrap(
        "allow_negative_infinity",
        TBool::default().with_doc("This is true if negative infinity is allowed. When \
        the negative infinity is hit, range check is skipped."),
    );
    let allow_subnormal = builder.add_unwrap(
        "allow_subnormal",
        TBool::default().with_doc("If this is true, subnormal (denormal) \
        numbers are allowed (excluding the special case for +/- zero). You usually want \
        this to be false. Range check is also performed for subnormal numbers."),
    );

    // just an empty struct (but more fields will be added by the system)
    TStruct::default()
        .add(Field::new(
            Identifier::try_from("range").unwrap(),
            range_field,
        ))
        .add(Field::new(
            Identifier::try_from("allow_positive_zero").unwrap(),
            allow_positive_zero,
        ))
        .add(Field::new(
            Identifier::try_from("allow_negative_zero").unwrap(),
            allow_negative_zero,
        ))
        .add(Field::new(
            Identifier::try_from("allow_nan").unwrap(),
            allow_nan_field,
        ))
        .add(Field::new(
            Identifier::try_from("allow_positive_infinity").unwrap(),
            allow_positive_infinity_field,
        ))
        .add(Field::new(
            Identifier::try_from("allow_negative_infinity").unwrap(),
            allow_negative_infinity_field,
        ))
        .add(Field::new(
            Identifier::try_from("allow_subnormal").unwrap(),
            allow_subnormal,
        ))
        .with_doc(
            "A floating point number. This should be used for technical things. When \
             working with numbers like amount of money consider using the decimal type.",
        )
}

impl BaseTypeSchemaBuilder for Float32 {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
        where
            B: SchemaBuilder<'static>,
    {
        build_schema(builder, true)
    }
}

impl BaseTypeSchemaBuilder for TFloat<'_, F32Ext> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
        where
            B: SchemaBuilder<'static>,
    {
        build_schema(builder, true)
    }
}

impl BaseTypeSchemaBuilder for Float64 {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
        where
            B: SchemaBuilder<'static>,
    {
        build_schema(builder, false)
    }
}

impl BaseTypeSchemaBuilder for TFloat<'_, F64Ext> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
        where
            B: SchemaBuilder<'static>,
    {
        build_schema(builder, false)
    }
}

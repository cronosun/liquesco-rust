use crate::boolean::TBool;
use crate::core::TypeRef;
use crate::core::{Context, Type};
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::NameDescription;
use crate::metadata::NameOnly;
use crate::metadata::WithMetadata;
use crate::range::Inclusion;
use crate::range::TRange;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::Field;
use crate::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_common::float::F32Ext;
use liquesco_common::float::F64Ext;
use liquesco_common::range::LqRangeBounds;
use liquesco_common::range::Range;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::float::Float32;
use liquesco_serialization::float::Float64;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Debug;

pub type TFloat32<'a> = TFloat<'a, F32Ext>;
pub type TFloat64<'a> = TFloat<'a, F64Ext>;

const NOT_A_NUMBER_ERR_STR: &str = "Expected a float value that is a number. \
                                    This value is not a number (float NaN).";
const NO_POSITIVE_INFINITY: &str = "Positive infinity is not allowed for \
                                    this float value according to the schema.";
const NO_NEGATIVE_INFINITY: &str = "Negative infinity is not allowed for \
                                    this float value according to the schema.";

#[derive(new, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct TFloat<'a, F: Eq + PartialOrd + Debug> {
    #[new(value = "Meta::empty()")]
    pub meta: Meta<'a>,
    pub range: Range<F>,
    #[new(value = "false")]
    pub allow_nan: bool,
    #[new(value = "false")]
    pub allow_positive_infinity: bool,
    #[new(value = "false")]
    pub allow_negative_infinity: bool,
}

impl<F: Eq + PartialOrd + Debug> TFloat<'_, F> {
    /// creates a new float; range inclusive; nan and infinity not allowed.
    pub fn try_new(min: F, max: F) -> Result<Self, LqError> {
        let range = Range::<F>::try_new_inclusive(min, max)?;
        Ok(Self::new(range))
    }

    pub fn range(&self) -> &Range<F> {
        &self.range
    }

    fn validate(
        &self,
        value: F,
        is_nan: bool,
        is_positive_infinity: bool,
        is_negative_infinity: bool,
    ) -> Result<(), LqError> {
        if is_nan {
            // validation for not-a-number
            if !self.allow_nan {
                return LqError::err_new(NOT_A_NUMBER_ERR_STR);
            }
            Result::Ok(())
        } else if is_positive_infinity {
            if !self.allow_positive_infinity {
                return LqError::err_new(NO_POSITIVE_INFINITY);
            }
            Result::Ok(())
        } else if is_negative_infinity {
            if !self.allow_positive_infinity {
                return LqError::err_new(NO_NEGATIVE_INFINITY);
            }
            Result::Ok(())
        } else {
            // it's a number
            self.range.require_within(
                "Float range validation \
                 (schema)",
                &value,
            )
        }
    }
}

impl Type for TFloat32<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let float_value = Float32::de_serialize(context.reader())?;
        let (is_nan, is_p_infinite, is_n_infinite) = if float_value.is_nan() {
            (true, false, false)
        } else {
            if float_value.is_infinite() {
                let negative = float_value.is_sign_negative();
                (false, !negative, negative)
            } else {
                (false, false, false)
            }
        };

        self.validate(float_value.into(), is_nan, is_p_infinite, is_n_infinite)
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        let float1 = Float32::de_serialize(r1)?;
        let float2 = Float32::de_serialize(r2)?;

        Result::Ok(F32Ext::from(float1).cmp(&F32Ext::from(float2)))
    }

    fn reference(&self, _: usize) -> Option<TypeRef> {
        None
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
        C: Context<'c>,
    {
        let float_value = Float64::de_serialize(context.reader())?;
        let (is_nan, is_p_infinite, is_n_infinite) = if float_value.is_nan() {
            (true, false, false)
        } else {
            if float_value.is_infinite() {
                let negative = float_value.is_sign_negative();
                (false, !negative, negative)
            } else {
                (false, false, false)
            }
        };

        self.validate(float_value.into(), is_nan, is_p_infinite, is_n_infinite)
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        let float1 = Float64::de_serialize(r1)?;
        let float2 = Float64::de_serialize(r2)?;
        Result::Ok(F64Ext::from(float1).cmp(&F64Ext::from(float2)))
    }

    fn reference(&self, _: usize) -> Option<TypeRef> {
        None
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
    B: SchemaBuilder,
{
    // range
    let range_item = if float_32 {
        builder.add(
            TFloat32::try_new(std::f32::MIN.into(), std::f32::MAX.into())
                .unwrap()
                .with_meta(NameDescription {
                    name: "float_32_range_element",
                    description:
                        "The start or end of the float range bounds. Note: Whether this is \
                         included or not can be defined.",
                }),
        )
    } else {
        builder.add(
            TFloat64::try_new(std::f64::MIN.into(), std::f64::MAX.into())
                .unwrap()
                .with_meta(NameDescription {
                    name: "float_64_range_element",
                    description:
                        "The start or end of the float range bounds. Note: Whether this is \
                         included or not can be defined.",
                }),
        )
    };

    let range_field = builder.add(
        TRange {
            meta: Meta::empty(),
            element: range_item,
            inclusion: Inclusion::Supplied,
            allow_empty: false,
        }
        .with_meta(NameDescription {
            name: if float_32 {
                "float_32_range"
            } else {
                "float_64_range"
            },
            description: "The range the float must be contained within.",
        }),
    );

    // other config

    let allow_nan_field = builder.add(TBool::default().with_meta(NameDescription {
        name: "allow_nan",
        description: "This is true if NaN ('not a number') is allowed. This \
                      should usually be false.",
    }));
    let allow_positive_infinity_field = builder.add(TBool::default().with_meta(NameDescription {
        name: "allow_positive_infinity",
        description: "This is true if positive infinity is allowed.",
    }));
    let allow_negative_infinity_field = builder.add(TBool::default().with_meta(NameDescription {
        name: "allow_negative_infinity",
        description: "This is true if negative infinity is allowed.",
    }));

    // just an empty struct (but more fields will be added by the system)
    TStruct::default()
        .add(Field::new(
            Identifier::try_from("range").unwrap(),
            range_field,
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
        .with_meta(NameOnly {
            name: if float_32 { "float_32" } else { "float_64" },
        })
}

impl BaseTypeSchemaBuilder for Float32 {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        build_schema(builder, true)
    }
}

impl BaseTypeSchemaBuilder for TFloat<'_, F32Ext> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        build_schema(builder, true)
    }
}

impl BaseTypeSchemaBuilder for Float64 {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        build_schema(builder, false)
    }
}

impl BaseTypeSchemaBuilder for TFloat<'_, F64Ext> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        build_schema(builder, false)
    }
}

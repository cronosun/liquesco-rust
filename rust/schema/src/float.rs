use crate::boolean::TBool;
use crate::core::{Context, Type};
use crate::doc_type::DocType;
use crate::identifier::Identifier;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::seq::Direction::Ascending;
use crate::seq::Ordering as SeqOrdering;
use crate::seq::TSeq;
use crate::structure::Field;
use crate::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_common::float::F32Ext;
use liquesco_common::float::F64Ext;
use liquesco_common::ine_range::IneRange;
use liquesco_common::range::LqRangeBounds;
use liquesco_common::range::Range;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::float::Float32;
use liquesco_serialization::float::Float64;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt::Debug;

pub type TFloat32 = TFloat<F32Ext>;
pub type TFloat64 = TFloat<F64Ext>;

const NOT_A_NUMBER_ERR_STR: &str = "Expected a float value that is a number. \
                                    This value is not a number (float NaN).";
const NO_POSITIVE_INFINITY: &str = "Positive infinity is not allowed for \
                                    this float value according to the schema.";
const NO_NEGATIVE_INFINITY: &str = "Negative infinity is not allowed for \
                                    this float value according to the schema.";

#[derive(new, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct TFloat<F: Eq + PartialOrd + Debug> {
    #[serde(flatten)]
    pub range: Range<F>,
    #[new(value = "false")]
    #[serde(default)]
    pub allow_nan: bool,
    #[new(value = "false")]
    #[serde(default)]
    pub allow_positive_infinity: bool,
    #[new(value = "false")]
    #[serde(default)]
    pub allow_negative_infinity: bool,
}

impl<F: Eq + PartialOrd + Debug> TFloat<F> {
    /// creates a new float; range inclusive; nan and infinity not allowed.
    pub fn try_new(min: F, max: F) -> Result<Self, LqError> {
        let range = Range::<F>::try_inclusive(min, max)?;
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
                return LqError::err_static(NOT_A_NUMBER_ERR_STR);
            }
            Result::Ok(())
        } else if is_positive_infinity {
            if !self.allow_positive_infinity {
                return LqError::err_static(NO_POSITIVE_INFINITY);
            }
            Result::Ok(())
        } else if is_negative_infinity {
            if !self.allow_positive_infinity {
                return LqError::err_static(NO_NEGATIVE_INFINITY);
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

impl Type for TFloat32 {
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
}

impl Type for TFloat64 {
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
}

fn build_schema<B>(builder: &mut B, float_32: bool) -> DocType<'static, TStruct<'static>>
where
    B: SchemaBuilder,
{
    // range
    let range_item = if float_32 {
        builder.add(
            DocType::from(TFloat32::try_new(std::f32::MIN.into(), std::f32::MAX.into()).unwrap())
                .with_name_unwrap("float_bounds_element")
                .with_description(
                    "The start or end of the float range bounds. Note: Whether this is \
                     included or not can be defined.",
                ),
        )
    } else {
        builder.add(
            DocType::from(TFloat64::try_new(std::f64::MIN.into(), std::f64::MAX.into()).unwrap())
                .with_name_unwrap("float_bounds_element")
                .with_description(
                    "The start or end of the float range bounds. Note: Whether this is \
                     included or not can be defined.",
                ),
        )
    };
    let bounds_field = builder.add(
        DocType::from(TSeq {
            element: range_item,
            length: IneRange::try_new(2, 2).unwrap(),
            ordering: SeqOrdering::Sorted {
                direction: Ascending,
                unique: true,
            },
            multiple_of: None,
        })
        .with_name_unwrap("float_bounds")
        .with_description(
            "The bounds the float must be contained within. It's two values: The \
             first value is the bounds start, the second value is the bounds end.",
        ),
    );

    let start_included_field = builder.add(
        DocType::from(TBool)
            .with_name_unwrap("start_included")
            .with_description(
                "This is true if you want the start of the given bounds to be included.",
            ),
    );
    let end_included_field = builder.add(
        DocType::from(TBool)
            .with_name_unwrap("end_included")
            .with_description(
                "This is true if you want the end of the given bounds to be included.",
            ),
    );

    // other config

    let allow_nan_field = builder.add(
        DocType::from(TBool)
            .with_name_unwrap("allow_nan")
            .with_description(
                "This is true if NaN ('not a number') is allowed. This \
                 should usually be false.",
            ),
    );
    let allow_positive_infinity_field = builder.add(
        DocType::from(TBool)
            .with_name_unwrap("allow_positive_infinity")
            .with_description("This is true if posive infinity is allowed."),
    );
    let allow_negative_infinity_field = builder.add(
        DocType::from(TBool)
            .with_name_unwrap("allow_negative_infinity")
            .with_description("This is true if negative infinity is allowed."),
    );

    // just an empty struct (but more fields will be added by the system)
    DocType::from(
        TStruct::default()
            .add(Field::new(
                Identifier::try_from("bounds").unwrap(),
                bounds_field,
            ))
            .add(Field::new(
                Identifier::try_from("start_included").unwrap(),
                start_included_field,
            ))
            .add(Field::new(
                Identifier::try_from("end_included").unwrap(),
                end_included_field,
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
            )),
    )
    .with_name_unwrap(if float_32 { "float_32" } else { "float_64" })
}

impl BaseTypeSchemaBuilder for Float32 {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        build_schema(builder, true)
    }
}

impl BaseTypeSchemaBuilder for TFloat<F32Ext> {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        build_schema(builder, true)
    }
}

impl BaseTypeSchemaBuilder for Float64 {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        build_schema(builder, false)
    }
}

impl BaseTypeSchemaBuilder for TFloat<F64Ext> {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        build_schema(builder, false)
    }
}

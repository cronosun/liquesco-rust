use crate::common::error::LqError;
use crate::common::internal_utils::try_from_int_result;
use crate::schema::core::DeSerializationContext;
use crate::schema::core::Schema;
use crate::schema::core::Validator;
use crate::schema::validators::Validators;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use crate::serialization::tenum::EnumHeader;
use crate::serialization::tbool::Bool;
use std::convert::TryFrom;

const TRUE_FALSE: u32 = 0;
const TRUE_ONLY: u32 = 1;
const FALSE_ONLY: u32 = 2;

pub enum BoolValues {
    /// "usual" boolean: can be true or false.
    TrueFalse,
    /// Boolean restricted to true only (one single value).
    TrueOnly,
    /// Boolean restricted to false only (one single value).
    FalseOnly,
}

pub struct VBool {
    values: BoolValues,
}

impl VBool {
    pub fn new(values: BoolValues) -> Self {
        Self { values }
    }
}

impl Default for VBool {
    fn default() -> Self {
        Self {
            values: BoolValues::TrueFalse,
        }
    }
}

impl<'a> From<VBool> for Validators<'a> {
    fn from(value: VBool) -> Self {
        Validators::Bool(value)
    }
}

impl<'a> Validator<'a> for VBool {
    type DeSerItem = Self;

    fn validate<S, R>(&self, _: &S, reader: &mut R) -> Result<(), LqError>
    where
        S: Schema<'a>,
        R: BinaryReader<'a>,
    {
        let bool_value = Bool::de_serialize(reader)?;
        match self.values {
            BoolValues::TrueFalse => (),
            BoolValues::TrueOnly => {
                if !bool_value {
                    return LqError::err_static(
                        "Boolean must be true (according to schema); got false.",
                    );
                }
            }
            BoolValues::FalseOnly => {
                if bool_value {
                    return LqError::err_static(
                        "Boolean must be false (according to schema); got true.",
                    );
                }
            }
        };

        Result::Ok(())
    }

    fn de_serialize<TContext>(context: &mut TContext) -> Result<Self::DeSerItem, LqError>
    where
        TContext: DeSerializationContext<'a>,
    {
        let enum_header = EnumHeader::de_serialize(context.reader())?;
        let fields_to_skip = enum_header.number_of_values();
        context
            .reader()
            .skip_n_values(try_from_int_result(usize::try_from(fields_to_skip))?)?;
        let ordinal = enum_header.ordinal();
        let values = match ordinal {
            TRUE_ONLY => BoolValues::TrueOnly,
            TRUE_FALSE => BoolValues::TrueFalse,
            FALSE_ONLY => BoolValues::FalseOnly,
            _ => return LqError::err_static("Unknown bool values"),
        };
        Result::Ok(Self::DeSerItem::new(values))
    }

    fn serialize<S, W>(&self, _: &S, writer: &mut W) -> Result<(), LqError>
    where
        S: Schema<'a>,
        W: BinaryWriter,
    {
        let ordinal = match self.values {
            BoolValues::TrueFalse => TRUE_FALSE,
            BoolValues::FalseOnly => FALSE_ONLY,
            BoolValues::TrueOnly => TRUE_ONLY,
        };

        let header = EnumHeader::new(ordinal, 0);
        EnumHeader::serialize(writer, &header)
    }
}

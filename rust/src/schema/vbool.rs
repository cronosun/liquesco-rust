use crate::schema::core::Context;
use crate::common::error::LqError;
use crate::schema::core::Validator;
use crate::schema::validators::Validators;
use crate::serialization::core::DeSerializer;
use crate::serialization::tbool::Bool;

#[derive(Clone)]
pub enum BoolValues {
    /// "usual" boolean: can be true or false.
    TrueFalse,
    /// Boolean restricted to true only (one single value).
    TrueOnly,
    /// Boolean restricted to false only (one single value).
    FalseOnly,
}

#[derive(Clone)]
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

    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>{
        let bool_value = Bool::de_serialize(context.reader())?;
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
}

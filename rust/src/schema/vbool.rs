use crate::schema::core::Context;
use crate::common::error::LqError;
use crate::schema::core::Validator;
use crate::schema::validators::AnyValidator;
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

#[derive(new, Clone)]
pub struct VBool {
    pub values: BoolValues,
}

impl Default for VBool {
    fn default() -> Self {
        Self {
            values: BoolValues::TrueFalse,
        }
    }
}

impl<'a> From<VBool> for AnyValidator<'static> {
    fn from(value: VBool) -> Self {
        AnyValidator::Bool(value)
    }
}

impl Validator<'static> for VBool {

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

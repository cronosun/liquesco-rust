use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::serialization::core::DeSerializer;
use crate::serialization::boolean::Bool;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum BoolValues {
    /// "usual" boolean: can be true or false.
    TrueFalse,
    /// Boolean restricted to true only (one single value).
    TrueOnly,
    /// Boolean restricted to false only (one single value).
    FalseOnly,
}

#[derive(new, Clone, Debug)]
pub struct TBool {
    pub values: BoolValues, // TODO: Better use Option<Restriction> here (because in 99% of the cases this will be empty) - or better remove altogether
}

impl Default for TBool {
    fn default() -> Self {
        Self {
            values: BoolValues::TrueFalse,
        }
    }
}

impl Type<'static> for TBool {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
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

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        let bool1 = Bool::de_serialize(r1)?;
        let bool2 = Bool::de_serialize(r2)?;
        Result::Ok(bool1.cmp(&bool2))
    }
}

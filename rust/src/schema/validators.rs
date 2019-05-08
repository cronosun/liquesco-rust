use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::schema::vascii::VAscii;
use crate::schema::vbool::VBool;
use crate::schema::vsint::VSInt;
use crate::schema::vstruct::VStruct;
use crate::schema::vuint::VUInt;

#[derive(Clone)]
pub enum AnyValidator<'a> {
    Struct(VStruct<'a>),
    UInt(VUInt),
    SInt(VSInt),
    Ascii(VAscii),
    Bool(VBool),
}

impl<'a> Validator<'a> for AnyValidator<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
        where
            C: Context<'c>,
    {
        match self {
            AnyValidator::Struct(value) => value.validate(context),
            AnyValidator::UInt(value) => value.validate(context),
            AnyValidator::SInt(value) => value.validate(context),
            AnyValidator::Ascii(value) => value.validate(context),
            AnyValidator::Bool(value) => value.validate(context),
        }
    }
}

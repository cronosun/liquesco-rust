use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::schema::vascii::VAscii;
use crate::schema::vbool::VBool;
use crate::schema::vsint::VSInt;
use crate::schema::vstruct::VStruct;
use crate::schema::vuint::VUInt;

// TODO: Rename to AnyValidator?
#[derive(Clone)]
pub enum Validators<'a> {
    Struct(VStruct<'a>),
    UInt(VUInt),
    SInt(VSInt),
    Ascii(VAscii),
    Bool(VBool),
}

impl<'a> Validators<'a> {
   
    // TODO: Actually it's exactly the same inteface as 'Validator'...
    pub fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        match self {
            Validators::Struct(value) => value.validate(context),
            Validators::UInt(value) => value.validate(context),
            Validators::SInt(value) => value.validate(context),
            Validators::Ascii(value) => value.validate(context),
            Validators::Bool(value) => value.validate(context),
        }
    }
}

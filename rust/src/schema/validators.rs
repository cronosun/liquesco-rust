use std::cmp::Ordering;
use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::schema::vanchors::VAnchors;
use crate::schema::vascii::VAscii;
use crate::schema::vbool::VBool;
use crate::schema::venum::VEnum;
use crate::schema::vfloat::VFloat32;
use crate::schema::vfloat::VFloat64;
use crate::schema::voption::VOption;
use crate::schema::vreference::VReference;
use crate::schema::vseq::VSeq;
use crate::schema::vsint::VSInt;
use crate::schema::vstruct::VStruct;
use crate::schema::vuint::VUInt;

#[derive(Clone, FromVariants)]
pub enum AnyValidator<'a> {
    Struct(VStruct<'a>),
    UInt(VUInt),
    SInt(VSInt),
    Ascii(VAscii),
    Bool(VBool),
    Enum(VEnum<'a>),
    Anchors(VAnchors),
    Reference(VReference),
    Seq(VSeq),
    Float32(VFloat32),
    Float64(VFloat64),
    Option(VOption),
}

impl<'a> Validator<'a> for AnyValidator<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // is there no macro for this?
        match self {
            AnyValidator::Struct(value) => value.validate(context),
            AnyValidator::UInt(value) => value.validate(context),
            AnyValidator::SInt(value) => value.validate(context),
            AnyValidator::Ascii(value) => value.validate(context),
            AnyValidator::Bool(value) => value.validate(context),
            AnyValidator::Enum(value) => value.validate(context),
            AnyValidator::Anchors(value) => value.validate(context),
            AnyValidator::Reference(value) => value.validate(context),
            AnyValidator::Seq(value) => value.validate(context),
            AnyValidator::Float32(value) => value.validate(context),
            AnyValidator::Float64(value) => value.validate(context),
            AnyValidator::Option(value) => value.validate(context),
        }
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        // is there no macro for this?
        match self {
            AnyValidator::Struct(value) => value.compare(context, r1, r2),
            AnyValidator::UInt(value) => value.compare(context, r1, r2),
            AnyValidator::SInt(value) => value.compare(context, r1, r2),
            AnyValidator::Ascii(value) => value.compare(context, r1, r2),
            AnyValidator::Bool(value) => value.compare(context, r1, r2),
            AnyValidator::Enum(value) => value.compare(context, r1, r2),
            AnyValidator::Anchors(value) => value.compare(context, r1, r2),
            AnyValidator::Reference(value) => value.compare(context, r1, r2),
            AnyValidator::Seq(value) => value.compare(context, r1, r2),
            AnyValidator::Float32(value) => value.compare(context, r1, r2),
            AnyValidator::Float64(value) => value.compare(context, r1, r2),
            AnyValidator::Option(value) => value.compare(context, r1, r2),
        }
    }
}

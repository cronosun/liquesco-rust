use std::cmp::Ordering;
use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Type;
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

/// This is an enumeration of all `Type`s that are known to the system.
#[derive(Clone, FromVariants)]
pub enum AnyType<'a> {
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

impl<'a> Type<'a> for AnyType<'a> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // is there no macro for this?
        match self {
            AnyType::Struct(value) => value.validate(context),
            AnyType::UInt(value) => value.validate(context),
            AnyType::SInt(value) => value.validate(context),
            AnyType::Ascii(value) => value.validate(context),
            AnyType::Bool(value) => value.validate(context),
            AnyType::Enum(value) => value.validate(context),
            AnyType::Anchors(value) => value.validate(context),
            AnyType::Reference(value) => value.validate(context),
            AnyType::Seq(value) => value.validate(context),
            AnyType::Float32(value) => value.validate(context),
            AnyType::Float64(value) => value.validate(context),
            AnyType::Option(value) => value.validate(context),
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
            AnyType::Struct(value) => value.compare(context, r1, r2),
            AnyType::UInt(value) => value.compare(context, r1, r2),
            AnyType::SInt(value) => value.compare(context, r1, r2),
            AnyType::Ascii(value) => value.compare(context, r1, r2),
            AnyType::Bool(value) => value.compare(context, r1, r2),
            AnyType::Enum(value) => value.compare(context, r1, r2),
            AnyType::Anchors(value) => value.compare(context, r1, r2),
            AnyType::Reference(value) => value.compare(context, r1, r2),
            AnyType::Seq(value) => value.compare(context, r1, r2),
            AnyType::Float32(value) => value.compare(context, r1, r2),
            AnyType::Float64(value) => value.compare(context, r1, r2),
            AnyType::Option(value) => value.compare(context, r1, r2),
        }
    }
}

use crate::common::error::LqError;
use crate::schema::anchors::TAnchors;
use crate::schema::ascii::TAscii;
use crate::schema::boolean::TBool;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::schema::doc_type::DocType;
use crate::schema::enumeration::TEnum;
use crate::schema::float::TFloat32;
use crate::schema::float::TFloat64;
use crate::schema::option::TOption;
use crate::schema::reference::TReference;
use crate::schema::seq::TSeq;
use crate::schema::sint::TSInt;
use crate::schema::structure::TStruct;
use crate::schema::uint::TUInt;
use crate::schema::unicode::TUnicode;
use std::cmp::Ordering;

/// This is an enumeration of all `Type`s that are known to the system.
#[derive(Clone, FromVariants, Debug)]
pub enum AnyType<'a> {
    Struct(DocType<'a, TStruct<'a>>),
    UInt(DocType<'a, TUInt>),
    SInt(DocType<'a, TSInt>),
    Ascii(DocType<'a, TAscii>),
    Bool(DocType<'a, TBool>),
    Enum(DocType<'a, TEnum<'a>>),
    Anchors(DocType<'a, TAnchors>),
    Reference(DocType<'a, TReference>),
    Seq(DocType<'a, TSeq>),
    Float32(DocType<'a, TFloat32>),
    Float64(DocType<'a, TFloat64>),
    Option(DocType<'a, TOption>),
    Unicode(DocType<'a, TUnicode>),
}

impl<'a> Type for AnyType<'a> {
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
            AnyType::Unicode(value) => value.validate(context),
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
            AnyType::Unicode(value) => value.compare(context, r1, r2),
        }
    }
}

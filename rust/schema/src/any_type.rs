use liquesco_common::error::LqError;
use crate::anchors::TAnchors;
use crate::ascii::TAscii;
use crate::boolean::TBool;
use crate::core::Type;
use crate::core::{Context, TypeRef};
use crate::doc_type::DocType;
use crate::enumeration::TEnum;
use crate::enumeration::Variant;
use crate::float::TFloat32;
use crate::float::TFloat64;
use crate::identifier::Identifier;
use crate::option::TOption;
use crate::reference::TReference;
use crate::schema_builder::BaseTypeSchemaBuilder;
use crate::schema_builder::{BuildsOwnSchema, SchemaBuilder};
use crate::seq::TSeq;
use crate::sint::TSInt;
use crate::structure::TStruct;
use crate::uint::TUInt;
use crate::unicode::TUnicode;
use crate::uuid::TUuid;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;
use from_variants::FromVariants;

/// This is an enumeration of all `Type`s that are known to the system.
///
/// Note: Sorted according to serialization major type.
#[derive(Clone, FromVariants, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub enum AnyType<'a> {
    Bool(DocType<'a, TBool>),
    Option(DocType<'a, TOption>),
    Seq(DocType<'a, TSeq>),
    // TODO: Binary
    Unicode(DocType<'a, TUnicode>),
    UInt(DocType<'a, TUInt>),
    SInt(DocType<'a, TSInt>),
    Float32(DocType<'a, TFloat32>),
    Float64(DocType<'a, TFloat64>),
    Enum(DocType<'a, TEnum<'a>>),

    Struct(DocType<'a, TStruct<'a>>),
    Ascii(DocType<'a, TAscii>),
    Anchors(DocType<'a, TAnchors>),
    Reference(DocType<'a, TReference>),
    Uuid(DocType<'a, TUuid>),
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
            AnyType::Uuid(value) => value.validate(context),
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
            AnyType::Uuid(value) => value.compare(context, r1, r2),
        }
    }
}

impl BuildsOwnSchema for AnyType<'_> {
    fn build_schema<B>(builder: &mut B) -> TypeRef
    where
        B: SchemaBuilder,
    {
        let ref_bool = doc_type_ref::<TBool, B>(builder);
        let ref_option = doc_type_ref::<TOption, B>(builder);
        let ref_seq = doc_type_ref::<TSeq, B>(builder);
        let ref_unicode = doc_type_ref::<TUnicode, B>(builder);
        let ref_uint = doc_type_ref::<TUInt, B>(builder);
        let ref_sint = doc_type_ref::<TSInt, B>(builder);
        let ref_float_32 = doc_type_ref::<TFloat32, B>(builder);
        let ref_float_64 = doc_type_ref::<TFloat64, B>(builder);
        let ref_enum = doc_type_ref::<TEnum, B>(builder);
        let ref_struct = doc_type_ref::<TStruct, B>(builder);
        let ref_ascii = doc_type_ref::<TAscii, B>(builder);
        let ref_anchors = doc_type_ref::<TAnchors, B>(builder);
        let ref_reference = doc_type_ref::<TReference, B>(builder);
        let ref_uuid = doc_type_ref::<TUuid, B>(builder);

        builder.add(DocType::from(
            TEnum::default()
                .add(variant(ref_bool, "bool"))
                .add(variant(ref_option, "option"))
                .add(variant(ref_seq, "seq"))
                .add(variant(ref_unicode, "unicode"))
                .add(variant(ref_uint, "uint"))
                .add(variant(ref_sint, "sint"))
                .add(variant(ref_float_32, "f32"))
                .add(variant(ref_float_64, "f64"))
                .add(variant(ref_enum, "enum"))
                .add(variant(ref_struct, "struct"))
                .add(variant(ref_ascii, "ascii"))
                .add(variant(ref_anchors, "anchors"))
                .add(variant(ref_reference, "reference"))
                .add(variant(ref_uuid, "uuid")),
        ))
    }
}

fn variant(reference: TypeRef, id: &'static str) -> Variant<'static> {
    Variant::new(Identifier::try_from(id).unwrap()).add_value(reference)
}

fn doc_type_ref<T, B>(builder: &mut B) -> TypeRef
where
    T: BaseTypeSchemaBuilder + Type,
    B: SchemaBuilder,
{
    let doc_type = DocType::<T>::build_schema(builder);
    builder.add(doc_type)
}

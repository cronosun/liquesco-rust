use crate::anchors::TAnchors;
use crate::ascii::TAscii;
use crate::boolean::TBool;
use crate::core::Doc;
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
use crate::range::TRange;
use crate::uint::TUInt;
use crate::unicode::TUnicode;
use crate::uuid::TUuid;
use from_variants::FromVariants;
use liquesco_common::error::LqError;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::convert::TryFrom;

/// This is an enumeration of all `Type`s that are known to the system.
///
/// Note: Sorted according to serialization major type.
#[derive(Clone, FromVariants, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    Range(DocType<'a, TRange>),
}

impl<'a> AnyType<'a> {
    pub fn doc(&'a self) -> &Doc<'a> {
        match self {
            AnyType::Struct(value) => value.doc(),
            AnyType::UInt(value) => value.doc(),
            AnyType::SInt(value) => value.doc(),
            AnyType::Ascii(value) => value.doc(),
            AnyType::Bool(value) => value.doc(),
            AnyType::Enum(value) => value.doc(),
            AnyType::Anchors(value) => value.doc(),
            AnyType::Reference(value) => value.doc(),
            AnyType::Seq(value) => value.doc(),
            AnyType::Float32(value) => value.doc(),
            AnyType::Float64(value) => value.doc(),
            AnyType::Option(value) => value.doc(),
            AnyType::Unicode(value) => value.doc(),
            AnyType::Uuid(value) => value.doc(),
            AnyType::Range(value) => value.doc(),
        }
    }
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
            AnyType::Range(value) => value.validate(context),
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
            AnyType::Range(value) => value.compare(context, r1, r2),
        }
    }

     fn reference(&self, index : usize) -> Option<TypeRef> {
         match self {
            AnyType::Struct(value) => value.reference(index),
            AnyType::UInt(value) => value.reference(index),
            AnyType::SInt(value) => value.reference(index),
            AnyType::Ascii(value) => value.reference(index),
            AnyType::Bool(value) => value.reference(index),
            AnyType::Enum(value) => value.reference(index),
            AnyType::Anchors(value) => value.reference(index),
            AnyType::Reference(value) => value.reference(index),
            AnyType::Seq(value) => value.reference(index),
            AnyType::Float32(value) => value.reference(index),
            AnyType::Float64(value) => value.reference(index),
            AnyType::Option(value) => value.reference(index),
            AnyType::Unicode(value) => value.reference(index),
            AnyType::Uuid(value) => value.reference(index),
             AnyType::Range(value) => value.reference(index),
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
        let ref_range = doc_type_ref::<TRange, B>(builder);

        builder.add(
            DocType::from(
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
                    .add(variant(ref_uuid, "uuid"))
                    .add(variant(ref_range, "range")),
            )
            .with_name_unwrap("any_type")
            .with_description(
                "The any type is an enumeration of all possible types available \
                 in the type system.",
            ),
        )
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

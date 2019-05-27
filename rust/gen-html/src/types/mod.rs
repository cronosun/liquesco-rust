use crate::body_writer::BodyWriter;
use crate::body_writer::Context;
use crate::types::wanchor::WAnchors;
use crate::types::wanchor::WReference;
use crate::types::wascii::WAscii;
use crate::types::wbool::WBool;
use crate::types::wenum::WEnum;
use crate::types::wfloat::WFloat32;
use crate::types::wfloat::WFloat64;
use crate::types::wint::WSInt;
use crate::types::wint::WUInt;
use crate::types::woption::WOption;
use crate::types::wrange::WRange;
use crate::types::wseq::WSeq;
use crate::types::wstruct::WStruct;
use crate::types::wunicode::WUnicode;
use crate::types::wuuid::WUuid;
use crate::usage::Usage;
use liquesco_processing::names::Names;

use liquesco_processing::schema::SchemaReader;
use liquesco_schema::any_type::AnyType;
use liquesco_schema::core::TypeRef;
use minidom::Element;

pub mod wanchor;
pub mod wascii;
pub mod wbool;
pub mod wenum;
pub mod wfloat;
pub mod wint;
pub mod woption;
pub mod wrange;
pub mod wseq;
pub mod wstruct;
pub mod wunicode;
pub mod wuuid;

pub fn write_body(ctx: BodyWriteContext) -> Element {
    let any_type = ctx.schema.require(ctx.type_ref);

    match any_type {
        AnyType::Struct(value) => WStruct::write(&mut ctx.into(value)),
        AnyType::Seq(value) => WSeq::write(&mut ctx.into(value)),
        AnyType::Enum(value) => WEnum::write(&mut ctx.into(value)),
        AnyType::Anchors(value) => WAnchors::write(&mut ctx.into(value)),
        AnyType::Reference(value) => WReference::write(&mut ctx.into(value)),
        AnyType::Ascii(value) => WAscii::write(&mut ctx.into(value)),
        AnyType::Bool(value) => WBool::write(&mut ctx.into(value)),
        AnyType::Float32(value) => WFloat32::write(&mut ctx.into(value)),
        AnyType::Float64(value) => WFloat64::write(&mut ctx.into(value)),
        AnyType::UInt(value) => WUInt::write(&mut ctx.into(value)),
        AnyType::SInt(value) => WSInt::write(&mut ctx.into(value)),
        AnyType::Option(value) => WOption::write(&mut ctx.into(value)),
        AnyType::Unicode(value) => WUnicode::write(&mut ctx.into(value)),
        AnyType::Uuid(value) => WUuid::write(&mut ctx.into(value)),
        AnyType::Range(value) => WRange::write(&mut ctx.into(value)),
    }
}

pub struct BodyWriteContext<'a> {
    pub schema: &'a SchemaReader,
    pub type_ref: TypeRef,
    pub names: &'a mut Names,
    pub usage: &'a mut Usage,
}

impl<'a> BodyWriteContext<'a> {
    fn into<T>(self, r#type: &'a T) -> Context<T> {
        Context {
            schema: self.schema,
            r#type,
            type_ref: self.type_ref,
            names: self.names,
        }
    }
}

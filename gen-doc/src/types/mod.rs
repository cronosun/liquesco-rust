use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use crate::types::wascii::WAscii;
use crate::types::wbinary::WBinary;
use crate::types::wbool::WBool;
use crate::types::wdecimal::WDecimal;
use crate::types::wenum::WEnum;
use crate::types::wfloat::{WFloat32, WFloat64};
use crate::types::wint::{WSInt, WUInt};
use crate::types::wkey_ref::WKeyRef;
use crate::types::wmap::WMap;
use crate::types::woption::WOption;
use crate::types::wrange::WRange;
use crate::types::wroot_map::WRootMap;
use crate::types::wseq::WSeq;
use crate::types::wstruct::WStruct;
use crate::types::wunicode::WUnicode;
use crate::types::wuuid::WUuid;
use liquesco_common::error::LqError;
use liquesco_schema::any_type::AnyType;

pub mod wascii;
pub mod wbinary;
pub mod wbool;
pub mod wdecimal;
pub mod wenum;
pub mod wfloat;
pub mod wint;
pub mod wkey_ref;
pub mod wmap;
pub mod woption;
pub mod wrange;
pub mod wroot_map;
pub mod wseq;
pub mod wstruct;
pub mod wunicode;
pub mod wuuid;

pub(crate) mod common;

pub fn write_type_body<'a, TContext>(ctx: &TContext) -> Result<Vec<Row<'static>>, LqError>
where
    TContext: ContextProvider<'a>,
{
    match ctx.type_info().any_type() {
        AnyType::Option(value) => WOption::write(ctx, value),
        AnyType::Range(value) => WRange::write(ctx, value),
        AnyType::RootMap(value) => WRootMap::write(ctx, value),
        AnyType::Enum(value) => WEnum::write(ctx, value),
        AnyType::Struct(value) => WStruct::write(ctx, value),
        AnyType::Seq(value) => WSeq::write(ctx, value),
        AnyType::Binary(value) => WBinary::write(ctx, value),
        AnyType::Ascii(value) => WAscii::write(ctx, value),
        AnyType::Bool(value) => WBool::write(ctx, value),
        AnyType::Float32(value) => WFloat32::write(ctx, value),
        AnyType::Float64(value) => WFloat64::write(ctx, value),
        AnyType::UInt(value) => WUInt::write(ctx, value),
        AnyType::SInt(value) => WSInt::write(ctx, value),
        AnyType::Unicode(value) => WUnicode::write(ctx, value),
        AnyType::Uuid(value) => WUuid::write(ctx, value),
        AnyType::Map(value) => WMap::write(ctx, value),
        AnyType::KeyRef(value) => WKeyRef::write(ctx, value),
        AnyType::Decimal(value) => WDecimal::write(ctx, value),
    }
}

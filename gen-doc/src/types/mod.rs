use crate::context::ContextProvider;
use crate::model::row::Row;
use liquesco_common::error::LqError;
use liquesco_schema::any_type::AnyType;
use crate::types::woption::WOption;
use crate::types::wrange::WRange;
use crate::type_writer::TypeBodyWriter;
use crate::types::wroot_map::WRootMap;

pub mod woption;
pub mod wrange;
pub mod wroot_map;

pub fn write_type_body<'a, TContext>(ctx: &TContext) -> Result<Vec<Row<'static>>, LqError>
    where TContext : ContextProvider<'a>{

    match ctx.type_info().any_type() {
        AnyType::Option(value) => WOption::write(ctx, value),
        AnyType::Range(value) => WRange::write(ctx, value),
        AnyType::RootMap(value) => WRootMap::write(ctx, value),
        _ => {
            Ok(vec![
                Row::text("Not yet implemented")
            ])
        },

        /*
        AnyType::Struct(value) => WStruct::write(ctx, value),
        AnyType::Seq(value) => WSeq::write(ctx, value),
        AnyType::Binary(value) => WBinary::write(ctx, value),
        AnyType::Enum(value) => WEnum::write(ctx, value),
        AnyType::Ascii(value) => WAscii::write(ctx, value),
        AnyType::Bool(value) => WBool::write(ctx, value),
        AnyType::Float32(value) => WFloat32::write(ctx, value),
        AnyType::Float64(value) => WFloat64::write(ctx, value),
        AnyType::UInt(value) => WUInt::write(ctx, value),
        AnyType::SInt(value) => WSInt::write(ctx, value),
        AnyType::Option(value) => WOption::write(ctx, value),
        AnyType::Unicode(value) => WUnicode::write(ctx, value),
        AnyType::Uuid(value) => WUuid::write(ctx, value),
        AnyType::Range(value) => WRange::write(ctx, value),
        AnyType::Map(value) => WMap::write(ctx, value),
        AnyType::RootMap(value) => WRootMap::write(ctx, value),
        AnyType::KeyRef(value) => WKeyRef::write(ctx, value),
        AnyType::Decimal(value) => WDecimal::write(ctx, value),*/
    }
}
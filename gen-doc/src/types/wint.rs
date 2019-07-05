use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use crate::types::common::Common;
use liquesco_common::error::LqError;
use liquesco_schema::types::sint::TSInt;
use liquesco_schema::types::tint::TInt;
use liquesco_schema::types::uint::TUInt;
use std::marker::PhantomData;

pub struct WUInt<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WUInt<'a> {
    type T = TUInt<'a>;

    fn write<'b, TContext>(_: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        let range = typ.range();
        Ok(vec![
            Row::association_with_text("Min value (inclusive)", Common::fmt_u128(range.start())),
            Row::association_with_text("Max value (inclusive)", Common::fmt_u128(range.end())),
            Row::association_with_text(
                "Memory width",
                format!("{} bits", typ.memory().number_of_bits()),
            ),
        ])
    }
}

pub struct WSInt<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WSInt<'a> {
    type T = TSInt<'a>;

    fn write<'b, TContext>(_: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        let range = typ.range();
        Ok(vec![
            Row::association_with_text("Min value (inclusive)", Common::fmt_i128(range.start())),
            Row::association_with_text("Max value (inclusive)", Common::fmt_i128(range.end())),
            Row::association_with_text(
                "Memory width",
                format!("{} bits", typ.memory().number_of_bits()),
            ),
        ])
    }
}

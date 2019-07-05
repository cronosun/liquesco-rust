use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use crate::types::common::Common;
use liquesco_common::error::LqError;
use liquesco_schema::types::binary::TBinary;
use std::marker::PhantomData;

pub struct WBinary<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WBinary<'a> {
    type T = TBinary<'a>;

    fn write<'b, TContext>(_: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        Ok(vec![
            Row::association_with_text(
                "Minimum length (inclusive)",
                Common::fmt_u64(*typ.length().start()),
            ),
            Row::association_with_text(
                "Maximum length (inclusive)",
                Common::fmt_u64(*typ.length().end()),
            ),
        ])
    }
}

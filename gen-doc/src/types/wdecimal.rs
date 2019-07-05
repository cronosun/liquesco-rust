use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use crate::types::common::Common;
use liquesco_common::error::LqError;
use liquesco_schema::types::decimal::TDecimal;
use std::marker::PhantomData;

pub struct WDecimal<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WDecimal<'a> {
    type T = TDecimal<'a>;

    fn write<'b, TContext>(_: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        let range = typ.range();
        Ok(vec![
            Row::association_with_text(
                format!("Minimum ({})", included(range.start_included())),
                Common::fmt_decimal(range.start()),
            ),
            Row::association_with_text(
                format!("Maximum ({})", included(range.end_included())),
                Common::fmt_decimal(range.end()),
            ),
        ])
    }
}

fn included(included: bool) -> &'static str {
    if included {
        "inclusive"
    } else {
        "exclusive"
    }
}

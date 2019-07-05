use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use crate::types::common::{Common, TxtSorting};
use liquesco_common::error::LqError;
use liquesco_schema::types::map::Sorting;
use liquesco_schema::types::root_map::TRootMap;
use std::marker::PhantomData;

pub struct WRootMap<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WRootMap<'a> {
    type T = TRootMap<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        Ok(vec![
            ctx.named_link_to_type("Root type", typ.root())?,
            ctx.named_link_to_type("Key type", typ.key())?,
            ctx.named_link_to_type("Value type", typ.value())?,
            Row::association_with_text(
                "Sorting",
                Common::txt_sorting(match typ.sorting() {
                    Sorting::Ascending => TxtSorting::Ascending,
                    Sorting::Descending => TxtSorting::Descending,
                }),
            ),
            Row::association_with_text(
                "Min length (inclusive)",
                Common::fmt_u32(*typ.length().start()),
            ),
            Row::association_with_text(
                "Max length (inclusive)",
                Common::fmt_u32(*typ.length().end()),
            ),
        ])
    }
}

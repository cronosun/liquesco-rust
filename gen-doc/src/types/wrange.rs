use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use crate::types::common::Common;
use liquesco_common::error::LqError;
use liquesco_schema::types::range::{Inclusion, TRange};
use std::marker::PhantomData;

pub struct WRange<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WRange<'a> {
    type T = TRange<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        let inclusion: (&str, &str) = match typ.inclusion() {
            Inclusion::BothInclusive => ("Inclusive", "Inclusive"),
            Inclusion::StartInclusive => ("Inclusive", "Exclusive"),
            Inclusion::BothExclusive => ("Exclusive", "Exclusive"),
            Inclusion::EndInclusive => ("Exclusive", "Inclusive"),
            Inclusion::Supplied => ("Supplied (by data)", "Supplied (by data)"),
        };

        Ok(vec![
            ctx.named_link_to_type("Range element", typ.element())?,
            Row::association_with_text("Start inclusive", inclusion.0),
            Row::association_with_text("End inclusive", inclusion.1),
            Row::association_with_text(
                "Allow empty range",
                Common::fmt_bool_yes_no(typ.allow_empty()),
            ),
        ])
    }
}

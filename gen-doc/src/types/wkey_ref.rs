use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use crate::types::common::Common;
use liquesco_common::error::LqError;
use liquesco_schema::types::key_ref::TKeyRef;
use std::marker::PhantomData;

pub struct WKeyRef<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WKeyRef<'a> {
    type T = TKeyRef<'a>;

    fn write<'b, TContext>(_: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        Ok(vec![Row::association_with_text(
            "Level",
            Common::fmt_u64(u64::from(typ.level())),
        )])
    }
}

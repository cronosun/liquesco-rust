use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use liquesco_common::error::LqError;
use liquesco_schema::types::boolean::TBool;
use std::marker::PhantomData;

pub struct WBool<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WBool<'a> {
    type T = TBool<'a>;

    fn write<'b, TContext>(_: &TContext, _: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        Ok(Vec::new())
    }
}

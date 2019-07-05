use crate::context::ContextProvider;
use crate::model::row::Row;
use crate::type_writer::TypeBodyWriter;
use liquesco_common::error::LqError;
use liquesco_schema::types::option::TOption;
use std::marker::PhantomData;

pub struct WOption<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WOption<'a> {
    type T = TOption<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        Ok(vec![ctx.named_link_to_type("Present type", typ.r#type())?])
    }
}

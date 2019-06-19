use liquesco_schema::core::Type;
use crate::context::{Context, ContextProvider};
use liquesco_common::error::LqError;
use crate::model::row::Row;

pub trait TypeBodyWriter {
    type T: Type + Sized;
    fn write<'a, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'a>;
}

pub trait TypePartWriter {
    fn write<'a, TContext>(ctx: &TContext) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'a>;
}
use crate::context::ContextProvider;
use crate::model::row::{Association, Row};
use crate::type_writer::TypeBodyWriter;
use liquesco_common::error::LqError;
use liquesco_schema::identifier::Format;
use liquesco_schema::types::structure::TStruct;
use std::marker::PhantomData;

pub struct WStruct<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WStruct<'a> {
    type T = TStruct<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'b>,
    {
        let mut rows = Vec::with_capacity(typ.fields().len());

        for field in typ.fields() {
            let link = ctx.link_to_type(field.r#type())?;
            let field_name = field.name();

            let mut association = Association::new(field_name.to_string(Format::SnakeCase));
            association.push_value(link);

            rows.push(Row::Association(association));
        }

        Ok(rows)
    }
}

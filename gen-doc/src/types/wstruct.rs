use crate::context::{Context, ContextProvider};
use crate::context::ContextFunctions;
use liquesco_common::error::LqError;
use liquesco_processing::type_info::TypeInfo;
use liquesco_schema::types::option::TOption;
use minidom::Element;
use std::marker::PhantomData;
use crate::type_writer::TypeBodyWriter;
use crate::model::row::{Row, Link, Association};
use crate::model::row;
use crate::model::card::CardId;
use liquesco_schema::types::root_map::TRootMap;
use liquesco_schema::types::enumeration::{TEnum, Specialization};
use liquesco_schema::identifier::Format;
use crate::types::common::Common;
use liquesco_schema::types::structure::TStruct;

pub struct WStruct<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WStruct<'a> {
    type T = TStruct<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'b> {

        let mut rows = Vec::with_capacity(typ.fields().len());

        for field in typ.fields() {
            let link = ctx.link_to_type(field.r#type())?;
            let field_name = field.name();

            let mut association = Association::new(
                field_name.to_string(Format::SnakeCase));
            association.push_value(link);

            rows.push(Row::Association(association));
        }

        Ok(rows)
    }
}

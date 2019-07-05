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

pub struct WEnum<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WEnum<'a> {
    type T = TEnum<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'b> {

        let mut rows = Vec::with_capacity(typ.variants().len() + 3);

        for  variant in typ.variants() {
            let variant_name = variant.name().to_string(Format::SnakeCase);
            let mut association = Association::new(
                variant_name);
            for value in variant.values() {
                let primitive = ctx.link_to_type(value)?;
                association.push_value(primitive);
            }
            rows.push(Row::association(association));
        }

        rows.push(Row::section("About"));
        let about = typ.about();
        rows.push(Row::association_with_text("Has variants with value(s)",
                                             Common::fmt_bool_yes_no(about.value_variants())));
        rows.push(Row::association_with_text("Specialization",
                                             fmt_specialization(about.specialization())));

        Ok(rows)
    }
}

fn fmt_specialization(specialization : Specialization) -> &'static str {
    match specialization {
        Specialization::None => "None",
        Specialization::Result => "Result enum"
    }
}
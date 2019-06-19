use crate::context::Context;
use crate::context::ContextFunctions;
use crate::context::ContextProvider;
use crate::type_description::type_description;
use liquesco_common::error::LqError;
use liquesco_schema::metadata::WithMetadata;
use minidom::Element;
use crate::type_writer::TypePartWriter;
use crate::model::row::Row;
use liquesco_processing::type_info::TypeInfo;
use crate::model::card::CardId;

pub struct TypeHeader;

impl TypePartWriter for TypeHeader {
    fn write<'a, TContext>(ctx: &TContext) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'a> {

        let mut result = Vec::with_capacity(3);

        // type description
        let (_, type_name, type_description) =
            type_description(ctx.type_info().any_type());
        result.push(Row::note(format!("{}: {}", type_name, type_description)));

        // maybe documentation
        if let Some(description) = ctx.type_info().any_type().meta().doc() {
            result.push(Row::text(description.to_string()));
        }

        // TODO: Conformance

        Ok(result)
    }
}

pub struct TypeFooter;

impl TypePartWriter for TypeFooter {
    fn write<'a, TContext>(ctx: &TContext) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'a> {
        let used_by = ctx.usage().is_used_by(&ctx.type_info().reference());
        if !used_by.is_empty() {
            let mut result = Vec::with_capacity(1 + used_by.len());

            result.push(Row::section("This type is used by"));

            for used_by_item in used_by {
                let type_info = TypeInfo::try_from(ctx.schema(), used_by_item)?;
                result.push(Row::text_with_link(
                    ctx.display_name_for(&type_info),
                    CardId::from(used_by_item)
                ));
            }

            Ok(result)
        } else {
            Ok(vec![])
        }
    }
}

use crate::context::ContextProvider;
use crate::model::row::{Association, Primitive, Row};
use crate::type_description::type_description;
use crate::type_writer::TypePartWriter;
use liquesco_common::error::LqError;
use liquesco_processing::type_info::TypeInfo;
use liquesco_schema::metadata::{Information, WithMetadata};
use liquesco_schema::core::TypeRef;

pub struct TypeHeader;

impl TypePartWriter for TypeHeader {
    fn write<'a, TContext>(ctx: &TContext) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'a>,
    {
        let mut result = Vec::with_capacity(3);

        // type description
        let (_, type_name, type_description) = type_description(ctx.type_info().any_type());
        result.push(Row::note(format!("{}: {}", type_name, type_description)));

        // maybe documentation
        if let Some(description) = ctx.type_info().any_type().meta().doc() {
            result.push(Row::text(description.to_string()));
        }

        Ok(result)
    }
}

pub struct TypeFooter;

impl TypePartWriter for TypeFooter {
    fn write<'a, TContext>(ctx: &TContext) -> Result<Vec<Row<'static>>, LqError>
    where
        TContext: ContextProvider<'a>,
    {
        let mut result = Vec::new();
        write_conformance_footer(ctx, &mut result)?;
        write_used_by_footer(ctx, &mut result)?;
        write_hashes_footer(ctx, &mut result)?;

        Ok(result)
    }
}

fn write_used_by_footer<'a, TContext>(ctx: &TContext, result: &mut Vec<Row>) -> Result<(), LqError>
where
    TContext: ContextProvider<'a>,
{
    let used_by = ctx.usage().is_used_by(&ctx.type_info().reference());
    if !used_by.is_empty() {
        result.push(Row::section("This type is used by"));

        for used_by_item in used_by {
            let type_info = TypeInfo::try_from(ctx.schema(), used_by_item)?;
            result.push(Row::text_with_link(
                ctx.display_name_for(&type_info),
                std::convert::TryFrom::<&TypeRef>::try_from(used_by_item)?
            ));
        }

        Ok(())
    } else {
        Ok(())
    }
}

fn write_hashes_footer<'a, TContext>(ctx: &TContext, result: &mut Vec<Row>) -> Result<(), LqError>
where
    TContext: ContextProvider<'a>,
{
    result.push(Row::section("Hashes"));

    let full_hash = ctx
        .schema()
        .type_hash(&ctx.type_info().reference(), Information::Full)?;
    let technical_hash = ctx
        .schema()
        .type_hash(&ctx.type_info().reference(), Information::Technical)?;
    let typ_hash = ctx
        .schema()
        .type_hash(&ctx.type_info().reference(), Information::Type)?;

    result.push(Row::association(
        Association::new("Full hash").with_value(Primitive::code(Into::<String>::into(&full_hash))),
    ));
    result.push(Row::association(
        Association::new("Technical hash")
            .with_value(Primitive::code(Into::<String>::into(&technical_hash))),
    ));
    result.push(Row::association(
        Association::new("Type hash").with_value(Primitive::code(Into::<String>::into(&typ_hash))),
    ));

    Ok(())
}

fn write_conformance_footer<'a, TContext>(
    ctx: &TContext,
    result: &mut Vec<Row>,
) -> Result<(), LqError>
where
    TContext: ContextProvider<'a>,
{
    // maybe conformance
    let implements = ctx.type_info().any_type().meta().implements();
    if !implements.is_empty() {
        result.push(Row::section("Conforms to"));
        for (index, implements) in implements.iter().enumerate() {
            result.push(Row::association_with_text(
                format!("Implements #{}", index + 1),
                implements.as_hex_string(),
            ));
        }
    }
    Ok(())
}

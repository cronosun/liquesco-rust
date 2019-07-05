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
use crate::types::common::{Common, TxtSorting};
use liquesco_schema::types::seq::TSeq;
use liquesco_schema::types::seq;

pub struct WSeq<'a> {
    _phantom: &'a PhantomData<()>,
}

impl<'a> TypeBodyWriter for WSeq<'a> {
    type T = TSeq<'a>;

    fn write<'b, TContext>(ctx: &TContext, typ: &Self::T) -> Result<Vec<Row<'static>>, LqError>
        where TContext : ContextProvider<'b> {

        let mut rows = Vec::new();

        rows.push(ctx.named_link_to_type("Element type", typ.element())?);

        // information about length
        let length = typ.length();
        if length.start() != length.end() {
            rows.push(Row::association_with_text("Length minimum (inclusive)",
                                                 Common::fmt_u32(*length.start())));
            rows.push(Row::association_with_text("Length maximum (inclusive)",
                                                 Common::fmt_u32(*length.end())));
        } else {
            rows.push(Row::association_with_text("Fixed length",
            Common::fmt_u32(*length.start())));
        }

        if let Some(multiple_of) = typ.multiple_of() {
            rows.push(Row::association_with_text("Length multiple of",
                                                 format!("{}", multiple_of)));
        }

        // ordering
        let ordering = typ.ordering();
        let unique = match ordering {
            seq::Ordering::None => {
                rows.push(Row::association_with_text(
                    "Ordering",
                                                     "Undefined; any ordering is allowed."));
                false
            }
            seq::Ordering::Sorted(value) => {
                rows.push(Row::association_with_text(
                    "Ordering",
                    format!("{} sorting is required",
                    Common::txt_sorting(match value.direction {
                        seq::Direction::Ascending => TxtSorting::Ascending,
                        seq::Direction::Descending => TxtSorting::Descending,
                    }))));
                value.unique
            }
        };

        rows.push(Row::association_with_text(
            "Unique",
            if unique { "Yes (only unique elements)"} else { "No (duplicates are allowed)"}));

        Ok(rows)
    }
}

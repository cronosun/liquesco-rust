use crate::parser::converter::Converter;
use crate::parser::core::Context;
use crate::parser::core::Parser;
use crate::parser::value::TextValue;
use liquesco_common::error::LqError;
use liquesco_schema::anchors::TAnchors;
use liquesco_serialization::core::Serializer;
use liquesco_serialization::seq::SeqHeader;
use std::convert::TryFrom;

pub struct PAnchors;

impl<'a> Parser<'a> for PAnchors {
    type T = TAnchors<'a>;

    fn parse<'c, C>(
        context: &mut C,
        writer: &mut C::TWriter,
        value: &TextValue,
        r#type: &Self::T,
    ) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        C::TConverter::require_no_name(value)?;

        // save anchor info (this is required with nested anchors)
        let preserved_anchor_info = context.take_anchor_info();

        SeqHeader::serialize(writer, &SeqHeader::new(2))?;

        // need a map
        let string_map = C::TConverter::require_string_map(value.as_ref())?;
        let result =
            if let Some(master_anchor) = string_map.get(C::TConverter::master_anchor()) {
                context.parse(writer, r#type.master(), master_anchor)?;
                // add master to the reference map
                context
                    .present_anchor_info()
                    .reference(C::TConverter::master_anchor());

                // now write the rest
                let number_of_anchors = u32::try_from(string_map.len() - 1)?;
                SeqHeader::serialize(writer, &SeqHeader::new(2))?;

                // note: index starts at 1 (since index 0 is the master)
                for index in 1..number_of_anchors + 1 {
                    if let Some(anchor_name) = context.present_anchor_info().by_index(index) {
                        if let Some(anchor) = string_map.get(anchor_name) {
                            context.parse(writer, r#type.anchor(), anchor)?;
                        } else {
                            return Err(LqError::new(format!(
                                "Got a reference to anchor named \
                                 `{:?}` - but no such anchor found!",
                                anchor_name
                            )));
                        }
                    } else {
                        return Err(LqError::new(format!("Not all anchors are referenced \
                    (there are {:?} anchors; excluding the master) but only {:?} anchors \
                    are referenced. Remove unused anchors! Currently referenced anchors: {:?}",
                    number_of_anchors,index-1, context.present_anchor_info())));
                    }
                }

                Ok(())
            } else {
                Err(LqError::new(format!(
                    "Master anchor not found. Master anchor must be 
            called `{:?}`. Found {:?}",
                    C::TConverter::master_anchor(),
                    string_map.keys()
                )))
            };

        // restore anchor info
        context.set_anchor_info(preserved_anchor_info);

        result
    }
}

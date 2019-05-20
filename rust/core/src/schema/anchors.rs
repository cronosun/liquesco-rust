use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::schema::core::TypeRef;
use crate::schema::doc_type::DocType;
use crate::schema::identifier::Identifier;
use crate::schema::option::TOption;
use crate::schema::reference::TReference;
use crate::schema::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::schema::seq::seq_compare;
use crate::schema::structure::Field;
use crate::schema::structure::TStruct;
use crate::schema::uint::TUInt;
use crate::serialization::core::DeSerializer;
use crate::serialization::seq::SeqHeader;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// A list containing 1-n anchors. Every anchor (except anchor 0, the master anchor) has to be
/// referenced (see `TReference`). To make sure data is canonical, anchors have to be
/// referenced sequentially.
#[derive(new, Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct TAnchors {
    pub master: TypeRef,
    pub anchor: TypeRef,

    /// The maximum number of anchors allowed (inclusive). This does not include the master. So
    /// a value of 0 means that only a master is allowed but no anchor.
    ///
    /// If missing, there's no limit: u32::MAX is the maximum.
    #[new(value = "Option::None")]
    pub max_anchors: Option<u32>,
}

impl Type for TAnchors {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // this sequence contains the master + a sequence of anchors
        let container_seq = SeqHeader::de_serialize(context.reader())?;
        if container_seq.length() != 2 {
            return LqError::err_new(format!(
                "Anchors have to look like this 'Seq(master, Seq(anchors))' - the container \
                 (outer) sequence has to have a length of exactly 2; this container has a \
                 length of {:?}",
                container_seq.length()
            ));
        }

        // preserve the anchor index (this is required if we have nested anchors)
        let saved_index = context.anchor_index();
        let saved_max_index = context.max_used_anchor_index();

        // index 0 is already "used"
        context.set_max_used_anchor_index(Option::Some(0));
        // first is to validate the master
        context.set_anchor_index(Option::Some(0));
        context.validate(self.master)?;

        // no validate all anchors
        let anchors_seq = SeqHeader::de_serialize(context.reader())?;
        let number_of_anchors = anchors_seq.length();
        if let Some(max_anchors) = self.max_anchors {
            if number_of_anchors > max_anchors {
                return LqError::err_new(format!(
                    "According to the schema {:?} anchors are \
                     allowed at max (not couting the master anchor). You have {:?} anchors \
                     (not counting the master anchor).",
                    max_anchors, number_of_anchors
                ));
            }
        }
        for index in 1..number_of_anchors + 1 {
            // first make sure there's a reference to this or is it unused?
            let max_used_index_opt = context.max_used_anchor_index();
            if let Some(max_used_index) = max_used_index_opt {
                if max_used_index < index {
                    return LqError::err_new(format!(
                        "There's no reference to anchor at index {:?}. Every \
                         anchor has to be referenced (except the master anchor). \
                         Unused anchors are not allowed. The last anchor that \
                         has been referenced is the anchor at index {:?}.",
                        index, max_used_index
                    ));
                }
            } else {
                // this should never happen
                return LqError::err_static("Invalid max used index (None).");
            }

            context.set_anchor_index(Option::Some(index));
            context.validate(self.anchor)?;
        }

        // Make sure there's no overflow
        let max_used_index_opt = context.max_used_anchor_index();
        if let Some(max_used_index) = max_used_index_opt {
            if max_used_index >= number_of_anchors + 1 {
                return LqError::err_new(format!(
                    "There's {:?} anchors (including master); last reference \
                     is {:?} - there's no such anchor at index {:?}.",
                    number_of_anchors + 1,
                    max_used_index,
                    max_used_index
                ));
            }
        }

        // and restore again
        context.set_anchor_index(saved_index);
        context.set_max_used_anchor_index(saved_max_index);

        Result::Ok(())
    }

    fn compare<'c, C>(
        &self,
        context: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<std::cmp::Ordering, LqError>
    where
        C: Context<'c>,
    {
        let container_seq1 = SeqHeader::de_serialize(r1)?;
        let container_seq2 = SeqHeader::de_serialize(r2)?;
        if container_seq1.length() != 2 || container_seq2.length() != 2 {
            return LqError::err_static("Invalid anchors (cannot compare)");
        }

        // compare master
        let master_cmp = context.compare(self.master, r1, r2)?;
        if master_cmp != std::cmp::Ordering::Equal {
            Ok(master_cmp)
        } else {
            // ok, master is identical, now compare anchors
            seq_compare(|_| self.anchor, context, r1, r2)
        }
    }
}

impl BaseTypeSchemaBuilder for TAnchors {
    fn build_schema<B>(builder: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        let field_master = builder.add(DocType::from(TReference));
        let field_anchor = builder.add(DocType::from(TReference));
        let max_anchors = builder.add(DocType::from(
            TUInt::try_new(0, std::u32::MAX as u64).unwrap(),
        ));
        let field_max_anchors = builder.add(DocType::from(TOption::new(max_anchors)));

        DocType::from(
            TStruct::default()
                .add(Field::new(
                    Identifier::try_from("master").unwrap(),
                    field_master,
                ))
                .add(Field::new(
                    Identifier::try_from("anchor").unwrap(),
                    field_anchor,
                ))
                .add(Field::new(
                    Identifier::try_from("max_anchors").unwrap(),
                    field_max_anchors,
                )),
        )
    }
}

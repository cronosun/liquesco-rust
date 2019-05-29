use crate::core::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::identifier::Identifier;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::NameDescription;
use crate::metadata::NameOnly;
use crate::metadata::WithMetadata;
use crate::option::TOption;
use crate::reference::TReference;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::seq::seq_compare;
use crate::structure::Field;
use crate::structure::TStruct;
use crate::uint::TUInt;
use liquesco_common::error::LqError;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::seq::SeqHeader;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// A list containing 1-n anchors. Every anchor (except anchor 0, the master anchor) has to be
/// referenced (see `TReference`). To make sure data is canonical, anchors have to be
/// referenced sequentially.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TAnchors<'a> {
    meta: Meta<'a>,
    master: TypeRef,
    anchor: TypeRef,
    max_anchors: Option<u32>,
}

impl TAnchors<'_> {
    /// The master anchor type.
    pub fn master(&self) -> TypeRef {
        self.master
    }

    /// The type of all other anchors.
    pub fn anchor(&self) -> TypeRef {
        self.anchor
    }

    /// The maximum number of anchors allowed (inclusive). This does not include the master. So
    /// a value of 0 means that only a master is allowed but no anchor.
    ///
    /// If missing, there's no limit: u32::MAX is the maximum.
    pub fn max_anchors(&self) -> Option<u32> {
        self.max_anchors
    }

    pub fn new(master: TypeRef, anchor: TypeRef) -> Self {
        Self {
            meta: Meta::empty(),
            master,
            anchor,
            max_anchors: None,
        }
    }
}

impl Type for TAnchors<'_> {
    // TODO: Vermutlich reihenfoge gerade umdrehen!
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

        // now validate all anchors
        let anchors_seq = SeqHeader::de_serialize(context.reader())?;
        let number_of_anchors = anchors_seq.length();
        if let Some(max_anchors) = self.max_anchors {
            if number_of_anchors > max_anchors {
                return LqError::err_new(format!(
                    "According to the schema {:?} anchors are \
                     allowed at max (not counting the master anchor). You have {:?} anchors \
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
                    if !context.config().weak_reference_validation {
                        return LqError::err_new(format!(
                            "There's no reference to anchor at index {:?}. Every \
                             anchor has to be referenced (except the master anchor). \
                             Unused anchors are not allowed. The last anchor that \
                             has been referenced is the anchor at index {:?}.",
                            index, max_used_index
                        ));
                    }
                }
            } else {
                // this should never happen
                return LqError::err_new("Invalid max used index (None).");
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
            return LqError::err_new("Invalid anchors (cannot compare)");
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

    fn reference(&self, index: usize) -> Option<TypeRef> {
        match index {
            0 => Some(self.master()),
            1 => Some(self.anchor()),
            _ => None,
        }
    }
}

impl<'a> WithMetadata for TAnchors<'a> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TAnchors<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TAnchors<'_> {
    fn build_schema<B>(builder: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        let field_master = builder.add(TReference::default().with_meta(NameDescription {
            name: "anchor_master_type",
            doc: "Anchors have exactly one master (required) and 0-n more \
                  anchors. This defines the master type.",
        }));
        let field_anchor = builder.add(TReference::default().with_meta(NameDescription {
            name: "anchor_type",
            doc: "Defines the type of the anchors. Note: There's also the master \
                  anchor type which can (but usually doesn't) differ from this.",
        }));
        let max_anchors = builder.add(TUInt::try_new(0, std::u32::MAX as u64).unwrap().with_meta(
            NameDescription {
                name: "max_anchors",
                doc: "This is the maximum number of \
                      anchors allowed. This does not include the master anchor (which is mandatory \
                      anyway).",
            },
        ));
        let field_max_anchors = builder.add(TOption::new(max_anchors).with_meta(NameOnly {
            name: "maybe_max_anchors",
        }));

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
                ))
                .with_meta(NameDescription {
                    name : "anchors",
                    doc: "Anchors (in combination with references) can be used to create \
             recursive data type. The anchors is basically a sequence of 1-n anchors. Those \
             anchors can be referenced using the reference type. Multiple anchors can be nested; \
             references reference always the anchor in the next anchor sequence in the \
             hierarchy (upwards)."
                })
    }
}

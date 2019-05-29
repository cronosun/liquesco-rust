use crate::core::Context;
use crate::core::Type;
use crate::core::TypeRef;
use crate::metadata::Meta;
use crate::metadata::MetadataSetter;
use crate::metadata::NameDescription;
use crate::metadata::WithMetadata;
use crate::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::structure::TStruct;
use liquesco_common::error::LqError;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::uint::UInt32;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// A reference can be used in combination with `TAnchors`. This references one anchor in
/// the anchors sequence.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TReference<'a> {
    meta: Meta<'a>,
}

impl<'a> Default for TReference<'a> {
    fn default() -> Self {
        Self {
            meta: Meta::empty(),
        }
    }
}

impl Type for TReference<'_> {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // reference is a uint32.
        let reference = UInt32::de_serialize(context.reader())?;

        // We can only increment max used index by at max one
        let opt_max_used_index = context.max_used_anchor_index();
        let opt_current_index = context.anchor_index();

        if let (Some(current_index), Some(max_used_index)) = (opt_current_index, opt_max_used_index)
        {
            if reference > max_used_index + 1 {
                if !context.config().weak_reference_validation {
                    return LqError::err_new(format!(
                        "The current anchor index is {:?}. The last reference is {:?}. It's only possible to \
                    reference next item - so reference has to be within the range [0 - {:?}] (inclusive). \
                    This usually happens when ordering of anchors is invalid. Anchors have to be \
                    ordered to make sure data is canonical.",
                        current_index,
                        max_used_index,
                        max_used_index + 1,
                    ));
                }
            }
            if reference > max_used_index {
                context.set_max_used_anchor_index(Option::Some(reference));
            }
        } else {
            return LqError::err_new(format!(
                "Found a reference referencing \
                 anchor {:?}. Problem: There's no anchors and or no max used index. References \
                 can only be used \
                 as children of anchors. If you see this message, something with your \
                 schema might be wrong.",
                reference
            ));
        };

        Result::Ok(())
    }

    fn compare<'c, C>(
        &self,
        _: &C,
        r1: &mut C::Reader,
        r2: &mut C::Reader,
    ) -> Result<Ordering, LqError>
    where
        C: Context<'c>,
    {
        let int1 = UInt32::de_serialize(r1)?;
        let int2 = UInt32::de_serialize(r2)?;
        Result::Ok(int1.cmp(&int2))
    }

    fn reference(&self, _: usize) -> Option<TypeRef> {
        None
    }
}

impl WithMetadata for TReference<'_> {
    fn meta(&self) -> &Meta {
        &self.meta
    }
}

impl<'a> MetadataSetter<'a> for TReference<'a> {
    fn set_meta(&mut self, meta: Meta<'a>) {
        self.meta = meta;
    }
}

impl BaseTypeSchemaBuilder for TReference<'_> {
    fn build_schema<B>(_: &mut B) -> TStruct<'static>
    where
        B: SchemaBuilder,
    {
        // just an empty struct (but more fields will be added by the system)
        TStruct::default().with_meta(NameDescription {
            name: "reference",
            doc: "A reference references a value in the anchors list. See \
                  anchors for more details.",
        })
    }
}

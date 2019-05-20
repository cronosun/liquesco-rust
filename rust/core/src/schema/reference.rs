use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::schema::doc_type::DocType;
use crate::schema::schema_builder::{BaseTypeSchemaBuilder, SchemaBuilder};
use crate::schema::structure::TStruct;
use crate::serialization::core::DeSerializer;
use crate::serialization::uint::UInt32;
use std::cmp::Ordering;

/// A reference can be used in combination with `TAnchors`. You can reference
/// one anchor.
#[derive(Clone, Debug)]
pub struct TReference;

impl Type for TReference {
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
}

impl BaseTypeSchemaBuilder for TReference {
    fn build_schema<B>(_: &mut B) -> DocType<'static, TStruct<'static>>
    where
        B: SchemaBuilder,
    {
        // just an empty struct (but more fields will be added by the system)
        DocType::from(TStruct::default())
    }
}

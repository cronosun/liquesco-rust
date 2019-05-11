use crate::common::error::LqError;
use crate::schema::core::Context;
use crate::schema::core::Validator;
use crate::schema::core::ValidatorRef;
use crate::schema::vseq::seq_compare;
use crate::serialization::core::DeSerializer;
use crate::serialization::tseq::SeqHeader;

/// A list containing 1-n anchors. Every anchor (except anchor 0, the master anchor) has to be
/// referenced (see `VReference`). To make sure data is canonical, anchors have to be
/// referenced sequentially.
#[derive(new, Clone)]
pub struct VAnchors {
    pub master_validator: ValidatorRef,
    pub anchor_validator: ValidatorRef,
    #[new(value = "4294967295")]
    pub max_anchors: u32,
}

impl<'a> Validator<'static> for VAnchors {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        // internally anchors are a sequence
        let seq_header = SeqHeader::de_serialize(context.reader())?;
        let number_of_items = seq_header.length();

        // we need at least one item (the master)
        if number_of_items < 1 {
            return LqError::err_static(
                "Anchors are a sequence of values with a one master anchor \
                 (at index 0; required) and optionally 0-n more anchors. \
                 The input list has no values (the required master anchor is missing)",
            );
        }

        let number_of_items_excluding_master = number_of_items - 1;
        // important: master anchor does not count
        if number_of_items_excluding_master > self.max_anchors {
            return LqError::err_new(format!(
                "According to the schema {:?} anchors are \
                 allowed at max (not couting the master anchor). You have {:?} anchors \
                 (not counting the master anchor).",
                self.max_anchors, number_of_items_excluding_master
            ));
        }

        // preserve the anchor index (this is required if we have nested anchors)
        let saved_index = context.anchor_index();
        let saved_max_index = context.max_used_anchor_index();

        // index 0 is already "used"
        context.set_max_used_anchor_index(Option::Some(0));
        // first is to validate the master
        context.set_anchor_index(Option::Some(0));
        context.validate(self.master_validator)?;

        // now validate all other anchors
        for index in 1..number_of_items {
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
            context.validate(self.anchor_validator)?;
        }

        // Make sure there's no overflow
        let max_used_index_opt = context.max_used_anchor_index();
        if let Some(max_used_index) = max_used_index_opt {
            if max_used_index >= number_of_items {
                return LqError::err_new(format!(
                    "There's {:?} anchors (including master); last reference \
                     is {:?} - there's no such anchor at index {:?}.",
                    number_of_items, max_used_index, max_used_index
                ));
            }
        }

        // restore anchor index
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
        // Compare is identical to the one in "seq" (except that the first element can have a different type)
        seq_compare(
            |index| {
                if index == 0 {
                    self.master_validator
                } else {
                    self.anchor_validator
                }
            },
            context,
            r1,
            r2,
        )
    }
}

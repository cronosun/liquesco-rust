use crate::common::error::LqError;
use crate::common::ine_range::U32IneRange;
use crate::schema::core::Context;
use crate::schema::core::Type;
use crate::schema::core::TypeRef;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LqReader;
use crate::serialization::seq::SeqHeader;
use crate::common::range::LqRangeBounds;

#[derive(new, Clone)]
pub struct TSeq {
    pub element: TypeRef,
    pub length: U32IneRange,
    pub ordering: Ordering,
}

#[derive(Clone)]
pub enum Ordering {
    None,
    Sorted { direction: Direction, unique: bool },
}

#[derive(Clone, PartialEq)]
pub enum Direction {
    Ascending,
    Descending,
}

impl TSeq {
    pub fn try_new(element: TypeRef, min_len: u32, max_len: u32) -> Result<Self, LqError> {
        Result::Ok(Self {
            element,
            length: U32IneRange::try_new(min_len, max_len)?,
            ordering: Ordering::None,
        })
    }
}

impl Type<'static> for TSeq {
    fn validate<'c, C>(&self, context: &mut C) -> Result<(), LqError>
    where
        C: Context<'c>,
    {
        let seq = SeqHeader::de_serialize(context.reader())?;
        let number_of_items = seq.length();

        self.length.require_within(
            "Sequence length validation \
             (schema; min/max elements in sequence)",
            &number_of_items,
        )?;

        match &self.ordering {
            Ordering::None => {
                // validate each element
                for _ in 0..number_of_items {
                    context.validate(self.element)?;
                }
            }
            Ordering::Sorted { direction, unique } => {
                validate_with_ordering(self, context, direction.clone(), *unique, number_of_items)?;
            }
        }

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
        seq_compare(|_| self.element, context, r1, r2)
    }
}

#[inline]
pub(crate) fn seq_compare<'c, C, FGetType: Fn(u32) -> TypeRef>(
    type_for_index: FGetType,
    context: &C,
    r1: &mut C::Reader,
    r2: &mut C::Reader,
) -> Result<std::cmp::Ordering, LqError>
where
    C: Context<'c>,
{
    let header1 = SeqHeader::de_serialize(r1)?;
    let header2 = SeqHeader::de_serialize(r2)?;

    // it's very imporant that we finish reading to the end (see contract)
    let finish_reading =
        |header: SeqHeader, reader: &mut LqReader, num_read: u32| -> Result<(), LqError> {
            let len = header.length();
            if len > num_read {
                let missing = len - num_read;
                reader.skip_n_values_u32(missing)
            } else {
                Result::Ok(())
            }
        };

    // lex compare: first compare each element (even if their length is not equal)
    let min_to_read = header1.length().min(header2.length());
    for index in 0..min_to_read {
        let r#type = type_for_index(index);
        let cmp = context.compare(r#type, r1, r2)?;
        if cmp != std::cmp::Ordering::Equal {
            // no need to finish to the end (see contract)
            return Result::Ok(cmp);
        }
    }

    // if their length are equal, we've read everything
    let result = header1.length().cmp(&header2.length());

    // here we have to finish to the end (if the result is `Equal`)
    if result == std::cmp::Ordering::Equal {
        finish_reading(header1, r1, min_to_read)?;
        finish_reading(header2, r2, min_to_read)?;
    }

    Result::Ok(result)
}

fn validate_with_ordering<'c, C>(
    this: &TSeq,
    context: &mut C,
    direction: Direction,
    unique: bool,
    number_of_items: u32,
) -> Result<(), LqError>
where
    C: Context<'c>,
{
    // here validation is a bit more complex
    let mut previous: Option<C::Reader> = Option::None;
    for idx in 0..number_of_items {
        // we need 3 readers (one for validation, one for this cmp and one for next cmp)
        let mut saved_reader1 = context.reader().clone();
        let saved_reader2 = context.reader().clone();
        context.validate(this.element)?;

        if let Some(mut previous) = previous.take() {
            let equality = context.compare(this.element, &mut previous, &mut saved_reader1)?;
            match equality {
                std::cmp::Ordering::Greater => {
                    // previous is greater: this is OK for descending lists
                    if direction != Direction::Descending {
                        return LqError::err_new(format!(
                            "Element at index {:?} is greater \
                             than element at index {:?}; this is OK but only for descending \
                             lists. This list is not a descending list.",
                            idx - 1,
                            idx
                        ));
                    }
                }
                std::cmp::Ordering::Less => {
                    // previous is less: this is OK for ascending lists
                    if direction != Direction::Ascending {
                        return LqError::err_new(format!(
                            "Element at index {:?} is less \
                             than element at index {:?}; this is OK but only for ascending \
                             lists. This list is not an ascending list.",
                            idx - 1,
                            idx
                        ));
                    }
                }
                std::cmp::Ordering::Equal => {
                    // this is only allowed when we accept duplicates
                    if unique {
                        return LqError::err_new(format!(
                            "Elements at index {:?} and {:?} in \
                             sequence are equal. This is not allowed, since sequence must not \
                             contain duplicates.",
                            idx - 1,
                            idx
                        ));
                    }
                }
            }
        }
        previous = Option::Some(saved_reader2);
    }

    Result::Ok(())
}

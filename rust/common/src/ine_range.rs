use crate::error::LqError;
use crate::range::LqRangeBounds;
use crate::range::NewFull;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Bound;
use std::ops::Deref;
use std::ops::RangeBounds;

pub type F32IneRange = IneRange<f32>;
pub type F64IneRange = IneRange<f64>;
pub type U64IneRange = IneRange<u64>;
pub type I64IneRange = IneRange<i64>;
pub type U32IneRange = IneRange<u32>;
pub type U8IneRange = IneRange<u8>;

/// Inclusive non-empty range.
///
/// Similar to `std::ops::RangeInclusive` but is never empty; and `min` is always <= `max`.
#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct IneRange<T> {
    start: T,
    end: T,
}

impl<T> IneRange<T> {
    pub fn try_new<Msg: Debug>(msg: Msg, start: T, end: T) -> Result<Self, LqError>
    where
        T: PartialOrd + Debug,
    {
        if start > end {
            LqError::err_new(format!(
                "{:?}: You're trying to construct a range. Those ranges require \
                 max>=main. Got {:?} for start and {:?} for end.",
                msg, start, end
            ))
        } else {
            Ok(IneRange { start, end })
        }
    }

    pub fn start(&self) -> &T {
        &self.start
    }

    pub fn end(&self) -> &T {
        &self.end
    }
}

impl<T> RangeBounds<T> for IneRange<T> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&T> {
        Bound::Included(&self.end)
    }
}

impl<T> LqRangeBounds<T> for IneRange<T> {}

impl NewFull for F32IneRange {
    fn full() -> Self {
        Self::try_new("Full range", std::f32::MIN, std::f32::MAX).unwrap()
    }
}

impl NewFull for F64IneRange {
    fn full() -> Self {
        Self::try_new("Full range", std::f64::MIN, std::f64::MAX).unwrap()
    }
}

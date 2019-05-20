use crate::common::error::LqError;
use crate::common::range::Bounds;
use crate::common::range::LqRangeBounds;
use crate::common::range::NewFull;
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
#[derive(Clone, Hash, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct IneRange<T>(Bounds<T>);

impl<T> RangeBounds<T> for IneRange<T> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.0.start())
    }

    fn end_bound(&self) -> Bound<&T> {
        Bound::Included(&self.0.end())
    }
}

impl<T> LqRangeBounds<T> for IneRange<T> {}

impl<T: PartialOrd + Debug> IneRange<T> {
    #[inline]
    pub fn try_new(start: T, end: T) -> Result<Self, LqError> {
        Result::Ok(Self(Bounds::try_new(start, end)?))
    }

    #[inline]
    pub fn try_new_msg<M: Debug>(msg: M, start: T, end: T) -> Result<Self, LqError> {
        Result::Ok(Self(Bounds::try_new_msg(msg, start, end)?))
    }
}

impl<T> Deref for IneRange<T> {
    type Target = Bounds<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl NewFull for F32IneRange {
    fn full() -> Self {
        Self::try_new(std::f32::MIN, std::f32::MAX).unwrap()
    }
}

impl NewFull for F64IneRange {
    fn full() -> Self {
        Self::try_new(std::f64::MIN, std::f64::MAX).unwrap()
    }
}

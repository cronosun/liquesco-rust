use crate::common::error::LqError;
use std::fmt::Debug;

pub type F32IneRange = IneRange<f32>;
pub type F64IneRange = IneRange<f64>;
pub type U64IneRange = IneRange<u64>;
pub type I64IneRange = IneRange<i64>;
pub type U32IneRange = IneRange<u32>;
pub type U8IneRange = IneRange<u8>;

/// Inclusive non-empty range.
///
/// Similar to `std::ops::RangeInclusive` but is never empty; and `min` is always <= `max`.
#[derive(Clone, Hash, Debug, PartialEq, PartialOrd)]
pub struct IneRange<T> {
    min: T,
    max: T,
}

impl<T: PartialOrd + Debug> IneRange<T> {
    #[inline]
    pub fn try_new(min: T, max: T) -> Result<Self, LqError> {
        if max < min {
            LqError::err_new(format!(
                "You're trying to construct an inclusive non-empty \
                 range. Those ranges require max>=main. Got {:?} for min and {:?} for max.",
                min, max
            ))
        } else {
            Result::Ok(IneRange { min, max })
        }
    }

    #[inline]
    pub fn try_new_msg<M: Debug>(msg: M, min: T, max: T) -> Result<Self, LqError> {
        if max < min {
            LqError::err_new(format!(
                "{:?}: You're trying to construct an inclusive non-empty \
                 range. Those ranges require max>=main. Got {:?} for min and {:?} for max.",
                msg, min, max
            ))
        } else {
            Result::Ok(IneRange { min, max })
        }
    }

    #[inline]
    pub fn contains(&self, item: &T) -> bool {
        item >= &self.min && item <= &self.max
    }

    #[inline]
    pub fn min(&self) -> &T {
        &self.min
    }

    #[inline]
    pub fn max(&self) -> &T {
        &self.max
    }

    #[inline]
    pub fn require_within<M: Debug>(&self, msg: M, item: &T) -> Result<(), LqError> {
        if !self.contains(item) {
            LqError::err_new(format!(
                "{:?}: The given value {:?} is not within given range \
                 {:?} (inclusive) to {:?} (inclusive).",
                msg, item, self.min, self.max
            ))
        } else {
            Result::Ok(())
        }
    }
}

pub trait NewFull {
    fn full() -> Self;
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

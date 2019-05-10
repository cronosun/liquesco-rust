use std::fmt::Debug;
use crate::common::error::LqError;

pub type F32IneRange = IneRange<f32>;
pub type F64IneRange = IneRange<f64>;

/// Inclusive non-empty range.
///
/// Similar to `std::ops::RangeInclusive` but is never empty; and `min` is always <= `max`.
#[derive(Clone, Hash, Debug, PartialEq, PartialOrd)]
pub struct IneRange<T> {
    min : T,
    max : T
}

impl<T : PartialOrd + Debug> IneRange<T> {

    pub fn try_new(min : T, max : T) -> Result<Self, LqError> {
        if max<min {
            LqError::err_new(format!("You're trying to construct an inclusive non-empty \
            range. Those ranges require max>=main. Got {:?} for min and {:?} for max.",
            min, max))
        } else {
            Result::Ok(IneRange {
                min,
                max
            })
        }
    }

    pub fn contains(&self, item : &T) -> bool {
        item >= &self.min && item <= &self.max
    }

    pub fn min(&self) -> &T {
        &self.min
    }

    pub fn max(&self) -> &T {
        &self.max
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
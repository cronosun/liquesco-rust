use crate::error::LqError;
use crate::float::F32Ext;
use crate::float::F64Ext;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Bound;
use std::ops::RangeBounds;

/// A range with defined bounds.
#[derive(new, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Range<T> {
    start: T,
    end: T,
    #[new(value = "true")]
    pub start_included: bool,
    #[new(value = "true")]
    pub end_included: bool,
}

impl<T> Range<T> {
    pub fn try_inclusive(start: T, end: T) -> Result<Range<T>, LqError>
        where
            T: PartialOrd + Debug,
    {
        if start > end {
            return LqError::err_new(format!(
                "You're trying to construct a range. Those ranges require \
                 max>=main. Got {:?} for start and {:?} for end.",
                start, end
            ));
        }
        Ok(Self {
            start,
            end,
            start_included: true,
            end_included: true,
        })
    }

    #[inline]
    pub fn try_new_msg<M: Debug>(msg: M, start: T, end: T) -> Result<Self, LqError>         where
        T: PartialOrd + Debug,{
        if end < start {
            LqError::err_new(format!(
                "{:?}: You're trying to construct a range. Those ranges require \
                 max>=main. Got {:?} for min and {:?} for max.",
                msg, start, end
            ))
        } else {
            Ok(Self {
                start,
                end,
                start_included: true,
                end_included: true,
            })
        }
    }

    pub fn start(&self) -> &T {
        &self.start
    }

    pub fn end(&self) -> &T {
        &self.end
    }

    pub fn start_included(&self) -> bool {
        self.start_included
    }

    pub fn end_included(&self) -> bool {
        self.end_included
    }
}

impl<T> RangeBounds<T> for Range<T> {
    fn start_bound(&self) -> Bound<&T> {
        if self.start_included {
            Bound::Included(&self.start)
        } else {
            Bound::Excluded(&self.start)
        }
    }

    fn end_bound(&self) -> Bound<&T> {
        if self.start_included {
            Bound::Included(&self.end)
        } else {
            Bound::Excluded(&self.end)
        }
    }
}

impl<T> LqRangeBounds<T> for Range<T> {}

pub trait LqRangeBounds<T>: RangeBounds<T> {
    fn is_within_range(&self, item: &T) -> bool
        where
            T: PartialOrd,
    {
        match self.start_bound() {
            Bound::Included(start) => {
                if item >= start {
                    match self.end_bound() {
                        Bound::Included(end) => item <= end,
                        Bound::Excluded(end) => item < end,
                        Bound::Unbounded => true,
                    }
                } else {
                    false
                }
            }
            Bound::Excluded(start) => {
                if item > start {
                    match self.end_bound() {
                        Bound::Included(end) => item <= end,
                        Bound::Excluded(end) => item < end,
                        Bound::Unbounded => true,
                    }
                } else {
                    false
                }
            }
            Bound::Unbounded => match self.end_bound() {
                Bound::Included(end) => item <= end,
                Bound::Excluded(end) => item < end,
                Bound::Unbounded => true,
            },
        }
    }

    #[inline]
    fn require_within<M: Debug>(&self, msg: M, item: &T) -> Result<(), LqError>
        where
            T: Debug + PartialOrd,
    {
        if !self.is_within_range(item) {
            LqError::err_new(format!(
                "{:?}: The given value {:?} is not within given range \
                 {:?} - {:?}.",
                msg,
                item,
                self.start_bound(),
                self.end_bound()
            ))
        } else {
            Result::Ok(())
        }
    }
}

pub trait NewFull {
    fn full() -> Self;
}

impl NewFull for Range<f32> {
    fn full() -> Self {
        Self::try_inclusive(std::f32::MIN, std::f32::MAX).unwrap()
    }
}

impl NewFull for Range<f64> {
    fn full() -> Self {
        Self::try_inclusive(std::f64::MIN, std::f64::MAX).unwrap()
    }
}

impl NewFull for Range<F32Ext> {
    fn full() -> Self {
        Self::try_inclusive(std::f32::MIN.into(), std::f32::MAX.into()).unwrap()
    }
}

impl NewFull for Range<F64Ext> {
    fn full() -> Self {
        Self::try_inclusive(std::f64::MIN.into(), std::f64::MAX.into()).unwrap()
    }
}

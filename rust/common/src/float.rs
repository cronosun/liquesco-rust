use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::Hash;
use std::hash::Hasher;

/// Extends float to add missing implementation for hash, ord and eq (see
/// implementation fot rules).
#[derive(Clone, Copy, Debug, PartialOrd, Serialize, Deserialize, From, Into)]
pub struct F32Ext(f32);

/// Extends float to add missing implementation for hash, ord and eq (see
/// implementation fot rules).
#[derive(Clone, Copy, Debug, PartialOrd, Serialize, Deserialize, From, Into)]
pub struct F64Ext(f64);

/// Unfortunately we MUST have ord for the floats (need something to make sure there is
/// unique ordering in lists)
///
/// Rules:
/// NaN = NaN
/// NaN < Infinite
/// NaN < Number
/// -Infinite < Number < +Infinite
impl Ord for F32Ext {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ord) = self.0.partial_cmp(&other.0) {
            ord
        } else {
            if self.0.is_nan() {
                if other.0.is_nan() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            } else {
                panic!("Incomplete cmp implementation for float")
            }
        }
    }
}

/// Unfortunately we MUST have ord for the floats (need something to make sure there is
/// unique ordering in lists)
///
/// Rules:
/// NaN = NaN
/// NaN < Infinite
/// NaN < Number
/// -Infinite < Number < +Infinite
impl Ord for F64Ext {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ord) = self.0.partial_cmp(&other.0) {
            ord
        } else {
            if self.0.is_nan() {
                if other.0.is_nan() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            } else {
                panic!("Incomplete cmp implementation for float")
            }
        }
    }
}

impl Eq for F32Ext {}

impl Eq for F64Ext {}

impl PartialEq<F32Ext> for F32Ext {
    fn eq(&self, other: &F32Ext) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl PartialEq<F64Ext> for F64Ext {
    fn eq(&self, other: &F64Ext) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Hash for F32Ext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0.to_bits());
    }
}

impl Hash for F64Ext {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0.to_bits());
    }
}

use serde::{Deserialize, Serialize};
use crate::error::LqError;
use std::convert::TryFrom;
use std::cmp::Ordering;

/// A simple decimal numbers with 128 bit coefficient and an 8 bit exponent. Does not support
/// NaN or infinity. Supports normalization, so there's only one single representation
/// for a value.
///
/// Note: To make sure cmp and equals work you have to use normalized values.
///
/// Note: This is only to be used for serialization and validation; In your application you should
/// use your own decimal type that supports ops like add, sub, mul.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Decimal {
    coefficient: i128,
    exponent: i8,
}

impl Decimal {
    /// The only possible representation of zero.
    pub const ZERO: Decimal = Self::from_parts_de_normalized(0, 0);
    /// Minimum possible decimal value.
    pub const MIN: Decimal = Self::from_parts_de_normalized(std::i128::MIN, std::i8::MAX);
    /// Maximum possible decimal value.
    pub const MAX: Decimal = Self::from_parts_de_normalized(std::i128::MAX, std::i8::MAX);
    pub const ONE: Decimal = Self::from_parts_de_normalized(1, 0);

    #[inline]
    pub fn coefficient(&self) -> i128 {
        self.coefficient
    }

    #[inline]
    pub fn exponent(&self) -> i8 {
        self.exponent
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.coefficient == 0
    }

    /// Constructs a raw decimal value that's maybe not normalized. Only use this function
    /// when you know what you're doing.
    pub const fn from_parts_de_normalized(
        coefficient: i128,
        exponent: i8) -> Self  {
        Self {
            coefficient,
            exponent,
        }
    }

    pub fn from_parts<TC>(
        coefficient: TC,
        exponent: i8) -> Self where TC : Into<i128> {
        Self::from_parts_de_normalized(coefficient.into(), exponent).normalize()
    }

    /// True if this is normalized. Decimal should always be normalized to make sure
    /// compare and equal return useful values.
    pub fn is_normalized(&self) -> bool {
        &self.normalize() == self
    }

    /// True if coefficient is negative.
    pub fn is_sign_negative(&self) -> bool {
        self.coefficient<0
    }

    #[inline]
    pub fn normalize(self) -> Self {
        if self.coefficient == 0 {
            // there's only one zero value
            Self::ZERO
        } else if self.exponent != 0 {
            // The exponent must always be as close to zero as possible (without loosing
            // precision). E.g. 120*10^1 becomes 1200, 120*10^-1 becomes 12, 120*10^2 becomes
            // 12000.
            if self.exponent > 0 {
                let new_exponent = self.exponent - 1;
                // this could overflow
                let new_coefficient = self.coefficient.checked_mul(10);
                if let Some(new_coefficient) = new_coefficient {
                    Self::normalize(Self::from_parts_de_normalized(new_coefficient, new_exponent))
                } else {
                    self
                }
            } else if self.coefficient % 10 == 0 {
                let new_exponent = self.exponent + 1;
                // this will never cause problems (since % 10 is true)
                let new_coefficient = self.coefficient / 10;
                Self::normalize(Self::from_parts_de_normalized(new_coefficient, new_exponent))
            } else {
                self
            }
        } else {
            self
        }
    }

    fn from_str_no_normalization(string : &str) -> Result<Decimal, LqError> {
        if let Some(dot_position) = string.find(".") {
            // segment0.segment1
            // coefficient0.coefficient1
            let (segment0, negative_coefficient) = if string.starts_with('-') {
                (&string[1..dot_position], true)
            } else {
                (&string[0..dot_position], false)
            };
            let segment1 = &string[dot_position+1..];
            let segment1_len = segment1.len();
            let coefficient0 = if segment0.len()==0 {
                0i128
            } else {
                i128::from_str_radix(&segment0, 10)?
            };
            let coefficient1 = if segment1_len==0 {
                0i128
            } else { i128::from_str_radix(&segment1, 10)? };
            let multiplication = 10i128.checked_pow(u32::try_from(segment1_len)?).ok_or_else(|| {
                LqError::new(
                    format!("Value too large, cannot compute 10^x where x is {}. \
                    Given decimal value {}.",
                                         segment1_len, string))
            })?;
            let multiplied_coefficient0 = coefficient0.checked_mul(multiplication).ok_or_else(||{
                LqError::new(
                    format!("Value too large, cannot compute x*y where x is {} and y is {}. \
                    Given decimal value {}.",
                                     coefficient0, multiplication, string))
            })?;

            let coefficient = coefficient1.checked_add(multiplied_coefficient0).ok_or_else(|| {
                LqError::new(
                    format!("Value too large, cannot compute x+y where x is {} and y is {}. \
                    Given decimal value {}.",
                            coefficient1, multiplied_coefficient0, string))
            })?;
            let coefficient = if negative_coefficient {
                -coefficient
            } else {
                coefficient
            };
            let exponent = -i8::try_from(segment1_len)?;
            Ok(Self::from_parts_de_normalized(coefficient, exponent))
        } else if let Some(e_position) = string.find("e") {
            let segment0 = &string[0..e_position];
            let segment1 = &string[e_position+1..];
            let coefficient = i128::from_str_radix(&segment0, 10)?;
            let exponent = i8::from_str_radix(&segment1, 10)?;
            Ok(Self::from_parts_de_normalized(coefficient, exponent))
        } else {
            // just a plain number
            let coefficient = i128::from_str_radix(string, 10)?;
            Ok(Self::from_parts_de_normalized(coefficient, 0))
        }
    }
}

impl TryFrom<&str> for Decimal {
    type Error = LqError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let not_normalized = Self::from_str_no_normalization(value)?;
        Ok(not_normalized.normalize())
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        if self==other {
            return Ordering::Equal;
        }
        let exponent_cmp = self.exponent.cmp(&other.exponent);
        if exponent_cmp==Ordering::Equal {
            self.coefficient.cmp(&other.coefficient)
        } else {
            // Quick exit if major differences
            let self_negative = self.is_sign_negative();
            let other_negative = other.is_sign_negative();
            if self_negative && !other_negative {
                return Ordering::Less;
            } else if !self_negative && other_negative {
                return Ordering::Greater;
            }

            if self.coefficient<0 && other.coefficient>=0 {
                Ordering::Less
            } else if self.coefficient>=0 && other.coefficient<0 {
                Ordering::Greater
            } else if self.exponent>other.exponent {
                if let Some(new_self) = decrement_exponent_to(self, other.exponent) {
                    Decimal::cmp(&new_self, &other)
                } else {
                    // overflow of self
                    if self_negative {
                        // self overflows to negative
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            } else {
                if let Some(new_other) = decrement_exponent_to(other, self.exponent) {
                    Decimal::cmp(&self, &new_other)
                } else {
                    // overflow of other
                    if other_negative {
                        // other overflows to negative
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            }
        }
    }
}

/// Returns none on overflow. Only supports decrementing.
fn decrement_exponent_to(value : &Decimal, exponent : i8) -> Option<Decimal> {
    let mut current_exponent = value.exponent;
    let mut current_coefficient = value.coefficient;
    while current_exponent>exponent {
        if let Some(new_coefficient) = current_coefficient.checked_mul(10) {
            current_coefficient = new_coefficient;
        } else {
            // overflow
            return None;
        }
        current_exponent -= 1;
    }
    Some(Decimal::from_parts_de_normalized(current_coefficient, current_exponent))
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_is_normalized() {
    // normalized
    assert!(Decimal::from_parts_de_normalized(10, 0).is_normalized());
    assert!(Decimal::from_parts_de_normalized(100, 0).is_normalized());
    assert!(Decimal::from_parts_de_normalized(-100, 0).is_normalized());
    assert!(Decimal::from_parts_de_normalized(124, 0).is_normalized());
    assert!(Decimal::from_parts_de_normalized(0, 0).is_normalized());
    // not normalized
    assert!(!Decimal::from_parts_de_normalized(10, -1).is_normalized()); //norm: 1, 0
    assert!(!Decimal::from_parts_de_normalized(10, 1).is_normalized()); //norm: 100, 0
    assert!(!Decimal::from_parts_de_normalized(0, 1).is_normalized()); //norm: 0, 0
}

#[test]
fn test_normalize() {
    assert_eq!(
        Decimal::from_parts_de_normalized(10, -1).normalize(),
        Decimal::from_parts_de_normalized(1, 0),
    );
    assert_eq!(
        Decimal::from_parts_de_normalized(10, 1).normalize(),
        Decimal::from_parts_de_normalized(100, 0),
    );
    assert_eq!(
        Decimal::from_parts_de_normalized(5, 2).normalize(),
        Decimal::from_parts_de_normalized(500, 0),
    );
    assert_eq!(
        Decimal::from_parts_de_normalized(500, -2).normalize(),
        Decimal::from_parts_de_normalized(5, 0),
    );
    assert_eq!(
        Decimal::from_parts_de_normalized(-500, -2).normalize(),
        Decimal::from_parts_de_normalized(-5, 0),
    );
    assert_eq!(
        Decimal::from_parts_de_normalized(0, 1).normalize(),
        Decimal::from_parts_de_normalized(0, 0),
    );
    // this is already correct (since would overflow)
    assert_eq!(
        Decimal::from_parts_de_normalized(std::i128::MAX, 1).normalize(),
        Decimal::from_parts_de_normalized(std::i128::MAX, 1),
    );
}

#[test]
fn test_from_string() {
    // exponent = 0
    assert_eq!(
        Decimal::try_from("1").unwrap(),
        Decimal::from_parts_de_normalized(1, 0),
    );
    assert_eq!(
        Decimal::try_from("100").unwrap(),
        Decimal::from_parts_de_normalized(100, 0),
    );
    assert_eq!(
        Decimal::try_from("3342345323").unwrap(),
        Decimal::from_parts_de_normalized(3342345323, 0),
    );
    assert_eq!(
        Decimal::try_from("-3342345323").unwrap(),
        Decimal::from_parts_de_normalized(-3342345323, 0),
    );
    assert_eq!(
        Decimal::try_from("2345.0").unwrap(),
        Decimal::from_parts_de_normalized(2345, 0),
    );
    assert_eq!(
        Decimal::try_from("2345.00").unwrap(),
        Decimal::from_parts_de_normalized(2345, 0),
    );
    // exponent != 0
    assert_eq!(
        Decimal::try_from("2345.01").unwrap(),
        Decimal::from_parts_de_normalized(234501, -2),
    );
    assert_eq!(
        Decimal::try_from("2345.0002300001").unwrap(),
        Decimal::from_parts_de_normalized(23450002300001, -10),
    );
    assert_eq!(
        Decimal::try_from(".072").unwrap(),
        Decimal::from_parts_de_normalized(72, -3),
    );
    assert_eq!(
        Decimal::try_from("15.").unwrap(),
        Decimal::from_parts_de_normalized(15, 0),
    );
    assert_eq!(
        Decimal::try_from("-1.2").unwrap(),
        Decimal::from_parts_de_normalized(-12, -1),
    );
    assert_eq!(
        Decimal::try_from("-1.1").unwrap(),
        Decimal::from_parts_de_normalized(-11, -1),
    );
}

#[test]
fn test_from_string_sn() {
    // exponent = 0
    assert_eq!(
        Decimal::try_from("1e0").unwrap(),
        Decimal::from_parts_de_normalized(1, 0),
    );
    assert_eq!(
        Decimal::try_from("100e0").unwrap(),
        Decimal::from_parts_de_normalized(100, 0),
    );
    assert_eq!(
        Decimal::try_from("3342345323e0").unwrap(),
        Decimal::from_parts_de_normalized(3342345323, 0),
    );
    assert_eq!(
        Decimal::try_from("-3342345323e0").unwrap(),
        Decimal::from_parts_de_normalized(-3342345323, 0),
    );
    assert_eq!(
        Decimal::try_from("2345e0").unwrap(),
        Decimal::from_parts_de_normalized(2345, 0),
    );
    assert_eq!(
        Decimal::try_from("2345e0").unwrap(),
        Decimal::from_parts_de_normalized(2345, 0),
    );
    // exponent != 0
    assert_eq!(
        Decimal::try_from("234501e-2").unwrap(),
        Decimal::from_parts_de_normalized(234501, -2),
    );
    assert_eq!(
        Decimal::try_from("23450002300001e-10").unwrap(),
        Decimal::from_parts_de_normalized(23450002300001, -10),
    );
    assert_eq!(
        Decimal::try_from("15e58").unwrap(),
        Decimal::from_parts_de_normalized(150000000000000000000000000000000000000, 21),
    );
}

#[test]
fn test_ord() {
    assert_ord(
        &Decimal::try_from("1").unwrap(),
        &Decimal::try_from("2").unwrap()
    );
    assert_ord(
        &Decimal::try_from("0").unwrap(),
        &Decimal::try_from("1").unwrap()
    );
    assert_ord(
        &Decimal::try_from("-1").unwrap(),
        &Decimal::try_from("0").unwrap()
    );
    assert_ord(
        &Decimal::try_from("-2").unwrap(),
        &Decimal::try_from("-1").unwrap()
    );
    assert_ord(
        &Decimal::try_from("-1.2").unwrap(),
        &Decimal::try_from("-1.1").unwrap()
    );
    assert_ord(
        &Decimal::try_from("1.1").unwrap(),
        &Decimal::try_from("1.2").unwrap()
    );
    assert_ord(
        &Decimal::try_from("-1.1").unwrap(),
        &Decimal::try_from("1.2").unwrap()
    );
    assert_ord(
        &Decimal::try_from("11.1111212").unwrap(),
        &Decimal::try_from("12").unwrap()
    );
    assert_ord(
        &Decimal::try_from("11").unwrap(),
        &Decimal::try_from("12.111122").unwrap()
    );
    assert_ord(
        &Decimal::from_parts(std::i64::MAX-1, std::i8::MAX),
        &Decimal::from_parts(std::i64::MAX, std::i8::MAX)
    );
    assert_ord(
        &Decimal::from_parts(std::i64::MAX, std::i8::MAX-1),
        &Decimal::from_parts(std::i64::MAX, std::i8::MAX)
    );
    assert_ord(
        &Decimal::from_parts(std::i128::MAX, std::i8::MAX-1),
        &Decimal::from_parts(std::i128::MAX, std::i8::MAX)
    );
    assert_ord(
        &Decimal::from_parts(std::i128::MIN+1, std::i8::MAX),
            &Decimal::ZERO,
    );
    assert_ord(
        &Decimal::from_parts(std::i128::MIN, std::i8::MAX-1),
        &Decimal::from_parts(std::i128::MIN+1, std::i8::MAX-2),

    );
    assert_ord(
        &Decimal::ZERO,
        &Decimal::from_parts(std::i128::MAX-1, std::i8::MAX)
    );
    assert_ord(
        &Decimal::from_parts( -170141183460469231731687303715884105727i128, 127),
        &Decimal::from_parts(-10, 0)
    );
    assert_ord(
        &Decimal::from_parts(-10, 0),
        &Decimal::from_parts(  170141183460469231731687303715884105726i128, 127),
    );
}

#[test]
fn test_ord_eq() {
    assert_eq(
        &Decimal::try_from("1").unwrap(),
        &Decimal::try_from("1").unwrap()
    );
    assert_eq(
        &Decimal::try_from("0").unwrap(),
        &Decimal::try_from("0").unwrap()
    );
    assert_eq(
        &Decimal::try_from("-1").unwrap(),
        &Decimal::try_from("-1").unwrap()
    );
    assert_eq(
        &Decimal::try_from("10").unwrap(),
        &Decimal::try_from("10.0").unwrap()
    );
    assert_eq(
        &Decimal::try_from("-99.99").unwrap(),
        &Decimal::try_from("-99.9900").unwrap()
    );
    assert_eq(
        &Decimal::try_from(".001").unwrap(),
        &Decimal::try_from("000.0010").unwrap()
    );
}

#[cfg(test)]
fn assert_ord(lesser : &Decimal, greater : &Decimal) {
    assert_eq!(lesser.cmp(greater), Ordering::Less);
}

#[cfg(test)]
fn assert_eq(lhs : &Decimal, rhs : &Decimal) {
    assert_eq!(lhs.cmp(rhs), Ordering::Equal);
}
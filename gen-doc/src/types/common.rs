// Common functions

use liquesco_common::decimal::Decimal;
use num_format::{Locale, ToFormattedString};
use std::convert::TryFrom;
use std::fmt::Display;
use std::num::FpCategory;

pub(crate) struct Common;

const U64_U8_MAX: u64 = std::u8::MAX as u64;
const U64_U16_MAX: u64 = std::u16::MAX as u64;
const U64_U32_MAX: u64 = std::u32::MAX as u64;
const I64_U8_MAX: i64 = std::i8::MAX as i64;
const I64_U16_MAX: i64 = std::i16::MAX as i64;
const I64_U32_MAX: i64 = std::i32::MAX as i64;
const I64_U8_MIN: i64 = std::i8::MIN as i64;
const I64_U16_MIN: i64 = std::i16::MIN as i64;
const I64_U32_MIN: i64 = std::i32::MIN as i64;

impl Common {
    pub fn txt_sorting(sorting: TxtSorting) -> &'static str {
        match sorting {
            TxtSorting::Ascending => "Ascending",
            TxtSorting::Descending => "Descending",
        }
    }

    pub fn fmt_bool_yes_no(value: bool) -> &'static str {
        if value {
            "Yes"
        } else {
            "No"
        }
    }

    pub fn fmt_u32(value: u32) -> String {
        Self::fmt_u64(u64::from(value))
    }

    pub fn fmt_u64(value: u64) -> String {
        let special_value = match value {
            U64_U8_MAX => Some("Unsigned int 8 maximum"),
            U64_U16_MAX => Some("Unsigned int 16 maximum"),
            U64_U32_MAX => Some("Unsigned int 32 maximum"),
            std::u64::MAX => Some("Unsigned int 64 maximum"),
            _ => None,
        };
        if let Some(special_value) = special_value {
            format!(
                "{} ({})",
                special_value,
                ToFormattedString::to_formatted_string(&value, &Locale::en)
            )
        } else {
            ToFormattedString::to_formatted_string(&value, &Locale::en)
        }
    }

    pub fn fmt_u128(value: &u128) -> String {
        if let Ok(u64_value) = u64::try_from(*value) {
            Self::fmt_u64(u64_value)
        } else {
            if value == &std::u128::MAX {
                format!("Unsigned int 128 maximum")
            } else {
                ToFormattedString::to_formatted_string(value, &Locale::en)
            }
        }
    }

    pub fn fmt_i64(value: i64) -> String {
        if value > 0 {
            if let Ok(value) = u64::try_from(value) {
                return Self::fmt_u64(value);
            }
        }

        let special_value = match value {
            I64_U8_MAX => Some("Signed int 8 maximum"),
            I64_U16_MAX => Some("Signed int 16 maximum"),
            I64_U32_MAX => Some("Signed int 32 maximum"),
            std::i64::MAX => Some("Signed int 64 maximum"),
            I64_U8_MIN => Some("Signed int 8 minimum"),
            I64_U16_MIN => Some("Signed int 16 minimum"),
            I64_U32_MIN => Some("Signed int 32 minimum"),
            std::i64::MIN => Some("Signed int 64 minimum"),
            _ => None,
        };
        if let Some(special_value) = special_value {
            format!(
                "{} ({})",
                special_value,
                ToFormattedString::to_formatted_string(&value, &Locale::en)
            )
        } else {
            ToFormattedString::to_formatted_string(&value, &Locale::en)
        }
    }

    pub fn fmt_i128(value: &i128) -> String {
        if let Ok(i64_value) = i64::try_from(*value) {
            Self::fmt_i64(i64_value)
        } else {
            match value {
                &std::i128::MAX => format!("Unsigned int 128 maximum"),
                &std::i128::MIN => format!("Unsigned int 128 minimum"),
                _ => ToFormattedString::to_formatted_string(value, &Locale::en),
            }
        }
    }

    pub fn fmt_f64(value: &f64) -> String {
        Self::fmt_float(
            value.classify(),
            value.is_sign_negative(),
            value,
            &std::f64::MIN,
            &std::f64::MAX,
        )
    }

    pub fn fmt_f32(value: &f32) -> String {
        Self::fmt_float(
            value.classify(),
            value.is_sign_negative(),
            value,
            &std::f32::MIN,
            &std::f32::MAX,
        )
    }

    pub fn fmt_decimal(value: &Decimal) -> String {
        match value {
            &Decimal::MAX => format!("Decimal maximum value"),
            &Decimal::MIN => format!("Decimal minimum value"),
            _ => format!("{}", value),
        }
    }

    fn fmt_float<T>(category: FpCategory, negative: bool, value: &T, min: &T, max: &T) -> String
    where
        T: Display + PartialEq,
    {
        let (special_value, display_value) = match category {
            FpCategory::Nan => (Some("Not a number"), false),
            FpCategory::Infinite => {
                if negative {
                    (Some("Negative infinity"), false)
                } else {
                    (Some("Positive infinity"), false)
                }
            }
            FpCategory::Zero => {
                if negative {
                    (Some("Negative zero"), false)
                } else {
                    (Some("Positive zero"), false)
                }
            }
            FpCategory::Subnormal => (Some("Subnormal"), true),
            FpCategory::Normal => {
                if value == max {
                    (Some("Float maximum value"), false)
                } else if value == min {
                    (Some("Float minimum value"), false)
                } else {
                    (None, true)
                }
            }
        };
        if let Some(special_value) = special_value {
            if display_value {
                format!("{} ({})", special_value, value)
            } else {
                format!("{}", special_value)
            }
        } else {
            if display_value {
                format!("{}", value)
            } else {
                format!("Unknown")
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum TxtSorting {
    Ascending,
    Descending,
}

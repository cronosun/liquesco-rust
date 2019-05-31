use crate::core::MajorType;
use std::fmt::{Debug, Formatter, Error};

// Minimum type ID is 0, maximum is 20 (inclusive) - so there's
// enough for 20 major types.

/// Boolean value false.
pub(crate) const TYPE_BOOL_FALSE: MajorType = MajorType::new(0);
/// Boolean value true.
pub(crate) const TYPE_BOOL_TRUE: MajorType = MajorType::new(1);
/// The option type: present(value) or absent.
pub(crate) const TYPE_OPTION: MajorType = MajorType::new(2);
/// A sequence of items. Items do not have to be of the same type (so this can be used to
/// encode sequences and also structs).
pub(crate) const TYPE_SEQ: MajorType = MajorType::new(3);
/// Arbitrary binary.
pub(crate) const TYPE_BINARY: MajorType = MajorType::new(4);
/// Unicode (UTF-8) text.
pub(crate) const TYPE_UNICODE: MajorType = MajorType::new(5);

/// 64 bit unsigned integer.
pub(crate) const TYPE_UINT: MajorType = MajorType::new(6);
/// 64 bit signed integer.
pub(crate) const TYPE_SINT: MajorType = MajorType::new(7);

/// 32 or 64 bit float.
pub(crate) const TYPE_FLOAT: MajorType = MajorType::new(8);

/// Enum variant ordinal = 0.
pub(crate) const TYPE_ENUM_0: MajorType = MajorType::new(9);
/// Enum variant ordinal = 1.
pub(crate) const TYPE_ENUM_1: MajorType = MajorType::new(10);
/// Enum variant ordinal = 2.
pub(crate) const TYPE_ENUM_2: MajorType = MajorType::new(11);
/// Enum variant ordinal = 3.
pub(crate) const TYPE_ENUM_3: MajorType = MajorType::new(12);
/// Enum variant ordinal > 3.
pub(crate) const TYPE_ENUM_N: MajorType = MajorType::new(13);

impl Debug for MajorType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let name : &str = match self {
            &TYPE_BOOL_FALSE => "TYPE_BOOL_FALSE",
            &TYPE_BOOL_TRUE => "TYPE_BOOL_TRUE",
            &TYPE_OPTION => "TYPE_OPTION",
            &TYPE_SEQ => "TYPE_SEQ",
            &TYPE_BINARY => "TYPE_BINARY",
            &TYPE_UNICODE => "TYPE_UNICODE",
            &TYPE_UINT => "TYPE_UINT",
            &TYPE_SINT => "TYPE_SINT",
            &TYPE_FLOAT => "TYPE_FLOAT",
            &TYPE_ENUM_0 => "TYPE_ENUM_0",
            &TYPE_ENUM_1 => "TYPE_ENUM_1",
            &TYPE_ENUM_2 => "TYPE_ENUM_2",
            &TYPE_ENUM_3 => "TYPE_ENUM_3",
            &TYPE_ENUM_N => "TYPE_ENUM_N",
            _ => "Unknown"
        };
        write!(f, "MajorType({}; {})", self.id(), name)
    }
}
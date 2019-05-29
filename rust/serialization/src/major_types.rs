use crate::core::MajorType;

// Minimum type ID is 0, maximum is 20 (inclusive) - so there's
// enough for 20 major types.

/// Boolean value false.
pub const TYPE_BOOL_FALSE: MajorType = MajorType::new(0);
/// Boolean value true.
pub const TYPE_BOOL_TRUE: MajorType = MajorType::new(1);
/// The option type: present(value) or absent.
pub const TYPE_OPTION: MajorType = MajorType::new(2);
/// A sequence of items. Items do not have to be of the same type (so this can be used to
/// encode sequences and also structs).
pub const TYPE_SEQ: MajorType = MajorType::new(3);
/// Arbitrary binary.
pub const TYPE_BINARY: MajorType = MajorType::new(4);
/// Unicode (UTF-8) text.
pub const TYPE_UNICODE: MajorType = MajorType::new(5);

/// 64 bit unsigned integer.
pub const TYPE_UINT: MajorType = MajorType::new(6);
/// 64 bit signed integer.
pub const TYPE_SINT: MajorType = MajorType::new(7);

/// 32 or 64 bit float.
pub const TYPE_FLOAT: MajorType = MajorType::new(8);

/// Enum variant ordinal = 0.
pub const TYPE_ENUM_0: MajorType = MajorType::new(9);
/// Enum variant ordinal = 1.
pub const TYPE_ENUM_1: MajorType = MajorType::new(10);
/// Enum variant ordinal = 2.
pub const TYPE_ENUM_2: MajorType = MajorType::new(11);
/// Enum variant ordinal = 3.
pub const TYPE_ENUM_3: MajorType = MajorType::new(12);
/// Enum variant ordinal > 3.
pub const TYPE_ENUM_N: MajorType = MajorType::new(13);

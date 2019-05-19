use crate::serialization::core::MajorType;

// Minimum type ID is 0, maximum is 20 (inclusive) - so there's 
// enough for 20 major types.

pub const TYPE_BOOL_FALSE: MajorType = MajorType::new(0);
pub const TYPE_BOOL_TRUE: MajorType = MajorType::new(1);
pub const TYPE_OPTION: MajorType = MajorType::new(2);
pub const TYPE_SEQ: MajorType = MajorType::new(3);
pub const TYPE_BINARY: MajorType = MajorType::new(4);
pub const TYPE_UNICODE: MajorType = MajorType::new(5);

pub const TYPE_UINT: MajorType = MajorType::new(6);
pub const TYPE_SINT: MajorType = MajorType::new(7);

pub const TYPE_FLOAT: MajorType = MajorType::new(8);

pub const TYPE_ENUM_0: MajorType = MajorType::new(9);
pub const TYPE_ENUM_1: MajorType = MajorType::new(10);
pub const TYPE_ENUM_2: MajorType = MajorType::new(11);
pub const TYPE_ENUM_3: MajorType = MajorType::new(12);
pub const TYPE_ENUM_N: MajorType = MajorType::new(13);
use crate::serialization::core::TypeId;

// Minimum type ID is 0, maximum is 31 (inclusive).

pub const TYPE_BOOL_FALSE: TypeId = TypeId::new(0);
pub const TYPE_BOOL_TRUE: TypeId = TypeId::new(1);

pub const TYPE_OPTION: TypeId = TypeId::new(2);

pub const TYPE_STRUCT_0: TypeId = TypeId::new(3);
pub const TYPE_STRUCT_1: TypeId = TypeId::new(4);
pub const TYPE_STRUCT_2: TypeId = TypeId::new(5);
pub const TYPE_STRUCT_3: TypeId = TypeId::new(6);
pub const TYPE_STRUCT_U8: TypeId = TypeId::new(7);
pub const TYPE_STRUCT_U16: TypeId = TypeId::new(8);

pub const TYPE_BINARY: TypeId = TypeId::new(9);
pub const TYPE_UTF8: TypeId = TypeId::new(10);

// TODO: Integer types (2), timestamp (c.a. 3), duration (c.a. 2), floats (2), dec64 (1)

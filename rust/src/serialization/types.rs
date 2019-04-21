use crate::serialization::core::TypeBlock;
use crate::serialization::core::TypeId;

// 4 blocks are reserved for base types (one block = 16 types).

pub const BLOCK_ID_BINARY : TypeBlock = TypeBlock::new(4);
pub const BLOCK_ID_UTF8 : TypeBlock = TypeBlock::new(5);
pub const BLOCK_ID_EMBEDDED : TypeBlock = TypeBlock::new(6); // TODO: Do we really need this?
pub const BLOCK_ID_REF_BLAKE2B : TypeBlock = TypeBlock::new(7);

// types. Range 0 to 63 (the 4 lowest blocks)

pub const TYPE_BOOL_FALSE: TypeId = TypeId::new(0);
pub const TYPE_BOOL_TRUE: TypeId = TypeId::new(1);

pub const TYPE_OPTION_ABSENT: TypeId = TypeId::new(2);
pub const TYPE_OPTION_PRESENT: TypeId = TypeId::new(3);

pub const TYPE_STRUCT_0: TypeId = TypeId::new(4);
pub const TYPE_STRUCT_1: TypeId = TypeId::new(5);
pub const TYPE_STRUCT_2: TypeId = TypeId::new(6);
pub const TYPE_STRUCT_3: TypeId = TypeId::new(7);
pub const TYPE_STRUCT_4: TypeId = TypeId::new(8);
pub const TYPE_STRUCT_5: TypeId = TypeId::new(9);
pub const TYPE_STRUCT_6: TypeId = TypeId::new(10);
pub const TYPE_STRUCT_U8: TypeId = TypeId::new(11);
pub const TYPE_STRUCT_U16: TypeId = TypeId::new(12);

// TODO: Integer types (10), timestamp (c.a. 3), duration (c.a. 2), floats (2), dec64 (1)




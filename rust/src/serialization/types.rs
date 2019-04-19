use crate::serialization::binary::BlockId;
use crate::serialization::core::TypeId;

// 4 blocks are reserved for base types (one block = 16 types).

pub const BLOCK_ID_BINARY : BlockId = BlockId(4);
pub const BLOCK_ID_UTF8 : BlockId = BlockId(5);
pub const BLOCK_ID_EMBEDDED : BlockId = BlockId(6); // TODO: Do we really need this?
pub const BLOCK_ID_REF_BLAKE2B : BlockId = BlockId(7);

// types. Range 0 to 63 (the 4 lowest blocks)

pub const TYPE_BOOL_FALSE: TypeId = TypeId(0);
pub const TYPE_BOOL_TRUE: TypeId = TypeId(1);

pub const TYPE_OPTION_ABSENT: TypeId = TypeId(2);
pub const TYPE_OPTION_PRESENT: TypeId = TypeId(3);

pub const TYPE_STRUCT_0: TypeId = TypeId(4);
pub const TYPE_STRUCT_1: TypeId = TypeId(5);
pub const TYPE_STRUCT_2: TypeId = TypeId(6);
pub const TYPE_STRUCT_3: TypeId = TypeId(7);
pub const TYPE_STRUCT_4: TypeId = TypeId(8);
pub const TYPE_STRUCT_5: TypeId = TypeId(9);
pub const TYPE_STRUCT_6: TypeId = TypeId(10);
pub const TYPE_STRUCT_U8: TypeId = TypeId(11);
pub const TYPE_STRUCT_U16: TypeId = TypeId(12);

// TODO: Integer types (10), timestamp (c.a. 3), duration (c.a. 2), floats (2), dec64 (1)




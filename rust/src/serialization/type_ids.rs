use crate::serialization::core::TypeId;

// Minimum type ID is 0, maximum is 24 (inclusive).

pub const TYPE_BOOL_FALSE: TypeId = TypeId::new(0);
pub const TYPE_BOOL_TRUE: TypeId = TypeId::new(1);
pub const TYPE_OPTION: TypeId = TypeId::new(2);
pub const TYPE_LIST: TypeId = TypeId::new(3);
pub const TYPE_BINARY: TypeId = TypeId::new(4);
pub const TYPE_UTF8: TypeId = TypeId::new(5);

pub const TYPE_ENUM_0: TypeId = TypeId::new(6);
pub const TYPE_ENUM_1: TypeId = TypeId::new(7);
pub const TYPE_ENUM_2: TypeId = TypeId::new(8);
pub const TYPE_ENUM_N: TypeId = TypeId::new(9);

pub const TYPE_UINT: TypeId = TypeId::new(10);
pub const TYPE_SINT: TypeId = TypeId::new(11);

pub const TYPE_UUID: TypeId = TypeId::new(12);

//pub const TYPE_DEC128: TypeId = TypeId::new(12);

// -> ne, glaube custom brauchts nicht.. dazu haben wir ja das schema
// custom0: 20
// custom1: 21
// custom2: 22
// custom3: 23
// custom_arb: 24

// TODO: Integer types (2), timestamp (1), time of day, day (calendar), floats (2), Decimal128, Extension, UUID? reverse domain?

use crate::serialization::core::MajorType;

// Minimum type ID is 0, maximum is 24 (inclusive).

pub const TYPE_BOOL_FALSE: MajorType = MajorType::new(0);
pub const TYPE_BOOL_TRUE: MajorType = MajorType::new(1);
pub const TYPE_OPTION: MajorType = MajorType::new(2);
pub const TYPE_LIST: MajorType = MajorType::new(3);
pub const TYPE_BINARY: MajorType = MajorType::new(4);
pub const TYPE_UNICODE: MajorType = MajorType::new(5);

pub const TYPE_ENUM_0: MajorType = MajorType::new(6);
pub const TYPE_ENUM_1: MajorType = MajorType::new(7);
pub const TYPE_ENUM_2: MajorType = MajorType::new(8);
pub const TYPE_ENUM_N: MajorType = MajorType::new(9);

pub const TYPE_UINT: MajorType = MajorType::new(10);
pub const TYPE_SINT: MajorType = MajorType::new(11);

pub const TYPE_FLOAT: MajorType = MajorType::new(12);

// TOOD: Ne, ich glaube wir lassen das eher... denn das zeugs kann serde alles nicht serialisieren.. Ich würde da eher das binary nehmen oder eine struct und dann das nur im schema drinn haben... vielleicht noch das int128 rein nehmen?
// man könnte dann auch die breite erhähen, damit man in der länge auch 128bit noch drinn hätte... ggf. auch noch das 256 bit und das 256bit
// dann hätte man auch noch platz für einen zusätzliche enum

//pub const TYPE_DEC128: TypeId = TypeId::new(12);

// -> ne, glaube custom brauchts nicht.. dazu haben wir ja das schema
// custom0: 20
// custom1: 21
// custom2: 22
// custom3: 23
// custom_arb: 24

// TODO: Integer types (2), timestamp (1), time of day, day (calendar), floats (2), Decimal128, Extension, UUID? reverse domain?

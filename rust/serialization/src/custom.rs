
use crate::core::TypeId;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CustomTypeId(pub u8);

impl TypeId {
    fn extract_custom(&self) -> CustomInfo {

    }
}

pub enum CustomInfo {
    NotCustom,
    EmbeddedWithLength((CustomTypeId, u8)),
    EmbeddedVariableLength(CustomTypeId),
    U8Length,
    U16Length,
    U24Length
}
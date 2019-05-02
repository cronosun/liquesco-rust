use crate::serialization::core::LengthMarker;
use crate::serialization::core::TypeHeader;
use crate::serialization::core::TypeId;

#[test]
fn new_type_header() {
    let header = TypeHeader::new(LengthMarker::Len1, TypeId::new(10));
    assert_eq!(101, header.id());
    let header = TypeHeader::new(LengthMarker::Len2, TypeId::new(10));
    assert_eq!(102, header.id());
    let header = TypeHeader::new(LengthMarker::VarInt, TypeId::new(10));
    assert_eq!(105, header.id());
    let header = TypeHeader::new(LengthMarker::VarInt, TypeId::new(0));
    assert_eq!(5, header.id());
    let header = TypeHeader::new(LengthMarker::ContainerVarIntVarInt, TypeId::new(5));
    assert_eq!(56, header.id());
}

#[test]
fn decompose_type_header() {
    assert_eq!(LengthMarker::Len1, TypeHeader::from_u8(101).length_marker());
    assert_eq!(TypeId::new(10), TypeHeader::from_u8(101).type_id());
    assert_eq!(
        LengthMarker::ContainerVarIntVarInt,
        TypeHeader::from_u8(56).length_marker()
    );
    assert_eq!(TypeId::new(5), TypeHeader::from_u8(56).type_id());
}

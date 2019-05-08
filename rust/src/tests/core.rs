use crate::serialization::core::ContentInfo;
use crate::serialization::core::TypeHeader;
use crate::serialization::core::MajorType;

const FACTOR : u8 = 12;

#[test]
fn new_type_header() {
    let header = TypeHeader::new(ContentInfo::Len1, MajorType::new(10));
    assert_eq!(10 * FACTOR + 1, header.id());
    let header = TypeHeader::new(ContentInfo::Len2, MajorType::new(10));
    assert_eq!(10 * FACTOR + 2, header.id());
    let header = TypeHeader::new(ContentInfo::VarInt, MajorType::new(10));
    assert_eq!(10 * FACTOR + 6, header.id());
    let header = TypeHeader::new(ContentInfo::VarInt, MajorType::new(0));
    assert_eq!(0 * FACTOR + 6, header.id());
    let header = TypeHeader::new(ContentInfo::ContainerVarIntVarInt, MajorType::new(5));
    assert_eq!(5 * FACTOR + 7, header.id());
}

#[test]
fn decompose_type_header() {
    assert_eq!(ContentInfo::Len1, TypeHeader::from_u8(10 * FACTOR + 1).content_info());
    assert_eq!(MajorType::new(10), TypeHeader::from_u8(10 * FACTOR + 1).major_type());
    assert_eq!(
        ContentInfo::ContainerVarIntVarInt,
        TypeHeader::from_u8(5 * FACTOR + 7).content_info()
    );
    assert_eq!(MajorType::new(5), TypeHeader::from_u8(5 * FACTOR + 6).major_type());
}

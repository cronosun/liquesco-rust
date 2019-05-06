use crate::serialization::core::ContentInfo;
use crate::serialization::core::TypeHeader;
use crate::serialization::core::MajorType;

#[test]
fn new_type_header() {
    let header = TypeHeader::new(ContentInfo::Len1, MajorType::new(10));
    assert_eq!(101, header.id());
    let header = TypeHeader::new(ContentInfo::Len2, MajorType::new(10));
    assert_eq!(102, header.id());
    let header = TypeHeader::new(ContentInfo::VarInt, MajorType::new(10));
    assert_eq!(105, header.id());
    let header = TypeHeader::new(ContentInfo::VarInt, MajorType::new(0));
    assert_eq!(5, header.id());
    let header = TypeHeader::new(ContentInfo::ContainerVarIntVarInt, MajorType::new(5));
    assert_eq!(56, header.id());
}

#[test]
fn decompose_type_header() {
    assert_eq!(ContentInfo::Len1, TypeHeader::from_u8(101).content_info());
    assert_eq!(MajorType::new(10), TypeHeader::from_u8(101).major_type());
    assert_eq!(
        ContentInfo::ContainerVarIntVarInt,
        TypeHeader::from_u8(56).content_info()
    );
    assert_eq!(MajorType::new(5), TypeHeader::from_u8(56).major_type());
}

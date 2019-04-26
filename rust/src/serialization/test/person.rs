use crate::serialization::core::BinaryWriter;
use crate::serialization::core::BinaryReader;
use crate::serialization::toption::Presence;

use crate::serialization::core::DeSerializer;
use crate::serialization::core::LqError;
use crate::serialization::core::Serializer;
use crate::serialization::tstruct::StructInfo;
use crate::serialization::tutf8::TUtf8;
use std::borrow::Cow;

#[derive(Debug, Eq, PartialEq)]
struct Person<'a> {
    first_name: Cow<'a, str>,
    last_name: Cow<'a, str>,
    male: bool,
    address: Option<Address<'a>>,
}

#[derive(Debug, Eq, PartialEq)]
struct Address<'a> {
    street: Cow<'a, str>,
}

impl<'a> DeSerializer<'a> for Address<'a> {
    type Item = Self;

    fn de_serialize<T: BinaryReader<'a>>(reader: &mut T) -> Result<Self::Item, LqError> {
        let reading = StructInfo::de_serialize(reader)?.begin(1)?;
        let result = Result::Ok(Self {
            street: Cow::Borrowed(TUtf8::de_serialize(reader)?),
        });
        reading.finish(reader)?;
        result
    }   
}

impl<'a> Serializer for Address<'a> {
    type Item = Self;

    fn serialize<T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        StructInfo::serialize(writer, &StructInfo::new(1))?;
        TUtf8::serialize(writer, &item.street)
    }   
}

impl<'a> DeSerializer<'a> for Person<'a> {
    type Item = Self;

    fn de_serialize<T: BinaryReader<'a>>(reader: &mut T) -> Result<Self::Item, LqError> {
        let struct_read = StructInfo::de_serialize(reader)?.begin(4)?;
        let first_name = TUtf8::de_serialize(reader)?;
        let last_name = TUtf8::de_serialize(reader)?;
        let male = bool::de_serialize(reader)?;
        let has_address = Presence::de_serialize(reader)?;
        let maybe_address = match has_address {
            Presence::Present => {
                Option::Some(Address::de_serialize(reader)?)
            },
            Presence::Absent => {
                Option::None
            }
        };        
        struct_read.finish(reader)?;
        Result::Ok(Self {
            first_name: Cow::Borrowed(first_name),
            last_name: Cow::Borrowed(last_name),
            male: male,
            address: maybe_address,
        })
    }
}

impl<'a> Serializer for Person<'a> {
    type Item = Self;

    fn serialize<T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        StructInfo::serialize(writer, &StructInfo::new(4))?;
        TUtf8::serialize(writer, &item.first_name)?;
        TUtf8::serialize(writer, &item.last_name)?;
        bool::serialize(writer, &item.male)?;
        match &item.address {
            Option::Some(address) => {
                Presence::serialize(writer, &Presence::Present)?;
                Address::serialize(writer, address)?
            },
            Option::None => Presence::serialize(writer, &Presence::Absent)?
        };
        Result::Ok(())
    }
}
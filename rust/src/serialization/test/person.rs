use crate::serialization::toption::TOption;
use crate::serialization::toption::Presence;

use crate::serialization::core::DeSerializable;
use crate::serialization::core::LqError;
use crate::serialization::core::Reader;
use crate::serialization::core::Serializable;
use crate::serialization::core::Writer;
use crate::serialization::tbool::TBool;
use crate::serialization::tstruct::StructInfo;
use crate::serialization::tstruct::TStruct;
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

impl<'a> DeSerializable<'a> for Address<'a> {
    fn de_serialize<T: Reader<'a>>(reader: &mut T) -> Result<Self, LqError>
    where
        Self: Sized,
    {
        reader.read::<TStruct>()?.assert(1)?;
        Result::Ok(Self {
            street: Cow::Borrowed(reader.read::<TUtf8>()?),
        })
    }
}

impl<'a> Serializable for Address<'a> {
    fn serialize<T: Writer>(&self, writer: &mut T) -> Result<(), LqError> {
        writer.write::<TStruct>(&StructInfo::new(1))?;
        writer.write::<TUtf8>(&self.street)
    }
}

impl<'a> DeSerializable<'a> for Person<'a> {
    fn de_serialize<T: Reader<'a>>(reader: &'a mut T) -> Result<Self, LqError>
    where
        Self: Sized,
    {
        reader.read::<TStruct>()?.assert(4)?;
        let first_name = reader.read::<TUtf8>()?;
        let last_name = reader.read::<TUtf8>()?;
        let male = reader.read::<TBool>()?;
        let has_address = reader.read::<TOption>()?;
        let maybe_address = match has_address {
            Presence::Present => {
                Option::Some(Address::de_serialize(reader)?)
            },
            Presence::Absent => {
                Option::None
            }
        } ;
        Result::Ok(Self {
            first_name: Cow::Borrowed(first_name),
            last_name: Cow::Borrowed(last_name),
            male: male,
            address: maybe_address,
        })
    }
}

impl<'a> Serializable for Person<'a> {
    fn serialize<T: Writer>(&self, writer: &mut T) -> Result<(), LqError> {
        writer.write::<TStruct>(&StructInfo::new(4))?;
        writer.write::<TUtf8>(&self.first_name)?;
        writer.write::<TUtf8>(&self.last_name)?;
        writer.write::<TBool>(&self.male)?;
        match &self.address {
            Option::Some(address) => {
                writer.write::<TOption>(&Presence::Present)?;
                address.serialize(writer)?
            },
            Option::None => writer.write::<TOption>(&Presence::Absent)?
        };
        Result::Ok(())
    }
}
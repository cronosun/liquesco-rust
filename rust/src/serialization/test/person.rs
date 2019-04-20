use crate::serialization::core::DeSerializable;
use crate::serialization::core::LqError;
use crate::serialization::core::Reader;
use crate::serialization::tutf8::TUtf8;
use std::borrow::Cow;

struct Person<'a> {
    first_name: Cow<'a, str>,
    last_name: Cow<'a, str>,
    male: bool,
    address: Option<Address<'a>>,
}

struct Address<'a> {
    street : Cow<'a, str>
}

impl<'a> DeSerializable<'a> for Address<'a> {
    fn de_serialize<T: Reader<'a>>(reader: &'a mut T) -> Result<Self, LqError>
    where
        Self: Sized,
    {
        Result::Ok(Self {
            street : Cow::Borrowed(reader.read::<TUtf8>()?)
        })
    }
}

impl<'a> DeSerializable<'a> for Person<'a> {
    fn de_serialize<T: Reader<'a>>(reader: &'a mut T) -> Result<Self, LqError>
    where
        Self: Sized,
    {
        let first_name = reader.read::<TUtf8>()?;
        let address = Address::de_serialize(reader)?;
        Result::Ok(Self {
            first_name: Cow::Borrowed(first_name),
            last_name: Cow::Borrowed("todo"),
            male: false,
            address: Option::Some(address),
        })
    }
}

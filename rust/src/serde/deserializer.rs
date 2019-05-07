use crate::common::error::LqError;
use crate::common::internal_utils::try_from_int_result;
use crate::serde::error::SLqError;
use crate::serialization::core::DeSerializer;
use crate::serialization::tbinary::TBinary;
use crate::serialization::tbool::TBool;
use crate::serialization::tlist::ListHeader;
use crate::serialization::toption::Presence;
use crate::serialization::tsint::{TSInt, TSInt16, TSInt32, TSInt8};
use crate::serialization::tuint::{TUInt, TUInt16, TUInt32, TUInt8};
use crate::serialization::tutf8::TUtf8;
use serde::de::DeserializeSeed;
use serde::de::SeqAccess;
use serde::de::Visitor;
use std::convert::TryFrom;
use std::marker::PhantomData;

use serde::de;

use crate::serialization::core::BinaryReader;

pub fn new_deserializer<'de, R>(reader : R) -> Deserializer<'de, R> where R : BinaryReader<'de> {
    Deserializer {
        reader,
        _marker: &PhantomData,
    }
}

pub struct Deserializer<'de, R: BinaryReader<'de>> {
    reader: R,
    _marker: &'de PhantomData<()>,
}

impl<'de, R: BinaryReader<'de>> Deserializer<'de, R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            _marker: &PhantomData,
        }
    }
}

type Result<Ok> = std::result::Result<Ok, SLqError>;

impl<'de, R: BinaryReader<'de>> de::Deserializer<'de> for &'de mut Deserializer<'de, R> {
    type Error = SLqError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(LqError::new("This is not a self-describing format. Operation not supported").into())
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TBool::de_serialize(&mut self.reader)?;
        visitor.visit_bool(value)
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TSInt8::de_serialize(&mut self.reader)?;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TSInt16::de_serialize(&mut self.reader)?;
        visitor.visit_i16(value)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TSInt32::de_serialize(&mut self.reader)?;
        visitor.visit_i32(value)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TSInt::de_serialize(&mut self.reader)?;
        visitor.visit_i64(value)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TUInt8::de_serialize(&mut self.reader)?;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TUInt16::de_serialize(&mut self.reader)?;
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TUInt32::de_serialize(&mut self.reader)?;
        visitor.visit_u32(value)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TUInt::de_serialize(&mut self.reader)?;
        visitor.visit_u64(value)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!("floats not yet implemented")
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!("floats not yet implemented")
    }

    // Characters are just u32
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TUInt32::de_serialize(&mut self.reader)?;
        let maybe_char = std::char::from_u32(value);
        if let Some(chr) = maybe_char {
            visitor.visit_char(chr)
        } else {
            Err(LqError::new(format!(
                "Value {:?} is not a valid character (unicode)",
                value
            ))
            .into())
        }
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TUtf8::de_serialize(&mut self.reader)?;
        visitor.visit_borrowed_str(value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TUtf8::de_serialize(&mut self.reader)?;
        visitor.visit_string(value.into())
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TBinary::de_serialize(&mut self.reader)?;
        visitor.visit_bytes(value)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = TBinary::de_serialize(&mut self.reader)?;
        visitor.visit_byte_buf(value.into())
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = Presence::de_serialize(&mut self.reader)?;
        match value {
            Presence::Absent => visitor.visit_none(),
            Presence::Present => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = Presence::de_serialize(&mut self.reader)?;
        if value != Presence::Absent {
            return Err(LqError::new(
                "Trying to deserialize unit; expecting to have an absent value \
                 (but got a present value)",
            )
            .into());
        }
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    // the newtype is the same as its inner value.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let list_like = ListLike::unknown_length(self)?;
        visitor.visit_seq(list_like)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let list_like = ListLike::fixed_length(self, len)?;
        visitor.visit_seq(list_like)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let list_like = ListLike::fixed_length(self, len)?;
        visitor.visit_seq(list_like)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!("TODO")
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let list_like = ListLike::fixed_length(self, fields.len())?;
        visitor.visit_seq(list_like)        
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!("TODO")
    }

    fn deserialize_identifier<V>(self, _: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(LqError::new("This format has no identifiers. Operation not supported").into())
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.reader.skip()?;
        visitor.visit_unit()
    }
}

struct ListLike<'de, R : BinaryReader<'de>> {
    deserializer: &'de mut Deserializer<'de, R>,
    number_of_items_left: u32,
    items_to_skip: u32,
    _marker: &'de PhantomData<()>,
}

impl<'de, R: BinaryReader<'de>> ListLike<'de, R> {
    fn unknown_length(deserializer: &'de mut Deserializer<'de, R>) -> Result<Self> {
        let header = ListHeader::de_serialize(&mut deserializer.reader)?;
        Ok(Self {
            deserializer,
            number_of_items_left: header.length(),
            items_to_skip: 0,
            _marker: &PhantomData,
        })
    }

    fn fixed_length(deserializer: &'de mut Deserializer<'de, R>, length: usize) -> Result<Self> {
        let header = ListHeader::de_serialize(&mut deserializer.reader)?;
        let real_length = header.length();
        let u32_length = try_from_int_result(u32::try_from(length))?;
        if real_length < u32_length {
            return Err(LqError::new(format!(
                "Got a sequence and need at least {:?} items but only \
                 have {:?} items.",
                u32_length, real_length
            ))
            .into());
        }
        let items_to_skip = real_length - u32_length;

        Ok(Self {
            deserializer,
            number_of_items_left: header.length(),
            items_to_skip: items_to_skip,
            _marker: &PhantomData,
        })
    }
}

impl<'de, R: BinaryReader<'de>> SeqAccess<'de> for ListLike<'de, R> {
    type Error = SLqError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.number_of_items_left == 0 {
            // 99% case this is 0
            if self.items_to_skip > 0 {
                let usize_items_to_skip = try_from_int_result(usize::try_from(self.items_to_skip))?;
                self.deserializer.reader.skip_n_values(usize_items_to_skip)?;
            }
            return Ok(None);
        } else {            
            self.number_of_items_left = self.number_of_items_left - 1;
            seed.deserialize(self.deserializer).map(Some)
        }
    }
}

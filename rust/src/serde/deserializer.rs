use crate::common::error::LqError;
use crate::common::internal_utils::try_from_int_result;
use crate::serde::error::SLqError;
use crate::serialization::core::DeSerializer;
use crate::serialization::tbinary::TBinary;
use crate::serialization::tbool::TBool;
use crate::serialization::tenum::EnumHeader;
use crate::serialization::tlist::ListHeader;
use crate::serialization::toption::Presence;
use crate::serialization::tsint::{TSInt, TSInt16, TSInt32, TSInt8};
use crate::serialization::tuint::{TUInt, TUInt16, TUInt32, TUInt8};
use crate::serialization::tutf8::TUtf8;
use serde::de::IntoDeserializer;
use serde::de::Visitor;
use std::convert::TryFrom;
use std::marker::PhantomData;

use crate::serialization::core::BinaryReader;

pub struct Deserializer<'de, R: BinaryReader<'de>> {
    reader: R,
    _phantom: &'de PhantomData<()>,
}

pub fn new_deserializer<'de, R: BinaryReader<'de>>(reader: R) -> Deserializer<'de, R> {
    Deserializer {
        reader,
        _phantom: &PhantomData,
    }
}

type Result<Ok> = std::result::Result<Ok, SLqError>;

impl<'de, 'a, R> serde::Deserializer<'de> for &'a mut Deserializer<'de, R>
where
    R: BinaryReader<'de>,
{
    type Error = SLqError;

    fn deserialize_any<V>(self, _: V) -> Result<V::Value>
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
        self.deserialize_seq_like(Option::None, visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq_like(Option::Some(len), visitor)
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
        self.deserialize_seq_like(Option::Some(len), visitor)
    }

    /// A map is just a list of lists, like this: [[key1, value1][key2, value2]]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let list_header = ListHeader::de_serialize(&mut self.reader)?;
        let usize_list_header = try_from_int_result(usize::try_from(list_header.length()))?;

        visitor.visit_map(MapAccessStruct {
            deserializer: self,
            items_left: usize_list_header,
        })
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
        self.deserialize_seq_like(Option::Some(fields.len()), visitor)
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
        visitor.visit_enum(EnumAccessStruct {
            deserializer: self,
            input_data_len: 0,
        })
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

struct EnumAccessStruct<'a, 'de, R: BinaryReader<'de> + 'a> {
    deserializer: &'a mut Deserializer<'de, R>,
    input_data_len: usize,
}

impl<'de, 'a, 'b: 'a, R: BinaryReader<'de> + 'b> serde::de::EnumAccess<'de>
    for EnumAccessStruct<'a, 'de, R>
where
    R: BinaryReader<'de>,
{
    type Error = SLqError;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        let enum_header = EnumHeader::de_serialize(&mut self.deserializer.reader)?;
        let ordinal = enum_header.ordinal();
        let number_of_values = enum_header.number_of_values();
        self.input_data_len = try_from_int_result(usize::try_from(number_of_values))?;

        let val: std::result::Result<_, Self::Error> =
            seed.deserialize(ordinal.into_deserializer());
        Ok((val?, self))
    }
}

impl<'de, 'a, 'b: 'a, R: BinaryReader<'de> + 'b> serde::de::VariantAccess<'de>
    for EnumAccessStruct<'a, 'de, R>
{
    type Error = SLqError;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let value = serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;
        let to_skip = self.input_data_len - 1;
        if to_skip > 0 {
            self.deserializer.reader.skip_n_values(to_skip)?;
        }
        Ok(value)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserializer
            .deserialize_seq_no_header(Option::Some(len), self.input_data_len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let len = fields.len();
        self.deserializer
            .deserialize_seq_no_header(Option::Some(len), self.input_data_len, visitor)
    }
}

impl<'de, 'a, R> Deserializer<'de, R>
where
    R: BinaryReader<'de>,
{
    #[inline]
    fn deserialize_seq_like<V>(&'a mut self, len: Option<usize>, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // read length
        let list_header = ListHeader::de_serialize(&mut self.reader)?;
        let len_in_input_data = list_header.length();
        let usize_len_in_input_data = try_from_int_result(usize::try_from(len_in_input_data))?;

        self.deserialize_seq_no_header(len, usize_len_in_input_data, visitor)
    }

    #[inline]
    fn deserialize_seq_no_header<V>(
        &'a mut self,
        len: Option<usize>,
        real_len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let to_read = match len {
            Option::Some(given_len) => {
                if given_len > real_len {
                    real_len
                } else {
                    given_len
                }
            }
            Option::None => real_len,
        };

        let to_skip = real_len - to_read;

        visitor.visit_seq(SeqAccessStruct {
            deserializer: self,
            remaining_len: to_read,
            to_skip,
        })
    }
}

struct SeqAccessStruct<'a, 'de, R: BinaryReader<'de> + 'a> {
    deserializer: &'a mut Deserializer<'de, R>,
    remaining_len: usize,
    to_skip: usize,
}

impl<'de, 'a, 'b: 'a, R: BinaryReader<'de> + 'b> serde::de::SeqAccess<'de>
    for SeqAccessStruct<'a, 'de, R>
{
    type Error = SLqError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.remaining_len > 0 {
            self.remaining_len -= 1;
            let value = serde::de::DeserializeSeed::deserialize(seed, &mut *self.deserializer)?;

            // we skip here, since there's no guarantee we're called again by serde
            if self.remaining_len == 0 {
                if self.to_skip > 0 {
                    self.deserializer.reader.skip_n_values(self.to_skip)?;
                }
            }
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining_len)
    }
}

struct MapAccessStruct<'a, 'de, R: BinaryReader<'de> + 'a> {
    deserializer: &'a mut Deserializer<'de, R>,
    items_left: usize,
}

impl<'de, 'a, 'b: 'a, R: BinaryReader<'de> + 'b> serde::de::MapAccess<'de>
    for MapAccessStruct<'a, 'de, R>
{
    type Error = SLqError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.items_left > 0 {
            // Now a pair of key+value starts... so the list _always_ has to have a length of 2
            let list_header = ListHeader::de_serialize(&mut self.deserializer.reader)?;
            if list_header.length() != 2 {
                return Err(LqError::new(
                    format!("You're trying to deserialize a map. A map has to \
                be a list of entries; every entry has to be a list of 2 items (key and value). So \
                a map looks like this: [[key1; value1]; [key2; value2]; [key3; value3]; ...]. The \
                input list I got does not have 2 items (key and value) for an entry, it has \
                {:?} items.", list_header.length()),
                )
                .into());
            }

            self.items_left -= 1;
            Ok(Some(seed.deserialize(&mut *self.deserializer)?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        Ok(seed.deserialize(&mut *self.deserializer)?)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.items_left)
    }
}

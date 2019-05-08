use crate::common::error::LqError;
use crate::common::internal_utils::try_from_int_result;
use crate::serde::error::SLqError;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::Serializer as S;
use crate::serialization::tbinary::TBinary;
use crate::serialization::tenum::EnumHeader;
use crate::serialization::tfloat::TFloat32;
use crate::serialization::tfloat::TFloat64;
use crate::serialization::tlist::ListHeader;
use crate::serialization::toption::Presence;
use crate::serialization::tsint::{TSInt, TSInt16, TSInt32, TSInt8};
use crate::serialization::tuint::{TUInt, TUInt16, TUInt32, TUInt8};
use crate::serialization::tunicode::TUnicode;
use std::convert::TryFrom;

use serde::ser;

#[inline]
pub fn serialize<W: BinaryWriter, S: ser::Serialize>(
    writer: &mut W,
    value: S,
) -> std::result::Result<(), LqError> {
    let mut serializer = Serializer { writer };
    value.serialize(&mut serializer).map_err(|err| err.into())
}

pub struct Serializer<'a, W: BinaryWriter> {
    writer: &'a mut W,
}

type Result<Ok> = std::result::Result<Ok, SLqError>;

impl<'a, W: BinaryWriter> ser::Serializer for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = SLqError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        Ok(bool::serialize(self.writer, &v)?)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        Ok(TSInt8::serialize(self.writer, &v)?)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        Ok(TSInt16::serialize(self.writer, &v)?)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        Ok(TSInt32::serialize(self.writer, &v)?)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        Ok(TSInt::serialize(self.writer, &v)?)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        Ok(TUInt8::serialize(self.writer, &v)?)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        Ok(TUInt16::serialize(self.writer, &v)?)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        Ok(TUInt32::serialize(self.writer, &v)?)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        Ok(TUInt::serialize(self.writer, &v)?)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        Ok(TFloat32::serialize(self.writer, &v)?)        
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        Ok(TFloat64::serialize(self.writer, &v)?)
    }

    // Serialize as integer
    fn serialize_char(self, v: char) -> Result<()> {
        Ok(TUInt32::serialize(self.writer, &(v as u32))?)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        Ok(TUnicode::serialize(self.writer, v)?)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        Ok(TBinary::serialize(self.writer, v)?)
    }

    fn serialize_none(self) -> Result<()> {
        Ok(Presence::serialize(self.writer, &Presence::Absent)?)
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        Presence::serialize(self.writer, &Presence::Present)?;
        value.serialize(self)
    }

    // Same as optional absent
    fn serialize_unit(self) -> Result<()> {
        Ok(Presence::serialize(self.writer, &Presence::Absent)?)
    }

    // Same as optional absent
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(Presence::serialize(self.writer, &Presence::Absent)?)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        let enum_header = EnumHeader::new(variant_index, 0);
        Ok(EnumHeader::serialize(self.writer, &enum_header)?)
    }

    // The same as the contained type
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        let enum_header = EnumHeader::new(variant_index, 1);
        EnumHeader::serialize(self.writer, &enum_header)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let present_len: usize = len.ok_or(LqError::new(
            "Only supports sequences with computed length.",
        ))?;
        let u32_len = try_from_int_result(u32::try_from(present_len))?;
        let list_header = ListHeader::new(u32_len);
        ListHeader::serialize(self.writer, &list_header)?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        let u32_len = try_from_int_result(u32::try_from(len))?;
        let list_header = ListHeader::new(u32_len);
        ListHeader::serialize(self.writer, &list_header)?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let u32_len = try_from_int_result(u32::try_from(len))?;
        let list_header = ListHeader::new(u32_len);
        ListHeader::serialize(self.writer, &list_header)?;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let u32_len = try_from_int_result(u32::try_from(len))?;
        let enum_header = EnumHeader::new(variant_index, u32_len);
        EnumHeader::serialize(self.writer, &enum_header)?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let present_len: usize = len.ok_or(LqError::new(
            "Only supports sequences with computed length.",
        ))?;
        let u32_len = try_from_int_result(u32::try_from(present_len))?;
        let list_header = ListHeader::new(u32_len);
        ListHeader::serialize(self.writer, &list_header)?;
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        let u32_len = try_from_int_result(u32::try_from(len))?;
        let list_header = ListHeader::new(u32_len);
        ListHeader::serialize(self.writer, &list_header)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        let u32_len = try_from_int_result(u32::try_from(len))?;
        let enum_header = EnumHeader::new(variant_index, u32_len);
        EnumHeader::serialize(self.writer, &enum_header)?;
        Ok(self)
    }
}

impl<'a, W: BinaryWriter> ser::SerializeSeq for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = SLqError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.serialize_inner(value)
    }

    fn end(self) -> Result<()> {
        // nothing to do here
        Ok(())
    }
}

impl<'a, W: BinaryWriter> ser::SerializeTuple for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = SLqError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.serialize_inner(value)
    }

    fn end(self) -> Result<()> {
        // nothing to do here
        Ok(())
    }
}

impl<'a, W: BinaryWriter> ser::SerializeTupleStruct for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = SLqError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.serialize_inner(value)
    }

    fn end(self) -> Result<()> {
        // nothing to do here
        Ok(())
    }
}

impl<'a, W: BinaryWriter> ser::SerializeTupleVariant for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = SLqError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.serialize_inner(value)
    }

    fn end(self) -> Result<()> {
        // nothing to do here
        Ok(())
    }
}

impl<'a, W: BinaryWriter> ser::SerializeMap for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = SLqError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        // key-value is wrapped inside a list (length = 2).
        let list_header = ListHeader::new(2);
        ListHeader::serialize(self.writer, &list_header)?;
        self.serialize_inner(key)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.serialize_inner(value)
    }

    fn end(self) -> Result<()> {
        // nothing to do
        Ok(())
    }
}

impl<'a, W: BinaryWriter> ser::SerializeStruct for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = SLqError;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.serialize_inner(value)
    }

    fn end(self) -> Result<()> {
        // nothing to do here
        Ok(())
    }
}

impl<'a, W: BinaryWriter> ser::SerializeStructVariant for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = SLqError;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        self.serialize_inner(value)
    }

    fn end(self) -> Result<()> {
        // nothing to do here
        Ok(())
    }
}

impl<'a, W: BinaryWriter> Serializer<'a, W> {
    #[inline]
    fn serialize_inner<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        let mut new_self = Serializer {
            writer: self.writer,
        };
        value.serialize(&mut new_self)
    }
}

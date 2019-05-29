use crate::binary::Binary;
use crate::boolean::Bool;
use crate::core::LqWriter;
use crate::core::Serializer as S;
use crate::enumeration::EnumHeader;
use crate::float::Float32;
use crate::float::Float64;
use crate::option::Presence;
use crate::seq::SeqHeader;
use crate::serde::error::SLqError;
use crate::sint::{SInt16, SInt32, SInt64, SInt8};
use crate::uint::{UInt16, UInt32, UInt64, UInt8};
use crate::unicode::Unicode;
use liquesco_common::error::LqError;
use std::convert::TryFrom;

use serde::ser;

pub(crate) struct Serializer<'a, W: LqWriter> {
    writer: &'a mut W,
}

impl<'a, W: LqWriter>  Serializer<'a, W> {
    pub(crate) fn new(writer : &'a mut W) -> Self {
        Self {
            writer
        }
    }
}

type Result<Ok> = std::result::Result<Ok, SLqError>;

impl<'a, W: LqWriter> ser::Serializer for &'a mut Serializer<'a, W> {
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
        Ok(Bool::serialize(self.writer, &v)?)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        Ok(SInt8::serialize(self.writer, &v)?)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        Ok(SInt16::serialize(self.writer, &v)?)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        Ok(SInt32::serialize(self.writer, &v)?)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        Ok(SInt64::serialize(self.writer, &v)?)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        Ok(UInt8::serialize(self.writer, &v)?)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        Ok(UInt16::serialize(self.writer, &v)?)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        Ok(UInt32::serialize(self.writer, &v)?)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        Ok(UInt64::serialize(self.writer, &v)?)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        Ok(Float32::serialize(self.writer, &v)?)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        Ok(Float64::serialize(self.writer, &v)?)
    }

    // Serialize as integer
    fn serialize_char(self, v: char) -> Result<()> {
        Ok(UInt32::serialize(self.writer, &(v as u32))?)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        Ok(Unicode::serialize(self.writer, v)?)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        Ok(Binary::serialize(self.writer, v)?)
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
        let u32_len = u32::try_from(present_len)?;
        let list_header = SeqHeader::new(u32_len);
        SeqHeader::serialize(self.writer, &list_header)?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        let u32_len = u32::try_from(len)?;
        let list_header = SeqHeader::new(u32_len);
        SeqHeader::serialize(self.writer, &list_header)?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let u32_len = u32::try_from(len)?;
        let list_header = SeqHeader::new(u32_len);
        SeqHeader::serialize(self.writer, &list_header)?;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let u32_len = u32::try_from(len)?;
        let enum_header = EnumHeader::new(variant_index, u32_len);
        EnumHeader::serialize(self.writer, &enum_header)?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        let present_len: usize = len.ok_or(LqError::new(
            "Only supports maps with computed length. This can happen with structs #flatten.",
        ))?;
        let u32_len = u32::try_from(present_len)?;
        let list_header = SeqHeader::new(u32_len);
        SeqHeader::serialize(self.writer, &list_header)?;
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        let u32_len = u32::try_from(len)?;
        let list_header = SeqHeader::new(u32_len);
        SeqHeader::serialize(self.writer, &list_header)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        let u32_len = u32::try_from(len)?;
        let enum_header = EnumHeader::new(variant_index, u32_len);
        EnumHeader::serialize(self.writer, &enum_header)?;
        Ok(self)
    }
}

impl<'a, W: LqWriter> ser::SerializeSeq for &'a mut Serializer<'a, W> {
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

impl<'a, W: LqWriter> ser::SerializeTuple for &'a mut Serializer<'a, W> {
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

impl<'a, W: LqWriter> ser::SerializeTupleStruct for &'a mut Serializer<'a, W> {
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

impl<'a, W: LqWriter> ser::SerializeTupleVariant for &'a mut Serializer<'a, W> {
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

impl<'a, W: LqWriter> ser::SerializeMap for &'a mut Serializer<'a, W> {
    type Ok = ();
    type Error = SLqError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + ser::Serialize,
    {
        // key-value is wrapped inside a list (length = 2).
        let list_header = SeqHeader::new(2);
        SeqHeader::serialize(self.writer, &list_header)?;
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

impl<'a, W: LqWriter> ser::SerializeStruct for &'a mut Serializer<'a, W> {
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

impl<'a, W: LqWriter> ser::SerializeStructVariant for &'a mut Serializer<'a, W> {
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

impl<'a, W: LqWriter> Serializer<'a, W> {
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

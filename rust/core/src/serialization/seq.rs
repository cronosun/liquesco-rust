use crate::serialization::major_types::TYPE_SEQ;

use crate::common::error::LqError;
use crate::serialization::core::ContentDescription;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LqReader;
use crate::serialization::core::LqWriter;
use crate::serialization::core::Serializer;
use std::convert::TryFrom;

pub struct SeqHeader {
    length: u32,
}

impl SeqHeader {
    pub fn new(length: u32) -> Self {
        SeqHeader { length }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn begin(&self, wanted_number_of_items: u32) -> Result<SeqRead, LqError> {
        if wanted_number_of_items < self.length {
            LqError::err_new(format!(
                "Expecting to have a struct with at least {:?} fields; 
            have {:?} fields.",
                wanted_number_of_items, self.length
            ))
        } else {
            Result::Ok(SeqRead {
                actual_number_of_items: self.length,
                wanted_number_of_items,
            })
        }
    }

    /// Calls `begin`, reads the list (struct) (see `function`) and then calls `finish`.
    pub fn read_struct<'a, Ret, R: LqReader<'a>, ReadFn: FnOnce(&mut R) -> Result<Ret, LqError>>(
        &self,
        reader: &mut R,
        number_of_fields: u32,
        function: ReadFn,
    ) -> Result<Ret, LqError> {
        let list_reader = self.begin(number_of_fields)?;
        let result = function(reader)?;
        list_reader.finish(reader)?;
        Result::Ok(result)
    }
}

pub struct SeqRead {
    actual_number_of_items: u32,
    wanted_number_of_items: u32,
}

impl SeqRead {
    pub fn finish<'a, R: LqReader<'a>>(self, reader: &mut R) -> Result<(), LqError> {
        let fields_to_skip = self.actual_number_of_items - self.wanted_number_of_items;
        reader.skip_n_values(usize::try_from(fields_to_skip)?)
    }
}

impl<'a> DeSerializer<'a> for SeqHeader {
    type Item = Self;

    fn de_serialize<Reader: LqReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let type_header = reader.read_type_header()?;
        if type_header.major_type() != TYPE_SEQ {
            return LqError::err_new(format!(
                "Got something that's not a sequence (major type \
                 {:?}). Got major type {:?}.",
                TYPE_SEQ,
                type_header.major_type()
            ));
        }
        let content_description = reader.read_content_description_given_type_header(type_header)?;
        if content_description.self_length() != 0 {
            return LqError::err_new(format!(
                "Lists always have a self length of 0. This 'list' has a self 
            length of {:?}",
                content_description.self_length()
            ));
        }
        Result::Ok(Self {
            length: content_description.number_of_embedded_values(),
        })
    }
}

impl<'a> Serializer for SeqHeader {
    type Item = Self;

    fn serialize<T: LqWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        writer.write_content_description(
            TYPE_SEQ,
            &ContentDescription::new_number_of_embedded_values(item.length),
        )
    }
}

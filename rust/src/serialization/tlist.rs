use crate::serialization::major_types::TYPE_LIST;

use crate::common::error::LqError;
use crate::common::internal_utils::try_from_int_result;
use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::ContentDescription;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::Serializer;
use std::convert::TryFrom;

pub struct ListHeader {
    length: u32,
}

impl ListHeader {
    pub fn new(length: u32) -> Self {
        ListHeader { length }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn begin(&self, wanted_number_of_items: u32) -> Result<ListRead, LqError> {
        if wanted_number_of_items < self.length {
            LqError::err_new(format!(
                "Expecting to have a struct with at least {:?} fields; 
            have {:?} fields.",
                wanted_number_of_items, self.length
            ))
        } else {
            Result::Ok(ListRead {
                actual_number_of_items: self.length,
                wanted_number_of_items,
            })
        }
    }

    /// Calls `begin`, reads the list (struct) (see `function`) and then calls `finish`.
    pub fn read_struct<
        'a,
        Ret,
        R: BinaryReader<'a>,
        ReadFn: FnOnce(&mut R) -> Result<Ret, LqError>,
    >(
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

pub struct ListRead {
    actual_number_of_items: u32,
    wanted_number_of_items: u32,
}

impl ListRead {
    pub fn finish<'a, R: BinaryReader<'a>>(self, reader: &mut R) -> Result<(), LqError> {
        let fields_to_skip = self.actual_number_of_items - self.wanted_number_of_items;
        reader.skip_n_values(try_from_int_result(usize::try_from(fields_to_skip))?)        
    }
}

impl<'a> DeSerializer<'a> for ListHeader {
    type Item = Self;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let type_header = reader.read_type_header()?;
        if type_header.major_type() != TYPE_LIST {
            return LqError::err_static("Not a list type");
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

impl<'a> Serializer for ListHeader {
    type Item = Self;

    fn serialize<T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        writer.write_content_description(
            TYPE_LIST,
            &ContentDescription::new_number_of_embedded_values(item.length),
        )
    }
}

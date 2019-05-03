use crate::serialization::type_ids::TYPE_LIST;

use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::ContainerHeader;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::LengthMarker;
use crate::common::error::LqError;
use crate::serialization::core::Serializer;

pub struct ListHeader {
    length: usize,
}

impl ListHeader {
    pub fn new(length: usize) -> Self {
        ListHeader { length }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn begin(&self, wanted_number_of_items: usize) -> Result<ListRead, LqError> {
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
    pub fn read_struct<'a, TResult, T: BinaryReader<'a>, ReadFn: FnOnce(&mut T)
        -> Result<TResult, LqError>>(
        &self,
        reader: &mut T,
        number_of_fields: usize,
        function: ReadFn) -> Result<TResult, LqError> {
        let list_reader = self.begin(number_of_fields)?;
        let result = function(reader)?;
        list_reader.finish(reader)?;
        Result::Ok(result)
    }
}

pub struct ListRead {
    actual_number_of_items: usize,
    wanted_number_of_items: usize,
}

impl ListRead {
    pub fn finish<'a, T: BinaryReader<'a>>(self, reader: &mut T) -> Result<(), LqError> {
        let fields_to_skip = self.actual_number_of_items - self.wanted_number_of_items;
        if fields_to_skip > 0 {
            for _ in 0..fields_to_skip {
                reader.skip()?;
            }
            Result::Ok(())
        } else {
            Result::Ok(())
        }
    }
}

impl<'a> DeSerializer<'a> for ListHeader {
    type Item = Self;

    fn de_serialize<Reader: BinaryReader<'a>>(reader: &mut Reader) -> Result<Self::Item, LqError> {
        let header = reader.read_header()?;
        // special case for zero sized lists
        if header.length_marker() == LengthMarker::Len0 {
            Result::Ok(Self { length: 0 })
        } else {
            // if it's not zero sized, it's a container
            let container_info = reader.read_header_container(header)?;
            if header.type_id() != TYPE_LIST {
                return LqError::err_static("Not a struct type");
            }
            if container_info.self_length() != 0 {
                return LqError::err_static("Invalid encoding; length of struct must be 0.");
            }
            Result::Ok(Self {
                length: container_info.number_of_items(),
            })
        }
    }
}

impl<'a> Serializer for ListHeader {
    type Item = Self;

    fn serialize<T: BinaryWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        let length = item.length;
        // special case for zero size
        if length == 0 {
            writer.write_header_u8(TYPE_LIST, 0)
        } else {
            let header = ContainerHeader::new(length, 0);
            writer.write_container_header(TYPE_LIST, header)
        }
    }
}

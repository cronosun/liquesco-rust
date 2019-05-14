use crate::common::error::LqError;
use crate::serialization::core::DeSerializer;
use crate::serialization::core::{LqReader, LqWriter, Serializer};
use crate::serialization::seq::SeqHeader;
use crate::serialization::unicode::Unicode;
use core::convert::TryFrom;
use smallvec::SmallVec;
use std::borrow::Cow;
use std::ops::Deref;

const SEGMENT_MIN_LEN: usize = 1;
const SEGMENT_MAX_LEN: usize = 30;
const MIN_NUMBER_OF_SEGMENTS: usize = 1;
const MAX_NUMBER_OF_SEGMENTS: usize = 12;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Segment<'a>(Cow<'a, str>);

/// We embed identifiers with 1-3 segments (since that covers 95% of all cases).
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Identifier<'a>(SmallVec<[Segment<'a>; 3]>);

impl<'a> Deref for Identifier<'a> {
    type Target = [Segment<'a>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> Deref for Segment<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Identifier<'a> {
    pub fn segments(&self) -> &[Segment<'a>] {
        &self.0.as_slice()
    }

    pub fn to_string(&self, format: Format) -> String {
        match format {
            Format::SnakeCase => self
                .segments()
                .iter()
                .map(|segment| {
                    let string: &str = &segment.0;
                    string
                })
                .collect::<Vec<&str>>()
                .join("_"),
        }
    }
}

pub enum Format {
    SnakeCase,
}

impl<'a> TryFrom<&'a str> for Identifier<'a> {
    type Error = LqError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let splits = value.split("_");
        let mut segments = SmallVec::<[Segment<'a>; 3]>::new();
        for split in splits {
            segments.push(Segment::try_from(split)?);
        }
        let number_of_segments = segments.len();
        Identifier::validate_number_of_segments(number_of_segments)?;
        Result::Ok(Identifier(segments))
    }
}

impl<'a> TryFrom<&'a str> for Segment<'a> {
    type Error = LqError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Segment::validate(value)?;
        Result::Ok(Segment(Cow::Borrowed(value)))
    }
}

impl<'a> Segment<'a> {
    fn validate(value: &str) -> Result<(), LqError> {
        let num_bytes = value.len();
        if num_bytes < SEGMENT_MIN_LEN {
            return LqError::err_new(format!(
                "Segment in identifier is too short (min {:?}), got: {:?}",
                SEGMENT_MIN_LEN, value
            ));
        } else if num_bytes > SEGMENT_MAX_LEN {
            return LqError::err_new(format!(
                "Segment in identifier is too long (max {:?}), got: {:?}",
                SEGMENT_MAX_LEN, value
            ));
        }
        // iterating bytes is OK, since we only accept ASCII anyway
        for byte_char in value.bytes() {
            let is_valid =
                (byte_char >= 97 && byte_char <= 122) || (byte_char >= 48 && byte_char <= 57);
            if !is_valid {
                return LqError::err_new(format!(
                    "The given segment in identifier is not valid. Only supports ASCII a-z \
                     (lower case) and 0-9; got: {:?}",
                    value
                ));
            }
        }
        Result::Ok(())
    }
}

impl<'a> Identifier<'a> {
    fn validate_number_of_segments(number: usize) -> Result<(), LqError> {
        if number < MIN_NUMBER_OF_SEGMENTS {
            LqError::err_new(format!(
                "An identifier needs at least {:?} segment(s); \
                 got {:?} segments",
                MIN_NUMBER_OF_SEGMENTS, number
            ))
        } else if number > MAX_NUMBER_OF_SEGMENTS {
            LqError::err_new(format!(
                "An identifier can have at max {:?} segments; \
                 got {:?} segments",
                MAX_NUMBER_OF_SEGMENTS, number
            ))
        } else {
            Result::Ok(())
        }
    }
}

impl<'a> DeSerializer<'a> for Identifier<'a> {
    type Item = Self;

    fn de_serialize<T: LqReader<'a>>(reader: &mut T) -> Result<Self::Item, LqError> {
        let list_header = SeqHeader::de_serialize(reader)?;
        let number_of_segments = list_header.length();
        let usize_number_of_segments = usize::try_from(number_of_segments)?;
        Identifier::validate_number_of_segments(usize_number_of_segments)?;

        let mut segments = SmallVec::<[Segment<'a>; 3]>::with_capacity(usize_number_of_segments);
        for _ in 0..number_of_segments {
            let segment_str = Unicode::de_serialize(reader)?;
            segments.push(Segment::try_from(segment_str)?);
        }
        Result::Ok(Identifier(segments))
    }
}

impl<'a> Serializer for Identifier<'a> {
    type Item = Self;

    fn serialize<T: LqWriter>(writer: &mut T, item: &Self::Item) -> Result<(), LqError> {
        let number_of_segments = item.len();
        let u32_number_of_segments = u32::try_from(number_of_segments)?;
        let list_header = SeqHeader::new(u32_number_of_segments);
        SeqHeader::serialize(writer, &list_header)?;

        for segment in item.segments() {
            Unicode::serialize(writer, &segment)?;
        }
        Result::Ok(())
    }
}

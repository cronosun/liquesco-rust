use crate::ascii::{CodeRange, TAscii};
use crate::core::TypeRef;
use crate::doc_type::DocType;
use crate::schema_builder::{BuildsOwnSchema, SchemaBuilder};
use crate::seq::TSeq;
use core::convert::TryFrom;
use liquesco_common::error::LqError;
use liquesco_common::ine_range::U64IneRange;
use liquesco_serialization::core::DeSerializer;
use liquesco_serialization::core::{LqReader, LqWriter, Serializer};
use liquesco_serialization::seq::SeqHeader;
use liquesco_serialization::unicode::Unicode;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;

const SEGMENT_MIN_LEN: usize = 1;
const SEGMENT_MAX_LEN: usize = 30;
const MIN_NUMBER_OF_SEGMENTS: usize = 1;
const MAX_NUMBER_OF_SEGMENTS: usize = 12;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Segment<'a>(Cow<'a, str>);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Identifier<'a>(Vec<Segment<'a>>);

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

    // TODO: What is this used for, we already have Eq?
    pub fn is_equal<'b>(&self, other: &Identifier<'b>) -> bool {
        let len = self.0.len();
        if len == other.0.len() {
            for index in 0..len {
                if !self.0[index].is_equal(&other.0[index]) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn append(&mut self, segment: Segment<'a>) -> Result<(), LqError> {
        if self.segments().len() + 1 > MAX_NUMBER_OF_SEGMENTS {
            LqError::err_new(format!(
                "Cannot add another segment to identifier {:?}. Max number of segments reached.",
                self
            ))
        } else {
            self.0.push(segment);
            Ok(())
        }
    }

    pub fn into_owned(self) -> Identifier<'static> {
        let mut new_segments = Vec::with_capacity(self.0.len());
        for segment in self.0 {
            new_segments.push(segment.into_owned());
        }
        Identifier(new_segments)
    }
}

impl BuildsOwnSchema for Identifier<'_> {
    fn build_schema<B>(builder: &mut B) -> TypeRef
    where
        B: SchemaBuilder,
    {
        let mut code_range = CodeRange::try_new(48, 57 + 1).unwrap();
        code_range.add(97, 122 + 1).unwrap();
        let segment_ref = builder.add(
            DocType::from(TAscii {
                length: U64IneRange::try_new(SEGMENT_MIN_LEN as u64, SEGMENT_MAX_LEN as u64)
                    .unwrap(),
                codes: code_range,
            })
            .with_name_unwrap("segment")
            .with_description(
                "A single segment of an identifier. \
                 An identifier can only contain certain ASCII characters and is limited in length.",
            ),
        );
        builder.add(
            DocType::from(
                TSeq::try_new(
                    segment_ref,
                    MIN_NUMBER_OF_SEGMENTS as u32,
                    MAX_NUMBER_OF_SEGMENTS as u32,
                )
                .unwrap(),
            )
            .with_name_unwrap("identifier")
            .with_description(format!(
                "An identifier identifies something in the system. An \
                 identifier is composed of {min}-{max} segments. Each segment is composed of ASCII \
                 characters (see segment for details what characters are allowed and about min/max \
                 length). These strict constraints allow simple conversions of identifiers to \
                 identifiers of the target system (e.g. Java class names, Rust trait names, Dart \
                 class names, ...).",
                min = MIN_NUMBER_OF_SEGMENTS,
                max = MAX_NUMBER_OF_SEGMENTS
            )),
        )
    }
}

pub enum Format {
    SnakeCase,
}

impl<'a> TryFrom<&'a str> for Identifier<'a> {
    type Error = LqError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let splits = value.split("_");
        let mut segments = Vec::new();
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

impl<'a> TryFrom<String> for Segment<'a> {
    type Error = LqError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Segment::validate(&value)?;
        Result::Ok(Segment(Cow::Owned(value)))
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

    // TODO: WHat is this used for, there's already Eq?
    pub fn is_equal<'b>(&self, other: &Segment<'b>) -> bool {
        self.0 == other.0
    }

    pub fn into_owned(self) -> Segment<'static> {
        match self.0 {
            Cow::Owned(item) => Segment(Cow::Owned(item)),
            Cow::Borrowed(item) => Segment(Cow::Owned(item.to_string())),
        }
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
        let mut segments = Vec::with_capacity(usize_number_of_segments);
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

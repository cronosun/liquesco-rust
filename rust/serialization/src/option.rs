use crate::core::ContentDescription;
use crate::core::DeSerializer;
use crate::core::LqReader;
use crate::core::LqWriter;
use crate::core::Serializer;
use crate::major_types::TYPE_OPTION;
use liquesco_common::error::LqError;

/// Can either be:
///  - Absent: In this case there's no embedded item.
///  - Present: In this case has exactly one embedded item.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Presence {
    /// Absent: No value - no embedded item.
    Absent,
    /// Present: Has exactly one embedded item.
    Present,
}

impl<'a> DeSerializer<'a> for Presence {
    type Item = Self;

    fn de_serialize<R: LqReader<'a>>(reader: &mut R) -> Result<Self::Item, LqError> {
        let type_header = reader.read_header_byte()?;
        let content_description = reader.read_content_description_given_header_byte(type_header)?;

        if type_header.major_type() != TYPE_OPTION {
            return LqError::err_static("Given type is not the option type");
        }
        if content_description.self_length() != 0 {
            return LqError::err_new(format!(
                "Option types must have a self length of 0 (this value has a self 
            length of {:?})",
                content_description.self_length()
            ));
        }

        match content_description.number_of_embedded_items() {
            0 => Result::Ok(Presence::Absent),
            1 => Result::Ok(Presence::Present),
            n => LqError::err_new(format!(
                "Invalid option type (option types need to have 0 or 1 
            embedded item(s)). This value has {:?} embedded items.",
                n
            )),
        }
    }
}

impl Serializer for Presence {
    type Item = Self;

    fn serialize<W: LqWriter>(writer: &mut W, item: &Self::Item) -> Result<(), LqError> {
        match item {
            Presence::Present => writer.write_content_description(
                TYPE_OPTION,
                &ContentDescription::new_number_of_embedded_values(1),
            ),
            Presence::Absent => {
                writer.write_content_description(TYPE_OPTION, &ContentDescription::default())
            }
        }
    }
}

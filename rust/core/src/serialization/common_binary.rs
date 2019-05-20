use crate::common::error::LqError;
use crate::serialization::core::ContentDescription;
use crate::serialization::core::LqReader;
use crate::serialization::core::LqWriter;
use crate::serialization::core::MajorType;
use std::convert::TryFrom;

#[inline]
pub fn binary_write<W: LqWriter>(
    data: &[u8],
    writer: &mut W,
    major_type: MajorType,
) -> Result<(), LqError> {
    let bin_len = data.len();
    let bin_len_as_u64 = u64::try_from(bin_len)?;
    let mut content_description = ContentDescription::default();
    content_description.set_self_length(bin_len_as_u64);
    writer.write_content_description(major_type, &content_description)?;
    writer.write(data)?;
    Result::Ok(())
}

#[inline]
pub fn binary_read<'a, R: LqReader<'a>>(reader: &mut R) -> Result<(MajorType, &'a [u8]), LqError> {
    let header = reader.read_type_header()?;
    let content_description = reader.read_content_description_given_type_header(header)?;
    let len = content_description.self_length();
    // binaries can never contain embedded values
    if content_description.number_of_embedded_values() != 0 {
        return LqError::err_new(format!(
            "Binary types can never contain embedded values. Got {:?} \
             embedded values. Major type {:?}.",
            content_description.number_of_embedded_values(),
            header.major_type()
        ));
    }

    let usize_len = usize::try_from(len)?;
    let read_result = reader.read_slice(usize_len)?;
    Result::Ok((header.major_type(), read_result))
}

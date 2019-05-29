use crate::core::ContentDescription;
use crate::core::LqReader;
use crate::core::LqWriter;
use crate::core::MajorType;
use liquesco_common::error::LqError;
use std::convert::TryFrom;

#[inline]
pub(crate) fn binary_write<W: LqWriter>(
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
pub(crate) fn binary_read<'a, R: LqReader<'a>>(
    reader: &mut R,
) -> Result<(MajorType, &'a [u8]), LqError> {
    let header = reader.read_header_byte()?;
    let content_description = reader.read_content_description_given_header_byte(header)?;
    let len = content_description.self_length();
    // binaries can never contain embedded values
    if content_description.number_of_embedded_items() != 0 {
        return LqError::err_new(format!(
            "Binary types can never contain embedded values. Got {:?} \
             embedded values. Major type {:?}.",
            content_description.number_of_embedded_items(),
            header.major_type()
        ));
    }

    let usize_len = usize::try_from(len)?;
    let read_result = reader.read_slice(usize_len)?;
    Result::Ok((header.major_type(), read_result))
}

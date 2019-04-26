use crate::serialization::core::BinaryReader;
use crate::serialization::core::LqError;
use crate::serialization::core::SkipMore;
use crate::serialization::types::Types;

pub fn skip_all<'a, T: BinaryReader<'a>>(reader: &mut T, count: usize) -> Result<(), LqError> {
        for _ in 0..count {
                let skip_more = skip_single(reader)?;
                let number_of_additional_items = skip_more.number_of_additional_items();
                if number_of_additional_items > 0 {
                        skip_all(reader, number_of_additional_items)?
                }
        }
        Result::Ok(())
}

fn skip_single<'a, T: BinaryReader<'a>>(reader: &mut T) -> Result<SkipMore, LqError> {
        let type_id = reader.preview_type_id()?;
        let typ = Types::from_id(type_id)?;
        typ.skip(reader)
}

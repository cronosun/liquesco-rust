use crate::serialization::core::BinaryReader;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeId;
use crate::serialization::util::io_result;
use byteorder::ByteOrder;
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Write;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BlockId(pub u8);

const U24_MAX: usize = 16777215;

impl TypeId {
    fn extract_binary_info(&self) -> BinaryInfo {
        let block = BlockId((self.0 & 0xF0) / 16);
        let data = self.0 & 0x0F;

        extract(data, block)
    }

    fn from_data(block: BlockId, data: u8) -> TypeId {
        TypeId(block.0 * 16u8 + data)
    }
}

#[inline]
pub(crate) fn binary_write<'a, Writer: BinaryWriter<'a> + 'a>(
    data: &[u8],
    writer: Writer,
    block: BlockId,
) -> Result<(), LqError> {
    let bin_len = data.len();
    if bin_len > std::u32::MAX as usize {
        return LqError::err_new(format!(
            "Given binary is too long (max u32). Got {:?}",
            bin_len
        ));
    }

    let (type_id, length_type) = type_id(block, bin_len);
    let writer = writer.begin(type_id)?;

    match length_type {
        LengthType::U8 => {
            io_result(writer.write(&[bin_len as u8]))?;
        }
        LengthType::U16 => {
            io_result(writer.write_u16::<LittleEndian>(bin_len as u16))?;
        }
        LengthType::U24 => {
            io_result(writer.write_u24::<LittleEndian>(bin_len as u32))?;
        }
        LengthType::U32 => {
            io_result(writer.write_u32::<LittleEndian>(bin_len as u32))?;
        }
        LengthType::Embedded => {}
    }

    io_result(writer.write(data))?;
    Result::Ok(())
}

#[inline]
pub(crate) fn binary_read<'a, Reader: BinaryReader<'a>>(
    type_id: TypeId,
    reader: &mut Reader,
) -> Result<(BlockId, &'a [u8]), LqError> {
    let bin_info = type_id.extract_binary_info();
    let (block, len) = match bin_info {
        BinaryInfo::Invalid => return LqError::err_static("Invalid type id for binary data"),
        BinaryInfo::WithLength((block, length)) => (block, length as usize),
        //TODO: Add checks to make sure length is canonical (e.g. len=3 is always embedded)
        BinaryInfo::U8Length(block) => (block, reader.read_u8()? as usize),
        BinaryInfo::U16Length(block) => {
            let sliced_len = reader.read_slice(2)?;
            (block, LittleEndian::read_u16(sliced_len) as usize)
        }
        BinaryInfo::U24Length(block) => {
            let sliced_len = reader.read_slice(3)?;
            (block, LittleEndian::read_u24(sliced_len) as usize)
        }
        BinaryInfo::U32Length(block) => {
            let sliced_len = reader.read_slice(4)?;
            (block, LittleEndian::read_u32(sliced_len) as usize)
        }
    };
    let read_result = reader.read_slice(len)?;
    Result::Ok((block, read_result))
}

enum BinaryInfo {
    Invalid,
    WithLength((BlockId, u8)),
    U8Length(BlockId),
    U16Length(BlockId),
    U24Length(BlockId),
    U32Length(BlockId),
}

#[inline]
fn type_id(block: BlockId, len: usize) -> (TypeId, LengthType) {
    match len {
        0 => (TypeId::from_data(block, 0), LengthType::Embedded),
        1 => (TypeId::from_data(block, 1), LengthType::Embedded),
        2 => (TypeId::from_data(block, 2), LengthType::Embedded),
        3 => (TypeId::from_data(block, 3), LengthType::Embedded),
        4 => (TypeId::from_data(block, 4), LengthType::Embedded),
        5 => (TypeId::from_data(block, 5), LengthType::Embedded),
        6 => (TypeId::from_data(block, 6), LengthType::Embedded),
        7 => (TypeId::from_data(block, 7), LengthType::Embedded),
        8 => (TypeId::from_data(block, 8), LengthType::Embedded),
        16 => (TypeId::from_data(block, 9), LengthType::Embedded),
        32 => (TypeId::from_data(block, 10), LengthType::Embedded),
        _ => {
            // other size
            if len <= std::u8::MAX as usize {
                (TypeId::from_data(block, 11), LengthType::U8)
            } else if len <= std::u16::MAX as usize {
                (TypeId::from_data(block, 12), LengthType::U16)
            } else if len <= U24_MAX {
                (TypeId::from_data(block, 13), LengthType::U24)
            } else if len <= std::u32::MAX as usize {
                (TypeId::from_data(block, 15), LengthType::U32)
            } else {
                panic!("Given binary is too large")
            }
        }
    }
}

enum LengthType {
    Embedded,
    U8,
    U16,
    U24,
    U32,
}

#[inline]
fn extract(data: u8, block: BlockId) -> BinaryInfo {
    match data {
        0 => BinaryInfo::WithLength((block, 0)),
        1 => BinaryInfo::WithLength((block, 1)),
        2 => BinaryInfo::WithLength((block, 2)),
        3 => BinaryInfo::WithLength((block, 3)),
        4 => BinaryInfo::WithLength((block, 4)),
        5 => BinaryInfo::WithLength((block, 5)),
        6 => BinaryInfo::WithLength((block, 6)),
        7 => BinaryInfo::WithLength((block, 7)),
        8 => BinaryInfo::WithLength((block, 8)),
        9 => BinaryInfo::WithLength((block, 16)),
        10 => BinaryInfo::WithLength((block, 32)),
        11 => BinaryInfo::U8Length(block),
        12 => BinaryInfo::U16Length(block),
        13 => BinaryInfo::U24Length(block),
        14 => BinaryInfo::U32Length(block),
        // the 15 is reserved for further extensions
        15 => BinaryInfo::Invalid,
        _ => BinaryInfo::Invalid,
    }
}

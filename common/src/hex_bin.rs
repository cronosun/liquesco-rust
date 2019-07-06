use crate::error::LqError;
use std::convert::TryFrom;
use std::num::NonZeroU32;

// TODO: Currently unused

pub struct HexBin {}

impl HexBin {
    pub fn as_vec(from: &str, relaxed: bool) -> Result<Vec<u8>, LqError> {
        let expected_len = from.as_bytes().len() / 2 + 1;
        let mut vec = Vec::with_capacity(expected_len);
        Self::read_to_vec(from, relaxed, &mut vec)?;
        Ok(vec)
    }

    pub fn read_to_vec(from: &str, relaxed: bool, into: &mut Vec<u8>) -> Result<(), LqError> {
        let str_bytes = from.as_bytes();

        let mut has_previous = false;
        let mut previous = 0u8;

        for byte in str_bytes {
            let char_byte = *byte as char;
            let value = match char_byte {
                '0'...'9' => Some(byte - 48),
                'a'...'f' => Some(byte - 97 + 10),
                'A'...'F' => Some(byte - 65 + 10),
                _ => None,
            };

            if let Some(value) = value {
                if has_previous {
                    into.push(previous * 16 + value);
                    has_previous = false;
                } else {
                    previous = value;
                    has_previous = true;
                }
            } else {
                if !relaxed {
                    return Err(LqError::new(format!(
                        "Got invalid character when parsing hex \
                         binary: {}; this character is invalid when not inrelaxed mode.",
                        char_byte
                    )));
                }
            }
        }

        if has_previous {
            return Err(LqError::new(format!(
                "Got imblanced number of hex codes: To create one byte need 2 characters 
                        - so number of characters must be a multipe of 2 (it's not). 
                        The remaining value is {}.",
                previous
            )));
        }

        Ok(())
    }

    pub fn add_to_string(string: &mut String, bin: &[u8], block_len: Option<NonZeroU32>) {
        let mut current_block_len = 0u32;
        for byte in bin {
            // need to begin a new block?
            if let Some(block_len) = block_len {
                if current_block_len >= block_len.get() {
                    string.push(' ');
                    current_block_len = 0;
                }

                current_block_len += 1;
            }

            let hi = (byte & 0xF0) >> 4;
            let lo = byte & 0x0F;

            string.push(to_char(hi));
            string.push(to_char(lo));
        }
    }

    pub fn to_string(bin: &[u8], str_block_len: Option<NonZeroU32>) -> String {
        let expected_len = if let Some(block_len) = str_block_len {
            let len_without_spacing = bin.len() * 2;
            let number_of_blocks =
                len_without_spacing / usize::try_from(block_len.get() * 2).unwrap();
            len_without_spacing + number_of_blocks + 1
        } else {
            bin.len() * 2
        };
        let mut string = String::with_capacity(expected_len);
        Self::add_to_string(&mut string, bin, str_block_len);
        string
    }
}

fn to_char(value: u8) -> char {
    if value < 10 {
        (value + 48) as char
    } else {
        (value - 10 + 97) as char
    }
}

#[cfg(test)]
mod test {

    use crate::hex_bin::HexBin;
    use std::num::NonZeroU32;

    #[test]
    fn to_string() {
        let binary = b"aaabbbcccddd000::--7891011";
        let string = HexBin::to_string(binary, Some(NonZeroU32::new(4).unwrap()));
        assert_eq!(
            string,
            "61616162 62626363 63646464 3030303a 3a2d2d37 38393130 3131"
        );
    }

    #[test]
    fn bin1() {
        let binary = b"aaabbbcccddd000::--7891011";
        assert_to_string_from_string(binary);
    }

    #[test]
    fn bin2() {
        let binary = b"This is some arbitrary text\0uFF\0u00and some more";
        assert_to_string_from_string(binary);
    }

    #[test]
    fn bin3_empty() {
        let binary = b"";
        assert_to_string_from_string(binary);
    }

    fn assert_to_string_from_string(bin: &[u8]) {
        let string = HexBin::to_string(bin, Some(NonZeroU32::new(4).unwrap()));
        let resulting_vec = HexBin::as_vec(&string, true).unwrap();
        assert_eq!(bin, resulting_vec.as_slice());
    }

}

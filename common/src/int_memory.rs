// Integer memory representation.

use std::cmp::max;

/// Integer memory representation; number of bits required to store the integer.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum IntMemory {
    M8,
    M16,
    M32,
    M64,
    M128,
}

impl IntMemory {
    pub fn number_of_bytes(&self) -> u8 {
        match self {
            &IntMemory::M8 => {
                1
            }
            &IntMemory::M16 => {
                2
            }
            &IntMemory::M32 => {
                4
            }
            &IntMemory::M64 => {
                8
            }
            &IntMemory::M128 => {
                16
            }
        }
    }

    pub fn number_of_bits(&self) -> u8 {
        self.number_of_bytes() * 8
    }

    /// Computes the maximum of the two.
    pub fn max(self, other : Self) -> Self {
        max(self, other)
    }
}

impl From<&u128> for IntMemory {
    fn from(value: &u128) -> Self {
        if value <= &(std::u8::MAX as u128) {
            IntMemory::M8
        } else if value <= &(std::u16::MAX as u128) {
            IntMemory::M16
        } else if value <= &(std::u32::MAX as u128) {
            IntMemory::M32
        } else if value <= &(std::u64::MAX as u128) {
            IntMemory::M64
        } else {
            IntMemory::M128
        }
    }
}

impl From<&i128> for IntMemory {
    fn from(value: &i128) -> Self {
        if value <= &(std::i8::MAX as i128) && value >= &(std::i8::MIN as i128) {
            IntMemory::M8
        } else if value <= &(std::i16::MAX as i128) && value >= &(std::i16::MIN as i128) {
            IntMemory::M16
        } else if value <= &(std::i32::MAX as i128) && value >= &(std::i32::MIN as i128) {
            IntMemory::M32
        } else if value <= &(std::i64::MAX as i128) && value >= &(std::i64::MIN as i128) {
            IntMemory::M64
        } else {
            IntMemory::M128
        }
    }
}
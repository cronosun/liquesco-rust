use std::hash::Hasher;
use blake2::VarBlake2b;
use blake2::digest::{Input, VariableOutput};
use std::fmt::{Display, Error, Formatter};

const TH_LEN : usize = 16;

/// The hash of a type.
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TypeHash([u8; TH_LEN]);

/// The default format is the lower hex format.
impl Display for TypeHash {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for byte in &self.0 {
            std::fmt::LowerHex::fmt(byte, f)?;
        }
        Ok(())
    }
}

impl Into<String> for &TypeHash {
    fn into(self) -> String {
        format!("{}", self)
    }
}

// TODO: Into<Identifier> ('th0x_SEG1_SEG2')
// TODO: From<String> (lower hex)

pub(crate) struct TypeHasher {
    blake2 : VarBlake2b
}

impl Hasher for TypeHasher {
    fn finish(&self) -> u64 {
        panic!("finish is not implemented for this hasher")
    }

    fn write(&mut self, bytes: &[u8]) {
        self.blake2.input(bytes);
    }
}

impl TypeHasher {
    pub(crate) fn finish(self) -> TypeHash {
        let mut result = [0u8; TH_LEN];
        self.blake2.variable_result(|res| {
            result.copy_from_slice(res);
        });
        TypeHash(result)
    }
}

impl Default for TypeHasher {
    fn default() -> Self {
        Self {
            blake2 : VarBlake2b::new(TH_LEN).unwrap()
        }
    }
}


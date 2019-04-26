use crate::serialization::core::BinaryWriter;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeId;
use crate::serialization::core::Serializer;
use crate::serialization::core::Writer;

pub struct VecWriter {
    data: Vec<u8>,
}

impl Default for VecWriter {
    fn default() -> Self {
        VecWriter { data: Vec::new() }
    }
}

impl Writer for VecWriter {
    fn write<T: Serializer>(&mut self, item: &T::Item) -> Result<(), LqError> {
        T::serialize(self, item)
    }
}

impl std::io::Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.extend_from_slice(buf);
        Result::Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Result::Ok(())
    }
}

impl BinaryWriter for VecWriter {
    fn write_u8(&mut self, data: u8) -> Result<(), LqError> {
        self.data.push(data);
        Result::Ok(())
    }

    fn write_slice(&mut self, buf: &[u8]) -> Result<(), LqError> {
        self.data.extend_from_slice(buf);
        Result::Ok(())
    }

    fn type_id(&mut self, id: TypeId) -> Result<(), LqError> {
        self.write_u8(id.id())
    }
}

impl VecWriter {
    pub fn finish(self) -> Vec<u8> {
        self.data
    }
}

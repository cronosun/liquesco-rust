use crate::serialization::core::TypeId;
use crate::serialization::core::BinaryWriter;
use crate::serialization::core::LqError;
use crate::serialization::core::TypeWriter;
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
    fn write<T: TypeWriter>(&mut self, item: &T::Item) -> Result<(), LqError> {
        let header_writer = HeaderWriterStruct {
            data: &mut self.data,
        };
        T::write(header_writer, item)
    }
}

impl VecWriter {
    pub fn finish(self) -> Vec<u8> {
        self.data
    }
}

struct HeaderWriterStruct<'a> {
    data: &'a mut Vec<u8>,
}

impl<'a> BinaryWriter<'a> for HeaderWriterStruct<'a> {
    type Writer = Vec<u8>;

    fn begin(self, id: TypeId) -> Result<&'a mut Self::Writer, LqError> {
        self.data.push(id.id());
        Result::Ok(self.data)
    }
}
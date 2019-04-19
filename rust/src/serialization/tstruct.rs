use crate::serialization::core::BinaryWriter;
use crate::serialization::core::Type;
use crate::serialization::core::LqError;
use crate::serialization::core::ReadResult;
use crate::serialization::core::TypeId;

pub struct TStruct;

pub struct Struct {
    number_of_fields: usize,
}

impl Struct {
    pub fn new(number_of_fields : usize) -> Self {
        Self {
            number_of_fields 
        }
    }

    pub fn number_of_fields(&self) -> usize {
        self.number_of_fields
    }
}

impl<'a> Type<'a> for TStruct {
    type ReadItem = Struct;
    type WriteItem = Struct;

    fn read(id: TypeId, _: &[u8]) -> Result<ReadResult<Self::ReadItem>, LqError> {
        unimplemented!()
    }

    fn write<'b, Writer: BinaryWriter<'b> + 'b>(
        writer: Writer,
        item: &Self::WriteItem,
    ) -> Result<(), LqError> {
        unimplemented!()
    }
}

fn number_of_fields(id: TypeId) -> Result<Option<usize>, LqError> {
   /* match id {
        
    }*/
    unimplemented!()
}

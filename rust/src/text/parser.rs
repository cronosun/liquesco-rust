use crate::text::value::Value;
use crate::common::error::LqError;
use crate::schema::uint::TUInt;
use crate::schema::core::Type;
use crate::serialization::core::LqWriter;
use crate::serialization::uint::UInt64;
use crate::schema::core::Schema;
use crate::text::value::Converter;
use crate::serialization::core::Serializer;

/*pub struct Context<'a, C, S, W> where C : Converter, S : Schema, W : LqWriter {    
    writer : &'a mut W,
    schema : &'a S,
    _phantom : PhantomData<C>,
}*/

pub trait Context {
    type TConverter : Converter;
    type TSchema : Schema;
    type TWriter : LqWriter;

    fn writer(&mut self) -> &mut Self::TWriter;
    fn schema(&self) -> &Self::TSchema;
    fn value(&self) -> &Value;
}

pub trait Parser<'a> {
    type T : Type<'a>;

    fn parse<C>(context : &mut C, r#type : Self::T) -> Result<(), ParseError>  where C : Context;
}

pub struct ParseError {
    msg : Option<String>,
    lq_error : Option<LqError>,
}

pub struct UIntParser;

impl Parser<'static> for UIntParser {
    type T = TUInt;

    fn parse<C>(context : &mut C, r#type : Self::T) -> Result<(), ParseError> where C : Context {
        let value = C::TConverter::require_u64(context.value())?;
        UInt64::serialize(context.writer(), &value).unwrap(); // TODO
        Result::Ok(())
    }
}
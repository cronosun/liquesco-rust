use crate::text::reader::SrcPosition;
use crate::common::error::LqError;

pub struct ReadError {
    msg : Option<String>,
    position : Option<SrcPosition>,
    lq_error : Option<LqError>,
}
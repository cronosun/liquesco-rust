use crate::text::read_error::ReadError;

pub enum Token<'a> {
    String(&'a str),
    StartSeq,
    EndSeq,    
    Bool(bool),
    U64(u64),
    I64(i64),
    F64(f64),
}

#[derive(Clone, Debug)]
pub struct SrcPosition(pub usize);

pub trait TextReader {
    fn next_opt(&mut self) -> Result<Option<&Token>, ReadError>;
    fn next(&mut self) -> Result<&Token, ReadError>;

    fn finish_sequence(&mut self) -> Result<(), ReadError> {
        let mut number_to_finish = 1;
        while number_to_finish>0 {
            let item = self.next()?;
            match item {
                Token::StartSeq => number_to_finish = number_to_finish + 1,
                Token::EndSeq => number_to_finish = number_to_finish - 1,
                _ => {}
            }
        };
        Result::Ok(())
    }

    fn to_string(&self, token : &Token) -> Option<&str> {
        match token {
            Token::String(value) => Option::Some(value),
            _ => Option::None
        }
    }
    
    fn position(&self) -> &SrcPosition;

    fn current_name(&self) -> &str;
}
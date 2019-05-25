use liquesco_schema::identifier::Identifier;
use std::convert::TryInto;
use std::fmt::Debug;

pub mod builder;

pub mod yaml_schema1;
pub mod yaml_schema2;
pub mod yaml_schema3;
pub mod yaml_schema4;
pub mod yaml_schema5;

pub fn assert_ok<T, R: Debug + Send + 'static>(result: Result<T, R>) {
    if result.is_err() {
        panic!(format!("Got error: {:?}", result.err().unwrap()))
    }
}

pub fn assert_err<T, R>(result: Result<T, R>) {
    assert!(result.is_err())
}

pub fn id(string: &'static str) -> Identifier<'static> {
    string.try_into().unwrap()
}

use std::fmt::Debug;

pub mod builder;

pub mod yaml_schema1;

pub fn assert_ok<T, R : Debug + Send + 'static>(result : Result<T, R>) {
    if result.is_err() {
        panic!(format!("Got error: {:?}", result.err().unwrap()))
    }
}

pub fn assert_err<T, R>(result : Result<T, R>) {
    assert!(result.is_err())
}
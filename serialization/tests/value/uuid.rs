use crate::value::check_value;
use liquesco_serialization::types::uuid::Uuid;

#[test]
fn test_uuids() {
    for index in 0..10 {
        let uuid = Uuid::from([47, 79, index as u8, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
        check_value(&(uuid).into());
    }
}

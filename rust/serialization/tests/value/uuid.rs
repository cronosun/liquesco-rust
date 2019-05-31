use crate::value::check_value;
use liquesco_serialization::uuid::Uuid;

#[test]
fn test_uuids() {
    for _ in 0..10 {
        check_value(&(Uuid::new_v4()).into());
    }
}


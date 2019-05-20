use crate::uuid::Uuid;
use crate::tests::value::utils::check_value;

#[test]
fn test_uuids() {
    for _ in 0..10 {
        check_value(&(Uuid::new_v4()).into());
    }
}

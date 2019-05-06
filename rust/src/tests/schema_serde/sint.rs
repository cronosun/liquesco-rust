use crate::tests::schema_serde::utils::check_serde;
use crate::schema::vsint::VSInt;

#[test]
fn simple_sint() {
    check_serde(|builder| {
        builder.add(VSInt::try_new(std::i64::MIN, -1).unwrap().into())
    });
}
use crate::tests::schema_serde::utils::check_serde;
use crate::schema::vuint::VUInt;

#[test]
fn simple_uint() {
    check_serde(|builder| {
        builder.add(VUInt::try_new(0, 230).unwrap().into())
    });
}
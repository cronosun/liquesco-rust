use crate::tests::schema_serde::utils::check_serde;
use crate::schema::vascii::VAscii;

#[test]
fn simple_ascii() {
    check_serde(|builder| {
        builder.add(VAscii::try_new(0, 230, 0, 127).unwrap().into())
    });
}
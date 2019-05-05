use crate::schema::identifier::Identifier;
use crate::schema::vstruct::Field;
use crate::schema::vstruct::VStruct;
use crate::schema::vuint::VUInt;
use crate::schema::vascii::VAscii;
use crate::tests::schema_serde::utils::check_serde;
use std::convert::TryFrom;

#[test]
fn simple_struct() {
    check_serde(|builder| {
        let type1 = builder.add(VUInt::try_new(0, 230).unwrap().into());
        let type2 = builder.add(VAscii::try_new(45, 53453455, 97, 122).unwrap().into());

        let mut st = VStruct::default();
        st.add(Field::new(
            Identifier::try_from("my_field_one").unwrap(),
            type1,
        ));
        st.add(Field::new(
            Identifier::try_from("another_field").unwrap(),
            type2,
        ));

        builder.add(st.into())
    });
}

use crate::schema::core::Schema;
use crate::schema::vascii::VAscii;
use crate::schema::vbool::BoolValues;
use crate::schema::vbool::VBool;
use crate::schema::voption::VOption;
use crate::schema::vseq::Direction;
use crate::tests::schema::builder::builder;
use crate::tests::schema::ordering::ord_schema;
use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;

#[test]
fn schema1() {
    let mut builder = builder();
    let boolean = builder.add(VBool::new(BoolValues::TrueOnly));
    let schema = builder.finish(VOption::new(boolean));

    // some valid items
    assert_valid_strict(Option::<bool>::None, &schema);
    assert_valid_strict(Option::Some(true), &schema);

    // The only invalid value (false is not allowed)
    assert_invalid_strict(Option::Some(false), &schema);

    // completely wrong type
    assert_invalid_strict(Option::Some("expecting a bool here".to_string()), &schema);
}

fn ordering_create_schema() -> impl Schema {
    ord_schema(
        |builder| {
            let element = builder.add(VAscii::try_new(0, std::u64::MAX, 0, 127).unwrap());
            let option = VOption::new(element);
            builder.add(option)
        },
        Direction::Ascending,
        true,
    )
}

#[test]
fn ordering() {
    let schema = ordering_create_schema();

    // Present is always greater than absent
    assert_valid_strict(
        (Option::<String>::None, Option::Some("".to_string())),
        &schema,
    );

    assert_valid_strict(
        (Option::Some("a".to_string()), Option::Some("b".to_string())),
        &schema,
    );

    assert_valid_strict(
        (
            Option::Some("aaaaa".to_string()),
            Option::Some("aaaaaa".to_string()),
        ),
        &schema,
    );

    assert_valid_strict(
        (
            Option::Some("aaaaaaaaa".to_string()),
            Option::Some("aaaab".to_string()),
        ),
        &schema,
    );

    // invalid: duplicate
    assert_invalid_strict((Option::<String>::None, Option::<String>::None), &schema);

    // invalid: duplicate
    assert_invalid_strict(
        (
            Option::Some("aaaab".to_string()),
            Option::Some("aaaab".to_string()),
        ),
        &schema,
    );

    // invalid: wrong ordering
    assert_invalid_strict(
        (
            Option::Some("aaaab".to_string()),
            Option::Some("aaaaa".to_string()),
        ),
        &schema,
    );

    // invalid: wrong ordering
    assert_invalid_strict(
        (Option::Some("".to_string()), Option::<String>::None),
        &schema,
    );
}

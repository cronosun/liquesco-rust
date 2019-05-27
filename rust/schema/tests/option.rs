mod common;

use common::builder::builder;
use common::ordering::ord_schema;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::boolean::TBool;
use liquesco_schema::core::Schema;
use liquesco_schema::option::TOption;
use liquesco_schema::seq::Direction;

#[test]
fn schema1() {
    let mut builder = builder();
    let boolean = builder.add(TBool::default());
    let schema = builder.finish(TOption::new(boolean));

    // some valid items
    assert_valid_strict(Option::<bool>::None, &schema);
    assert_valid_strict(Option::Some(true), &schema);

    // completely wrong type
    assert_invalid_strict(Option::Some("expecting a bool here".to_string()), &schema);
}

fn ordering_create_schema() -> impl Schema<'static> {
    ord_schema(
        |builder| {
            let element = builder.add(TAscii::try_new(0, std::u64::MAX, 0, 127).unwrap());
            let option = TOption::new(element);
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

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
use liquesco_schema::schema_builder::SchemaBuilder;
use liquesco_schema::type_container::DefaultTypeContainer;
use liquesco_schema::schema::DefaultSchema;

#[test]
fn schema1() {
    let mut builder = builder();
    let boolean = builder.add_unwrap("boolean", TBool::default());
    let schema : DefaultSchema<'static, DefaultTypeContainer<'static>> =
        builder.finish(TOption::new(boolean)).unwrap().into();

    // some valid items
    assert_valid_strict(Option::<bool>::None, &schema);
    assert_valid_strict(Option::Some(true), &schema);

    // completely wrong type
    assert_invalid_strict(Option::Some("expecting a bool here".to_string()), &schema);
}

fn ordering_create_schema() -> impl Schema<'static> {
    ord_schema(
        |builder| {
            let element = builder.add_unwrap("element_in_ord", TAscii::try_new(0, std::u64::MAX, 0, 127).unwrap());
            let option = TOption::new(element);
            builder.add_unwrap("option_element_in_ord", option)
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

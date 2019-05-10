use crate::schema::vbool::BoolValues;
use crate::schema::vbool::VBool;
use crate::schema::voption::VOption;
use crate::tests::schema::builder::builder;
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

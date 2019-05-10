use crate::schema::vint_range::VUIntRange;
use crate::schema::vint_range::VSIntRange;
use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use crate::tests::schema::utils::single_schema;

#[test]
fn urange() {
    let schema = single_schema(VUIntRange::try_new(4, 14000).unwrap());

    // some valid items
    assert_valid_strict((4usize, 14000usize), &schema);
    assert_valid_strict((10usize, 14000usize), &schema);
    assert_valid_strict((4usize, 13999usize), &schema);
    assert_valid_strict((50usize, 200usize), &schema);
    assert_valid_strict((50usize, 50usize), &schema);

    // some invalid items
    assert_invalid_strict((3usize, 14000usize), &schema);
    assert_invalid_strict((4usize, 14001usize), &schema);
    // !min<=max
    assert_invalid_strict((51usize, 50usize), &schema);
}

#[test]
fn srange() {
    let schema = single_schema(VSIntRange::try_new(-25, 14000).unwrap());

    // some valid items
    assert_valid_strict((-25isize, 14000isize), &schema);
    assert_valid_strict((-10isize, 14000isize), &schema);
    assert_valid_strict((-25isize, 13999isize), &schema);
    assert_valid_strict((50isize, 200isize), &schema);
    assert_valid_strict((50isize, 50isize), &schema);

    // some invalid items
    assert_invalid_strict((-26isize, 14000isize), &schema);
    assert_invalid_strict((-25isize, 14001isize), &schema);
    // !min<=max
    assert_invalid_strict((51isize, 50isize), &schema);
}

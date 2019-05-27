mod common;

use common::builder::builder;
use common::utils::assert_invalid_strict;
use common::utils::assert_valid_strict;
use common::utils::id;
use liquesco_schema::anchors::TAnchors;
use liquesco_schema::ascii::TAscii;
use liquesco_schema::boolean::TBool;
use liquesco_schema::core::Schema;
use liquesco_schema::reference::TReference;
use liquesco_schema::structure::Field;
use liquesco_schema::structure::TStruct;

use serde::{Deserialize, Serialize};

#[test]
fn container_has_to_have_two_elements() {
    let schema = create_schema1();
    assert_invalid_strict((), &schema);
}

/// No need to have anchors at all
#[test]
fn master_alone_is_sufficient() {
    let schema = create_schema1();
    assert_valid_strict(
        (
            Schema1Master {
                name: "hello master!".to_string(),
                next_ref: 0, // references myself
                i_am_great: true,
            },
            Vec::<Schema1>::new(),
        ),
        &schema,
    );
}

#[test]
fn with_one_anchor() {
    let schema = create_schema1();
    assert_valid_strict(
        (
            Schema1Master {
                name: "hello master!".to_string(),
                next_ref: 1, // references first slave
                i_am_great: true,
            },
            vec![Schema1 {
                name: "frist slave".to_string(),
                next_ref: 0, // references master
            }],
        ),
        &schema,
    );
}

/// This is invalid since anchor at index 1 is not used. It's only
/// referenced by itself. The master anchor is the only anchor that
/// is allowed to be unreferenced.
#[test]
fn unused_anchor() {
    let schema = create_schema1();
    assert_invalid_strict(
        (
            Schema1Master {
                name: "hello master!".to_string(),
                next_ref: 0, // references myself
                i_am_great: true,
            },
            vec![Schema1 {
                name: "frist slave".to_string(),
                next_ref: 1, // references myself
            }],
        ),
        &schema,
    );
}

#[test]
fn unused_anchor_v2() {
    let schema = create_schema1();
    assert_invalid_strict(
        (
            Schema1Master {
                name: "hello master!".to_string(),
                next_ref: 1,
                i_am_great: true,
            },
            vec![
                Schema1 {
                    name: "index 1".to_string(),
                    next_ref: 2,
                },
                Schema1 {
                    name: "index 2".to_string(),
                    next_ref: 2,
                },
                // unused: not allowed
                Schema1 {
                    name: "index 3".to_string(),
                    next_ref: 1,
                },
            ],
        ),
        &schema,
    );
}

#[test]
fn with_4_anchors() {
    let schema = create_schema1();
    assert_valid_strict(
        (
            Schema1Master {
                name: "hello master!".to_string(),
                next_ref: 1,
                i_am_great: true,
            },
            vec![
                Schema1 {
                    name: "index 1".to_string(),
                    next_ref: 2,
                },
                Schema1 {
                    name: "index 2".to_string(),
                    next_ref: 3,
                },
                Schema1 {
                    name: "index 3".to_string(),
                    next_ref: 1, // and back to 1
                },
            ],
        ),
        &schema,
    );
}

#[test]
fn wrong_ordering() {
    let schema = create_schema1();
    assert_invalid_strict(
        (
            Schema1Master {
                name: "hello master!".to_string(),
                next_ref: 1,
                i_am_great: true,
            },
            vec![
                // references index 3 (skips index 2)
                Schema1 {
                    name: "index 1".to_string(),
                    next_ref: 3,
                },
                Schema1 {
                    name: "index 2".to_string(),
                    next_ref: 2,
                },
                Schema1 {
                    name: "index 3".to_string(),
                    next_ref: 2,
                },
            ],
        ),
        &schema,
    );
}

fn create_schema1() -> impl Schema<'static> {
    let mut builder = builder();
    let reference = builder.add(TReference::default());
    let name = builder.add(TAscii::try_new(0, 100, 0, 127).unwrap());
    let bool_field = builder.add(TBool::default());
    let structure = builder.add(Into::<TStruct>::into(
        TStruct::default()
            .add(Field::new(id("name"), name))
            .add(Field::new(id("reference"), reference)),
    ));
    let structure_master = builder.add(Into::<TStruct>::into(
        TStruct::default()
            .add(Field::new(id("name"), name))
            .add(Field::new(id("reference"), reference))
            .add(Field::new(id("i_am_great"), bool_field)),
    ));

    builder.finish(TAnchors::new(structure_master, structure))
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Schema1 {
    name: String,
    next_ref: u32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Schema1Master {
    name: String,
    next_ref: u32,
    i_am_great: bool,
}

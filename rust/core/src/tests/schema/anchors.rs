use crate::schema::core::Schema;
use crate::schema::anchors::TAnchors;
use crate::schema::ascii::TAscii;
use crate::schema::boolean::TBool;
use crate::schema::reference::TReference;
use crate::schema::structure::TStruct;
use crate::tests::schema::builder::builder;
use crate::tests::schema::utils::assert_invalid_strict;
use crate::tests::schema::utils::assert_valid_strict;
use crate::tests::schema::utils::id;

use serde::{Deserialize, Serialize};

#[test]
fn need_at_least_the_master() {
    let schema = create_schema1();
    assert_invalid_strict((), &schema);
}

#[test]
fn master_alone_is_sufficient() {
    let schema = create_schema1();
    assert_valid_strict(
        vec![Schema1Master {
            name: "hello master!".to_string(),
            next_ref: 0, // references myself
            i_am_great: true,
        }],
        &schema,
    );
}

#[test]
fn with_two_anchors_v2() {
    let schema = create_schema1();
    assert_valid_strict(
        (Schema1Master {
            name: "hello master!".to_string(),
            next_ref: 1, // references first slave
            i_am_great: true,
        },
        Schema1 {
            name : "frist slave".to_string(),
            next_ref : 0 // references master
        }),
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
        (Schema1Master {
            name: "hello master!".to_string(),
            next_ref: 0, // references myself
            i_am_great: true,
        },
        Schema1 {
            name : "frist slave".to_string(),
            next_ref : 1 // references myself
        }),
        &schema,
    );
}

#[test]
fn unused_anchor_v2() {
    let schema = create_schema1();
    assert_invalid_strict(
        (Schema1Master {
            name: "hello master!".to_string(),
            next_ref: 1, 
            i_am_great: true,
        },
        Schema1 {
            name : "index 1".to_string(),
            next_ref : 2 
        },
        Schema1 {
            name : "index 2".to_string(),
            next_ref : 2 
        },
        // unused: not allowed
        Schema1 {
            name : "index 3".to_string(),
            next_ref : 1 
        }),
        &schema,
    );
}

#[test]
fn with_4_anchors() {
    let schema = create_schema1();
    assert_valid_strict(
        (Schema1Master {
            name: "hello master!".to_string(),
            next_ref: 1,
            i_am_great: true,
        },
        Schema1 {
            name : "index 1".to_string(),
            next_ref : 2 
        },
        Schema1 {
            name : "index 2".to_string(),
            next_ref : 3 
        },
        Schema1 {
            name : "index 3".to_string(),
            next_ref : 1 // and back to 1 
        }),
        &schema,
    );
}

#[test]
fn wrong_ordering() {
    let schema = create_schema1();
    assert_invalid_strict(
        (Schema1Master {
            name: "hello master!".to_string(),
            next_ref: 1, 
            i_am_great: true,
        },
        // references index 3 (skips index 2)
        Schema1 {
            name : "index 1".to_string(),
            next_ref : 3 
        },
        Schema1 {
            name : "index 2".to_string(),
            next_ref : 2 
        },        
        Schema1 {
            name : "index 3".to_string(),
            next_ref : 2 
        }),
        &schema,
    );
}

fn create_schema1() -> impl Schema<'static> {
    let mut builder = builder();
    let reference = builder.add(TReference);
    let name = builder.add(TAscii::try_new(0, 100, 0, 127).unwrap());
    let bool_field = builder.add(TBool::default());
    let structure = builder.add(
        TStruct::builder()
            .field(id("name"), name)
            .field(id("reference"), reference)
            .build(),
    );
    let structure_master = builder.add(
        TStruct::builder()
            .field(id("name"), name)
            .field(id("reference"), reference)
            .field(id("i_am_great"), bool_field)
            .build(),
    );

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

use crate::tests::serde::utils::can_decode_from;
use serde::{Deserialize, Serialize};

/// This is required when we want to read old formats (version 1). Serde
/// should then just take default values.
///
/// (it's the reverse of 'extension')
#[test]
fn can_read_old_format() {
    // this generates 'old' data.
    let source = Version1 {
        info: Version1Inner { age: 34.7 },
        first_name: "Johannes".to_string(),
        last_name: "Doe".to_string(),
    };
    // and this is what we get when trying to parse using a new serializer.
    let destination = Version2 {
        info: Version2Inner {
            age: 34.7,
            female: false,
            number_of_children: 0,
        },
        first_name: "Johannes".to_string(),
        last_name: "Doe".to_string(),
        // this value is missing in source, so just take the default
        suffix: Option::default(),
        // this value is missing in source, so just take the default
        employee: String::default(),
    };

    can_decode_from(source, destination);
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Version1 {
    info: Version1Inner,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Version1Inner {
    age: f32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Version2 {
    info: Version2Inner,
    first_name: String,
    last_name: String,
    #[serde(default)]
    suffix: Option<String>,
    #[serde(default)]
    employee: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Version2Inner {
    age: f32,
    #[serde(default)]
    female: bool,
    #[serde(default)]
    number_of_children: usize,
}

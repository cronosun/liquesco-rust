use crate::serde::can_decode_from;
use serde::{Deserialize, Serialize};

/// It's possible to extend enums (schema evolution) - it's still possible
/// to encode this data using "old" deserializers (all new data is discarded).
#[test]
fn enum_are_extensible_1() {
    let version_2 = Version2::CreateAccount {
        email: "demo@demo.com".to_string(),
        password: "secret".to_string(),
        picture: vec![47, 47, 78, 0, 48, 75],
    };
    let version_1 = Version1::CreateAccount {
        email: "demo@demo.com".to_string(),
        password: "secret".to_string(),
    };

    can_decode_from(version_2, version_1);
}

#[test]
fn enum_are_extensible_2() {
    let version_2 = Version2::DeleteAccount {
        account_id: "114256585".to_string(),
        delete_user_data: true,
    };
    let version_1 = Version1::DeleteAccount("114256585".to_string());

    can_decode_from(version_2, version_1);
}

#[test]
fn enum_are_extensible_3() {
    let version_2 = Version2::Upgrade {
        account_id: "114256585_7".to_string(),
        megabytes: 4_458_754,
        activate_premium_account: true,
    };
    let version_1 = Version1::Upgrade {
        account_id: "114256585_7".to_string(),
        megabytes: 4_458_754,
    };

    can_decode_from(version_2, version_1);
}

#[test]
fn enum_are_extensible_4() {
    let version_2 = Version2::Ping(true, "hello serve!".to_string());
    let version_1 = Version1::Ping;

    can_decode_from(version_2, version_1);
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Version1 {
    CreateAccount {
        email: String,
        password: String,
    },
    DeleteAccount(String),
    Upgrade {
        account_id: String,
        megabytes: usize,
    },
    Ping,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
enum Version2 {
    CreateAccount {
        email: String,
        password: String,
        picture: Vec<u8>,
    },
    DeleteAccount {
        account_id: String,
        delete_user_data: bool,
    },
    Upgrade {
        account_id: String,
        megabytes: usize,
        activate_premium_account: bool,
    },
    Ping(bool, String),
}

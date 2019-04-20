use crate::serialization::test::assert_binary;
use crate::serialization::test::de_serialize;
use crate::serialization::test::serialize;
use crate::serialization::tutf8::TUtf8;

#[test]
fn small_strings_utf8() {
    assert_string_serde_eq("");
    assert_string_serde_eq("1");
    assert_string_serde_eq("1è");
    assert_string_serde_eq("1èf");
    assert_string_serde_eq("1èfö");
    assert_string_serde_eq("hello");
    assert_string_serde_eq("hello6");
    assert_string_serde_eq("hello67");
    assert_string_serde_eq("hello678");
}

#[test]
fn test_small_utf8() {
    assert_binary::<TUtf8>("hello", &[0x55, 104, 101, 108, 108, 111]);
    assert_binary::<TUtf8>("hell", &[0x54, 104, 101, 108, 108]);
    assert_binary::<TUtf8>("hel", &[0x53, 104, 101, 108]);
    assert_binary::<TUtf8>("he", &[0x52, 104, 101]);
    assert_binary::<TUtf8>("h", &[0x51, 104]);
    assert_binary::<TUtf8>("", &[0x50]);
}

#[test]
fn u8_utf8() {
    // longer than 8 bytes
    assert_string_serde_eq("of_11_bytes");
    assert_string_serde_eq("16_bytes_ooooooo");
    assert_string_serde_eq("32_bytes_oooooooiiiiiiiiiiiiiiii");
}

#[test]
fn u8_utf8_binary() {
    // longer than 8 bytes
    assert_binary::<TUtf8>(
        "of_11_bytes",
        &[0x5B, 11, 111, 102, 95, 49, 49, 95, 98, 121, 116, 101, 115],
    );
    assert_binary::<TUtf8>(
        "16_bytes_ooooooo",
        &[
            0x59, 49, 54, 95, 98, 121, 116, 101, 115, 95, 111, 111, 111, 111, 111, 111, 111,
        ],
    );
    assert_binary::<TUtf8>(
        "32_bytes_oooooooiiiiiiiiiiiiiiii",
        &[
            0x5A, 51, 50, 95, 98, 121, 116, 101, 115, 95, 111, 111, 111, 111, 111, 111, 111, 105,
            105, 105, 105, 105, 105, 105, 105, 105, 105, 105, 105, 105, 105, 105, 105,
        ],
    );
}

fn assert_string_serde_eq(string: &'static str) {
    let binary = serialize::<TUtf8>(string);
    let restored = de_serialize::<TUtf8>(&binary);

    assert_eq!(string, restored);
}

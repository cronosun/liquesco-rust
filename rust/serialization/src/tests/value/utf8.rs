use crate::tests::value::utils::check_value;

#[test]
fn str_var_len() {
    let mut string = String::new();
    for index in 0..100 {
        check_value(&(&string).into());
        string.push(index as u8 as char);
    }
}

#[test]
fn str_empty() {
    check_value(&"".into());
}

#[test]
fn str_example1() {
    check_value(&"This is some string".into());
}

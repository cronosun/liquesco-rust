use crate::tests::value::utils::check_value;

#[test]
fn variable_binary() {
    let mut binary = Vec::new();
    for index in 0..17000 {
        check_value(&(&binary).into());
        binary.push((index % 255) as u8);
    }
}

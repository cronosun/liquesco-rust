use crate::value::check_value;

#[test]
fn variable_binary() {
    let mut binary = Vec::new();
    for index in 0..128457 {
        if index % 1785 == 0 {
            check_value(&(&binary).into());
        }
        binary.push((index % 255) as u8);
    }
}

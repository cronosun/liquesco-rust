use crate::serialization::value::Value;
use crate::tests::value::utils::check_value;

#[test]
fn list_empty() {
    let list = vec![];
    check_value(&(&list).into());
}

#[test]
fn list_variable() {
    for index in 0..300 {
        let mut list: Vec<Value<'static>> = vec![];
        for inner in 0..index {
            list.push(format!("item_{:?}", inner).into())
        }
        check_value(&list.into());
    }
}

#[test]
fn list_sample() {
    let list: Vec<Value<'static>> = vec![
        "hello".into(),
        Option::Some::<Value<'static>>("world".into()).into(),
        8.into(),
        (-345 as i32).into(),
        true.into(),
    ];
    check_value(&list.into());
}

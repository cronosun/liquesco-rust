use crate::serialization::core::SliceReader;
use crate::serialization::tutf8::TUtf8;
use crate::serialization::test::new_writer;
use crate::serialization::core::Writer;
use crate::serialization::core::Reader;

#[test]
fn simple_utf8() {
    let mut writer = new_writer();
    let original = "Hello World!";
    writer.write::<TUtf8>(original).unwrap();
    let done = writer.finish();
    let mut reader : SliceReader = SliceReader::from(done.as_slice());
    let result = reader.read::<TUtf8>().unwrap();
    assert_eq!(original, result);
}
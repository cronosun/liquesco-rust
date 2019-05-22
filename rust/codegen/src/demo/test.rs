use crate::code_receiver::DefaultCodeReceiver;
use crate::demo::HtmlCodeGen;
use crate::path::{Path, Segment};
use crate::settings::Settings;
use crate::Plugin;

#[test]
fn test1() {
    let mut cr = DefaultCodeReceiver::default();
    let html_gen = HtmlCodeGen;
    html_gen.process(&mut cr, &Settings::default()).unwrap(); // TODO
    let result = cr
        .take_string(&Path::new(Segment::new("schema.html")))
        .unwrap()
        .unwrap();
    println!("{result}", result=result);
}

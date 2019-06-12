use crate::plugin::HtmlGenSchemaPlugin;
use liquesco_processing::code_receiver::DefaultCodeReceiver;
use liquesco_processing::path::Path;
use liquesco_processing::path::Segment;
use liquesco_processing::plugin::Plugin;
use liquesco_processing::settings::Settings;

#[test]
fn test1() {
    let mut cr = DefaultCodeReceiver::default();
    let plugin = HtmlGenSchemaPlugin;
    plugin.process(&mut cr, &Settings::default()).unwrap(); // TODO
    let result = cr
        .take_string(&Path::new(Segment::new("schema.html")))
        .unwrap()
        .unwrap();
    println!("{result}", result = result);
}

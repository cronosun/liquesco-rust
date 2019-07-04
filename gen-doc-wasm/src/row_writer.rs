use std::borrow::Cow;
use web_sys::Element;
use web_sys::Document;
use liquesco_gen_doc::model::row::{Row, TextWithLink};
use liquesco_gen_doc::model::Model;

use typed_html::html;
use typed_html::text;
use typed_html::elements::EmbeddedContent;
use typed_html::dom::{Node, DOMTree};

struct RowWriter<'a, TModel : Model> {
    model : &'a TModel,
}

impl<'a, TModel : Model> RowWriter<'a, TModel> {

    pub fn write(&self, row : &Row) -> DOMTree<String> {
        match row {
            Row::Note(value) => self.write_note(value),
        }

    }

    fn write_note(&self, row : &str) -> DOMTree<String> {
        html!(
            <tr>
                <td rowspan="2" class="liquesco-row-note">{ text!("{}", row) }</td>
                <td></td>
            </tr>
        )
    }

    fn write_link(&self, text_with_link : &TextWithLink) -> DOMTree<String> {
        html!(
            <div class="liquesco-row-link" onclick={|_, _, _| {
                ()
            }}>{ text!("{}", text_with_link.text()) }</div>
        )
    }
}




/*fn write_section(doc : &Document, text : &str) -> Element {
    let element = doc.create_element("div").unwrap();
    element.set_inner_html(text);
    element
}*/

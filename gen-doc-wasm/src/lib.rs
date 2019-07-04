// #![feature(proc_macro_hygiene)]

//use virtual_dom_rs::prelude::*;
//use typed_html_macros::html;
use typed_html::html;


mod api;
mod utils;
mod row_writer;

use wasm_bindgen::prelude::*;
use typed_html::dom::DOMTree;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn app() -> String {
    let some_component : DOMTree<String> = html!(
        <div class="cool-component">"Hello World"</div>
    );

    some_component.to_string()
}

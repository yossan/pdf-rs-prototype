use pdf::core::document::{ PdfDocument };

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace=console)]
    fn log(str: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, sample!");
}

#[wasm_bindgen]
pub fn load_data(data: Vec<u8>) -> String {
    log(&format!("data.len = {}", data.len()));
    let _document = PdfDocument::loadData(data, None);
    // log(&format!("result = {}", _document));
    return _document;
}

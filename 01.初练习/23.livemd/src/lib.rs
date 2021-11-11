use comrak::{markdown_to_html, ComrakOptions};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(source: &str) -> String {
    markdown_to_html(source, &ComrakOptions::default())
}

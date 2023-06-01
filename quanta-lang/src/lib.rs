mod utils;

use wasm_bindgen::prelude::*;
use quanta_parser::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, quanta-lang!");
}

#[wasm_bindgen]
pub fn is_great_text(source : &str) -> bool {
    return true;
    //alert(&format!("Is good text \"{}\"? Result: {}", source, quanta_parser::parse_text(source)));
    //return quanta_parser::parse_text(source);
}

mod utils;
mod compiler;
mod program;

use wasm_bindgen::prelude::*;
//use quanta_parser::parse_text;

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
pub fn compile_code(source : &str) -> String {
    compiler::compilation_result(source)
}

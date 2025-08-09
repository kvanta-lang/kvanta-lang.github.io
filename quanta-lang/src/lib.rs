mod utils;
mod compiler;
mod program;
mod execution;

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
    "background #071022
circle 200 160 80 fill=tomato stroke=white width=4
rectangle 320 80 200 120 fill=#1e293b stroke=#94a3b8 width=2
line 60 300 540 300 stroke=#22d3ee width=3
polygon 100 150 150 150 200 100 50 100 fill=gold
arc 200 160 110 20 320 stroke=purple width=6".to_string()
    //compiler::compilation_result(source)
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a:i32, b:i32) -> i32 {
    a + b + 10
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Wow, {} is a great text!", name));
}

#[wasm_bindgen]
pub fn is_good_text(s: &str) -> i32 {
    if s == "circle(320, 240, 100);" {
        0
    } else {
        1
    }
}
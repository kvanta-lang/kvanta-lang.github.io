mod utils;
mod compiler;
mod program;
mod execution;

use wasm_bindgen::prelude::*;

use crate::{execution::{Execution}, utils::{canvas::{construct_canvas, Canvas, CanvasReader}, message::CompilationMessage}};
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
pub struct Compiler {
    execution: Option<Execution>,
    canvas: Canvas,
    canvas_reader: CanvasReader

}

#[wasm_bindgen]
impl Compiler {
    pub fn new() -> Compiler {
        let (c, r) = construct_canvas();
        Compiler{execution: None, canvas: c, canvas_reader: r}
    }

    pub fn compile_code(&mut self, source : &str) -> CompilationMessage {
        self.compile(source)
    }

    pub fn execute(&self) -> String {
        if let Some(exec) = self.execution.clone() {
            return match exec.clone().execute() {
                Err(err) => {
                    format!("{}", err).into()
                },
                _ => String::from(""),
            }
        }
        String::from("")
    }

    pub fn get_commands(&mut self) -> Vec<String> {
        self.canvas_reader.get_commands()
    }
}

#[test]
    fn test_file() {
        let file_path = "../grammar/test.txt";

        let contents = std::fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        //assert!(contents.len() > 0);
        let result = Compiler::new().compile(&contents);
        println!("{}\n=====================================", result);
    }

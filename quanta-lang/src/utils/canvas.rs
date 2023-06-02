use quanta_parser::ast::BaseValue;
use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<i32>,
}

impl Default for Canvas {
    fn default() -> Canvas {
        let mut vec = Vec::new();
        vec.resize(600 * 420, 0);
        Canvas { width: 600, height: 420, pixels: vec }
    } 
}

impl Canvas {
    pub fn empty() -> Canvas {
        Canvas { width:0, height: 0, pixels: Vec::with_capacity(0) }
    }

    pub fn setPixel(&mut self, x : i32, y : i32, c : BaseValue) {
        if let BaseValue::Color(r, g, b) = c {
            let val:i32 = (r as i32) * 256 * 256 + (g as i32) * 256 + (b as i32);
            self.pixels[(y as usize) * self.width + (x as usize)] = val;
            return;
        }
        panic!("Not color in setPixel!");
    }
}


impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.pixels.as_slice().chunks(self.width as usize) {
            for &pixel in line {
                write!(f, "{}", pixel as usize)?;
            }
        }
        Ok(())
    }
}
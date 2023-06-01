use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

impl Default for Canvas {
    fn default() -> Canvas {
        Canvas { width: 600, height: 420, pixels: Vec::with_capacity(600*420) }
    } 
}

impl Canvas {
    pub fn empty() -> Canvas {
        Canvas { width:0, height: 0, pixels: Vec::with_capacity(0) }
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
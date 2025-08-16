use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Canvas {
    commands: Vec<String>,
}

impl Default for Canvas {
    fn default() -> Canvas {
        Canvas { commands: Vec::new() }
    } 
}

impl Canvas {
    pub fn empty() -> Canvas {
        Canvas::default()
    }

    pub fn add_command(&mut self, c : String) {
        self.commands.push(c);
    }

    pub fn get_commands(&self) -> Vec<String> {
        self.commands.clone()
    }
}


impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.commands {
            write!(f, "{}\n", line)?;
        }
        Ok(())
    }
}
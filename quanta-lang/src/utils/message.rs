use wasm_bindgen::prelude::*;
use crate::utils::canvas::{CanvasReader};
use quanta_parser::error::Error;

#[wasm_bindgen]
pub struct Message {
    pub error_code: u32,
    error_message: String,
    commands : Vec<String>
}

impl Default for Message {
    fn default() -> Message {
        Message {
            error_code: 0,
            error_message: "".to_string(),
            commands: vec![]
        }
    }
}

impl Message {
    pub fn create_error_message(error: Error) -> Message {
        match error {
            Error::ParseError { ref message } => {
                Message { error_code:1, error_message: message.to_string(), commands: vec![] }
            }
            Error::LogicError { ref message } => {
                Message { error_code:2, error_message: message.to_string(), commands: vec![] }
            }
            Error::TypeError { ref message } => {
                Message { error_code:3, error_message: message.to_string(), commands: vec![] }
            }
            Error::RuntimeError { ref message } => {
                Message { error_code:4, error_message: message.to_string(), commands: vec![] }
            }
        }
    }

    pub fn from_canvas(mut canvas: CanvasReader) -> Message {
        Message { error_code:0, error_message: String::from(""), commands: canvas.get_commands() }
    }

    pub fn get_commands(&self) -> Vec<String>
    {
        self.commands.clone()
    }

    pub fn get_error_message(&self) -> String
    {
        self.error_message.clone()
    }
}

use std::fmt;

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.error_code as usize)?;
        write!(f, "{}\n", self.error_message)?;
        for command in &self.commands {
            write!(f, "{}\n", command)?;
        }
        Ok(())
    }
}
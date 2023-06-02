use wasm_bindgen::prelude::*;
use crate::utils::canvas::Canvas;
use quanta_parser::error::Error;

#[wasm_bindgen]
pub struct Message {
    pub error_code: u32,
    error_message: String,
    canvas : Canvas
}

impl Default for Message {
    fn default() -> Message {
        Message {
            error_code: 0,
            error_message: "".to_string(),
            canvas: Canvas::default()
        }
    }
}

impl Message {
    pub fn create_error_message(error: Error) -> Message {
        match error {
            Error::ParseError { ref message } => {
                Message { error_code:1, error_message: message.to_string(), canvas: Canvas::empty() }
            }
            Error::LogicError { ref message } => {
                Message { error_code:2, error_message: message.to_string(), canvas: Canvas::empty() }
            }
            Error::TypeError { ref message } => {
                Message { error_code:3, error_message: message.to_string(), canvas: Canvas::empty() }
            }
            Error::RuntimeError { ref message } => {
                Message { error_code:4, error_message: message.to_string(), canvas: Canvas::empty() }
            }
        }
    }

    pub fn from_canvas(canvas: Canvas) -> Message {
        Message { error_code:0, error_message: String::from(""), canvas }
    }
}

use std::fmt;

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.error_code as usize)?;
        write!(f, "{}\n", self.error_message)?;
        write!(f, "{}", self.canvas)?;
        Ok(())
    }
}
use wasm_bindgen::prelude::*;
use quanta_parser::error::Error;

#[wasm_bindgen]
pub struct CompilationMessage {
    pub error_code: u32,
    error_message: String
}

impl Default for CompilationMessage {
    fn default() -> CompilationMessage {
        CompilationMessage {
            error_code: 0,
            error_message: "".to_string()
        }
    }
}

impl CompilationMessage {
    pub fn create_error_message(error: Error) -> CompilationMessage {
        match error {
            Error::ParseError { ref message } => {
                CompilationMessage { error_code:1, error_message: message.to_string() }
            }
            Error::LogicError { ref message } => {
                CompilationMessage { error_code:2, error_message: message.to_string() }
            }
            Error::TypeError { ref message } => {
                CompilationMessage { error_code:3, error_message: message.to_string() }
            }
            Error::RuntimeError { ref message } => {
                CompilationMessage { error_code:4, error_message: message.to_string() }
            }
        }
    }

    pub fn get_error_message(&self) -> String
    {
        self.error_message.clone()
    }
}

use std::fmt;

impl fmt::Display for CompilationMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.error_code as usize)?;
        write!(f, "{}\n", self.error_message)?;
        Ok(())
    }
}
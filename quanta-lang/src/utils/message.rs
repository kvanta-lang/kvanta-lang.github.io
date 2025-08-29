use std::fmt;

use wasm_bindgen::prelude::*;
use quanta_parser::{error::Error};
use crate::{linear_runtime, runtime::Runtime};


#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct CommandBlock{
    commands: Vec<String>,
    pub sleep_for: i32,
    pub should_draw_frame: bool
}

#[wasm_bindgen]
impl CommandBlock {
    pub fn get_commands(&self) -> Vec<String> {
        self.commands.clone()
    }
}

impl CommandBlock {
    pub fn new() -> CommandBlock {
        CommandBlock { commands: vec![], sleep_for: 0, should_draw_frame: false }
    }

    pub fn push(&mut self, command: String) -> CommandBlock {
        self.commands.push(command);
        self.clone()
    }
}


#[wasm_bindgen]
pub struct CompilationMessage {
    pub error_code: u32,
    error_message: String,
    runtime: Option<Runtime>
}


pub struct LinearCompilationMessage {
    pub error_code: u32,
    error_message: String,
    runtime: Option<linear_runtime::Runtime>
}

#[wasm_bindgen]
impl CompilationMessage {
    #[wasm_bindgen]
    pub fn get_runtime(&self) -> Runtime {
        self.runtime.clone().unwrap()
    }

    #[wasm_bindgen]
    pub fn get_error_message(&self) -> String {
        self.error_message.clone()
    }
}

impl CompilationMessage {

    pub(crate) fn ok(runtime: Runtime) -> CompilationMessage {
        CompilationMessage {
            error_code: 0,
            error_message: "".to_string(),
            runtime: Some(runtime)
        }
    }

    pub(crate) fn create_error_message(error: Error) -> CompilationMessage {
        match error {
            Error::ParseError { ref message } => {
                CompilationMessage { error_code:1, error_message: message.to_string(), runtime: None }
            }
            Error::LogicError { ref message } => {
                CompilationMessage { error_code:2, error_message: message.to_string(), runtime: None }
            }
            Error::TypeError { ref message } => {
                CompilationMessage { error_code:3, error_message: message.to_string(), runtime: None }
            }
            Error::RuntimeError { ref message } => {
                CompilationMessage { error_code:4, error_message: message.to_string(), runtime: None }
            }
        }
    }
}

impl LinearCompilationMessage {


    pub fn get_runtime(&self) -> linear_runtime::Runtime {
        self.runtime.clone().unwrap()
    }

    pub fn get_error_message(&self) -> String {
        self.error_message.clone()
    }


    pub(crate) fn ok(runtime: linear_runtime::Runtime) -> LinearCompilationMessage {
        LinearCompilationMessage {
            error_code: 0,
            error_message: "".to_string(),
            runtime: Some(runtime)
        }
    }

    pub(crate) fn create_error_message(error: Error) -> LinearCompilationMessage {
        match error {
            Error::ParseError { ref message } => {
                LinearCompilationMessage { error_code:1, error_message: message.to_string(), runtime: None }
            }
            Error::LogicError { ref message } => {
                LinearCompilationMessage { error_code:2, error_message: message.to_string(), runtime: None }
            }
            Error::TypeError { ref message } => {
                LinearCompilationMessage { error_code:3, error_message: message.to_string(), runtime: None }
            }
            Error::RuntimeError { ref message } => {
                LinearCompilationMessage { error_code:4, error_message: message.to_string(), runtime: None }
            }
        }
    }
}



impl fmt::Display for CompilationMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.error_code as usize)?;
        write!(f, "{}\n", self.error_message)?;
        Ok(())
    }
}
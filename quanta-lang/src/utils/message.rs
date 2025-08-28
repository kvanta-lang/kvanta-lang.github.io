use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local};
use quanta_parser::error::Error;


#[wasm_bindgen]
#[derive(Clone)]
pub struct Runtime {
    execution: Execution,
    canvas: CanvasReader
}

#[wasm_bindgen]
#[derive(Clone)]
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
    fn new() -> CommandBlock {
        CommandBlock { commands: vec![], sleep_for: 0, should_draw_frame: false }
    }

    fn push(&mut self, command: String) -> CommandBlock {
        self.commands.push(command);
        self.clone()
    }
}

#[wasm_bindgen]
impl Runtime {
    pub fn execute(&self) {
        let new_exec = self.execution.clone();
        spawn_local(async move {
            match new_exec.clone().execute().await {
            Ok(_) => {},
            Err(err) => {
                panic!("Got error: {}", err);
            }
        }
        })
        
    }

    pub fn get_commands(&mut self) -> Vec<CommandBlock> {
        let mut result = vec![];
        let mut block = CommandBlock::new();
        
        for command in self.canvas.get_commands() {
            if command.starts_with("sleep") {
                let time = command.split(' ').collect::<Vec<&str>>().get(1).unwrap().parse::<i32>().unwrap(); //parse i32
                block.sleep_for = time;
                result.push(block);
                block = CommandBlock::new();
            } else if command.starts_with("frame") {
                block.should_draw_frame = true;
                result.push(block);
                block = CommandBlock::new();
            } else if command.starts_with("end") {
                block.sleep_for = -1;
                result.push(block);
                return result;
            } else {
                block.push(command);
            }
        }
        result.push(block);
        return result;
    }
}

impl Runtime {
    pub fn new(exec: Execution, canvas: CanvasReader) -> Runtime {
        Runtime { execution: exec, canvas: canvas }
    }
}

#[wasm_bindgen]
pub struct CompilationMessage {
    pub error_code: u32,
    error_message: String,
    runtime: Option<Runtime>
}

#[wasm_bindgen]
impl CompilationMessage {
    pub fn get_runtime(&self) -> Runtime {
        self.runtime.clone().unwrap()
    }

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

use std::fmt;

use crate::{execution::{Execution}, utils::canvas::CanvasReader};

impl fmt::Display for CompilationMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.error_code as usize)?;
        write!(f, "{}\n", self.error_message)?;
        Ok(())
    }
}
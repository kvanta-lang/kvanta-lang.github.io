use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local};
use quanta_parser::{ast::{BaseValue, Expression, Type}, error::Error};

use crate::{execution::{Execution, Scope}, program::Program, utils::{canvas::{Canvas, CanvasReader}, message::CommandBlock}};

use std::{collections::HashMap, fmt, sync::{Arc, Mutex}};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Runtime {
    execution: Execution,
    canvas: CanvasReader,
    global_vars : Arc<Mutex<HashMap<String, BaseValue>>>,
    global_var_definitions: Arc<Mutex<HashMap<String, (Type, Expression)>>>,
    figure_color : Arc<Mutex<String>>,
    line_color : Arc<Mutex<String>>,
    line_width : Arc<Mutex<i32>>,
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
    pub fn new(prog : Program, canv: Canvas, canvas: CanvasReader) -> Runtime {
        //let exec = Execution::from_program(prog.clone(), canv);
        let global_vars = Arc::new(Mutex::new(HashMap::new()));
        let global_var_defs = Arc::new(Mutex::new(prog.global_vars));
        let fig_col = Arc::new(Mutex::new(String::from("#ffffff")));
        let lin_col = Arc::new(Mutex::new(String::from("#ffffff")));
        let lin_wid = Arc::new(Mutex::new(1));

        let exec = Execution {
            lines : prog.lines.clone(),
            scope : Scope { variables: HashMap::new(), outer_scope: None },
            global_vars: Arc::clone(&global_vars),
            global_var_definitions: Arc::clone(&global_var_defs),
            canvas: canv,
            functions: prog.functions.clone(),
            figure_color: Arc::clone(&fig_col),
            line_color: Arc::clone(&lin_col),
            line_width: Arc::clone(&lin_wid),
        };
        Runtime { 
            execution: exec, 
            canvas: canvas, 
            global_vars: global_vars,
            global_var_definitions: global_var_defs,
            figure_color: fig_col,
            line_color: lin_col,
            line_width: lin_wid
        }
    }
}
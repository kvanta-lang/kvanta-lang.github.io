
use quanta_parser::{parse_ast};
use crate::execution::Execution;
use crate::program::create_program;
use crate::utils::canvas::construct_canvas;
use crate::utils::message::CompilationMessage;
use crate::{Compiler, runtime::Runtime};

impl Compiler {
    pub fn compile(&mut self, source : &str) -> CompilationMessage {
        match parse_ast(source) {
            Ok(ast) => {
                let mut program = create_program(ast);
                match program.type_check() {
                    Err(error) =>  {
                        CompilationMessage::create_error_message(error)
                    },
                    Ok(_) => {
                        let (c, r) = construct_canvas();
                        CompilationMessage::ok(Runtime::new(program, c, r))
                    }
                }
            },
            Err(err) => {
                CompilationMessage::create_error_message(err)
            }
        }
    }
}




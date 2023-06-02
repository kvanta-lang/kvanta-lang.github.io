use std::fs;
use std::sync::Arc;

use quanta_parser::error::Error;
use quanta_parser::{parse_ast};
use crate::program::{Program, create_program};
use crate::utils::message::Message;

pub fn compilation_result(source : &str) -> String {
    compile(source).to_string()
}

fn compile(source : &str) -> Message {
    match parse_ast(source) {
        Ok(ast) => {
            let mut program = create_program(ast);
            let result = program.type_check();
            match result {
                Some(error) => Message::create_error_message(error),
                None => Message::default(),
            }
        }
        Err(err) => Message::create_error_message(err)
    }
}

#[test]
    fn test_file() {
        let file_path = "../grammar/test.txt";

        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        assert!(contents.len() > 0);
        assert!(compile(&contents).error_code == 0);
    }
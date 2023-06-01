use quanta_parser::{parse_ast};
use quanta_parser::ast::builder::{logic_check, type_check};
use crate::utils::message::Message;

pub fn compilation_result(source : &str) -> String {
    compile(source).to_string()
}

fn compile(source : &str) -> Message {
    let message = Message::default();
    let result = parse_ast(source)
        .and_then(|ast| logic_check(ast))
        .and_then(|ast| type_check(ast));

    match result {
        Ok(ast) => message,
        Err(error) => Message::create_error_message(error),
    }


    // if let Err(text) = parsed_doc {
    //     return Message::parsing_error(text);
    // }
    // if let Err(text) = build_ast(parsed_doc.unwrap()) {
    //     return Message::
    // }
    // if let Err(text) = logic_check()
    //message
}
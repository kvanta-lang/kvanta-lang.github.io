
use quanta_parser::{parse_ast};
use crate::execution::Execution;
use crate::program::create_program;
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
                Some(error) =>  {
                    print!("COMPILATION ERROR: {}", error.to_string());
                    Message::create_error_message(error)
                },
                None => {
                    print!("COMPILATION SUCCESS!!!!!!!");
                    let mut exec = Execution::from_program(program);
                    exec.execute()
                }
            }
        }
        Err(err) => {
            print!("PARSING ERROR: {}", err.to_string());
            Message::create_error_message(err)
        }
    }
}

#[test]
    fn test_file() {
        let file_path = "../grammar/test.txt";

        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        //assert!(contents.len() > 0);
        let result = compile(&contents);
        //print!("{}\n=====================================", result);
    }
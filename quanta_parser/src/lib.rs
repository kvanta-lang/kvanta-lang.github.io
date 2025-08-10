
use ast::builder::build_ast_from_doc;
use ast::AstBlock;
use error::Error;
use pest::Parser;
use pest_derive::Parser;
pub mod ast;
pub mod error;

#[derive(Parser)]
#[grammar = "../grammar/grammar.pest"]
pub struct QuantaParser;

pub fn parse_ast(source : &str) -> Result<AstBlock, Error> {
    let parsed_doc = QuantaParser::parse(Rule::document, source);
    match parsed_doc {
        Ok(doc) => build_ast_from_doc(doc),
        Err(err) => Err(Error::from_pest_error(err))
    }
}


#[cfg(test)]
mod tests {
    use std::{fs};

    use super::*;

    #[test]
    fn it_works() {
        let text = "circle(320, 240, 100);";
        let wrong_text = "circle(320q, 240, 100);";
        assert!(parse_ast(text).is_ok());
        assert!(parse_ast(wrong_text).is_err());
    }

    #[test]
    fn test_file() {
        let file_path = "../grammar/test.txt";

        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        assert!(contents.len() > 0);
        let res = parse_ast(contents.as_str());
        match &res {
            Ok(_ast) => {},
            Err(Error::ParseError{message}) => {print!("{}", message.to_string())}
            Err(Error::LogicError{message}) => {print!("{}", message.to_string())}
            Err(Error::TypeError{message}) => {print!("{}", message.to_string())}
            Err(Error::RuntimeError{message}) => {print!("{}", message.to_string())}
        }
        assert!(res.is_ok());
    }
}

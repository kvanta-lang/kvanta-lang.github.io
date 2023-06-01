use ast::{AstBlock};
use error::Error;
use pest::Parser;
use pest::iterators::Pairs;
use pest_derive::Parser;
pub mod ast;
pub mod error;

#[derive(Parser)]
#[grammar = "../grammar/grammar.pest"]
pub struct QuantaParser;

pub fn parse_ast(source : &str) -> Result<AstBlock, Error> {
    let parsed_doc = QuantaParser::parse(Rule::document, source);
    match parsed_doc {
        Ok(doc) => build_ast(doc),
        Err(err) => Err(Error::from_pest_error(err))
    }
}

fn build_ast(pairs: Pairs<Rule>) -> Result<AstBlock<'static>, Error> {
    Ok(AstBlock{ nodes: Vec::with_capacity(0) })

}


// for pair in doc.into_iter() {
//     match pair.as_rule() {
//         Rule::block => {
//             for tag in pair.into_inner() {
//                 println!("{}", tag)
//             }
//         }
//         _ => {}
//     }
// }
// return true    


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "circle(320, 240, 100);";
        let wrong_text = "circle(320q, 240, 100);";
        assert!(parse_ast(text).is_ok());
        assert!(parse_ast(wrong_text).is_err());
    }
}

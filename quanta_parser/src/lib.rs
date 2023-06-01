use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../grammar/grammar.pest"]
pub struct HtmlParser;

pub fn parse_text(source : &str) -> bool {
    let parsed_doc = HtmlParser::parse(Rule::document, source);
    match parsed_doc {
        Ok(doc) => {
            for pair in doc.into_iter() {
                match pair.as_rule() {
                    Rule::block => {
                        for tag in pair.into_inner() {
                            println!("{}", tag)
                        }
                    }
                    _ => {}
                }
            }
            return true
        },
        Err(_) => return false
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let text = "circle(320, 240, 100);";
        let wrong_text = "circle(320q, 240, 100);";
        assert!(parse_text(text));
        assert!(!parse_text(wrong_text));
    }
}

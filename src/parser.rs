use tree_sitter::Parser;
use tree_sitter_quanta::get_language;

pub fn parse(source: &str) -> bool
{
    let language = get_language();
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let tree = parser.parse(source, None).unwrap();

    !tree.root_node().has_error()
}

#[test]
fn test_parser() {
    let language = get_language();
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    let source_code = "circle(320, 240, 100)";
    let tree = parser.parse(source_code, None).unwrap();

    assert!(tree.root_node().has_error());
    //assert_eq!(tree.root_node().to_sexp(), "(source_file (block (statement (command (identifier) (number (nat_number)) (number (nat_number)) (number (nat_number))))))");
}
use pest::iterators::Pairs;
use crate::Rule;
use crate::error::{Error};

use super::AstBlock;

pub fn build_ast(pair : Pairs<Rule>) -> Result<AstBlock, Error> {
    Ok(AstBlock { nodes: Vec::with_capacity(0) })
}

pub fn logic_check(ast : AstBlock) -> Result<AstBlock, Error> {
    Ok(AstBlock { nodes: Vec::with_capacity(0) })
}

pub fn type_check(ast: AstBlock) -> Result<AstBlock, Error> {
    Ok(AstBlock { nodes: Vec::with_capacity(0) })
}


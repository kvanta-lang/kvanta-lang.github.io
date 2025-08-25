use std::collections::HashMap;

use pest::iterators::{Pairs, Pair};
use crate::{ast::{AstFunction, AstProgram, FunctionsAndGlobals, HalfParsedAstFunction, SimpleExpression, SimpleValue, Type, TypeName, VariableCall}, error::Error, Rule};


use super::{AstBlock, AstNode, Expression, Operator,  BaseType, BaseValue, goes_before, UnaryOperator };



pub struct AstBuilder {
    pub function_signatures : HashMap<String, (Vec<Type>, Option<Type>)>
}

impl AstBuilder {

pub fn new() -> AstBuilder
{
    AstBuilder{ function_signatures: HashMap::new() }
}

pub fn build_ast_from_doc(&mut self, docs: Pairs<Rule>) -> Result<AstProgram, Error> {
    assert!(docs.len() == 1);
    let doc = docs.into_iter().next().unwrap();
    assert!(doc.as_rule() == Rule::document);

    let mut doc_iter = doc.into_inner().into_iter();
    assert!(doc_iter.len() == 2);
    let block_rule = doc_iter.next().unwrap();
    let eof_rule = doc_iter.next().unwrap();

    assert!(eof_rule.as_rule() == Rule::EOI);
    if block_rule.as_rule() == Rule::block {
        Ok(AstProgram::Block(self.build_ast_from_block(block_rule.into_inner())?))
    } else {
        Ok(AstProgram::Forest(self.build_ast_from_forest(block_rule.into_inner())?))
    }    
}

fn build_ast_from_forest(&mut self, statements: Pairs<Rule>) -> Result<FunctionsAndGlobals, Error> {
    let mut half_functions = vec![];
    let mut init_statements :HashMap<String, (Type, Expression)> = HashMap::new();
    let mut blocks : Vec<AstFunction> = vec![];
    for pair in statements.clone() {
        match pair.as_rule() {
            Rule::function => {
                let res = self.get_function_signature(pair.into_inner())?;
                self.function_signatures.insert(res.name.clone(), (res.args.iter().map(|(_, t)| t.clone()).collect(), res.return_type.clone()));
                half_functions.push(res);
            }
            Rule::global_block => {
                let mut iter = pair.into_inner().into_iter();
                
                while let Some(init) = iter.next() {
                    if init.as_rule() == Rule::strong_init {
                        let mut init_iter = init.into_inner().into_iter();
                        let type_name = self.build_ast_from_type(init_iter.next().unwrap())?;
                        let mut init_iter2 = init_iter.next().unwrap().into_inner().into_iter();
                        let name = self.build_ast_from_ident(init_iter2.next().unwrap())?;
                        let expr = self.build_ast_from_expression(init_iter2.next().unwrap())?;
                        init_statements.insert(name, (type_name, expr));
                    } else {
                        return Err(Error::ParseError { message: format!("Expected global variable initialization, found: {:?}", init.as_rule()).into() });
                    }
                }
            },
            _ => return Err(Error::ParseError { message: format!("Expected a function at: {:?}", pair.as_rule()).into() })
        }
    }
    for func in half_functions {
        blocks.push(self.build_ast_from_function(func)?);
    }
    Ok((blocks, init_statements))
}

fn build_ast_from_function(&self, function: HalfParsedAstFunction) -> Result<AstFunction, Error> {
    let body = self.build_ast_from_block(function.statements)?;
    Ok(AstFunction{name: function.name, args: function.args, return_type: function.return_type, block: body})
}


fn get_function_signature<'a>(&self, statement: Pairs<'a, Rule>) -> Result<HalfParsedAstFunction<'a>, Error> {
    let mut iter = statement.into_iter();
    let header = iter.next().unwrap();
    assert!(header.as_rule() == Rule::fn_header);
    let body = iter.next().unwrap();
    assert!(body.as_rule() == Rule::block);

    let mut header_iter = header.into_inner().into_iter();
    let name = self.build_ast_from_ident(header_iter.next().unwrap())?;
    let mut args = vec![];
    let args_iter = header_iter.next().unwrap().into_inner().into_iter();
    for arg in args_iter {
        let mut arg_iter = arg.into_inner().into_iter();
        let arg_type = self.build_ast_from_type(arg_iter.next().unwrap())?;
        let arg_name = self.build_ast_from_ident(arg_iter.next().unwrap())?;
        args.push((arg_name, arg_type));
    }
    let typer = {
        if let Some(x) = header_iter.next() {
            assert!(x.as_rule() == Rule::type_name);
            let typed = self.build_ast_from_type(x)?;
            Some(typed)
        } else {
            None
        }
    };
    
    Ok(HalfParsedAstFunction { 
        name, 
        args, 
        return_type: typer, 
        statements: body.into_inner()
    })

}

fn build_ast_from_block(&self, statements: Pairs<Rule>) -> Result<AstBlock, Error> {
    let mut block = AstBlock{ nodes: vec![] };
    for pair in statements {
        match pair.as_rule() {
            Rule::statement => {
                block.nodes.push(self.build_ast_from_statement(pair.into_inner())?);
            }
            _ => unreachable!("Unexpected code 6")
        }
    }
    Ok(block)
}

fn build_ast_from_statement(&self, statement: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = statement.into_iter();
    let state = iter.next().unwrap();
    match state.as_rule() {
        Rule::command => self.build_ast_from_command(state.into_inner()),
        Rule::init_statement => self.build_ast_from_init(state.into_inner()),
        Rule::if_statement => self.build_ast_from_if(state.into_inner()),
        Rule::for_statement => self.build_ast_from_for(state.into_inner()),
        Rule::while_statement => self.build_ast_from_while(state.into_inner()),
        Rule::return_statement => {
            let expr = self.build_ast_from_expression(state.into_inner().into_iter().next().unwrap())?;
            Ok(AstNode::Return { expr: expr })
        }
        _ => unreachable!("Unexpected code 7")
    }
}

fn build_ast_from_command(&self, command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter().next().unwrap().into_inner().into_iter();
    let name = self.build_ast_from_ident(iter.next().unwrap())?;
    let args = self.build_ast_from_arglist(iter)?;
    return Ok(AstNode::Command { 
        name: name,
        args: args
    });
}

fn build_ast_from_ident(&self, ident: Pair<Rule>) -> Result<String, Error> {
    Ok(String::from(ident.as_str().trim()))
}

fn build_ast_from_noun(&self, ident: Pair<Rule>) -> Result<VariableCall, Error> {
    if ident.as_rule() == Rule::noun {
        let mut ident = ident.into_inner().into_iter();
        if let Some(name) = ident.next() {
            if ident.clone().count() > 0 {
                let mut args = vec![];
                for arg in ident {
                    args.push(self.build_ast_for_simple_expression(arg.into_inner().into_iter().next().unwrap())?);
                }
                return Ok(VariableCall::ArrayCall(String::from(name.as_str()), args));
            }
            return Ok(VariableCall::Name(String::from(name.as_str())));
        }
        return Ok(VariableCall::Name(String::from(ident.as_str())));
    }
    Err(Error::ParseError { message: format!("Expected identifier, found: {}", ident.as_str()).into() })
}

fn build_ast_from_arglist(&self, args: Pairs<Rule>) -> Result<Vec<Expression>, Error> {
    let mut expressions = vec![];
    for pair in args {
        expressions.push(self.build_ast_from_expression(pair)?);
    }
    Ok(expressions)
}

fn improve_expr(&self, expr : Expression) -> (Expression, bool) {
    match expr {
        Expression::Value(_) => (expr, false),
        Expression::Unary(_, _) => (expr, false),
        Expression::Binary(op, left, right) => {
            let mut new_left : Expression = *left;
            if let Expression::Unary(UnaryOperator::Parentheses, _) = new_left {
                (new_left, _) = self.improve_expr(new_left);
            }
            if let Expression::Binary(r_op, r_left, r_right) = *right.clone() {
                if goes_before(op, r_op) {
                    return self.improve_expr(Expression::Binary(r_op, Expression::Binary(op, new_left.into(), r_left).into(), r_right))
                } else {
                    let (new_right, redo) = self.improve_expr(*right);
                    if redo {
                        return self.improve_expr(Expression::Binary(op, new_left.into(), new_right.into()));
                    }
                    return (Expression::Binary(op, new_left.into(), new_right.into()), false)
                }
            }
            (Expression::Binary(op, new_left.into(), right), false)
        },
    }
}

// todo remove separation for simple expressions
fn improve_simple_expr(&self, expr : SimpleExpression) -> (SimpleExpression, bool) {
    match expr {
        SimpleExpression::Value(_) => (expr, false),
        SimpleExpression::Unary(_, _) => (expr, false),
        SimpleExpression::Binary(op, left, right) => {
            let mut new_left : SimpleExpression = *left;
            if let SimpleExpression::Unary(UnaryOperator::Parentheses, _) = new_left {
                (new_left, _) = self.improve_simple_expr(new_left);
            }
            if let SimpleExpression::Binary(r_op, r_left, r_right) = *right.clone() {
                if goes_before(op, r_op) {
                    return self.improve_simple_expr(SimpleExpression::Binary(r_op, SimpleExpression::Binary(op, new_left.into(), r_left).into(), r_right))
                } else {
                    let (new_right, redo) = self.improve_simple_expr(*right);
                    if redo {
                        return self.improve_simple_expr(SimpleExpression::Binary(op, new_left.into(), new_right.into()));
                    }
                    return (SimpleExpression::Binary(op, new_left.into(), new_right.into()), false)
                }
            }
            (SimpleExpression::Binary(op, new_left.into(), right), false)
        },
    }
}

fn build_ast_for_simple_expression(&self, expression : Pair<Rule>) -> Result<SimpleExpression, Error> {
    let expr = self.build_ast_from_simple_expression_inner(expression)?;
    let (res, _) = self.improve_simple_expr(expr);
    Ok(res)
}

fn build_ast_from_simple_expression_inner(&self, expression: Pair<Rule>) -> Result<SimpleExpression, Error> {
    match expression.as_rule() {
        Rule::monadicExpr => {
            let mut iter = expression.into_inner().into_iter();
            let operator = iter.next().unwrap();
            let right = self.build_ast_from_simple_expression_inner(iter.next().unwrap())?;
            if operator.as_str() == "-" {
                Ok(SimpleExpression::Unary(super::UnaryOperator::UnaryMinus, right.into()))
            } else {
                Err(Error::ParseError { message: format!("Unknown unary operator {}", operator.as_str()).into() })
            }
        },
        Rule::dyadicExpr => {
            let mut iter = expression.into_inner().into_iter();
            let left = self.build_ast_from_simple_expression_inner(iter.next().unwrap())?;
            let operator = iter.next().unwrap();
            let right = self.build_ast_from_simple_expression_inner(iter.next().unwrap())?;
            match operator.as_str() {
                "+" => Ok(SimpleExpression::Binary(Operator::Plus, left.into(), right.into())),
                "-" => Ok(SimpleExpression::Binary(Operator::Minus, left.into(), right.into())),
                "*" => Ok(SimpleExpression::Binary(Operator::Mult, left.into(), right.into())),
                "/" => Ok(SimpleExpression::Binary(Operator::Div, left.into(), right.into())),
                "%" => Ok(SimpleExpression::Binary(Operator::Mod, left.into(), right.into())),

                ">"   => Ok(SimpleExpression::Binary(Operator::GT, left.into(), right.into())),
                "<"   => Ok(SimpleExpression::Binary(Operator::LT, left.into(), right.into())),
                ">="  => Ok(SimpleExpression::Binary(Operator::GQ, left.into(), right.into())),
                "<="  => Ok(SimpleExpression::Binary(Operator::LQ, left.into(), right.into())),
                "=="  => Ok(SimpleExpression::Binary(Operator::EQ, left.into(), right.into())),
                "!="  => Ok(SimpleExpression::Binary(Operator::NQ, left.into(), right.into())),

                "&&"  => Ok(SimpleExpression::Binary(Operator::AND, left.into(), right.into())),
                "||"  => Ok(SimpleExpression::Binary(Operator::OR, left.into(), right.into())),

                op => Err(Error::ParseError { message: format!("Unknown operator {}", op).into() })
            }
        },
        Rule::expression => {
            return self.build_ast_from_simple_expression_inner(expression.into_inner().into_iter().next().unwrap())
        },
        Rule::parenth_expr => {
            let inner_expr = self.build_ast_from_simple_expression_inner(expression.into_inner().into_iter().next().unwrap().into_inner().into_iter().next().unwrap())?;
            Ok(SimpleExpression::Unary(super::UnaryOperator::Parentheses, inner_expr.into()))
        },
        _ => {
            return Ok(SimpleExpression::Value(self.build_ast_from_simple_value(expression)?))
        }
    }


}

fn build_ast_from_expression(&self, expression: Pair<Rule>) -> Result<Expression, Error> {
    let expr = self.build_ast_from_expression_inner(expression)?;
    let (res, _) = self.improve_expr(expr);
    Ok(res)
}

fn build_ast_from_expression_inner(&self, expression: Pair<Rule>) -> Result<Expression, Error> {
    match expression.as_rule() {
        Rule::monadicExpr => {
            let mut iter = expression.into_inner().into_iter();
            let operator = iter.next().unwrap();
            let right = self.build_ast_from_expression_inner(iter.next().unwrap())?;
            if operator.as_str() == "-" {
                Ok(Expression::Unary(super::UnaryOperator::UnaryMinus, right.into()))
            } else {
                Err(Error::ParseError { message: format!("Unknown unary operator {}", operator.as_str()).into() })
            }
        },
        Rule::dyadicExpr => {
            let mut iter = expression.into_inner().into_iter();
            let left = self.build_ast_from_expression_inner(iter.next().unwrap())?;
            let operator = iter.next().unwrap();
            let right = self.build_ast_from_expression_inner(iter.next().unwrap())?;
            match operator.as_str() {
                "+" => Ok(Expression::Binary(Operator::Plus, left.into(), right.into())),
                "-" => Ok(Expression::Binary(Operator::Minus, left.into(), right.into())),
                "*" => Ok(Expression::Binary(Operator::Mult, left.into(), right.into())),
                "/" => Ok(Expression::Binary(Operator::Div, left.into(), right.into())),
                "%" => Ok(Expression::Binary(Operator::Mod, left.into(), right.into())),

                ">"   => Ok(Expression::Binary(Operator::GT, left.into(), right.into())),
                "<"   => Ok(Expression::Binary(Operator::LT, left.into(), right.into())),
                ">="  => Ok(Expression::Binary(Operator::GQ, left.into(), right.into())),
                "<="  => Ok(Expression::Binary(Operator::LQ, left.into(), right.into())),
                "=="  => Ok(Expression::Binary(Operator::EQ, left.into(), right.into())),
                "!="  => Ok(Expression::Binary(Operator::NQ, left.into(), right.into())),

                "&&"  => Ok(Expression::Binary(Operator::AND, left.into(), right.into())),
                "||"  => Ok(Expression::Binary(Operator::OR, left.into(), right.into())),

                op => Err(Error::ParseError { message: format!("Unknown operator {}", op).into() })
            }
        },
        Rule::expression => {
            return self.build_ast_from_expression_inner(expression.into_inner().into_iter().next().unwrap())
        },
        Rule::parenth_expr => {
            let inner_expr = self.build_ast_from_expression_inner(expression.into_inner().into_iter().next().unwrap().into_inner().into_iter().next().unwrap())?;
            Ok(Expression::Unary(super::UnaryOperator::Parentheses, inner_expr.into()))
        },
        _ => {
            return Ok(Expression::Value(self.build_ast_from_value(expression)?))
        }
    }


}

fn build_ast_from_init(&self, command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    let mut first = iter.next().unwrap();
    if let Rule::type_name = first.as_rule() {
        let type_val = self.build_ast_from_type(first)?;
        first = iter.next().unwrap();
        let mut assign = first.into_inner().into_iter();
        let name = self.build_ast_from_ident(assign.next().unwrap())?;
        let expr = self.build_ast_from_expression(assign.next().unwrap())?;
        return Ok(AstNode::Init { typ: type_val, val: name, expr });
    } 
    let mut assign = first.into_inner().into_iter();
    let name = self.build_ast_from_noun(assign.next().unwrap())?;
    let expr = self.build_ast_from_expression(assign.next().unwrap())?;

    Ok(AstNode::SetVal { val: name, expr })
}

fn build_ast_from_if(&self, command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    return Ok(AstNode::If { 
        clause: self.build_ast_from_expression(iter.next().unwrap())?, 
        block: self.build_ast_from_block(iter.next().unwrap().into_inner().into_iter().next().unwrap().into_inner())?,
        else_block: iter.next().and_then(|rule| Some(self.build_ast_from_block(rule.into_inner().into_iter().next().unwrap().into_inner()).unwrap()))
    })
}

fn build_ast_from_for(&self, command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    let name = iter.next().unwrap();
    let mut range = iter.next().unwrap().into_inner().into_iter();
    Ok(AstNode::For { 
        val:  self.build_ast_from_ident(name).unwrap(), 
        from: self.build_ast_from_value(range.next().unwrap())?, 
        to: self.build_ast_from_value(range.next().unwrap())?,
        block: self.build_ast_from_block(iter.next().unwrap().into_inner().into_iter().next().unwrap().into_inner())?
    })
}

fn build_ast_from_value(&self, val: Pair<Rule>) -> Result<BaseValue, Error> {
    match val.as_rule() {
        Rule::integer => Ok(BaseValue::Int(val.as_str().parse::<i32>().unwrap())),
        Rule::decimal => Ok(BaseValue::Float(val.as_str().parse::<f32>().unwrap())),
        Rule::boolean => Ok(BaseValue::Bool(val.as_str() == "true")),
        Rule::color   => self.build_ast_from_color(val),
        Rule::noun   => Ok(BaseValue::Id(self.build_ast_from_noun(val)?)),
        Rule::array_literal => {
            let mut elements = vec![];
            for item in val.into_inner() {
                elements.push(self.build_ast_from_value(item)?);
            }
            Ok(BaseValue::Array(elements))
        },
        Rule::function_call => {
            let mut iter = val.into_inner().into_iter();
            let name = self.build_ast_from_ident(iter.next().unwrap())?;
            let args = self.build_ast_from_arglist(iter)?;
            if let Some((_, return_type)) = self.function_signatures.get(&name) {
                if let Some(typ) = return_type {
                    Ok(BaseValue::FunctionCall(name, args, typ.clone()))
                } else {
                    Err(Error::TypeError { message: format!("Function {} has no return type", name).into() })
                }
            } else {
                Err(Error::TypeError { message: format!("Unknown function {}", name).into() })
            }
        }
        _ => unreachable!("Unexpected code 8")
    }
}

fn build_ast_from_simple_value(&self, val: Pair<Rule>) -> Result<SimpleValue, Error> {
    match val.as_rule() {
        Rule::integer => Ok(SimpleValue::Int(val.as_str().parse::<i32>().unwrap())),
        Rule::noun   => Ok(SimpleValue::Id(self.build_ast_from_noun(val)?)),
        _ => unreachable!("Unexpected code 9")
    }
}

fn build_ast_from_color(&self, val: Pair<Rule>) -> Result<BaseValue, Error> {
    match val.as_str() {
        "Color::Red" => Ok(BaseValue::Color(233,35,49)),
        "Color::Green" => Ok(BaseValue::Color(126,183,134)),
        "Color::Blue" => Ok(BaseValue::Color(46,115,230)),
        "Color::Yellow" => Ok(BaseValue::Color(253,226,93)),
        "Color::Pink" => Ok(BaseValue::Color(251,154,181)),
        "Color::Cyan" => Ok(BaseValue::Color(59,168,231)),
        "Color::Black" => Ok(BaseValue::Color(0, 0, 0)),
        "Color::White" => Ok(BaseValue::Color(255, 255, 255)),
        "Color::Random" => Ok(BaseValue::RandomColor),
        _ => Err(Error::ParseError { message: format!("Unknown color: {}", val.as_str()).into() })
    }
}

fn build_ast_from_while(&self, command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    Ok(AstNode::While { 
        clause: self.build_ast_from_expression(iter.next().unwrap())?, 
        block: self.build_ast_from_block(iter.next().unwrap().into_inner().into_iter().next().unwrap().into_inner())?,
    })
}

fn build_ast_from_array_type(&self, type_val: Pairs<Rule>) -> Result<TypeName, Error> {
    let mut iter = type_val.into_iter().next().unwrap().into_inner().into_iter();
    let inner_type = self.build_ast_from_type(iter.next().unwrap())?;
    if let BaseValue::Int(array_size) = self.build_ast_from_value(iter.next().unwrap())? {
        if array_size <= 0 {
            return Err(Error::ParseError { message: "Array size must be greater than 0".into() });
        }
        return Ok(TypeName::Array(Box::new(Some(inner_type)), array_size as usize));
    } else {
        return Err(Error::ParseError { message: "Expected integer for array size".into() });
    }
}

fn build_ast_from_inner_type(&self, type_val: Pairs<Rule>) -> Result<TypeName, Error> {
    use BaseType::*;
    if let Some(i) = type_val.clone().next() {
        if i.as_rule() == Rule::array_type {
            return self.build_ast_from_array_type(type_val);
        }
    }
    match type_val.as_str() {
        "int" => Ok(TypeName::Primitive(Int)),
        "bool" => Ok(TypeName::Primitive(Bool)),
        "color" => Ok(TypeName::Primitive(Color)),
        "float" => Ok(TypeName::Primitive(Float)),
        t => Err(Error::ParseError { message: format!("Unknown type 22: {}", t).into() })
    }
}

fn build_ast_from_type(&self, type_val: Pair<Rule>) -> Result<Type, Error> {
    let mut whole = type_val.clone().into_inner().into_iter();
    if let Some(first) = whole.next(){
        if first.as_rule() == Rule::const_key {
            return Ok(Type{type_name: self.build_ast_from_inner_type(whole)?, is_const: true});
        }
    }
    Ok(Type{type_name: self.build_ast_from_inner_type(type_val.into_inner().into_iter())?, is_const: false})

}

}

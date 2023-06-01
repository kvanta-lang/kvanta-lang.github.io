use pest::iterators::{Pairs, Pair};
use crate::Rule;
use crate::error::{Error};

use super::{AstBlock, AstNode, Expression, Operator,  BaseType, BaseValue, goes_before, UnaryOperator };

pub fn build_ast_from_doc(docs: Pairs<Rule>) -> Result<AstBlock, Error> {
    assert!(docs.len() == 1);
    let doc = docs.into_iter().next().unwrap();
    assert!(doc.as_rule() == Rule::document);

    let mut docIter = doc.into_inner().into_iter();
    assert!(docIter.len() == 2);
    let blockRule = docIter.next().unwrap();
    let eofRule = docIter.next().unwrap();

    assert!(blockRule.as_rule() == Rule::block);
    assert!(eofRule.as_rule() == Rule::EOI);

    let block = build_ast_from_block(blockRule.into_inner());
    println!("{:?}", block.as_ref().unwrap());
    block
}

fn build_ast_from_block(statements: Pairs<Rule>) -> Result<AstBlock, Error> {
    let mut block = AstBlock{ nodes: vec![] };
    for pair in statements {
        match pair.as_rule() {
            Rule::statement => {
                block.nodes.push(build_ast_from_statement(pair.into_inner())?);
            }
            _ => unreachable!()
        }
    }
    Ok(block)
}

fn build_ast_from_statement(statement: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = statement.into_iter();
    let state = iter.next().unwrap();
    match state.as_rule() {
        Rule::command => build_ast_from_command(state.into_inner()),
        Rule::init_statement => build_ast_from_init(state.into_inner()),
        Rule::if_statement => build_ast_from_if(state.into_inner()),
        Rule::for_statement => build_ast_from_for(state.into_inner()),
        Rule::while_statement => build_ast_from_while(state.into_inner()),
        _ => unreachable!()
    }
}

fn build_ast_from_command(command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    return Ok(AstNode::Command { 
        name: build_ast_from_ident(iter.next().unwrap())?,
        args: build_ast_from_arglist(iter).unwrap()
    });
}

fn build_ast_from_ident(ident: Pair<Rule>) -> Result<String, Error> {
    Ok(String::from(ident.as_str()))
}

fn build_ast_from_arglist(mut args: Pairs<Rule>) -> Result<Vec<Expression>, Error> {
    let mut expressions = vec![];
    for pair in args {
        expressions.push(build_ast_from_expression(pair).unwrap());
    }
    Ok(expressions)
}

fn improve_expr(expr : Expression) -> (Expression, bool) {
    match expr {
        Expression::Value(_) => (expr, false),
        Expression::Unary(_, _) => (expr, false),
        Expression::Binary(op, left, right) => {
            let mut new_left : Expression = *left;
            if let Expression::Unary(UnaryOperator::Parentheses, _) = new_left {
                (new_left, _) = improve_expr(new_left);
            }
            if let Expression::Binary(r_op, r_left, r_right) = *right.clone() {
                if goes_before(op, r_op) {
                    return improve_expr(Expression::Binary(r_op, Expression::Binary(op, new_left.into(), r_left).into(), r_right))
                } else {
                    let (new_right, redo) = improve_expr(*right);
                    if redo {
                        return improve_expr(Expression::Binary(op, new_left.into(), new_right.into()));
                    }
                    return (Expression::Binary(op, new_left.into(), new_right.into()), false)
                }
            }
            (Expression::Binary(op, new_left.into(), right), false)
        },
    }
}

fn build_ast_from_expression(expression: Pair<Rule>) -> Result<Expression, Error> {
    let expr = build_ast_from_expression_inner(expression)?;
    let (res, _) = improve_expr(expr);
    Ok(res)
}

fn build_ast_from_expression_inner(expression: Pair<Rule>) -> Result<Expression, Error> {
    match expression.as_rule() {
        Rule::monadicExpr => {
            let mut iter = expression.into_inner().into_iter();
            let operator = iter.next().unwrap();
            let right = build_ast_from_expression_inner(iter.next().unwrap())?;
            if operator.as_str() == "-" {
                Ok(Expression::Unary(super::UnaryOperator::UnaryMinus, right.into()))
            } else {
                Err(Error::ParseError { message: format!("Unknown unary operator {}", operator.as_str()).into() })
            }
        },
        Rule::dyadicExpr => {
            let mut iter = expression.into_inner().into_iter();
            let left = build_ast_from_expression_inner(iter.next().unwrap())?;
            let operator = iter.next().unwrap();
            let right = build_ast_from_expression_inner(iter.next().unwrap())?;
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
            return build_ast_from_expression_inner(expression.into_inner().into_iter().next().unwrap())
        },
        Rule::parenth_expr => {
            let inner_expr = build_ast_from_expression_inner(expression.into_inner().into_iter().next().unwrap().into_inner().into_iter().next().unwrap())?;
            Ok(Expression::Unary(super::UnaryOperator::Parentheses, inner_expr.into()))
        },
        _ => {
            return Ok(Expression::Value(build_ast_from_value(expression)?))
        }
    }


}

fn build_ast_from_init(command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    let mut first = iter.next().unwrap();
    let mut type_val : Option<BaseType> = None;
    if let Rule::type_name = first.as_rule() {
        type_val = Some(build_ast_from_type(first).unwrap());
        first = iter.next().unwrap();
    } 
    let mut assign = first.into_inner().into_iter();
    let name = build_ast_from_ident(assign.next().unwrap())?;
    let expr = build_ast_from_expression(assign.next().unwrap())?;

    Ok(AstNode::Init { typ: type_val, val: name, expr })
}

fn build_ast_from_if(command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    return Ok(AstNode::If { 
        clause: build_ast_from_expression(iter.next().unwrap())?, 
        block: build_ast_from_block(iter.next().unwrap().into_inner().into_iter().next().unwrap().into_inner())?,
        else_block: iter.next().and_then(|rule| Some(build_ast_from_block(rule.into_inner().into_iter().next().unwrap().into_inner()).unwrap()))
    })
}

fn build_ast_from_for(command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    let name = iter.next().unwrap();
    let mut range = iter.next().unwrap().into_inner().into_iter();
    Ok(AstNode::For { 
        val:  build_ast_from_ident(name).unwrap(), 
        from: build_ast_from_value(range.next().unwrap())?, 
        to: build_ast_from_value(range.next().unwrap())?,
        block: build_ast_from_block(iter.next().unwrap().into_inner().into_iter().next().unwrap().into_inner())?
    })
}

fn build_ast_from_value(val: Pair<Rule>) -> Result<BaseValue, Error> {
    match val.as_rule() {
        Rule::integer => Ok(BaseValue::Int(val.as_str().parse::<i32>().unwrap())),
        Rule::decimal => Ok(BaseValue::Float(val.as_str().parse::<f32>().unwrap())),
        Rule::boolean => Ok(BaseValue::Bool(val.as_str() == "true")),
        Rule::color   => build_ast_from_color(val),
        Rule::ident   => Ok(BaseValue::Id(build_ast_from_ident(val).unwrap())),
        _ => unreachable!()
    }
}

fn build_ast_from_color(val: Pair<Rule>) -> Result<BaseValue, Error> {
    match val.as_str() {
        "Color::Red" => Ok(BaseValue::Color(255, 0, 0)),
        "Color::Green" => Ok(BaseValue::Color(0, 255, 0)),
        "Color::Blue" => Ok(BaseValue::Color(0, 0, 255)),
        "Color::Yellow" => Ok(BaseValue::Color(255, 255, 0)),
        "Color::Pink" => Ok(BaseValue::Color(255, 0, 0)),
        "Color::Cyan" => Ok(BaseValue::Color(255, 0, 0)),
        _ => Err(Error::ParseError { message: format!("Unknown color: {}", val.as_str()).into() })
    }
}

fn build_ast_from_while(command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    return Ok(AstNode::While { 
        clause: build_ast_from_expression(iter.next().unwrap())?, 
        block: build_ast_from_block(iter.next().unwrap().into_inner().into_iter().next().unwrap().into_inner())?,
    })
}

fn build_ast_from_type(type_val: Pair<Rule>) -> Result<BaseType, Error> {
    match type_val.as_str() {
        "int" => Ok(BaseType::Int),
        "bool" => Ok(BaseType::Bool),
        "color" => Ok(BaseType::Color),
        "float" => Ok(BaseType::Float),
        _ => unreachable!()
    }
}

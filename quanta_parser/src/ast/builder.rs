use pest::iterators::{Pairs, Pair};
use crate::{ast::{SimpleExpression, SimpleValue, VariableCall}, error::Error, Rule};


use super::{AstBlock, AstNode, Expression, Operator,  BaseType, Type, BaseValue, goes_before, UnaryOperator };

pub fn build_ast_from_doc(docs: Pairs<Rule>) -> Result<AstBlock, Error> {
    assert!(docs.len() == 1);
    let doc = docs.into_iter().next().unwrap();
    assert!(doc.as_rule() == Rule::document);

    let mut doc_iter = doc.into_inner().into_iter();
    assert!(doc_iter.len() == 2);
    let block_rule = doc_iter.next().unwrap();
    let eof_rule = doc_iter.next().unwrap();

    assert!(block_rule.as_rule() == Rule::block);
    assert!(eof_rule.as_rule() == Rule::EOI);

    build_ast_from_block(block_rule.into_inner())
}

fn build_ast_from_block(statements: Pairs<Rule>) -> Result<AstBlock, Error> {
    let mut block = AstBlock{ nodes: vec![] };
    for pair in statements {
        match pair.as_rule() {
            Rule::statement => {
                block.nodes.push(build_ast_from_statement(pair.into_inner())?);
            }
            _ => unreachable!("Unexpected code 6")
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
        _ => unreachable!("Unexpected code 7")
    }
}

fn build_ast_from_command(command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    return Ok(AstNode::Command { 
        name: build_ast_from_ident(iter.next().unwrap())?,
        args: build_ast_from_arglist(iter)?
    });
}

fn build_ast_from_ident(ident: Pair<Rule>) -> Result<String, Error> {
    Ok(String::from(ident.as_str().trim()))
}

fn build_ast_from_noun(ident: Pair<Rule>) -> Result<VariableCall, Error> {
    if ident.as_rule() == Rule::noun {
        let mut ident = ident.into_inner().into_iter();
        if let Some(name) = ident.next() {
            if ident.clone().count() > 0 {
                let mut args = vec![];
                for arg in ident {
                    args.push(build_ast_for_simple_expression(arg.into_inner().into_iter().next().unwrap())?);
                }
                return Ok(VariableCall::ArrayCall(String::from(name.as_str()), args));
            }
            return Ok(VariableCall::Name(String::from(name.as_str())));
        }
        return Ok(VariableCall::Name(String::from(ident.as_str())));
    }
    Err(Error::ParseError { message: format!("Expected identifier, found: {}", ident.as_str()).into() })
}

fn build_ast_from_arglist(args: Pairs<Rule>) -> Result<Vec<Expression>, Error> {
    let mut expressions = vec![];
    for pair in args {
        expressions.push(build_ast_from_expression(pair)?);
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

// todo remove separation for simple expressions
fn improve_simple_expr(expr : SimpleExpression) -> (SimpleExpression, bool) {
    match expr {
        SimpleExpression::Value(_) => (expr, false),
        SimpleExpression::Unary(_, _) => (expr, false),
        SimpleExpression::Binary(op, left, right) => {
            let mut new_left : SimpleExpression = *left;
            if let SimpleExpression::Unary(UnaryOperator::Parentheses, _) = new_left {
                (new_left, _) = improve_simple_expr(new_left);
            }
            if let SimpleExpression::Binary(r_op, r_left, r_right) = *right.clone() {
                if goes_before(op, r_op) {
                    return improve_simple_expr(SimpleExpression::Binary(r_op, SimpleExpression::Binary(op, new_left.into(), r_left).into(), r_right))
                } else {
                    let (new_right, redo) = improve_simple_expr(*right);
                    if redo {
                        return improve_simple_expr(SimpleExpression::Binary(op, new_left.into(), new_right.into()));
                    }
                    return (SimpleExpression::Binary(op, new_left.into(), new_right.into()), false)
                }
            }
            (SimpleExpression::Binary(op, new_left.into(), right), false)
        },
    }
}

fn build_ast_for_simple_expression(expression : Pair<Rule>) -> Result<SimpleExpression, Error> {
    let expr = build_ast_from_simple_expression_inner(expression)?;
    let (res, _) = improve_simple_expr(expr);
    Ok(res)
}

fn build_ast_from_simple_expression_inner(expression: Pair<Rule>) -> Result<SimpleExpression, Error> {
    match expression.as_rule() {
        Rule::monadicExpr => {
            let mut iter = expression.into_inner().into_iter();
            let operator = iter.next().unwrap();
            let right = build_ast_from_simple_expression_inner(iter.next().unwrap())?;
            if operator.as_str() == "-" {
                Ok(SimpleExpression::Unary(super::UnaryOperator::UnaryMinus, right.into()))
            } else {
                Err(Error::ParseError { message: format!("Unknown unary operator {}", operator.as_str()).into() })
            }
        },
        Rule::dyadicExpr => {
            let mut iter = expression.into_inner().into_iter();
            let left = build_ast_from_simple_expression_inner(iter.next().unwrap())?;
            let operator = iter.next().unwrap();
            let right = build_ast_from_simple_expression_inner(iter.next().unwrap())?;
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
            return build_ast_from_simple_expression_inner(expression.into_inner().into_iter().next().unwrap())
        },
        Rule::parenth_expr => {
            let inner_expr = build_ast_from_simple_expression_inner(expression.into_inner().into_iter().next().unwrap().into_inner().into_iter().next().unwrap())?;
            Ok(SimpleExpression::Unary(super::UnaryOperator::Parentheses, inner_expr.into()))
        },
        _ => {
            return Ok(SimpleExpression::Value(build_ast_from_simple_value(expression)?))
        }
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
    if let Rule::type_name = first.as_rule() {
        let type_val = Some(build_ast_from_type(first)?);
        first = iter.next().unwrap();
        let mut assign = first.into_inner().into_iter();
        let name = build_ast_from_ident(assign.next().unwrap())?;
        let expr = build_ast_from_expression(assign.next().unwrap())?;
        return Ok(AstNode::Init { typ: type_val.unwrap(), val: name, expr });
    } 
    let mut assign = first.into_inner().into_iter();
    let name = build_ast_from_noun(assign.next().unwrap())?;
    let expr = build_ast_from_expression(assign.next().unwrap())?;

    Ok(AstNode::SetVal { val: name, expr })
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
        Rule::noun   => Ok(BaseValue::Id(build_ast_from_noun(val)?)),
        Rule::array_literal => {
            let mut elements = vec![];
            for item in val.into_inner() {
                elements.push(build_ast_from_value(item)?);
            }
            if elements.is_empty() {
                return Ok(BaseValue::Array(None, elements));
            }
            let first_type = elements[0].get_type();
            if elements.iter().any(|e| e.get_type() != first_type) {
                return Err(Error::TypeError { message: "All elements in the array must be of the same type".into() });
            }
            Ok(BaseValue::Array(Some(first_type), elements))
        }
        _ => unreachable!("Unexpected code 8")
    }
}

fn build_ast_from_simple_value(val: Pair<Rule>) -> Result<SimpleValue, Error> {
    match val.as_rule() {
        Rule::integer => Ok(SimpleValue::Int(val.as_str().parse::<i32>().unwrap())),
        Rule::noun   => Ok(SimpleValue::Id(build_ast_from_noun(val)?)),
        _ => unreachable!("Unexpected code 8")
    }
}

fn build_ast_from_color(val: Pair<Rule>) -> Result<BaseValue, Error> {
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

fn build_ast_from_while(command: Pairs<Rule>) -> Result<AstNode, Error> {
    let mut iter = command.into_iter();
    Ok(AstNode::While { 
        clause: build_ast_from_expression(iter.next().unwrap())?, 
        block: build_ast_from_block(iter.next().unwrap().into_inner().into_iter().next().unwrap().into_inner())?,
    })
}

fn build_ast_from_array_type(type_val: Pairs<Rule>) -> Result<Type, Error> {
    use Type::*;
    let mut iter = type_val.into_iter().next().unwrap().into_inner().into_iter();
    let inner_type = build_ast_from_type(iter.next().unwrap())?;
    //return Ok(Array(Box::new(Some(inner_type)), 3));
    if let BaseValue::Int(array_size) = build_ast_from_value(iter.next().unwrap())? {
        if array_size <= 0 {
            return Err(Error::ParseError { message: "Array size must be greater than 0".into() });
        }
        return Ok(Array(Box::new(Some(inner_type)), array_size as usize));
    } else {
        return Err(Error::ParseError { message: "Expected integer for array size".into() });
    }
}

fn build_ast_from_type(type_val: Pair<Rule>) -> Result<Type, Error> {
    use BaseType::*;
    use Type::*;

    if let Some(i) = type_val.clone().into_inner().into_iter().next() {
        if i.as_rule() == Rule::array_type {
            return build_ast_from_array_type(type_val.into_inner());
        }
    }
    match type_val.as_str() {
        "int" => Ok(Primitive(Int)),
        "bool" => Ok(Primitive(Bool)),
        "color" => Ok(Primitive(Color)),
        "float" => Ok(Primitive(Float)),
        t => Err(Error::ParseError { message: format!("Unknown type 22: {}", t).into() })
    }
}

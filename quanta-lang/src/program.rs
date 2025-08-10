use std::collections::HashMap;

use quanta_parser::{ast::*, error::Error};
use BaseType::*;
use Type::*;

#[derive(Debug, Clone)]
pub struct Program {
    pub lines: AstBlock, 
    pub variables : HashMap<String, (Type, Expression)>,
    pub functions : HashMap<String, Vec<Type>>
}

pub fn create_program(ast: AstBlock) -> Program {
    Program {lines: ast, variables: HashMap::new(), functions: HashMap::from([
        (String::from("circle"), vec![Primitive(Int), Primitive(Int), Primitive(Int)]),
        (String::from("line"), vec![Primitive(Int), Primitive(Int), Primitive(Int), Primitive(Int)]),
        (String::from("rectangle"), vec![Primitive(Int), Primitive(Int), Primitive(Int), Primitive(Int)]),
        (String::from("setLineColor"), vec![Primitive(Color)]),
        (String::from("setFigureColor"), vec![Primitive(Color)]),
        (String::from("setLineWidth"), vec![Primitive(Int)]),
        (String::from("polygon"), vec![]), // at least 6 Ints for polygon
        (String::from("arc"), vec![Primitive(Int), Primitive(Int), Primitive(Int), Primitive(Int), Primitive(Int)]),
    ])}
}

impl Program {
    pub fn type_check(&mut self) -> Option<Error> {
        let nodes = self.lines.nodes.clone();
        for line in nodes {
            match line {
                AstNode::Command { name, args } => {
                    if let Some(err) = self.clone().type_check_command(name.clone(), args.clone()) {
                        return Some(err);
                    }
                },
                AstNode::Init { typ, val, expr } => {
                     match self.clone().type_check_init(typ.clone(), val.clone(), expr.clone()) {
                        Err(err) => return Some(err),
                        Ok(tupl) => {
                            self.variables.insert(val.clone().trim().to_string(), tupl);
                        }
                    }
                },
                AstNode::SetVal { val, expr } => {
                    match self.clone().type_check_set_val(val.clone(), expr.clone()) {
                        Err(err) => return Some(err),
                        Ok((var_type, expr)) => {
                            self.variables.insert(val.to_string(), (var_type, expr));
                        }
                    }
                },
                AstNode::If { clause, block, else_block } => {
                    if let Some(err) = self.clone().type_check_if(clause.clone(), block.clone(), else_block.clone()) {
                        return Some(err);
                    }
                },
                AstNode::For { val, from, to, block } => {
                    if let Some(err) = self.clone().type_check_for(val.clone(), from.clone(), to.clone(), block.clone()) {
                        return Some(err);
                    }
                },
                AstNode::While { clause, block } => {
                    if let Some(err) = self.clone().type_check_while(clause.clone(), block.clone()) {
                        return Some(err);
                    }
                }
            }
        }
        None
    }


    fn type_check_command(&self, name : String, args : Vec<Expression>) -> Option<Error> {
        if let Some(params) = self.functions.get(&name) {
            if name == "polygon" {
                if args.len() < 6 || args.len() % 2 != 0 {
                    return Some(Error::LogicError { message: format!("Wrong number of arguments for command polygon: got {}, expected at least 6 (even number) for polygon", args.len()).into() });
                }
                for arg in &args {
                    match self.clone().type_check_expr(arg) {
                        Err(error) => return Some(error),
                        Ok(arg_type) => {
                            if arg_type != Primitive(Int) {
                                return Some(Error::TypeError { message: format!("Wrong type of argument for command {}: got {:?}, expected Int", name, arg_type).into() });
                            }
                        }
                    }
                }
                return None;
            }
            if params.len() != args.len() {
                return Some(Error::LogicError { message: format!("Wrong number of arguments for command {}: got {}, expected {}", name, args.len(), params.len()).into() });
            }
            for i in 0 .. params.len() {
                match self.clone().type_check_expr(&args[i]) {
                    Err(error) => return Some(error),
                    Ok(arg_type) => {
                        if arg_type != params[i] {
                            return Some(Error::TypeError { message: format!("Wrong type of argument number {} for command {}: got {:?}, expected {:?}", i, name, arg_type, params[i]).into() });
                        }
                    }
                }
                
            }
        } else {
            return Some(Error::LogicError { message: format!("Unknown command: {}", name).into() });
        }
        None
    }

    fn type_check_set_val(&self, val: VariableCall, expr: Expression) -> Result<(Type, Expression), Error> {
        let var_type = self.clone().type_check_var(&val)?;
        let expr_type = self.clone().type_check_expr(&expr)?;
        if var_type != expr_type {
            return Err(Error::LogicError { message: format!("Cannot assign expression of type {:?} to variable {} of type {:?}!", expr_type, val, var_type).into() });
        }
        Ok((var_type, expr))
    }

    fn type_check_init(&self, new_type_def : Type, val : String, expr : Expression) -> Result<(Type, Expression), Error>{
        if let Some(_) = self.variables.get(&val) {
            return Err(Error::LogicError { message: format!("Variable {} is re-defined!", val).into() }); 
        } else {
            let expr_type = self.clone().type_check_expr(&expr)?;
            if expr_type != new_type_def.clone() {
                return Err(Error::LogicError { message: format!("Cannot assign expression of type {:?} to variable {} of type {:?}!", expr_type, val, new_type_def).into() }); 
            }
            Ok((expr_type, expr))
        }
    }

    fn type_check_if(&self, clause : Expression, block : AstBlock, else_block : Option<AstBlock>) -> Option<Error> {
        match self.clone().type_check_expr(&clause) {
            Err(error) => return Some(error),
            Ok(clause_type) => {
                if clause_type != Primitive(Bool) {
                    return Some(Error::LogicError { message: format!("If clause must be a bool expression").into() })
                }
                let mut if_prog = self.clone();
                if_prog.lines = block;
                if let Some(err) = if_prog.type_check() {
                    return Some(err);
                }
                if let Some(else_lines) = else_block {
                    let mut else_prog = self.clone();
                    else_prog.lines = else_lines;
                    if let Some(err) = else_prog.type_check() {
                        return Some(err);
                    }
                }
            }
        }
    
        None
    }

    fn type_check_for(&self, val : String, from : BaseValue, to : BaseValue, block : AstBlock) -> Option<Error> {
        match self.clone().type_check_baseval(&from) {
            Err(error) => return Some(error),
            Ok(t) => {
                match  self.clone().type_check_baseval(&to) {
                    Err(error) => return Some(error),
                    Ok(f) => {
                        if t != f || t != Primitive(Int) {
                            return Some(Error::LogicError { message: format!("For loop range can only be integer values").into() })  
                        }
                        let mut for_prog = self.clone();
                        for_prog.lines = block;
                        for_prog.variables.insert(val, (Primitive(Int), Expression::Value(from)));
                        if let Some(err) = for_prog.type_check() {
                            return Some(err);
                        }
                    }
                }
            }
        }
        None
    }

    fn type_check_while(&self, clause : Expression, block : AstBlock) -> Option<Error> {
        match  self.clone().type_check_expr(&clause) {
            Err(error) => return Some(error),
            Ok(clause_type) => {
                if clause_type != Primitive(Bool) {
                    return Some(Error::LogicError { message: format!("While clause must be a bool expression").into() })
                }
                let mut while_prog = self.clone();
                while_prog.lines = block;
                if let Some(err) = while_prog.type_check() {
                    return Some(err);
                }
            }
        }
    
        None
    }

    fn type_check_expr(&self, expr : &Expression) -> Result<Type, Error> {
        match expr {
            Expression::Value(base_value) => {
                let expr_type =  self.clone().type_check_baseval(base_value)?;
                Ok(expr_type)
            },
            Expression::Unary(op, inner) => {
                match op {
                    UnaryOperator::UnaryMinus => {
                        let inner_type = self.clone().type_check_expr(&*inner)?;
                        if inner_type == Primitive(Int) {Ok(Primitive(Int))} else 
                        if inner_type == Primitive(Float) {Ok(Primitive(Float))} else 
                        {Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })}
                    },
                    UnaryOperator::Parentheses =>  self.clone().type_check_expr(&*inner),
                }
            },
            Expression::Binary(op, lhs, rhs) => {
                let lhs_type =  self.clone().type_check_expr(&*lhs)?;
                let rhs_type =  self.clone().type_check_expr(&*rhs)?;
                if *op == Operator::AND || *op == Operator::OR {
                    if lhs_type != Primitive(Bool) || rhs_type != Primitive(Bool) {
                        return Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })
                    }
                    Ok(Primitive(Bool))
                } else {
                    if lhs_type != Primitive(Int) && lhs_type != Primitive(Float) {
                        return Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })
                    }
                    if rhs_type != Primitive(Int) && rhs_type != Primitive(Float) {
                        return Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })
                    }
                    if !is_arith(*op) {
                        return Ok(Primitive(Bool))
                    }
                    if lhs_type == Primitive(Float) || rhs_type == Primitive(Float) {
                        return Ok(Primitive(Float))
                    }
                    Ok(Primitive(Int))
                }
            },
        }
    }

    fn recursive_type_check_var(&self, tp: &Type, depth: usize) -> Result<Type, Error> {
        if let Type::Array(inner_type, _) = tp {
            if let Some(inner) = inner_type.as_ref() {
                if depth == 1 {
                    return Ok(inner.clone());
                } else {
                    return self.recursive_type_check_var(inner, depth - 1);
                }
            } else {
                return Err(Error::ParseError { message: "Array type is not defined".into() });
            }
        }
        Err(Error::ParseError { message: "Expected an array type".into() })
    }

    fn type_check_var(&self, var: &VariableCall) -> Result<Type, Error> {
        let (name, depth) = match var {
            VariableCall::Name(name) => (name, 0),
            VariableCall::ArrayCall(name, inds) => (name, inds.len())
        };
        if let Some((tp, _)) = self.variables.get(name) {
            if depth == 0 { return Ok(tp.clone());} else {
                return self.recursive_type_check_var(tp, depth);
            }
        } else {
            Err(Error::LogicError { message: format!("Variable {} is not defined!", var).into() })
        }
    }

    fn type_check_baseval(&self, base : &BaseValue) -> Result<Type, Error> {
        use BaseType::*;
        use Type::*;
        match base {
            BaseValue::Id(var) => self.type_check_var(var),
            BaseValue::Int(_) => Ok(Primitive(Int)),
            BaseValue::Bool(_) => Ok(Primitive(Bool)),
            BaseValue::Color(_, _, _) => Ok(Primitive(Color)),
            BaseValue::RandomColor => Ok(Primitive(Color)),
            BaseValue::Float(_) => Ok(Primitive(Float)),
            BaseValue::Array(inner_type, arr) => {
                let types: Result<Vec<Type>, Error> = arr.iter()
                    .map(|item| self.type_check_baseval(item))
                    .collect();
                let types = types?;
                if types.is_empty() {
                    return Ok(Array(Box::new(None), 0));
                }
                if inner_type.is_none() {
                    return Err(Error::ParseError { message: "Array type is not defined".into() });
                }
                let inner_type = &inner_type.clone().unwrap();
                
                if types.iter().any(|t| t != inner_type) {
                    return Err(Error::TypeError { message: format!("Array elements must all be of type {:?}, got {:?}", inner_type, types).into() });
                }
                Ok(Array(Box::new(Some(inner_type.clone())), arr.len()))
            }
        }
    }

}


use std::{collections::HashMap, sync::Arc};

use quanta_parser::{ast::*, error::Error};

#[derive(Debug, Clone)]
pub struct Program {
    pub lines: AstBlock, 
    pub variables : HashMap<String, (BaseType, Expression)>,
    pub functions : HashMap<String, Vec<BaseType>>
}

pub fn create_program(ast: AstBlock) -> Program {
    Program {lines: ast, variables: HashMap::new(), functions: HashMap::from([
        (String::from("circle"), vec![BaseType::Int, BaseType::Int, BaseType::Int]),
        (String::from("line"), vec![BaseType::Int, BaseType::Int, BaseType::Int, BaseType::Int]),
        (String::from("rectangle"), vec![BaseType::Int, BaseType::Int, BaseType::Int, BaseType::Int]),
        (String::from("setLineColor"), vec![BaseType::Color]),
        (String::from("setFigureColor"), vec![BaseType::Color])
    ])}
}

impl Program {
    pub fn type_check(&mut self) -> Option<Error> {
        for line in &self.lines.nodes {
            match line {
                AstNode::Command { name, args } => {self.clone().type_check_command(name.clone(), args.clone())?;},
                AstNode::Init { typ, val, expr } => {self.clone().type_check_init(typ.clone(), val.clone(), expr.clone())?;},
                AstNode::If { clause, block, else_block } => {self.clone().type_check_if(clause.clone(), block.clone(), else_block.clone())?;},
                AstNode::For { val, from, to, block } => {self.clone().type_check_for(val.clone(), from.clone(), to.clone(), block.clone())?;},
                AstNode::While { clause, block } => {self.clone().type_check_while(clause.clone(), block.clone())?;},
            }
        }
        None
    }


    fn type_check_command(&self, name : String, args : Vec<Expression>) -> Option<Error> {
        if let Some(params) = self.functions.get(&name) {
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

    fn type_check_init(&mut self, new_type_def : Option<BaseType>, val : String, expr : Expression) -> Option<Error>{
        if let Some((old_type, _)) = self.variables.get(&val) {
            if new_type_def.is_some() {
                return Some(Error::LogicError { message: format!("Variable {} is re-defined!", val).into() }); 
            }
            match  self.clone().type_check_expr(&expr) {
                Err(error) => return Some(error),
                Ok(new_type) => {
                    self.variables.insert(val, (new_type, expr));
                }
            }
            None
        } else if new_type_def.is_none() {
            Some(Error::LogicError { message: format!("Type of variable {} is not defined", val).into() })
        } else {
            match self.clone().type_check_expr(&expr) {
                Err(error) => return Some(error),
                Ok(new_type) => {
                    if new_type != new_type_def.unwrap() {
                        return Some(Error::LogicError { message: format!("Cannot assign expression of type {:?} to variable {} of type {:?}!", new_type, val, new_type_def.unwrap()).into() }); 
                    }
                }
            }
            None
        }
    }

    fn type_check_if(&self, clause : Expression, block : AstBlock, else_block : Option<AstBlock>) -> Option<Error> {
        match self.clone().type_check_expr(&clause) {
            Err(error) => return Some(error),
            Ok(clause_type) => {
                if clause_type != BaseType::Bool {
                    return Some(Error::LogicError { message: format!("If clause must be a bool expression").into() })
                }
                let mut if_prog = self.clone();
                if_prog.lines = block;
                let _  = if_prog.type_check()?;
                if let Some(else_lines) = else_block {
                    let mut else_prog = self.clone();
                    else_prog.lines = else_lines;
                    let _  = else_prog.type_check()?;
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
                        if t != f || t != BaseType::Int {
                            return Some(Error::LogicError { message: format!("For loop range can only be integer values").into() })  
                        }
                        let mut for_prog = self.clone();
                        for_prog.lines = block;
                        for_prog.variables.insert(val, (BaseType::Int, Expression::Value(from)));
                        _ = for_prog.type_check()?;
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
                if clause_type != BaseType::Bool {
                    return Some(Error::LogicError { message: format!("While clause must be a bool expression").into() })
                }
                let mut while_prog = self.clone();
                while_prog.lines = block;
                let _  = while_prog.type_check()?;
            }
        }
    
        None
    }

    fn type_check_expr(&self, expr : &Expression) -> Result<BaseType, Error> {
        match expr {
            Expression::Value(baseValue) => {
                let expr_type =  self.clone().type_check_baseval(baseValue)?;
                Ok(expr_type)
            },
            Expression::Unary(op, inner) => {
                match op {
                    UnaryOperator::UnaryMinus => {
                        let inner_type =  self.clone().type_check_expr(&*inner)?;
                        if inner_type == BaseType::Int {Ok(BaseType::Int)} else 
                        if inner_type == BaseType::Float {Ok(BaseType::Float)} else 
                        {Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })}
                    },
                    UnaryOperator::Parentheses =>  self.clone().type_check_expr(&*inner),
                }
            },
            Expression::Binary(op, lhs, rhs) => {
                let lhs_type =  self.clone().type_check_expr(&*lhs)?;
                let rhs_type =  self.clone().type_check_expr(&*rhs)?;
                if *op == Operator::AND || *op == Operator::OR {
                    if lhs_type != BaseType::Bool || rhs_type != BaseType::Bool {
                        return Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })
                    }
                    Ok(BaseType::Bool)
                } else {
                    if lhs_type != BaseType::Int && lhs_type != BaseType::Float {
                        return Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })
                    }
                    if rhs_type != BaseType::Int && rhs_type != BaseType::Float {
                        return Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })
                    }
                    if !is_arith(*op) {
                        return Ok(BaseType::Bool)
                    }
                    if lhs_type == BaseType::Float || rhs_type == BaseType::Float {
                        return Ok(BaseType::Float)
                    }
                    Ok(BaseType::Int)
                }
            },
        }
    }

    fn type_check_baseval(&self, base : &BaseValue) -> Result<BaseType, Error> {
        match base {
            BaseValue::Id(var) => {
                if let Some((tp, _)) = self.variables.get(var) {
                    return Ok(*tp)
                }
                Err(Error::LogicError { message: format!("Variable {} is not defined!", var).into() })
            }
            BaseValue::Int(_) => Ok(BaseType::Int),
            BaseValue::Bool(_) => Ok(BaseType::Bool),
            BaseValue::Color(_, _, _) => Ok(BaseType::Color),
            BaseValue::Float(_) => Ok(BaseType::Float),
        }
    }

}


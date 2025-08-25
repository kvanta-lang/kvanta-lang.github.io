use std::collections::{HashMap, HashSet};

use quanta_parser::{ast::*, error::Error};
use BaseType::*;
use TypeName::*;

#[derive(Debug, Clone)]
pub struct Scope {
    variables: HashMap<String, (Type, Expression)>,
    outer_scope: Box<Option<Scope>>,
}

impl Scope {
    fn get(&self, name: &str) -> Option<&(Type, Expression)> {
        if let Some(var) = self.variables.get(name) {
            return Some(var);
        }
        if let Some(outer) = self.outer_scope.as_ref() {
            return outer.get(name);
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub lines: AstProgram, 
    pub scope : Scope,
    pub global_vars : HashMap<String, (Type, Expression)>,
    pub function_defs : HashMap<String, (Vec<(String, Type)>, Option<Type>)>,
    pub functions : HashMap<String, (Vec<(String, Type)>, Option<Type>, AstBlock)>,
    keywords: HashSet<String>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    None,
    Partial(Type),
    Full(Type),
}

impl ReturnType {
    pub fn t(&self) -> Option<&Type> {
        match self {
            ReturnType::None => None,
            ReturnType::Partial(t) => Some(t),
            ReturnType::Full(t) => Some(t),
        }
    }
}

fn int_type() -> Type 
{
    Type { type_name: Primitive(Int), is_const: false }
}

fn color_type() -> Type
{
    Type {type_name: Primitive(Color), is_const: false}
}

pub fn create_program(ast: AstProgram) -> Program {
    Program {lines: ast, scope: Scope { variables: HashMap::new(), outer_scope: Box::new(None) }, 
    global_vars: HashMap::new(),
    functions: HashMap::new(), function_defs: HashMap::from([
        (String::from("circle"), (vec![
            (String::from("x"), int_type()),
            (String::from("y"), int_type()),
            (String::from("radius"), int_type())
        ], None)),
        (String::from("line"), (vec![
            (String::from("x1"), int_type()),
            (String::from("y1"), int_type()),
            (String::from("x2"), int_type()),
            (String::from("y2"), int_type())
        ], None)),
        (String::from("rectangle"), (vec![
            (String::from("x1"), int_type()),
            (String::from("y1"), int_type()),
            (String::from("x2"), int_type()),
            (String::from("y2"), int_type())
        ], None)),
        (String::from("setLineColor"), (vec![
            (String::from("color"), color_type())
        ], None)),
        (String::from("setFigureColor"), (vec![
            (String::from("color"), color_type())
        ], None)),
        (String::from("setLineWidth"), (vec![
            (String::from("width"), int_type())
        ], None)),
        (String::from("polygon"), (vec![], None)), // at least 6 Ints for polygon
        (String::from("arc"), (vec![
            (String::from("circle_x"), int_type()),
            (String::from("circle_y"), int_type()),
            (String::from("radius"), int_type()),
            (String::from("angle_from"), int_type()),
            (String::from("angle_to"), int_type())
        ], None)),
    ]), keywords: HashSet::from(["circle", "line", "rectangle", 
                    "setLineColor", "setFigureColor", "setLineWidth", "polygon", "arc",
                    "for", "while", "global", "func", "if", "else",
                    "int", "bool", "color", "float", "array", "Color", "true", "false"
    ].map(|x| String::from(x)))}
}



impl Program {

    fn get(&self, name: &str) -> Option<&(Type, Expression)> {
        if let Some(var) = self.scope.get(name){
            return Some(var);
        }
        self.global_vars.get(name)
    }

    fn contains_key(&self, name: &str) -> bool {
        if self.scope.get(name).is_some() {
            return true;
        }
        self.global_vars.contains_key(name)
    }

    fn create_subprogram(&self, lines: Option<AstBlock>) -> Program {
        Program {
            lines: lines.map(|x| AstProgram::Block(x)).unwrap_or(self.lines.clone()),
            scope: Scope { variables: HashMap::new(), outer_scope: Box::new(Some(self.scope.clone())) },
            global_vars: self.global_vars.clone(),
            functions: self.functions.clone(),
            function_defs: self.function_defs.clone(),
            keywords: self.keywords.clone()
        }
    }

    pub fn type_check(&mut self) -> Result<ReturnType, Error> {
        match self.lines {
            AstProgram::Block(ref block) => self.type_check_block(block.clone()),
            AstProgram::Forest(ref forest) => {
                for func in &forest.0 {
                    if self.keywords.contains(&func.name) {
                        return Err(Error::TypeError { message: format!("'{}' is a keyword, it cannot be the name of a function", func.name).into() });
                    }
                    for (argname, _) in &func.args {
                        if self.keywords.contains(argname) { 
                            return Err(Error::TypeError { message: format!("'{}' is a keyword, it cannot be the name of a variable", argname).into() }); 
                        }
                    }
                    self.function_defs.insert(func.name.clone(), (func.args.clone(), func.return_type.clone()));
                }
                for (name, (typ, expr)) in &forest.1 {
                    if self.keywords.contains(name) {
                        
                    }
                    let expr_type = self.type_check_expr(&expr.clone())?;
                    if expr_type.type_name != typ.type_name {
                        return Err(Error::TypeError { message: format!("Global variable {} of type {} cannot be assigned a type {}", name, typ.to_string(), expr_type.to_string()).into() });
                    }
                    if self.contains_key(name) {
                        return Err(Error::LogicError { message: format!("Global variable {} is re-defined!", name).into() });
                    }
                    self.global_vars.insert(name.clone(), (typ.clone(), expr.clone()));
                }
                for func in &forest.0 {
                    
                    let mut sub = self.create_subprogram(None);
                    for arg in &func.args {
                        let simple_expr = {
                            match arg.1.type_name.clone() {
                                Primitive(Int) => Expression::Value(BaseValue::Int(0)),
                                Primitive(Float) => Expression::Value(BaseValue::Float(0.0)),
                                Primitive(Bool) => Expression::Value(BaseValue::Bool(false)),
                                Primitive(Color) => Expression::Value(BaseValue::RandomColor),
                                Array(_, _) => {
                                    Expression::Value(BaseValue::Array(vec![]))
                                }
                            }
                        };
                        sub.scope.variables.insert(arg.0.clone(), (arg.1.clone(), simple_expr));
                    }
                    if let Some(err) = sub.type_check_function(func.clone()) {
                        return Err(err);
                    }
                    self.functions.insert(func.name.clone(), (func.args.clone(), func.return_type.clone(), func.block.clone()));
                }
                Ok(ReturnType::None)
            }
        }
    }

    pub fn type_check_function(&mut self, func: AstFunction) -> Option<Error> {
        let mut func_prog = self.create_subprogram(Some(func.block));
        match func_prog.type_check() {
            Ok(ReturnType::Full(t)) => {
                if let Some(return_type) = &func.return_type {
                    if t != *return_type {
                        Some(Error::LogicError { message: format!("Function {} return type mismatch: expected {:?}, got {:?}", func.name, return_type, t).into() })
                    } else {
                        None
                    }
                } else {
                    Some(Error::LogicError { message: format!("Function {} has no return type defined, but returns {:?}", func.name, t).into() })
                }
            },
            Ok(ReturnType::Partial(t)) => {
                if let Some(return_type) = &func.return_type {
                    if t != *return_type {
                        return Some(Error::LogicError { message: format!("Function {} return type mismatch: expected {:?}, got {:?}", func.name, return_type, t).into() });
                    }
                    return Some(Error::LogicError { message: format!("Expected a return statement at the end of function {}", func.name).into() });
                } else {
                    return Some(Error::LogicError { message: format!("Function {} has no return type defined", func.name).into() });
                }
            },
            Ok(ReturnType::None) => {
                if let Some(rt) = func.return_type {
                    return Some(Error::LogicError { message: format!("Function {} has a return type {:?} defined but does not return anything", func.name, rt).into() });
                }
                None
            },
            Err(err) => return Some(err),   
        }
    }

    pub fn type_check_block(&mut self, block : AstBlock) -> Result<ReturnType, Error> {
        let mut return_type: Option<Type> = None;
        for line in block.nodes {
            match line {
                AstNode::Command { name, args } => {
                    if let Some(err) = self.clone().type_check_command(name.clone(), args.clone()) {
                        return Err(err);
                    }
                },
                AstNode::Init { typ, val, expr } => {
                     match self.clone().type_check_init(typ.clone(), val.clone(), expr.clone()) {
                        Err(err) => return Err(err),
                        Ok(tupl) => {
                            self.scope.variables.insert(val.clone().trim().to_string(), tupl);
                        }
                    }
                },
                AstNode::SetVal { val, expr } => {
                    match self.clone().type_check_set_val(val.clone(), expr.clone()) {
                        Err(err) => return Err(err),
                        Ok((var_type, expr)) => {
                            if self.global_vars.contains_key(val.clone().to_string().as_str()) {
                                self.global_vars.insert(val.clone().to_string(), (var_type, expr));
                            } else {
                                self.scope.variables.insert(val.to_string(), (var_type, expr));
                            }

                        }
                    }
                },
                AstNode::If { clause, block, else_block } => {
                    let if_prog = self.create_subprogram(None);
                    match if_prog.type_check_if(clause.clone(), block.clone(), else_block.clone())? {   
                        ReturnType::None => {},
                        ReturnType::Partial(t) => {
                            if let Some(rt) = &return_type {
                                if *rt != t {
                                    return Err(Error::LogicError { message: format!("If block return type mismatch: expected {:?}, got {:?}", rt, t).into() });
                                }
                            } else {
                                return_type = Some(t);
                            }
                        },
                        ReturnType::Full(t) => {
                            if let Some(rt) = &return_type {
                                if *rt != t {
                                    return Err(Error::LogicError { message: format!("If block return type mismatch: expected {:?}, got {:?}", rt, t).into() });
                                }
                            }
                            return Ok(ReturnType::Full(t));
                        }
                    }
                },
                AstNode::For { val, from, to, block } => {
                    match self.create_subprogram(None).type_check_for(val.clone(), from.clone(), to.clone(), block.clone())? {
                        ReturnType::None => {},
                        ReturnType::Partial(t) => {
                            if let Some(rt) = &return_type {
                                if *rt != t {
                                    return Err(Error::LogicError { message: format!("For block return type mismatch: expected {:?}, got {:?}", rt, t).into() });
                                }
                            } else {
                                return_type = Some(t);
                            }
                        },
                        ReturnType::Full(t) => {
                            if let Some(rt) = &return_type {
                                if *rt != t {
                                    return Err(Error::LogicError { message: format!("For block return type mismatch: expected {:?}, got {:?}", rt, t).into() });
                                }
                            }
                            return Ok(ReturnType::Full(t));
                        }
                    }
                },
                AstNode::While { clause, block } => {
                    match self.create_subprogram(None).type_check_while(clause.clone(), block.clone())? {
                        ReturnType::None => {},
                        ReturnType::Partial(t) => {
                            if let Some(rt) = &return_type {
                                if *rt != t {
                                    return Err(Error::LogicError { message: format!("For block return type mismatch: expected {:?}, got {:?}", rt, t).into() });
                                }
                            } else {
                                return_type = Some(t);
                            }
                        },
                        ReturnType::Full(t) => {
                            if let Some(rt) = &return_type {
                                if *rt != t {
                                    return Err(Error::LogicError { message: format!("For block return type mismatch: expected {:?}, got {:?}", rt, t).into() });
                                }
                            }
                            return Ok(ReturnType::Full(t));
                        }
                    }
                }
                AstNode::Return { expr } => {
                    let expr_type = self.create_subprogram(None).type_check_expr(&expr)?;
                    if let Some(rt) = &return_type {
                        if *rt != expr_type {
                            return Err(Error::LogicError { message: format!("Return type mismatch: expected {:?}, got {:?}", rt, expr_type).into() });
                        }
                    }
                    return Ok(ReturnType::Full(expr_type))
                },
            }
        }
        if let Some(rt) = &return_type {
            Ok(ReturnType::Partial(rt.clone()))
        } else {
            Ok(ReturnType::None)
        }
    }


    fn type_check_command(&self, name : String, args : Vec<Expression>) -> Option<Error> {
        // todo warning unused return type
        if let Some((params, _)) = self.function_defs.get(&name) {
            if name == "polygon" {
                if args.len() < 6 || args.len() % 2 != 0 {
                    return Some(Error::LogicError { message: format!("Wrong number of arguments for command polygon: got {}, expected at least 6 (even number) for polygon", args.len()).into() });
                }
                for arg in &args {
                    match self.clone().type_check_expr(arg) {
                        Err(error) => return Some(error),
                        Ok(arg_type) => {
                            if arg_type.type_name != Primitive(Int) {
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
            for (i, (param_name,param_type)) in params.iter().enumerate() {
                match self.clone().type_check_expr(&args[i]) {
                    Err(error) => return Some(error),
                    Ok(arg_type) => {
                        if arg_type.type_name != param_type.type_name {
                            return Some(Error::TypeError { message: format!("Wrong type of argument '{}' for command {}: got {:?}, expected {:?}", param_name, name, arg_type, params[i]).into() });
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
        if var_type.is_const {
            return Err(Error::TypeError { message: format!("Const variable {} cannot be reassigned", val).into() });
        }
        let expr_type = self.clone().type_check_expr(&expr)?;
        if !var_type.can_assign(&expr_type) {
            return Err(Error::LogicError { message: format!("Cannot assign expression of type {:?} to variable {} of type {:?}!", expr_type, val, var_type).into() });
        }
        Ok((var_type, expr))
    }

    fn type_check_init(&self, new_type_def : Type, val : String, expr : Expression) -> Result<(Type, Expression), Error>{
        if self.keywords.contains(&val) {
            return Err(Error::TypeError { message: format!("'{}' cannot be a variable, it is a keyword", val).into() })
        }
        if let Some(_) = self.get(&val) {
            return Err(Error::LogicError { message: format!("Variable {} is re-defined!", val).into() }); 
        } else {
            let expr_type = self.clone().type_check_expr(&expr)?;
            if !new_type_def.can_assign(&expr_type) {
                return Err(Error::LogicError { message: format!("Cannot assign expression of type {:?} to variable {} of type {:?}!", expr_type, val, new_type_def).into() }); 
            }
            Ok((new_type_def, expr))
        }
    }

    fn type_check_if(&self, clause : Expression, block : AstBlock, else_block : Option<AstBlock>) -> Result<ReturnType, Error> {
        let clause_type = self.clone().type_check_expr(&clause)?;
             
        if clause_type.type_name != Primitive(Bool) {
            return Err(Error::LogicError { message: format!("If clause must be a bool expression").into() })
        }
        let mut if_prog = self.create_subprogram(Some(block));
        let if_type = if_prog.type_check()?;

        if matches!(else_block, None) {
            if let ReturnType::Full(t) = if_type {
                return Ok(ReturnType::Partial(t));
            }
            return Ok(if_type);
        }

        let else_block = else_block.unwrap();
        let mut else_prog = self.create_subprogram(Some(else_block));
        let else_type = else_prog.type_check()?;

        if let (Some(t1), Some(t2)) = (if_type.t(), else_type.t()) {
            if t1 != t2 {
                return Err(Error::LogicError { message: format!("Return type of if and else block must match: {:?} != {:?}", t1, t2).into() });
            }
        }

        if if_type == else_type {
            return Ok(if_type);
        }

        return Ok(ReturnType::Partial(if_type.t().unwrap().clone()));
        
    }

    fn type_check_for(&self, val : String, from : BaseValue, to : BaseValue, block : AstBlock) -> Result<ReturnType, Error> {
        let t = self.clone().type_check_baseval(&from)?;
        let f = self.clone().type_check_baseval(&to)?;
        if t != f || t.type_name != Primitive(Int) {
            return Err(Error::LogicError { message: format!("For loop range can only be integer values").into() })  
        }
        let mut for_prog = self.create_subprogram(Some(block));
        for_prog.scope.variables.insert(val, (Type{type_name:Primitive(Int), is_const:false}, Expression::Value(from)));
        for_prog.type_check()
    }

    fn type_check_while(&self, clause : Expression, block : AstBlock) -> Result<ReturnType, Error> {
        let clause_type = self.clone().type_check_expr(&clause)?;
        if clause_type.type_name != Primitive(Bool) {
            return Err(Error::LogicError { message: format!("While clause must be a bool expression").into() });
        }
        let mut while_prog = self.clone();
        while_prog.lines = AstProgram::Block(block);
        while_prog.type_check()
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
                        if inner_type.type_name == Primitive(Int) {Ok(Type::typ(Int))} else 
                        if inner_type.type_name == Primitive(Float) {Ok(Type::typ(Float))} else 
                        {Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })}
                    },
                    UnaryOperator::Parentheses =>  self.clone().type_check_expr(&*inner),
                }
            },
            Expression::Binary(op, lhs, rhs) => {
                let lhs_type =  self.clone().type_check_expr(&*lhs)?;
                let rhs_type =  self.clone().type_check_expr(&*rhs)?;
                if *op == Operator::AND || *op == Operator::OR {
                    if lhs_type.type_name != Primitive(Bool) || rhs_type.type_name != Primitive(Bool) {
                        return Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })
                    }
                    Ok(Type::typ(Bool))
                } else {
                    if lhs_type.type_name != Primitive(Int) && lhs_type.type_name != Primitive(Float) {
                        return Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })
                    }
                    if rhs_type.type_name != Primitive(Int) && rhs_type.type_name!= Primitive(Float) {
                        return Err(Error::TypeError { message: format!("Type mismatch in expression: {:?}", expr).into() })
                    }
                    if !is_arith(*op) {
                        return Ok(Type::typ(Bool))
                    }
                    if lhs_type.type_name == Primitive(Float) || rhs_type.type_name == Primitive(Float) {
                        return Ok(Type::typ(Float))
                    }
                    Ok(Type::typ(Int))
                }
            },
        }
    }

    fn recursive_type_check_var(&self, tp: &Type, depth: usize) -> Result<Type, Error> {
        if let Array(inner_type, _) = &tp.type_name {
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
        if self.keywords.contains(name) {
            return Err(Error::TypeError { message: format!("'{}' is a keyword, it cannot be a name of a variable", name).into() });
        }
        if let Some((tp, _)) = self.get(name) {
            if depth == 0 { 
                return Ok(tp.clone());
            } else {
                return self.recursive_type_check_var(tp, depth);
            }
        } else {
            Err(Error::LogicError { message: format!("Variable {} is not defined!", var).into() })
        }
    }

    fn type_check_baseval(&self, base : &BaseValue) -> Result<Type, Error> {
        use BaseType::*;
        match base {
            BaseValue::Id(var) => self.type_check_var(var),
            BaseValue::Int(_) => Ok(Type::typ(Int)),
            BaseValue::Bool(_) => Ok(Type::typ(Bool)),
            BaseValue::Color(_, _, _) => Ok(Type::typ(Color)),
            BaseValue::RandomColor => Ok(Type::typ(Color)),
            BaseValue::Float(_) => Ok(Type::typ(Float)),
            BaseValue::Array(arr) => {
                let types: Result<Vec<Type>, Error> = arr.iter()
                    .map(|item| self.type_check_baseval(item))
                    .collect();
                let types = types?;
                if types.is_empty() {
                    return Ok(Type{type_name:Array(Box::new(None), 0), is_const: false});
                }
                let inner_type = &types.first().unwrap().clone();
                
                if types.iter().any(|t| t.type_name != inner_type.type_name) {
                    return Err(Error::TypeError { message: format!("Array elements must all be of type {:?}, got {:?}", inner_type, types).into() });
                }
                Ok(Type{type_name:Array(Box::new(Some(inner_type.clone())), arr.len()), is_const: false})
            },
            BaseValue::FunctionCall(name,arg_list, return_type ) => {
                match self.function_defs.get(name) {
                    None => Err(Error::TypeError { message: format!("Unknown function {}", name).into() }),
                    Some((arg_defs, _)) => {
                        if arg_list.len() != arg_defs.len() {
                            return Err(Error::TypeError { message: format!("Funcion '{}' expects {} arguments, but got {}", name, arg_defs.len(), arg_list.len()).into() })
                        }
                        for (i, (arg_name, arg_def)) in arg_defs.iter().enumerate() {
                            let expr_type = self.type_check_expr(arg_list.get(i).unwrap())?;
                            if !arg_def.can_assign(&expr_type) {
                                return Err(Error::TypeError { message: format!("Funcion '{}' expects argument '{}' of type '{}', but got '{}'", name, arg_name, arg_def.to_string(), expr_type.to_string()).into() })
                            }
                        } 
                        Ok(return_type.clone())
                    }
                }
            }
        }
    }

}


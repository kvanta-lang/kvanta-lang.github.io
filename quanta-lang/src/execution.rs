use std::{collections::HashMap};

use quanta_parser::{ast::{AstBlock, AstNode, AstProgram, BaseValue, Expression, Operator, Type, UnaryOperator, VariableCall}, error::Error};

use crate::{utils::{canvas::Canvas, message::Message}, program::Program};
use js_sys::Math;

#[derive(Debug, Clone)]
pub struct Scope {
    pub variables: HashMap<String, BaseValue>,
    pub outer_scope: Option<Box<Scope>>,
}

impl Scope {

    pub fn contains_key(&self, name: &str) -> bool {
        if self.variables.contains_key(name) {
            return true;
        }
        if let Some(outer) = self.outer_scope.as_ref() {
            return outer.contains_key(name);
        }
        false
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut BaseValue> {
        if let Some(var) = self.variables.get_mut(name) {
            return Some(var);
        }
        if let Some(outer) = self.outer_scope.as_mut() {
            return outer.get_mut(name);
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Execution {
    pub lines: AstProgram, 
    scope : Scope,
    pub global_vars : HashMap<String, BaseValue>,
    pub global_var_definitions: HashMap<String, (Type, Expression)>,
    pub functions : HashMap<String, (Vec<(String, Type)>, Option<Type>, AstBlock)>,
    pub canvas    : Canvas,
    figure_color : String,
    line_color : String,
    line_width : i32,
}

fn color_to_str(r: &u8, g : &u8, b: &u8) -> String {
    let s = format!("#{:02x}{:02x}{:02x}", r, g, b);
    s
}

macro_rules! expect_arg {
    // Варіант із полями: BaseValue::Variant(pats...)
    ($fname:expr, $vals:expr, $idx:expr, $Variant:ident ( $($pat:pat),* ) => $build:expr) => {{
        let __arg_index = $idx; // збережемо, щоб не обчислювати двічі
        match &$vals[__arg_index] {
            BaseValue::$Variant($($pat),*) => { $build }
            other => {
                return Err(Error::RuntimeError {
                    message: format!(
                        "{}: arg #{}: Expected argument type {} but got {}",
                        $fname,
                        __arg_index,            
                        stringify!($Variant),
                        other.get_type().to_string()
                    ).into(),
                });
            }
        }
    }};
}

impl Execution {

    pub fn from_program(prog : Program) -> Execution {
        Execution {
            lines : prog.lines.clone(),
            scope : Scope { variables: HashMap::new(), outer_scope: None },
            global_vars: HashMap::new(),
            global_var_definitions: prog.global_vars.clone(),
            canvas: Canvas::default(),
            functions: prog.functions.clone(),
            figure_color: "#FFFFFF".to_string(),
            line_color: "#000000".to_string(),
            line_width: 1,
        }
    }

    fn create_subprogram(&self) -> Execution {
        Execution {
            lines: self.lines.clone(),
            scope: Scope { variables: HashMap::new(), outer_scope: Some(Box::new(self.scope.clone())) },
            canvas: Canvas::default(),
            global_vars: self.global_vars.clone(),
            global_var_definitions: HashMap::new(),
            functions: self.functions.clone(),
            figure_color: self.figure_color.clone(),
            line_color: self.line_color.clone(),
            line_width: self.line_width,
        }
    }

    pub fn contains_key(&self, name: &str) -> bool {
        if self.scope.contains_key(name) {
            return true;
        }
        self.global_vars.contains_key(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut BaseValue> {
        if let Some(var) = self.scope.get_mut(name) {
            return Some(var);
        }
        self.global_vars.get_mut(name)
    }

    fn get_variable(&mut self, var: &VariableCall) -> Result<&mut BaseValue, Error> {
        match var {
            VariableCall::Name(name) => self.get_mut(name).ok_or(Error::RuntimeError { message: format!("Unknown variable: {}", name).into() }),
            VariableCall::ArrayCall(name, indices) => {
                if !self.contains_key(name) {
                    return Err(Error::RuntimeError { message: format!("Unknown array 1: {}, variables: {:?}", name, self.scope).into() });
                }
                if indices.is_empty() {
                    return Err(Error::RuntimeError { message: "Empty index".into() });
                }
                let mut integer_indices: Vec<i32> = vec![];
                for index in indices {
                    match self.calculate_expression(index.clone().to_expr()) {
                        Ok(BaseValue::Int(i)) => {
                            if i < 0 {
                                return Err(Error::RuntimeError { message: format!("Negative index for array {}: {}", name, i).into() });
                            }
                            integer_indices.push(i);
                        },
                        _ => return Err(Error::RuntimeError { message: "Array indices must be integers".into() }),
                    }
                }
                let maybe_array = self.get_mut(name);
                if maybe_array.is_none() {
                    return Err(Error::RuntimeError { message: format!("Unknown array: {} ", name).into() });
                }
                let mut array = maybe_array.unwrap();
                while integer_indices.len() > 0 {
                    if let BaseValue::Array(_, elems) = array {
                        let index = integer_indices.remove(0);
                        if index < 0 || index as usize >= elems.len() {
                            return Err(Error::RuntimeError { message: format!("Index out of bounds for array {}: {}", name, index).into() });
                        }
                        array = elems.get_mut(index as usize).unwrap();
                    } else {
                        return Err(Error::RuntimeError { message: format!("Variable {} is not an array", name).into() });
                    }
                }
                Ok(array)
            }
        }
    }

    fn execute_function(&mut self, function_name: &str, args: Vec<Expression>) -> Result<Option<BaseValue>, Error>{
        let mut vals : Vec<BaseValue> = vec![];
        for arg in args {
            let val = self.calculate_expression(arg)?;
            vals.push(val);
        }
        match function_name {
            "circle" => {
                let x1 = expect_arg!("circle", vals, 0, Int(v) => *v);
                let y1 = expect_arg!("circle", vals, 1, Int(v) => *v);
                let r = expect_arg!("circle", vals, 2, Int(v) => *v);

                self.canvas.add_command(format!("circle {} {} {} fill={} stroke={} width={}", x1, y1, r, self.figure_color, self.line_color, self.line_width));
                Ok(None)
            },
            "line" => {
                let x1 = expect_arg!("line", vals, 0, Int(v) => *v);
                let y1 = expect_arg!("line", vals, 1, Int(v) => *v);
                let x2 = expect_arg!("line", vals, 2, Int(v) => *v);
                let y2 = expect_arg!("line", vals, 3, Int(v) => *v);

                self.canvas.add_command(format!("line {} {} {} {} stroke={} width={}", x1, y1, x2, y2, self.line_color, self.line_width));
                Ok(None)
            },
            "rectangle" => {
                let x1 = expect_arg!("rectangle", vals, 0, Int(v) => *v);
                let y1 = expect_arg!("rectangle", vals, 1, Int(v) => *v);
                let x2 = expect_arg!("rectangle", vals, 2, Int(v) => *v);
                let y2 = expect_arg!("rectangle", vals, 3, Int(v) => *v);
                
                self.canvas.add_command(format!("rectangle {} {} {} {} fill={} stroke={} width={}", x1, y1, x2, y2, self.figure_color, self.line_color, self.line_width));
                Ok(None)
            },
            "polygon" => {
                let mut nums = String::new();
                for val in &vals {
                    if let BaseValue::Int(num) = val {
                        nums.push_str(&format!("{} ", num));
                    } else {
                        return Err(Error::RuntimeError { message: "Incorrect arguments for polygon function!".into() });
                    }
                }
                self.canvas.add_command(format!("polygon {} fill={} stroke={} width={}", nums.trim(), self.figure_color, self.line_color, self.line_width));
                Ok(None)
            },
            "arc" => {
                let x = expect_arg!("arc", vals, 0, Int(v) => *v);
                let y = expect_arg!("arc", vals, 1, Int(v) => *v);
                let r = expect_arg!("arc", vals, 2, Int(v) => *v);
                let start = expect_arg!("arc", vals, 3, Int(v) => *v);
                let end = expect_arg!("arc", vals, 4, Int(v) => *v);

                self.canvas.add_command(format!("arc {} {} {} {} {} fill={} stroke={} width={}", x, y, r, start, end, self.figure_color, self.line_color, self.line_width));
                Ok(None)
            },
            "setLineColor" => {
                if let BaseValue::Color(r,g,b) = &vals[0] {
                    self.line_color = color_to_str(r, g, b);
                    Ok(None)
                }
                else if let BaseValue::RandomColor = &vals[0] {
                    let r = (255.0 * Math::random()) as u8;
                    let g = (255.0 * Math::random()) as u8;
                    let b = (255.0 * Math::random()) as u8;
                    self.line_color = color_to_str(&r, &g, &b);
                    Ok(None)
                }
                else {
                    Err(Error::RuntimeError { message: format!("Incorrect arguments for setLineColor function: expected a color, got {:?}!", &vals[0]).into() })
                }
            },
            "setFigureColor" => {
                if let BaseValue::Color(r,g,b) = &vals[0] {
                    self.figure_color = color_to_str(r, g, b);
                    Ok(None)
                }
                else if let BaseValue::RandomColor = &vals[0] {
                    let r = (255.0 * Math::random()) as u8;
                    let g = (255.0 * Math::random()) as u8;
                    let b = (255.0 * Math::random()) as u8;
                    self.figure_color = color_to_str(&r, &g, &b);
                    Ok(None)
                }
                else {
                    Err(Error::RuntimeError { message: format!("Incorrect arguments for setFigureColor function: expected a color, got {:?}!", &vals[0]).into() })
                }
            },
            "setLineWidth" => {
                let width = expect_arg!("setLineWidth", vals, 0, Int(width) => *width);
                if width >= 0 {
                    self.line_width = width;
                    Ok(None)
                } else {
                    Err(Error::RuntimeError { message: "Line width can't be negative!".into() })
                }
            },
            name => {
                if self.functions.contains_key(name) {
                    let (params, _, body) = self.functions.get(name).unwrap();
                    if params.len() != vals.len() {
                        return Err(Error::RuntimeError { message: format!("Function {} expects {} arguments, but got {}", name, params.len(), vals.len()).into() });
                    }
                    let mut new_exec = self.create_subprogram();
                    new_exec.canvas = Canvas::default();
                    for (i, param) in params.iter().enumerate() {
                        new_exec.scope.variables.insert(param.0.clone(), vals[i].clone());
                    }
                    if let Some(return_value) = new_exec.execute_commands(body.nodes.clone())? {
                        for command in new_exec.canvas.get_commands().iter() {
                            self.canvas.add_command(command.clone());
                        }
                        return Ok(Some(return_value));
                    }
                    for command in new_exec.canvas.get_commands().iter() {
                        self.canvas.add_command(command.clone());
                    }
                    return Ok(None);
                }
                Err(Error::RuntimeError { message: format!("Unknown function: {}", function_name).into() })
            }
        }
    }

    fn execute_init(&mut self, var: String, expr: Expression) -> Option<Error>{
        let val = self.calculate_expression(expr);
        if let Err(err) = val {
            return Some(err);
        }
        if let Ok(value) = val {
            if let Some(_) = self.get_mut(&var) {
                return Some(Error::RuntimeError { message: format!("Variable {} is already defined!", &var).into() });
            }
            self.scope.variables.insert(var, value);
            return None
        }
        Some(Error::RuntimeError { message: "Couldn't assign new value".into() })
    }

    fn execute_set(&mut self, var: &VariableCall, expr: Expression) -> Option<Error> {
        let val = self.calculate_expression(expr);
        if let Err(err) = val {
            return Some(err);
        }
        if let Ok(value) = val {
            match self.get_variable(var) {
                Ok(variable) => {
                    *variable = value;
                    return None;
                },
                Err(err) => return Some(err),
            }
        }
        Some(Error::RuntimeError { message: "Couldn't set new value".into() })
    }

    pub fn execute(&mut self) -> Message {
        match self.lines {
            AstProgram::Block(ref block) => {
                if let Err(err) = self.execute_commands(block.nodes.clone()) {
                    return Message::create_error_message(err);
                }
                Message::from_canvas(self.canvas.clone())
            },
            AstProgram::Forest(ref funcs) => {
                for (name, (_, expr)) in &self.global_var_definitions {
                    let mut new_exec = self.create_subprogram();
                    match new_exec.calculate_expression(expr.clone()) {
                        Ok(val) => {
                            self.global_vars.insert(name.clone(), val);
                        },
                        Err(err) => return Message::create_error_message(err),
                    }
                }
                for func in &funcs.0 {
                    if func.name == "main" {
                        if let Err(err) = self.execute_commands(func.block.nodes.clone()) {
                            return Message::create_error_message(err);
                        }
                        return Message::from_canvas(self.canvas.clone());
                    }
                }
                return Message::create_error_message(Error::RuntimeError { message: "No main function found".into() });
            },
        }
        
    }

    pub fn execute_commands(&mut self, nodes : Vec<AstNode>) -> Result<Option<BaseValue>, Error> {
        for line in nodes {
            match line {
                AstNode::Command { name, args } => {
                    self.execute_function(&name, args)?;
                },
                AstNode::Init { typ : _, val, expr } => {
                    if let Some(err) = self.execute_init(val, expr) {
                        return Err(err);
                    }
                }
                AstNode::SetVal { val, expr } => {
                    if let Some(err) = self.execute_set(&val, expr) {
                        return Err(err);
                    }
                }
                
                AstNode::If { clause, block, else_block } => {
                    if let BaseValue::Bool(val) = self.calculate_expression(clause)? {
                        let mut new_exec = self.create_subprogram();
                        if val {
                            if let Some(return_value) = new_exec.execute_commands(block.nodes)? {
                                return Ok(Some(return_value));
                            }
                        } else if let Some(else_block) = else_block {
                            if let Some(return_value) = new_exec.execute_commands(else_block.nodes)? {
                                return Ok(Some(return_value));
                            }
                        }
                        self.scope = *new_exec.scope.outer_scope.unwrap();
                        self.global_vars = new_exec.global_vars;
                        self.canvas = new_exec.canvas;
                    } else {
                        return Err(Error::RuntimeError { message: "If clause must be a boolean expression".into() });
                    }
                },
                AstNode::While { clause, block } => {
                    loop {
                        let val = self.calculate_expression(clause.clone());
                        match val {
                            Ok(BaseValue::Bool(t)) => {
                                if t {
                                    let mut new_exec = self.create_subprogram();
                                    if let Some(return_value) = new_exec.execute_commands(block.nodes.clone())? {
                                        self.canvas = new_exec.canvas;
                                        self.scope = *new_exec.scope.outer_scope.unwrap();
                                        self.global_vars = new_exec.global_vars;
                                        return Ok(Some(return_value));
                                    }
                                    self.scope = *new_exec.scope.outer_scope.unwrap();
                                    self.global_vars = new_exec.global_vars;
                                    self.canvas = new_exec.canvas;
                                } else {
                                    break;
                                }
                            }
                            Err(err) => {
                                return Err(err);
                            }
                            _ => unreachable!("Unexpected code 2")
                        }
                    }
                },
                AstNode::For { val, from, to, block } => {
                    if let BaseValue::Int(f_) = from {
                        if let BaseValue::Int(t_) = to {
                            let (f,t) = {
                                if f_ <= t_ {
                                    (f_, t_)
                                } else {
                                    (t_, f_)
                                }
                            };
                            for cycle in f..=t {
                                if let Some(return_value) = self.execute_for(val.clone(), cycle, block.clone()) ?{
                                    return Ok(Some(return_value));
                                }
                            }                    
                        }
                    }
                },
                AstNode::Return { expr } => {
                    let val = self.calculate_expression(expr)?;
                    return Ok(Some(val.clone()))
                },
            }
        }
        Ok(None)
    }

    fn execute_for(&mut self, val: String, cycle : i32, block : AstBlock) -> Result<Option<BaseValue>, Error> {
        self.execute_init(val, Expression::Value(BaseValue::Int(cycle)));
        let mut new_exec = self.create_subprogram();
        let result = new_exec.execute_commands(block.nodes.clone());
        self.global_vars = new_exec.global_vars;
        self.scope = *new_exec.scope.outer_scope.unwrap();
        self.canvas = new_exec.canvas;
        result        
    }

    fn calculate_expression(&mut self, expr: Expression) -> Result<BaseValue, Error> {
        match expr {
            Expression::Value(base_value) => {
                match base_value {
                    BaseValue::Id(var) => {
                        self.get_variable(&var).cloned()
                    },
                    BaseValue::FunctionCall(name, exprs, _ ) => {
                        let mut vals = vec![];
                        for expr in exprs {
                            let val = self.calculate_expression(expr)?;
                            vals.push(val);
                        }
                        if let Some((params, _, body)) = self.functions.get(&name) {
                            let mut new_exec = self.create_subprogram();
                            new_exec.scope.variables = HashMap::new(); // Clear the scope for the function call
                            for (i, (name, _)) in params.iter().enumerate() {
                                if i < vals.len() {
                                    new_exec.scope.variables.insert(name.clone(), vals[i].clone());
                                } else {
                                    return Err(Error::RuntimeError { message: format!("Function {} expects {} arguments, but got {}", name, params.len(), vals.len()).into() });
                                }
                            }
                            if let Some(return_value) = new_exec.execute_commands(body.nodes.clone())? {
                                self.canvas = new_exec.canvas;
                                self.scope = *new_exec.scope.outer_scope.unwrap();
                                self.global_vars = new_exec.global_vars;
                                return Ok(return_value);
                            }
                            self.canvas = new_exec.canvas;
                            self.scope = *new_exec.scope.outer_scope.unwrap();
                            self.global_vars = new_exec.global_vars;
                            return Err(Error::RuntimeError { message: format!("Function {} didn't return a value", name).into() });
                        }
                        Err(Error::RuntimeError { message: format!("Unknown function: {}", name).into() })
                    }
                    x => Ok(x)
                }
            },
            Expression::Unary(op, inner) => {
                let inner_val = self.calculate_expression(*inner)?;
                match op {
                    UnaryOperator::UnaryMinus => {
                        match inner_val {
                            BaseValue::Int(num) => Ok(BaseValue::Int((-1) * num)),
                            BaseValue::Float(num) => Ok(BaseValue::Float((-1.0) * num)),
                            BaseValue::Bool(_) => Err(Error::RuntimeError { message: "Minus bool: {}".into() }),
                            BaseValue::Color(_, _, _) => Err(Error::RuntimeError { message: "Minus color: {}".into() }),
                            _ =>unreachable!("Unexpected code 3")
                        }
                    },
                    UnaryOperator::Parentheses => Ok(inner_val)
                }
            },
            Expression::Binary(op, lhs, rhs) => {
                let left_val = self.calculate_expression(*lhs)?;
                let right_val = self.calculate_expression(*rhs)?;

                if let BaseValue::Int(x) = left_val {
                    if let BaseValue::Int(y) = right_val {
                        return compare_ints(x, y, op);
                    }
                    if let BaseValue::Float(y) = right_val {
                        let t = x as f32;
                        return compare_floats(t, y, op);
                    }
                }

                if let BaseValue::Float(y) = left_val {
                    if let BaseValue::Int(x) = right_val {
                        let t = x as f32;
                        return compare_floats(y, t, op);
                    }
                    if let BaseValue::Float(x) = right_val {
                        return compare_floats(y, x, op);
                    }
                }

                if let BaseValue::Bool(a) = left_val {
                    if let BaseValue::Bool(b) = right_val {
                        return compare_bools(a, b, op);
                    }
                }

                Err(Error::RuntimeError { message: "Unsolvable expression!".into() })
            },
        }
    }
}


fn compare_ints(x: i32, y : i32, op: Operator) -> Result<BaseValue, Error> {
    match op {

        Operator::EQ => return Ok(BaseValue::Bool(x == y)),
        Operator::NQ => return Ok(BaseValue::Bool(x != y)),
        Operator::GT => return Ok(BaseValue::Bool(x > y)),
        Operator::LT => return Ok(BaseValue::Bool(x < y)),
        Operator::GQ => return Ok(BaseValue::Bool(x >= y)),
        Operator::LQ => return Ok(BaseValue::Bool(x <= y)),
        
        Operator::AND => unreachable!("Unexpected code 4"),
        Operator::OR => unreachable!("Unexpected code 5"),
        
        Operator::Plus => Ok(BaseValue::Int(x + y)),
        Operator::Minus => Ok(BaseValue::Int(x - y)),
        Operator::Mult => Ok(BaseValue::Int(x * y)),
        Operator::Div => Ok(BaseValue::Int(x / y)),
        Operator::Mod => Ok(BaseValue::Int(x % y)),
    }
}

fn compare_floats(x: f32, y : f32, op: Operator) -> Result<BaseValue, Error> {
    match op {

        Operator::EQ => Ok(BaseValue::Bool(x == y)),
        Operator::NQ => Ok(BaseValue::Bool(x != y)),
        Operator::GT => Ok(BaseValue::Bool(x > y)),
        Operator::LT => Ok(BaseValue::Bool(x < y)),
        Operator::GQ => Ok(BaseValue::Bool(x >= y)),
        Operator::LQ => Ok(BaseValue::Bool(x <= y)),
        
        Operator::AND => unreachable!("Unexpected code 10"),
        Operator::OR => unreachable!("Unexpected code 11"),
        
        Operator::Plus => Ok(BaseValue::Float(x + y)),
        Operator::Minus => Ok(BaseValue::Float(x - y)),
        Operator::Mult => Ok(BaseValue::Float(x * y)),
        Operator::Div => Ok(BaseValue::Float(x / y)),
        Operator::Mod => Ok(BaseValue::Float(x % y)),
    }
}

fn compare_bools(a: bool, b : bool, op: Operator) -> Result<BaseValue, Error> {
    match op {

        Operator::EQ => unreachable!("Unexpected code 12"),
        Operator::NQ => unreachable!("Unexpected code 13"),
        Operator::GT => unreachable!("Unexpected code 14"),
        Operator::LT => unreachable!("Unexpected code 15"),
        Operator::GQ => unreachable!("Unexpected code 16"),
        Operator::LQ => unreachable!("Unexpected code 17"),
        
        Operator::AND => Ok(BaseValue::Bool(a && b)),
        Operator::OR => Ok(BaseValue::Bool(a || b)),
        
        Operator::Plus => unreachable!("Unexpected code 18"),
        Operator::Minus => unreachable!("Unexpected code 19"),
        Operator::Mult => unreachable!("Unexpected code 20"),
        Operator::Div => unreachable!("Unexpected code 21"),
        Operator::Mod => unreachable!("Unexpected code 22"),
    }
}




use std::collections::{HashMap};

use quanta_parser::{ast::{AstBlock, BaseValue, Expression, Operator, UnaryOperator, AstNode}, error::Error};

use crate::{utils::{canvas::Canvas, message::Message}, program::Program};
use js_sys::Math;

#[derive(Debug, Clone)]
pub struct Execution {
    pub lines: AstBlock, 
    pub variables : HashMap<String, BaseValue>,
    pub canvas    : Canvas,
    figure_color : String,
    line_color : String,
    line_width : i32,
}

fn color_to_str(r: &u8, g : &u8, b: &u8) -> String {
    let s = format!("#{:02x}{:02x}{:02x}", r, g, b);
    print!("Color to str: {}", &s);
    s
}

impl Execution {

    fn execute_function(&mut self, function_name: &str, args: Vec<Expression>) -> Option<Error>{
        let mut vals : Vec<BaseValue> = vec![];
        for arg in args {
            let val = self.calculate_expression(arg);
            if let Err(err) = val {
                return Some(err);
            } else {
                vals.push(val.unwrap());
            }
        }
        use BaseValue::*;
        match function_name {
            "circle" => {
                print!("Print circle!!!");
                // todo return good type error
                if let (Int(x), Int(y), Int(r)) = (&vals[0], &vals[1], &vals[2]) {
                    self.canvas.add_command(format!("circle {} {} {} fill={} stroke={} width={}", x, y, r, self.figure_color, self.line_color, self.line_width));
                    None
                } else {
                    Some(Error::RuntimeError { message: "Incorrect arguments for circle function!".into() })
                }
            },
            "line" => {
                if let (Int(x1), Int(y1), Int(x2), Int(y2)) = (&vals[0], &vals[1], &vals[2], &vals[3]) {
                    self.canvas.add_command(format!("line {} {} {} {} stroke={} width={}", x1, y1, x2, y2, self.line_color, self.line_width));
                    None
                } else {
                    Some(Error::RuntimeError { message: "Incorrect arguments for line function!".into() })
                }
            },
            "rectangle" => {
                if let (Int(x1), Int(y1), Int(x2), Int(y2)) = (&vals[0], &vals[1], &vals[2], &vals[3]) {
                    self.canvas.add_command(format!("rectangle {} {} {} {} fill={} stroke={} width={}", x1, y1, x2, y2, self.figure_color, self.line_color, self.line_width));
                    None
                } else {
                    Some(Error::RuntimeError { message: "Incorrect arguments for rectangle function!".into() })
                }
            },
            "polygon" => {
                let mut nums = String::new();
                for val in &vals {
                    if let BaseValue::Int(num) = val {
                        nums.push_str(&format!("{} ", num));
                    } else {
                        return Some(Error::RuntimeError { message: "Incorrect arguments for polygon function!".into() });
                    }
                }
                self.canvas.add_command(format!("polygon {} fill={} stroke={} width={}", nums.trim(), self.figure_color, self.line_color, self.line_width));
                None
            },
            "arc" => {
                if let (Int(x), Int(y), Int(r), Int(start), Int(end)) = (&vals[0], &vals[1], &vals[2], &vals[3], &vals[4]) {
                    self.canvas.add_command(format!("arc {} {} {} {} {} fill={} stroke={} width={}", x, y, r, start, end, self.figure_color, self.line_color, self.line_width));
                    None
                } else {
                    Some(Error::RuntimeError { message: "Incorrect arguments for arc function!".into() })
                }
            },
            "setLineColor" => {
                if let BaseValue::Color(r,g,b) = &vals[0] {
                    self.line_color = color_to_str(r, g, b);
                    None
                }
                else if let BaseValue::RandomColor = &vals[0] {
                    let r = (255.0 * Math::random()) as u8;
                    let g = (255.0 * Math::random()) as u8;
                    let b = (255.0 * Math::random()) as u8;
                    self.line_color = color_to_str(&r, &g, &b);
                    None
                }
                else {
                    Some(Error::RuntimeError { message: "Incorrect arguments for setLineColor function!".into() })
                }
            },
            "setFigureColor" => {
                if let BaseValue::Color(r,g,b) = &vals[0] {
                    self.figure_color = color_to_str(r, g, b);
                    None
                }
                else if let BaseValue::RandomColor = &vals[0] {
                    let r = (255.0 * Math::random()) as u8;
                    let g = (255.0 * Math::random()) as u8;
                    let b = (255.0 * Math::random()) as u8;
                    self.figure_color = color_to_str(&r, &g, &b);
                    None
                }
                else {
                    Some(Error::RuntimeError { message: "Incorrect arguments for setFigureColor function!".into() })
                }
            },
            "setLineWidth" => {
                if let BaseValue::Int(width) = &vals[0] {
                    if *width >= 0 {
                        self.line_width = *width;
                        None
                    } else {
                        Some(Error::RuntimeError { message: "Line width can't be negative!".into() })
                    }
                } else {
                    Some(Error::RuntimeError { message: "Incorrect arguments for setLineWidth function!".into() })
                }
            },
            _ => {
                return Some(Error::RuntimeError { message: format!("Unknown function: {}", function_name).into() })
            }
        }
    }

    fn execute_init(&mut self, var: &str, expr: Expression) -> Option<Error>{
        let val = self.calculate_expression(expr);
        if let Err(err) = val {
            return Some(err);
        }
        if let Ok(value) = val {
            self.variables.insert(var.to_string(), value);
            return None
        }
        Some(Error::RuntimeError { message: "Couldn't assign new value".into() })
    }


    pub fn from_program(prog : Program) -> Execution {
        Execution {
            lines : prog.lines.clone(),
            variables : HashMap::new(),
            canvas: Canvas::default(),
            figure_color: "#FFFFFF".to_string(),
            line_color: "#000000".to_string(),
            line_width: 1,
        }
    }

    pub fn update_variables(&mut self, other : &Execution){
        let cpy = self.variables.clone();
        for key in cpy.keys() {
            if let Some(val) = other.variables.get(key) {
                self.variables.insert(key.to_string(), val.clone());
            }
        }
    }

    pub fn execute(&mut self) -> Message {
        let commands = self.lines.nodes.clone();
        if let Some(err) = self.execute_commands(commands) {
            return Message::create_error_message(err);
        }
        Message::from_canvas(self.canvas.clone())
    }

    pub fn execute_commands(&mut self, nodes : Vec<AstNode>) -> Option<Error> {
        for line in nodes {
            match line {
                AstNode::Command { name, args } => {
                    if let Some(err) = self.execute_function(&name, args) {
                       return Some(err);
                    }
                },
                AstNode::Init { typ: _, val, expr } => {
                    if let Some(err) = self.execute_init(&val, expr) {
                        return Some(err);
                    }
                }
                
                AstNode::If { clause, block, else_block } => {
                    let val = self.calculate_expression(clause);
                    match val {
                        Ok(BaseValue::Bool(t)) => {
                            let mut new_exec = self.clone();
                            if t {
                                new_exec.execute_commands(block.nodes);
                            } else if else_block.is_some() {
                                new_exec.execute_commands(else_block.unwrap().nodes);
                            }
                            self.canvas = new_exec.canvas;
                        }
                        Err(err) => {
                            return Some(err);
                        }
                        _ => unreachable!("Unexpected code 1")
                    }
                },
                AstNode::While { clause, block } => {
                    loop {
                        let val = self.calculate_expression(clause.clone());
                        match val {
                            Ok(BaseValue::Bool(t)) => {
                                if t {
                                    let mut new_exec = self.clone();
                                    new_exec.execute_commands(block.nodes.clone());
                                    self.update_variables(&new_exec);
                                    self.canvas = new_exec.canvas;
                                } else {
                                    break;
                                }
                            }
                            Err(err) => {
                                return Some(err);
                            }
                            _ => unreachable!("Unexpected code 2")
                        }
                    }
                },
                AstNode::For { val, from, to, block } => {
                    if let BaseValue::Int(f) = from {
                        if let BaseValue::Int(t) = to {
                            if f <= t {
                                for cycle in f..=t {
                                    self.execute_for(val.clone(), cycle, block.clone());
                                }
                            } else {
                                for cycle in (t..=f).rev() {
                                    self.execute_for(val.clone(), cycle, block.clone());
                                }
                            }                       
                        }
                    }
                },
            }
        }
        None
    }

    fn execute_for(&mut self, val: String, cycle : i32, block : AstBlock) {
        self.execute_init(&val, Expression::Value(BaseValue::Int(cycle)));
        let mut new_exec = self.clone();
        new_exec.execute_commands(block.nodes.clone());
        self.update_variables(&new_exec);
        self.canvas = new_exec.canvas;
    }

    fn calculate_expression(&self, expr: Expression) -> Result<BaseValue, Error> {
        match expr {
            Expression::Value(base_value) => {
                match base_value {
                    BaseValue::Id(var) => {
                        if let Some(val) = self.variables.get(&var) {
                            return Ok(val.clone());
                        } else {
                            return Err(Error::RuntimeError { message: format!("Unknown variable: {}", var).into() })
                        }
                    },
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




use std::collections::{HashMap, HashSet};

use quanta_parser::{ast::{AstBlock, BaseValue, BaseType, Expression, Operator, UnaryOperator, AstNode}, error::Error};

use crate::{utils::{canvas::Canvas, message::Message}, program::Program};

#[derive(Debug, Clone)]
pub struct Execution {
    pub lines: AstBlock, 
    pub variables : HashMap<String, BaseValue>,
    pub canvas    : Canvas,
    figureColor : BaseValue,
    lineColor : BaseValue
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
        match function_name {
            "circle" => {
                print!("Print circle!!!");
                if let BaseValue::Int(x) = vals[0] {
                    if let BaseValue::Int(y) = vals[1] {
                        if let BaseValue::Int(r) = vals[2] {
                            for i in x-r..x+r {
                                for j in y-r..y+r {
                                    if (i - x) * (i - x) + (j - y) * (j - y) == r * r {
                                        self.canvas.setPixel(i, j, self.lineColor.clone());
                                    } else if (i - x) * (i - x) + (j - y) * (j - y) < r * r {
                                        self.canvas.setPixel(i, j, self.figureColor.clone());
                                    }
                                }
                            }
                            return None;
                        }
                    }
                }
                return Some(Error::RuntimeError { message: "Incorrect arguments for circle function!".into() })
            },
            "line" => {
                if let BaseValue::Int(x1) = vals[0] {
                    print!("Line number {}", x1);
                    if let BaseValue::Int(y1) = vals[1] {
                        if let BaseValue::Int(x2) = vals[2] {
                            if let BaseValue::Int(y2) = vals[3] {
                                if x2 == x1 {
                                    for i in y1 .. y2 {
                                        self.canvas.setPixel(x1, i, self.lineColor.clone());
                                    }
                                    return None;
                                }
                                if y2 == y1 {
                                    for i in x1 .. x2 {
                                        self.canvas.setPixel(i, y1, self.lineColor.clone());
                                    }
                                    return None;
                                }
                                for x3 in x1..x2 {
                                    for y3 in y1..y2 {
                                        if (x3 as f32 - x1 as f32) / (x2 as f32 - x1 as f32) == (y3 as f32 - y1 as f32) / (y2 as f32 - y1 as f32) {
                                            self.canvas.setPixel(x3, y3, self.lineColor.clone());
                                        }
                                    }
                                }
                                return None;
                            }
                        }
                    }
                }
                return Some(Error::RuntimeError { message: "Incorrect arguments for line function!".into() })
            },
            "rectangle" => {
                print!("rectangle!");
                if let BaseValue::Int(x1) = vals[0] {
                    if let BaseValue::Int(y1) = vals[1] {
                        if let BaseValue::Int(x2) = vals[2] {
                            if let BaseValue::Int(y2) = vals[3] {
                                for x3 in x1..x2 {
                                    for y3 in y1..y2 {
                                        if (x3 == x1 || x3 == x2) && (y3 == y1 || y3 == y2) {
                                            self.canvas.setPixel(x3, y3, self.lineColor.clone());
                                        } else {
                                            self.canvas.setPixel(x3, y3, self.figureColor.clone());
                                        }
                                    }
                                }
                                return None;
                            }
                        }
                    }
                }
                return Some(Error::RuntimeError { message: "Incorrect arguments for rectangle function!".into() })
            },
            "setLineColor" => {
                if let BaseValue::Color(r,g,b) = vals[0] {
                    self.lineColor = vals[0].clone();
                }
                return Some(Error::RuntimeError { message: "Incorrect arguments for setLineColor function!".into() })
            },
            "setFigureColor" => {
                if let BaseValue::Color(r,g,b) = vals[0] {
                    self.figureColor = vals[0].clone();
                }
                return Some(Error::RuntimeError { message: "Incorrect arguments for setFigureColor function!".into() })
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
            figureColor: BaseValue::Color(0, 0, 0),
            lineColor: BaseValue::Color(255, 255, 255),
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
        // self.functions =
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
                        _ => unreachable!()
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
                            _ => unreachable!()
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
            Expression::Value(baseValue) => {
                match baseValue {
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
                            id =>unreachable!()
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
        
        Operator::AND => unreachable!(),
        Operator::OR => unreachable!(),
        
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
        
        Operator::AND => unreachable!(),
        Operator::OR => unreachable!(),
        
        Operator::Plus => Ok(BaseValue::Float(x + y)),
        Operator::Minus => Ok(BaseValue::Float(x - y)),
        Operator::Mult => Ok(BaseValue::Float(x * y)),
        Operator::Div => Ok(BaseValue::Float(x / y)),
        Operator::Mod => Ok(BaseValue::Float(x % y)),
    }
}

fn compare_bools(a: bool, b : bool, op: Operator) -> Result<BaseValue, Error> {
    match op {

        Operator::EQ => unreachable!(),
        Operator::NQ => unreachable!(),
        Operator::GT => unreachable!(),
        Operator::LT => unreachable!(),
        Operator::GQ => unreachable!(),
        Operator::LQ => unreachable!(),
        
        Operator::AND => Ok(BaseValue::Bool(a && b)),
        Operator::OR => Ok(BaseValue::Bool(a || b)),
        
        Operator::Plus => unreachable!(),
        Operator::Minus => unreachable!(),
        Operator::Mult => unreachable!(),
        Operator::Div => unreachable!(),
        Operator::Mod =>unreachable!(),
    }
}




use std::fmt;

use pest::iterators::Pairs;

use crate::Rule;

pub mod builder;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseType {
    Int,
    Bool,
    Color,
    Float
}

impl BaseType {
    pub fn to_string(&self) -> String {
        match self {
            BaseType::Int => "int".to_string(),
            BaseType::Bool => "bool".to_string(),
            BaseType::Color => "color".to_string(),
            BaseType::Float => "float".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VariableCall {
    Name(String),
    ArrayCall(String, Vec<SimpleExpression>)
}

impl fmt::Display for VariableCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableCall::Name(name) => write!(f, "{}", name),
            VariableCall::ArrayCall(name, indices) => {
                write!(f, "{}[", name)?;
                for (i, index) in indices.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", index)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl fmt::Display for SimpleExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleExpression::Value(value) => write!(f, "{}", value),
            SimpleExpression::Unary(op, expr) => write!(f, "{:?}({})", op, expr),
            SimpleExpression::Binary(op, left, right) => write!(f, "({} {:?} {})", left, op, right),
        }
    }
}

impl fmt::Display for SimpleValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleValue::Id(var) => write!(f, "{}", var),
            SimpleValue::Int(value) => write!(f, "{}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimpleValue {
    Id(VariableCall),
    Int(i32)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseValue {
    Id(VariableCall),
    Int(i32),
    Bool(bool),
    Color(u8, u8, u8),
    RandomColor,
    Float(f32),
    Array(Option<Type>, Vec<BaseValue>), // Array of BaseValues
    FunctionCall(String, Vec<Expression>, Type), // Function call with name and arguments
}




#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(BaseType),
    Array(Box<Option<Type>>, usize), // Array of a certain type with a fixed size
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Primitive(base_type) => base_type.to_string(),
            Type::Array(inner_type, size) => {
                let inner = match inner_type.as_ref() {
                    Some(t) => t.to_string(),
                    None => "()".to_string(),
                };
                format!("array<{},{}>", inner, size)
            }
        }
    }
}

impl BaseValue {
    pub fn get_type(&self) -> Type {
        match self {
            BaseValue::Id(_) => Type::Primitive(BaseType::Int), // Default type for identifiers
            BaseValue::Int(_) => Type::Primitive(BaseType::Int),
            BaseValue::Bool(_) => Type::Primitive(BaseType::Bool),
            BaseValue::Color(_, _, _) => Type::Primitive(BaseType::Color),
            BaseValue::RandomColor => Type::Primitive(BaseType::Color),
            BaseValue::Float(_) => Type::Primitive(BaseType::Float),
            BaseValue::Array(inner_type, elems) => {
                if elems.is_empty() || inner_type.is_none() {
                    return Type::Array(Box::new(None), 0);
                }
                let inner = inner_type.as_ref().unwrap().clone();
                Type::Array(Box::new(Some(inner)), elems.len())
            }
            BaseValue::FunctionCall(_, _, t) => {
                t.clone()
            }
        }
    }
}

impl Eq for BaseValue {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    UnaryMinus,
    Parentheses
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    EQ,
    NQ,
    GT,
    LT,
    GQ,
    LQ,

    AND,
    OR,

    Plus,
    Minus,
    Mult,
    Div, 
    Mod
}

pub fn is_arith(op : Operator) -> bool {
    op == Operator::Plus  ||
    op == Operator::Minus || 
    op == Operator::Mult  ||
    op == Operator::Div   ||
    op == Operator::Mod
}

fn prec(op : Operator) -> i32 {
    if op == Operator::AND || op == Operator::OR {
        return 10;
    } else if op == Operator::Mult || op == Operator::Div || op == Operator::Mod {
        return 6;
    } else if op == Operator::Plus || op == Operator::Minus {
        return 4;
    } else {
        return 8;
    }
}
 
pub fn goes_before(op1 : Operator,  op2: Operator) -> bool {
    return prec(op1) >= prec(op2)
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimpleExpression {
    Value(SimpleValue),
    Unary(UnaryOperator, Box<SimpleExpression>),
    Binary(Operator, Box<SimpleExpression>, Box<SimpleExpression>)
}

impl SimpleExpression {
    pub fn to_expr(self) -> Expression {
        match self {
            SimpleExpression::Value(value) => {
                Expression::Value(match value {
                    SimpleValue::Id(var) => BaseValue::Id(var),
                    SimpleValue::Int(i) => BaseValue::Int(i),
                })   
            }
            SimpleExpression::Unary(op, expr) => Expression::Unary(op, Box::new(expr.to_expr())),
            SimpleExpression::Binary(op, left, right) => {
                Expression::Binary(op, Box::new(left.to_expr()), Box::new(right.to_expr()))
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Value(BaseValue),
    Unary(UnaryOperator, Box<Expression>),
    Binary(Operator, Box<Expression>, Box<Expression>)
}
#[derive(Debug, Clone)]
pub enum AstNode {
    Command { name: String, args: Vec<Expression> },
    Init    { typ: Type, val : String, expr: Expression },
    SetVal { val: VariableCall, expr: Expression },
    For     { val: String, from: BaseValue, to: BaseValue, block: AstBlock },
    While   { clause: Expression, block: AstBlock},
    If      { clause: Expression, block: AstBlock, else_block: Option<AstBlock>},
    Return  { expr: Expression },
}
#[derive(Debug, Clone)]
pub struct AstBlock {
    pub nodes : Vec<AstNode>
}


#[derive(Debug, Clone)]
pub struct HalfParsedAstFunction<'a> {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub statements: Pairs<'a, Rule>
}

#[derive(Debug, Clone)]
pub struct AstFunction {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub block: AstBlock
}

#[derive(Debug, Clone)]
pub enum AstProgram {
    Block(AstBlock),
    Forest(Vec<AstFunction>)
}
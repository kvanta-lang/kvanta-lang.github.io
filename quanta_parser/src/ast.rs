
pub mod builder;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseType {
    Int,
    Bool,
    Color,
    Float,
    ErrorType(String), // For error handling, not a real type
}
#[derive(Debug, Clone)]
pub enum BaseValue {
    Id(String),
    Int(i32),
    Bool(bool),
    Color(u8, u8, u8),
    RandomColor,
    Float(f32),
    Array(Option<Type>, Vec<BaseValue>), // Array of BaseValues
}




#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Primitive(BaseType),
    Array(Box<Option<Type>>, usize), // Array of a certain type with a fixed size
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
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOperator {
    UnaryMinus,
    Parentheses
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone)]
pub enum Expression {
    Value(BaseValue),
    Unary(UnaryOperator, Box<Expression>),
    Binary(Operator, Box<Expression>, Box<Expression>)
}
#[derive(Debug, Clone)]
pub enum AstNode {
    Command { name: String, args: Vec<Expression> },
    Init    { typ: Option<Type>, val : String, expr: Expression },
    For     { val: String, from: BaseValue, to: BaseValue, block: AstBlock },
    While   { clause: Expression, block: AstBlock},
    If      { clause: Expression, block: AstBlock, else_block: Option<AstBlock>}
}
#[derive(Debug, Clone)]
pub struct AstBlock {
    pub nodes : Vec<AstNode>
}
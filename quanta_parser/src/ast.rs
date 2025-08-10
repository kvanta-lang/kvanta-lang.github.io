pub mod builder;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseType {
    Int,
    Bool,
    Color,
    Float
}
#[derive(Debug, Clone)]
pub enum BaseValue {
    Id(String),
    Int(i32),
    Bool(bool),
    Color(u8, u8, u8),
    RandomColor,
    Float(f32)
}
#[derive(Debug, Clone, Copy)]
pub enum Type {
    Primitive(BaseType),
    ArrayOneD(BaseType),
    ArrayTwoD(BaseType)
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
    Init    { typ: Option<BaseType>, val : String, expr: Expression },
    For     { val: String, from: BaseValue, to: BaseValue, block: AstBlock },
    While   { clause: Expression, block: AstBlock},
    If      { clause: Expression, block: AstBlock, else_block: Option<AstBlock>}
}
#[derive(Debug, Clone)]
pub struct AstBlock {
    pub nodes : Vec<AstNode>
}
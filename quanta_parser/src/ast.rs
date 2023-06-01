pub mod builder;
#[derive(Debug)]
pub enum BaseType {
    Int,
    Bool,
    Color,
    Float
}
#[derive(Debug)]
pub enum BaseValue {
    Id(String),
    Int(i32),
    Bool(bool),
    Color(u8, u8, u8),
    Float(f32)
}
#[derive(Debug)]
pub enum Type {
    Primitive(BaseType),
    ArrayOneD(BaseType),
    ArrayTwoD(BaseType)
}
#[derive(Debug)]
pub enum LogicalExpression {
    Value(BaseValue),
    EQ(Box<Expression>, Box<Expression>),
    NQ(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    GQ(Box<Expression>, Box<Expression>),
    LQ(Box<Expression>, Box<Expression>),
    AND(Box<Expression>, Box<Expression>),
    OR(Box<Expression>, Box<Expression>)
}
#[derive(Debug)]
pub enum ArithmeticExpression {
    Value(BaseValue),
    UnaryMinus(Box<Expression>),
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>), 
    Mod(Box<Expression>, Box<Expression>)
}
#[derive(Debug)]
pub enum Expression {
    Value(BaseValue),
    Logical(LogicalExpression),
    Arithmetic(ArithmeticExpression),
    Parenthes(Box<Expression>)
}
#[derive(Debug)]
pub enum AstNode {
    Command { name: String, args: Vec<Expression> },
    Init    { typ: Option<BaseType>, val : String, expr: Expression },
    For     { val: String, from: BaseValue, to: BaseValue, block: AstBlock },
    While   { clause: Expression, block: AstBlock},
    If      { clause: Expression, block: AstBlock, else_block: Option<AstBlock>}
}
#[derive(Debug)]
pub struct AstBlock {
    pub(crate) nodes : Vec<AstNode>
}
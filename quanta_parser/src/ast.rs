pub mod builder;

pub enum BaseType {
    Int,
    Bool,
    Color,
    Float
}

pub enum BaseValue<'a> {
    Id(&'a str),
    Int(i32),
    Bool(bool),
    Color(u8, u8, u8),
    Float(f32)
}

pub enum Type {
    Primitive(BaseType),
    ArrayOneD(BaseType),
    ArrayTwoD(BaseType)
}

pub enum LogicalExpression<'a> {
    Value(BaseValue<'a>),
    EQ(Box<LogicalExpression<'a>>, Box<LogicalExpression<'a>>),
    NQ(Box<LogicalExpression<'a>>, Box<LogicalExpression<'a>>),
    GT(Box<LogicalExpression<'a>>, Box<LogicalExpression<'a>>),
    LT(Box<LogicalExpression<'a>>, Box<LogicalExpression<'a>>),
    GQ(Box<LogicalExpression<'a>>, Box<LogicalExpression<'a>>),
    LQ(Box<LogicalExpression<'a>>, Box<LogicalExpression<'a>>)
}

pub enum ArithmeticExpression<'a> {
    Value(BaseValue<'a>),
    UnaryMinus(Box<ArithmeticExpression<'a>>),
    Plus(Box<ArithmeticExpression<'a>>, Box<ArithmeticExpression<'a>>),
    Minus(Box<ArithmeticExpression<'a>>, Box<ArithmeticExpression<'a>>),
    Mult(Box<ArithmeticExpression<'a>>, Box<ArithmeticExpression<'a>>),
    Div(Box<ArithmeticExpression<'a>>, Box<ArithmeticExpression<'a>>), 
    Mod(Box<ArithmeticExpression<'a>>, Box<ArithmeticExpression<'a>>)
}

pub enum Expression<'a> {
    Primitive(Type),
    Logical(LogicalExpression<'a>),
    Arithmetic(ArithmeticExpression<'a>)
}

pub enum AstNode<'a> {
    Command { name: &'a str, args: Vec<Expression<'a>> },
    Init    { typ: Option<Type>, val : &'a str, expr: Expression<'a> },
    For     { val: &'a str, from: Expression<'a>, to: Expression<'a>, block: AstBlock<'a> },
    While   { clause: Expression<'a>, block: AstBlock<'a>},
    If      { clause: Expression<'a>, block: AstBlock<'a>}
}

pub struct AstBlock<'a> {
    pub(crate) nodes : Vec<AstNode<'a>>
}
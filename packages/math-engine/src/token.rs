#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Variable(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Equal,
    OpenParen,
    CloseParen,
    EOF,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
}
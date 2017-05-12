use super::super::super::Value;

#[derive(Debug, Clone)]
pub enum Expression {
    Atom(Value),
    Identifier(String),
    Operation(Box<Expression>, Operand, Box<Expression>),
    Call(Box<Vec<Expression>>),
    Function(Function),
    Return(Option<Box<Expression>>),
}
 
#[derive(Debug, Clone)]
pub enum Statement {
    Block(Box<Vec<Statement>>),
    Definition(String, Box<Expression>),
    Expression(Box<Expression>),
    Assignment(String, Box<Expression>),
    If(Box<Expression>, Box<Vec<Statement>>),
    IfElse(Box<Expression>, Box<Vec<Statement>>, Box<Vec<Statement>>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Option<String>,
    pub args: Vec<String>,
    pub body: Option<Vec<Statement>>,
}

impl Function {
    pub fn new(name: Option<String>, args: Vec<String>, body: Option<Vec<Statement>>) -> Function {
        Function {
            name, args, body,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Mul,
    Div,
    Plus,
    Minus,
    Equals,
    NEquals,
    Lt,
    Gt,
    LtEquals,
    GtEquals,
}

pub fn operand(v: &str) -> Option<(Operand, u8)> {
    match v {
        "*"  => Some((Operand::Mul, 1)),
        "/"  => Some((Operand::Div, 1)),
        "+"  => Some((Operand::Plus, 2)),
        "-"  => Some((Operand::Minus, 2)),
        "==" => Some((Operand::Equals, 3)),
        "!=" => Some((Operand::NEquals, 3)),
        "<"  => Some((Operand::Lt, 4)),
        ">"  => Some((Operand::Gt, 4)),
        "<=" => Some((Operand::LtEquals, 4)),
        ">=" => Some((Operand::GtEquals, 4)),
        _ => None,
    }
}
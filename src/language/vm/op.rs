use vm::Value;

#[derive(Debug, Clone)]
pub enum Op {
    Value(Value),
    Add,
    Sub,
    Mul,
    Div,
    Equals,
    NEquals,
    Lt,
    Gt,
    LtEquals,
    GtEquals,
    Return,
    Define,
    Call,
    JumpUnless(i32),
    JumpIf(i32),
    Jump(i32),
    Name(String),
}
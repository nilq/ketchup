pub mod syntax;
pub mod vm;

pub use vm::Value;

pub mod compiler {
    use vm::Op;
    use syntax::parser::{Expression, Statement, Operand};

    pub fn expression(script: &mut Vec<Op>, expr: &Expression) {
        match *expr {
            Expression::Atom(ref v) => script.push(Op::Value(v.clone())),
            Expression::Operation(ref l, ref op, ref r) => {
                expression(script, r);
                expression(script, l);
                match *op {
                    Operand::Plus     => script.push(Op::Add),
                    Operand::Minus    => script.push(Op::Sub),
                    Operand::Mul      => script.push(Op::Mul),
                    Operand::Div      => script.push(Op::Div),
                    Operand::Equals   => script.push(Op::Equals),
                    Operand::NEquals  => script.push(Op::NEquals),
                    Operand::Lt       => script.push(Op::Lt),
                    Operand::Gt       => script.push(Op::Gt),
                    Operand::LtEquals => script.push(Op::LtEquals),
                    Operand::GtEquals => script.push(Op::GtEquals),
                }
            },
            _ => panic!("unimplemented expression!") ,
        }
    }

    pub fn statements(stream: Vec<Statement>) -> Vec<Op> {
        let mut script = Vec::new();

        for s in stream {
            match s {
                Statement::Expression(e) => expression(&mut script, &e),
                Statement::Block(ve)     => script.append(&mut statements(*ve)),
            }
        }

        script
    }
}
pub mod syntax;

#[macro_use]
pub mod vm;

pub use vm::Value;

pub mod natives {
    use std::collections::HashMap;
    
    use vm::{Op, Value, Object, Native};

    pub fn apply(scope: &mut HashMap<String, Value>) {
        native!("putsln", putsln, scope);
        native!("puts", puts, scope);
        native!("angry", angry, scope);
    }

    fn putsln(args: Vec<Value>) -> Value {
        let s : Vec<String> = args.iter().map(
            |ref v| format!("{}", v)
        ).collect();

        let joined = s.join(" ");

        println!("{}", joined);

        Value::StringLiteral(joined)
    }

    fn puts(args: Vec<Value>) -> Value {
        let s : Vec<String> = args.iter().map(
            |ref v| format!("{}", v)
        ).collect();

        let joined = s.join(" ");

        print!("{}", joined);

        Value::StringLiteral(joined)
    }

    fn angry(args: Vec<Value>) -> Value {
        let s : Vec<String> = args.iter().map(
            |ref v| format!("{}", v)
        ).collect();

        panic!(s.join(" "));

        Value::Nil
    }
}

pub mod compiler {
    use vm::{Op, Value, Object};
    use syntax::parser::{Expression, Statement, Operand, Function};

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
            Expression::Identifier(ref n) => script.push(Op::Name(n.clone())),
            Expression::Call(ref args)    => {
                for a in &**args {
                    expression(script, &a)
                }

                script.push(Op::Value(Value::IntLiteral((args.len() as i64) - 1)));
                script.push(Op::Call)
            },
            Expression::Function(Function {ref name, ref args, ref body}) => {
                let body = match body {
                    &Some(ref b) => statements(b.clone()),
                    &None    => vec!(Op::Value(Value::Nil)), // xd this is good haha
                };
                let obj  = Object::Function {
                    args: args.clone(),
                    body,
                };

                script.push(Op::Value(Value::Object(obj)));

                match *name {
                    Some(ref n) => {
                        script.push(Op::Value(Value::StringLiteral(n.clone())));
                        script.push(Op::Define)
                    },
                    None => (),
                }
            },
            Expression::Return(ref e) => {
                match e.clone() {
                    Some(e) => expression(script, &*e),
                    None    => script.push(Op::Value(Value::Nil)),
                }

                script.push(Op::Return)
            },
            _ => panic!("unimplemented expression!") ,
        }
    }

    fn assignment(script: &mut Vec<Op>, name: &str, expr: &Expression) {
        expression(script, expr);

        script.push(Op::Value(Value::StringLiteral(name.to_owned())));
        script.push(Op::Define)
    }

    pub fn statements(stream: Vec<Statement>) -> Vec<Op> {
        let mut script = Vec::new();

        for s in stream {
            match s {
                Statement::Expression(e)    => expression(&mut script, &e),
                Statement::Block(ve)        => script.append(&mut statements(*ve)),
                Statement::Definition(n, e) => assignment(&mut script, &n, &*e),
                Statement::If(cond, body)   => {
                    expression(&mut script, &cond);
                    let body = statements(*body);

                    script.push(Op::JumpUnless(body.len() as i32 + 1));
                    script.extend(body.iter().cloned());

                    expression(&mut script, &cond)
                },
                Statement::IfElse(cond, body, else_body)   => {
                    expression(&mut script, &cond);
                    let body = statements(*body);

                    script.push(Op::JumpUnless(body.len() as i32 + 1));
                    script.extend(body.iter().cloned());

                    expression(&mut script, &cond);

                    let else_body = statements(*else_body);

                    script.push(Op::JumpIf(else_body.len() as i32 + 1));
                    script.extend(else_body.iter().cloned())
                },
                Statement::Assignment(ref id, ref expr) => assignment(&mut script, &id, &*expr),
                _ => panic!("unstable/unimplemented statement!?")
            }
        }

        script
    }
}
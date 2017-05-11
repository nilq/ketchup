use std::collections::HashMap;

pub mod value;
pub mod op;

#[macro_use]
pub mod object;

pub use self::value::Value;
pub use self::op::Op;
pub use self::object::*;

macro_rules! binary_op {
    ($vm_ref:expr, $a:ident, $b:ident, $r:expr) => {
        { let $b = $vm_ref.stack.pop().unwrap();
          let $a = $vm_ref.stack.pop().unwrap();
          $vm_ref.stack.push($r);
        }
    }
}

pub struct Machine {
    program: Vec<Op>,
    stack:   Vec<Value>,
    pointer: usize,
    running: bool,
}

impl Machine {
    pub fn new(program: Vec<Op>) -> Machine {
        Machine {
            program,
            stack: Vec::new(),
            pointer: 0,
            running: false,
        }
    }

    pub fn run(&mut self, scopes: &mut HashMap<String, Value>) -> Result<Option<Value>, String> {
        self.running = true;

        while self.running && self.pointer < self.program.len() {
            match self.program[self.pointer] {
                Op::Value(ref v) => self.stack.push(v.clone()),
                Op::Add      => binary_op!(self, a, b, try!(a.add(b))),
                Op::Sub      => binary_op!(self, a, b, try!(a.sub(b))),
                Op::Mul      => binary_op!(self, a, b, try!(a.mul(b))),
                Op::Div      => binary_op!(self, a, b, try!(a.div(b))),
                Op::Equals   => binary_op!(self, a, b, Value::BoolLiteral(a == b)),
                Op::NEquals  => binary_op!(self, a, b, Value::BoolLiteral(a != b)),
                Op::Lt       => binary_op!(self, a, b, Value::BoolLiteral(a < b)),
                Op::LtEquals => binary_op!(self, a, b, Value::BoolLiteral(a <= b)),
                Op::Gt       => binary_op!(self, a, b, Value::BoolLiteral(a > b)),
                Op::GtEquals => binary_op!(self, a, b, Value::BoolLiteral(a >= b)),
                Op::Define   => match self.stack.pop().unwrap() {
                    Value::StringLiteral(s) => { scopes.insert(s, self.stack.pop().unwrap()); },
                    _                       => return Err("very broken assignment code?!".to_owned()),
                },
                Op::Name(ref n) => match scopes.get(n) {
                    Some(v) => self.stack.push(v.clone()),
                    None => (),
                },
                Op::Name(ref n) => {
                    match scopes.get(n) {
                        Some(v) => self.stack.push(v.clone()),
                        None    => (),
                    }
                },
                Op::JumpUnless(ref n) => if !self.stack.pop().unwrap().to_boolean() {
                    self.pointer = (self.pointer as i32 + *n) as usize;
                    continue
                },
                Op::JumpIf(ref n) => if self.stack.pop().unwrap().to_boolean() {
                    self.pointer = (self.pointer as i32 + *n) as usize;
                    continue
                },
                Op::Jump(ref n) => {
                    self.pointer = (self.pointer as i32 + *n) as usize;
                    continue
                },
                Op::Return => self.running = false,
                Op::Call => {
                    let mut len = match self.stack.pop().unwrap() {
                        Value::IntLiteral(n) => n as usize,
                        _ => panic!("very bad arg-len bytecode!"),
                    };

                    let mut values = Vec::new();

                    for _ in 0 .. len {
                        values.push(self.stack.pop().unwrap())
                    }


                    match self.stack.pop().unwrap() {
                        Value::Object(o) => {
                            match o {
                                Object::Function {args, body} => {
                                    let mut frame = Machine::new(body);
                                    for arg in args {
                                        if len > 0 {
                                            scopes.insert(arg.clone(), values.pop().unwrap());
                                            len -= 1
                                        } else {
                                            scopes.insert(arg.clone(), Value::Nil);
                                        }
                                    }
                                    match try!(frame.run(scopes)) {
                                        Some(result) => self.stack.push(result),
                                        None         => (),
                                    }
                                },
                                Object::Native(nobj) => {
                                   match nobj {
                                      Native::Function(f) => {
                                          values.reverse();
                                          self.stack.push(f(values));
                                      },
                                   }
                                },
                            }
                        },
                        s => {
                            println!("found '{}' on stack", s);
                            return Err("very invalid and broken call!".to_owned());
                        },
                    }
                },
                _ => panic!("angery, not covered!?!"),
            }

            self.pointer += 1
        }

        self.running = false;
        Ok(self.stack.last().cloned())
    }
}
use std::collections::HashMap;

pub mod value;
pub mod op;
pub mod object;

pub use self::value::Value;
pub use self::op::Op;
pub use self::object::Object;

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
                Op::Name(ref n) => match scopes.get(n) {
                    Some(v) => self.stack.push(v.clone()),
                    None => (),
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
                _ => panic!("angery not covered!?!"),
            }
        }

        self.running = false;
        Ok(self.stack.last().cloned())
    }
}
use vm::Object;

use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),
    Object(Object),
    Nil,
}

#[allow(dead_code)]
impl Value {
    pub fn add(&self, b: Value) -> Result<Value, String> {
        Ok(match (self, b) {
            (&Value::IntLiteral(a),   Value::IntLiteral(b))             => Value::IntLiteral(a + b),
            (&Value::FloatLiteral(a), Value::FloatLiteral(b))           => Value::FloatLiteral(a + b),
            (&Value::FloatLiteral(a), Value::IntLiteral(b))             => Value::FloatLiteral(a + (b as f64)),
            (&Value::IntLiteral(a),   Value::FloatLiteral(b))           => Value::FloatLiteral((a as f64) + b),
            (&Value::StringLiteral(ref a), Value::StringLiteral(ref b)) => Value::StringLiteral(a.clone() + b.as_str()),
            (&Value::StringLiteral(ref a), Value::CharLiteral(ref b))   => Value::StringLiteral(a.clone() + &b.to_string()),
            (&Value::CharLiteral(a),   Value::CharLiteral(b))           => Value::StringLiteral(format!("{}{}", a, b)),

            _ => return Err("invalid operation".to_owned())
        })
    }

    pub fn sub(&self, b: Value) -> Result<Value, String> {
        Ok(match (self, b) {
            (&Value::IntLiteral(a),   Value::IntLiteral(b))   => Value::IntLiteral(a - b),
            (&Value::FloatLiteral(a), Value::IntLiteral(b))   => Value::FloatLiteral(a - (b as f64)),
            (&Value::IntLiteral(a), Value::FloatLiteral(b))   => Value::FloatLiteral((a as f64) - b),
            (&Value::FloatLiteral(a), Value::FloatLiteral(b)) => Value::FloatLiteral(a - b),

            _ => return Err("invalid operation".to_owned())
        })
    }

    pub fn mul(&self, b: Value) -> Result<Value, String> {
        Ok(match (self, b) {
            (&Value::IntLiteral(a),   Value::IntLiteral(b))   => Value::IntLiteral(a * b),
            (&Value::FloatLiteral(a), Value::IntLiteral(b))   => Value::FloatLiteral(a * (b as f64)),
            (&Value::IntLiteral(a), Value::FloatLiteral(b))   => Value::FloatLiteral((a as f64) * b),
            (&Value::FloatLiteral(a), Value::FloatLiteral(b)) => Value::FloatLiteral(a * b),

            _ => return Err("invalid operation".to_owned())
        })
    }

    pub fn div(&self, b: Value) -> Result<Value, String> {
        Ok(match (self, b) {
            (&Value::IntLiteral(a),   Value::IntLiteral(b))   => Value::IntLiteral(a / b),
            (&Value::FloatLiteral(a), Value::IntLiteral(b))   => Value::FloatLiteral(a / (b as f64)),
            (&Value::IntLiteral(a), Value::FloatLiteral(b))   => Value::FloatLiteral((a as f64) / b),
            (&Value::FloatLiteral(a), Value::FloatLiteral(b)) => Value::FloatLiteral(a / b),

            _ => return Err("invalid operation".to_owned())
        })
    }

    pub fn to_boolean(&self) -> bool {
        match *self {
            Value::IntLiteral(v)        => v != 0,
            Value::FloatLiteral(v)      => v != 0.0,
            Value::StringLiteral(ref v) => v.len() > 0,
            Value::BoolLiteral(v)    => v,
            Value::CharLiteral(_)       => true,
            Value::Object(_)            => true,
            Value::Nil            => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::IntLiteral(ref v)    => write!(f, "{}", v),
            Value::FloatLiteral(ref v)  => write!(f, "{}", v),
            Value::BoolLiteral(ref b)   => write!(f, "{}", b),
            Value::StringLiteral(ref s) => write!(f, "{}", s),
            Value::CharLiteral(ref c)   => write!(f, "{}", c),
            Value::Object(_)            => write!(f, "[object]"),
            Value::Nil                  => write!(f, "nil"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::IntLiteral(a), &Value::IntLiteral(b)) => a == b,
            (&Value::FloatLiteral(a), &Value::FloatLiteral(b)) =>
                if a == b {
                    a.is_sign_negative() == b.is_sign_negative()
                } else {
                    a.is_nan() && b.is_nan()
                },
            (&Value::FloatLiteral(a), &Value::IntLiteral(b)) => {
                let b = b as f64;
                if a == b {
                    a.is_sign_negative() == b.is_sign_negative()
                } else {
                    a.is_nan() && b.is_nan()
                }
            },
            (&Value::IntLiteral(a), &Value::FloatLiteral(b)) => {
                let a = a as f64;
                if a == b {
                    a.is_sign_negative() == b.is_sign_negative()
                } else {
                    a.is_nan() && b.is_nan()
                }
            },
            (&Value::BoolLiteral(ref a), &Value::BoolLiteral(ref b)) => a == b,
            (&Value::StringLiteral(ref a), &Value::StringLiteral(ref b)) => a == b,
            (&Value::Nil, &Value::Nil) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        if *self == *other {
            return Some(Ordering::Equal);
        }

        match (self, other) {
            (&Value::IntLiteral(a), &Value::IntLiteral(b)) =>
                if a < b {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                },
            (&Value::IntLiteral(a), &Value::FloatLiteral(b)) =>
                if (a as f64) < b {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                },
            (&Value::FloatLiteral(a), &Value::FloatLiteral(b)) =>
                if a < b {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                },
            (&Value::FloatLiteral(a), &Value::IntLiteral(b)) => 
                if a < (b as f64) {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                },
            (&Value::BoolLiteral(ref a), &Value::BoolLiteral(ref b)) =>
                if a < b {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                },
            (&Value::StringLiteral(ref a), &Value::StringLiteral(ref b)) =>
                if a < b {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                },
            _ => None
        }
    }
}
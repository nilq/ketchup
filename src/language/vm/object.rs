use vm::{Op, Value};

#[derive(Debug, Clone)]
pub enum Native {
    Function(fn(Vec<Value>) -> Value),
}

#[derive(Debug, Clone)]
pub enum Object {
    Native(Native),
    Function {
        args: Vec<String>,
        body: Vec<Op>,
    },
}

#[macro_export]
macro_rules! native {
    ($name: expr, $func: ident, $scope: ident) => {
        $scope.insert($name.to_string(), Value::Object(Object::Native(Native::Function($func))));
    };
}
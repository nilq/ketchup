mod language;

use language::syntax;
use language::vm;
use language::compiler;

use syntax::lexer;
use syntax::parser;

use lexer::lexer;
use parser::{Traveler, Parser};

use vm::Machine;

use std::collections::HashMap;

fn main() {
    let data = r#"
var a = r"raw string\n"

fun fib(a)
  if a < 3
    return a
  return fib(a - 1) + fib(a - 2)
    "#;

    let mut working = r#"
var a = r"hello" + " world"
a
    "#.chars();

    let lexer = lexer(&mut working);

    let traveler = Traveler::new(lexer.collect());
    let mut parser = Parser::new(traveler);

    let mut scopes = HashMap::new();

    let stack = compiler::statements(parser.parse());

    println!("ops =>\n{:#?}\n", stack);

    let mut vm = Machine::new(stack);

    println!("executed =>\n{:#?}", vm.run(&mut scopes))
}
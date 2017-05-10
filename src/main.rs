mod language;

use language::syntax;
use language::vm;

use syntax::lexer;
use syntax::parser;

use lexer::lexer;
use parser::{Traveler, Parser};

fn main() {
    let data = r#"
a := r"yes hello"
c := '\n'

let foo = "an expected thing"

let h = 0xABCDE1
let b = 0b101010

func fib: a
  if a < 3
    pass a
  else
    pass fib(a - 1) + fib(a - 2)
    "#;

    let mut working = r#"
123.2
.123
r"hey\n"
"hey\n"
yes
nah
'c'
"real string"
1 + 10 * 10.2 - .2
    "#.chars();

    let lexer = lexer(&mut working);

    let traveler = Traveler::new(lexer.collect());
    let mut parser = Parser::new(traveler);

    for e in parser.parse().iter() {
        println!("{:#?}", e)
    }
}
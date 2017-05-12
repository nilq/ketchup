mod language;

use language::syntax;
use language::vm;
use language::compiler;
use language::natives;

use syntax::lexer;
use syntax::parser;

use lexer::{process_branch, lexer};
use lexer::block_tree;

use parser::{Traveler, Parser};

use vm::Machine;

use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

#[allow(dead_code)]
fn repl() {
    let mut scopes = HashMap::new();
    natives::apply(&mut scopes);

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input_line = String::new();

        match io::stdin().read_line(&mut input_line) {
            Ok(_) => {
                let s = input_line.trim();

                if s == "" {
                    continue
                } else if s == ":quit" || s == ":q" {
                    println!("=> bye");
                    std::process::exit(0)
                }

                let lexer = lexer(&mut input_line.chars());

                let traveler = Traveler::new(lexer.collect());
                let mut parser = Parser::new(traveler);

                let stack = compiler::statements(parser.parse());
                let mut vm = Machine::new(stack);

                vm.run(&mut scopes);
            }

            Err(e) => panic!(e),
        }
    }
}

fn test() {
    let mut scopes = HashMap::new();
    natives::apply(&mut scopes);

    let mut test = r#"
var s = 0

fun add(a, b)
  s = a + b

add(10, 10)
puts("heyn", s)
    "#;

    let mut tree = block_tree::BlockTree::new(test, 0);
    let indents  = &tree.collect_indents();
    
    let root = tree.tree(indents);

    let lexer = process_branch(&root);

    let traveler = Traveler::new(lexer);
    let mut parser = Parser::new(traveler);

    let p = parser.parse();

    let stack = compiler::statements(p);
    let mut vm = Machine::new(stack);

    vm.run(&mut scopes);
}

fn main() {
    test()
}
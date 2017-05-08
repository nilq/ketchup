mod syntax;

use syntax::lexer;
use lexer::block_tree;
use lexer::process_branch;

fn main() {
    let data = r#"
a := r"hey\n"

hex := 0xfff
bin := 0b101010

f := .123
g := tru or fal

fun add(a, b)
    a + b
    "#;

    let mut tree = block_tree::BlockTree::new(&data, 0);
    let indents  = tree.collect_indents();
    
    let processed_tree = process_branch(&tree.tree(&indents));

    println!("{:#?}", processed_tree);
}
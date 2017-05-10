mod syntax;

use syntax::lexer;
use lexer::block_tree;
use lexer::process_branch;

fn main() {
    let data = r#"
le a = r"hey\n"

le hex = 0xfff
le bin = 0b101010

le f = .123
le g = yes

g = nah

func add: a, b
    le r = a + b
    pass r

add(10, 10)
    "#;

    let mut tree = block_tree::BlockTree::new(&data, 0);
    let indents  = tree.collect_indents();
    
    let processed_tree = process_branch(&tree.tree(&indents));

    println!("{:#?}", processed_tree);
}
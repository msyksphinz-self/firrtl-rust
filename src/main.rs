use std::env;

use firrtl_rust::tokenizer::tokenize;
use firrtl_rust::parser::Node;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("Usage: firrtl_rust code\n");
        return;
    }

    let tokens = tokenize(args.nth(1).unwrap());
    let node = Node::parse(&tokens);

    let out_tmp = node.gen();
    println!("  assign out = {};", out_tmp);
    println!("endmodule");
}

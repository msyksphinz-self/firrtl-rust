use std::env;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("Usage: firrtl_rust code\n");
        return;
    }

    print!("module main (\n");
    print!("  output logic [31: 0] out\n");
    print!(");\n");
    print!("assign out = {};\n", args.nth(1).unwrap());
    print!("endmodule\n");
}

use std::env;

pub fn strtol(s: &String) -> (Option<i64>, String) {
    if s.is_empty() {
        return (None, s.clone());
    }

    let mut pos = 0;
    let mut remaining = s.clone();
    let len = s.len();

    while len != pos {
        if !s.chars().nth(pos).unwrap().is_ascii_digit() {
            break;
        }
        pos += 1;
    }

    if len == pos {
        (Some(remaining.parse::<i64>().unwrap()), "".into())
    } else {
        let t: String = remaining.drain(..pos).collect();
        (Some(t.parse::<i64>().unwrap()), remaining)
    }
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("Usage: firrtl_rust code\n");
        return;
    }

    let p = args.nth(1).unwrap();
    let (n, mut p) = strtol(&p);

    print!("module main (\n");
    print!("  output logic [31: 0] out\n");
    print!(");\n");
    print!("logic [31: 0] tmp0;\n");
    print!("assign tmp0 = {}\n", n.unwrap());
    let mut tmp_idx = 1;
    while let Some(c) = p.chars().nth(0) {
        let s = p.split_off(1);
        match c {
            '+' => {
                let (t, remaining) = strtol(&s);
                p = remaining;
                print!("logic [31: 0] tmp{};\n", tmp_idx);
                print!("assign tmp{} = tmp{} + {}\n", tmp_idx, tmp_idx - 1, t.unwrap());
            }
            '-' => {
                let (t, remaining) = strtol(&s);
                p = remaining;
                print!("logic [31: 0] tmp{};\n", tmp_idx);
                print!("assign tmp{} = tmp{} - {};\n", tmp_idx, tmp_idx - 1, t.unwrap());
            }
            _ => {
                eprint!("unexpected character {}\n", p);
                return;
            }
        }
        tmp_idx = tmp_idx + 1;
    }

    print!("assign out = tmp{};\n", tmp_idx-1);

    print!("endmodule\n");
}

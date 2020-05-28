use std::char;
use std::env;
use std::process::exit;

static mut REG_TMP_IDX: u32 = 0;

enum TokenType {
    Num, // Number literal
}

// Token type
#[derive(Default, Debug)]
struct Token {
    ty: i32, // Token type
    val: i32, // Number literal
    input: String, // Token string (for error reporting)
}


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


enum NodeType {
    Num, // Number literal
}

#[derive(Default, Debug, Clone)]
struct Node {
    ty: i32, // Node type
    lhs: Option<Box<Node>>, // left-hand side
    rhs: Option<Box<Node>>, // right-hand side
    val: i32, // Number literal
}


impl Node {
    fn new(op: i32, lhs: Box<Node>, rhs: Box<Node>) -> Self {
        Self {
            ty: op,
            lhs: Some(lhs),
            rhs: Some(rhs),
            ..Default::default()
        }
    }

    fn new_num(val: i32) -> Self {
        Self {
            ty: NodeType::Num as i32,
            val: val,
            ..Default::default()
        }
    }

    fn create_num(tokens: &Vec<Token>, pos: usize) -> Self {
        if tokens[pos].ty == TokenType::Num as i32 {
            let val = tokens[pos].val;
            return Self::new_num(val);
        }
        panic!("number expected, but got {}", tokens[pos].input);
    }

    pub fn expr(tokens: Vec<Token>) -> Self {
        print!("module main (\n");
        print!("  output logic [31: 0] out\n");
        print!(");\n");

        let mut pos_idx = 0;
        let mut lhs = Self::create_num(&tokens, pos_idx);
        pos_idx += 1;
        while pos_idx != tokens.len() {
            match char::from_u32(tokens[pos_idx].ty as u32) {
                Some('+') => {
                    if tokens[pos_idx+1].ty != TokenType::Num as i32 {
                        fail(&tokens, pos_idx+1);
                    }
                    lhs = Self::new(tokens[pos_idx].ty, Box::new(lhs), Box::new(Self::create_num(&tokens, pos_idx+1)));
                    pos_idx += 2;
                }
                Some('-') => {
                    if tokens[pos_idx+1].ty != TokenType::Num as i32 {
                        fail(&tokens, pos_idx+1);
                    }
                    lhs = Self::new(tokens[pos_idx].ty, Box::new(lhs), Box::new(Self::create_num(&tokens, pos_idx+1)));
                    pos_idx += 2;
                }
                _ => {
                    fail(&tokens, pos_idx);
                }
            }
        }

        if tokens.len() != pos_idx {
            panic!("stray token: {}", tokens[pos_idx].input);
        }
        return lhs;
    }


    // Code generator
    fn gen(self) -> String {

        if self.ty == NodeType::Num as i32 {
            unsafe {
                let new_tmp = format!("{}{}", "tmp", REG_TMP_IDX.to_string());
                println!("  logic [31: 0] {};", format!("{}", new_tmp));
                println!("  assign {} = {};", new_tmp, self.val);
                REG_TMP_IDX += 1;
                return new_tmp;
            }
        }

        let operand0 = self.lhs.unwrap().gen();
        let operand1 = self.rhs.unwrap().gen();
        match self.ty as u8 as char {
            '+' => {
                unsafe {
                    let new_tmp = format!("{}{}", "tmp", REG_TMP_IDX.to_string());
                    REG_TMP_IDX += 1;
                    println!("  logic [31: 0] {}", new_tmp);
                    println!("  assign {} = {} + {};", new_tmp, operand0, operand1);
                    return new_tmp;
                }
            }
            '-' => {
                unsafe {
                    let new_tmp = format!("{}{}", "tmp", REG_TMP_IDX.to_string());
                    REG_TMP_IDX += 1;
                    println!("  logic [31: 0] {}",  new_tmp);
                    println!("  assign {} = {} - {};", new_tmp, operand0, operand1);
                    return new_tmp;
                }
            }
            _ => panic!("unknown operator"),
        }
    }

}


fn tokenize(mut p: String) -> Vec<Token> {
    // Tokenized input is stored to this vec.
    let mut tokens: Vec<Token> = vec![];

    let org = p.clone();
    while let Some(c) = p.chars().nth(0) {
        // Skip whitespce
        if c.is_whitespace() {
            p = p.split_off(1); // p++
            continue;
        }

        // + or -
        if c == '+' || c == '-' {
            let token = Token {
                ty: c as i32,
                input: org.clone(),
                ..Default::default()
            };
            p = p.split_off(1); // p++
            tokens.push(token);
            continue;
        }

        // Number
        if c.is_ascii_digit() {
            let (n, remaining) = strtol(&p);
            p = remaining;
            let token = Token {
                ty: TokenType::Num as i32,
                input: org.clone(),
                val: n.unwrap() as i32,
            };
            tokens.push(token);
            continue;
        }

        eprint!("cannot tokenize: {}\n", p);
        exit(1);
    }
    return tokens;
}


fn fail(tokens: &Vec<Token>, i: usize) {
    eprint!("unexpected character: {:?}\n", tokens[i]);
    exit(1);
}


fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprint!("Usage: firrtl_rust code\n");
        return;
    }

    let tokens = tokenize(args.nth(1).unwrap());
    let node = Node::expr(tokens);

    let out_tmp = node.gen();
    println!("  assign out = {};", out_tmp);
    println!("endmodule");
}

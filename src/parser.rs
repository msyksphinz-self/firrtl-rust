use crate::tokenizer::TokenType;
use crate::tokenizer::Token;
use std::process::exit;
use std::char;

static mut REG_TMP_IDX: u32 = 0;

enum NodeType {
    Num, // Number literal
}

#[derive(Default, Debug, Clone)]
pub struct Node {
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
    pub fn gen(self) -> String {

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


fn fail(tokens: &Vec<Token>, i: usize) {
    eprint!("unexpected character: {:?}\n", tokens[i]);
    exit(1);
}

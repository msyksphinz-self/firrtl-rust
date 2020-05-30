use crate::tokenizer::TokenType;
use crate::tokenizer::Token;
use std::process::exit;
use std::char;

static mut REG_TMP_IDX: u32 = 0;

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    Num, // Number literal
    Plus,
    Minus,
    Mul,
}


impl From<TokenType> for NodeType {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::Num => NodeType::Num,
            TokenType::Plus => NodeType::Plus,
            TokenType::Minus => NodeType::Minus,
            TokenType::Mul => NodeType::Mul,
        }
    }
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Num
    }
}


#[derive(Default, Debug, Clone)]
pub struct Node {
    ty: NodeType, // Node type
    lhs: Option<Box<Node>>, // left-hand side
    rhs: Option<Box<Node>>, // right-hand side
    val: i32, // Number literal
}


impl Node {
    fn new(op: NodeType, lhs: Box<Node>, rhs: Box<Node>) -> Self {
        Self {
            ty: op,
            lhs: Some(lhs),
            rhs: Some(rhs),
            ..Default::default()
        }
    }

    fn new_num(val: i32) -> Self {
        Self {
            ty: NodeType::Num,
            val: val,
            ..Default::default()
        }
    }

    fn create_num(tokens: &Vec<Token>, pos: usize) -> Self {
        if tokens[pos].ty == TokenType::Num {
            let val = tokens[pos].val;
            return Self::new_num(val);
        }
        panic!("number expected, but got {}", tokens[pos].input);
    }


    fn mul(tokens: &Vec<Token>, mut pos: usize) -> (Self, usize) {
        let mut lhs = Self::create_num(&tokens, pos);
        pos += 1;

        loop {
            if tokens.len() == pos {
                return (lhs, pos);
            }

            let op = tokens[pos].ty.clone();
            if op != TokenType::Mul {
                return (lhs, pos);
            }
            pos += 1;
            lhs = Self::new(
                NodeType::from(op),
                Box::new(lhs),
                Box::new(Self::create_num(&tokens, pos)),
            );
            pos += 1;
        }
    }


    pub fn expr(tokens: &Vec<Token>, pos: usize) -> (Self, usize) {
        let (mut lhs, mut pos) = Self::mul(&tokens, pos);

        loop {
            if tokens.len() == pos {
                return (lhs, pos);
            }

            let op = tokens[pos].ty.clone();
            if op != TokenType::Plus && op != TokenType::Minus {
                return (lhs, pos);
            }
            pos += 1;
            let (rhs, new_pos) = Self::mul(&tokens, pos);
            pos = new_pos;
            lhs = Self::new(NodeType::from(op), Box::new(lhs), Box::new(rhs));
        }
    }

    pub fn parse(tokens: &Vec<Token>) -> Self {
        let (node, pos) = Self::expr(tokens, 0);

        if tokens.len() != pos {
            panic!("stray token: {}", tokens[pos].input);
        }
        return node;
    }

    // Code generator
    pub fn gen(self) -> String {

        print!("module main (\n");
        print!("  output logic [31: 0] out\n");
        print!(");\n");


        if self.ty == NodeType::Num {
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

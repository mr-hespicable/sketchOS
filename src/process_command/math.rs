use crate::{print, println};

use super::{CommandResult, ResultFlags};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    // vec,
    vec::Vec,
};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref PANIC: Mutex<bool> = Mutex::new(false);
}

pub fn math(command: &str) -> CommandResult {
    *PANIC.lock() = false;
    let command = command.splitn(2, " ").collect::<Vec<_>>();
    let body = command.get(1);
    let result = match body {
        Some(expr_parsed) => {
            let expr = expr_parsed.replace(" ", "");
            let expr = expr.as_str();

            let tokens = tokenize(expr);
            if !tokens.is_empty() {
                let (ast, _) = parse(&tokens);
                let eval_result = evaluate(&ast);
                CommandResult {
                    data_bytes: eval_result.to_string().as_bytes().to_vec(),
                    flags: ResultFlags::new(),
                }
            } else {
                CommandResult {
                    data_bytes: String::new().as_bytes().to_vec(),
                    flags: ResultFlags {
                        contains_result: false,
                        clear_screen: false,
                    },
                }
            }
        }
        None => {
            CommandResult {
                // math command is empty
                data_bytes: "expected >= 1 args; got 0".to_string().as_bytes().to_vec(),
                flags: ResultFlags::new(),
            }
        }
    };

    if *PANIC.lock() {
        CommandResult {
            data_bytes: "invalid arguments".to_string().as_bytes().to_vec(),
            flags: ResultFlags::new(),
        }
    } else {
        result
    }

    // let expr_parsed: String = expr_raw.replace(" ", "");
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Add,
    Subtract,
    Multiply,
    Divide,
    LParen,
    RParen,
}

enum ASTNode {
    Number(f64),
    UnaryOp {
        // eg -5
        op: Token,
        expr: Box<ASTNode>,
    },
    BinaryOp {
        // eg 2+9
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut chars = expr.chars().peekable();

    while let Some(&char) = chars.peek() {
        match char {
            '0'..='9' => {
                let mut number_str = String::new();
                while let Some(&char) = chars.peek() {
                    if char.is_ascii_digit() || char == '.' {
                        number_str.push(char);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let number = number_str.parse::<f64>().unwrap();
                tokens.push(Token::Number(number));
            }
            '+' => {
                tokens.push(Token::Add);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Subtract);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            _ => {
                print!("\nunexpected character <{}>", char);
                for _ in chars.clone() {
                    chars.next(); // advance to the end of the expression
                }
                *PANIC.lock() = true;
            }
        }
    }

    tokens
}

fn parse(tokens: &[Token]) -> (ASTNode, &[Token]) {
    parse_expression(tokens)
}

fn parse_expression(tokens: &[Token]) -> (ASTNode, &[Token]) {
    let (mut node, mut tokens) = parse_term(tokens);

    while let Some(token) = tokens.first() {
        match token {
            Token::Add | Token::Subtract => {
                let (right_node, new_tokens) = parse_term(&tokens[1..]);
                node = ASTNode::BinaryOp {
                    left: Box::new(node),
                    op: token.clone(),
                    right: Box::new(right_node),
                };
                tokens = new_tokens;
            }
            _ => break,
        }
    }

    (node, tokens)
}

fn parse_term(tokens: &[Token]) -> (ASTNode, &[Token]) {
    let nt_pair = parse_factor(tokens);
    let (mut node, tokens) = (nt_pair.node, nt_pair.tokens);
    let mut tokens = tokens;
    while let Some(token) = tokens.first() {
        match token {
            Token::Multiply | Token::Divide => {
                let (right_node, new_tokens) = parse_term(&tokens[1..]);
                node = ASTNode::BinaryOp {
                    left: Box::new(node),
                    op: token.clone(),
                    right: Box::new(right_node),
                };
                tokens = new_tokens;
            }
            _ => break,
        }
    }

    (node, tokens)
}

fn parse_factor(tokens: &[Token]) -> NTPair {
    match tokens.first() {
        Some(Token::Number(_)) => parse_number(tokens),
        Some(Token::LParen) => {
            let (node, tokens) = parse_expression(&tokens[1..]);
            if tokens.first() == Some(&Token::RParen) {
                NTPair::new(node, &tokens[1..])
            } else {
                println!("Mismatched parentheses");
                panic()
            }
        }
        Some(Token::Add) | Some(Token::Subtract) => {
            let op = tokens[0].clone();
            let nt_pair = parse_factor(&tokens[1..]);
            let (expr, tokens) = (nt_pair.node, nt_pair.tokens);
            NTPair::new(
                ASTNode::UnaryOp {
                    op,
                    expr: Box::new(expr),
                },
                tokens,
            )
        }
        _ => panic(),
    }
}

fn parse_number(tokens: &[Token]) -> NTPair {
    if let Some(Token::Number(value)) = tokens.first() {
        NTPair::new(ASTNode::Number(*value), &tokens[1..])
    } else {
        panic()
    }
}

fn evaluate(node: &ASTNode) -> f64 {
    match node {
        ASTNode::Number(value) => *value,
        ASTNode::UnaryOp { op, expr } => {
            let value = evaluate(expr);
            match op {
                Token::Add => value,
                Token::Subtract => -value,
                _ => {
                    panic();
                    0.0
                }
            }
        }
        ASTNode::BinaryOp { left, op, right } => {
            let left_value = evaluate(left);
            let right_value = evaluate(right);
            match op {
                Token::Add => left_value + right_value,
                Token::Subtract => left_value - right_value,
                Token::Multiply => left_value * right_value,
                Token::Divide => left_value / right_value,
                _ => {
                    panic();
                    0.0
                }
            }
        }
    }
}

struct NTPair<'a> {
    node: ASTNode,
    tokens: &'a [Token],
}

impl<'a> NTPair<'a> {
    fn new(node: ASTNode, tokens: &'a [Token]) -> NTPair {
        NTPair { node, tokens }
    }

    fn panic() -> NTPair<'a> {
        NTPair {
            node: ASTNode::Number(0.0),
            tokens: &[Token::Number(0.0)],
        }
    }
}

fn panic<'a>() -> NTPair<'a> {
    *PANIC.lock() = true;
    NTPair::panic()
}

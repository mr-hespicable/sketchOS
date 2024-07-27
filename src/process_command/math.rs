use crate::println;

use super::CommandResult;
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

pub fn math(command: &str) -> CommandResult {
    let mut expr_raw: &str = command.splitn(2, " ").collect::<Vec<_>>()[1];
    let expr_parsed = expr_raw.replace(" ", "");
    let expr = expr_parsed.as_str();

    let tokens = tokenize(expr);
    let (ast, _) = parse(&tokens);
    let result = evaluate(&ast);
    CommandResult {
        data_bytes: result.to_string().as_bytes().to_vec(),
    }
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
        // eg 1/5
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
                println!("unexpected character <{}>", char);
                panic!()
            }
        }
    }

    tokens
}

fn parse(tokens: &[Token]) -> (ASTNode, &[Token]) {
    parse_expression(tokens)
}

fn parse_expression(tokens: &[Token]) -> (ASTNode, &[Token]) {
    let (mut node, tokens) = parse_term(tokens);

    let mut tokens = tokens;

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
    let (mut node, tokens) = parse_factor(tokens);
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

fn parse_factor(tokens: &[Token]) -> (ASTNode, &[Token]) {
    match tokens.first() {
        Some(Token::Number(_)) => parse_number(tokens),
        Some(Token::LParen) => {
            let (node, tokens) = parse_expression(&tokens[1..]);
            if tokens.first() == Some(&Token::RParen) {
                (node, &tokens[1..])
            } else {
                println!("Mismatched parentheses");
                panic!()
            }
        }
        Some(Token::Add) | Some(Token::Subtract) => {
            let op = tokens[0].clone();
            let (expr, tokens) = parse_factor(&tokens[1..]);
            (
                ASTNode::UnaryOp {
                    op,
                    expr: Box::new(expr),
                },
                tokens,
            )
        }
        _ => {
            println!("unexpected token <{:?}>", tokens.first().unwrap());
            panic!()
        }
    }
}

fn parse_number(tokens: &[Token]) -> (ASTNode, &[Token]) {
    if let Some(Token::Number(value)) = tokens.first() {
        (ASTNode::Number(*value), &tokens[1..])
    } else {
        println!("expected a number, got <{:?}>", tokens.first().unwrap());
        panic!();
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
                    println!("unexpected operator <{:?}>", op);
                    panic!();
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
                    println!("unexpected operator <{:?}>", op);
                    panic!();
                }
            }
        }
    }
}

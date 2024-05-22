use std::collections::VecDeque;

use crate::expression::{convert_to_expression, Expression};


#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Variable(String),
    PI,
    Plus,
    Minus,
    Multiply,
    Divide,
    Pow,
    Equals,
    OpenParen,
    CloseParen,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    Arcsin,
    Arccos,
    Arctan,
    Arccsc,
    Arcsec,
    Arccot,
}

fn match_token(input: &mut String, tokens: &mut Vec<Token>, token: Token, token_str: &str) -> bool {
    if input.starts_with(token_str) {
        tokens.push(token);
        input.drain(..token_str.len());
        return true;
    }
    false
}

pub fn tokenize(mut input: String) -> Vec<Token> {
    input = input.to_lowercase();
    let mut tokens = Vec::new();
    let whitespace_chars = [" ", "\t", "\n", "\r"];

    let static_tokens = [
        (Token::Plus, "+"),
        (Token::Minus, "-"),
        (Token::Multiply, "*"),
        (Token::Divide, "/"),
        (Token::OpenParen, "("),
        (Token::CloseParen, ")"),
        (Token::Pow, "^"),
        (Token::Equals, "="),
        (Token::Sin, "sin"),
        (Token::Cos, "cos"),
        (Token::Tan, "tan"),
        (Token::Csc, "csc"),
        (Token::Sec, "sec"),
        (Token::Cot, "cot"),
        (Token::Arcsin, "arcsin"),
        (Token::Arccos, "arccos"),
        (Token::Arctan, "arctan"),
        (Token::Arccsc, "arccsc"),
        (Token::Arcsec, "arcsec"),
        (Token::Arccot, "arccot"),
        (Token::Sqrt, "sqrt"),
        (Token::PI, "pi"),
    ];

    while !input.is_empty() {
        //check for whitespace, remove it and rerun loop if whitespace is found
        if whitespace_chars.contains(&&input[0..1]) {
            input.drain(..1);
            continue;
        }

        //check for all static tokens
        let mut found = false;
        for (token, token_str) in static_tokens.iter() {
            if match_token(&mut input, &mut tokens, token.clone(), token_str) {
                found = true;
                break;
            }
        }
        if found {
            continue;
        }

        //check for numbers
        let mut number = String::new();
        for c in input.chars() {
            if c.is_numeric() || c == '.' {
                number.push(c);
            } else {
                break;
            }
        }
        input.drain(..number.len());
        if !number.is_empty() {
            tokens.push(Token::Number(number.parse().unwrap()));
            continue;
        }

        //make sure that the next character is a letter
        let var = input.chars().next().unwrap();
        if var.is_alphabetic() {
            tokens.push(Token::Variable(var.to_string()));
            input.drain(..1);
            continue;
        }

        //if we get here, we have an invalid token
        panic!("Invalid token: {}", input);
    }

    tokens
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f64),
    PI,
    Variable(String),
    BinaryOp(Box<ASTNode>, Box<ASTNode>, BinaryOp),
    UnaryOp(Box<ASTNode>, UnaryOp),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Pow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    Arcsin,
    Arccos,
    Arctan,
    Arccsc,
    Arcsec,
    Arccot,
    Sqrt,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum ShuntingYardStack {
    Add,
    Subtract,
    Multiply,
    Divide,
    Pow,
    Paren,
    Sin,
    Cos,
    Tan,
    Csc,
    Sec,
    Cot,
    Arcsin,
    Arccos,
    Arctan,
    Arccsc,
    Arcsec,
    Arccot,
    Sqrt,
}

fn get_precedence(op: ShuntingYardStack) -> i32 {
    match op {
        ShuntingYardStack::Add => 1,
        ShuntingYardStack::Subtract => 1,
        ShuntingYardStack::Multiply => 2,
        ShuntingYardStack::Divide => 2,
        ShuntingYardStack::Pow => 3,
        ShuntingYardStack::Paren => 0,
        ShuntingYardStack::Sin => 0,
        ShuntingYardStack::Cos => 0,
        ShuntingYardStack::Tan => 0,
        ShuntingYardStack::Csc => 0,
        ShuntingYardStack::Sec => 0,
        ShuntingYardStack::Cot => 0,
        ShuntingYardStack::Arcsin => 0,
        ShuntingYardStack::Arccos => 0,
        ShuntingYardStack::Arctan => 0,
        ShuntingYardStack::Arccsc => 0,
        ShuntingYardStack::Arcsec => 0,
        ShuntingYardStack::Arccot => 0,
        ShuntingYardStack::Sqrt => 0,
    }
}

fn shunting_yard_to_binary_op(op: ShuntingYardStack) -> BinaryOp {
    match op {
        ShuntingYardStack::Add => BinaryOp::Add,
        ShuntingYardStack::Subtract => BinaryOp::Subtract,
        ShuntingYardStack::Multiply => BinaryOp::Multiply,
        ShuntingYardStack::Divide => BinaryOp::Divide,
        ShuntingYardStack::Pow => BinaryOp::Pow,
        _ => panic!("Invalid ShuntingYardStack to BinaryOp conversion"),
    }
}

fn shunting_yard_to_unary_op(op: ShuntingYardStack) -> Option<UnaryOp> {
    match op {
        ShuntingYardStack::Sin => Some(UnaryOp::Sin),
        ShuntingYardStack::Cos => Some(UnaryOp::Cos),
        ShuntingYardStack::Tan => Some(UnaryOp::Tan),
        ShuntingYardStack::Csc => Some(UnaryOp::Csc),
        ShuntingYardStack::Sec => Some(UnaryOp::Sec),
        ShuntingYardStack::Cot => Some(UnaryOp::Cot),
        ShuntingYardStack::Arcsin => Some(UnaryOp::Arcsin),
        ShuntingYardStack::Arccos => Some(UnaryOp::Arccos),
        ShuntingYardStack::Arctan => Some(UnaryOp::Arctan),
        ShuntingYardStack::Arccsc => Some(UnaryOp::Arccsc),
        ShuntingYardStack::Arcsec => Some(UnaryOp::Arcsec),
        ShuntingYardStack::Arccot => Some(UnaryOp::Arccot),
        ShuntingYardStack::Sqrt => Some(UnaryOp::Sqrt),
        _ => None,
    }
}

//parses a list of tokens into a list of ASTNodes, each representing an equivelent expression(seporated by an =)
pub fn parse(tokens: Vec<Token>) -> Vec<Expression> {
    let mut stack = Vec::new();
    let mut expression_output: VecDeque<_> = VecDeque::new();
    let mut true_output = Vec::new();

    for token in tokens {
        match token {
            Token::Multiply => {
                //compare to the top of the stack
                while !stack.is_empty()
                    && get_precedence(*stack.last().unwrap())
                        >= get_precedence(ShuntingYardStack::Multiply)
                {
                    let rhs = Box::new(expression_output.pop_back().unwrap());
                    let lhs = Box::new(expression_output.pop_back().unwrap());
                    expression_output.push_back(ASTNode::BinaryOp(
                        lhs,
                        rhs,
                        shunting_yard_to_binary_op(stack.pop().unwrap()),
                    ));
                }
                stack.push(ShuntingYardStack::Multiply);
            }
            Token::Plus => {
                //compare to the top of the stack
                while !stack.is_empty()
                    && get_precedence(*stack.last().unwrap())
                        >= get_precedence(ShuntingYardStack::Add)
                {
                    let rhs = Box::new(expression_output.pop_back().unwrap());
                    let lhs = Box::new(expression_output.pop_back().unwrap());
                    expression_output.push_back(ASTNode::BinaryOp(
                        lhs,
                        rhs,
                        shunting_yard_to_binary_op(stack.pop().unwrap()),
                    ));
                }
                stack.push(ShuntingYardStack::Add);
            }
            Token::Minus => {
                //compare to the top of the stack
                while !stack.is_empty()
                    && get_precedence(*stack.last().unwrap())
                        >= get_precedence(ShuntingYardStack::Subtract)
                {
                    let rhs = Box::new(expression_output.pop_back().unwrap());
                    let lhs = Box::new(expression_output.pop_back().unwrap());
                    expression_output.push_back(ASTNode::BinaryOp(
                        lhs,
                        rhs,
                        shunting_yard_to_binary_op(stack.pop().unwrap()),
                    ));
                }
                stack.push(ShuntingYardStack::Subtract);
            }
            Token::Divide => {
                //compare to the top of the stack
                while !stack.is_empty()
                    && get_precedence(*stack.last().unwrap())
                        >= get_precedence(ShuntingYardStack::Divide)
                {
                    let rhs = Box::new(expression_output.pop_back().unwrap());
                    let lhs = Box::new(expression_output.pop_back().unwrap());
                    expression_output.push_back(ASTNode::BinaryOp(
                        lhs,
                        rhs,
                        shunting_yard_to_binary_op(stack.pop().unwrap()),
                    ));
                }
                stack.push(ShuntingYardStack::Divide);
            }
            Token::Pow => {
                //compare to the top of the stack
                while !stack.is_empty()
                    && get_precedence(*stack.last().unwrap())
                        > get_precedence(ShuntingYardStack::Pow)
                {
                    let rhs = Box::new(expression_output.pop_back().unwrap());
                    let lhs = Box::new(expression_output.pop_back().unwrap());
                    expression_output.push_back(ASTNode::BinaryOp(
                        lhs,
                        rhs,
                        shunting_yard_to_binary_op(stack.pop().unwrap()),
                    ));
                }
                stack.push(ShuntingYardStack::Pow);
            }
            Token::Sin => {
                stack.push(ShuntingYardStack::Sin);
            }
            Token::Cos => {
                stack.push(ShuntingYardStack::Cos);
            }
            Token::Tan => {
                stack.push(ShuntingYardStack::Tan);
            }
            Token::Csc => {
                stack.push(ShuntingYardStack::Csc);
            }
            Token::Sec => {
                stack.push(ShuntingYardStack::Sec);
            }
            Token::Cot => {
                stack.push(ShuntingYardStack::Cot);
            }
            Token::Arcsin => {
                stack.push(ShuntingYardStack::Arcsin);
            }
            Token::Arccos => {
                stack.push(ShuntingYardStack::Arccos);
            }
            Token::Arctan => {
                stack.push(ShuntingYardStack::Arctan);
            }
            Token::Arccsc => {
                stack.push(ShuntingYardStack::Arccsc);
            }
            Token::Arcsec => {
                stack.push(ShuntingYardStack::Arcsec);
            }
            Token::Arccot => {
                stack.push(ShuntingYardStack::Arccot);
            }
            Token::Sqrt => {
                stack.push(ShuntingYardStack::Sqrt);
            }
            Token::OpenParen => {
                stack.push(ShuntingYardStack::Paren);
            }
            Token::CloseParen => {
                while *stack.last().unwrap() != ShuntingYardStack::Paren {
                    let rhs = Box::new(expression_output.pop_back().unwrap());
                    let lhs = Box::new(expression_output.pop_back().unwrap());
                    expression_output.push_back(ASTNode::BinaryOp(
                        lhs,
                        rhs,
                        shunting_yard_to_binary_op(stack.pop().unwrap()),
                    ));
                }
                stack.pop();
                //handle functions
                if let Some(op) = stack.last() {
                    if let Some(unary_op) = shunting_yard_to_unary_op(*op) {
                        stack.pop();
                        let arg = Box::new(expression_output.pop_back().unwrap());
                        expression_output.push_back(ASTNode::UnaryOp(arg, unary_op));
                    }
                }
            }
            Token::Number(n) => {
                expression_output.push_back(ASTNode::Number(n));
            }
            Token::Variable(v) => {
                expression_output.push_back(ASTNode::Variable(v));
            }
            Token::PI => {
                expression_output.push_back(ASTNode::PI);
            }
            Token::Equals => {
                //pop all remaining operators off the stack
                while !stack.is_empty() {
                    let rhs = Box::new(expression_output.pop_back().unwrap());
                    let lhs = Box::new(expression_output.pop_back().unwrap());
                    expression_output.push_back(ASTNode::BinaryOp(
                        lhs,
                        rhs,
                        shunting_yard_to_binary_op(stack.pop().unwrap()),
                    ));
                }
                true_output.push(expression_output.pop_back().unwrap());
            }
        }
    }
    while !stack.is_empty() {
        let rhs = Box::new(expression_output.pop_back().unwrap());
        let lhs = Box::new(expression_output.pop_back().unwrap());
        expression_output.push_back(ASTNode::BinaryOp(
            lhs,
            rhs,
            shunting_yard_to_binary_op(stack.pop().unwrap()),
        ));
    }
    true_output.push(expression_output.pop_back().unwrap());
    true_output.into_iter().map(|ast: ASTNode| convert_to_expression(&ast)).collect()
}
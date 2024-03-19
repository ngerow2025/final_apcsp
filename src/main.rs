use std::{error, io::Write, iter::Peekable};

fn main() {
    println!("Welcome to the mathmatical solver, please enter an expression: ");
    loop {
        main_loop();
    }
}

fn main_loop() {
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_owned();
    if input == "exit" {
        std::process::exit(0);
    }
    let result = solve(input);
    println!("Result: {}", result);
}

fn solve(input: String) -> f64 {
    let tokens = tokenize(input);
    println!("Solving: {:?}", tokens);
    0.0
}

#[derive(Debug, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParen,
    CloseParen,
    Variable(String),
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
    Pow,
    Sqrt,
    Pi,
    Equals,
}

fn match_token(input: &mut String, tokens: &mut Vec<Token>, token: Token, token_str: &str) -> bool {
    if input.starts_with(token_str) {
        tokens.push(token);
        input.drain(..token_str.len());
        return true;
    }
    false
}

fn tokenize(mut input: String) -> Vec<Token> {
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
        (Token::Pi, "pi"),
    ];

    while input.len() > 0 {
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
        if number.len() > 0 {
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

use std::io::Write;

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
    let input = input.trim();
    if input == "exit" {
        std::process::exit(0);
    }
    let result = solve(input);
    println!("Result: {}", result);
}

fn solve(input: &str) -> f64 {
    let tokens = tokenize(input);
    println!("Solving: {}", input);
    0.0
}

enum Token {
    Number(f64),
    Plus, Minus, Multiply, Divide,
    OpenParen, CloseParen,
    Variable(String),
    Sin, Cos, Tan,
    Csc, Sec, Cot,
    Arcsin, Arccos, Arctan,
    Arccsc, Arcsec, Arccot,
    Pow,
    Sqrt,
    Pi,
}



fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();
    while iter.peek() != None {
        let c = iter.peek().unwrap();
        if c.is_ascii_digit() {
            let mut number = String::new();
            while iter.peek().unwrap().is_ascii_digit() || iter.peek().unwrap() == &'.'{
                number.push(iter.next().unwrap());
            }
            tokens.push(Token::Number(number.parse().unwrap()));
            continue;
        }




    }
    tokens
}
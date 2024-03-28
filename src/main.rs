//Clap is a command line argument parser made freely available by the MIT license at https://github.com/clap-rs/clap
use std::io::Write;

use rustic_math::{tokenize, parse, simplify, print_expression};

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
    let ast = parse(tokens);
    let simplified = simplify(&ast[0]);
    println!("Solving:");
    print_expression(&simplified, 0);
    0.0
}

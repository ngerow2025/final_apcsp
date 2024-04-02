//Clap is a command line argument parser made freely available by the MIT license at https://github.com/clap-rs/clap
//Rustyline is a readline library made freely available by the MIT license at https://github.com/kkawakam/rustyline

use rustic_math::{eval, parse, print_expression, simplify, tokenize};
use rustyline::DefaultEditor;

fn main() {
    println!("Welcome to the mathmatical solver, please enter an expression: ");
    let mut rl = DefaultEditor::new().unwrap();
    let _ = rl.load_history("history.txt");
    loop {
        main_loop(&mut rl);
    }
}

fn main_loop(rl: &mut DefaultEditor) {
    let input = rl.readline(">> ");
    match input {
        Ok(input) => {
            rl.add_history_entry(input.clone()).unwrap();
            if input == "exit" {
                rl.save_history("history.txt").unwrap();
                std::process::exit(0);
            }
            let result = solve(input);
            println!("Result: {}", result);
        }
        Err(_) => {
            rl.save_history("history.txt").unwrap();
            std::process::exit(0);
        }
    }
}

fn solve(input: String) -> f64 {
    let tokens = tokenize(input);
    let ast = parse(tokens);
    let simplified = simplify(&ast[0]);
    let result = eval(&simplified);
    // print_expression(&simplified, 0);
    result
}

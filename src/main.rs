//Clap is a command line argument parser made freely available by the MIT license at https://github.com/clap-rs/clap
//Rustyline is a readline library made freely available by the MIT license at https://github.com/kkawakam/rustyline
use rustic_math::{eval, parse, tokenize};
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
            compute(input);
        }
        Err(_) => {
            rl.save_history("history.txt").unwrap();
            std::process::exit(0);
        }
    }
}

fn compute(input: String) {
    let tokens = tokenize(input);
    let expressions = parse(tokens);
    if expressions.len() == 1 {
        let result = eval(&expressions[0]);
        println!("Result: {}", result);
    } else {
        let evaled: Vec<f64> = expressions.iter().map(eval).collect();
        let val = evaled[0];
        for (i, item) in evaled.into_iter().enumerate() {
            if val != item {
                println!("the first expression evaluated to {}, but the {}th expression evaluated to {}", val, i, item);
                println!("False");
                return;
            }
        }
        println!("True({})", val);
    }
}

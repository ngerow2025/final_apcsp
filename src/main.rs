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
    println!("Solving: {}", input);
    0.0
}
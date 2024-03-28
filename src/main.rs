//Clap is a command line argument parser made freely available by the MIT license at https://github.com/clap-rs/clap
use std::{collections::VecDeque, io::Write};

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

#[derive(Debug, Clone)]
enum Token {
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
enum ASTNode {
    Number(f64),
    PI,
    Variable(String),
    BinaryOp(Box<ASTNode>, Box<ASTNode>, BinaryOp),
    UnaryOp(Box<ASTNode>, UnaryOp),
}

#[derive(Debug, Clone, PartialEq)]
enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Pow,
}

#[derive(Debug, Clone, PartialEq)]
enum UnaryOp {
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

#[allow(dead_code)]
fn print_ast(ast: &ASTNode, indent: i32) {
    print!("{}", "| ".repeat(indent as usize));
    match ast {
        ASTNode::Number(n) => {
            println!("Number: {}", n);
        }
        ASTNode::PI => {
            println!("PI");
        }
        ASTNode::Variable(v) => {
            println!("Variable: {}", v);
        }
        ASTNode::BinaryOp(lhs, rhs, op) => {
            println!("BinaryOp: {:?}", op);
            print_ast(lhs, indent + 1);
            print_ast(rhs, indent + 1);
        }
        ASTNode::UnaryOp(arg, op) => {
            println!("UnaryOp: {:?}", op);
            print_ast(arg, indent + 1);
        }
    }
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

fn shunting_yard_to_unary_op(op: ShuntingYardStack) -> UnaryOp {
    match op {
        ShuntingYardStack::Sin => UnaryOp::Sin,
        ShuntingYardStack::Cos => UnaryOp::Cos,
        ShuntingYardStack::Tan => UnaryOp::Tan,
        ShuntingYardStack::Csc => UnaryOp::Csc,
        ShuntingYardStack::Sec => UnaryOp::Sec,
        ShuntingYardStack::Cot => UnaryOp::Cot,
        ShuntingYardStack::Arcsin => UnaryOp::Arcsin,
        ShuntingYardStack::Arccos => UnaryOp::Arccos,
        ShuntingYardStack::Arctan => UnaryOp::Arctan,
        ShuntingYardStack::Arccsc => UnaryOp::Arccsc,
        ShuntingYardStack::Arcsec => UnaryOp::Arcsec,
        ShuntingYardStack::Arccot => UnaryOp::Arccot,
        ShuntingYardStack::Sqrt => UnaryOp::Sqrt,
        _ => panic!("Invalid ShuntingYardStack to UnaryOp conversion"),
    }
}

//parses a list of tokens into a list of ASTNodes, each representing an equivelent expression(seporated by an =)
fn parse(tokens: Vec<Token>) -> Vec<ASTNode> {
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
                if let Some(op) = stack.pop() {
                    let arg = Box::new(expression_output.pop_back().unwrap());
                    expression_output
                        .push_back(ASTNode::UnaryOp(arg, shunting_yard_to_unary_op(op)));
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
    true_output
}

#[derive(Debug, Clone)]
struct Multiplication {
    //TODO: don't have box here?
    #[allow(clippy::vec_box)]
    terms: Vec<Box<Expression>>,
}

#[derive(Debug, Clone)]
struct Division {
    numerator: Box<Expression>,
    denominator: Box<Expression>,
}

#[derive(Debug, Clone)]
struct Addition {
    //TODO: don't have box here?
    #[allow(clippy::vec_box)]
    terms: Vec<Box<Expression>>,
}

#[derive(Debug, Clone)]
struct Negation {
    term: Box<Expression>,
}

#[derive(Debug, Clone)]
struct Exponentiation {
    base: Box<Expression>,
    exponent: Box<Expression>,
}

#[derive(Debug, Clone)]
struct Sqrt {
    arg: Box<Expression>,
}

#[derive(Debug, Clone)]
enum Function {
    Sin(Box<Expression>),
    Cos(Box<Expression>),
    Tan(Box<Expression>),
    Csc(Box<Expression>),
    Sec(Box<Expression>),
    Cot(Box<Expression>),
    Arcsin(Box<Expression>),
    Arccos(Box<Expression>),
    Arctan(Box<Expression>),
    Arccsc(Box<Expression>),
    Arcsec(Box<Expression>),
    Arccot(Box<Expression>),
}

#[derive(Debug, Clone)]
enum Expression {
    Number(f64),
    Variable(String),
    Multiplication(Multiplication),
    Division(Division),
    Addition(Addition),
    Negation(Negation),
    Exponentiation(Exponentiation),
    Sqrt(Sqrt),
    Function(Function),
}

fn simplify(ast: &ASTNode) -> Expression {
    let converted = convert_to_expression(ast);
    let simplified = simplify_to_standard_form(converted);
    #[allow(clippy::let_and_return)]
    simplified
}

fn convert_to_expression(ast: &ASTNode) -> Expression {
    match ast {
        ASTNode::Number(n) => Expression::Number(*n),
        ASTNode::PI => Expression::Number(std::f64::consts::PI),
        ASTNode::Variable(v) => Expression::Variable(v.clone()),
        ASTNode::BinaryOp(lhs, rhs, op) => match op {
            BinaryOp::Add => {
                let lhs = convert_to_expression(lhs);
                let rhs = convert_to_expression(rhs);

                Expression::Addition(Addition {
                    terms: vec![Box::new(lhs), Box::new(rhs)],
                })
            }
            BinaryOp::Subtract => {
                let lhs = convert_to_expression(lhs);
                let rhs = convert_to_expression(rhs);
                Expression::Addition(Addition {
                    terms: vec![
                        Box::new(lhs),
                        Box::new(Expression::Negation(Negation {
                            term: Box::new(rhs),
                        })),
                    ],
                })
            }
            BinaryOp::Multiply => {
                let lhs = convert_to_expression(lhs);
                let rhs = convert_to_expression(rhs);
                Expression::Multiplication(Multiplication {
                    terms: vec![Box::new(lhs), Box::new(rhs)],
                })
            }
            BinaryOp::Divide => {
                let lhs = convert_to_expression(lhs);
                let rhs = convert_to_expression(rhs);
                Expression::Division(Division {
                    numerator: Box::new(lhs),
                    denominator: Box::new(rhs),
                })
            }
            BinaryOp::Pow => {
                let base = convert_to_expression(lhs);
                let exponent = convert_to_expression(rhs);
                Expression::Exponentiation(Exponentiation {
                    base: Box::new(base),
                    exponent: Box::new(exponent),
                })
            }
        },
        ASTNode::UnaryOp(arg, op) => {
            let arg = convert_to_expression(arg);
            match op {
                UnaryOp::Sin => Expression::Function(Function::Sin(Box::new(arg))),
                UnaryOp::Cos => Expression::Function(Function::Cos(Box::new(arg))),
                UnaryOp::Tan => Expression::Function(Function::Tan(Box::new(arg))),
                UnaryOp::Csc => Expression::Function(Function::Csc(Box::new(arg))),
                UnaryOp::Sec => Expression::Function(Function::Sec(Box::new(arg))),
                UnaryOp::Cot => Expression::Function(Function::Cot(Box::new(arg))),
                UnaryOp::Arcsin => Expression::Function(Function::Arcsin(Box::new(arg))),
                UnaryOp::Arccos => Expression::Function(Function::Arccos(Box::new(arg))),
                UnaryOp::Arctan => Expression::Function(Function::Arctan(Box::new(arg))),
                UnaryOp::Arccsc => Expression::Function(Function::Arccsc(Box::new(arg))),
                UnaryOp::Arcsec => Expression::Function(Function::Arcsec(Box::new(arg))),
                UnaryOp::Arccot => Expression::Function(Function::Arccot(Box::new(arg))),
                UnaryOp::Sqrt => Expression::Sqrt(Sqrt { arg: Box::new(arg) }),
            }
        }
    }
}

//exposed to api consumers, simplifies an expression to standard form
//loops until no more simplifications can be made
fn simplify_to_standard_form(mut expression: Expression) -> Expression {
    let mut simplified = false;
    let passes = [tree_walk_pass(&coalesce_multiplication)];
    while !simplified {
        simplified = true;
        for pass in &passes {
            let (pass_expression, pass_simplified) = pass(expression);
            expression = pass_expression;
            simplified &= pass_simplified;
        }
    }

    expression
}

fn tree_walk_pass(
    pass: &dyn Fn(Expression) -> (Expression, bool),
) -> impl Fn(Expression) -> (Expression, bool) + '_ {
    move |expression| {
        let (expression, simplified) = pass(expression);
        //recurse into the expression
        match expression {
            Expression::Multiplication(multiplication) => {
                let (terms, terms_simplified) = multiplication
                    .terms
                    .into_iter()
                    .map(|term| tree_walk_pass(pass)(*term))
                    .fold((Vec::new(), true), |mut a, e| {
                        a.0.push(Box::new(e.0));
                        a.1 &= e.1;
                        a
                    });
                (
                    Expression::Multiplication(Multiplication { terms }),
                    terms_simplified && simplified,
                )
            }
            Expression::Division(division) => {
                let (numerator, numerator_simplified) = tree_walk_pass(pass)(*division.numerator);
                let (denominator, denominator_simplified) =
                    tree_walk_pass(pass)(*division.denominator);
                (
                    Expression::Division(Division {
                        numerator: Box::new(numerator),
                        denominator: Box::new(denominator),
                    }),
                    numerator_simplified && denominator_simplified && simplified,
                )
            }
            Expression::Addition(addition) => {
                let (terms, terms_simplified) = addition
                    .terms
                    .into_iter()
                    .map(|term| tree_walk_pass(pass)(*term))
                    .fold((Vec::new(), true), |mut a, e| {
                        a.0.push(Box::new(e.0));
                        a.1 &= e.1;
                        a
                    });
                (
                    Expression::Addition(Addition { terms }),
                    terms_simplified && simplified,
                )
            }
            Expression::Negation(negation) => {
                let (term, term_simplified) = tree_walk_pass(pass)(*negation.term);
                (
                    Expression::Negation(Negation {
                        term: Box::new(term),
                    }),
                    term_simplified && simplified,
                )
            }
            Expression::Exponentiation(exponentiation) => {
                let (base, base_simplified) = tree_walk_pass(pass)(*exponentiation.base);
                let (exponent, exponent_simplified) =
                    tree_walk_pass(pass)(*exponentiation.exponent);
                (
                    Expression::Exponentiation(Exponentiation {
                        base: Box::new(base),
                        exponent: Box::new(exponent),
                    }),
                    base_simplified && exponent_simplified && simplified,
                )
            }
            Expression::Sqrt(sqrt) => {
                let (arg, arg_simplified) = tree_walk_pass(pass)(*sqrt.arg);
                (
                    Expression::Sqrt(Sqrt { arg: Box::new(arg) }),
                    arg_simplified && simplified,
                )
            }
            Expression::Function(function) => match function {
                Function::Sin(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Sin(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Cos(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Cos(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Tan(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Tan(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Csc(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Csc(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Sec(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Sec(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Cot(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Cot(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Arcsin(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arcsin(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Arccos(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arccos(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Arctan(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arctan(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Arccsc(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arccsc(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Arcsec(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arcsec(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
                Function::Arccot(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arccot(Box::new(arg))),
                        arg_simplified && simplified,
                    )
                }
            },
            Expression::Number(_) | Expression::Variable(_) => (expression, simplified),
        }
    }
}

fn coalesce_multiplication(expression: Expression) -> (Expression, bool) {
    match expression {
        Expression::Multiplication(multiplication) => {
            println!("running coalesce_multiplication pass");
            let mut simplified = true;
            let terms = multiplication
                .terms
                .into_iter()
                .flat_map(|term| match *term {
                    Expression::Multiplication(inner_multiplication) => {
                        if simplified {
                            println!("found a nested multiplication, coalescing")
                        }
                        simplified = false;
                        inner_multiplication.terms
                    }
                    _ => vec![term],
                })
                .collect::<Vec<_>>();
            (
                Expression::Multiplication(Multiplication { terms }),
                simplified,
            )
        }
        _ => (expression, true),
    }
}

fn print_expression(expression: &Expression, indent: i32) {
    print!("{}", "| ".repeat(indent as usize));
    match expression {
        Expression::Number(n) => {
            println!("Number: {}", n);
        }
        Expression::Variable(v) => {
            println!("Variable: {}", v);
        }
        Expression::Multiplication(multiplication) => {
            println!("Multiplication");
            for term in &multiplication.terms {
                print_expression(term, indent + 1);
            }
        }
        Expression::Division(division) => {
            println!("Division");
            print_expression(&division.numerator, indent + 1);
            print_expression(&division.denominator, indent + 1);
        }
        Expression::Addition(addition) => {
            println!("Addition");
            for term in &addition.terms {
                print_expression(term, indent + 1);
            }
        }
        Expression::Negation(negation) => {
            println!("Negation");
            print_expression(&negation.term, indent + 1);
        }
        Expression::Exponentiation(exponentiation) => {
            println!("Exponentiation");
            print_expression(&exponentiation.base, indent + 1);
            print_expression(&exponentiation.exponent, indent + 1);
        }
        Expression::Sqrt(sqrt) => {
            println!("Sqrt");
            print_expression(&sqrt.arg, indent + 1);
        }
        Expression::Function(function) => match function {
            Function::Sin(arg) => {
                println!("Sin");
                print_expression(arg, indent + 1);
            }
            Function::Cos(arg) => {
                println!("Cos");
                print_expression(arg, indent + 1);
            }
            Function::Tan(arg) => {
                println!("Tan");
                print_expression(arg, indent + 1);
            }
            Function::Csc(arg) => {
                println!("Csc");
                print_expression(arg, indent + 1);
            }
            Function::Sec(arg) => {
                println!("Sec");
                print_expression(arg, indent + 1);
            }
            Function::Cot(arg) => {
                println!("Cot");
                print_expression(arg, indent + 1);
            }
            Function::Arcsin(arg) => {
                println!("Arcsin");
                print_expression(arg, indent + 1);
            }
            Function::Arccos(arg) => {
                println!("Arccos");
                print_expression(arg, indent + 1);
            }
            Function::Arctan(arg) => {
                println!("Arctan");
                print_expression(arg, indent + 1);
            }
            Function::Arccsc(arg) => {
                println!("Arccsc");
                print_expression(arg, indent + 1);
            }
            Function::Arcsec(arg) => {
                println!("Arcsec");
                print_expression(arg, indent + 1);
            }
            Function::Arccot(arg) => {
                println!("Arccot");
                print_expression(arg, indent + 1);
            }
        },
    }
}

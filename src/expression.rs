use crate::parser::{ASTNode, BinaryOp, UnaryOp};

#[derive(Debug, Clone)]
pub struct Multiplication {
    //TODO: don't have box here?
    #[allow(clippy::vec_box)]
    pub terms: Vec<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct Division {
    pub numerator: Box<Expression>,
    pub denominator: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Addition {
    //TODO: don't have box here?
    #[allow(clippy::vec_box)]
    pub terms: Vec<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct Negation {
    pub term: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Exponentiation {
    pub base: Box<Expression>,
    pub exponent: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct Sqrt {
    pub arg: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Function {
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
pub enum Expression {
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

pub fn convert_to_expression(ast: &ASTNode) -> Expression {
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

// This coppies an Expression and all of the nodes that that is in its tree in order to make two independent Expressions that are identical.
// Most cases should not use this and should try to use no-copy algorithms
pub fn deep_copy(expression: &Expression) -> Expression {
    match expression {
        Expression::Number(n) => Expression::Number(*n),
        Expression::Variable(v) => Expression::Variable(v.clone()),
        Expression::Multiplication(multiplication) => Expression::Multiplication(Multiplication {
            terms: multiplication
                .terms
                .iter()
                .map(|term| Box::new(deep_copy(term)))
                .collect(),
        }),
        Expression::Division(division) => Expression::Division(Division {
            numerator: Box::new(deep_copy(&division.numerator)),
            denominator: Box::new(deep_copy(&division.denominator)),
        }),
        Expression::Addition(addition) => Expression::Addition(Addition {
            terms: addition
                .terms
                .iter()
                .map(|term| Box::new(deep_copy(term)))
                .collect(),
        }),
        Expression::Negation(negation) => Expression::Negation(Negation {
            term: Box::new(deep_copy(&negation.term)),
        }),
        Expression::Exponentiation(exponentiation) => Expression::Exponentiation(Exponentiation {
            base: Box::new(deep_copy(&exponentiation.base)),
            exponent: Box::new(deep_copy(&exponentiation.exponent)),
        }),
        Expression::Sqrt(sqrt) => Expression::Sqrt(Sqrt {
            arg: Box::new(deep_copy(&sqrt.arg)),
        }),
        Expression::Function(function) => match function {
            Function::Sin(arg) => Expression::Function(Function::Sin(Box::new(deep_copy(arg)))),
            Function::Cos(arg) => Expression::Function(Function::Cos(Box::new(deep_copy(arg)))),
            Function::Tan(arg) => Expression::Function(Function::Tan(Box::new(deep_copy(arg)))),
            Function::Csc(arg) => Expression::Function(Function::Csc(Box::new(deep_copy(arg)))),
            Function::Sec(arg) => Expression::Function(Function::Sec(Box::new(deep_copy(arg)))),
            Function::Cot(arg) => Expression::Function(Function::Cot(Box::new(deep_copy(arg)))),
            Function::Arcsin(arg) => {
                Expression::Function(Function::Arcsin(Box::new(deep_copy(arg))))
            }
            Function::Arccos(arg) => {
                Expression::Function(Function::Arccos(Box::new(deep_copy(arg))))
            }
            Function::Arctan(arg) => {
                Expression::Function(Function::Arctan(Box::new(deep_copy(arg))))
            }
            Function::Arccsc(arg) => {
                Expression::Function(Function::Arccsc(Box::new(deep_copy(arg))))
            }
            Function::Arcsec(arg) => {
                Expression::Function(Function::Arcsec(Box::new(deep_copy(arg))))
            }
            Function::Arccot(arg) => {
                Expression::Function(Function::Arccot(Box::new(deep_copy(arg))))
            }
        },
    }
}

pub fn eval(expr: &Expression) -> f64{
    match expr {
        Expression::Number(n) => *n,
        Expression::Variable(_) => panic!("Cannot evaluate variable"),
        Expression::Multiplication(multiplication) => {
            multiplication.terms.iter().map(|term| eval(term)).product()
        }
        Expression::Division(division) => eval(&division.numerator) / eval(&division.denominator),
        Expression::Addition(addition) => addition.terms.iter().map(|term| eval(term)).sum(),
        Expression::Negation(negation) => -eval(&negation.term),
        Expression::Exponentiation(exponentiation) => eval(&exponentiation.base).powf(eval(&exponentiation.exponent)),
        Expression::Sqrt(sqrt) => eval(&sqrt.arg).sqrt(),
        Expression::Function(function) => match function {
            Function::Sin(arg) => eval(arg).sin(),
            Function::Cos(arg) => eval(arg).cos(),
            Function::Tan(arg) => eval(arg).tan(),
            Function::Csc(arg) => 1.0 / eval(arg).sin(),
            Function::Sec(arg) => 1.0 / eval(arg).cos(),
            Function::Cot(arg) => 1.0 / eval(arg).tan(),
            Function::Arcsin(arg) => eval(arg).asin(),
            Function::Arccos(arg) => eval(arg).acos(),
            Function::Arctan(arg) => eval(arg).atan(),
            Function::Arccsc(arg) => 1.0 / eval(arg).asin(),
            Function::Arcsec(arg) => 1.0 / eval(arg).acos(),
            Function::Arccot(arg) => 1.0 / eval(arg).atan(),
        },
    }
}

pub fn print_expression(expression: &Expression, indent: i32) {
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
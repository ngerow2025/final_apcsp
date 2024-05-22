use crate::{expression::{Addition, Division, Exponentiation, Expression, Function, Multiplication, Negation, Sqrt}, passes::{coalescing::{coalesce_addition, coalesce_multiplication}, distribute_multiplication::distribute_multiplication}};

//exposed to api consumers, simplifies an expression to standard form
//loops until no more simplifications can be made
pub fn simplify_expression(mut expression: Expression) -> Expression {
    let mut simplified = false;
    let passes = [
        tree_walk_pass(&coalesce_multiplication),
        tree_walk_pass(&coalesce_addition),
        tree_walk_pass(&distribute_multiplication),
    ];

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
        //recurse into the expression
        let (expr, simplified) = match expression {
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
                    terms_simplified,
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
                    numerator_simplified && denominator_simplified,
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
                (Expression::Addition(Addition { terms }), terms_simplified)
            }
            Expression::Negation(negation) => {
                let (term, term_simplified) = tree_walk_pass(pass)(*negation.term);
                (
                    Expression::Negation(Negation {
                        term: Box::new(term),
                    }),
                    term_simplified,
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
                    base_simplified && exponent_simplified,
                )
            }
            Expression::Sqrt(sqrt) => {
                let (arg, arg_simplified) = tree_walk_pass(pass)(*sqrt.arg);
                (
                    Expression::Sqrt(Sqrt { arg: Box::new(arg) }),
                    arg_simplified,
                )
            }
            Expression::Function(function) => match function {
                Function::Sin(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Sin(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Cos(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Cos(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Tan(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Tan(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Csc(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Csc(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Sec(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Sec(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Cot(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Cot(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Arcsin(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arcsin(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Arccos(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arccos(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Arctan(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arctan(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Arccsc(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arccsc(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Arcsec(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arcsec(Box::new(arg))),
                        arg_simplified,
                    )
                }
                Function::Arccot(arg) => {
                    let (arg, arg_simplified) = tree_walk_pass(pass)(*arg);
                    (
                        Expression::Function(Function::Arccot(Box::new(arg))),
                        arg_simplified,
                    )
                }
            },
            Expression::Number(_) | Expression::Variable(_) => (expression, true),
        };
        let (self_expression, self_simplified) = pass(expr);
        (self_expression, simplified && self_simplified)
    }
}
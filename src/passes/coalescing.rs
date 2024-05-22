use crate::expression::{Addition, Expression, Multiplication};


pub fn coalesce_multiplication(expression: Expression) -> (Expression, bool) {
    match expression {
        Expression::Multiplication(multiplication) => {
            let mut simplified = true;
            let terms = multiplication
                .terms
                .into_iter()
                .flat_map(|term| match *term {
                    Expression::Multiplication(inner_multiplication) => {
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

pub fn coalesce_addition(expression: Expression) -> (Expression, bool) {
    match expression {
        Expression::Addition(addition) => {
            let mut simplified = true;
            let terms = addition
                .terms
                .into_iter()
                .flat_map(|term| match *term {
                    Expression::Addition(inner_addition) => {
                        simplified = false;
                        inner_addition.terms
                    }
                    _ => vec![term],
                })
                .collect::<Vec<_>>();
            (Expression::Addition(Addition { terms }), simplified)
        }
        _ => (expression, true),
    }
}
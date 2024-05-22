use crate::expression::{deep_copy, Addition, Expression, Multiplication};

pub fn distribute_multiplication(expression: Expression) -> (Expression, bool) {
    match expression {
        Expression::Multiplication(mut multiplication) => {
            // 5 * (3 + 5) -> (5 * 3) + (5 * 5)

            // 6 * (2 + 3) * (4 + 5) ->
            // ((6 * 2) + (6 * 3)) * (4 + 5) ->
            // (((6 * 2) + (6 * 3)) * 4) + (((6 * 2) + (6 * 3)) * 5) ->
            // ((6 * 2 * 4) + (6 * 3 * 4)) + ((6 * 2 * 5) + (6 * 3 * 5))
            // (6 * 2 * 4) + (6 * 3 * 4) + (6 * 2 * 5) + (6 * 3 * 5)

            // find first addition in terms and distribute
            // in general a * (b + c) -> (a * b) + (a * c)
            // where a is all the other terms in the multiplication multiplied together

            // find the first addition in the terms
            let addition_index = multiplication
                .terms
                .iter()
                .position(|term| matches!(**term, Expression::Addition(_)));

            if let Some(addition_index) = addition_index {
                if let Expression::Addition(addition) =
                    *multiplication.terms.swap_remove(addition_index)
                {
                    let mut terms = Vec::new();
                    for term in addition.terms {
                        let mut new_terms: Vec<Box<Expression>> = multiplication
                            .terms
                            .iter()
                            .map(|e| Box::new(deep_copy(e)))
                            .collect();
                        new_terms.push(term);
                        terms.push(Box::new(Expression::Multiplication(Multiplication {
                            terms: new_terms,
                        })));
                    }
                    return (Expression::Addition(Addition { terms }), false);
                }
            };
            (Expression::Multiplication(multiplication), true)
        }
        _ => (expression, true),
    }
}
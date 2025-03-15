use crate::expression::Expression;
use crate::expression::Function;
use crate::expression::Multiplication;
use crate::expression::Division;
use crate::expression::Addition;
use crate::expression::Negation;
use crate::expression::Exponentiation;
use crate::expression::Sqrt;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    AddSub,
    Mul,
    Unary,
    Exp,
    Atom,
}

pub fn to_latex(expr: &Expression) -> String {
    let (latex, _) = expr_to_latex(expr);
    latex
}

fn expr_to_latex(expr: &Expression) -> (String, Precedence) {
    match expr {
        Expression::Number(n) => {
            let s = if *n == std::f64::consts::PI {
                String::from("\\pi")
            } else if *n == std::f64::consts::E {
                String::from("e")
            } else {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            };
            (s, Precedence::Atom)
        }
        Expression::Variable(v) => (v.clone(), Precedence::Atom),
        Expression::Multiplication(m) => {
            let terms: Vec<String> = m.terms.iter().map(|term| {
                let (term_latex, term_prec) = expr_to_latex(term);
                if term_prec < Precedence::Mul {
                    format!("({})", term_latex)
                } else {
                    term_latex
                }
            }).collect();
            let latex = terms.join(" \\cdot ");
            (latex, Precedence::Mul)
        }
        Expression::Division(d) => {
            let numerator = expr_to_latex(&d.numerator).0;
            let denominator = expr_to_latex(&d.denominator).0;
            let latex = format!("\\frac{{{}}}{{{}}}", numerator, denominator);
            (latex, Precedence::Atom)
        }
        Expression::Addition(a) => {
            let terms: Vec<String> = a.terms.iter().map(|term| {
                let (term_latex, term_prec) = expr_to_latex(term);
                if term_prec < Precedence::AddSub {
                    format!("({})", term_latex)
                } else {
                    term_latex
                }
            }).collect();
            let latex = terms.join(" + ");
            (latex, Precedence::AddSub)
        }
        Expression::Negation(n) => {
            let (inner_latex, inner_prec) = expr_to_latex(&n.term);
            let latex = if inner_prec < Precedence::Unary {
                format!("-({})", inner_latex)
            } else {
                format!("-{}", inner_latex)
            };
            (latex, Precedence::Unary)
        }
        Expression::Exponentiation(e) => {
            let (base_latex, base_prec) = expr_to_latex(&e.base);
            let (exponent_latex, exponent_prec) = expr_to_latex(&e.exponent);
            let base_str = if base_prec < Precedence::Exp {
                format!("({})", base_latex)
            } else {
                base_latex
            };
            let exponent_str = if exponent_prec < Precedence::Atom {
                format!("({})", exponent_latex)
            } else {
                exponent_latex
            };
            let latex = format!("{}^{{{}}}", base_str, exponent_str);
            (latex, Precedence::Exp)
        }
        Expression::Sqrt(s) => {
            let (arg_latex, _) = expr_to_latex(&s.arg);
            let latex = format!("\\sqrt{{{}}}", arg_latex);
            (latex, Precedence::Atom)
        }
        Expression::Function(f) => {
            let (name, arg) = match f {
                Function::Sin(a) => ("\\sin", a),
                Function::Cos(a) => ("\\cos", a),
                Function::Tan(a) => ("\\tan", a),
                Function::Csc(a) => ("\\csc", a),
                Function::Sec(a) => ("\\sec", a),
                Function::Cot(a) => ("\\cot", a),
                Function::Arcsin(a) => ("\\arcsin", a),
                Function::Arccos(a) => ("\\arccos", a),
                Function::Arctan(a) => ("\\arctan", a),
                Function::Arccsc(a) => ("\\arccsc", a),
                Function::Arcsec(a) => ("\\arcsec", a),
                Function::Arccot(a) => ("\\arccot", a),
            };
            let (arg_latex, _) = expr_to_latex(arg);
            let latex = format!("{}({})", name, arg_latex);
            (latex, Precedence::Atom)
        }
    }
}
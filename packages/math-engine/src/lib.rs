mod token;
mod lexer;
mod parser;

use wasm_bindgen::prelude::*;
use crate::token::{Token, Expression};
use crate::lexer::Lexer;
use crate::parser::Parser;

impl Expression {
    fn evaluate(&self) -> f64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Variable(_) => f64::NAN,
            Expression::Add(a, b) => a.evaluate() + b.evaluate(),
            Expression::Subtract(a, b) => a.evaluate() - b.evaluate(),
            Expression::Multiply(a, b) => a.evaluate() * b.evaluate(),
            Expression::Divide(a, b) => a.evaluate() / b.evaluate(),
            Expression::Power(a, b) => a.evaluate().powf(b.evaluate()),
        }
    }

    pub fn visualize(&self) -> String {
        match self {
            Expression::Number(n) => n.to_string(),
            Expression::Variable(v) => v.clone(),
            Expression::Add(l, r) => format!("({} + {})", l.visualize(), r.visualize()),
            Expression::Subtract(l, r) => format!("({} - {})", l.visualize(), r.visualize()),
            Expression::Multiply(l, r) => format!("({} * {})", l.visualize(), r.visualize()),
            Expression::Divide(l, r) => format!("({} / {})", l.visualize(), r.visualize()),
            Expression::Power(l, r) => format!("({} ^ {})", l.visualize(), r.visualize()),
        }
    }

    // Aplanamos el árbol manejando el signo (suma de negativos)
    fn collect_terms(&self, terms: &mut Vec<(Expression, f64)>, sign: f64) {
        match self {
            Expression::Add(l, r) => {
                l.collect_terms(terms, sign);
                r.collect_terms(terms, sign);
            }
            Expression::Subtract(l, r) => {
                l.collect_terms(terms, sign);
                r.collect_terms(terms, -sign);
            }
            other => terms.push((other.clone(), sign)),
        }
    }

    pub fn simplify(self) -> Expression {
        match self {
            // Manejamos sumas y restas con la misma lógica de "bolsa de términos"
            Expression::Add(_, _) | Expression::Subtract(_, _) => {
                let mut all_terms = Vec::new();
                self.collect_terms(&mut all_terms, 1.0);

                let mut constant_sum = 0.0;
                let mut var_counts: Vec<(String, f64)> = Vec::new();
                let mut complex_terms: Vec<Expression> = Vec::new();

                for (term, sign) in all_terms {
                    match term.simplify() {
                        Expression::Number(n) => constant_sum += n * sign,
                        
                        Expression::Variable(v) => {
                            let count = sign;
                            if let Some(pos) = var_counts.iter().position(|(name, _)| name == &v) {
                                var_counts[pos].1 += count;
                            } else {
                                var_counts.push((v, count));
                            }
                        }
                        
                        Expression::Multiply(l, r) => {
                            if let (Expression::Number(n), Expression::Variable(v)) = (&*l, &*r) {
                                let count = n * sign;
                                if let Some(pos) = var_counts.iter().position(|(name, _)| name == v) {
                                    var_counts[pos].1 += count;
                                } else {
                                    var_counts.push((v.clone(), count));
                                }
                            } else {
                                complex_terms.push(Expression::Multiply(l, r));
                            }
                        }
                        other => complex_terms.push(other),
                    }
                }

                // Reconstrucción
                let mut final_parts: Vec<Expression> = Vec::new();

                for (name, count) in var_counts {
                    if count == 1.0 {
                        final_parts.push(Expression::Variable(name));
                    } else if count == -1.0 {
                        // Opcional: manejar -x visualmente mejor
                        final_parts.push(Expression::Multiply(
                            Box::new(Expression::Number(-1.0)),
                            Box::new(Expression::Variable(name))
                        ));
                    } else if count != 0.0 {
                        final_parts.push(Expression::Multiply(
                            Box::new(Expression::Number(count)),
                            Box::new(Expression::Variable(name))
                        ));
                    }
                }

                for ct in complex_terms {
                    final_parts.push(ct);
                }

                if final_parts.is_empty() {
                    return Expression::Number(constant_sum);
                }

                let mut result = final_parts.remove(0);
                for t in final_parts {
                    result = Expression::Add(Box::new(result), Box::new(t));
                }

                if constant_sum != 0.0 {
                    result = Expression::Add(Box::new(result), Box::new(Expression::Number(constant_sum)));
                }

                result
            }

            Expression::Multiply(l, r) => {
                let l_s = l.simplify();
                let r_s = r.simplify();
                match (l_s, r_s) {
                    (Expression::Number(n1), Expression::Number(n2)) => Expression::Number(n1 * n2),
                    (other, Expression::Number(n)) if n == 1.0 => other,
                    (Expression::Number(n), other) if n == 1.0 => other,
                    (_, Expression::Number(n)) if n == 0.0 => Expression::Number(0.0),
                    (Expression::Number(n), _) if n == 0.0 => Expression::Number(0.0),
                    (ls, rs) => Expression::Multiply(Box::new(ls), Box::new(rs)),
                }
            }

            Expression::Divide(l, r) => {
                let l_s = l.simplify();
                let r_s = r.simplify();
                match (l_s, r_s) {
                    (Expression::Number(n1), Expression::Number(n2)) if n2 != 0.0 => Expression::Number(n1 / n2),
                    (Expression::Number(n), _) if n == 0.0 => Expression::Number(0.0),
                    (ls, rs) => Expression::Divide(Box::new(ls), Box::new(rs)),
                }
            }

            Expression::Power(l, r) => {
                let l_s = l.simplify();
                let r_s = r.simplify();
                match (l_s, r_s) {
                    (Expression::Number(n1), Expression::Number(n2)) => Expression::Number(n1.powf(n2)),
                    (_, Expression::Number(n)) if n == 0.0 => Expression::Number(1.0),
                    (other, Expression::Number(n)) if n == 1.0 => other,
                    (ls, rs) => Expression::Power(Box::new(ls), Box::new(rs)),
                }
            }

            _ => self,
        }
    }
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    if input.trim().is_empty() { return "0".to_string(); }
    let mut parser = Parser::new(Lexer::new(input));
    if input.contains('=') {
        let (l, r) = parser.parse_statement();
        let sl = (*l).simplify();
        let sr = (*r).simplify();
        format!("{} = {}", sl.visualize(), sr.visualize())
    } else {
        let e = parser.parse_expression();
        (*e).simplify().visualize()
    }
}

#[wasm_bindgen]
pub fn get_ast_visual(input: &str) -> String {
    if input.trim().is_empty() { return "".to_string(); }
    let mut parser = Parser::new(Lexer::new(input));
    let (left, right) = parser.parse_statement();
    format!("L: {} | R: {}", left.visualize(), right.visualize())
}

#[wasm_bindgen]
pub fn count_tokens(input: &str) -> usize {
    if input.trim().is_empty() { return 0; }
    let mut lexer = Lexer::new(input);
    let mut count = 0;
    loop {
        let t = lexer.next_token();
        if t == Token::EOF { break; }
        count += 1;
    }
    count
}
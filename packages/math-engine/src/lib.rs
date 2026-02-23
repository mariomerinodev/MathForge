mod token;
mod lexer;
mod parser;

use wasm_bindgen::prelude::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::{Token, Expression, Fraction};

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    if input.trim().is_empty() { return "0".to_string(); }
    let mut parser = Parser::new(Lexer::new(input));

    if input.contains('=') {
        let (l, r) = parser.parse_statement();

        if let Expression::Error(msg) = *l { return msg; }
        if let Expression::Error(msg) = *r { return msg; }

        let unified = Expression::Add(
            Box::new(l.expand()),
            Box::new(Expression::Multiply(
                Box::new(Expression::Number(Fraction::minus_one())),
                Box::new(r.expand())
            ).expand())
        ).simplify();

        // --- NUEVA INTELIGENCIA DE AUTO-DETECCIÓN ---
        let vars = unified.get_variables();
        
        let target_var = if vars.is_empty() {
            "x".to_string() 
        } else {
            vars.iter().next().unwrap().clone()
        };

        let final_r = Expression::solve_linear(unified, Expression::Number(Fraction::zero()), &target_var);

        match final_r {
            Expression::Error(msg) => msg,
            _ => format!("{} = {}", target_var, final_r.visualize())
        }
    } else {
        // --- MODO SIMPLIFICADOR (Sin signo '=') ---
        let parsed = parser.parse_expression();
        if let Expression::Error(msg) = *parsed { return msg; }
        parsed.expand().simplify().visualize()
    }
}

#[wasm_bindgen]
pub fn count_tokens(input: &str) -> usize {
    let mut lexer = Lexer::new(input);
    let mut count = 0;
    while lexer.next_token() != Token::EOF {
        count += 1;
    }
    count
}

#[wasm_bindgen]
pub fn get_ast_visual(input: &str) -> String {
    if input.trim().is_empty() { return "".to_string(); }
    let mut parser = Parser::new(Lexer::new(input));
    let (left, right) = parser.parse_statement();
    format!("L: {} | R: {}", left.visualize(), right.visualize())
}
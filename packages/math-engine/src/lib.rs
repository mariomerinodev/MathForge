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
            Expression::Add(l, r) => format!("({} + {})", l.visualize(), r.visualize()),
            Expression::Subtract(l, r) => format!("({} - {})", l.visualize(), r.visualize()),
            Expression::Multiply(l, r) => format!("({} * {})", l.visualize(), r.visualize()),
            Expression::Divide(l, r) => format!("({} / {})", l.visualize(), r.visualize()),
            Expression::Power(l, r) => format!("({} ^ {})", l.visualize(), r.visualize()),
        }
    }
}

// --- API PARA WASM ---

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    if input.trim().is_empty() { return "0".to_string(); }
    let mut parser = Parser::new(Lexer::new(input));
    parser.parse_expression().evaluate().to_string()
}

#[wasm_bindgen]
pub fn get_ast_visual(input: &str) -> String {
    if input.trim().is_empty() { return "".to_string(); }
    let mut parser = Parser::new(Lexer::new(input));
    parser.parse_expression().visualize()
}

#[wasm_bindgen]
pub fn count_tokens(input: &str) -> usize {
    if input.trim().is_empty() { return 0; }
    let mut lexer = Lexer::new(input);
    let mut count = 0;
    loop {
        if lexer.next_token() == Token::EOF { break; }
        count += 1;
    }
    count
}
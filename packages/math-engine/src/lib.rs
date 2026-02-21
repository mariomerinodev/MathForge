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
            Expression::Variable(_) => f64::NAN, // Todavía no resolvemos 'x'
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
}

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    if input.trim().is_empty() { return "0".to_string(); }
    
    let mut parser = Parser::new(Lexer::new(input));
    
    // 1. Intentamos parsear la expresión izquierda
    let left = parser.parse_expression();
    
    // 2. Miramos si el siguiente token es un "="
    // Nota: Necesitas asegurarte de que current_token sea accesible o usar un método
    if input.contains('=') {
        let mut full_parser = Parser::new(Lexer::new(input));
        let (l, r) = full_parser.parse_statement();
        return format!("{} = {}", l.visualize(), r.visualize());
    } else {
        // 3. Si no hay "=", simplemente evaluamos el número
        let result = left.evaluate();
        return result.to_string();
    }
}

#[wasm_bindgen]
pub fn get_ast_visual(input: &str) -> String {
    if input.trim().is_empty() { return "".to_string(); }
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    
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
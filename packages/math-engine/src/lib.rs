mod token;
mod lexer;
mod parser;

use wasm_bindgen::prelude::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::Token;

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    if input.trim().is_empty() { return "0".to_string(); }
    let mut parser = Parser::new(Lexer::new(input));
    let (l, r) = parser.parse_statement();
    
    // Simplificamos ambos lados
    let sl = l.simplify();
    let sr = r.simplify();

    // Si el lado derecho es 0 (porque no había '='), solo mostramos el izquierdo
    if let crate::token::Expression::Number(0.0) = sr {
        if !input.contains('=') {
            return sl.visualize();
        }
    }
    
    format!("{} = {}", sl.visualize(), sr.visualize())
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
    
    // Ahora usamos .visualize() que vive en token.rs
    format!("L: {} | R: {}", left.visualize(), right.visualize())
}
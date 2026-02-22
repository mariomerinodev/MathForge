mod token;
mod lexer;
mod parser;

use wasm_bindgen::prelude::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::{Token, Expression}; // <--- IMPORTANTE: Añadir Expression aquí

#[wasm_bindgen]
pub fn solve(input: &str) -> String {
    if input.trim().is_empty() { return "0".to_string(); }
    let mut parser = Parser::new(Lexer::new(input));
    
    let (l, r) = parser.parse_statement();
    
    // Simplificamos primero
    let sl = l.simplify();
    let sr = r.simplify();

    // Si detectamos que hay una ecuación (el usuario puso '='), resolvemos
    if input.contains('=') {
        let (final_l, final_r) = Expression::solve_linear(sl, sr, "x");
        format!("{} = {}", final_l.visualize(), final_r.simplify().visualize())
    } else {
        // Si no hay '=', solo simplificamos el lado izquierdo
        sl.visualize()
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
    
    // Ahora usamos .visualize() que vive en token.rs
    format!("L: {} | R: {}", left.visualize(), right.visualize())
}
use crate::token::{Token, Fraction};

pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self { input: input.chars().collect(), pos: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        let current_char = self.input[self.pos];

        // 1. Números puros a Fracciones
        if current_char.is_digit(10) || current_char == '.' {
            let mut num_str = String::new();
            let mut is_decimal = false;
            let mut decimal_places = 0;
            
            while self.pos < self.input.len() && (self.input[self.pos].is_digit(10) || self.input[self.pos] == '.') {
                if self.input[self.pos] == '.' { 
                    is_decimal = true; 
                } else if is_decimal { 
                    decimal_places += 1; 
                }
                num_str.push(self.input[self.pos]);
                self.pos += 1;
            }

            if is_decimal {
                let parsed: f64 = num_str.parse().unwrap_or(0.0);
                let multiplier = 10_i64.pow(decimal_places);
                let num = (parsed * multiplier as f64).round() as i64;
                return Token::Number(Fraction::new(num, multiplier));
            } else {
                let parsed: i64 = num_str.parse().unwrap_or(0);
                return Token::Number(Fraction::new(parsed, 1));
            }
        }

        // 2. Variables
        if current_char.is_alphabetic() {
            let mut var_str = String::new();
            while self.pos < self.input.len() && self.input[self.pos].is_alphanumeric() {
                var_str.push(self.input[self.pos]);
                self.pos += 1;
            }
            return Token::Variable(var_str);
        }

        // 3. Símbolos
        let token = match current_char {
            '+' => Token::Plus,
            '-' => Token::Minus, 
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '^' => Token::Power,
            '=' => Token::Equal,
            _ => Token::EOF,
        };

        self.pos += 1;
        token
    }
}
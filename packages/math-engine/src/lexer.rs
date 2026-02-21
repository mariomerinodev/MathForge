use crate::token::Token; // Necesitamos saber qué es un Token

pub struct Lexer {
    pub input: Vec<char>,
    pub pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
        if self.pos >= self.input.len() { return Token::EOF; }
        let current_char = self.input[self.pos];

        if current_char.is_digit(10) || current_char == '.' {
            let mut num_str = String::new();
            while self.pos < self.input.len() && (self.input[self.pos].is_digit(10) || self.input[self.pos] == '.') {
                num_str.push(self.input[self.pos]);
                self.pos += 1;
            }
            return Token::Number(num_str.parse().unwrap_or(0.0));
        }

        let token = match current_char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '^' => Token::Power,
            _ => Token::EOF,
        };
        self.pos += 1;
        token
    }
}
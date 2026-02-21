use crate::token::{Token, Expression};
use crate::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let first_token = lexer.next_token();
        Self { lexer, current_token: first_token }
    }

    fn consume(&mut self, expected: Token) {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&expected) {
            self.current_token = self.lexer.next_token();
        }
    }

    fn parse_factor(&mut self) -> Box<Expression> {
        match self.current_token.clone() {
            Token::Number(n) => {
                self.consume(Token::Number(0.0));
                Box::new(Expression::Number(n))
            }
            Token::OpenParen => {
                self.consume(Token::OpenParen);
                let expr = self.parse_expression();
                self.consume(Token::CloseParen);
                expr
            }
            _ => Box::new(Expression::Number(f64::NAN)),
        }
    }

    fn parse_power(&mut self) -> Box<Expression> {
        let mut left = self.parse_factor();
        while matches!(self.current_token, Token::Power) {
            self.consume(Token::Power);
            left = Box::new(Expression::Power(left, self.parse_factor()));
        }
        left
    }

    fn parse_term(&mut self) -> Box<Expression> {
        let mut left = self.parse_power();
        while matches!(self.current_token, Token::Multiply | Token::Divide) {
            let op = self.current_token.clone();
            self.consume(op.clone());
            let right = self.parse_power();
            left = match op {
                Token::Multiply => Box::new(Expression::Multiply(left, right)),
                Token::Divide => Box::new(Expression::Divide(left, right)),
                _ => left,
            };
        }
        left
    }

    pub fn parse_expression(&mut self) -> Box<Expression> {
        let mut left = self.parse_term();
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = self.current_token.clone();
            self.consume(op.clone());
            let right = self.parse_term();
            left = match op {
                Token::Plus => Box::new(Expression::Add(left, right)),
                Token::Minus => Box::new(Expression::Subtract(left, right)),
                _ => left,
            };
        }
        left
    }
}
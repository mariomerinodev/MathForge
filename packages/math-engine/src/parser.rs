use crate::token::{Token, Expression, Fraction};
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

    fn consume(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_statement(&mut self) -> (Box<Expression>, Box<Expression>) {
        let left = self.parse_expression();
        if matches!(self.current_token, Token::Equal) {
            self.consume();
            let right = self.parse_expression();
            (left, right)
        } else {
            (left, Box::new(Expression::Number(Fraction::zero())))
        }
    }

    pub fn parse_expression(&mut self) -> Box<Expression> {
        let mut left = self.parse_term();
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = self.current_token.clone();
            self.consume();
            let right = self.parse_term();
            
            left = match op {
                Token::Plus => Box::new(Expression::Add(left, right)),
                Token::Minus => Box::new(Expression::Add(
                    left,
                    Box::new(Expression::Multiply(Box::new(Expression::Number(Fraction::minus_one())), right))
                )),
                _ => left,
            };
        }
        left
    }

    fn parse_term(&mut self) -> Box<Expression> {
        let mut left = self.parse_power();
        while matches!(self.current_token, Token::Multiply | Token::Divide) {
            let op = self.current_token.clone();
            self.consume();
            let right = self.parse_power();
            left = match op {
                Token::Multiply => Box::new(Expression::Multiply(left, right)),
                Token::Divide => Box::new(Expression::Divide(left, right)),
                _ => left,
            };
        }
        left
    }

    fn parse_power(&mut self) -> Box<Expression> {
        let mut left = self.parse_factor();
        while matches!(self.current_token, Token::Power) {
            self.consume();
            left = Box::new(Expression::Power(left, self.parse_factor()));
        }
        left
    }

    fn parse_factor(&mut self) -> Box<Expression> {
        let token = self.current_token.clone();
        let mut expr = match token {
            Token::Minus => {
                self.consume();
                let expr = self.parse_factor();
                Box::new(Expression::Multiply(Box::new(Expression::Number(Fraction::minus_one())), expr))
            }
            Token::Number(n) => {
                self.consume();
                Box::new(Expression::Number(n))
            }
            Token::Variable(v) => {
                self.consume();
                Box::new(Expression::Variable(v))
            }
            Token::OpenParen => {
                self.consume();
                let e = self.parse_expression();
                self.consume(); 
                e
            }
            _ => Box::new(Expression::Error("Sintaxis inválida".to_string())),
        };

        if matches!(self.current_token, Token::Variable(_) | Token::OpenParen) {
            expr = Box::new(Expression::Multiply(expr, self.parse_factor()));
        }

        expr
    }
}
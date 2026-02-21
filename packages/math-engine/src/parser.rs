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

    // Simplificamos consume para que solo avance sin importar el valor exacto
    fn consume(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    // Nueva función para procesar la ecuación entera
    pub fn parse_statement(&mut self) -> (Box<Expression>, Box<Expression>) {
        let left = self.parse_expression();
        
        if matches!(self.current_token, Token::Equal) {
            self.consume();
            let right = self.parse_expression();
            (left, right)
        } else {
            (left, Box::new(Expression::Number(0.0)))
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
                Token::Minus => Box::new(Expression::Subtract(left, right)),
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
        let mut expr = match self.current_token.clone() {
            Token::Number(n) => {
                self.consume();
                Box::new(Expression::Number(n))
            }
            Token::Variable(v) => { // ¡ESTO FALTABA!
                self.consume();
                Box::new(Expression::Variable(v))
            }
            Token::OpenParen => {
                self.consume();
                let e = self.parse_expression();
                self.consume(); 
                e
            }
            _ => Box::new(Expression::Number(f64::NAN)),
        };

        // Multiplicación implícita (2x -> 2 * x)
        if matches!(self.current_token, Token::Variable(_) | Token::OpenParen) {
            expr = Box::new(Expression::Multiply(expr, self.parse_factor()));
        }

        expr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    // Test de Aritmética básica
    #[test]
    fn test_arithmetic_precedence() {
        let mut parser = Parser::new(Lexer::new("3 + 2 * 4"));
        let expr = parser.parse_expression();
        assert_eq!(expr.evaluate(), 11.0);
    }

    // Test de Multiplicación Implícita
    #[test]
    fn test_implicit_multiplication() {
        let mut parser = Parser::new(Lexer::new("2x"));
        let expr = parser.parse_expression();
        assert_eq!(expr.visualize(), "(2 * x)");
    }

    // Test de Ecuación Completa
    #[test]
    fn test_equation_parsing() {
        let mut parser = Parser::new(Lexer::new("2x + 5 = 15"));
        let (left, right) = parser.parse_statement();
        assert_eq!(left.visualize(), "((2 * x) + 5)");
        assert_eq!(right.visualize(), "15");
    }

    // Test de Errores: Operadores huérfanos
    #[test]
    fn test_error_handling_incomplete() {
        // Aquí probamos que no haya "panic". 
        // El parser debería devolver lo que pueda o un error.
        let mut parser = Parser::new(Lexer::new("2 +"));
        let expr = parser.parse_expression();
        // Dependiendo de tu implementación, esto suele devolver el último número
        // o un nodo de error. Lo importante es que el test pase sin romperse.
        assert!(expr.visualize().contains('2'));
    }
}
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    OpenParen,
    CloseParen,
    EOF,
}

#[derive(Debug, Clone)]
enum Expression {
    Number(f64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
}

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

    fn visualize(&self) -> String {
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

struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return Token::EOF;
        }

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

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop{
            let token = self.next_token();
            if token == Token::EOF {
                tokens.push(Token::EOF);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let first_token = lexer.next_token();
        Self {
            lexer,
            current_token: first_token,
        }
    }

    fn consume(&mut self, expected: Token) {
        if std::mem::discriminant(&self.current_token) == std::mem::discriminant(&expected) {
            self.current_token = self.lexer.next_token();
        }
    }

    // Nivel 4 (Máximo): Números y Paréntesis
    fn parse_factor(&mut self) -> Box<Expression> {
        match self.current_token.clone() {
            Token::Number(n) => {
                self.consume(Token::Number(0.0));
                Box::new(Expression::Number(n))
            }
            Token::OpenParen => {
                self.consume(Token::OpenParen);
                let expr = self.parse_expression(); // Recursividad al inicio
                self.consume(Token::CloseParen);
                expr
            }
            _ => Box::new(Expression::Number(f64::NAN)),
        }
    }

    // Nivel 3: Potencias (Nuevo)
    fn parse_power(&mut self) -> Box<Expression> {
        let mut left = self.parse_factor();

        while matches!(self.current_token, Token::Power) {
            self.consume(Token::Power);
            let right = self.parse_factor();
            left = Box::new(Expression::Power(left, right));
        }
        left
    }

    // Nivel 2: Multiplicación y División
    fn parse_term(&mut self) -> Box<Expression> {
        let mut left = self.parse_power(); // Llama al nivel de potencia

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

    // Nivel 1 (Base): Suma y Resta
    pub fn parse_expression(&mut self) -> Box<Expression> {
        let mut left = self.parse_term(); // Llama al nivel de términos

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

#[wasm_bindgen]
pub fn test_ast() -> f64 {
    let ten = Box::new(Expression::Number(10.0));
    let five = Box::new(Expression::Number(5.0));
    let sum = Box::new(Expression::Add(ten, five));
    
    let two = Box::new(Expression::Number(2.0));
    let final_op = Expression::Multiply(sum, two);

    final_op.evaluate()
}

#[wasm_bindgen]
pub fn count_tokens(input: &str) -> usize {
    // Si el usuario borró todo, devolvemos 0
    if input.trim().is_empty() {
        return 0;
    }

    let mut lexer = Lexer::new(input);
    let mut count = 0;
    loop {
        let token = lexer.next_token();
        if token == Token::EOF {
            break;
        }
        count += 1;
    }
    count
}

#[wasm_bindgen]
pub fn get_tokens_debug(input: &str) -> String {
    let mut lexer = Lexer::new(input);
    let mut debug_str = String::new();
    loop {
        let token = lexer.next_token();
        if token == Token::EOF { break; }
        // Añadimos una representación visual de cada token
        debug_str.push_str(&format!("{:?} ", token));
    }
    debug_str.trim().to_string()
}

#[wasm_bindgen]
pub fn get_ast_visual(input: &str) -> String {
    if input.trim().is_empty() {
        return "".to_string();
    }
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_expression();

    ast.visualize()
}

#[wasm_bindgen]
pub fn solve(input: &str) -> f64 {
    if input.trim().is_empty() {
        return 0.0;
    }

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_expression();

    ast.evaluate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        assert_eq!(solve("10 + 5"), 15.0);
    }

    #[test]
    fn test_precedence() {
        // 10 + (5 * 2) = 20
        assert_eq!(solve("10 + 5 * 2"), 20.0);
    }

    #[test]
    fn test_parentheses() {
        // (10 + 5) * 2 = 30
        assert_eq!(solve("(10 + 5) * 2"), 30.0);
    }

    #[test]
    fn test_division_and_subtraction() {
        assert_eq!(solve("20 / 2 - 3"), 7.0);
    }

    #[test]
    fn test_spaces_handling() {
        assert_eq!(solve("   10    +   2 "), 12.0);
    }

    #[test]
    fn test_invalid_expression() {
        let result = solve("5 + * 2");
        assert!(result.is_nan());
    }

    #[test]
    fn test_powe_precedence() {
        assert_eq!(solve("2 ^ 3 + 1"), 9.0);
        assert_eq!(solve("2 * 3 ^ 2"), 18.0);
    }
}
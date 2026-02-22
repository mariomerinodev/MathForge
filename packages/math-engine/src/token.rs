#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Variable(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Equal,
    OpenParen,
    CloseParen,
    EOF,
}

#[derive(Debug, Clone, PartialEq)] // Añadido PartialEq para facilitar comparaciones
pub enum Expression {
    Number(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn evaluate(&self) -> f64 {
        match self {
            Expression::Number(n) => *n,
            Expression::Variable(_) => f64::NAN,
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

    pub fn contains_variable(&self, var_name: &str) -> bool {
        match self {
            Expression::Variable(v) => v == var_name,
            Expression::Add(l, r) | Expression::Subtract(l, r) | 
            Expression::Multiply(l, r) | Expression::Divide(l, r) | 
            Expression::Power(l, r) => {
                l.contains_variable(var_name) || r.contains_variable(var_name)
            }
            Expression::Number(_) => false,
        }
    }

    pub fn collect_terms(&self, terms: &mut Vec<(Expression, f64)>, sign: f64) {
        match self {
            Expression::Add(l, r) => {
                l.collect_terms(terms, sign);
                r.collect_terms(terms, sign);
            }
            Expression::Subtract(l, r) => {
                l.collect_terms(terms, sign);
                r.collect_terms(terms, -sign);
            }
            other => terms.push((other.clone(), sign)),
        }
    }

    pub fn simplify(self) -> Expression {
        match self {
            Expression::Add(_, _) | Expression::Subtract(_, _) => {
                let mut all_terms = Vec::new();
                self.collect_terms(&mut all_terms, 1.0);

                let mut constant_sum = 0.0;
                let mut var_counts: Vec<(String, f64)> = Vec::new();
                let mut complex_terms: Vec<Expression> = Vec::new();

                for (term, sign) in all_terms {
                    match term.simplify() {
                        Expression::Number(n) => constant_sum += n * sign,
                        Expression::Variable(v) => {
                            if let Some(pos) = var_counts.iter().position(|(name, _)| name == &v) {
                                var_counts[pos].1 += sign;
                            } else {
                                var_counts.push((v, sign));
                            }
                        }
                        Expression::Multiply(l, r) => {
                            if let (Expression::Number(n), Expression::Variable(v)) = (&*l, &*r) {
                                let total_n = n * sign;
                                if let Some(pos) = var_counts.iter().position(|(name, _)| name == v) {
                                    var_counts[pos].1 += total_n;
                                } else {
                                    var_counts.push((v.clone(), total_n));
                                }
                            } else {
                                complex_terms.push(Expression::Multiply(l, r));
                            }
                        }
                        other => complex_terms.push(other),
                    }
                }

                let mut final_parts: Vec<Expression> = Vec::new();
                for (name, count) in var_counts {
                    if count == 0.0 { continue; }
                    if count == 1.0 {
                        final_parts.push(Expression::Variable(name));
                    } else {
                        final_parts.push(Expression::Multiply(
                            Box::new(Expression::Number(count)),
                            Box::new(Expression::Variable(name))
                        ));
                    }
                }
                final_parts.extend(complex_terms);

                if final_parts.is_empty() { return Expression::Number(constant_sum); }

                let mut result = final_parts.remove(0);
                for t in final_parts {
                    result = Expression::Add(Box::new(result), Box::new(t));
                }
                if constant_sum != 0.0 {
                    result = Expression::Add(Box::new(result), Box::new(Expression::Number(constant_sum)));
                }
                result
            }
            Expression::Multiply(l, r) => {
                match (l.simplify(), r.simplify()) {
                    (Expression::Number(n1), Expression::Number(n2)) => Expression::Number(n1 * n2),
                    (other, Expression::Number(n)) if n == 1.0 => other,
                    (Expression::Number(n), other) if n == 1.0 => other,
                    (_, Expression::Number(n)) if n == 0.0 => Expression::Number(0.0),
                    (Expression::Number(n), _) if n == 0.0 => Expression::Number(0.0),
                    (ls, rs) => Expression::Multiply(Box::new(ls), Box::new(rs)),
                }
            }
            Expression::Divide(l, r) => {
                match (l.simplify(), r.simplify()) {
                    (Expression::Number(n1), Expression::Number(n2)) if n2 != 0.0 => Expression::Number(n1 / n2),
                    (Expression::Number(n), _) if n == 0.0 => Expression::Number(0.0),
                    (ls, rs) => Expression::Divide(Box::new(ls), Box::new(rs)),
                }
            }
            Expression::Power(l, r) => {
                match (l.simplify(), r.simplify()) {
                    (Expression::Number(n1), Expression::Number(n2)) => Expression::Number(n1.powf(n2)),
                    (_, Expression::Number(n)) if n == 0.0 => Expression::Number(1.0),
                    (other, Expression::Number(n)) if n == 1.0 => other,
                    (ls, rs) => Expression::Power(Box::new(ls), Box::new(rs)),
                }
            }
            _ => self,
        }
    }

    pub fn solve_linear(left: Expression, right: Expression, var_name: &str) -> (Expression, Expression) {
        let l = left.simplify();
        let r = right.simplify();

        match l {
            Expression::Add(a, b) => {
                if a.contains_variable(var_name) {
                    Self::solve_linear(*a, Expression::Subtract(Box::new(r), b), var_name)
                } else {
                    Self::solve_linear(*b, Expression::Subtract(Box::new(r), a), var_name)
                }
            },
            Expression::Subtract(a, b) => {
                if a.contains_variable(var_name) {
                    Self::solve_linear(*a, Expression::Add(Box::new(r), b), var_name)
                } else {
                    Self::solve_linear(Expression::Multiply(Box::new(Expression::Number(-1.0)), b), 
                                     Expression::Subtract(Box::new(r), a), var_name)
                }
            },
            Expression::Multiply(a, b) => {
                if a.contains_variable(var_name) {
                    Self::solve_linear(*a, Expression::Divide(Box::new(r), b), var_name)
                } else {
                    Self::solve_linear(*b, Expression::Divide(Box::new(r), a), var_name)
                }
            },
            Expression::Divide(l_expr, r_expr) => {
                if l_expr.contains_variable(var_name) {
                    // Caso: x / 2 = 10  =>  x = 10 * 2
                    Self::solve_linear(*l_expr, Expression::Multiply(Box::new(r), r_expr), var_name)
                } else if r_expr.contains_variable(var_name) {
                    // Caso: 10 / x = 2  =>  10 = 2 * x (Este es más avanzado, lo giramos)
                    Self::solve_linear(*r_expr, Expression::Divide(l_expr, Box::new(r)), var_name)
                } else {
                    (Expression::Divide(l_expr, r_expr), r)
                }
            },
            _ => (l, r)
        }
    }
}
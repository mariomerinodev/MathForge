use std::collections::BTreeMap;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

// 1. EL MOTOR DE FRACCIONES EXACTAS
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 { let temp = b; b = a % b; a = temp; }
    a.abs()
}

#[derive(Debug, Clone, Copy)]
pub struct Fraction {
    pub num: i64,
    pub den: i64,
}

impl Fraction {
    pub fn new(num: i64, den: i64) -> Self {
        if den == 0 { return Self { num: 0, den: 1 }; } 
        let divisor = gcd(num, den);
        let mut n = num / divisor;
        let mut d = den / divisor;
        if d < 0 { n = -n; d = -d; } 
        Self { num: n, den: d }
    }
    
    pub fn zero() -> Self { Self::new(0, 1) }
    pub fn one() -> Self { Self::new(1, 1) }
    pub fn minus_one() -> Self { Self::new(-1, 1) }
}

impl Add for Fraction { type Output = Self; fn add(self, other: Self) -> Self { Self::new(self.num * other.den + other.num * self.den, self.den * other.den) } }
impl Sub for Fraction { type Output = Self; fn sub(self, other: Self) -> Self { Self::new(self.num * other.den - other.num * self.den, self.den * other.den) } }
impl Mul for Fraction { type Output = Self; fn mul(self, other: Self) -> Self { Self::new(self.num * other.num, self.den * other.den) } }
impl Div for Fraction { type Output = Self; fn div(self, other: Self) -> Self { Self::new(self.num * other.den, self.den * other.num) } }
impl PartialEq for Fraction { fn eq(&self, other: &Self) -> bool { self.num == other.num && self.den == other.den } }
impl fmt::Display for Fraction { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        if self.den == 1 { write!(f, "{}", self.num) } else { write!(f, "{}/{}", self.num, self.den) } 
    } 
}

// 2. DEFINICIÓN DEL AST
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(Fraction), Variable(String),
    Plus, Minus, Multiply, Divide, Power,
    Equal, OpenParen, CloseParen, EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(Fraction),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    Error(String),
}

impl Expression {
    pub fn visualize(&self) -> String {
        match self {
            Expression::Number(n) => n.to_string(),
            Expression::Variable(v) => v.clone(),
            Expression::Error(e) => format!("Error: {}", e),
            Expression::Add(l, r) => {
                let l_str = l.visualize();
                if let Expression::Multiply(n, var) = &**r {
                    if let Expression::Number(val) = **n {
                        if val.num < 0 {
                            let positive_val = Fraction::new(-val.num, val.den);
                            let var_str = if val == Fraction::minus_one() { var.visualize() } else { format!("{} * {}", positive_val, var.visualize()) };
                            return format!("({} - {})", l_str, var_str);
                        }
                    }
                }
                if let Expression::Number(n) = &**r {
                    if n.num < 0 { return format!("({} - {})", l_str, Fraction::new(-n.num, n.den)); }
                }
                format!("({} + {})", l_str, r.visualize())
            }
            Expression::Multiply(l, r) => {
                if let Expression::Number(n) = &**l {
                    if *n == Fraction::one() { return r.visualize(); }
                    if *n == Fraction::minus_one() { return format!("-{}", r.visualize()); }
                }
                format!("({} * {})", l.visualize(), r.visualize())
            }
            Expression::Divide(l, r) => format!("({} / {})", l.visualize(), r.visualize()),
            Expression::Power(l, r) => format!("({} ^ {})", l.visualize(), r.visualize()),
        }
    }

    pub fn contains_variable(&self, var_name: &str) -> bool {
        match self {
            Expression::Variable(v) => v == var_name,
            Expression::Add(l, r) | Expression::Multiply(l, r) | 
            Expression::Divide(l, r) | Expression::Power(l, r) => {
                l.contains_variable(var_name) || r.contains_variable(var_name)
            }
            _ => false,
        }
    }

    pub fn expand(self) -> Expression {
        match self {
            Expression::Multiply(l, r) => {
                let l_exp = l.expand();
                let r_exp = r.expand();
                match (l_exp, r_exp) {
                    (a, Expression::Add(b, c)) => Expression::Add(
                        Box::new(Expression::Multiply(Box::new(a.clone()), b).expand()),
                        Box::new(Expression::Multiply(Box::new(a), c).expand())
                    ),
                    (Expression::Add(a, b), c) => Expression::Add(
                        Box::new(Expression::Multiply(a, Box::new(c.clone())).expand()),
                        Box::new(Expression::Multiply(b, Box::new(c)).expand())
                    ),
                    (a, b) => Expression::Multiply(Box::new(a), Box::new(b)),
                }
            }
            Expression::Add(l, r) => Expression::Add(Box::new(l.expand()), Box::new(r.expand())),
            Expression::Divide(l, r) => Expression::Divide(Box::new(l.expand()), Box::new(r.expand())),
            _ => self,
        }
    }

    pub fn simplify(self) -> Expression {
        let mut current = self;
        loop {
            let next = current.clone().simplify_step();
            if next == current { break current; }
            current = next;
        }
    }

    fn simplify_step(self) -> Expression {
        match self {
            Expression::Add(l, r) => {
                let l_sim = l.simplify_step();
                let r_sim = r.simplify_step();
                
                if let Expression::Number(n) = l_sim { if n == Fraction::zero() { return r_sim; } }
                if let Expression::Number(n) = r_sim { if n == Fraction::zero() { return l_sim; } }

                let mut terms = Vec::new();
                Self::flatten_add(Expression::Add(Box::new(l_sim), Box::new(r_sim)), &mut terms);
                
                let mut num_sum = Fraction::zero();
                let mut vars: BTreeMap<String, Fraction> = BTreeMap::new(); 
                let mut complex = Vec::new();

                for term in terms {
                    match term {
                        Expression::Number(n) => num_sum = num_sum + n,
                        Expression::Variable(v) => {
                            let e = vars.entry(v).or_insert(Fraction::zero());
                            *e = *e + Fraction::one();
                        },
                        Expression::Multiply(a, b) => {
                            match (*a, *b) {
                                (Expression::Number(n), Expression::Variable(v)) | 
                                (Expression::Variable(v), Expression::Number(n)) => {
                                    let e = vars.entry(v).or_insert(Fraction::zero());
                                    *e = *e + n;
                                }
                                (a_other, b_other) => complex.push(Expression::Multiply(Box::new(a_other), Box::new(b_other)))
                            }
                        }
                        other => complex.push(other)
                    }
                }

                let mut final_expr: Option<Expression> = None;
                for (v, coef) in vars {
                    if coef == Fraction::zero() { continue; }
                    let term = if coef == Fraction::one() { Expression::Variable(v) } 
                               else { Expression::Multiply(Box::new(Expression::Number(coef)), Box::new(Expression::Variable(v))) };
                    final_expr = match final_expr {
                        None => Some(term),
                        Some(e) => Some(Expression::Add(Box::new(e), Box::new(term)))
                    };
                }

                for term in complex {
                    final_expr = match final_expr {
                        None => Some(term),
                        Some(e) => Some(Expression::Add(Box::new(e), Box::new(term)))
                    };
                }

                if num_sum != Fraction::zero() || final_expr.is_none() {
                    let num_expr = Expression::Number(num_sum);
                    final_expr = match final_expr {
                        None => Some(num_expr),
                        Some(e) => Some(Expression::Add(Box::new(e), Box::new(num_expr)))
                    };
                }

                final_expr.unwrap()
            }
            Expression::Multiply(l, r) => {
                let l_sim = l.simplify_step();
                let r_sim = r.simplify_step();
                match (l_sim.clone(), r_sim.clone()) {
                    (Expression::Number(n1), Expression::Number(n2)) => Expression::Number(n1 * n2),
                    (other, Expression::Number(n)) if n == Fraction::one() => other,
                    (Expression::Number(n), other) if n == Fraction::one() => other,
                    (_, Expression::Number(n)) if n == Fraction::zero() => Expression::Number(Fraction::zero()),
                    (Expression::Number(n), _) if n == Fraction::zero() => Expression::Number(Fraction::zero()),
                    
                    (Expression::Number(n1), Expression::Multiply(a, b)) => {
                        if let Expression::Number(n2) = *a {
                            Expression::Multiply(Box::new(Expression::Number(n1 * n2)), b).simplify_step()
                        } else {
                            Expression::Multiply(Box::new(l_sim), Box::new(r_sim))
                        }
                    },
                    (Expression::Multiply(a, b), Expression::Number(n2)) => {
                        if let Expression::Number(n1) = *a {
                            Expression::Multiply(Box::new(Expression::Number(n1 * n2)), b).simplify_step()
                        } else {
                            Expression::Multiply(Box::new(l_sim), Box::new(r_sim))
                        }
                    },
                    (ls, rs) => Expression::Multiply(Box::new(ls), Box::new(rs)),
                }
            }
            Expression::Divide(l, r) => {
                let l_sim = l.simplify_step();
                let r_sim = r.simplify_step();
                match (l_sim, r_sim) {
                    (_, Expression::Number(n)) if n == Fraction::zero() => Expression::Error("División por cero".to_string()),
                    (Expression::Number(n1), Expression::Number(n2)) => Expression::Number(n1 / n2),
                    (Expression::Number(n), _) if n == Fraction::zero() => Expression::Number(Fraction::zero()),
                    
                    // MAGIA: Convertir x / 3 en (1/3) * x
                    (expr, Expression::Number(n)) => {
                        let inv = Fraction::new(n.den, n.num);
                        Expression::Multiply(
                            Box::new(Expression::Number(inv)),
                            Box::new(expr)
                        ).simplify_step()
                    },
                    (ls, rs) => Expression::Divide(Box::new(ls), Box::new(rs)),
                }
            }
            _ => self,
        }
    }

    fn flatten_add(expr: Expression, target: &mut Vec<Expression>) {
        if let Expression::Add(l, r) = expr {
            Self::flatten_add(*l, target);
            Self::flatten_add(*r, target);
        } else {
            target.push(expr);
        }
    }

    pub fn solve_linear(left: Expression, right: Expression, var_name: &str) -> Expression {
        let l = left.simplify();
        let r = right.simplify();

        if let Expression::Error(_) = l { return l; }
        if let Expression::Error(_) = r { return r; }

        if !l.contains_variable(var_name) {
            if l == r { return Expression::Error("Infinitas soluciones (Identidad)".into()); }
            return Expression::Error("Sin solución (Contradicción)".into());
        }

        match l {
            Expression::Add(a, b) => {
                if a.contains_variable(var_name) {
                    Self::solve_linear(*a, Expression::Add(Box::new(r), Box::new(Expression::Multiply(Box::new(Expression::Number(Fraction::minus_one())), b))), var_name)
                } else {
                    Self::solve_linear(*b, Expression::Add(Box::new(r), Box::new(Expression::Multiply(Box::new(Expression::Number(Fraction::minus_one())), a))), var_name)
                }
            },
            Expression::Multiply(a, b) => {
                if a.contains_variable(var_name) {
                    Self::solve_linear(*a, Expression::Divide(Box::new(r), b), var_name)
                } else {
                    Self::solve_linear(*b, Expression::Divide(Box::new(r), a), var_name)
                }
            },
            Expression::Divide(a, b) => {
                if a.contains_variable(var_name) {
                    Self::solve_linear(*a, Expression::Multiply(Box::new(r), b), var_name)
                } else {
                    Self::solve_linear(*b, Expression::Divide(a, Box::new(r)), var_name)
                }
            },
            Expression::Variable(_) => r.simplify(),
            _ => Expression::Error("No se pudo despejar la variable".into()),
        }
    }
}
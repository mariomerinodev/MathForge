use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64), Variable(String),
    Plus, Minus, Multiply, Divide, Power,
    Equal, OpenParen, CloseParen, EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    // ELIMINADO: Subtract. El árbol es puramente aditivo y multiplicativo.
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    Error(String), // Manejo limpio de errores
}

impl Expression {
    // 1. VISUALIZACIÓN INTELIGENTE (Oculta la normalización al usuario)
    pub fn visualize(&self) -> String {
        match self {
            Expression::Number(n) => n.to_string(),
            Expression::Variable(v) => v.clone(),
            Expression::Error(e) => format!("Error: {}", e),
            Expression::Add(l, r) => {
                let l_str = l.visualize();
                // Si el derecho es un negativo, lo mostramos como resta
                if let Expression::Multiply(n, var) = &**r {
                    if let Expression::Number(val) = **n {
                        if val < 0.0 {
                            let var_str = if val == -1.0 { var.visualize() } else { format!("{} * {}", -val, var.visualize()) };
                            return format!("({} - {})", l_str, var_str);
                        }
                    }
                }
                if let Expression::Number(n) = &**r {
                    if *n < 0.0 { return format!("({} - {})", l_str, -n); }
                }
                format!("({} + {})", l_str, r.visualize())
            }
            Expression::Multiply(l, r) => {
                // Ocultar el "1 * x"
                if let Expression::Number(n) = &**l {
                    if *n == 1.0 { return r.visualize(); }
                    if *n == -1.0 { return format!("-{}", r.visualize()); }
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

    // 2. EXPANSIÓN TOTAL DISTRIBUTIVA
    pub fn expand(self) -> Expression {
        match self {
            Expression::Multiply(l, r) => {
                let l_exp = l.expand();
                let r_exp = r.expand();
                match (l_exp, r_exp) {
                    // a * (b + c) = ab + ac
                    (a, Expression::Add(b, c)) => Expression::Add(
                        Box::new(Expression::Multiply(Box::new(a.clone()), b).expand()),
                        Box::new(Expression::Multiply(Box::new(a), c).expand())
                    ),
                    // (a + b) * c = ac + bc
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

    // 3. PUNTO FIJO DE SIMPLIFICACIÓN
    pub fn simplify(self) -> Expression {
        let mut current = self;
        loop {
            let next = current.clone().simplify_step();
            if next == current { break current; }
            current = next;
        }
    }

    // Lógica interna de simplificación
    fn simplify_step(self) -> Expression {
        match self {
            Expression::Add(l, r) => {
                let l_sim = l.simplify_step();
                let r_sim = r.simplify_step();
                
                // Elementos neutros
                if let Expression::Number(n) = l_sim { if n == 0.0 { return r_sim; } }
                if let Expression::Number(n) = r_sim { if n == 0.0 { return l_sim; } }

                let mut terms = Vec::new();
                Self::flatten_add(Expression::Add(Box::new(l_sim), Box::new(r_sim)), &mut terms);
                
                let mut num_sum = 0.0;
                let mut vars: BTreeMap<String, f64> = BTreeMap::new(); // <-- Fix del Bucle Infinito
                let mut complex = Vec::new();

                for term in terms {
                    match term {
                        Expression::Number(n) => num_sum += n,
                        Expression::Variable(v) => *vars.entry(v).or_insert(0.0) += 1.0,
                        Expression::Multiply(a, b) => {
                            match (*a, *b) {
                                (Expression::Number(n), Expression::Variable(v)) | 
                                (Expression::Variable(v), Expression::Number(n)) => {
                                    *vars.entry(v).or_insert(0.0) += n;
                                }
                                (a_other, b_other) => complex.push(Expression::Multiply(Box::new(a_other), Box::new(b_other)))
                            }
                        }
                        other => complex.push(other)
                    }
                }

                let mut final_expr: Option<Expression> = None;
                for (v, coef) in vars {
                    if coef == 0.0 { continue; }
                    let term = if coef == 1.0 { Expression::Variable(v) } 
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

                if num_sum != 0.0 || final_expr.is_none() {
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
                    (other, Expression::Number(1.0)) | (Expression::Number(1.0), other) => other,
                    (_, Expression::Number(0.0)) | (Expression::Number(0.0), _) => Expression::Number(0.0),
                    
                    // MAGIA ASOCIATIVA: Convierte -1 * (5 * x) en -5 * x
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
                    (_, Expression::Number(0.0)) => Expression::Error("División por cero".to_string()),
                    (Expression::Number(n1), Expression::Number(n2)) => Expression::Number(n1 / n2),
                    (Expression::Number(0.0), _) => Expression::Number(0.0),
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

    // 4. SOLVER ROBUSTO
    pub fn solve_linear(left: Expression, right: Expression, var_name: &str) -> Expression {
        let l = left.simplify();
        let r = right.simplify();

        if let Expression::Error(_) = l { return l; }
        if let Expression::Error(_) = r { return r; }

        if !l.contains_variable(var_name) {
            // Evaluamos si llegamos a una identidad (ej. 0 = 0) o contradicción (ej. 5 = 0)
            if l == r { return Expression::Error("Infinitas soluciones (Identidad)".into()); }
            return Expression::Error("Sin solución (Contradicción)".into());
        }

        match l {
            Expression::Add(a, b) => {
                if a.contains_variable(var_name) {
                    Self::solve_linear(*a, Expression::Add(Box::new(r), Box::new(Expression::Multiply(Box::new(Expression::Number(-1.0)), b))), var_name)
                } else {
                    Self::solve_linear(*b, Expression::Add(Box::new(r), Box::new(Expression::Multiply(Box::new(Expression::Number(-1.0)), a))), var_name)
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
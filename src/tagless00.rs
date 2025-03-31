pub enum Expr {
    Lit(i32),                       // literal
    Neg(Box<Expr>),                 // negation
    Add(Box<Expr>, Box<Expr>),      // addition
}

impl Expr {
    pub fn lit(n: i32) -> Self {
        Self::Lit(n)
    }

    pub fn neg(e: Expr) -> Self {
        Self::Neg(Box::new(e))
    }

    pub fn add(e1: Expr, e2: Expr) -> Self {
        Self::Add(Box::new(e1), Box::new(e2))
    }
}

impl Expr {

    pub fn eval(&self) -> i32 {
        match self {
            Self::Lit(n) => *n,
            Self::Neg(e) => -e.eval(),
            Self::Add(e1, e2) => e1.eval() + e2.eval(),
        }
    }
}

impl Expr {
    pub fn show(&self) -> String {
        match self {
            Self::Lit(n) => n.to_string(),
            Self::Neg(e) => format!("-({})", e.show()),
            Self::Add(e1, e2) => format!("({} + {})", e1.show(), e2.show()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let e = Expr::add(Expr::lit(1), Expr::lit(2));
        assert_eq!(e.eval(), 3);
    }

    #[test]
    fn test_show() {
        let e = Expr::add(Expr::lit(1), Expr::lit(2));
        assert_eq!(e.show(), "(1 + 2)");
    }
}


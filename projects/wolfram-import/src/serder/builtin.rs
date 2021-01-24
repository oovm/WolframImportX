use crate::{Expr, Symbol, ToWolfram};

impl ToWolfram for () {
    fn to_wolfram(&self) -> Expr {
        Symbol::new("System`None").into()
    }
}

impl ToWolfram for bool {
    fn to_wolfram(&self) -> Expr {
        match self {
            true => Symbol::new("System`True").into(),
            false => Symbol::new("System`False").into(),
        }
    }
}

impl ToWolfram for &str {
    fn to_wolfram(&self) -> Expr {
        Expr::string(self.to_string())
    }
}

impl ToWolfram for &String {
    fn to_wolfram(&self) -> Expr {
        Expr::string(self.to_string())
    }
}

impl ToWolfram for String {
    fn to_wolfram(&self) -> Expr {
        Expr::string(self)
    }
}

impl ToWolfram for f64 {
    fn to_wolfram(&self) -> Expr {
        if self.is_infinite() {
            return Symbol::new("System`Infinity").into();
        }
        if self.is_nan() {
            return Symbol::new("System`Indeterminate").into();
        }
        Expr::real(*self)
    }
}

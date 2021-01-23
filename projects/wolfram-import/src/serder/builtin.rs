use crate::{Expr, ToWolfram};

impl ToWolfram for () {
    fn to_wolfram(&self) -> Expr {
        Expr::symbol("System`None")
    }
}

impl ToWolfram for bool {
    fn to_wolfram(&self) -> Expr {
        match self {
            true => {Expr::symbol("System`True")}
            false => {Expr::symbol("System`False")}
        }
    }
}
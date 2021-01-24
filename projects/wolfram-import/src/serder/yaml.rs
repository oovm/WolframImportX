use crate::{Expr, ToWolfram};
use serde_yaml::{Mapping, Number, Sequence, Value};
use wolfram_library_link::expr::Symbol;

impl ToWolfram for Value {
    fn to_wolfram(&self) -> Expr {
        match self {
            Value::Null => ().to_wolfram(),
            Value::Bool(v) => v.to_wolfram(),
            Value::Number(v) => v.to_wolfram(),
            Value::String(v) => v.to_wolfram(),
            Value::Sequence(v) => v.to_wolfram(),
            Value::Mapping(v) => v.to_wolfram(),
        }
    }
}

impl ToWolfram for Number {
    fn to_wolfram(&self) -> Expr {
        if let Some(n) = self.as_u64() {
            return (n as u32).into();
        }
        if let Some(n) = self.as_i64() {
            return n.into();
        }
        if let Some(n) = self.as_f64() {
            return n.to_wolfram();
        }
        Expr::null()
    }
}

impl ToWolfram for Sequence {
    fn to_wolfram(&self) -> Expr {
        Expr::list(self.iter().cloned().map(|f| f.to_wolfram()).collect())
    }
}

impl ToWolfram for Mapping {
    fn to_wolfram(&self) -> Expr {
        let mut association = Vec::with_capacity(self.len());
        for (k, v) in self {
            association.push(Expr::rule(k.to_wolfram(), v.to_wolfram()))
        }
        Expr::normal(Symbol::new("System`Association"), association)
    }
}

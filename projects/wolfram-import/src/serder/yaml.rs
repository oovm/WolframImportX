use serde_yaml::{Mapping, Number, Sequence, Value};
use crate::{Expr, ToWolfram};

impl ToWolfram for Value {
    fn to_wolfram(&self) -> Expr {
        match self {
            Value::Null => { Expr::null() }
            Value::Bool(v) => { Expr::from(v) }
            Value::Number(v) => { Expr::from(v) }
            Value::String(v) => { Expr::from(v) }
            Value::Sequence(v) => { Expr::from(v) }
            Value::Mapping(v) => { Expr::from(v) }
        }
    }
}

impl ToWolfram for Number {
    fn to_wolfram(&self) -> Expr {
        if let Some(n) = self.as_u64() {
            return n.into();
        }
        if let Some(n) = self.as_i64() {
            return n.into();
        }
        if let Some(n) = self.as_f64() {
            return n.into();
        }
        Expr::null()
    }
}

impl ToWolfram for Sequence {
    fn to_wolfram(&self) -> Expr {
        todo!()
    }
}

impl ToWolfram for Mapping {
    fn to_wolfram(&self) -> Expr {
        todo!()
    }
}
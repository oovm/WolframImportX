use crate::{Expr, ToWolfram};
use toml::{
    value::{Array, Datetime, Table},
    Value,
};
use wolfram_library_link::expr::Symbol;

impl ToWolfram for Value {
    fn to_wolfram(&self) -> Expr {
        match self {
            Value::String(v) => v.to_wolfram(),
            Value::Integer(v) => Expr::from(*v),
            Value::Float(v) => v.to_wolfram(),
            Value::Boolean(v) => v.to_wolfram(),
            Value::Datetime(v) => v.to_wolfram(),
            Value::Array(v) => v.to_wolfram(),
            Value::Table(v) => v.to_wolfram(),
        }
    }
}

impl ToWolfram for Datetime {
    fn to_wolfram(&self) -> Expr {
        todo!()
    }
}

impl ToWolfram for Array {
    fn to_wolfram(&self) -> Expr {
        Expr::list(self.iter().cloned().map(|f| f.to_wolfram()).collect())
    }
}

impl ToWolfram for Table {
    fn to_wolfram(&self) -> Expr {
        let mut association = Vec::with_capacity(self.len());
        for (k, v) in self {
            association.push(Expr::rule(k.to_wolfram(), v.to_wolfram()))
        }
        Expr::normal(Symbol::new("System`Association"), association)
    }
}

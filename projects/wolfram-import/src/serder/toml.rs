use toml::Value;
use toml::value::{Array, Datetime, Table};
use wolfram_library_link::expr::Symbol;
use crate::{Expr, ToWolfram};

impl ToWolfram for Value {
    fn to_wolfram(&self) -> Expr {
        match self {
            Value::String(v) => { Expr::from(v.clone()) }
            Value::Integer(v) => { Expr::from(v) }
            Value::Float(v) => { Expr::from(v) }
            Value::Boolean(v) => { Expr::from(v) }
            Value::Datetime(v) => { Expr::from(v) }
            Value::Array(v) => { Expr::from(v) }
            Value::Table(v) => { Expr::from(v) }
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
            association.push(Expr::rule(k, v.to_wolfram()))
        }
        Expr::normal(Symbol::new("System`Association"), association)
    }
}
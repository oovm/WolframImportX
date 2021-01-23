use serde_json::{Map, Number, Value};
use wolfram_library_link::expr::Symbol;
use crate::{Expr, ToWolfram};


impl ToWolfram for serde_json::Value {
    fn to_wolfram(&self) -> Expr {
        match self {
            Value::Null => { ().to_wolfram() }
            Value::Bool(v) => { v.to_wolfram() }
            Value::Number(v) => {}
            Value::String(v) => {}
            Value::Array(v) => { Expr::list(ex) }
            Value::Object(v) => {
                let mut association = Vec::with_capacity(self.len());
                for (k, v) in self {
                    association.push(Expr::rule(k, v.to_wolfram()))
                }
                Expr::normal(Symbol::new("System`Association"), association)
            }
        }
    }
}

impl ToWolfram for Number {
    fn to_wolfram(&self) -> Expr {
        todo!()
    }
}

use crate::{import_error, parse_error, Expr, Result, ToWolfram};
use json5::from_str;
use serde_json::{Map, Number, Value};
use wolfram_library_link::{
    expr::{ExprKind, Symbol},
    wstp,
    wstp::{Error, Link},
};

pub fn try_import_json5(args: Vec<Expr>) -> Result<Expr> {
    let input = match args.as_slice() {
        [] => return import_error!("Must use one string argument as input"),
        [a] => match a.kind() {
            ExprKind::String(s) => s.as_str(),
            _ => return import_error!("argument must be a string"),
        },
        _ => return import_error!("too many arguments"),
    };

    match from_str::<Value>(input) {
        Ok(s) => Ok(s.to_wolfram()),
        Err(e) => parse_error!("{}", e),
    }
}

impl ToWolfram for Value {
    fn to_wolfram(&self) -> Expr {
        match self {
            Value::Null => ().to_wolfram(),
            Value::Bool(v) => v.to_wolfram(),
            Value::Number(v) => v.to_wolfram(),
            Value::String(v) => v.to_wolfram(),
            Value::Array(v) => Expr::list(v.iter().map(|f| f.to_wolfram()).collect()),
            Value::Object(v) => {
                let mut association = vec![];
                for (k, v) in v {
                    association.push(Expr::rule(k.to_wolfram(), v.to_wolfram()))
                }
                Expr::normal(Symbol::new("System`Association"), association)
            }
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

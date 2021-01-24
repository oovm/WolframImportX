use crate::{Expr, Result, ToWolfram};
use serde_json::{from_str, Map, Number, Value};
use wolfram_library_link::{
    expr::{ExprKind, Symbol},
    wstp,
    wstp::{Error, Link},
};

pub fn import_json(args: Vec<Expr>) -> Result<Expr> {
    let input = match args.as_slice() {
        [] => return Ok(Expr::string("import_json: no input")),
        [a] => match a.kind() {
            ExprKind::String(s) => s.as_str(),
            _ => return Ok(Expr::string("import_json: input must a string")),
        },
        _ => return Ok(Expr::string("import_json: too many arguments")),
    };

    match from_str::<Value>(input) {
        Ok(s) => Ok(s.to_wolfram()),
        Err(e) => Ok(Expr::string(format!("import_json: error parsing input: {}", e))),
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
        todo!()
    }
}

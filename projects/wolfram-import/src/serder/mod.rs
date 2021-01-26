mod builtin;
mod json;
mod toml;
mod yaml;

pub use self::{json::try_import_json5, toml::try_import_toml, yaml::try_import_yaml};
use crate::{import_error, Expr, ImporterError, Result};
pub use wolfram_library_link::export;
use wolfram_library_link::expr::ExprKind;

#[export(wstp)]
pub fn import_json5(args: Vec<Expr>) -> Expr {
    try_import_json5(args).unwrap_or_else(|_| Expr::null())
}

#[export(wstp)]
pub fn import_toml(args: Vec<Expr>) -> Expr {
    try_import_toml(args).unwrap_or_else(|_| Expr::null())
}

#[export(wstp)]
pub fn import_yaml(args: Vec<Expr>) -> Expr {
    try_import_yaml(args).unwrap_or_else(|_| Expr::null())
}

fn get_string_from_args(args: &[Expr]) -> Result<&str> {
    match args {
        [] => import_error!("Must use one string argument as input"),
        [a] => match a.kind() {
            ExprKind::String(s) => Ok(s.as_str()),
            _ => import_error!("argument must be a string"),
        },
        _ => import_error!("too many arguments"),
    }
}
#[allow(dead_code)]
fn get_bytes_from_args(args: &[Expr]) -> Result<Vec<u8>> {
    match args {
        [] => import_error!("Must use one string argument as input"),
        [_] => todo!(),
        _ => import_error!("too many arguments"),
    }
}

mod builtin;
mod json;
mod toml;
mod yaml;

use crate::Expr;
pub use json::try_import_json5;
pub use wolfram_library_link::export;

#[export(wstp)]
pub fn import_json5(args: Vec<Expr>) -> Expr {
    try_import_json5(args).unwrap()
}

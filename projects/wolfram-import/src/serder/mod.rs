mod builtin;
mod json;
mod toml;
mod yaml;

use crate::Expr;
pub use json::import_json;
pub use wolfram_library_link::export;

#[export(wstp)]
pub fn import_string_json(args: Vec<Expr>) -> Expr {
    import_json(args).unwrap()
}

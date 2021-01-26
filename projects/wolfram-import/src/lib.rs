mod errors;
mod serder;

pub use errors::{ImporterError, Result};
pub use serder::try_import_json5;

use wolfram_library_link::expr::{Expr, Symbol};

pub trait ToWolfram {
    fn to_wolfram(&self) -> Expr;
}

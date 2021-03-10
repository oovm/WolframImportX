use crate::{Expr, ToWolfram};
use usvg::{Node, NodeKind, Tree};

impl ToWolfram for Tree {
    fn to_wolfram(&self) -> Expr {
        todo!()
    }
}

impl ToWolfram for Node {
    fn to_wolfram(&self) -> Expr {
        todo!()
    }
}

impl ToWolfram for NodeKind {
    fn to_wolfram(&self) -> Expr {
        todo!()
    }
}

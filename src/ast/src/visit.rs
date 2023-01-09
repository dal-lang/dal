//! AST walker.
//!
//! This module defines the `Visitor` trait, which is used to traverse the AST.
//! It also defines the `walk_*` functions, which are used to implement the
//! `Visitor` trait.

use crate::ast::*;

pub trait Visitor<'ast>: Sized {
    fn visit_ident(&mut self, _ident: &'ast Ident) {}
}

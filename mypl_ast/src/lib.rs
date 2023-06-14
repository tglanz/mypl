extern crate mypl_lex;
extern crate anyhow;

mod ast;
mod ast_formatter;
mod visitor;

pub mod prelude {
    use crate::*;
    pub use ast::*;
    pub use ast_formatter::AstFormatter;
    pub use visitor::{ExprVisitor, AcceptExprVisitor, StmtVisitor, AcceptStmtVisitor};
}

extern crate thiserror;
extern crate anyhow;

extern crate mypl_lex;
extern crate mypl_ast;

mod interperter;

pub mod prelude {
    use crate::*;

    pub use interperter::{Interperter, ExprValue};
}

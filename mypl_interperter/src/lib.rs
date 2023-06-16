extern crate thiserror;
extern crate anyhow;

extern crate mypl_lex;
extern crate mypl_ast;

mod interperter;
mod expr_eval;
mod error;

pub mod prelude {
    use crate::*;

    pub use error::InterperterError;
    pub use interperter::Interperter;
}
